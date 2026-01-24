#!/bin/bash
# Setup Go Environment for FibBenchmark (Linux/macOS)
# This script helps set up the Go + CGO environment for the Rust-Go bridge

set -e

echo "🔧 FibBenchmark - Go Setup Script"
echo "=================================="
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Check if Go is installed
echo -e "${YELLOW}📦 Checking Go installation...${NC}"
if command -v go &> /dev/null; then
    GO_VERSION=$(go version)
    echo -e "${GREEN}✅ Go found: $GO_VERSION${NC}"
else
    echo -e "${RED}❌ Go not found. Please install Go.${NC}"
    echo ""
    echo -e "${YELLOW}Installation options:${NC}"
    if [[ "$OSTYPE" == "darwin"* ]]; then
        echo "  brew install go"
    else
        echo "  sudo apt-get install golang-go  # Debian/Ubuntu"
        echo "  sudo dnf install golang         # Fedora"
        echo "  sudo pacman -S go               # Arch"
    fi
    echo "  Or download from https://golang.org/dl/"
    exit 1
fi

# Check if GCC is installed (required for CGO)
echo ""
echo -e "${YELLOW}📦 Checking GCC installation (required for CGO)...${NC}"
if command -v gcc &> /dev/null; then
    GCC_VERSION=$(gcc --version | head -n 1)
    echo -e "${GREEN}✅ GCC found: $GCC_VERSION${NC}"
else
    echo -e "${RED}❌ GCC not found. CGO requires GCC.${NC}"
    echo ""
    echo -e "${YELLOW}Installation options:${NC}"
    if [[ "$OSTYPE" == "darwin"* ]]; then
        echo "  xcode-select --install  # or"
        echo "  brew install gcc"
    else
        echo "  sudo apt-get install build-essential  # Debian/Ubuntu"
        echo "  sudo dnf groupinstall 'Development Tools'  # Fedora"
        echo "  sudo pacman -S base-devel  # Arch"
    fi
    exit 1
fi

# Check CGO_ENABLED
echo ""
echo -e "${YELLOW}📦 Checking CGO configuration...${NC}"
if [[ "${CGO_ENABLED}" == "1" ]]; then
    echo -e "${GREEN}✅ CGO_ENABLED=1${NC}"
elif [[ "${CGO_ENABLED}" == "0" ]]; then
    echo -e "${YELLOW}⚠️  CGO_ENABLED=0 (CGO is disabled)${NC}"
    echo "   To enable: export CGO_ENABLED=1"
else
    echo -e "${CYAN}ℹ️  CGO_ENABLED not set (defaults to enabled)${NC}"
fi

# Try to build the Go library
echo ""
echo -e "${YELLOW}🔨 Testing Go library build...${NC}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GO_DIR="$SCRIPT_DIR/../crates/fib-go/go"
OUT_DIR="$SCRIPT_DIR/../target/go-test"

mkdir -p "$OUT_DIR"
LIB_PATH="$OUT_DIR/libfibgo.a"

cd "$GO_DIR"
export CGO_ENABLED=1

if go build -buildmode=c-archive -o "$LIB_PATH" fib.go 2>&1; then
    echo -e "${GREEN}✅ Go library built successfully!${NC}"
    echo "   Output: $LIB_PATH"
    
    # Clean up test build
    rm -f "$LIB_PATH" "${LIB_PATH%.a}.h"
else
    echo -e "${RED}❌ Go library build failed${NC}"
fi

# Summary
echo ""
echo -e "${CYAN}📋 Summary${NC}"
echo "─────────────────────────────────────"
echo -e "${GREEN}✅ Environment is ready for native Go bridge!${NC}"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo "  1. cargo build -p fib-go"
echo "  2. cargo run --bin fib-bench -- compare-go -n 1000"
echo ""
