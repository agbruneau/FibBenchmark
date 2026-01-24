//! Fibonacci Profiler
//!
//! Profiling and memory analysis tools for Fibonacci benchmarks

use fib_core::allocator::TrackingAllocator;
use fib_core::{iterative, matrix};
use std::time::{Duration, Instant};

#[global_allocator]
static ALLOCATOR: TrackingAllocator = TrackingAllocator::new();

fn main() {
    println!("🔬 Fibonacci Performance Profiler");
    println!("==================================");
    println!();

    // Profile different methods
    profile_iterative();
    profile_matrix();
    profile_memory_usage();
    profile_scaling();
    profile_flamegraph();
}

fn profile_iterative() {
    println!("📊 Iterative Method Profile");
    println!("─────────────────────────────");

    let test_values = [10, 100, 1000, 10000, 100000];

    for n in test_values {
        let iterations = 1000;
        let mut total_time = Duration::ZERO;

        for _ in 0..iterations {
            let start = Instant::now();
            let _ = iterative::fib_iterative(n);
            total_time += start.elapsed();
        }

        let avg_time = total_time / iterations;
        println!("  n = {:6}: avg {:?}", n, avg_time);
    }
    println!();
}

fn profile_matrix() {
    println!("📊 Matrix Method Profile");
    println!("─────────────────────────");

    let test_values = [10, 100, 1000, 10000, 100000];

    for n in test_values {
        let iterations = 1000;
        let mut total_time = Duration::ZERO;

        for _ in 0..iterations {
            let start = Instant::now();
            let _ = matrix::fib_matrix_fast(n);
            total_time += start.elapsed();
        }

        let avg_time = total_time / iterations;
        println!("  n = {:6}: avg {:?}", n, avg_time);
    }
    println!();
}

fn profile_memory_usage() {
    println!("📊 Memory Analysis");
    println!("──────────────────");

    println!("  Iterative: O(1) - uses only 2 u128 values = 32 bytes");
    println!("  Matrix:    O(1) - uses 2x2 matrix of u128 = 64 bytes");
    println!("  Recursive Memo: O(n) - allocates Vec<u128>");
    println!();

    // Reset stats before measurement
    ALLOCATOR.reset();
    let _initial_usage = ALLOCATOR.get_current_usage();

    // Demonstrate recursive memo memory
    for n in [100, 1000, 10000] {
        let before_alloc = ALLOCATOR.get_allocation_count();

        let _result = fib_core::recursive::fib_recursive_memo(n);

        // Note: Vec is deallocated when _result is dropped? No, fib_recursive_memo returns basic type u128.
        // The implementation of fib_recursive_memo creates a cache internally and drops it.
        // So we might need to query the maximum usage *during* the call if we could,
        // but here we are checking the net effect or traffic.
        // Let's rely on theoretical calculation for now but printed alongside real tracking if possible.
        // Actually, since the Vec is dropped inside the function, the 'current usage' will return to initial.
        // To track peak usage, our simple allocator needs peak tracking.
        // But the plan was just to "show real stats".
        // Let's show TOTAL allocations made.

        let after_alloc = ALLOCATOR.get_allocation_count();
        let alloc_count = after_alloc - before_alloc;

        let theoretical_bytes = (n + 1) * 16; // u128 = 16 bytes
        println!(
            "  Recursive Memo (n={:<5}): ~{:>6} bytes theoretical. Allocations made: {}",
            n, theoretical_bytes, alloc_count
        );
    }

    // Demonstrate explicit allocation for benchmark
    println!();
    println!("  Tracking test (vector allocation):");
    let prev = ALLOCATOR.get_current_usage();
    let mut vec = Vec::new();
    for i in 0..100 {
        vec.push(i as u128);
    }
    let current = ALLOCATOR.get_current_usage();
    println!(
        "    Allocated vector of 100 u128: {} bytes (System tracked)",
        current - prev
    );
    println!("    (Expected: ~1600 bytes + capacity overhead)");

    // Keep vec alive to measure
    std::mem::forget(vec);

    println!();
}

fn profile_scaling() {
    println!("📊 Scaling Analysis (Iterative vs Matrix)");
    println!("──────────────────────────────────────────");

    let test_values = [100, 1000, 10000, 100000];
    let iterations = 100;

    println!(
        "  {:>10} │ {:>15} │ {:>15} │ {:>10}",
        "n", "Iterative", "Matrix", "Speedup"
    );
    println!("  ───────────┼─────────────────┼─────────────────┼───────────");

    for n in test_values {
        // Time iterative
        let mut iter_time = Duration::ZERO;
        for _ in 0..iterations {
            let start = Instant::now();
            let _ = iterative::fib_iterative(n);
            iter_time += start.elapsed();
        }
        let iter_avg = iter_time / iterations;

        // Time matrix
        let mut matrix_time = Duration::ZERO;
        for _ in 0..iterations {
            let start = Instant::now();
            let _ = matrix::fib_matrix_fast(n);
            matrix_time += start.elapsed();
        }
        let matrix_avg = matrix_time / iterations;

        let speedup = iter_avg.as_nanos() as f64 / matrix_avg.as_nanos() as f64;

        println!(
            "  {:>10} │ {:>15?} │ {:>15?} │ {:>10.2}x",
            n, iter_avg, matrix_avg, speedup
        );
    }
    println!();
    println!("💡 Matrix method shows O(log n) advantage for large n");
}

fn profile_flamegraph() {
    println!("🔥 Flamegraph Generation");
    println!("──────────────────────");

    #[cfg(unix)]
    {
        use std::fs::File;

        println!("  Capturing profile for flamegraph...");
        let guard = pprof::ProfilerGuard::new(100).unwrap();

        // Run a heavy computation to profile
        for _ in 0..10_000 {
            let _ = matrix::fib_matrix_fast(10_000);
        }

        if let Ok(report) = guard.report().build() {
            let file = File::create("flamegraph.svg").unwrap();
            report.flamegraph(file).unwrap();
            let file_path = "flamegraph.svg";
            let file = File::create(file_path).unwrap();
            report.flamegraph(file).unwrap();
            println!("  ✅ Generated: {}", file_path);
            println!("  ℹ️  Open this SVG file in a browser to view the flamegraph.");
        } else {
            println!("  ❌ Failed to generate flamegraph");
        }
    }

    #[cfg(not(unix))]
    {
        println!("  ⚠️  Flamegraph generation only supported on Unix systems (requires pprof)");
        println!("  (This is expected behavior on Windows)");
    }
    println!();
}
