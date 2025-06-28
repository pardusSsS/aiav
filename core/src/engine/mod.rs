//! engine/mod.rs
//! AI-Antivirus çekirdek motoru: tarama, karantina, web koruma.

pub mod quarantine;
pub mod scanner;

use crate::config::settings::Settings;
use crate::detection::ai::AiClient;
use crate::detection::{web_security::WebSecurity, ThreatStatus};
use crate::engine::quarantine::{QuarantineManager, QuarantineStats};
use crate::engine::scanner::Scanner;

use anyhow::Result;
use futures::executor::block_on;
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;
use tracing::{debug, error, info, warn};
use uuid::Uuid;
use walkdir::WalkDir;

/// Tarama ilerleme bilgisi – API’ye JSON olarak gönderilir.
#[derive(Clone, Serialize, Default, Debug)]
pub struct ScanProgress {
    pub is_running: bool,
    pub current_file: String,
    pub total_files: u64,
    pub scanned_files: u64,
}

/// API’nin döndüğü genel motor durumu.
#[derive(Serialize)]
pub struct EngineStatus {
    is_scanning: bool,
    files_scanned: u64,
    threats_found: u64,
    quarantined_count: u64,
    is_web_protection_active: bool,
}

/// Ana motor koordinatörü – tüm bileşenleri yönetir.
pub struct Engine {
    pub settings: Settings,
    pub scanner: Arc<Scanner>,
    pub quarantine: Arc<QuarantineManager>,
    // DEĞİŞİKLİK: WebSecurity artık Mutex içinde, çünkü state'i (&mut self) değişiyor.
    pub web_security: Arc<Mutex<WebSecurity>>,
    pub files_scanned: Arc<AtomicU64>,
    pub threats_found: Arc<AtomicU64>,
    pub is_scanning: Arc<AtomicBool>,
    pub scan_progress: Arc<Mutex<ScanProgress>>,
}

impl Engine {
    /// Yeni motor oluşturur ve alt modülleri başlatır.
    pub async fn new(settings: Settings) -> Result<Self> {
        let ai_client = Arc::new(AiClient::new(settings.ai_endpoint.clone()));
        let quarantine = Arc::new(QuarantineManager::new());
        let scanner = Arc::new(Scanner::new(settings.clone(), ai_client).await?);
        let web_security = Arc::new(Mutex::new(WebSecurity::new()?));

        let engine = Self {
            settings,
            scanner,
            quarantine,
            web_security,
            files_scanned: Arc::new(AtomicU64::new(0)),
            threats_found: Arc::new(AtomicU64::new(0)),
            is_scanning: Arc::new(AtomicBool::new(false)),
            scan_progress: Arc::new(Mutex::new(ScanProgress::default())),
        };

        // Başlangıçta web korumasını etkinleştirme kontrolü
        if engine.settings.web_security.enable_malware_protection || engine.settings.web_security.enable_tracker_protection {
            info!("Initial configuration enables web protection. Activating...");
            if let Err(e) = engine.enable_web_protection().await {
                error!("Failed to activate web protection on startup: {}", e);
            }
        }
        Ok(engine)
    }

    /// Anlık durum metriklerini toplar.
    pub async fn get_current_status(&self) -> Result<EngineStatus> {
        let q_stats = self.quarantine.get_stats(Path::new(&self.settings.quarantine_dir)).await?;
        Ok(EngineStatus {
            is_scanning: self.is_scanning.load(Ordering::Relaxed),
            files_scanned: self.files_scanned.load(Ordering::Relaxed),
            threats_found: self.threats_found.load(Ordering::Relaxed),
            quarantined_count: q_stats.count,
            is_web_protection_active: self.is_web_protection_active().await,
        })
    }

    // ---------------------------------------------------------------------
    // TARAYICI FONKSİYONLARI
    // ---------------------------------------------------------------------

    /// “Hızlı tarama”: kritik dizinleri çabucak kontrol eder.
    pub async fn run_quick_scan(&self) -> Result<()> {
        self.is_scanning.store(true, Ordering::Relaxed);
        info!("⚡ Quick Scan initiated.");

        // Örnek – test klasörleri (gerçek uygulamada SystemRoot vb. kullanın)
        let critical_paths = vec![
            "C:\\AIAV_Test\\TempA".to_string(),
            "C:\\AIAV_Test\\TempB".to_string(),
        ];

        for path_str in critical_paths {
            if path_str.is_empty() {
                continue;
            }
            let p = Path::new(&path_str);
            if p.exists() && p.is_dir() {
                self.run_path_scan(&path_str).await?;
            } else {
                warn!("Quick scan path missing or not a dir: {path_str}");
            }
        }

        self.is_scanning.store(false, Ordering::Relaxed);
        info!("⚡ Quick Scan finished.");
        Ok(())
    }

    /// Verilen dizindeki tüm dosyaları ayrıntılı tarar.
    pub async fn run_path_scan(&self, path_to_scan: &str) -> Result<()> {
        let quarantine_path = Path::new(&self.settings.quarantine_dir);
        let scan_path = Path::new(path_to_scan);

        // Karantina dizini içinde kendi kendini taramayı engelle.
        if scan_path.starts_with(quarantine_path) {
            warn!("Scan path is inside quarantine dir; skipping: {path_to_scan}");
            return Ok(());
        }

        // Toplam dosya sayısını blok-thread’de sayarak ilerleme çubuğu sağlar.
        let total_files = task::spawn_blocking({
            let path_buf = PathBuf::from(path_to_scan);
            move || {
                WalkDir::new(path_buf)
                    .into_iter()
                    .filter_map(Result::ok)
                    .filter(|e| e.file_type().is_file())
                    .count() as u64
            }
        })
        .await?;

        // İlerleme reset
        {
            let mut prog = self.scan_progress.lock().await;
            *prog = ScanProgress {
                is_running: true,
                current_file: "Scan starting...".into(),
                total_files,
                scanned_files: 0,
            };
        }

        info!("🔁 Comprehensive scan on: {path_to_scan}");
        for entry in WalkDir::new(path_to_scan).into_iter().filter_map(Result::ok) {
            if !entry.file_type().is_file() {
                continue;
            }

            // İlerleme güncelle
            {
                let mut prog = self.scan_progress.lock().await;
                prog.current_file = entry.path().to_string_lossy().to_string();
            }

            // Dosyayı tara
            self.scan_and_handle(entry.path()).await;

            // Tarama sonrası sayacı artır
            {
                let mut prog = self.scan_progress.lock().await;
                prog.scanned_files += 1;
            }
        }

        // Bitti
        {
            let mut prog = self.scan_progress.lock().await;
            prog.is_running = false;
        }
        Ok(())
    }

    /// Tekil dosyayı tarar; tehditse karantinaya alır.
    pub async fn scan_and_handle(&self, path: &Path) {
        if !path.is_file() {
            return;
        }

        // Karantinada zaten UUID’li dosyayı atla.
        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            if file_name.len() >= 36 && Uuid::parse_str(&file_name[..36]).is_ok() {
                debug!("Skip quarantined file: {}", path.display());
                return;
            }
        }

        // Hariç tutulan uzantı kontrolü
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            if self
                .settings
                .excluded_extensions
                .iter()
                .any(|e| ext.eq_ignore_ascii_case(e.trim_start_matches('.')))
            {
                debug!(file = %path.display(), "Extension excluded; skipping.");
                return;
            }
        }

        self.files_scanned.fetch_add(1, Ordering::Relaxed);
        info!(file = %path.display(), "Scan starting...");

        match self.scanner.scan_file(path).await {
            Ok(scan_result) => {
                if let ThreatStatus::ThreatDetected { signature } = scan_result.status {
                    self.threats_found.fetch_add(1, Ordering::Relaxed);
                    warn!(file = %path.display(), %signature, "❗ THREAT DETECTED!");

                    let q_dir = Path::new(&self.settings.quarantine_dir);
                    if let Err(e) = self.quarantine.quarantine_file(path, &signature, q_dir).await {
                        error!(file = %path.display(), %e, "Failed to quarantine file!");
                    } else {
                        info!(file = %path.display(), "File quarantined.");
                    }
                }
            }
            Err(e) => error!(file = %path.display(), %e, "Scan error."),
        }
    }

    // ---------------------------------------------------------------------
    // KARANTİNA YÖNETİMİ
    // ---------------------------------------------------------------------

    pub async fn get_quarantine_stats(&self) -> Result<QuarantineStats> {
        self.quarantine
            .get_stats(Path::new(&self.settings.quarantine_dir))
            .await
    }

    pub async fn delete_quarantined_file(&self, filename: &str) -> Result<()> {
        self.quarantine
            .delete_file(filename, Path::new(&self.settings.quarantine_dir))
            .await
    }

    pub async fn restore_quarantined_file(&self, filename: &str) -> Result<()> {
        self.quarantine
            .restore_file(filename, Path::new(&self.settings.quarantine_dir))
            .await
    }

    // ---------------------------------------------------------------------
    // AYAR GÜNCELLEME
    // ---------------------------------------------------------------------

    pub async fn update_and_save_settings(&mut self, new_settings: Settings) -> Result<()> {
        new_settings.save(Path::new("config/default.toml")).await?;
        self.settings = new_settings;
        info!("Settings saved. Re-applying web protection...");

        let should_be_active = self.settings.web_security.enable_malware_protection 
            || self.settings.web_security.enable_tracker_protection
            || !self.settings.web_security.user_blacklist.is_empty();
            
        if should_be_active {
            self.enable_web_protection().await?;
        } else {
            self.disable_web_protection().await?;
        }
        Ok(())
    }

    // ---------------------------------------------------------------------
    // WEB KORUMA API’si
    // ---------------------------------------------------------------------

    pub async fn enable_web_protection(&self) -> Result<()> {
        let mut ws_guard = self.web_security.lock().await;
        ws_guard.start_protection(&self.settings.web_security).await
    }

    // DÜZELTME: `block_on` kaldırıldı, fonksiyon artık tam asenkron.
    pub async fn disable_web_protection(&self) -> Result<()> {
        let mut ws_guard = self.web_security.lock().await;
        ws_guard.stop_protection()
    }

    // DÜZELTME: `block_on` kaldırıldı, fonksiyon artık tam asenkron.
    pub async fn is_web_protection_active(&self) -> bool {
        self.web_security.lock().await.is_protection_active()
    }
}
