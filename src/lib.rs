//! QRNG API - Rust SDK
//!
//! Official Rust client library for the QRNG API

pub mod client;
pub mod errors;
pub mod types;

pub use client::QRNGClient;
pub use errors::{QRNGError, Result};
pub use types::{EntropyResult, HealthStatus, GenerateOptions};
