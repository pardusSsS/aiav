use anyhow::{anyhow, Result};
use serde::{Serialize, Deserialize};
use std::ffi::OsString;
use std::fs as std_fs;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::task;
use tracing::info;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuarantinedFileInfo {
    pub quarantined_name: String,
    pub original_path: String,
    pub quarantined_at: String,
    pub signature: String,
}

#[derive(Default, Debug, Serialize)]
pub struct QuarantineStats {
    pub count: u64,
    pub files: Vec<QuarantinedFileInfo>,
}

#[derive(Clone)]
pub struct QuarantineManager;

impl QuarantineManager {
    pub fn new() -> Self {
        std_fs::create_dir_all("C:/AIAV/quarantine").ok();
        Self {}
    }
    
    /// Karantinadaki dosyanın ve metadata'sının tam yollarını oluşturur.
    fn get_quarantine_paths(&self, quarantine_dir: &Path, quarantined_name: &str) -> (PathBuf, PathBuf) {
        let file_path = quarantine_dir.join(quarantined_name);
        let mut meta_path_os_str: OsString = file_path.as_os_str().into();
        meta_path_os_str.push(".meta");
        let meta_path = PathBuf::from(meta_path_os_str);
        (file_path, meta_path)
    }

    /// NİHAİ DÜZELTME: Dosya adını ve uzantısını doğru koruyan versiyon.
    pub async fn quarantine_file(&self, original_path: &Path, signature: &str, quarantine_dir: &Path) -> Result<()> {
        // .file_name() kullanarak dosya adını UZANTISIYLA BİRLİKTE alıyoruz.
        let file_name = original_path
            .file_name()
            .ok_or_else(|| anyhow!("Could not get file name from path: {}", original_path.display()))?;

        let unique_name = format!("{}-{}", Uuid::new_v4(), file_name.to_string_lossy());
        let destination_path = quarantine_dir.join(&unique_name);

        // Dosyayı yeni adıyla karantinaya taşı
        fs::rename(original_path, &destination_path).await?;
        
        let meta_content = format!("[Threat Info]\nsignature = \"{}\"\noriginal_path = \"{}\"", signature, original_path.display());

        // Meta dosyasının adını, karantinadaki dosyanın tam adının sonuna .meta ekleyerek oluştur.
        let (_, meta_path) = self.get_quarantine_paths(quarantine_dir, &unique_name);
        fs::write(meta_path, meta_content).await?;
        
        info!(from = %original_path.display(), to = %destination_path.display(), "File successfully quarantined.");
        Ok(())
    }

    // `read_meta_file` fonksiyonunu, meta dosyasının adından `.meta` uzantısını atarak
    // orijinal karantina adını (uzantısıyla birlikte) alacak şekilde güncelliyoruz.
    fn read_meta_file(meta_path: &Path) -> Result<QuarantinedFileInfo> {
        let content = std_fs::read_to_string(meta_path)?;
        let mut original_path = String::new();
        let mut signature = String::new();

        for line in content.lines() {
            if let Some(val) = line.strip_prefix("signature = ") {
                signature = val.trim_matches('"').to_string();
            } else if let Some(val) = line.strip_prefix("original_path = ") {
                original_path = val.trim_matches('"').to_string();
            }
        }
        
        // NİHAİ DÜZELTME: .file_stem() `.meta` uzantısını kaldırır ve geriye kalan
        // tam dosya adını (örn: uuid-asd.cpp) bize verir.
        let quarantined_name = meta_path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow!("Could not get a valid file name from meta path: {}", meta_path.display()))?
            .to_string();

        Ok(QuarantinedFileInfo { quarantined_name, original_path, signature, quarantined_at: "N/A".to_string() })
    }
    
    // Diğer tüm fonksiyonlar (list_files, delete_file, restore_file, get_stats)
    // bu yeni ve doğru mantıkla sorunsuz çalışacağı için aynı kalabilir.
    // Kolaylık olması için tam hallerini ekliyorum.
    
    pub async fn list_files(&self, quarantine_dir: &Path) -> Result<Vec<QuarantinedFileInfo>> {
        let dir = quarantine_dir.to_path_buf();
        task::spawn_blocking(move || {
            let mut files = Vec::new();
            for entry in std_fs::read_dir(&dir)?.filter_map(Result::ok) {
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("meta") {
                    if let Ok(info) = Self::read_meta_file(&path) { files.push(info); }
                }
            }
            Ok(files)
        }).await?
    }

    pub async fn delete_file(&self, quarantined_name: &str, quarantine_dir: &Path) -> Result<()> {
        let (file_path, meta_path) = self.get_quarantine_paths(quarantine_dir, quarantined_name);
        task::spawn_blocking(move || {
            if file_path.exists() { std_fs::remove_file(&file_path)?; }
            if meta_path.exists() { std_fs::remove_file(&meta_path)?; }
            Ok(())
        }).await?
    }

    pub async fn restore_file(&self, quarantined_name: &str, quarantine_dir: &Path) -> Result<()> {
        let (file_path, meta_path) = self.get_quarantine_paths(quarantine_dir, quarantined_name);
        task::spawn_blocking(move || {

            if !meta_path.exists() { return Err(anyhow!("Metadata file not found: {}", meta_path.display())); }
            if !file_path.exists() { return Err(anyhow!("Quarantined data file not found: {}", file_path.display())); }
            let info = Self::read_meta_file(&meta_path)?;
            let original_path = PathBuf::from(info.original_path);
            if original_path.exists() {
                return Err(anyhow!("Cannot restore file, a file already exists at the original path: {}", original_path.display()));
            }
            if let Some(parent_dir) = original_path.parent() { std_fs::create_dir_all(parent_dir)?; }
            std_fs::rename(&file_path, &original_path)?;
            std_fs::remove_file(&meta_path)?;
            Ok(())
        }).await?
    }
    
    pub async fn get_stats(&self, quarantine_dir: &Path) -> Result<QuarantineStats> {
        let files = self.list_files(quarantine_dir).await?;
        Ok(QuarantineStats { count: files.len() as u64, files })
    }
}