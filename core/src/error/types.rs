use thiserror::Error;
use tokio::task::JoinError;
use yara::{Error as YaraError, errors::YaraError as YaraErrorDetail};

#[derive(Error, Debug)]
pub enum AvError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("YARA error: {0}")]
    Yara(#[from] YaraError),

    #[error("Task join error: {0}")]
    Join(#[from] JoinError),

    #[error("Network error: {0}")]
    Net(#[from] reqwest::Error),

    #[error("{0}")]
    Other(String),
}
impl AvError {
    /// Hata mesajını alır
    pub fn message(&self) -> String {
        match self {
            AvError::Io(e) => e.to_string(),
            AvError::Yara(e) => e.to_string(),
            AvError::Join(e) => e.to_string(),
            AvError::Net(e) => e.to_string(),
            AvError::Other(msg) => msg.clone(),
        }
    }

    /// Yeni bir hata mesajı oluşturur
    pub fn new(message: &str) -> Self {
        AvError::Other(message.to_string())
    }
}

// Implement conversion from YaraErrorDetail only
impl From<YaraErrorDetail> for AvError {
    fn from(error: YaraErrorDetail) -> Self {
        AvError::Yara(YaraError::Yara(error))
    }
}