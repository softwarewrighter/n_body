#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${GREEN}🚀 N-Body Simulation Development Mode${NC}"
echo ""

# Run build first
echo -e "${BLUE}📦 Building WASM module...${NC}"
./scripts/build.sh

if [ $? -eq 0 ]; then
    echo ""
    echo -e "${BLUE}🌐 Starting development server...${NC}"
    ./scripts/serve.sh
else
    echo -e "${RED}❌ Build failed! Please fix errors before starting the server.${NC}"
    exit 1
fi