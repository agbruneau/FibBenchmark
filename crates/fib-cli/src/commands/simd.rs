//! SIMD command implementation
//!
//! Demonstrates SIMD-accelerated batch Fibonacci calculations

use fib_core::simd::{fib_simd_batch, SimdBatchCalculator, SimdFeatures};
use std::time::Instant;

/// Run the SIMD demonstration command
pub fn run(batch: &[u64], show_info: bool, compare: bool) {
    println!("\n🚀 SIMD Fibonacci Batch Calculator\n");
    println!("{}", "=".repeat(50));

    // Show SIMD feature info
    let features = SimdFeatures::detect();
    println!("\n📊 CPU SIMD Features:");
    println!("   SSE2:    {}", if features.sse2 { "✅" } else { "❌" });
    println!("   SSE4.1:  {}", if features.sse41 { "✅" } else { "❌" });
    println!("   AVX:     {}", if features.avx { "✅" } else { "❌" });
    println!("   AVX2:    {}", if features.avx2 { "✅" } else { "❌" });
    println!("   AVX-512: {}", if features.avx512f { "✅" } else { "❌" });
    println!("   Best width: {}-bit", features.best_width());

    if show_info {
        println!("\n💡 SIMD Info:");
        println!("   SIMD (Single Instruction, Multiple Data) allows calculating");
        println!("   multiple Fibonacci numbers in parallel using vector operations.");
        println!("   This implementation uses 4-wide u64 lanes (256-bit AVX2).");
        return;
    }

    if batch.is_empty() {
        println!("\n⚠️ No indices provided. Use --batch to specify indices.");
        println!("   Example: fib-bench simd --batch 10,20,30,40");
        return;
    }

    println!("\n📝 Input batch: {:?}", batch);
    println!("   Batch size: {} indices", batch.len());

    // Calculate using SIMD
    let start = Instant::now();
    let results = fib_simd_batch(batch);
    let simd_duration = start.elapsed();

    println!("\n✅ Results:");
    for (i, (&n, &result)) in batch.iter().zip(results.iter()).enumerate() {
        println!("   F({}) = {}", n, result);
        if i >= 9 && batch.len() > 10 {
            println!("   ... and {} more", batch.len() - 10);
            break;
        }
    }

    println!("\n⏱️ SIMD batch time: {:?}", simd_duration);

    // Performance comparison
    if compare {
        let calc = SimdBatchCalculator::new();
        let iterations = 1000;

        println!("\n📈 Performance Comparison ({} iterations):", iterations);
        let (simd_ns, scalar_ns) = calc.benchmark(batch, iterations);

        let speedup = if simd_ns > 0 {
            scalar_ns as f64 / simd_ns as f64
        } else {
            0.0
        };

        println!("   SIMD:   {} ns/batch", simd_ns);
        println!("   Scalar: {} ns/batch", scalar_ns);
        println!(
            "   Speedup: {:.2}x {}",
            speedup,
            if speedup > 1.0 { "🚀" } else { "" }
        );
    }

    println!("\n{}", "=".repeat(50));
}
