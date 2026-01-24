# fib-profiler

A profiling tool wrapper for the Fibonacci Benchmark Suite. It helps generate flamegraphs to analyze CPU usage and bottlenecks.

## Usage

```bash
cargo run --bin fib-profiler -- profile --method iterative -n 100000
```

## Requirements

*   **Linux/macOS**: `perf` or `dtrace` may be required depending on the underlying profiler backend (usually `pprof` or similar integration).
