//! # fib-core
//!
//! Core Fibonacci algorithm implementations with multiple complexity levels.
//!
//! ## Algorithms Provided
//!
//! | Algorithm | Time Complexity | Space Complexity | Best For |
//! |-----------|-----------------|------------------|----------|
//! | Recursive | O(2^n) | O(n) | Demonstration only |
//! | Recursive Memo | O(n) | O(n) | Small n with caching |
//! | Iterative | O(n) | O(1) | General use |
//! | Matrix | O(log n) | O(1) | Large n values |
//! | Fast Doubling | O(log n) | O(log n) | Large n values (alternative) |
//! | Binet | O(1) | O(1) | Approximation (n ≤ 78) |
//!
//! ## BigInt Support
//!
//! For calculations exceeding `u128::MAX` (F(186)), use `calculate_bigint` or the specific BigInt implementations.
//!
//! ## Example
//!
//! ```rust
//! use fib_core::{iterative, matrix};
//!
//! let fib_20 = iterative::fib_iterative(20);
//! assert_eq!(fib_20, 6765);
//!
//! // Calculate large Fibonacci numbers using BigInt
//! use fib_core::FibMethod;
//! use num_bigint::BigUint;
//!
//! let method = FibMethod::Matrix;
//! let big_fib = method.calculate_bigint(200);
//! // 280571172992510140037611932413038677189525
//! ```

pub mod allocator;
pub mod bigint;
pub mod closed_form;
pub mod iterative;
pub mod matrix;
pub mod memory;
pub mod recursive;

#[cfg(feature = "simd")]
pub mod simd;

// Re-export main functions for convenience
pub use bigint::{fib_iterative_big, fib_matrix_big};
pub use closed_form::{binet_error_analysis, fib_binet_f64};
pub use iterative::{fib_iterative, fib_iterative_batch, fib_iterative_branchless};
pub use matrix::{fib_doubling, fib_matrix_fast, fib_matrix_modulo};
pub use recursive::{fib_recursive, fib_recursive_memo};

#[cfg(feature = "simd")]
pub use simd::{fib_simd_batch, SimdBatchCalculator, SimdFeatures};

/// Enum representing available Fibonacci algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FibMethod {
    /// Naive recursive - O(2^n)
    Recursive,
    /// Recursive with memoization - O(n)
    RecursiveMemo,
    /// Iterative - O(n)
    Iterative,
    /// Iterative branchless - O(n)
    IterativeBranchless,
    /// Matrix exponentiation - O(log n)
    Matrix,
    /// Fast doubling - O(log n)
    FastDoubling,
    /// Binet formula - O(1) with precision limits
    Binet,
}

impl FibMethod {
    /// Calculate Fibonacci using the specified method
    ///
    /// # Arguments
    /// * `n` - The Fibonacci index to calculate
    ///
    /// # Returns
    /// The nth Fibonacci number
    ///
    /// # Panics
    /// May panic if the result exceeds u128::MAX (approx F(186)).
    /// For larger numbers, use `calculate_bigint`.
    pub fn calculate(&self, n: u64) -> u128 {
        match self {
            FibMethod::Recursive => fib_recursive(n),
            FibMethod::RecursiveMemo => fib_recursive_memo(n),
            FibMethod::Iterative => fib_iterative(n),
            FibMethod::IterativeBranchless => fib_iterative_branchless(n),
            FibMethod::Matrix => fib_matrix_fast(n),
            FibMethod::FastDoubling => fib_doubling(n),
            FibMethod::Binet => fib_binet_f64(n) as u128,
        }
    }

    /// Calculate Fibonacci using BigUint for arbitrary precision
    ///
    /// # Arguments
    /// * `n` - The Fibonacci index to calculate
    ///
    /// # Returns
    /// The nth Fibonacci number as a BigUint
    pub fn calculate_bigint(&self, n: u64) -> num_bigint::BigUint {
        match self {
            FibMethod::Iterative | FibMethod::IterativeBranchless => fib_iterative_big(n),
            FibMethod::Matrix | FibMethod::FastDoubling => fib_matrix_big(n),
            // For others, fall back to calculate() and convert, or error if too big?
            // For safety, we use iterative big for any method that doesn't natively support it
            // if n is large, otherwise we can cast.
            // But since this is a BigInt method, users expect it to work for large n.
            // So we default to Matrix BigInt for "fast" methods and Iterative Big for others.
            FibMethod::Recursive | FibMethod::RecursiveMemo => fib_iterative_big(n),
            FibMethod::Binet => {
                // Binet is approximate, but let's try to be consistent
                num_bigint::BigUint::from(self.calculate(n))
            }
        }
    }

    /// Get the name of the method
    pub fn name(&self) -> &'static str {
        match self {
            FibMethod::Recursive => "recursive",
            FibMethod::RecursiveMemo => "recursive_memo",
            FibMethod::Iterative => "iterative",
            FibMethod::IterativeBranchless => "iterative_branchless",
            FibMethod::Matrix => "matrix",
            FibMethod::FastDoubling => "fast_doubling",
            FibMethod::Binet => "binet",
        }
    }

    /// Get the time complexity of the method
    pub fn time_complexity(&self) -> &'static str {
        match self {
            FibMethod::Recursive => "O(2^n)",
            FibMethod::RecursiveMemo => "O(n)",
            FibMethod::Iterative => "O(n)",
            FibMethod::IterativeBranchless => "O(n)",
            FibMethod::Matrix => "O(log n)",
            FibMethod::FastDoubling => "O(log n)",
            FibMethod::Binet => "O(1)",
        }
    }

    /// Get the space complexity of the method
    pub fn space_complexity(&self) -> &'static str {
        match self {
            FibMethod::Recursive => "O(n)",
            FibMethod::RecursiveMemo => "O(n)",
            FibMethod::Iterative => "O(1)",
            FibMethod::IterativeBranchless => "O(1)",
            FibMethod::Matrix => "O(1)",
            FibMethod::FastDoubling => "O(log n)",
            FibMethod::Binet => "O(1)",
        }
    }
}

impl std::str::FromStr for FibMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "recursive" => Ok(FibMethod::Recursive),
            "recursive_memo" | "memo" => Ok(FibMethod::RecursiveMemo),
            "iterative" => Ok(FibMethod::Iterative),
            "iterative_branchless" | "branchless" => Ok(FibMethod::IterativeBranchless),
            "matrix" => Ok(FibMethod::Matrix),
            "fast_doubling" | "doubling" => Ok(FibMethod::FastDoubling),
            "binet" => Ok(FibMethod::Binet),
            _ => Err(format!("Unknown method: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIRST_20_FIBS: [u128; 21] = [
        0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765,
    ];

    #[test]
    fn test_all_methods_match() {
        for (n, expected) in FIRST_20_FIBS.iter().enumerate() {
            let n = n as u64;
            assert_eq!(fib_iterative(n), *expected, "iterative failed at n={}", n);
            assert_eq!(fib_matrix_fast(n), *expected, "matrix failed at n={}", n);
            assert_eq!(
                fib_doubling(n),
                *expected,
                "fast_doubling failed at n={}",
                n
            );

            if n <= 15 {
                assert_eq!(fib_recursive(n), *expected, "recursive failed at n={}", n);
            }

            assert_eq!(
                fib_recursive_memo(n),
                *expected,
                "recursive_memo failed at n={}",
                n
            );
        }
    }

    #[test]
    fn test_large_fibonacci() {
        // F(100) = 354224848179261915075
        let expected = 354224848179261915075u128;
        assert_eq!(fib_iterative(100), expected);
        assert_eq!(fib_matrix_fast(100), expected);
        assert_eq!(fib_doubling(100), expected);
    }

    #[test]
    fn test_fib_method_enum() {
        let method = FibMethod::Iterative;
        assert_eq!(method.calculate(10), 55);
        assert_eq!(method.name(), "iterative");
        assert_eq!(method.time_complexity(), "O(n)");
    }

    #[test]
    fn test_fib_method_from_str() {
        use std::str::FromStr;
        assert_eq!(
            FibMethod::from_str("recursive").unwrap(),
            FibMethod::Recursive
        );
        assert_eq!(
            FibMethod::from_str("RECURSIVE").unwrap(),
            FibMethod::Recursive
        );
        assert_eq!(
            FibMethod::from_str("memo").unwrap(),
            FibMethod::RecursiveMemo
        );
        assert_eq!(
            FibMethod::from_str("iterative").unwrap(),
            FibMethod::Iterative
        );
        assert_eq!(
            FibMethod::from_str("branchless").unwrap(),
            FibMethod::IterativeBranchless
        );
        assert_eq!(FibMethod::from_str("matrix").unwrap(), FibMethod::Matrix);
        assert_eq!(
            FibMethod::from_str("doubling").unwrap(),
            FibMethod::FastDoubling
        );
        assert_eq!(FibMethod::from_str("binet").unwrap(), FibMethod::Binet);

        assert!(FibMethod::from_str("invalid").is_err());
    }

    #[test]
    fn test_fib_method_complexity_strings() {
        let methods = [
            FibMethod::Recursive,
            FibMethod::RecursiveMemo,
            FibMethod::Iterative,
            FibMethod::IterativeBranchless,
            FibMethod::Matrix,
            FibMethod::FastDoubling,
            FibMethod::Binet,
        ];

        for method in methods {
            assert!(!method.time_complexity().is_empty());
            assert!(!method.space_complexity().is_empty());
        }
    }
}
