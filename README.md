# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: Random selection of available nodes for request forwarding
- **Memory Caching**: Fast access to active node lists
- **RESTful API**: Health check and statistics information endpoints

## Dependencies

Make sure you have installed:
- Rust (1.70+)
- Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Usage

### Build Project

```bash
cargo build --release
```

### Start Server

```bash
cargo run --release -- --port 8080 --cluster-url https://rpc.testnet.x1.xyz
```

### Command Line Arguments

- `--port`: Proxy server listening port (default: 8080)
- `--cluster-url`: X1 cluster RPC URL (default: https://rpc.testnet.x1.xyz)
- `--health-check-interval`: Node health check interval in seconds (default: 30)
- `--rpc-timeout`: RPC request timeout in seconds (default: 10)
- `--verbose`: Enable verbose logging

### API Endpoints

- `POST /`: RPC proxy endpoint, forwards JSON-RPC requests
- `GET /health`: Health check endpoint
- `GET /stats`: Node statistics information

### Example RPC Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getHealth",
    "params": []
  }'
```

### Check Server Status

```bash
# Health check
curl http://localhost:8080/health

# Node statistics
curl http://localhost:8080/stats
```

## Project Structure

```bash:run.sh
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

# Start proxy server, connecting to X1 cluster
echo "Starting X1 RPC proxy server..."
cargo run --release -- \
    --port 8080 \
    --cluster-url https://rpc.testnet.x1.xyz \
    --health-check-interval 30 \
    --rpc-timeout 10 \
    --verbose
```

```markdown:README.md
# X1 RPC Proxy Server

This is an RPC proxy server designed for the X1 blockchain cluster that can:

1. Get the list of RPC nodes in the current X1 blockchain cluster via Solana gossip command
2. Poll and test available RPC nodes and record response speeds
3. Cache available public RPC nodes in memory
4. Provide external RPC services that randomly forward requests to available nodes

## Features

- **Dynamic Node Discovery**: Real-time discovery of RPC nodes via gossip protocol
- **Health Checking**: Periodic testing of node availability and response times
- **Load Balancing**: