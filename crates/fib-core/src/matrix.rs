//! Matrix-based Fibonacci implementations
//!
//! This module uses matrix exponentiation to calculate Fibonacci numbers
//! in O(log n) time complexity. This is the fastest method for large n values.
//!
//! ## Mathematical Background
//!
//! The key insight is that:
//! ```text
//! [[1, 1],    ^n     = [[F(n+1), F(n)  ],
//!  [1, 0]]              [F(n),   F(n-1)]]
//! ```
//!
//! Using fast exponentiation (repeated squaring), we can compute the
//! matrix power in O(log n) matrix multiplications.

use std::ops::Mul;

/// 2x2 Matrix structure for Fibonacci calculation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Matrix2x2 {
    data: [[u128; 2]; 2],
}

impl Matrix2x2 {
    /// Create a new 2x2 matrix
    pub fn new(data: [[u128; 2]; 2]) -> Self {
        Self { data }
    }

    /// Create the identity matrix
    pub fn identity() -> Self {
        Self::new([[1, 0], [0, 1]])
    }

    /// Create the Fibonacci base matrix [[1,1],[1,0]]
    pub fn fibonacci_base() -> Self {
        Self::new([[1, 1], [1, 0]])
    }

    /// Get element at position (row, col)
    pub fn get(&self, row: usize, col: usize) -> u128 {
        self.data[row][col]
    }
}

impl Mul for Matrix2x2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let a = self.data;
        let b = other.data;

        Matrix2x2::new([
            [
                a[0][0]
                    .wrapping_mul(b[0][0])
                    .wrapping_add(a[0][1].wrapping_mul(b[1][0])),
                a[0][0]
                    .wrapping_mul(b[0][1])
                    .wrapping_add(a[0][1].wrapping_mul(b[1][1])),
            ],
            [
                a[1][0]
                    .wrapping_mul(b[0][0])
                    .wrapping_add(a[1][1].wrapping_mul(b[1][0])),
                a[1][0]
                    .wrapping_mul(b[0][1])
                    .wrapping_add(a[1][1].wrapping_mul(b[1][1])),
            ],
        ])
    }
}

/// Fast Fibonacci using matrix exponentiation
///
/// Uses the identity that [[1,1],[1,0]]^n = [[F(n+1),F(n)],[F(n),F(n-1)]]
/// and computes the matrix power using repeated squaring in O(log n).
///
/// # Complexity
/// - Time: O(log n)
/// - Space: O(1)
///
/// # Example
/// ```
/// use fib_core::matrix::fib_matrix_fast;
///
/// assert_eq!(fib_matrix_fast(0), 0);
/// assert_eq!(fib_matrix_fast(10), 55);
/// assert_eq!(fib_matrix_fast(100), 354224848179261915075);
/// ```
pub fn fib_matrix_fast(n: u64) -> u128 {
    if n == 0 {
        return 0;
    }

    let mut result = Matrix2x2::identity();
    let mut base = Matrix2x2::fibonacci_base();
    let mut exp = n;

    // Fast exponentiation using repeated squaring
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base;
        }
        base = base * base;
        exp /= 2;
    }

    result.get(0, 1) // F(n)
}

/// Matrix Fibonacci with modular arithmetic
///
/// Computes F(n) mod m, useful for very large n where overflow would occur.
///
/// # Complexity
/// - Time: O(log n)
/// - Space: O(1)
///
/// # Example
/// ```
/// use fib_core::matrix::fib_matrix_modulo;
///
/// // F(1000) mod 1_000_000_007
/// let result = fib_matrix_modulo(1000, 1_000_000_007);
/// assert_eq!(result, 517691607);
/// ```
pub fn fib_matrix_modulo(n: u64, modulo: u128) -> u128 {
    if n == 0 {
        return 0;
    }

    fn mul_mod(a: [[u128; 2]; 2], b: [[u128; 2]; 2], m: u128) -> [[u128; 2]; 2] {
        [
            [
                ((a[0][0] * b[0][0]) % m + (a[0][1] * b[1][0]) % m) % m,
                ((a[0][0] * b[0][1]) % m + (a[0][1] * b[1][1]) % m) % m,
            ],
            [
                ((a[1][0] * b[0][0]) % m + (a[1][1] * b[1][0]) % m) % m,
                ((a[1][0] * b[0][1]) % m + (a[1][1] * b[1][1]) % m) % m,
            ],
        ]
    }

    let mut exp = n;
    let mut result = [[1, 0], [0, 1]];
    let mut base = [[1, 1], [1, 0]];

    while exp > 0 {
        if exp & 1 == 1 {
            result = mul_mod(result, base, modulo);
        }
        base = mul_mod(base, base, modulo);
        exp /= 2;
    }

    result[0][1]
}

/// Calculate F(2n) using the identity: F(2n) = F(n) * (2*F(n+1) - F(n))
///
/// This is useful for calculating even-indexed Fibonacci numbers faster.
///
/// # Example
/// ```
/// use fib_core::matrix::fib_doubling;
///
/// assert_eq!(fib_doubling(20), 6765);
/// assert_eq!(fib_doubling(100), 354224848179261915075);
/// ```
pub fn fib_doubling(n: u64) -> u128 {
    if n == 0 {
        return 0;
    }

    fn fib_pair(n: u64) -> (u128, u128) {
        if n == 0 {
            return (0, 1);
        }

        let (f_k, f_k1) = fib_pair(n / 2);
        let f_2k = f_k * (2 * f_k1 - f_k);
        let f_2k1 = f_k * f_k + f_k1 * f_k1;

        if n & 1 == 0 {
            (f_2k, f_2k1)
        } else {
            (f_2k1, f_2k + f_2k1)
        }
    }

    fib_pair(n).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_multiplication() {
        let a = Matrix2x2::fibonacci_base();
        let b = Matrix2x2::identity();
        let result = a * b;
        assert_eq!(result, a);
    }

    #[test]
    fn test_fib_matrix_base_cases() {
        assert_eq!(fib_matrix_fast(0), 0);
        assert_eq!(fib_matrix_fast(1), 1);
        assert_eq!(fib_matrix_fast(2), 1);
    }

    #[test]
    fn test_fib_matrix_known_values() {
        assert_eq!(fib_matrix_fast(10), 55);
        assert_eq!(fib_matrix_fast(20), 6765);
        assert_eq!(fib_matrix_fast(50), 12586269025);
        assert_eq!(fib_matrix_fast(100), 354224848179261915075);
    }

    #[test]
    fn test_fib_matrix_modulo() {
        let modulo = 1_000_000_007u128;
        assert_eq!(fib_matrix_modulo(10, modulo), 55);
        assert_eq!(fib_matrix_modulo(1000, modulo), 517691607);
    }

    #[test]
    fn test_fib_doubling() {
        for n in 0..50 {
            assert_eq!(fib_doubling(n), fib_matrix_fast(n), "Mismatch at n={}", n);
        }
    }

    #[test]
    fn test_matrix_matches_iterative() {
        use crate::iterative::fib_iterative;

        for n in 0..100 {
            assert_eq!(
                fib_matrix_fast(n),
                fib_iterative(n),
                "Matrix/iterative mismatch at n={}",
                n
            );
        }
    }

    #[test]
    fn test_matrix_get() {
        let m = Matrix2x2::identity();
        assert_eq!(m.get(0, 0), 1);
        assert_eq!(m.get(0, 1), 0);
        assert_eq!(m.get(1, 0), 0);
        assert_eq!(m.get(1, 1), 1);
    }

    #[test]
    fn test_matrix_identity() {
        let id = Matrix2x2::identity();
        let base = Matrix2x2::fibonacci_base();
        assert_eq!(base * id, base);
        assert_eq!(id * base, base);
    }

    #[test]
    fn test_fib_doubling_edge_cases() {
        assert_eq!(fib_doubling(0), 0);
        assert_eq!(fib_doubling(1), 1);
        assert_eq!(fib_doubling(2), 1);
        assert_eq!(fib_doubling(3), 2);
    }
}
