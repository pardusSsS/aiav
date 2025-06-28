//! web_security.rs
//! AI-Antivirus – Yerel DNS Sunucusu ile alan-adı engelleme modülü.

use anyhow::{anyhow, Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fs,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::Path,
    process::Command,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::{net::UdpSocket, sync::RwLock, task::JoinHandle, time::sleep};
use tracing::{error, info, warn};
use url::Url;

// ---- Hickory ------------------------------------------------------------
use hickory_resolver::{
    config::{ResolverConfig, ResolverOpts},
    name_server::TokioConnectionProvider,
    TokioResolver,
};
use hickory_server::{
    authority::MessageResponseBuilder,
    proto::{
        op::{Header, ResponseCode},
        rr::{RData, Record},
    },
    server::{Request, RequestHandler, ResponseHandler, ResponseInfo, ServerFuture},
};

// ---- Ayarlar ------------------------------------------------------------
use crate::config::settings::WebSecuritySettings;

// ---- Sabitler -----------------------------------------------------------
const LOCAL_DNS_PORT: u16 = 53;
const BLOCK_IPV4: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);

const CACHE_DURATION_HOURS: u64 = 24;
const MALWARE_HOSTS_CACHE: &str = "config/signatures/malware_hosts.txt";
const TRACKER_HOSTS_CACHE: &str = "config/signatures/tracker_hosts.txt";
const NETSH_PATH: &str = "C:\\Windows\\System32\\netsh.exe";

// ---- Yardımcı -----------------------------------------------------------
#[derive(Serialize, Deserialize)]
struct CacheMetadata {
    last_updated: u64,
    source_url: String,
}

// ========================================================================
// DNS Handler
// ========================================================================
#[derive(Clone)]
struct ForwardingDnsHandler {
    blocklist: Arc<RwLock<HashSet<String>>>,
    upstream:  TokioResolver,
}

impl ForwardingDnsHandler {
    fn new(blocklist: Arc<RwLock<HashSet<String>>>) -> Result<Self> {
        let upstream = TokioResolver::builder_with_config(
            ResolverConfig::google(),
            TokioConnectionProvider::default(),
        )
        .with_options(ResolverOpts::default())
        .build();

        Ok(Self { blocklist, upstream })
    }
}

#[async_trait::async_trait]
impl RequestHandler for ForwardingDnsHandler {
    async fn handle_request<R>(
        &self,
        request: &Request,
        mut h: R,
    ) -> ResponseInfo
    where
        R: ResponseHandler,
    {
        // ---- Sorgu ayrıştır ----
        let query = match request.queries().first() {
            Some(q) => q,
            None => {
                let mut hdr = Header::response_from_request(request.header());
                hdr.set_response_code(ResponseCode::FormErr);
                let msg =
                    MessageResponseBuilder::from_message_request(request).build_no_records(hdr);
                return h.send_response(msg)
                    .await
                    .unwrap_or_else(|_| ResponseInfo::from(hdr));
            }
        };

        let domain = query
            .name()
            .to_string()
            .trim_end_matches('.')
            .to_ascii_lowercase();

        // ---- Engelleme kontrolü ----
        if self.blocklist.read().await.contains(&domain) {
            info!("Blocking domain: {domain}");

            let mut hdr = Header::response_from_request(request.header());
            hdr.set_response_code(ResponseCode::NoError);

            let rec = Record::from_rdata(query.name().into(), 300, RData::A(BLOCK_IPV4.into()));
            let msg = MessageResponseBuilder::from_message_request(request).build(
                hdr,
                std::iter::once(&rec),
                [].iter(),
                [].iter(),
                [].iter(),
            );

            return h.send_response(msg)
                .await
                .unwrap_or_else(|_| ResponseInfo::from(hdr));
        }

        // ---- Upstream forward ----
        let builder = MessageResponseBuilder::from_message_request(request);
        let mut hdr  = Header::response_from_request(request.header());

        let res = match self.upstream.lookup_ip(query.name().clone()).await {
            Ok(ips) => {
                let answers: Vec<Record> = ips
                    .iter()
                    .map(|ip| match ip {
                     IpAddr::V4(v4) => Record::from_rdata(
                         query.name().into(),
                         60,
                         RData::A(v4.into()),
                     ),
                     IpAddr::V6(v6) => Record::from_rdata(
                         query.name().into(),
                         60,
                         RData::AAAA(v6.into()),
                     ),
                    })
                    .collect();

                let msg = builder.build(
                    hdr,
                    answers.iter(),
                    [].iter(),
                    [].iter(),
                    [].iter(),
                );
                h.send_response(msg).await
            }
            Err(e) => {
                warn!("Upstream lookup failed for {domain}: {e}");
                hdr.set_response_code(ResponseCode::ServFail);
                let msg = builder.build_no_records(hdr);
                h.send_response(msg).await
            }
        };

        res.unwrap_or_else(|_| ResponseInfo::from(hdr))
    }
}

// ========================================================================
// WebSecurity
// ========================================================================
pub struct WebSecurity {
    http: Client,
    blocklist: Arc<RwLock<HashSet<String>>>,
    dns_handle: Option<JoinHandle<()>>,
}

impl WebSecurity {
    pub fn new() -> Result<Self> {
        let http = Client::builder().timeout(Duration::from_secs(120)).build()?;

        if let Some(parent) = Path::new(MALWARE_HOSTS_CACHE).parent() {
            fs::create_dir_all(parent)?;
        }

        Ok(Self {
            http,
            blocklist: Arc::new(RwLock::new(HashSet::new())),
            dns_handle: None,
        })
    }

    // -------------------------------------------------------------------
    pub async fn start_protection(&mut self, s: &WebSecuritySettings) -> Result<()> {
        info!("Attempting to start web protection…");
        self.update_blocklist(s).await?;
        if self.dns_handle.is_some() {
            info!("DNS server is already running; blocklist updated.");
            return Ok(());
        }

        info!("Spawning local DNS server task…");
        let handler = ForwardingDnsHandler::new(self.blocklist.clone())?;
        let mut srv = ServerFuture::new(handler);
        srv.register_socket(UdpSocket::bind(("127.0.0.1", LOCAL_DNS_PORT)).await?);
        
        let server_handle = tokio::spawn(async move {
            if let Err(e) = srv.block_until_done().await {
                error!("DNS server exited: {}", e);
            }
        });

        match set_dns_to_local() {
            Ok(_) => {
                self.dns_handle = Some(server_handle);
                info!("Web protection ACTIVE.");
                Ok(())
            }
            Err(e) => {
                server_handle.abort();
                self.dns_handle = None;
                Err(e)
            }
        }
    }

    pub fn stop_protection(&mut self) -> Result<()> {
        info!("Stopping web protection…");
        if let Some(h) = self.dns_handle.take() {
            h.abort();
            info!("DNS server task stopped.");
        }
        restore_dns_auto()?;
        info!("Web protection INACTIVE.");
        Ok(())
    }

    pub fn is_protection_active(&self) -> bool {
        self.dns_handle.is_some()
    }

    // -------------------------------------------------------------------
    pub async fn update_blocklist(&self, s: &WebSecuritySettings) -> Result<()> {
        let domains = self.gather_domains(s).await?;
        *self.blocklist.write().await = domains;
        Ok(())
    }

    async fn gather_domains(&self, s: &WebSecuritySettings) -> Result<HashSet<String>> {
        for url in &s.blocklist_urls {
            let cache = if url.contains("malware") {
                MALWARE_HOSTS_CACHE
            } else {
                TRACKER_HOSTS_CACHE
            };
            self.refresh_cache_if_needed(cache, url).await?;
        }

        let mut set = HashSet::new();
        if s.enable_malware_protection {
            set.extend(Self::load(MALWARE_HOSTS_CACHE)?);
        }
        if s.enable_tracker_protection {
            set.extend(Self::load(TRACKER_HOSTS_CACHE)?);
        }
        set.extend(self.normalize(&s.user_blacklist));
        for d in self.normalize(&s.user_whitelist) {
            set.remove(&d);
        }
        Ok(set)
    }

    async fn refresh_cache_if_needed(&self, path: &str, url: &str) -> Result<()> {
        let meta = format!("{path}.meta.json");
        let now  = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        let stale = match fs::read_to_string(&meta) {
            Ok(s) => {
                let m: CacheMetadata = serde_json::from_str(&s)?;
                m.source_url != url || self.expired(m.last_updated)
            }
            Err(_) => true,
        };

        if stale {
            match self.download(url).await {
                Ok(body) => {
                    fs::write(path, &body)?;
                    let m = CacheMetadata { last_updated: now, source_url: url.into() };
                    fs::write(meta, serde_json::to_string(&m)?)?;
                }
                Err(e) => warn!("Download {url} failed: {e}"),
            }
        }
        Ok(())
    }

    async fn download(&self, url: &str) -> Result<String> {
        const MAX: usize = 3;
        for i in 1..=MAX {
            match self.http.get(url).send().await {
                Ok(r) => return Ok(r.error_for_status()?.text().await?),
                Err(e) if i < MAX => {
                    warn!("Attempt {i}/{MAX} failed: {e}; retrying");
                    sleep(Duration::from_millis(600 * i as u64)).await;
                }
                Err(e) => return Err(e.into()),
            }
        }
        unreachable!()
    }

    fn load(path: &str) -> Result<HashSet<String>> {
        match fs::read_to_string(path) {
            Ok(c) => Ok(c
                .lines()
                .filter_map(|l| {
                    let l = l.trim();
                    if l.is_empty() || l.starts_with('#') {
                        None
                    } else {
                        l.split_whitespace().nth(1).or(Some(l)).map(|d| d.to_ascii_lowercase())
                    }
                })
                .collect()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(HashSet::new()),
            Err(e) => Err(e.into()),
        }
    }

    fn expired(&self, ts: u64) -> bool {
        ts == 0
            || SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
                .saturating_sub(ts)
                > CACHE_DURATION_HOURS * 3600
    }

    fn normalize(&self, list: &[String]) -> HashSet<String> {
        list.iter()
            .filter_map(|s| {
                let t = s.trim();
                if t.is_empty() || t.starts_with('#') {
                    None
                } else if t.starts_with("http") {
                    Url::parse(t)
                        .ok()
                        .and_then(|u| u.host_str().map(|h| h.to_ascii_lowercase().to_string()))
                } else {
                    Some(t.to_ascii_lowercase())
                }
            })
            .collect()
    }
}

// ========================================================================
// Windows DNS helpers
// ========================================================================
/// YENİ FONKSİYON: Aktif ağ bağdaştırıcısını bulur.
#[cfg(target_os = "windows")]
fn get_active_interface_name() -> Result<String> {
    info!("Searching for active network interface with a default gateway...");
    let adapters = ipconfig::get_adapters().context("Failed to get network adapters.")?;
    
    // İnternete çıkan adaptör, genellikle bir ağ geçidine (gateway) sahip olandır.
    for adapter in adapters {
        if !adapter.gateways().is_empty() && adapter.oper_status() == ipconfig::OperStatus::IfOperStatusUp {
            let name = adapter.friendly_name();
            info!("Found active interface: '{}'", name);
            return Ok(name.to_owned());
        }
    }

    Err(anyhow!("No active network interface with a default gateway found."))
}

#[cfg(target_os = "windows")]
fn flush_dns_cache() -> Result<()> {
    info!("Flushing Windows DNS resolver cache...");
    let out = Command::new("ipconfig").arg("/flushdns").output().context("Failed to run 'ipconfig /flushdns'")?;
    if !out.status.success() {
        return Err(anyhow!("ipconfig /flushdns failed: {}", String::from_utf8_lossy(&out.stderr)));
    }
    Ok(())
}

#[cfg(target_os = "windows")]
fn set_dns_to_local() -> Result<()> {
    let iface = get_active_interface_name()?;
    info!("Setting DNS for '{iface}' → 127.0.0.1");

    Command::new(NETSH_PATH)
        .args([
            "interface", "ipv4", "set", "dnsservers",
            &format!("name=\"{iface}\""),
            "static", "127.0.0.1", "primary",
            "validate=no",
        ])
        .output()
        .context("netsh failed")          // I/O
        .and_then(check_netsh)?;

    info!("System DNS successfully set for '{}'", iface);
    
    // YENİ EKLEME: netsh başarılı olduktan sonra sistem önbelleğini temizle.
    flush_dns_cache()?;
    
    Ok(())
}

#[cfg(target_os = "windows")]
fn restore_dns_auto() -> Result<()> {
    let iface = get_active_interface_name()?;
    info!("Restoring DNS for '{iface}' → DHCP");

    Command::new(NETSH_PATH)
        .args([
            "interface", "ipv4", "set", "dnsservers",
            &format!("name=\"{iface}\""),
            "source=dhcp",
        ])
        .output()
        .context("netsh failed")
        .and_then(check_netsh)?;
    Ok(())
}

/// Ortak stderr / exit-code kontrolü
#[cfg(target_os = "windows")]
fn check_netsh(out: std::process::Output) -> Result<std::process::Output> {
    if out.status.success() {
        Ok(out)
    } else {
        Err(anyhow!(
            "netsh exited {}.\nSTDOUT: {}\nSTDERR: {}",
            out.status,
            String::from_utf8_lossy(&out.stdout).trim(),
            String::from_utf8_lossy(&out.stderr).trim(),
        ))
    }
}


#[cfg(not(target_os = "windows"))]
fn set_dns_to_local() -> Result<()> { Ok(()) }
#[cfg(not(target_os = "windows"))]
fn restore_dns_auto() -> Result<()> { Ok(()) }