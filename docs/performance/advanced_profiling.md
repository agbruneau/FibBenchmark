# Advanced Profiling Guide

This guide details the advanced profiling capabilities of the Fibonacci Benchmark Suite, including memory tracking and flamegraph generation.

## Tools Overview

Two primary tools provide profiling data:

1.  **`fib-profiler`**: A dedicated crate for running pre-configured profiling scenarios.
2.  **`fib-bench memory`**: A CLI command for ad-hoc memory analysis of specific calculations.

## Memory Tracking

We implemented a custom `TrackingAllocator` that wraps the system allocator to provide precise heap usage statistics.

### Using `fib-bench memory`

Analyze the memory footprint of a specific calculation:

```bash
# Analyze recursive memoization for n=1000
cargo run -p fib-cli -- memory -n 1000 -m memo

# Result:
# 📊 Statistics:
#   Total Allocations: 1
#   Net Bytes Leaked/Held: 0
```

### Understanding the Output

- **Total Allocations**: The number of times the allocator was called.
- **Net Bytes**: The difference between allocated and deallocated bytes at the end of execution. A value of 0 usually means all memory was clean up (dropped), which is expected for Rust.

## Flamegraph Generation (Unix Only)

On Unix systems (Linux/macOS), the `fib-profiler` can automatically generate a flamegraph to visualize CPU time distribution.

### Prerequisites

- A Unix-like environment (Linux or WSL).
- `perf` installed (on Linux).

### Generating a Flamegraph

Run the profiler:

```bash
cargo run --release -p fib-profiler
```

If successful, a `flamegraph.svg` file will be created in the current directory. Open this file in a web browser to explore the call stack performance.

**Note for Windows Users**: This feature is disabled on Windows. The tool will print a notification and skip flamegraph generation.

## Architecture

### `TrackingAllocator`

Located in `crates/fib-core/src/allocator.rs`, this struct implements the `GlobalAlloc` trait. It uses atomic counters to thread-safely track:

- Current allocated bytes.
- Total allocation count.

### `fib-profiler`

Located in `crates/fib-profiler`, this tool sets the `TrackingAllocator` as the `#[global_allocator]` for the binary, enabling it to measure its own memory usage.
