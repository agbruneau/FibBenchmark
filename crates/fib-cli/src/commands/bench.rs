//! Benchmark command implementation

pub fn run(filter: Option<String>) {
    println!("📊 Running Criterion Benchmarks...");
    println!();

    if let Some(ref f) = filter {
        println!("Filter: {}", f);
    }

    println!("To run full benchmarks, use:");
    println!();
    println!("  cargo bench");
    println!();

    if let Some(f) = &filter {
        println!("  cargo bench -- {}", f);
    }

    println!();
    println!("Benchmark results will be saved to: target/criterion/");
    println!("Open target/criterion/report/index.html for the full report.");
}
