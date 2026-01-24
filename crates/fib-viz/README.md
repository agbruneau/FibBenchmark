# fib-viz

A visualization tool for the Fibonacci Benchmark Suite. It takes benchmark data and generates charts and visual reports.

## Usage

Usually invoked via the root `fib-bench` wrapper, but can be run directly:

```bash
cargo run --bin fib-viz
```

This will look for benchmark results in the `target/criterion` directory and generate HTML reports in `results/reports/`.
