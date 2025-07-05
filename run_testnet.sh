#!/bin/bash

echo "🚀 Starting X1 RPC Proxy Server..."
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Rust/Cargo not found"
    echo "Please install Rust first: https://rustup.rs/"
    exit 1
fi

# Check if Solana CLI is installed
if ! command -v solana &> /dev/null; then
    echo "❌ Error: Solana CLI not found"
    echo "Please install Solana CLI first:"
    echo "sh -c \"$(curl -sSfL https://release.anza.xyz/stable/install)\""
    exit 1
fi

# Display current configuration
echo "📋 Current Solana Configuration:"
solana config get

echo ""
echo "📦 Building project..."
cargo build --release

# Check if we need sudo for port 80
PORT=80
USE_SUDO=false

echo ""
echo "🔍 Checking port $PORT permissions..."

# Test if we can bind to port 80 without sudo
if [ "$PORT" -lt 1024 ]; then
    echo "⚠️  Port $PORT requires elevated privileges"
    
    # Check if we're already running as root
    if [ "$EUID" -eq 0 ]; then
        echo "✅ Running as root, no sudo needed"
        USE_SUDO=false
    else
        echo "🔐 Will use sudo to bind to port $PORT"
        USE_SUDO=true
        
        # Test sudo access
        if ! sudo -n true 2>/dev/null; then
            echo "🔑 Please enter your password for sudo access:"
            sudo -v
            if [ $? -ne 0 ]; then
                echo "❌ Error: sudo access required for port $PORT"
                exit 1
            fi
        fi
    fi
else
    echo "✅ Port $PORT doesn't require elevated privileges"
fi

echo ""
echo "🏃 Starting X1 RPC Proxy Server..."
echo "Service will run at: http://localhost:$PORT"
echo "Target cluster: https://rpc.testnet.x1.xyz"
echo ""

# Start the service with or without sudo
if [ "$USE_SUDO" = true ]; then
    echo "🔐 Starting with elevated privileges..."
    # Preserve environment variables for cargo and solana
    sudo -E env PATH="$PATH" HOME="$HOME" USER="$USER" cargo run --release -- \
        --port $PORT \
        --cluster-url https://rpc.testnet.x1.xyz \
        --health-check-interval 30 \
        --rpc-timeout 10
else
    cargo run --release -- \
        --port $PORT \
        --cluster-url https://rpc.testnet.x1.xyz \
        --health-check-interval 30 \
        --rpc-timeout 10
fi 