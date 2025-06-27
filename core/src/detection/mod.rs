pub mod signature;
pub mod ai;
pub mod watcher;
pub mod web_security;
use std::path::PathBuf;

/// Tarama sonucunda bir dosyanın durumunu belirtir.
#[derive(Debug, Clone)]
pub enum ThreatStatus {
    Clean,
    ThreatDetected { signature: String },
}

/// Tek bir dosya taramasının sonucunu paketler.
#[derive(Debug, Clone)]
pub struct ScanResult {
    pub path: PathBuf,
    pub status: ThreatStatus,
}


