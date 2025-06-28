#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${GREEN}🌐 Starting N-Body Simulation Server...${NC}"

# Check if server binary exists
if [ ! -f "target/release/n_body_server" ]; then
    echo -e "${YELLOW}⚠️  Warning: Server binary not found!${NC}"
    echo -e "${YELLOW}Run ${GREEN}./scripts/build.sh${NC} first to build the server.${NC}"
    echo ""
    read -p "Would you like to build now? (y/N) " -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        ./scripts/build.sh
        if [ $? -ne 0 ]; then
            echo -e "${RED}❌ Build failed!${NC}"
            exit 1
        fi
    else
        exit 1
    fi
fi

echo -e "${GREEN}🚀 Starting server...${NC}"
echo -e "${BLUE}📍 URL: ${GREEN}http://localhost:8080${NC}"
echo -e "${YELLOW}📝 Server will use all available CPU cores for physics computation${NC}"
echo -e "${YELLOW}📝 Press Ctrl+C to stop the server${NC}"
echo ""

# Run the server
RUST_LOG=info ./target/release/n_body_server