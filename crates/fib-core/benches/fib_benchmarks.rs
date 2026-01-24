//! Criterion benchmarks for Fibonacci implementations
//!
//! Run with: `cargo bench`

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use fib_core::{closed_form, iterative, matrix, recursive};

/// Benchmark comparing algorithm complexities
fn complexity_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("complexity_comparison");
    group.sample_size(100);

    // Test small n values where all algorithms are feasible
    for n in [10, 15, 20, 25].iter() {
        // Recursive is only practical for small n
        if *n <= 25 {
            group.bench_with_input(BenchmarkId::new("recursive", n), n, |b, &n| {
                b.iter(|| recursive::fib_recursive(black_box(n)))
            });
        }

        group.bench_with_input(BenchmarkId::new("recursive_memo", n), n, |b, &n| {
            b.iter(|| recursive::fib_recursive_memo(black_box(n)))
        });

        group.bench_with_input(BenchmarkId::new("iterative", n), n, |b, &n| {
            b.iter(|| iterative::fib_iterative(black_box(n)))
        });

        group.bench_with_input(BenchmarkId::new("matrix", n), n, |b, &n| {
            b.iter(|| matrix::fib_matrix_fast(black_box(n)))
        });

        group.bench_with_input(BenchmarkId::new("fast_doubling", n), n, |b, &n| {
            b.iter(|| matrix::fib_doubling(black_box(n)))
        });

        group.bench_with_input(BenchmarkId::new("binet", n), n, |b, &n| {
            b.iter(|| closed_form::fib_binet_f64(black_box(n)))
        });
    }

    group.finish();
}

/// Benchmark scaling behavior for larger n values
fn large_n_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_n");
    group.sample_size(50);

    for n in [100, 500, 1000, 5000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("iterative", n), n, |b, &n| {
            b.iter(|| iterative::fib_iterative(black_box(n)))
        });

        group.bench_with_input(BenchmarkId::new("matrix", n), n, |b, &n| {
            b.iter(|| matrix::fib_matrix_fast(black_box(n)))
        });

        group.bench_with_input(BenchmarkId::new("doubling", n), n, |b, &n| {
            b.iter(|| matrix::fib_doubling(black_box(n)))
        });
    }

    group.finish();
}

/// Benchmark iterative variants
fn iterative_variants(c: &mut Criterion) {
    let mut group = c.benchmark_group("iterative_variants");
    group.sample_size(100);

    for n in [50, 100, 500].iter() {
        group.bench_with_input(BenchmarkId::new("standard", n), n, |b, &n| {
            b.iter(|| iterative::fib_iterative(black_box(n)))
        });

        group.bench_with_input(BenchmarkId::new("branchless", n), n, |b, &n| {
            b.iter(|| iterative::fib_iterative_branchless(black_box(n)))
        });
    }

    group.finish();
}

/// Benchmark batch operations
fn batch_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch");
    group.sample_size(50);

    let small_batch: Vec<u64> = (1..=10).collect();
    let medium_batch: Vec<u64> = (1..=50).collect();
    let large_batch: Vec<u64> = (1..=100).collect();

    group.bench_function("batch_10", |b| {
        b.iter(|| iterative::fib_iterative_batch(black_box(&small_batch)))
    });

    group.bench_function("batch_50", |b| {
        b.iter(|| iterative::fib_iterative_batch(black_box(&medium_batch)))
    });

    group.bench_function("batch_100", |b| {
        b.iter(|| iterative::fib_iterative_batch(black_box(&large_batch)))
    });

    group.finish();
}

/// Benchmark cache vs direct calculation
fn cache_vs_direct(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_vs_direct");
    group.sample_size(100);

    let cache = iterative::FibonacciCache::new(100);
    let queries: Vec<u64> = vec![10, 25, 50, 75, 100];

    group.bench_function("direct_lookups", |b| {
        b.iter(|| {
            queries
                .iter()
                .map(|&n| iterative::fib_iterative(black_box(n)))
                .collect::<Vec<_>>()
        })
    });

    group.bench_function("cached_lookups", |b| {
        b.iter(|| {
            queries
                .iter()
                .map(|&n| cache.get(black_box(n)))
                .collect::<Vec<_>>()
        })
    });

    group.finish();
}

/// Benchmark modular arithmetic
fn modular_arithmetic(c: &mut Criterion) {
    let mut group = c.benchmark_group("modular");
    group.sample_size(50);

    let modulo = 1_000_000_007u128;

    for n in [1000, 10000, 100000].iter() {
        group.bench_with_input(BenchmarkId::new("matrix_mod", n), n, |b, &n| {
            b.iter(|| matrix::fib_matrix_modulo(black_box(n), black_box(modulo)))
        });
    }

    group.finish();
}

/// Benchmark SIMD batch operations (requires simd feature)
#[cfg(feature = "simd")]
fn simd_vs_scalar(c: &mut Criterion) {
    use fib_core::simd::fib_simd_batch;

    let mut group = c.benchmark_group("simd_vs_scalar");
    group.sample_size(100);

    // Test various batch sizes
    let batch_4: Vec<u64> = vec![10, 20, 30, 40];
    let batch_8: Vec<u64> = vec![10, 20, 30, 40, 50, 60, 70, 80];
    let batch_16: Vec<u64> = (10..26).collect();
    let batch_32: Vec<u64> = (10..42).collect();
    let batch_64: Vec<u64> = (10..74).collect();

    // SIMD benchmarks
    group.bench_function("simd_4", |b| b.iter(|| fib_simd_batch(black_box(&batch_4))));

    group.bench_function("simd_8", |b| b.iter(|| fib_simd_batch(black_box(&batch_8))));

    group.bench_function("simd_16", |b| {
        b.iter(|| fib_simd_batch(black_box(&batch_16)))
    });

    group.bench_function("simd_32", |b| {
        b.iter(|| fib_simd_batch(black_box(&batch_32)))
    });

    group.bench_function("simd_64", |b| {
        b.iter(|| fib_simd_batch(black_box(&batch_64)))
    });

    // Scalar benchmarks for comparison
    group.bench_function("scalar_4", |b| {
        b.iter(|| iterative::fib_iterative_batch(black_box(&batch_4)))
    });

    group.bench_function("scalar_8", |b| {
        b.iter(|| iterative::fib_iterative_batch(black_box(&batch_8)))
    });

    group.bench_function("scalar_16", |b| {
        b.iter(|| iterative::fib_iterative_batch(black_box(&batch_16)))
    });

    group.bench_function("scalar_32", |b| {
        b.iter(|| iterative::fib_iterative_batch(black_box(&batch_32)))
    });

    group.bench_function("scalar_64", |b| {
        b.iter(|| iterative::fib_iterative_batch(black_box(&batch_64)))
    });

    group.finish();
}

#[cfg(not(feature = "simd"))]
criterion_group!(
    benches,
    complexity_comparison,
    large_n_scaling,
    iterative_variants,
    batch_operations,
    cache_vs_direct,
    modular_arithmetic,
);

#[cfg(feature = "simd")]
criterion_group!(
    benches,
    complexity_comparison,
    large_n_scaling,
    iterative_variants,
    batch_operations,
    cache_vs_direct,
    modular_arithmetic,
    simd_vs_scalar,
);

criterion_main!(benches);
