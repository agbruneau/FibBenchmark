# 🚀 SIMD Optimization for Fibonacci Batch Calculations

This module provides SIMD-accelerated (Single Instruction, Multiple Data) batch Fibonacci calculations using portable SIMD abstractions.

---

## Overview

### What is SIMD?

SIMD allows a single CPU instruction to operate on multiple data elements simultaneously. Modern CPUs support various SIMD instruction sets:

| Instruction Set | Vector Width | Lanes (u64) |
| --------------- | ------------ | ----------- |
| SSE2            | 128-bit      | 2           |
| AVX             | 256-bit      | 4           |
| AVX2            | 256-bit      | 4 (integer) |
| AVX-512         | 512-bit      | 8           |

### SIMD for Fibonacci

Standard Fibonacci calculation is inherently sequential (F(n) depends on F(n-1) and F(n-2)), so SIMD cannot directly accelerate a single calculation. Instead, we use **data parallelism**:

```
┌────────────────────────────────────────────┐
│         SIMD Batch Calculation             │
├────────────────────────────────────────────┤
│  Lane 0: F(10) → 55                        │
│  Lane 1: F(20) → 6765        ─── Parallel  │
│  Lane 2: F(30) → 832040      ─── Execution │
│  Lane 3: F(40) → 102334155                 │
└────────────────────────────────────────────┘
```

---

## Usage

### Enable the SIMD Feature

```toml
# Cargo.toml
[dependencies]
fib-core = { version = "1.0", features = ["simd"] }
```

### API Examples

```rust
use fib_core::simd::{fib_simd_batch, SimdFeatures, SimdBatchCalculator};

// Calculate batch of Fibonacci numbers
let indices = vec![10, 20, 30, 40, 50, 60, 70, 80];
let results = fib_simd_batch(&indices);

assert_eq!(results[0], 55);       // F(10)
assert_eq!(results[3], 102334155); // F(40)

// Detect available SIMD features
let features = SimdFeatures::detect();
println!("Available: {}", features);
println!("Best width: {}-bit", features.best_width());

// Use the batch calculator for benchmarking
let calc = SimdBatchCalculator::new();
let (simd_ns, scalar_ns) = calc.benchmark(&indices, 1000);
println!("SIMD: {} ns, Scalar: {} ns", simd_ns, scalar_ns);
```

### CLI Usage

```bash
# Enable SIMD feature when building
cargo run -p fib-cli --features simd -- simd --batch 10,20,30,40

# Show CPU SIMD capabilities
cargo run -p fib-cli --features simd -- simd --info

# Compare SIMD vs scalar performance
cargo run -p fib-cli --features simd -- simd --batch 10,20,30,40,50,60,70,80 --compare
```

---

## CPU Requirements

| Feature | Required For        | Detection        |
| ------- | ------------------- | ---------------- |
| SSE2    | Basic SIMD (x86-64) | Always available |
| AVX2    | 256-bit integer ops | Runtime detected |
| AVX-512 | 512-bit ops         | Runtime detected |

The implementation automatically detects available features at runtime and uses the best available instruction set.

---

## Performance Characteristics

### When SIMD Helps

✅ **Large batches**: Processing many Fibonacci indices at once  
✅ **Similar indices**: Lanes that complete around the same time  
✅ **High-throughput workloads**: Many calculations needed quickly

### When SIMD May Not Help

❌ **Small batches**: Overhead outweighs benefits for < 8 elements  
❌ **Very different indices**: F(1) and F(1000) in same batch wastes work  
❌ **Single calculations**: No parallelism possible

### Benchmark Results (Example)

```
Batch Size | SIMD (ns) | Scalar (ns) | Speedup
-----------|-----------|-------------|--------
4          | 500       | 120         | 0.24x
8          | 800       | 240         | 0.30x
16         | 1,200     | 480         | 0.40x
32         | 2,000     | 960         | 0.48x
64         | 3,500     | 1,920       | 0.55x
```

> **Note**: The current implementation prioritizes correctness and portability over maximum performance. Future versions may include architecture-specific optimizations.

---

## Implementation Details

### Library Used

We use the [`wide`](https://crates.io/crates/wide) crate for portable SIMD on stable Rust:

```toml
wide = { version = "0.7", optional = true }
```

### Why Not std::simd?

- `std::simd` requires nightly Rust
- Not yet stabilized (as of 2026)
- `wide` provides similar functionality on stable

### Algorithm

```rust
// Pseudo-code for SIMD batch calculation
fn fib_simd_batch(indices: &[u64]) -> Vec<u64> {
    for chunk in indices.chunks(4) {
        // Initialize 4 lanes: (a, b) = (0, 1) for each
        let mut a = u64x4::splat(0);
        let mut b = u64x4::splat(1);

        // Iterate to max(indices in chunk)
        for i in 0..max_n {
            let active = targets.cmp_gt(current_iter);
            let temp = a + b;
            a = active.blend(b, a);
            b = active.blend(temp, b);
        }

        results.extend(a.to_array());
    }
}
```

---

## Running Benchmarks

```bash
# Run SIMD benchmarks
cargo bench --features simd -- simd

# Run all benchmarks including SIMD
cargo bench --features simd
```

---

## Limitations

1. **u64 precision**: Results limited to F(93) before overflow
2. **Alignment**: Non-multiple-of-4 batches handled with scalar fallback
3. **Overhead**: SIMD setup cost makes very small batches slower

---

## See Also

- [Optimization Techniques](./optimization_techniques.md)
- [Memory Analysis](./memory_analysis.md)
- [rust-lang/portable-simd](https://github.com/rust-lang/portable-simd)
