pub mod quarantine;
pub mod scanner;

use crate::config::settings::Settings;
use crate::detection::ai::AiClient;
use crate::detection::{ThreatStatus, web_security::WebSecurity};
use crate::engine::quarantine::{QuarantineManager, QuarantineStats};
use crate::engine::scanner::Scanner;
use anyhow::Result;
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;
use tracing::{debug, error, info, warn};
use uuid::Uuid;
use walkdir::WalkDir;

/// Tarama ilerlemesinin anlık durumunu tutan yapı.
#[derive(Clone, Serialize, Default, Debug)]
pub struct ScanProgress {
    pub is_running: bool,
    pub current_file: String,
    pub total_files: u64,
    pub scanned_files: u64,
}

/// API'ye gönderilecek genel motor durumu yapısı.
#[derive(Serialize)]
pub struct EngineStatus {
    is_scanning: bool,
    files_scanned: u64,
    threats_found: u64,
    quarantined_count: u64,
}

/// Engine koordinatörü: Diğer tüm bileşenleri yönetir.
pub struct Engine {
    pub settings: Settings,
    pub scanner: Arc<Scanner>,
    pub quarantine: Arc<QuarantineManager>,
    pub web_security: Arc<WebSecurity>, // Web güvenliği modülü
    // Durum takibi için atomik alanlar
    pub files_scanned: Arc<AtomicU64>,
    pub threats_found: Arc<AtomicU64>,
    pub is_scanning: Arc<AtomicBool>,
    pub scan_progress: Arc<Mutex<ScanProgress>>,
}

impl Engine {
    /// Yeni Engine oluşturur ve gerekli bileşenleri başlatır.
    pub async fn new(settings: Settings) -> Result<Self> {
        let ai_client = Arc::new(AiClient::new(settings.ai_endpoint.clone()));
        let quarantine = Arc::new(QuarantineManager::new());
        let scanner = Arc::new(Scanner::new(settings.clone(), ai_client).await?);
        let web_security = Arc::new(WebSecurity::new()?);

        Ok(Self {
            settings,
            scanner,
            quarantine,
            web_security,
            files_scanned: Arc::new(AtomicU64::new(0)),
            threats_found: Arc::new(AtomicU64::new(0)),
            is_scanning: Arc::new(AtomicBool::new(false)),
            scan_progress: Arc::new(Mutex::new(ScanProgress::default())),
        })
    }

    /// Anlık motor durumunu ve istatistiklerini toplar.
    pub async fn get_current_status(&self) -> Result<EngineStatus> {
        let q_dir = Path::new(&self.settings.quarantine_dir);
        let stats = self.quarantine.get_stats(q_dir).await?;
        Ok(EngineStatus {
            is_scanning: self.is_scanning.load(Ordering::Relaxed),
            files_scanned: self.files_scanned.load(Ordering::Relaxed),
            threats_found: self.threats_found.load(Ordering::Relaxed),
            quarantined_count: stats.count,
        })
    }
    
    /// Hızlı tarama mantığını başlatan fonksiyon
    pub async fn run_quick_scan(&self) -> Result<()> {
        self.is_scanning.store(true, Ordering::Relaxed);
        info!("⚡ Quick Scan initiated.");

        // Windows için kritik ve yaygın olan yolları tanımlıyoruz.
        let critical_paths = vec![
            "C:\\AIAV_Test\\TempA".to_string(), // Test amaçlı
            "C:\\AIAV_Test\\TempB".to_string(), // Test amaçlı
            // std::env::var("ProgramFiles").unwrap_or_default(),
            // std::env::var("SystemRoot").unwrap_or_default() + "\\System32",
        ];

        for path_str in critical_paths {
            if path_str.is_empty() { continue; }
            let path = Path::new(&path_str);
            if path.exists() && path.is_dir() {
                self.run_path_scan(&path_str).await?;
            } else {
                warn!("Quick scan path does not exist or is not a directory: {}", path_str);
            }
        }

        self.is_scanning.store(false, Ordering::Relaxed);
        info!("⚡ Quick Scan finished.");
        Ok(())
    }

    /// Belirtilen bir yoldaki tüm dosyaları tarar, ilerlemeyi raporlar ve tehditleri karantinaya alır.
    pub async fn run_path_scan(&self, path_to_scan: &str) -> Result<()> {
        let quarantine_path = Path::new(&self.settings.quarantine_dir);
        let scan_path = Path::new(path_to_scan);
        if scan_path.starts_with(quarantine_path) {
            warn!("Scan path is inside quarantine directory. Skipping to prevent loop: {}", path_to_scan);
            return Ok(());
        }
        
        let path_buf = PathBuf::from(path_to_scan);
        let total_files = task::spawn_blocking(move || {
            WalkDir::new(path_buf).into_iter().filter_map(Result::ok).filter(|e| e.file_type().is_file()).count() as u64
        }).await?;
        
        {
            let mut progress = self.scan_progress.lock().await;
            progress.is_running = true;
            progress.total_files = total_files;
            progress.scanned_files = 0;
            progress.current_file = "Scan starting...".to_string();
        }

        info!("🔁 Starting comprehensive scan on directory: {}", path_to_scan);
        for entry in WalkDir::new(path_to_scan).into_iter().filter_map(Result::ok) {
            if entry.file_type().is_file() {
                {
                    let mut progress = self.scan_progress.lock().await;
                    progress.current_file = entry.path().to_string_lossy().to_string();
                }
                self.scan_and_handle(entry.path()).await;
                {
                     let mut progress = self.scan_progress.lock().await;
                     progress.scanned_files += 1;
                }
            }
        }
        
        {
            let mut progress = self.scan_progress.lock().await;
            progress.is_running = false;
        }
        Ok(())
    }

    /// Dosya olaylarını alır, tarar ve sonuca göre aksiyon alır.
    pub async fn scan_and_handle(&self, path: &Path) {
        if !path.is_file() { return; }

        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            if file_name.len() >= 36 {
                if Uuid::parse_str(&file_name[..36]).is_ok() {
                    debug!("Skipping already processed/quarantined file: {}", path.display());
                    return;
                }
            }
        }

        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            if self.settings.excluded_extensions.iter().any(|excluded_ext| ext.eq_ignore_ascii_case(excluded_ext.trim_start_matches('.'))) {
                debug!(file = %path.display(), "File extension is in exclusion list. Skipping scan.");
                return;
            }
        }
        
        self.files_scanned.fetch_add(1, Ordering::Relaxed);
        info!(file = %path.display(), "Scan starting...");

        match self.scanner.scan_file(path).await {
            Ok(scan_result) => {
                if let ThreatStatus::ThreatDetected { signature } = scan_result.status {
                    self.threats_found.fetch_add(1, Ordering::Relaxed);
                    warn!(file = %path.display(), signature = %signature, "❗ THREAT DETECTED!");
                    
                    let q_dir = Path::new(&self.settings.quarantine_dir);
                    if let Err(e) = self.quarantine.quarantine_file(path, &signature, q_dir).await {
                        error!(file = %path.display(), error = %e, "Failed to quarantine file!");
                    } else {
                        info!(file = %path.display(), "File successfully quarantined.");
                    }
                }
            }
            Err(e) => {
                error!(file = %path.display(), error = %e, "An error occurred during scanning.");
            }
        }
    }

    // --- Karantina Yönetim Metotları ---
    pub async fn get_quarantine_stats(&self) -> Result<QuarantineStats> {
        let q_dir = Path::new(&self.settings.quarantine_dir);
        self.quarantine.get_stats(q_dir).await
    }

    pub async fn delete_quarantined_file(&self, filename: &str) -> Result<()> {
        let q_dir = Path::new(&self.settings.quarantine_dir);
        self.quarantine.delete_file(filename, q_dir).await
    }

    pub async fn restore_quarantined_file(&self, filename: &str) -> Result<()> {
        let q_dir = Path::new(&self.settings.quarantine_dir);
        self.quarantine.restore_file(filename, q_dir).await
    }

    /// Gelen yeni ayarları uygular ve dosyaya kaydeder.
    pub async fn update_and_save_settings(&mut self, new_settings: Settings) -> Result<()> {
        new_settings.save(Path::new("config/default.toml")).await?;
        self.settings = new_settings;
        info!("Settings updated and saved.");

        // YENİ: Ayarlar kaydedildiğinde, web korumasını yeni kurallara göre hemen uygula.
        info!("Re-applying web protection with new settings...");
        self.enable_web_protection().await?;
        
        Ok(())
    }

    // --- Web Güvenliği Metotları ---
    pub async fn enable_web_protection(&self) -> Result<()> {
        self.web_security.apply_blocks(
            &self.settings.web_security
        ).await
    }

    pub fn disable_web_protection(&self) -> Result<()> {
        self.web_security.remove_blocks()
    }
    
    pub fn is_web_protection_active(&self) -> bool {
        self.web_security.is_protection_active()
    }
}