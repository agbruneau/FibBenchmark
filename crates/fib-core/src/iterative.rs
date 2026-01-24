//! Iterative Fibonacci implementations
//!
//! This module provides efficient iterative approaches to calculating Fibonacci numbers.
//! The iterative approach is the standard recommended method for most use cases.
//!
//! ## Complexity
//! - Time: O(n)
//! - Space: O(1)

/// Standard iterative Fibonacci
///
/// # Complexity
/// - Time: O(n)
/// - Space: O(1) - constant space
///
/// # Example
/// ```
/// use fib_core::iterative::fib_iterative;
///
/// assert_eq!(fib_iterative(0), 0);
/// assert_eq!(fib_iterative(1), 1);
/// assert_eq!(fib_iterative(10), 55);
/// assert_eq!(fib_iterative(100), 354224848179261915075);
/// ```
pub fn fib_iterative(n: u64) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let (mut a, mut b) = (0u128, 1u128);
            for _ in 2..=n {
                let temp = a + b;
                a = b;
                b = temp;
            }
            b
        }
    }
}

/// Branchless iterative Fibonacci for CPU pipeline optimization
///
/// This version avoids conditional branches in the main loop,
/// which can improve performance on modern CPUs due to better
/// branch prediction and pipeline utilization.
///
/// # Complexity
/// - Time: O(n)
/// - Space: O(1)
///
/// # Example
/// ```
/// use fib_core::iterative::fib_iterative_branchless;
///
/// assert_eq!(fib_iterative_branchless(10), 55);
/// assert_eq!(fib_iterative_branchless(50), 12586269025);
/// ```
#[inline]
pub fn fib_iterative_branchless(n: u64) -> u128 {
    let (mut a, mut b) = (0u128, 1u128);
    for _ in 0..n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    a
}

/// Batch calculation of multiple Fibonacci numbers
///
/// Efficiently calculates Fibonacci for multiple values of n.
///
/// # Example
/// ```
/// use fib_core::iterative::fib_iterative_batch;
///
/// let ns = vec![5, 10, 15, 20];
/// let results = fib_iterative_batch(&ns);
///
/// assert_eq!(results, vec![5, 55, 610, 6765]);
/// ```
pub fn fib_iterative_batch(ns: &[u64]) -> Vec<u128> {
    if ns.is_empty() {
        return Vec::new();
    }

    // Map to (original_index, n) to preserve order
    let mut indexed_ns: Vec<(usize, u64)> = ns.iter().copied().enumerate().collect();
    // Sort by n to allow incremental calculation
    indexed_ns.sort_unstable_by_key(|&(_, n)| n);

    let mut results = vec![0u128; ns.len()];
    let mut current_n = 0;
    let mut a = 0u128; // F(current_n)
    let mut b = 1u128; // F(current_n + 1)

    for (original_index, target_n) in indexed_ns {
        if target_n == 0 {
            results[original_index] = 0;
            continue;
        }

        // Advance state to target_n
        while current_n < target_n {
            let temp = a + b;
            a = b;
            b = temp;
            current_n += 1;
        }

        results[original_index] = a;
    }

    results
}

/// Calculate Fibonacci with a maximum n cache for repeated queries
///
/// Pre-computes all Fibonacci numbers up to max_n for O(1) lookups.
///
/// # Example
/// ```
/// use fib_core::iterative::FibonacciCache;
///
/// let cache = FibonacciCache::new(100);
/// assert_eq!(cache.get(50), Some(12586269025));
/// assert_eq!(cache.get(101), None);  // Beyond cache limit
/// ```
pub struct FibonacciCache {
    values: Vec<u128>,
}

impl FibonacciCache {
    /// Create a new cache with all Fibonacci numbers up to n
    pub fn new(max_n: u64) -> Self {
        let size = (max_n + 1) as usize;
        let mut values = vec![0u128; size];

        if size > 0 {
            values[0] = 0;
        }
        if size > 1 {
            values[1] = 1;
        }

        for i in 2..size {
            values[i] = values[i - 1] + values[i - 2];
        }

        Self { values }
    }

    /// Get a cached Fibonacci value
    ///
    /// Returns None if n is beyond the cache limit
    pub fn get(&self, n: u64) -> Option<u128> {
        self.values.get(n as usize).copied()
    }

    /// Get the maximum n that can be retrieved from this cache
    pub fn max_n(&self) -> u64 {
        (self.values.len() - 1) as u64
    }
}

/// Iterator over Fibonacci numbers
///
/// # Example
/// ```
/// use fib_core::iterative::FibonacciIterator;
///
/// let fibs: Vec<u128> = FibonacciIterator::new().take(10).collect();
/// assert_eq!(fibs, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
/// ```
pub struct FibonacciIterator {
    current: u128,
    next: u128,
}

impl FibonacciIterator {
    pub fn new() -> Self {
        Self {
            current: 0,
            next: 1,
        }
    }
}

impl Default for FibonacciIterator {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for FibonacciIterator {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current;
        let new_next = self.current + self.next;
        self.current = self.next;
        self.next = new_next;
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_iterative_base_cases() {
        assert_eq!(fib_iterative(0), 0);
        assert_eq!(fib_iterative(1), 1);
        assert_eq!(fib_iterative(2), 1);
    }

    #[test]
    fn test_fib_iterative_known_values() {
        assert_eq!(fib_iterative(10), 55);
        assert_eq!(fib_iterative(20), 6765);
        assert_eq!(fib_iterative(50), 12586269025);
    }

    #[test]
    fn test_branchless_matches_standard() {
        for n in 0..100 {
            assert_eq!(
                fib_iterative(n),
                fib_iterative_branchless(n),
                "Mismatch at n={}",
                n
            );
        }
    }

    #[test]
    fn test_batch_calculation() {
        let ns = vec![0, 1, 5, 10, 20];
        let results = fib_iterative_batch(&ns);
        assert_eq!(results, vec![0, 1, 5, 55, 6765]);
    }

    #[test]
    fn test_batch_calculation_unsorted_duplicates() {
        let ns = vec![10, 5, 10, 0, 1];
        let results = fib_iterative_batch(&ns);
        // F(10)=55, F(5)=5, F(10)=55, F(0)=0, F(1)=1
        assert_eq!(results, vec![55, 5, 55, 0, 1]);
    }

    #[test]
    fn test_fibonacci_cache() {
        let cache = FibonacciCache::new(100);
        assert_eq!(cache.get(0), Some(0));
        assert_eq!(cache.get(1), Some(1));
        assert_eq!(cache.get(50), Some(12586269025));
        assert_eq!(cache.get(100), Some(354224848179261915075));
        assert_eq!(cache.get(101), None);
    }

    #[test]
    fn test_fibonacci_iterator() {
        let fibs: Vec<u128> = FibonacciIterator::new().take(10).collect();
        assert_eq!(fibs, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn test_fibonacci_iterator_manual_next() {
        let mut iter = FibonacciIterator::new();
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
    }

    #[test]
    fn test_fibonacci_cache_limit() {
        let cache = FibonacciCache::new(10);
        assert_eq!(cache.max_n(), 10);
        assert!(cache.get(10).is_some());
        assert!(cache.get(11).is_none());
    }
}
