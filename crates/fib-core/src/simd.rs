//! SIMD-optimized Fibonacci batch calculations
//!
//! This module provides SIMD-accelerated batch Fibonacci calculations using
//! the `wide` crate for portable SIMD operations on stable Rust.
//!
//! # SIMD Approach
//!
//! Since Fibonacci calculation is inherently sequential (F(n) depends on F(n-1) and F(n-2)),
//! true SIMD parallelization of a single calculation isn't possible. Instead, we parallelize
//! by calculating **multiple independent Fibonacci values simultaneously**.
//!
//! # Example
//!
//! ```
//! # #[cfg(feature = "simd")]
//! # {
//! use fib_core::simd::{fib_simd_batch, SimdFeatures};
//!
//! let indices = vec![10, 20, 30, 40];
//! let results = fib_simd_batch(&indices);
//!
//! assert_eq!(results[0], 55);       // F(10)
//! assert_eq!(results[1], 6765);     // F(20)
//! assert_eq!(results[2], 832040);   // F(30)
//! assert_eq!(results[3], 102334155); // F(40)
//!
//! // Check available SIMD features
//! let features = SimdFeatures::detect();
//! println!("SIMD support: {:?}", features);
//! # }
//! ```

use wide::u64x4;

/// SIMD feature detection results
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimdFeatures {
    /// SSE2 is available (always true on x86-64)
    pub sse2: bool,
    /// SSE4.1 is available
    pub sse41: bool,
    /// AVX is available
    pub avx: bool,
    /// AVX2 is available (256-bit integer SIMD)
    pub avx2: bool,
    /// AVX-512F is available (512-bit SIMD)
    pub avx512f: bool,
}

impl SimdFeatures {
    /// Detect available SIMD features on the current CPU
    #[cfg(target_arch = "x86_64")]
    pub fn detect() -> Self {
        Self {
            sse2: is_x86_feature_detected!("sse2"),
            sse41: is_x86_feature_detected!("sse4.1"),
            avx: is_x86_feature_detected!("avx"),
            avx2: is_x86_feature_detected!("avx2"),
            avx512f: is_x86_feature_detected!("avx512f"),
        }
    }

    /// Detect available SIMD features on non-x86 platforms
    #[cfg(not(target_arch = "x86_64"))]
    pub fn detect() -> Self {
        Self {
            sse2: false,
            sse41: false,
            avx: false,
            avx2: false,
            avx512f: false,
        }
    }

    /// Get a human-readable description of available features
    pub fn description(&self) -> String {
        let mut features = Vec::new();
        if self.avx512f {
            features.push("AVX-512F");
        }
        if self.avx2 {
            features.push("AVX2");
        }
        if self.avx {
            features.push("AVX");
        }
        if self.sse41 {
            features.push("SSE4.1");
        }
        if self.sse2 {
            features.push("SSE2");
        }
        if features.is_empty() {
            "No SIMD".to_string()
        } else {
            features.join(", ")
        }
    }

    /// Get the best available SIMD width in bits
    pub fn best_width(&self) -> u32 {
        if self.avx512f {
            512
        } else if self.avx2 {
            256
        } else if self.sse2 {
            128
        } else {
            64 // scalar fallback
        }
    }
}

impl std::fmt::Display for SimdFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

/// Calculate Fibonacci for a batch of indices using SIMD parallelism
///
/// Processes indices in groups of 4 using 64-bit SIMD lanes.
/// Falls back to scalar calculation for remaining elements.
///
/// # Arguments
/// * `indices` - Slice of Fibonacci indices to calculate
///
/// # Returns
/// Vector of Fibonacci numbers corresponding to each index
///
/// # Example
/// ```
/// # #[cfg(feature = "simd")]
/// # {
/// use fib_core::simd::fib_simd_batch;
///
/// let results = fib_simd_batch(&[5, 10, 15, 20]);
/// assert_eq!(results, vec![5, 55, 610, 6765]);
/// # }
/// ```
pub fn fib_simd_batch(indices: &[u64]) -> Vec<u64> {
    let mut results = Vec::with_capacity(indices.len());
    let chunks = indices.chunks_exact(4);
    let remainder = chunks.remainder();

    // Process 4 indices at a time using SIMD
    for chunk in chunks {
        let batch: [u64; 4] = [chunk[0], chunk[1], chunk[2], chunk[3]];
        let simd_results = fib_simd_4(batch);
        results.extend_from_slice(&simd_results);
    }

    // Handle remaining elements with scalar calculation
    for &n in remainder {
        results.push(fib_scalar(n));
    }

    results
}

/// Calculate 4 Fibonacci numbers simultaneously using SIMD
///
/// Uses u64x4 (4 lanes of 64-bit integers) for parallel calculation.
/// Each lane independently calculates its own Fibonacci number.
#[inline]
fn fib_simd_4(indices: [u64; 4]) -> [u64; 4] {
    // For SIMD to be effective, all lanes should iterate the same number of times
    // We iterate to max_n and each lane accumulates its result
    let max_n = indices.iter().copied().max().unwrap_or(0);

    if max_n == 0 {
        return indices.map(|n| if n == 0 { 0 } else { 1 });
    }

    // Initialize separate a and b for each lane
    let mut a = u64x4::from([0, 0, 0, 0]);
    let mut b = u64x4::from([1, 1, 1, 1]);

    // Target indices
    let targets = u64x4::from(indices);

    // Track which iteration each lane should stop at
    // We'll iterate to max_n and use conditional logic

    for i in 0..max_n {
        let current_iter = u64x4::splat(i);

        // Create a mask: 1 if we should still iterate, 0 otherwise
        // We iterate while current_iter < target (i.e., target > current_iter)
        let should_update = targets.cmp_gt(current_iter);

        // temp = a + b
        let temp = a + b;

        // Use blend to conditionally update
        // If should_update is true for a lane, update; otherwise keep old value
        a = should_update.blend(b, a);
        b = should_update.blend(temp, b);
    }

    a.to_array()
}

/// Scalar Fibonacci calculation for fallback
#[inline]
fn fib_scalar(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let (mut a, mut b) = (0u64, 1u64);
    for _ in 2..=n {
        let temp = a.wrapping_add(b);
        a = b;
        b = temp;
    }
    b
}

/// Batch calculator with statistics and comparison support
#[derive(Debug)]
pub struct SimdBatchCalculator {
    features: SimdFeatures,
}

impl SimdBatchCalculator {
    /// Create a new SIMD batch calculator
    pub fn new() -> Self {
        Self {
            features: SimdFeatures::detect(),
        }
    }

    /// Get detected SIMD features
    pub fn features(&self) -> &SimdFeatures {
        &self.features
    }

    /// Calculate Fibonacci batch using SIMD
    pub fn calculate(&self, indices: &[u64]) -> Vec<u64> {
        fib_simd_batch(indices)
    }

    /// Calculate Fibonacci batch using scalar (for comparison)
    pub fn calculate_scalar(&self, indices: &[u64]) -> Vec<u64> {
        indices.iter().map(|&n| fib_scalar(n)).collect()
    }

    /// Compare SIMD vs scalar performance
    ///
    /// Returns (simd_duration, scalar_duration) in nanoseconds
    pub fn benchmark(&self, indices: &[u64], iterations: u32) -> (u128, u128) {
        use std::time::Instant;

        // SIMD timing
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = std::hint::black_box(fib_simd_batch(std::hint::black_box(indices)));
        }
        let simd_ns = start.elapsed().as_nanos();

        // Scalar timing
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = std::hint::black_box(self.calculate_scalar(std::hint::black_box(indices)));
        }
        let scalar_ns = start.elapsed().as_nanos();

        (simd_ns / iterations as u128, scalar_ns / iterations as u128)
    }
}

impl Default for SimdBatchCalculator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_scalar_base_cases() {
        assert_eq!(fib_scalar(0), 0);
        assert_eq!(fib_scalar(1), 1);
        assert_eq!(fib_scalar(2), 1);
    }

    #[test]
    fn test_fib_scalar_known_values() {
        assert_eq!(fib_scalar(10), 55);
        assert_eq!(fib_scalar(20), 6765);
        assert_eq!(fib_scalar(30), 832040);
        assert_eq!(fib_scalar(40), 102334155);
    }

    #[test]
    fn test_fib_simd_batch_basic() {
        let results = fib_simd_batch(&[0, 1, 2, 3]);
        assert_eq!(results, vec![0, 1, 1, 2]);
    }

    #[test]
    fn test_fib_simd_batch_known_values() {
        let results = fib_simd_batch(&[10, 20, 30, 40]);
        assert_eq!(results, vec![55, 6765, 832040, 102334155]);
    }

    #[test]
    fn test_fib_simd_batch_non_aligned() {
        // Test with non-multiple-of-4 length
        let results = fib_simd_batch(&[5, 10, 15, 20, 25]);
        assert_eq!(results, vec![5, 55, 610, 6765, 75025]);
    }

    #[test]
    fn test_fib_simd_batch_empty() {
        let results = fib_simd_batch(&[]);
        assert!(results.is_empty());
    }

    #[test]
    fn test_simd_features_detection() {
        let features = SimdFeatures::detect();
        // On x86-64, SSE2 should always be available
        #[cfg(target_arch = "x86_64")]
        assert!(features.sse2);

        // Description should not be empty
        assert!(!features.description().is_empty());
    }

    #[test]
    fn test_simd_batch_calculator() {
        let calc = SimdBatchCalculator::new();
        let indices = vec![5, 10, 15, 20];

        let simd_results = calc.calculate(&indices);
        let scalar_results = calc.calculate_scalar(&indices);

        assert_eq!(simd_results, scalar_results);
    }

    #[test]
    fn test_simd_matches_scalar() {
        let _calc = SimdBatchCalculator::new();

        // Test a range of values
        for n in 0..50 {
            let simd = fib_simd_batch(&[n])[0];
            let scalar = fib_scalar(n);
            assert_eq!(simd, scalar, "Mismatch at n={}", n);
        }
    }

    #[test]
    fn test_large_batch() {
        let indices: Vec<u64> = (0..100).collect();
        let results = fib_simd_batch(&indices);

        assert_eq!(results.len(), 100);
        assert_eq!(results[0], 0);
        assert_eq!(results[1], 1);
        assert_eq!(results[10], 55);
        assert_eq!(results[20], 6765);
    }
}
