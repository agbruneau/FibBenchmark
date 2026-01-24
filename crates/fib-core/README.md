# fib-core

The core library for the Fibonacci Benchmark Suite. This crate implements various algorithms for calculating Fibonacci numbers, ranging from basic recursive definitions to optimized matrix exponentiation and SIMD-accelerated batch processing.

## Algorithms

| Algorithm | Time Complexity | Space Complexity | Description |
|-----------|-----------------|------------------|-------------|
| Recursive | O(2^n) | O(n) | Naive implementation (slow). |
| Recursive Memo | O(n) | O(n) | Recursive with memoization. |
| Iterative | O(n) | O(1) | Standard iterative loop. |
| Matrix | O(log n) | O(1) | Matrix exponentiation. |
| Fast Doubling | O(log n) | O(log n) | Optimized matrix approach. |
| Binet | O(1) | O(1) | Closed-form approximation (float). |

## Usage

```rust
use fib_core::{FibMethod, iterative, matrix};

fn main() {
    // Direct usage
    let f100 = iterative::fib_iterative(100);

    // Via enum strategy
    let method = FibMethod::FastDoubling;
    let result = method.calculate(100);

    println!("F(100) = {}", result);
}
```

## Large Number Support

The library supports calculating Fibonacci numbers beyond `u128::MAX` (approx. F(186)) using the `num-bigint` crate.

- `FibMethod::calculate(n)` returns `u128` and will panic on overflow if `n > 186` (in debug mode) or wrap (release mode), unless checked.
- `FibMethod::calculate_bigint(n)` returns `BigUint` and supports arbitrarily large numbers.

## Features

*   **SIMD**: Enable `simd` feature for AVX2/AVX512 batch processing support.
