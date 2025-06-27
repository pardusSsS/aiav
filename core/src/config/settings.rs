use anyhow::Result;
use serde::{Deserialize, Serialize}; // DÜZELTME 1: Serialize'ı içeri aktarıyoruz.
use std::path::Path;
use tokio::fs; // DÜZELTME 2: Tokio'nun asenkron 'fs' modülünü içeri aktarıyoruz.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSecuritySettings {
    pub blocklist_urls: Vec<String>,
    pub user_whitelist: Vec<String>, 
    pub user_blacklist: Vec<String>,
    pub enable_tracker_protection: bool, 
    pub enable_malware_protection: bool,
}

// DÜZELTME 3: `derive` listesine 'Serialize' özelliğini ekliyoruz.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// Tehdit imza veritabanının yolu
    pub signature_db: String,
    /// Karantina dizini
    pub quarantine_dir: String,
    /// AI sunucusu URL'si
    pub ai_endpoint: String,
    /// Tarama iş parçacığı havuzu
    pub scan_threads: u32,
    /// İzlenecek dizin
    pub watch_dir: String,
    /// Tarama sırasında hariç tutulacak dosya uzantıları
    pub excluded_extensions: Vec<String>,
    
    pub web_security: WebSecuritySettings,

}

impl Settings {
    /// Varsayılan ayarları 'config/default.toml' dosyasından yükler.
    pub fn load_default() -> Result<Self> {
        let config_str = std::fs::read_to_string("config/default.toml")?;
        let settings: Settings = toml::from_str(&config_str)?;
        Ok(settings)
    }

    /// Ayarları TOML formatında dosyaya asenkron olarak kaydeder.
    pub async fn save(&self, path: &Path) -> Result<()> {
        let toml_string = toml::to_string_pretty(self)?;
        // 'use tokio::fs;' sayesinde bu artık asenkron bir çağrıdır ve .await ile kullanılabilir.
        fs::write(path, toml_string).await?;
        Ok(())
    }
}