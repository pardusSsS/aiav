use crate::error::AvError;
use std::{collections::HashSet, fs, path::Path, result::Result};
use tracing::info;
use yara::{Compiler, Rules};

/// Wrapper for yara::Rules to implement Debug
pub struct DebuggableRules(Rules);

impl std::fmt::Debug for DebuggableRules {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rules").finish() // Placeholder implementation
    }
}

/// SHA‑256 + YARA tabanlı tespit motoru
#[derive(Debug)]
pub struct SignatureDetector {
    hash_db:    HashSet<String>,
    yara_rules: DebuggableRules,
}

impl SignatureDetector {
    /// İmza veritabanı + YARA kural setini yükle.
    pub async fn load(db_path: &str) -> Result<Self, AvError> {
        // 1) Hash DB yükle
        let hash_db: HashSet<String> = fs::read_to_string(db_path)?
            .lines()
            .map(|l| l.trim().to_ascii_lowercase())
            .collect();

        // 2) YARA kurallarını derle
        let yara_rules = tokio::task::spawn_blocking(|| -> Result<Rules, AvError> {            let c = Compiler::new()?                                         // yara::Error → AvError
                .add_rules_file("config/signatures/malware-patterns.yar")?   // yara::Error → AvError
                .compile_rules()?;                                           // yara::Error → AvError
            Ok(c)// yara::Error → AvError
        })
        .await??;   // 1. ? = JoinError→AvError, 2. ? = YaraError→AvError

        info!(
            hashes = hash_db.len(),
            rules  = yara_rules.get_rules().len(),
            "Signature DB ve YARA kuralları yüklendi"
        );

        Ok(Self {
            hash_db,
            yara_rules: DebuggableRules(yara_rules),
        })
    }

    pub fn yara_match<P: AsRef<Path>>(&self, path: P) -> Result<bool, AvError> {
        Ok(!self.yara_rules.0.scan_file(path.as_ref(), 5)? // yara::Error → AvError
            .is_empty())
    }

        #[inline]
        pub fn is_malicious_hash(&self, sha256: &str) -> bool {
            self.hash_db.contains(&sha256.to_ascii_lowercase())
        }
}


