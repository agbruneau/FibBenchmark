# fib-cli

`fib-cli` is the command-line interface for the Fibonacci Benchmark Suite. It exposes all the functionality of the core library, visualization tools, and benchmarking capabilities through a unified entry point.

## Installation

```bash
cargo install --path .
```

## Usage

```bash
fib-bench [COMMAND] [OPTIONS]
```

### Commands

*   `calc`: Calculate F(n) using a specified algorithm.
*   `compare`: Compare all available algorithms for a given n.
*   `bench`: Run Criterion benchmarks.
*   `info`: Display complexity information for algorithms.
*   `sequence`: Generate a sequence of Fibonacci numbers.
*   `binet-analysis`: Analyze the precision of Binet's formula.
*   `memory`: Analyze memory usage of an algorithm.
*   `report`: Generate HTML reports from benchmark results.
*   `compare-go`: Compare Rust implementation with Go (requires Go installed).
*   `simd`: (If enabled) Run SIMD batch processing demos.

### Examples

```bash
# Calculate F(50) using fast doubling
fib-bench calc -n 50 --method fast_doubling

# Check memory usage of matrix method
fib-bench memory -n 10000 --method matrix

# Compare Rust vs Go
fib-bench compare-go -n 5000
```
