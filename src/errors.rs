//! Error types for QRNG API

use std::fmt;

pub type Result<T> = std::result::Result<T, QRNGError>;

#[derive(Debug)]
pub enum QRNGError {
    Authentication(String),
    RateLimit(String),
    QuotaExceeded(String),
    Network(reqwest::Error),
    Json(serde_json::Error),
    Api { message: String, status: u16 },
}

impl fmt::Display for QRNGError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QRNGError::Authentication(msg) => write!(f, "Authentication error: {}", msg),
            QRNGError::RateLimit(msg) => write!(f, "Rate limit: {}", msg),
            QRNGError::QuotaExceeded(msg) => write!(f, "Quota exceeded: {}", msg),
            QRNGError::Network(err) => write!(f, "Network error: {}", err),
            QRNGError::Json(err) => write!(f, "JSON error: {}", err),
            QRNGError::Api { message, status } => {
                write!(f, "API error ({}): {}", status, message)
            }
        }
    }
}

impl std::error::Error for QRNGError {}

impl From<reqwest::Error> for QRNGError {
    fn from(err: reqwest::Error) -> Self {
        QRNGError::Network(err)
    }
}

impl From<serde_json::Error> for QRNGError {
    fn from(err: serde_json::Error) -> Self {
        QRNGError::Json(err)
    }
}
