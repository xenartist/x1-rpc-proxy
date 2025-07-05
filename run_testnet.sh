#!/bin/bash

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust/Cargo not found, please install Rust first"
    exit 1
fi

# Check if Solana CLI is installed
if ! command -v solana &> /dev/null; then
    echo "Error: Solana CLI not found, please install Solana CLI first"
    exit 1
fi

# Show current solana configuration
echo "Current Solana configuration:"
solana config get

# Build the project
echo "Building project..."
cargo build --release

# Start proxy server on port 80 for production
echo "Starting X1 RPC proxy server on port 80..."
cargo run --release -- \
    --port 80 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 