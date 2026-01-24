//! Go FFI Bridge for Fibonacci Algorithms
//!
//! This crate provides Rust bindings to Fibonacci implementations written in Go,
//! allowing for cross-language performance comparisons.
//!
//! # Example
//!
//! ```
//! use fib_go::{go_fib_iterative, go_fib_matrix, is_go_available};
//!
//! if is_go_available() {
//!     let result = go_fib_iterative(100);
//!     println!("F(100) via Go: {}", result);
//! }
//! ```

use std::time::{Duration, Instant};

// When CGO is available, use FFI
#[cfg(not(use_rust_stub))]
mod ffi {
    use std::ffi::CStr;

    extern "C" {
        fn FibIterative(n: u64) -> u64;
        fn FibRecursive(n: u64) -> u64;
        fn FibMemo(n: u64) -> u64;
        fn FibMatrix(n: u64) -> u64;
        fn FibDoubling(n: u64) -> u64;
        fn GetGoVersion() -> *const std::os::raw::c_char;
    }

    pub fn fib_iterative(n: u64) -> u64 {
        unsafe { FibIterative(n) }
    }

    pub fn fib_recursive(n: u64) -> u64 {
        unsafe { FibRecursive(n) }
    }

    pub fn fib_memo(n: u64) -> u64 {
        unsafe { FibMemo(n) }
    }

    pub fn fib_matrix(n: u64) -> u64 {
        unsafe { FibMatrix(n) }
    }

    pub fn fib_doubling(n: u64) -> u64 {
        unsafe { FibDoubling(n) }
    }

    pub fn get_version() -> String {
        unsafe {
            let ptr = GetGoVersion();
            if ptr.is_null() {
                return "unknown".to_string();
            }
            CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }

    pub fn is_available() -> bool {
        true
    }
}

// When CGO is not available, use pure Rust stub (simulating Go behavior)
#[cfg(use_rust_stub)]
mod ffi {
    /// Iterative Fibonacci - O(n)
    pub fn fib_iterative(n: u64) -> u64 {
        if n <= 1 {
            return n;
        }
        let (mut a, mut b) = (0u64, 1u64);
        for _ in 2..=n {
            let temp = a.wrapping_add(b);
            a = b;
            b = temp;
        }
        b
    }

    /// Recursive Fibonacci - O(2^n) - Very slow!
    pub fn fib_recursive(n: u64) -> u64 {
        if n <= 1 {
            return n;
        }
        fib_recursive(n - 1).wrapping_add(fib_recursive(n - 2))
    }

    /// Memoized Fibonacci - O(n)
    pub fn fib_memo(n: u64) -> u64 {
        fn fib_memo_inner(n: u64, memo: &mut std::collections::HashMap<u64, u64>) -> u64 {
            if n <= 1 {
                return n;
            }
            if let Some(&val) = memo.get(&n) {
                return val;
            }
            let result = fib_memo_inner(n - 1, memo).wrapping_add(fib_memo_inner(n - 2, memo));
            memo.insert(n, result);
            result
        }
        let mut memo = std::collections::HashMap::new();
        fib_memo_inner(n, &mut memo)
    }

    /// Matrix Fibonacci - O(log n)
    pub fn fib_matrix(n: u64) -> u64 {
        if n == 0 {
            return 0;
        }

        #[derive(Clone, Copy)]
        struct Matrix {
            a: u64,
            b: u64,
            c: u64,
            d: u64,
        }

        fn multiply(m1: Matrix, m2: Matrix) -> Matrix {
            Matrix {
                a: m1
                    .a
                    .wrapping_mul(m2.a)
                    .wrapping_add(m1.b.wrapping_mul(m2.c)),
                b: m1
                    .a
                    .wrapping_mul(m2.b)
                    .wrapping_add(m1.b.wrapping_mul(m2.d)),
                c: m1
                    .c
                    .wrapping_mul(m2.a)
                    .wrapping_add(m1.d.wrapping_mul(m2.c)),
                d: m1
                    .c
                    .wrapping_mul(m2.b)
                    .wrapping_add(m1.d.wrapping_mul(m2.d)),
            }
        }

        fn power(m: Matrix, mut n: u64) -> Matrix {
            let mut result = Matrix {
                a: 1,
                b: 0,
                c: 0,
                d: 1,
            };
            let mut base = m;
            while n > 0 {
                if n % 2 == 1 {
                    result = multiply(result, base);
                }
                base = multiply(base, base);
                n /= 2;
            }
            result
        }

        let fib_mat = Matrix {
            a: 1,
            b: 1,
            c: 1,
            d: 0,
        };
        power(fib_mat, n).b
    }

    /// Doubling method - O(log n)
    pub fn fib_doubling(n: u64) -> u64 {
        fn helper(n: u64) -> (u64, u64) {
            if n == 0 {
                return (0, 1);
            }
            let (fk, fk1) = helper(n / 2);
            let f2k = fk.wrapping_mul(fk1.wrapping_mul(2).wrapping_sub(fk));
            let f2k1 = fk.wrapping_mul(fk).wrapping_add(fk1.wrapping_mul(fk1));
            if n.is_multiple_of(2) {
                (f2k, f2k1)
            } else {
                (f2k1, f2k.wrapping_add(f2k1))
            }
        }
        helper(n).0
    }

    pub fn get_version() -> String {
        "Rust stub (CGO not available - install GCC/MinGW for native Go)".to_string()
    }

    pub fn is_available() -> bool {
        false
    }
}

/// Available Go Fibonacci methods
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GoFibMethod {
    /// Iterative method - O(n)
    Iterative,
    /// Naive recursive method - O(2^n) - WARNING: very slow for n > 35
    Recursive,
    /// Memoized recursive method - O(n)
    Memoized,
    /// Matrix exponentiation - O(log n)
    Matrix,
    /// Doubling method - O(log n)
    Doubling,
}

impl GoFibMethod {
    /// Get the name of the method
    pub fn name(&self) -> &'static str {
        match self {
            GoFibMethod::Iterative => "Go Iterative",
            GoFibMethod::Recursive => "Go Recursive",
            GoFibMethod::Memoized => "Go Memoized",
            GoFibMethod::Matrix => "Go Matrix",
            GoFibMethod::Doubling => "Go Doubling",
        }
    }

    /// Get the time complexity
    pub fn time_complexity(&self) -> &'static str {
        match self {
            GoFibMethod::Iterative => "O(n)",
            GoFibMethod::Recursive => "O(2^n)",
            GoFibMethod::Memoized => "O(n)",
            GoFibMethod::Matrix => "O(log n)",
            GoFibMethod::Doubling => "O(log n)",
        }
    }

    /// Calculate Fibonacci using this method
    pub fn calculate(&self, n: u64) -> u64 {
        match self {
            GoFibMethod::Iterative => go_fib_iterative(n),
            GoFibMethod::Recursive => go_fib_recursive(n),
            GoFibMethod::Memoized => go_fib_memo(n),
            GoFibMethod::Matrix => go_fib_matrix(n),
            GoFibMethod::Doubling => go_fib_doubling(n),
        }
    }

    /// Get all available methods
    pub fn all() -> &'static [GoFibMethod] {
        &[
            GoFibMethod::Iterative,
            GoFibMethod::Memoized,
            GoFibMethod::Matrix,
            GoFibMethod::Doubling,
        ]
    }

    /// Get all methods including slow ones (for small n)
    pub fn all_including_slow() -> &'static [GoFibMethod] {
        &[
            GoFibMethod::Iterative,
            GoFibMethod::Recursive,
            GoFibMethod::Memoized,
            GoFibMethod::Matrix,
            GoFibMethod::Doubling,
        ]
    }
}

/// Calculate Fibonacci using Go's iterative implementation
///
/// # Arguments
/// * `n` - The Fibonacci index to calculate
///
/// # Returns
/// The nth Fibonacci number
#[inline]
pub fn go_fib_iterative(n: u64) -> u64 {
    ffi::fib_iterative(n)
}

/// Calculate Fibonacci using Go's naive recursive implementation
///
/// # Warning
/// This is extremely slow for n > 35. Use only for demonstration purposes.
#[inline]
pub fn go_fib_recursive(n: u64) -> u64 {
    ffi::fib_recursive(n)
}

/// Calculate Fibonacci using Go's memoized recursive implementation
#[inline]
pub fn go_fib_memo(n: u64) -> u64 {
    ffi::fib_memo(n)
}

/// Calculate Fibonacci using Go's matrix exponentiation implementation
#[inline]
pub fn go_fib_matrix(n: u64) -> u64 {
    ffi::fib_matrix(n)
}

/// Calculate Fibonacci using Go's doubling method implementation
#[inline]
pub fn go_fib_doubling(n: u64) -> u64 {
    ffi::fib_doubling(n)
}

/// Get the Go runtime version
pub fn get_go_version() -> String {
    ffi::get_version()
}

/// Check if Go implementation is available (not a stub)
pub fn is_go_available() -> bool {
    ffi::is_available()
}

/// Result of a benchmark comparison
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    /// Method name
    pub method: String,
    /// Language (Rust or Go)
    pub language: String,
    /// Input value n
    pub n: u64,
    /// Result of F(n)
    pub result: u64,
    /// Average execution time
    pub avg_time: Duration,
    /// Number of iterations
    pub iterations: u32,
}

/// Compare Rust and Go implementations for a given n
pub fn compare_implementations(n: u64, iterations: u32) -> Vec<BenchmarkResult> {
    use fib_core::{recursive, FibMethod};

    let mut results = Vec::new();

    // Rust implementations
    let rust_methods = [
        ("Iterative", FibMethod::Iterative),
        ("Matrix", FibMethod::Matrix),
    ];

    for (name, method) in rust_methods {
        let mut total = Duration::ZERO;
        let mut result = 0u128;

        for _ in 0..iterations {
            let start = Instant::now();
            result = method.calculate(n);
            total += start.elapsed();
        }

        results.push(BenchmarkResult {
            method: name.to_string(),
            language: "Rust".to_string(),
            n,
            result: result as u64,
            avg_time: total / iterations,
            iterations,
        });
    }

    // Go implementations
    let go_methods = [
        ("Iterative", GoFibMethod::Iterative),
        ("Matrix", GoFibMethod::Matrix),
        ("Doubling", GoFibMethod::Doubling),
    ];

    for (name, method) in go_methods {
        let mut total = Duration::ZERO;
        let mut result = 0u64;

        for _ in 0..iterations {
            let start = Instant::now();
            result = method.calculate(n);
            total += start.elapsed();
        }

        results.push(BenchmarkResult {
            method: name.to_string(),
            language: "Go".to_string(),
            n,
            result,
            avg_time: total / iterations,
            iterations,
        });
    }

    // Also compare memoized for smaller n
    if n <= 10000 {
        // Rust memoized
        let mut total = Duration::ZERO;
        let mut result = 0u128;
        for _ in 0..iterations {
            let start = Instant::now();
            result = recursive::fib_recursive_memo(n);
            total += start.elapsed();
        }
        results.push(BenchmarkResult {
            method: "Memoized".to_string(),
            language: "Rust".to_string(),
            n,
            result: result as u64,
            avg_time: total / iterations,
            iterations,
        });

        // Go memoized
        let mut total = Duration::ZERO;
        let mut result = 0u64;
        for _ in 0..iterations {
            let start = Instant::now();
            result = go_fib_memo(n);
            total += start.elapsed();
        }
        results.push(BenchmarkResult {
            method: "Memoized".to_string(),
            language: "Go".to_string(),
            n,
            result,
            avg_time: total / iterations,
            iterations,
        });
    }

    results
}

/// Format benchmark results as a table
pub fn format_comparison_table(results: &[BenchmarkResult]) -> String {
    let mut output = String::new();

    output.push_str(&format!("\n{:─^80}\n", " Rust vs Go Fibonacci Comparison "));
    output.push_str(&format!(
        "| {:^12} | {:^8} | {:^15} | {:^15} | {:^12} |\n",
        "Method", "Language", "n", "Time (avg)", "Result"
    ));
    output.push_str(&format!("{:─^80}\n", ""));

    for r in results {
        let time_str = format!("{:?}", r.avg_time);
        let result_str = if r.result > 999_999_999 {
            format!("{}...", &r.result.to_string()[..10])
        } else {
            r.result.to_string()
        };

        output.push_str(&format!(
            "| {:^12} | {:^8} | {:^15} | {:^15} | {:^12} |\n",
            r.method, r.language, r.n, time_str, result_str
        ));
    }

    output.push_str(&format!("{:─^80}\n", ""));

    // Calculate speedups
    output.push_str("\n📊 Speedup Analysis (Rust vs Go):\n");

    let rust_results: Vec<_> = results.iter().filter(|r| r.language == "Rust").collect();
    let go_results: Vec<_> = results.iter().filter(|r| r.language == "Go").collect();

    for rust_r in &rust_results {
        if let Some(go_r) = go_results.iter().find(|g| g.method == rust_r.method) {
            let rust_ns = rust_r.avg_time.as_nanos() as f64;
            let go_ns = go_r.avg_time.as_nanos() as f64;

            if rust_ns > 0.0 && go_ns > 0.0 {
                let speedup = go_ns / rust_ns;
                let winner = if speedup > 1.0 { "Rust" } else { "Go" };
                let factor = if speedup > 1.0 {
                    speedup
                } else {
                    1.0 / speedup
                };

                output.push_str(&format!(
                    "  {}: {} is {:.2}x faster\n",
                    rust_r.method, winner, factor
                ));
            }
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_go_iterative() {
        assert_eq!(go_fib_iterative(0), 0);
        assert_eq!(go_fib_iterative(1), 1);
        assert_eq!(go_fib_iterative(10), 55);
        assert_eq!(go_fib_iterative(20), 6765);
    }

    #[test]
    fn test_go_matrix() {
        assert_eq!(go_fib_matrix(0), 0);
        assert_eq!(go_fib_matrix(1), 1);
        assert_eq!(go_fib_matrix(10), 55);
        assert_eq!(go_fib_matrix(20), 6765);
    }

    #[test]
    fn test_go_doubling() {
        assert_eq!(go_fib_doubling(0), 0);
        assert_eq!(go_fib_doubling(1), 1);
        assert_eq!(go_fib_doubling(10), 55);
        assert_eq!(go_fib_doubling(20), 6765);
    }

    #[test]
    fn test_go_memo() {
        assert_eq!(go_fib_memo(0), 0);
        assert_eq!(go_fib_memo(1), 1);
        assert_eq!(go_fib_memo(10), 55);
        assert_eq!(go_fib_memo(20), 6765);
    }

    #[test]
    fn test_methods_match() {
        for n in [0, 1, 5, 10, 20, 30] {
            let iter = go_fib_iterative(n);
            let matrix = go_fib_matrix(n);
            let doubling = go_fib_doubling(n);
            let memo = go_fib_memo(n);

            assert_eq!(iter, matrix, "Mismatch at n={}", n);
            assert_eq!(iter, doubling, "Mismatch at n={}", n);
            assert_eq!(iter, memo, "Mismatch at n={}", n);
        }
    }

    #[test]
    fn test_go_version() {
        let version = get_go_version();
        assert!(!version.is_empty());
    }

    #[test]
    fn test_rust_go_match() {
        use fib_core::iterative::fib_iterative;

        for n in [0, 1, 5, 10, 20, 50] {
            let rust_result = fib_iterative(n);
            let go_result = go_fib_iterative(n);
            assert_eq!(
                rust_result, go_result as u128,
                "Rust/Go mismatch at n={}",
                n
            );
        }
    }

    #[test]
    fn test_go_large_number_handling() {
        // Go's uint64 max is 18446744073709551615
        // F(93) = 12200160415121876738 which fits in u64
        // F(94) = 19740274219868223167 which overflows u64
        let n = 93;
        let result = go_fib_iterative(n);
        // Compare with Rust
        let rust_result = fib_core::fib_iterative(n);
        assert_eq!(result as u128, rust_result);
    }
}
