#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

PORT=${1:-8000}

echo -e "${GREEN}🌐 Starting N-Body Simulation Server...${NC}"

# Check if index.html exists
if [ ! -f "index.html" ]; then
    echo -e "${RED}❌ Error: index.html not found!${NC}"
    echo -e "${YELLOW}Please run this script from the project root directory.${NC}"
    exit 1
fi

# Check if pkg directory exists
if [ ! -d "pkg" ]; then
    echo -e "${YELLOW}⚠️  Warning: pkg directory not found!${NC}"
    echo -e "${YELLOW}Run ${GREEN}./scripts/build.sh${NC} first to build the WASM module.${NC}"
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

echo -e "${GREEN}🚀 Starting server on port ${PORT}...${NC}"
echo -e "${BLUE}📍 URL: ${GREEN}http://localhost:${PORT}${NC}"
echo -e "${YELLOW}📝 Press Ctrl+C to stop the server${NC}"
echo ""

# Try to use Python 3 first, then Python 2
if command -v python3 &> /dev/null; then
    echo -e "${GREEN}Using Python 3 HTTP server${NC}"
    python3 -m http.server ${PORT}
elif command -v python &> /dev/null; then
    echo -e "${GREEN}Using Python 2 SimpleHTTPServer${NC}"
    python -m SimpleHTTPServer ${PORT}
else
    echo -e "${RED}❌ Error: Python is not installed!${NC}"
    echo -e "${YELLOW}Please install Python to run the development server.${NC}"
    echo -e "${YELLOW}Alternatively, you can use any other HTTP server.${NC}"
    exit 1
fi