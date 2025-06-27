use crate::config::settings::Settings;
use crate::detection::{ScanResult, ThreatStatus};
use crate::detection::ai::AiClient;
use anyhow::Result;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs;
use tracing::{debug, info, warn};
use walkdir::WalkDir;
use yara::{Compiler, Rules};

/// Dosya tarama işlemlerini yürüten bileşen.
pub struct Scanner {
    signature_db: HashMap<String, String>, // <SHA256 Hash, Tehdit Adı>
    ai_client:    Arc<AiClient>,
    yara_rules:   Rules,
}

impl Scanner {
    /// Yeni bir Scanner örneği oluşturur, imza veritabanını yükler ve AI istemcisini alır.
    pub async fn new(settings: Settings, ai_client: Arc<AiClient>) -> Result<Self> {
        
        // --- YENİ: Sağlamlaştırılmış Dosya Yolu Mantığı ---
        // Programın çalıştığı ana dizini bulmaya çalışalım.
        // Bu yapı, hem `cargo run` ile hem de doğrudan .exe çalıştırıldığında çalışır.
        let mut root_path = env::current_dir()?;
        if root_path.ends_with("target/release") || root_path.ends_with("target/debug") {
            if let Some(parent) = root_path.parent().and_then(|p| p.parent()) {
                root_path = parent.to_path_buf();
            }
        }
        
        info!("Project root path identified as: {}", root_path.display());
        
        // Göreli yolları, bulduğumuz kök yolla birleştirerek mutlak yollar oluştur.
        let signature_db_path = root_path.join(&settings.signature_db);
        let yara_rules_path = root_path.join("config/signatures/malware_patterns.yar");

        // --- GÜNCELLENMİŞ YÜKLEME KISMI ---
        let signature_db = Self::load_signature_db(&signature_db_path).await?;
        
        info!("Loading YARA rules from: {}", yara_rules_path.display());
        let yara_rules = Compiler::new()?
            .add_rules_file(yara_rules_path)?
            .compile_rules()?;
        info!("YARA rules loaded successfully.");

        Ok(Self { signature_db, ai_client, yara_rules })
    }

    /// İmza veritabanı dosyasını okur ve bir HashMap'e yükler.
    async fn load_signature_db(db_path: &PathBuf) -> Result<HashMap<String, String>> {
        let mut db = HashMap::new();
        if !db_path.exists() {
            warn!(path = %db_path.display(), "Signature database not found.");
            return Ok(db);
        }
        let content = fs::read_to_string(db_path).await?;
        for line in content.lines() {
            if let Some((hash, threat_name)) = line.split_once(':') {
                db.insert(hash.trim().to_lowercase(), threat_name.trim().to_string());
            }
        }
        info!("Loaded {} signatures from aof.", db.len());
        Ok(db)
    }

    /// Belirtilen bir dosya yolunu iki katmanlı olarak tarar ve sonucu döndürür.
    pub async fn scan_file(&self, path: &Path) -> Result<ScanResult> {
        // Güven Eşiği Tanımı: Sadece bu skordan daha yüksek güvene sahip AI tespitlerini ciddiye alacağız.
        const AI_CONFIDENCE_THRESHOLD: f64 = 0.80; // %80

        debug!(file = %path.display(), "Scanning file for threats...");

        let file_bytes = match fs::read(path).await {
            Ok(bytes) => bytes,
            Err(e) => return Err(anyhow::anyhow!("Failed to read file {}: {}", path.display(), e)),
        };

        let yara_matches = self.yara_rules.scan_mem(&file_bytes, 10)?;
        debug!("YARA scan completed. Matched rules: {}", yara_matches.len());

        if !yara_matches.is_empty() {
            // Eşleşen ilk kuralın adını imza olarak al
            let signature = yara_matches[0].identifier.to_string();
            return Ok(ScanResult {
                path: path.to_path_buf(),
                status: ThreatStatus::ThreatDetected { signature },
            });
        }

        // --- 1. Katman: İmza Tabanlı Tarama ---
        let hex_hash = format!("{:x}", Sha256::digest(&file_bytes));
        if let Some(signature) = self.signature_db.get(&hex_hash) {
            return Ok(ScanResult {
                path: path.to_path_buf(),
                status: ThreatStatus::ThreatDetected { signature: signature.clone() },
            });
        }
        
        // --- 2. Katman: Yapay Zeka Tabanlı Tarama ---
        info!(file = %path.display(), "File not in signature DB. Forwarding to AI engine for deep analysis...");
        match self.ai_client.predict(path).await {
            Ok(ai_response) => {
                // Hem 'malicious' flag'ini hem de güven skorunu kontrol ediyoruz.
                if ai_response.malicious && ai_response.confidence >= AI_CONFIDENCE_THRESHOLD {
                    let signature = format!("AI-Detected (Confidence: {:.2}%)", ai_response.confidence * 100.0);
                    return Ok(ScanResult {
                        path: path.to_path_buf(),
                        status: ThreatStatus::ThreatDetected { signature },
                    });
                } else {
                    // AI bir şey bulsa bile güven skoru düşükse, temiz kabul et.
                    debug!(
                        file = %path.display(), 
                        confidence = ai_response.confidence, 
                        "AI detection did not meet the confidence threshold."
                    );
                }
            }
            Err(e) => {
                warn!(error = %e, "Could not get a prediction from AI engine. Skipping AI scan.");
            }
        }

        // --- Tarama Sonu: Temiz ---
        Ok(ScanResult {
            path: path.to_path_buf(),
            status: ThreatStatus::Clean,
        })
    }
    
    /// Belirtilen bir klasör yolunu ve tüm alt klasörlerini yinelemeli olarak tarar.
    pub async fn scan_path(&self, path_to_scan: &str) -> Result<()> {
        info!(path = %path_to_scan, "Starting recursive scan on directory...");
        
        for entry in WalkDir::new(path_to_scan).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                if let Ok(scan_result) = self.scan_file(entry.path()).await {
                    if let ThreatStatus::ThreatDetected { signature } = scan_result.status {
                        // Bir tehdit bulunduğunda log basıyoruz. Karantina gibi aksiyonları
                        // bu fonksiyonu çağıran Engine gibi üst katmanlar yönetir.
                        warn!(
                            file = %entry.path().display(),
                            signature = %signature,
                            "Threat found during path scan."
                        );
                    }
                }
            }
        }
        
        info!(path = %path_to_scan, "Recursive scan finished.");
        Ok(())
    }
}