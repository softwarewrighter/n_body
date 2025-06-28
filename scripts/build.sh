#!/bin/bash
set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}🚀 Building N-Body Simulation...${NC}"

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo -e "${RED}❌ wasm-pack is not installed!${NC}"
    echo -e "${YELLOW}Please install it with: cargo install wasm-pack${NC}"
    exit 1
fi

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}❌ Error: Cargo.toml not found!${NC}"
    echo -e "${YELLOW}Please run this script from the project root directory.${NC}"
    exit 1
fi

echo -e "${GREEN}📦 Building server...${NC}"
cargo build --release -p n_body_server

echo -e "${GREEN}📦 Building WASM client...${NC}"
cd client && wasm-pack build --target web --out-dir ../server/pkg && cd ..

echo -e "${GREEN}✅ Build complete!${NC}"
echo -e "${YELLOW}📝 Next steps:${NC}"
echo -e "   1. Run ${GREEN}./scripts/serve.sh${NC} to start the server"
echo -e "   2. Open ${GREEN}http://localhost:8080${NC} in your browser"
echo ""
echo -e "${YELLOW}📊 Build artifacts:${NC}"
echo -e "   - Server binary: ${GREEN}target/release/n_body_server${NC}"
echo -e "   - WASM module: ${GREEN}server/pkg/n_body_client_bg.wasm${NC}"
echo -e "   - JavaScript bindings: ${GREEN}server/pkg/n_body_client.js${NC}"