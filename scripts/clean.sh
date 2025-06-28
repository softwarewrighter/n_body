#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${YELLOW}🧹 Cleaning build artifacts...${NC}"

# Remove Rust build artifacts
if [ -d "target" ]; then
    echo -e "  Removing ${RED}target/${NC} directory..."
    rm -rf target
fi

# Remove WASM build artifacts
if [ -d "pkg" ]; then
    echo -e "  Removing ${RED}pkg/${NC} directory..."
    rm -rf pkg
fi

# Remove Cargo.lock if requested
if [ "$1" == "--all" ]; then
    if [ -f "Cargo.lock" ]; then
        echo -e "  Removing ${RED}Cargo.lock${NC}..."
        rm -f Cargo.lock
    fi
fi

echo -e "${GREEN}✅ Clean complete!${NC}"

if [ "$1" != "--all" ]; then
    echo -e "${YELLOW}💡 Tip: Use ${GREEN}./scripts/clean.sh --all${NC} to also remove Cargo.lock${NC}"
fi