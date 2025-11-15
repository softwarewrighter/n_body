#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}🚀 N-Body Simulation Development Mode${NC}"
echo ""

# Check for --clean flag
if [ "$1" = "--clean" ] || [ "$1" = "-c" ]; then
    echo -e "${YELLOW}🧹 Cleaning build artifacts first...${NC}"
    ./scripts/clean.sh
    echo ""
fi

# Run build first
echo -e "${BLUE}📦 Building WASM module...${NC}"
./scripts/build.sh

if [ $? -eq 0 ]; then
    echo ""
    echo -e "${BLUE}🌐 Starting development server...${NC}"
    echo -e "${YELLOW}💡 Note: HTML/CSS changes in www/ are served directly${NC}"
    echo -e "${YELLOW}   If browser caching issues occur, restart the server${NC}"
    echo -e "${YELLOW}   or use: ./scripts/dev.sh --clean${NC}"
    echo ""
    ./scripts/serve.sh
else
    echo -e "${RED}❌ Build failed! Please fix errors before starting the server.${NC}"
    exit 1
fi