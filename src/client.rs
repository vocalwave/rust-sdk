//! QRNG API client

use crate::errors::{QRNGError, Result};
use crate::types::{EntropyResult, HealthStatus, GenerateOptions};
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;

/// QRNG API client
pub struct QRNGClient {
    api_key: String,
    base_url: String,
    client: Client,
}

impl QRNGClient {
    /// Create a new QRNG API client
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::with_base_url(api_key, "https://qrngapi.com")
    }

    /// Create a new client with custom base URL
    pub fn with_base_url(api_key: impl Into<String>, base_url: impl Into<String>) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            api_key: api_key.into(),
            base_url: base_url.into(),
            client,
        }
    }

    /// Generate random entropy
    pub async fn generate(&self, options: GenerateOptions) -> Result<EntropyResult> {
        let mut url = format!("{}/api/random", self.base_url);
        let mut params = vec![];

        let bytes = options.bytes.unwrap_or(32);
        let format = options.format.as_deref().unwrap_or("hex");

        params.push(format!("bytes={}", bytes));
        params.push(format!("format={}", format));

        if let Some(method) = options.method {
            params.push(format!("method={}", method));
        }
        if let Some(sig_type) = options.signature_type {
            params.push(format!("signatureType={}", sig_type));
        }

        url.push('?');
        url.push_str(&params.join("&"));

        let response = self
            .client
            .get(&url)
            .header("X-API-Key", &self.api_key)
            .send()
            .await?;

        match response.status().as_u16() {
            401 => return Err(QRNGError::Authentication("Invalid API key".into())),
            429 => return Err(QRNGError::RateLimit("Rate limit exceeded".into())),
            402 => return Err(QRNGError::QuotaExceeded("Monthly quota exceeded".into())),
            200 => {
                let result: EntropyResult = response.json().await?;
                Ok(result)
            }
            status => {
                let body: Value = response.json().await?;
                let message = body["error"]
                    .as_str()
                    .unwrap_or("Unknown error")
                    .to_string();
                Err(QRNGError::Api { message, status })
            }
        }
    }

    /// Get system health status
    pub async fn health(&self) -> Result<HealthStatus> {
        let url = format!("{}/api/health", self.base_url);

        let response = self
            .client
            .get(&url)
            .header("X-API-Key", &self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(QRNGError::Api {
                message: "Health check failed".into(),
                status: response.status().as_u16(),
            });
        }

        let health: HealthStatus = response.json().await?;
        Ok(health)
    }
}
