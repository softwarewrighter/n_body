#!/bin/bash
set -e

echo "Building WASM module..."

# Build the WASM module
wasm-pack build --target web --out-dir pkg

echo "Build complete! Open index.html in a web server to run the simulation."
echo "You can use: python3 -m http.server 8000"