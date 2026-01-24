#!/bin/bash

# Define directories
CRATES_DIR="crates"

# Output header
echo "========================================================"
echo "📊 Line of Code Analysis (LOC)"
echo "========================================================"
echo ""

# Function to count lines
count_lines() {
    local name=$1
    local pattern=$2
    local files=$(find "$CRATES_DIR" -name "$pattern" -not -path "*/target/*" -not -path "*/node_modules/*")

    if [ -z "$files" ]; then
        echo "0"
        return
    fi

    # Use wc -l, handle potential errors if no files found
    # We pipe to awk to sum the first column
    echo "$files" | xargs wc -l 2>/dev/null | tail -n 1 | awk '{print $1}'
}

# 1. Implementation Code
# Rust source files excluding test files and tests directories
# This is an approximation. A robust tool like tokei is better but not available.
# We assume files not named *test* and not in tests/ folders are implementation.
# However, many Rust files contain inline tests. We will count all .rs files as "Total Rust"
# and then try to approximate.

# Let's separate by "Source" vs "Tests" based on directory/filename convention.
# Test files: tests/*.rs, *_test.rs, or inside tests/ modules.

echo "--- Breakdown by Language ---"

# Rust
RUST_TOTAL=$(count_lines "Rust" "*.rs")
echo "Rust (.rs): $RUST_TOTAL lines"

# Go
GO_TOTAL=$(count_lines "Go" "*.go")
echo "Go (.go):   $GO_TOTAL lines"

# Web (JS/HTML/CSS)
JS_TOTAL=$(count_lines "JS" "*.js")
HTML_TOTAL=$(count_lines "HTML" "*.html")
CSS_TOTAL=$(count_lines "CSS" "*.css")
WEB_TOTAL=$((JS_TOTAL + HTML_TOTAL + CSS_TOTAL))
echo "Web (JS/HTML/CSS): $WEB_TOTAL lines"

# Shell Scripts
SH_TOTAL=$(find scripts -name "*.sh" -o -name "*.ps1" | xargs wc -l 2>/dev/null | tail -n 1 | awk '{print $1}')
echo "Scripts: $SH_TOTAL lines"

TOTAL_LOC=$((RUST_TOTAL + GO_TOTAL + WEB_TOTAL + SH_TOTAL))

echo ""
echo "--- Breakdown by Category (Approximate) ---"

# Test Code:
# 1. Files in `tests/` directories
# 2. Files named `*test*.rs`
TEST_FILES=$(find "$CRATES_DIR" \( -path "*/tests/*" -o -name "*test*.rs" \) -name "*.rs" -not -path "*/target/*")
TEST_LOC=$(echo "$TEST_FILES" | xargs wc -l 2>/dev/null | tail -n 1 | awk '{print $1}')
if [ -z "$TEST_LOC" ]; then TEST_LOC=0; fi

# Implementation Code: Total Rust - Test LOC
# This is imperfect because unit tests in src/lib.rs are counted as impl,
# and impl code in tests/ is counted as tests. But it's a standard approximation.
IMPL_RUST_LOC=$((RUST_TOTAL - TEST_LOC))

# Go tests usually `*_test.go`
GO_TEST_FILES=$(find "$CRATES_DIR" -name "*_test.go")
GO_TEST_LOC=$(echo "$GO_TEST_FILES" | xargs wc -l 2>/dev/null | tail -n 1 | awk '{print $1}')
if [ -z "$GO_TEST_LOC" ]; then GO_TEST_LOC=0; fi
IMPL_GO_LOC=$((GO_TOTAL - GO_TEST_LOC))

TOTAL_TEST_LOC=$((TEST_LOC + GO_TEST_LOC))
TOTAL_IMPL_LOC=$((IMPL_RUST_LOC + IMPL_GO_LOC + WEB_TOTAL + SH_TOTAL))

echo "Implementation (Rust+Go+Web+Scripts): $TOTAL_IMPL_LOC lines"
echo "Test Code (Rust+Go explicit test files): $TOTAL_TEST_LOC lines"

echo ""
echo "========================================================"
echo "TOTAL LINES OF CODE: $TOTAL_LOC"
echo "========================================================"
