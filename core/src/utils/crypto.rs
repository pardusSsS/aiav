use anyhow::Result;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// Verilen dosyanın SHA-256 karmasını hex olarak döndürür.
pub fn sha256_file<P: AsRef<Path>>(path: P) -> Result<String> {
    let mut file = BufReader::new(File::open(&path)?);
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 8192];

    loop {
        let n = file.read(&mut buf)?;
        if n == 0 { break }
        hasher.update(&buf[..n]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}
