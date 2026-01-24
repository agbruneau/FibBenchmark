//! Recursive Fibonacci implementations
//!
//! This module provides recursive approaches to calculating Fibonacci numbers,
//! including the naive exponential approach and memoized version.
//!
//! ## Warning
//!
//! The naive recursive implementation has O(2^n) complexity and should only
//! be used for demonstration purposes with small n values (n < 30).

/// Naive recursive Fibonacci - for demonstration only
///
/// # Complexity
/// - Time: O(2^n) - exponential
/// - Space: O(n) - call stack depth
///
/// # Warning
/// This implementation is extremely slow for n > 30.
/// For n=50, this would require ~10^15 operations!
///
/// # Example
/// ```
/// use fib_core::recursive::fib_recursive;
///
/// assert_eq!(fib_recursive(10), 55);
/// assert_eq!(fib_recursive(0), 0);
/// assert_eq!(fib_recursive(1), 1);
/// ```
pub fn fib_recursive(n: u64) -> u128 {
    if n <= 1 {
        n as u128
    } else {
        fib_recursive(n - 1) + fib_recursive(n - 2)
    }
}

/// Recursive Fibonacci with memoization
///
/// Uses dynamic programming with a memoization table to achieve O(n) time complexity.
///
/// # Complexity
/// - Time: O(n)
/// - Space: O(n) for the memo table
///
/// # Example
/// ```
/// use fib_core::recursive::fib_recursive_memo;
///
/// assert_eq!(fib_recursive_memo(50), 12586269025);
/// assert_eq!(fib_recursive_memo(100), 354224848179261915075);
/// ```
pub fn fib_recursive_memo(n: u64) -> u128 {
    if n == 0 {
        return 0;
    }
    let mut memo = vec![0u128; (n + 1) as usize];
    fib_recursive_memo_impl(n, &mut memo)
}

#[inline]
fn fib_recursive_memo_impl(n: u64, memo: &mut [u128]) -> u128 {
    if n <= 1 {
        return n as u128;
    }

    if memo[n as usize] != 0 {
        return memo[n as usize];
    }

    memo[n as usize] = fib_recursive_memo_impl(n - 1, memo) + fib_recursive_memo_impl(n - 2, memo);
    memo[n as usize]
}

/// Count the number of recursive calls for naive implementation
///
/// Useful for demonstrating the exponential nature of the naive approach.
///
/// # Example
/// ```
/// use fib_core::recursive::count_recursive_calls;
///
/// let calls_n10 = count_recursive_calls(10);
/// let calls_n20 = count_recursive_calls(20);
///
/// // Shows exponential growth
/// assert!(calls_n20 > calls_n10 * 100);
/// ```
pub fn count_recursive_calls(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        1 + count_recursive_calls(n - 1) + count_recursive_calls(n - 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_recursive_base_cases() {
        assert_eq!(fib_recursive(0), 0);
        assert_eq!(fib_recursive(1), 1);
        assert_eq!(fib_recursive(2), 1);
    }

    #[test]
    fn test_fib_recursive_known_values() {
        assert_eq!(fib_recursive(10), 55);
        assert_eq!(fib_recursive(15), 610);
        assert_eq!(fib_recursive(20), 6765);
    }

    #[test]
    fn test_fib_recursive_memo() {
        assert_eq!(fib_recursive_memo(0), 0);
        assert_eq!(fib_recursive_memo(1), 1);
        assert_eq!(fib_recursive_memo(50), 12586269025);
    }

    #[test]
    fn test_count_calls_grows_exponentially() {
        let calls_10 = count_recursive_calls(10);
        let calls_15 = count_recursive_calls(15);

        // Approximately 3x growth per 5 increments in n
        assert!(calls_15 > calls_10 * 2);
    }

    #[test]
    fn test_count_recursive_calls_base_cases() {
        assert_eq!(count_recursive_calls(0), 1);
        assert_eq!(count_recursive_calls(1), 1);
        assert_eq!(count_recursive_calls(2), 3); // 1 + calls(1) + calls(0) = 1 + 1 + 1 = 3
    }
}
