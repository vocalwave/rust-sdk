# QRNG API - Rust SDK

Official Rust client library for the QRNG API.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
qrng-api = "1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

```rust
use qrng_api::{QRNGClient, GenerateOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = QRNGClient::new("your-api-key");
    
    let options = GenerateOptions::new()
        .bytes(32)
        .format("hex");
    
    let result = client.generate(options).await?;
    println!("Random data: {}", result.data);
    println!("Proof ID: {}", result.proof_id);
    
    Ok(())
}
```

## Features

- ✅ Async/await support with Tokio
- ✅ Strong type safety
- ✅ REST API for entropy generation
- ✅ Post-Quantum Cryptography support
- ✅ Comprehensive error handling
- ✅ Zero-cost abstractions

## Usage

### Basic Generation

```rust
use qrng_api::{QRNGClient, GenerateOptions};

let client = QRNGClient::new("your-api-key");

let options = GenerateOptions::new()
    .bytes(32)
    .format("hex");

let result = client.generate(options).await?;
println!("{}", result.data);
```

### Quantum Methods

```rust
let options = GenerateOptions::new()
    .bytes(32)
    .format("hex")
    .method("photon"); // photon, tunneling, vacuum, simulator

let result = client.generate(options).await?;
```

### Post-Quantum Signatures

```rust
// Pro tier: Dilithium2
let options = GenerateOptions::new()
    .bytes(32)
    .signature_type("dilithium2");

let result = client.generate(options).await?;

// Enterprise tier: Dilithium3/5
let options = GenerateOptions::new()
    .bytes(32)
    .signature_type("dilithium3");
```

### Health Check

```rust
let health = client.health().await?;
println!("Status: {}", health.status);
```

### Error Handling

```rust
use qrng_api::QRNGError;

match client.generate(options).await {
    Ok(result) => println!("Success: {}", result.data),
    Err(QRNGError::Authentication(msg)) => eprintln!("Auth error: {}", msg),
    Err(QRNGError::RateLimit(msg)) => eprintln!("Rate limit: {}", msg),
    Err(QRNGError::QuotaExceeded(msg)) => eprintln!("Quota: {}", msg),
    Err(e) => eprintln!("Error: {}", e),
}
```

## API Reference

### QRNGClient

#### `new(api_key: impl Into<String>) -> Self`

Create a new QRNG API client.

#### `generate(options: GenerateOptions) -> Result<EntropyResult>`

Generate random entropy asynchronously.

### GenerateOptions

Builder pattern for generation options:

```rust
GenerateOptions::new()
    .bytes(32)           // Number of bytes (1-1024)
    .format("hex")       // hex, base64, binary, uint8, uint32
    .method("photon")    // auto, photon, tunneling, vacuum, simulator
    .signature_type("dilithium2") // ed25519, dilithium2, dilithium3, dilithium5
```

## Requirements

- Rust 1.70 or higher
- Tokio runtime for async support

## License

MIT

## Support

- Documentation: https://qrngapi.com/docs
- GitHub: https://github.com/qrng-api/rust-sdk
- Email: support@qrngapi.com
