//! web_security.rs
//! AI-Antivirus: Hosts tabanlı site engelleme modülü (güncellenmiş)

use anyhow::{anyhow, Context, Result};
use tracing::{info, warn};
use crate::config::settings::WebSecuritySettings;
use reqwest::Client;
use std::{
    collections::HashSet,
    fs,
    path::Path,
    process::Command,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::time::sleep;
use serde::{Deserialize, Serialize};
use url::Url;
use std::os::windows::process::CommandExt;

// --- Sabitler ---
const HOSTS_PATH: &str = r"C:\Windows\System32\drivers\etc\hosts";
const BLOCK_IP: &str = "0.0.0.0";
const MARKER_START: &str = "# AI-ANTIVIRUS BLOCKLIST START";
const MARKER_END: &str = "# AI-ANTIVIRUS BLOCKLIST END";

const MALWARE_HOSTS_CACHE: &str = "config/signatures/malware_hosts.txt";
const TRACKER_HOSTS_CACHE: &str = "config/signatures/tracker_hosts.txt";
const CACHE_DURATION_HOURS: u64 = 24;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CacheMetadata {
    last_updated: u64,
    source_url: String,
}

#[derive(Clone)]
pub struct WebSecurity {
    http_client: Client,
}

impl WebSecurity {
    pub fn new() -> Result<Self> {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(120))
            .connect_timeout(Duration::from_secs(20))
            .build()?;
        
        if let Some(parent) = Path::new(MALWARE_HOSTS_CACHE).parent() {
            fs::create_dir_all(parent)?;
        }

        Ok(Self { http_client })
    }

    fn is_cache_expired(&self, last_updated: u64) -> bool {
        let current = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        if last_updated == 0 { return true; }
        current.saturating_sub(last_updated) > (CACHE_DURATION_HOURS * 3600)
    }

    /// Bir dosyadaki listeyi günceller (cache kontrolü ile).
    async fn update_list_if_needed(&self, local_path: &str, url: &str) -> Result<()> {
        let metadata_path = format!("{}.meta.json", local_path);
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        let needs_update = match fs::read_to_string(&metadata_path) {
            Ok(content) => {
                let metadata: CacheMetadata = serde_json::from_str(&content)?;
                metadata.source_url != url || self.is_cache_expired(metadata.last_updated)
            },
            Err(_) => true, // Metadata yoksa her zaman güncelle
        };

        if needs_update {
            info!("Cache for {} is stale; downloading from {}", local_path, url);
            match self.download_with_retry(url).await {
                Ok(domains_str) => {
                    fs::write(local_path, &domains_str)?;
                    let new_metadata = CacheMetadata { last_updated: now, source_url: url.to_string() };
                    fs::write(metadata_path, serde_json::to_string(&new_metadata)?)?;
                    info!("Successfully updated local cache for {}", local_path);
                },
                Err(e) => warn!("Unable to refresh {}: {}. Using stale cache if available.", local_path, e),
            }
        } else {
            info!("Using cached list for {} (fresh).", local_path);
        }
        Ok(())
    }

    /// Tüm listeleri, kullanıcının ayarlarına göre birleştirir.
    pub async fn gather_all_domains(&self, settings: &WebSecuritySettings) -> Result<HashSet<String>> {
        info!("Gathering and normalizing domains based on current settings...");
        
        // Dinamik listeleri (eğer cache süresi dolmuşsa) güncelle
        for url in &settings.blocklist_urls {
            let local_cache_file = if url.contains("StevenBlack") { MALWARE_HOSTS_CACHE } else { TRACKER_HOSTS_CACHE };
            self.update_list_if_needed(local_cache_file, url).await?;
        }

        let mut final_blocklist = HashSet::new();

        if settings.enable_malware_protection {
            info!("Malware protection is enabled. Loading malware lists.");
            final_blocklist.extend(Self::load_list_from_file(MALWARE_HOSTS_CACHE)?);
            final_blocklist.extend(self.normalize_domains(&settings.user_blacklist));
        }
        
        if settings.enable_tracker_protection {
            info!("Tracker protection is enabled. Loading tracker lists.");
            final_blocklist.extend(Self::load_list_from_file(TRACKER_HOSTS_CACHE)?);
        }
        
        let user_whitelist = self.normalize_domains(&settings.user_whitelist);
        for domain in &user_whitelist {
            final_blocklist.remove(domain);
        }

        info!("Total unique domains to block: {}", final_blocklist.len());
        Ok(final_blocklist)
    }
    
    /// Engelleme listesini `hosts` dosyasına uygular.
    pub async fn apply_blocks(&self, settings: &WebSecuritySettings) -> Result<()> {
        let final_blocklist = self.gather_all_domains(settings).await?;
        // Eğer hiçbir koruma seçili değilse ve özel engelleme listesi boşsa, blokları kaldır.
        if final_blocklist.is_empty() {
            self.remove_blocks()
        } else {
            self.update_hosts_file(Some(&final_blocklist))
        }
    }

    /// `hosts` dosyasındaki engelleri kaldırır.
    pub fn remove_blocks(&self) -> Result<()> {
        info!("Removing all custom blocks from hosts file.");
        self.update_hosts_file(None)
    }

    /// Hosts dosyasını güvenli bir şekilde güncelleyen merkezi fonksiyon.
    fn update_hosts_file(&self, blocklist: Option<&HashSet<String>>) -> Result<()> {
        let original_content = fs::read_to_string(HOSTS_PATH)?;
        
        let mut new_content_str = String::new();
        let mut in_our_block = false;
        for line in original_content.lines() {
            if line.trim() == MARKER_START { in_our_block = true; continue; }
            if line.trim() == MARKER_END { in_our_block = false; continue; }
            if !in_our_block {
                new_content_str.push_str(line);
                new_content_str.push_str("\r\n");
            }
        }
        let mut new_content_str = new_content_str.trim_end().to_string();

        if let Some(domains) = blocklist {
            if !domains.is_empty() {
                new_content_str.push_str("\r\n\r\n");
                new_content_str.push_str(MARKER_START);
                new_content_str.push_str("\r\n");
                let mut sorted_domains: Vec<_> = domains.iter().collect();
                sorted_domains.sort_unstable();
                for domain in sorted_domains {
                    new_content_str.push_str(&format!("{} {}\r\n", BLOCK_IP, domain));
                }
                new_content_str.push_str(MARKER_END);
            }
        }
        
        // Atomik yazma işlemi
        let temp_path_str = format!("{}.tmp", HOSTS_PATH);
        fs::write(&temp_path_str, new_content_str)?;
        
        if let Err(e) = fs::rename(&temp_path_str, HOSTS_PATH) {
            let _ = fs::remove_file(&temp_path_str);
            return Err(anyhow!("Failed to replace hosts file. Run as Admin. Error: {}", e));
        }

        info!("Hosts file updated successfully.");
        self.flush_dns_cache()
    }
    
    // Diğer yardımcı fonksiyonlar...
    fn flush_dns_cache(&self) -> Result<()> {
        info!("Flushing DNS cache...");
        let status = Command::new("ipconfig").arg("/flushdns").creation_flags(0x08000000).status()?;
        if status.success() { Ok(()) } else { Err(anyhow!("'ipconfig /flushdns' failed")) }
    }


    fn normalize_domains(&self, list: &[String]) -> HashSet<String> {
        list.iter()
            .filter_map(|entry| {
                let trimmed = entry.trim();
                if trimmed.is_empty() { return None; }
                if trimmed.starts_with("http") {
                    Url::parse(trimmed)
                        .ok()
                        .and_then(|u| u.host_str().map(String::from))
                } else {
                    Some(trimmed.to_string())
                }
            })
            .collect()
    }


    // -------------------------------------------------------------------------
    // Listeleri yükleme - yardımcılar
    // -------------------------------------------------------------------------
    fn load_list_from_file(path: &str) -> Result<HashSet<String>> {
        match fs::read_to_string(path) {
            Ok(content) => Ok(content
                .lines()
                .map(str::trim)
                .filter(|s| !s.is_empty() && !s.starts_with('#'))
                .map(String::from)
                .collect()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                warn!("Host list file not found: '{}'. Continuing with an empty list.", path);
                Ok(HashSet::new())
            }
            Err(e) => Err(e.into()),
        }
    }

    async fn download_with_retry(&self, url: &str) -> Result<String> {
        const MAX_RETRIES: usize = 3;
        const BACKOFF: Duration  = Duration::from_millis(800);

        for attempt in 1..=MAX_RETRIES {
            match self.http_client.get(url).send().await {
                Ok(resp) => return Ok(resp.error_for_status()?.text().await?),
                Err(e) => {
                    warn!("Attempt {}/{} failed for {}: {}", attempt, MAX_RETRIES, url, e);
                    if attempt < MAX_RETRIES {
                        sleep(BACKOFF * attempt as u32).await;
                    } else {
                        return Err(e).with_context(|| format!("GET {}", url));
                    }
                }
            }
        }
        unreachable!("loop returns/errs above")
    }


    // -------------------------------------------------------------------------
    // Diğer yardımcılar
    // -------------------------------------------------------------------------
    /// Korumanın aktif olup olmadığını hosts dosyasından anlar.
    pub fn is_protection_active(&self) -> bool {
        fs::read_to_string(HOSTS_PATH)
            .map(|c| c.contains(MARKER_START))
            .unwrap_or(false)
    }
}
