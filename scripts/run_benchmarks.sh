#!/bin/bash
# Fibonacci Benchmark Suite - Run All Benchmarks
# Bash Script for Unix/Linux/macOS

set -e

FILTER="${1:-}"
SAVE_BASELINE="${2:-}"

echo "🔬 Fibonacci Benchmark Suite"
echo "============================="
echo ""

# Create timestamp for report directory
TIMESTAMP=$(date +"%Y-%m-%d_%H-%M-%S")
REPORT_DIR="reports/$TIMESTAMP"
CRITERION_DIR="$REPORT_DIR/criterion"

echo "📁 Creating report directory: $REPORT_DIR"
mkdir -p "$CRITERION_DIR"

# Build release first
echo "🔨 Building in release mode..."
cargo build --release

# Run benchmarks
echo "📊 Running benchmarks..."
BENCH_ARGS=""
if [ -n "$FILTER" ]; then
    BENCH_ARGS="-- $FILTER"
fi
if [ "$SAVE_BASELINE" = "--save-baseline" ]; then
    BENCH_ARGS="$BENCH_ARGS --save-baseline main"
fi

cargo bench $BENCH_ARGS

# Copy Criterion reports
echo "📋 Copying Criterion reports..."
if [ -d "target/criterion/report" ]; then
    cp -r target/criterion/report/* "$CRITERION_DIR/"
    echo "   ✓ Criterion reports copied to $CRITERION_DIR"
else
    echo "   ⚠ No Criterion reports found"
fi

echo ""
echo "✅ Benchmarks complete!"
echo "   Reports saved to: $REPORT_DIR"
