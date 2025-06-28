#!/bin/bash

# N-Body Simulation Debug Mode
# Runs server with verbose logging and enables client diagnostics

cd "$(dirname "$0")/.."

echo "Starting N-Body simulation in debug mode..."
echo "Server will run with RUST_LOG=debug"
echo "Client will receive debug flag for additional diagnostics"
echo ""

# Export debug environment variables
export RUST_LOG=debug
export RUST_BACKTRACE=1
export N_BODY_DEBUG=1

# Build in debug mode if needed
echo "Building server in debug mode..."
cargo build --bin n_body_server

# Run server with debug logging
echo "Starting server with debug logging..."
echo "Logs will be written to server-debug.log"
echo ""

# Run server and tee output to both console and file
cargo run --bin n_body_server 2>&1 | tee server-debug.log