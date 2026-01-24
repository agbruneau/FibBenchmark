//! Fibonacci Visualization Tool
//!
//! Generates charts and visualizations for benchmark results

use fib_core::{closed_form, iterative, matrix};
use fib_viz::data_parser::{AccuracyPoint, ComplexityPoint, GoldenRatioPoint};
use fib_viz::generate_report;
use std::fs;

fn main() {
    println!("📊 Fibonacci Visualization Generator");
    println!("=====================================");
    println!();

    // Create results directory
    fs::create_dir_all("results").ok();

    if let Err(e) = generate_data() {
        eprintln!("Error generating data: {}", e);
        std::process::exit(1);
    }

    println!("✅ Data files generated in results/");
    println!();

    // Generate Charts
    if let Err(e) = generate_report("results", "results") {
        eprintln!("Error generating charts: {}", e);
        std::process::exit(1);
    }
}

fn generate_data() -> std::io::Result<()> {
    generate_complexity_data()?;
    generate_accuracy_data()?;
    generate_golden_ratio_data()?;
    Ok(())
}

fn generate_complexity_data() -> std::io::Result<()> {
    println!("📈 Generating complexity comparison data...");

    let mut data = Vec::new();

    for n in (10..=180).step_by(10) {
        let iterations = 100;

        // Time iterative
        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let _ = iterative::fib_iterative(n);
        }
        let iter_ns = start.elapsed().as_nanos() / iterations as u128;

        // Time matrix
        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let _ = matrix::fib_matrix_fast(n);
        }
        let matrix_ns = start.elapsed().as_nanos() / iterations as u128;

        data.push(ComplexityPoint {
            n,
            iterative_ns: iter_ns,
            matrix_ns,
        });
    }

    let json = serde_json::to_string_pretty(&data).unwrap();
    fs::write("results/complexity_comparison.json", json)?;
    println!("   ✓ results/complexity_comparison.json");
    Ok(())
}

fn generate_accuracy_data() -> std::io::Result<()> {
    println!("📈 Generating Binet accuracy data...");

    let mut data = Vec::new();

    for n in 0..=100 {
        let exact = iterative::fib_iterative(n);
        let binet = closed_form::fib_binet_f64(n);
        let (abs_error, rel_error) = closed_form::binet_error_analysis(n);

        data.push(AccuracyPoint {
            n,
            exact,
            binet,
            abs_error,
            rel_error,
        });
    }

    let json = serde_json::to_string_pretty(&data).unwrap();
    fs::write("results/binet_accuracy.json", json)?;
    println!("   ✓ results/binet_accuracy.json");
    Ok(())
}

fn generate_golden_ratio_data() -> std::io::Result<()> {
    println!("📈 Generating golden ratio convergence data...");

    let mut data = Vec::new();
    let phi = closed_form::PHI;

    for n in 1..=50 {
        let ratio = closed_form::fibonacci_ratio(n);
        let error = (ratio - phi).abs();

        data.push(GoldenRatioPoint {
            n,
            ratio,
            error_from_phi: error,
        });
    }

    let json = serde_json::to_string_pretty(&data).unwrap();
    fs::write("results/golden_ratio_convergence.json", json)?;
    println!("   ✓ results/golden_ratio_convergence.json");
    Ok(())
}
