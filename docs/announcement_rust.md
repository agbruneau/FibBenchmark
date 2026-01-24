# r/rust Announcement Draft - FibBenchmark SIMD

---

## Post Title

**[Project] FibBenchmark v1.1.0: Comprehensive Fibonacci Benchmark Suite with SIMD, Go Bridge, and More**

---

## Post Content

Hey r/rust! 👋

I'm excited to share **FibBenchmark**, a comprehensive Fibonacci benchmark suite written in Rust. What started as a learning project has grown into a full-featured library with multiple algorithm implementations, profiling tools, and now SIMD optimization!

### 🚀 Key Features

- **6 Algorithm Implementations**: Recursive, memoized, iterative, matrix exponentiation, doubling, and Binet formula
- **Interactive Dashboard**: Modern web interface with dark/light mode for visualizing results
- **SIMD Batch Processing**: Parallel Fibonacci calculation using the `wide` crate on stable Rust
- **Go FFI Bridge**: Compare Rust vs Go performance
- **Memory Profiling**: Custom allocator for tracking memory usage
- **Visualization**: Interactive HTML reports with Plotly charts
- **Comprehensive Benchmarks**: Criterion-based microbenchmarks

### 📦 Crates & Components

| Component      | Description                       |
| -------------- | --------------------------------- |
| `fib-core`     | Core algorithms with SIMD support |
| `fib-cli`      | CLI with 10+ commands             |
| `fib-profiler` | Memory tracking & flamegraphs     |
| `fib-viz`      | HTML report generation            |
| `fib-go`       | Rust-Go FFI bridge                |
| `dashboard/`   | Interactive web UI                |

### 🔧 SIMD Implementation

The new SIMD module uses the [`wide`](https://crates.io/crates/wide) crate for portable SIMD on **stable Rust**:

```rust
use fib_core::simd::{fib_simd_batch, SimdFeatures};

// Calculate multiple Fibonacci numbers in parallel
let results = fib_simd_batch(&[10, 20, 30, 40]);
assert_eq!(results, vec![55, 6765, 832040, 102334155]);

// Check CPU SIMD features
let features = SimdFeatures::detect();
println!("Available: {} (best: {}-bit)", features, features.best_width());
```

The implementation processes indices in 4-wide lanes (AVX2). While SIMD doesn't speed up single Fibonacci calculations (due to sequential dependencies), it shines for batch processing scenarios.

### 📊 What I Learned

- **Algorithm complexity matters**: Matrix exponentiation is O(log n) vs O(n) iterative
- **SIMD has overhead**: For small workloads, scalar can be faster
- **FFI is complex**: CGO adds significant overhead; pure Rust was often faster
- **Criterion is amazing**: Statistical benchmarking catches subtle regressions

### 🔗 Links

- **GitHub**: [https://github.com/agbru/FibBenchmark](https://github.com/agbru/FibBenchmark)
- **Documentation**: See `/docs` folder
- **Interactive Dashboard**: `/dashboard/index.html`

### 📝 Feedback Welcome!

This project is MIT licensed. I'd love feedback on:

1. SIMD implementation approach (using `wide` vs waiting for `std::simd`)
2. Algorithm optimizations I might have missed
3. Benchmark methodology improvements
4. Any code review comments!

Thanks for reading! 🦀

---

## Suggested Flair

`project`

---

## Before Posting Checklist

- [ ] All tests passing (`cargo test --all`)
- [ ] Clippy clean (`cargo clippy`)
- [ ] README polished
- [ ] License visible
- [ ] Benchmarks run recently
- [ ] Screenshots/GIFs prepared
