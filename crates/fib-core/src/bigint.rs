//! BigInt Fibonacci implementations
//!
//! This module provides Fibonacci implementations using BigUint
//! to support values larger than u128::MAX (F(186)).

use num_bigint::BigUint;
use num_traits::{One, Zero};

/// Iterative Fibonacci using BigUint
///
/// # Complexity
/// - Time: O(n)
/// - Space: O(n) (due to BigInt storage)
pub fn fib_iterative_big(n: u64) -> BigUint {
    match n {
        0 => Zero::zero(),
        1 => One::one(),
        _ => {
            let mut a: BigUint = Zero::zero();
            let mut b: BigUint = One::one();
            for _ in 2..=n {
                let temp = &a + &b;
                a = b;
                b = temp;
            }
            b
        }
    }
}

/// Matrix Fibonacci using BigUint
///
/// # Complexity
/// - Time: O(log n)
/// - Space: O(n) (due to BigInt storage)
pub fn fib_matrix_big(n: u64) -> BigUint {
    if n == 0 {
        return Zero::zero();
    }
    if n == 1 {
        return One::one();
    }

    let mut result: [[BigUint; 2]; 2] = [[One::one(), Zero::zero()], [Zero::zero(), One::one()]];
    let mut base: [[BigUint; 2]; 2] = [[One::one(), One::one()], [One::one(), Zero::zero()]];
    let mut exp = n;

    while exp > 0 {
        if exp & 1 == 1 {
            result = mul_matrix_big(&result, &base);
        }
        base = mul_matrix_big(&base, &base);
        exp /= 2;
    }

    result[0][1].clone()
}

fn mul_matrix_big(a: &[[BigUint; 2]; 2], b: &[[BigUint; 2]; 2]) -> [[BigUint; 2]; 2] {
    [
        [
            &a[0][0] * &b[0][0] + &a[0][1] * &b[1][0],
            &a[0][0] * &b[0][1] + &a[0][1] * &b[1][1],
        ],
        [
            &a[1][0] * &b[0][0] + &a[1][1] * &b[1][0],
            &a[1][0] * &b[0][1] + &a[1][1] * &b[1][1],
        ],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_iterative_big() {
        assert_eq!(fib_iterative_big(0), BigUint::zero());
        assert_eq!(fib_iterative_big(1), BigUint::one());
        assert_eq!(fib_iterative_big(10), BigUint::from(55u32));
    }

    #[test]
    fn test_fib_matrix_big() {
        assert_eq!(fib_matrix_big(0), BigUint::zero());
        assert_eq!(fib_matrix_big(1), BigUint::one());
        assert_eq!(fib_matrix_big(10), BigUint::from(55u32));
        assert_eq!(fib_matrix_big(20), BigUint::from(6765u32));
    }

    #[test]
    fn test_large_fib() {
        // F(200) - verifies it works beyond u128
        // Value from WolframAlpha: 280571172992510140037611932413038677189525
        let f200 = fib_matrix_big(200);
        let s = f200.to_string();
        assert_eq!(s, "280571172992510140037611932413038677189525");
    }
}
