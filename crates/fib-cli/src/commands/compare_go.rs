//! Compare Go command - compares Rust vs Go Fibonacci implementations

use fib_go::{compare_implementations, format_comparison_table, get_go_version, is_go_available};

/// Run the compare-go command
pub fn run(n: u64, iterations: u32) {
    println!("🔬 Rust vs Go Fibonacci Comparison");
    println!("===================================");
    println!();

    // Check Go availability
    let go_version = get_go_version();
    println!("📦 Go Version: {}", go_version);

    if !is_go_available() {
        println!();
        println!("⚠️  Note: Running with Rust stub (CGO not available)");
        println!("   To use native Go implementation:");
        println!("   1. Install MinGW-w64 (GCC for Windows)");
        println!("   2. Add GCC to PATH");
        println!("   3. Rebuild with: cargo build -p fib-go");
        println!();
    }

    println!("📊 Parameters: n={}, iterations={}", n, iterations);
    println!();

    // Run comparison
    let results = compare_implementations(n, iterations);

    // Display results
    let table = format_comparison_table(&results);
    println!("{}", table);

    // Additional analysis
    println!("\n📋 Analysis Summary:");
    println!("─────────────────────");

    let rust_matrix = results
        .iter()
        .find(|r| r.language == "Rust" && r.method == "Matrix");
    let go_matrix = results
        .iter()
        .find(|r| r.language == "Go" && r.method == "Matrix");

    if let (Some(rust), Some(go)) = (rust_matrix, go_matrix) {
        let rust_ns = rust.avg_time.as_nanos() as f64;
        let go_ns = go.avg_time.as_nanos() as f64;

        if rust_ns > 0.0 && go_ns > 0.0 {
            println!("\n🏆 Matrix Method Comparison:");
            println!("   Rust: {:?}", rust.avg_time);
            println!("   Go:   {:?}", go.avg_time);

            if rust_ns < go_ns {
                let speedup = go_ns / rust_ns;
                println!("   → Rust is {:.2}x faster", speedup);
            } else {
                let speedup = rust_ns / go_ns;
                println!("   → Go is {:.2}x faster", speedup);
            }
        }
    }

    // Verify results match
    let all_match = results.windows(2).all(|w| w[0].result == w[1].result);

    println!();
    if all_match {
        println!(
            "✅ All implementations produce the same result: F({}) = {}",
            n, results[0].result
        );
    } else {
        println!("⚠️  Warning: Results differ between implementations!");
        for r in &results {
            println!("   {} {}: {}", r.language, r.method, r.result);
        }
    }

    println!();
    println!("💡 Tip: For accurate benchmarks, use larger n values (1000+) and more iterations.");
}
