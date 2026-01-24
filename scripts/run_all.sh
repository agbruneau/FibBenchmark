#!/bin/bash
# Fibonacci Benchmark Suite - Run All
# Bash Script for Unix/Linux/macOS
# Orchestrator script that runs benchmarks and generates reports

set -e

FILTER="${1:-}"
SKIP_BENCHMARKS="${2:-}"
SKIP_REPORT="${3:-}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "🚀 Fibonacci Benchmark Suite - Full Run"
echo "========================================"
echo ""

# Create shared timestamp for this run
TIMESTAMP=$(date +"%Y-%m-%d_%H-%M-%S")
REPORT_DIR="reports/$TIMESTAMP"

echo "📅 Run timestamp: $TIMESTAMP"
echo "📁 Report directory: $REPORT_DIR"
echo ""

# Step 1: Run benchmarks
if [ "$SKIP_BENCHMARKS" != "--skip-benchmarks" ]; then
    echo "═══════════════════════════════════════"
    echo "Step 1/2: Running Benchmarks"
    echo "═══════════════════════════════════════"
    "$SCRIPT_DIR/run_benchmarks.sh" "$FILTER"
    echo ""
fi

# Step 2: Generate report
if [ "$SKIP_REPORT" != "--skip-report" ]; then
    echo "═══════════════════════════════════════"
    echo "Step 2/2: Generating Report"
    echo "═══════════════════════════════════════"
    "$SCRIPT_DIR/generate_report.sh" "$REPORT_DIR"
    echo ""
fi

echo "═══════════════════════════════════════"
echo "🎉 All tasks complete!"
echo "═══════════════════════════════════════"
echo ""
echo "Reports available at:"
echo "   $REPORT_DIR"
echo ""
echo "To view Criterion HTML report:"
echo "   open $REPORT_DIR/criterion/index.html"
