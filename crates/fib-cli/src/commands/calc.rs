//! Calculate command implementation

use fib_core::FibMethod;
use std::time::Instant;

pub fn run(n: u64, method: &str, show_time: bool, json: bool) {
    let method: FibMethod = match method.parse() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            eprintln!("Available methods: recursive, recursive_memo, iterative, matrix, fast_doubling, binet");
            std::process::exit(1);
        }
    };

    // Warn about slow recursive for large n
    if !json && matches!(method, FibMethod::Recursive) && n > 35 {
        eprintln!("⚠️  Warning: Recursive method is extremely slow for n > 35");
        eprintln!("    Consider using --method iterative or --method matrix");
    }

    let start = Instant::now();

    // Use BigInt if n is large (>186 overflows u128) or just to be safe
    let result_string = if n > 186 {
        if matches!(method, FibMethod::Binet) && !json {
            eprintln!("⚠️  Warning: Binet method loses precision for n > 78. Result will be approximate/truncated.");
        }
        method.calculate_bigint(n).to_string()
    } else {
        method.calculate(n).to_string()
    };

    let elapsed = start.elapsed();

    if json {
        // Simple JSON output
        // We output result as string to preserve precision for consumers
        println!(
            "{{\"n\": {}, \"method\": \"{}\", \"result\": \"{}\", \"time_ns\": {}}}",
            n,
            method.name(),
            result_string,
            elapsed.as_nanos()
        );
    } else {
        println!("┌─────────────────────────────────────────────────┐");
        println!("│ 🔢 Fibonacci Calculation                        │");
        println!("├─────────────────────────────────────────────────┤");
        println!("│ Method:     {:20}              │", method.name());
        println!("│ n:          {:20}              │", n);
        // Truncate output if too long
        if result_string.len() > 60 {
            println!(
                "│ F(n):       {}... ({} digits)",
                &result_string[..20],
                result_string.len()
            );
            println!(
                "│             ...{}",
                &result_string[result_string.len() - 20..]
            );
        } else {
            println!("│ F(n):       {:30}   │", result_string);
        }

        if show_time {
            println!("├─────────────────────────────────────────────────┤");
            println!("│ ⏱️  Time: {:?}", elapsed);
        }

        println!("└─────────────────────────────────────────────────┘");
    }
}
