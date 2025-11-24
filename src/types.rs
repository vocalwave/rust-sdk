//! Type definitions for QRNG API

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyResult {
    pub data: String,
    #[serde(rename = "proofId")]
    pub proof_id: String,
    pub signature: String,
    #[serde(rename = "publicKey")]
    pub public_key: String,
    #[serde(rename = "signatureType")]
    pub signature_type: String,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub metrics: HashMap<String, serde_json::Value>,
    pub timestamp: String,
}

#[derive(Debug, Clone, Default)]
pub struct GenerateOptions {
    pub bytes: Option<u32>,
    pub format: Option<String>,
    pub method: Option<String>,
    pub signature_type: Option<String>,
}

impl GenerateOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bytes(mut self, bytes: u32) -> Self {
        self.bytes = Some(bytes);
        self
    }

    pub fn format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }

    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }

    pub fn signature_type(mut self, sig_type: impl Into<String>) -> Self {
        self.signature_type = Some(sig_type.into());
        self
    }
}
