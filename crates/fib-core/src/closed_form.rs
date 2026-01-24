//! Closed-form Fibonacci implementations
//!
//! This module provides the Binet formula implementation, which calculates
//! Fibonacci numbers in O(1) time but with floating-point precision limitations.
//!
//! ## Binet Formula
//!
//! F(n) = (φ^n - ψ^n) / √5
//!
//! Where:
//! - φ = (1 + √5) / 2 ≈ 1.618... (the golden ratio)
//! - ψ = (1 - √5) / 2 ≈ -0.618...
//!
//! ## Precision Warning
//!
//! Due to IEEE 754 double precision limits, this formula produces accurate
//! results only for n ≤ 78. Beyond that, floating-point errors accumulate.

/// The golden ratio φ = (1 + √5) / 2
pub const PHI: f64 = 1.618033988749895;

/// The conjugate of the golden ratio ψ = (1 - √5) / 2
pub const PSI: f64 = -0.6180339887498949;

/// Square root of 5
pub const SQRT_5: f64 = 2.23606797749979;

/// Maximum n for accurate Binet calculation with f64
pub const MAX_ACCURATE_N: u64 = 78;

/// Fibonacci using Binet's closed-form formula
///
/// This calculates F(n) in O(1) time using the formula:
/// F(n) = (φ^n - ψ^n) / √5
///
/// # Complexity
/// - Time: O(1)
/// - Space: O(1)
///
/// # Precision Warning
/// Results are only accurate for n ≤ 78 due to floating-point limitations.
///
/// # Example
/// ```
/// use fib_core::closed_form::fib_binet_f64;
///
/// let fib_10 = fib_binet_f64(10);
/// assert!((fib_10 - 55.0).abs() < 0.001);
///
/// let fib_50 = fib_binet_f64(50);
/// assert!((fib_50 - 12586269025.0).abs() < 1.0);
/// ```
pub fn fib_binet_f64(n: u64) -> f64 {
    if n == 0 {
        return 0.0;
    }

    let sqrt5 = 5.0_f64.sqrt();
    let phi = (1.0 + sqrt5) / 2.0;
    let psi = (1.0 - sqrt5) / 2.0;

    (phi.powi(n as i32) - psi.powi(n as i32)) / sqrt5
}

/// Binet formula using logarithmic calculation for large n support
///
/// Returns a tuple (mantissa, exponent) representing the value in base 10:
/// result = mantissa * 10^exponent
///
/// This avoids overflow for n > 1476 where standard f64 fails.
///
/// # Example
/// ```
/// use fib_core::closed_form::fib_binet_scientific;
///
/// // For n=10, F(10) = 55 = 5.5 * 10^1
/// let (mantissa, exponent) = fib_binet_scientific(10);
/// assert!((mantissa - 5.5).abs() < 0.001);
/// assert_eq!(exponent, 1);
/// ```
pub fn fib_binet_scientific(n: u64) -> (f64, i32) {
    if n == 0 {
        return (0.0, 0);
    }

    // log10(F_n) ≈ n * log10(phi) - log10(sqrt(5))
    // We ignore the psi term because for n > 0 it is negligible

    let log_phi = PHI.log10();
    let log_sqrt5 = SQRT_5.log10();

    let approx_log = (n as f64) * log_phi - log_sqrt5;

    let exponent = approx_log.floor() as i32;
    let mantissa = 10.0_f64.powf(approx_log - exponent as f64);

    (mantissa, exponent)
}

/// Binet formula with rounding to nearest integer
///
/// Since F(n) is always an integer, we can round the Binet result.
/// This improves accuracy for values just at the edge of precision.
///
/// # Example
/// ```
/// use fib_core::closed_form::fib_binet_rounded;
///
/// assert_eq!(fib_binet_rounded(10), 55);
/// assert_eq!(fib_binet_rounded(50), 12586269025);
/// ```
pub fn fib_binet_rounded(n: u64) -> u128 {
    fib_binet_f64(n).round() as u128
}

/// Simplified Binet using only φ (ignoring ψ^n which approaches 0)
///
/// For large n, ψ^n becomes negligible since |ψ| < 1.
/// We can approximate: F(n) ≈ φ^n / √5
///
/// # Example
/// ```
/// use fib_core::closed_form::fib_binet_simplified;
///
/// // Good approximation for large n
/// let approx = fib_binet_simplified(20);
/// assert!((approx - 6765.0).abs() < 1.0);
/// ```
pub fn fib_binet_simplified(n: u64) -> f64 {
    if n == 0 {
        return 0.0;
    }

    let sqrt5 = 5.0_f64.sqrt();
    let phi = (1.0 + sqrt5) / 2.0;

    (phi.powi(n as i32) / sqrt5).round()
}

/// Analyze the error of Binet formula compared to exact value
///
/// Returns (absolute_error, relative_error) tuple.
///
/// # Example
/// ```
/// use fib_core::closed_form::binet_error_analysis;
///
/// let (abs_err, rel_err) = binet_error_analysis(50);
/// println!("Absolute error: {}", abs_err);
/// println!("Relative error: {}", rel_err);
/// ```
pub fn binet_error_analysis(n: u64) -> (f64, f64) {
    let fib_approx = fib_binet_f64(n);
    let fib_exact = crate::iterative::fib_iterative(n) as f64;

    let absolute_error = (fib_approx - fib_exact).abs();
    let relative_error = if fib_exact != 0.0 {
        absolute_error / fib_exact
    } else {
        0.0
    };

    (absolute_error, relative_error)
}

/// Find the maximum n where Binet formula gives exact integer results
///
/// # Example
/// ```
/// use fib_core::closed_form::find_binet_accuracy_limit;
///
/// let limit = find_binet_accuracy_limit();
/// println!("Binet is accurate up to n = {}", limit);
/// ```
pub fn find_binet_accuracy_limit() -> u64 {
    for n in 0..200 {
        let binet_result = fib_binet_rounded(n);
        let exact_result = crate::iterative::fib_iterative(n);

        if binet_result != exact_result {
            return n - 1;
        }
    }
    200
}

/// Calculate the ratio F(n+1)/F(n) which approaches φ
///
/// # Example
/// ```
/// use fib_core::closed_form::{fibonacci_ratio, PHI};
///
/// let ratio = fibonacci_ratio(20);
/// assert!((ratio - PHI).abs() < 0.0001);
/// ```
pub fn fibonacci_ratio(n: u64) -> f64 {
    if n == 0 {
        return f64::INFINITY;
    }

    let fn_val = crate::iterative::fib_iterative(n) as f64;
    let fn_plus_1 = crate::iterative::fib_iterative(n + 1) as f64;

    fn_plus_1 / fn_val
}

/// Calculate how close F(n+1)/F(n) is to the golden ratio
pub fn convergence_to_phi(n: u64) -> f64 {
    let ratio = fibonacci_ratio(n);
    (ratio - PHI).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binet_base_cases() {
        assert_eq!(fib_binet_f64(0), 0.0);
        assert!((fib_binet_f64(1) - 1.0).abs() < 0.001);
        assert!((fib_binet_f64(2) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_binet_known_values() {
        assert!((fib_binet_f64(10) - 55.0).abs() < 0.001);
        assert!((fib_binet_f64(20) - 6765.0).abs() < 0.01);
    }

    #[test]
    fn test_binet_rounded() {
        assert_eq!(fib_binet_rounded(10), 55);
        assert_eq!(fib_binet_rounded(20), 6765);
        assert_eq!(fib_binet_rounded(50), 12586269025);
    }

    #[test]
    fn test_error_analysis() {
        let (abs_err, rel_err) = binet_error_analysis(30);
        assert!(abs_err < 1.0);
        assert!(rel_err < 1e-10);
    }

    #[test]
    fn test_fibonacci_ratio_converges() {
        let ratio_10 = fibonacci_ratio(10);
        let ratio_20 = fibonacci_ratio(20);
        let ratio_50 = fibonacci_ratio(50);

        // Should get closer to PHI as n increases
        assert!((ratio_50 - PHI).abs() < (ratio_20 - PHI).abs());
        assert!((ratio_20 - PHI).abs() < (ratio_10 - PHI).abs());
    }

    #[test]
    fn test_golden_ratio_constant() {
        let calculated_phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
        assert!((PHI - calculated_phi).abs() < 1e-10);
    }

    #[test]
    fn test_fib_binet_simplified_large_n() {
        // For large n, simplified binet should be close to regular binet
        let binet = fib_binet_f64(50);
        let simplified = fib_binet_simplified(50);
        assert!((binet - simplified).abs() < 1.0);
    }

    #[test]
    fn test_convergence_to_phi_properties() {
        // The convergence error should decrease as n increases
        let err_10 = convergence_to_phi(10);
        let err_20 = convergence_to_phi(20);
        assert!(err_20 < err_10);
    }

    #[test]
    fn test_find_binet_accuracy_limit_range() {
        let limit = find_binet_accuracy_limit();
        // Should be around 70-78
        assert!(limit >= 70 && limit <= 78);
    }
}
