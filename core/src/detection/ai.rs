use anyhow::Result;
use serde::Deserialize;
use serde_json;
use std::path::Path;
use tokio::fs; // Dosyayı okumak için
use tracing::debug;

// AiRequest struct'ına artık ihtiyacımız yok, çünkü dosyanın kendisini göndereceğiz.

// DÜZELTME 1: Yanıt yapısını, sunucudan gelen gerçek yanıta göre güncelliyoruz.
#[derive(Deserialize, Debug)]
pub struct AiResponse {
    pub malicious: bool, // "prediction" yerine "malicious" (true/false)
    pub confidence: f64,
    pub sha256: Option<String>, // Bu alan bazen olmayabilir, Option<> ile güvenli hale getirelim.
}

/// AI sunucusu ile iletişim kuran istemci.
#[derive(Clone, Debug)]
pub struct AiClient {
    client: reqwest::Client,
    base_url: String,
}

impl AiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    /// Bir dosyanın binary içeriğini analiz için AI sunucusuna gönderir.
    pub async fn predict(&self, path: &Path) -> Result<AiResponse> {
        let full_url = format!("{}/v1/predict", self.base_url);

        // DÜZELTME 2: Dosyanın yolunu değil, içeriğini okuyoruz.
        let file_bytes = fs::read(path).await?;

        debug!(url = %full_url, file = %path.display(), size = file_bytes.len(), "Sending file content to AI engine...");

        // DÜZELTME 3: İsteği, JSON yerine binary data (octet-stream) olarak gönderiyoruz.
        let response = self.client
            .post(&full_url)
            .header("Content-Type", "application/octet-stream")
            .body(file_bytes)
            .send()
            .await?;

        let response_text = response.text().await?;
        debug!(response_body = %response_text, "Received raw response from AI server.");
        
        // Gelen yanıtı yeni AiResponse yapımıza göre parse ediyoruz.
        let ai_response: AiResponse = serde_json::from_str(&response_text)?;
        
        debug!(malicious = %ai_response.malicious, confidence = %ai_response.confidence, "Successfully decoded response.");

        Ok(ai_response)
    }
}