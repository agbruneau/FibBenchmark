#!/bin/bash
# Fibonacci Benchmark Suite - Generate Report
# Bash Script for Unix/Linux/macOS

set -e

REPORT_DIR="${1:-}"

echo "📊 Fibonacci Report Generator"
echo "=============================="
echo ""

# Determine report directory
if [ -z "$REPORT_DIR" ]; then
    TIMESTAMP=$(date +"%Y-%m-%d_%H-%M-%S")
    REPORT_DIR="reports/$TIMESTAMP"
fi
DATA_DIR="$REPORT_DIR/data"

echo "📁 Report directory: $REPORT_DIR"
mkdir -p "$DATA_DIR"

# Build release first
echo "🔨 Building in release mode..."
cargo build --release -p fib-viz

# Run fib-viz to generate CSV data
echo "📈 Generating visualization data..."
cargo run --release -p fib-viz

# Move generated CSVs to report directory
echo "📋 Moving data files..."
if [ -d "results" ]; then
    cp -r results/* "$DATA_DIR/"
    echo "   ✓ Data files copied to $DATA_DIR"
else
    echo "   ⚠ No results directory found"
fi

echo ""
echo "✅ Report generation complete!"
echo "   Data saved to: $DATA_DIR"
echo ""
echo "Generated files:"
ls -1 "$DATA_DIR" 2>/dev/null | sed 's/^/   - /'
