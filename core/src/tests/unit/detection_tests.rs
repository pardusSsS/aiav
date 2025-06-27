use aiav_core::detection::signature::SignatureDetector;
use std::{fs, path::Path};

#[tokio::test]
async fn hash_positive_negative() {
    // Sahte DB – tek satır
    let tmp = tempfile::NamedTempFile::new().unwrap();
    fs::write(tmp.path(), "e3b0c44298fc1c149afbf4c8996fb924")   // boş string hash
        .unwrap();

    let sig = SignatureDetector::load(tmp.path().to_str().unwrap()).await.unwrap();
    assert!(sig.is_malicious_hash("e3b0c44298fc1c149afbf4c8996fb924"));
    assert!(!sig.is_malicious_hash("ffffffffffffffffffffffffffffffff"));
}
