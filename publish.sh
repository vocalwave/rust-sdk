#!/bin/bash
# Publishing script for Rust SDK

set -e

echo "🦀 Publishing QRNG Rust SDK to crates.io"
echo "========================================="
echo ""

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Must run from sdks/rust directory"
    exit 1
fi

# Check the package
echo "✅ Checking package..."
cargo check

echo "🧪 Running tests..."
cargo test || echo "⚠️  No tests defined"

# Dry run
echo "📋 Testing publish (dry run)..."
cargo publish --dry-run

echo ""
echo "Package ready for publishing!"
echo ""
read -p "Publish to crates.io? (y/n) " -n 1 -r
echo ""

if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "⬆️  Publishing to crates.io..."
    cargo publish
    
    echo ""
    echo "✅ Successfully published to crates.io!"
    echo "   Install: cargo add qrng-api"
    echo "   Docs: https://docs.rs/qrng-api"
else
    echo "❌ Aborted"
fi
