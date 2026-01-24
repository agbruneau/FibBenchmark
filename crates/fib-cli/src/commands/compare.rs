//! Compare command - compare all algorithms

use fib_core::{closed_form, iterative, matrix, recursive};
use std::time::Instant;

pub fn run(n: u64, max_recursive: u64) {
    let format_scientific = |val: u128| -> String {
        let f = val as f64;
        format!("{:.4e}", f)
    };

    println!("+------------------------------------------------------------------------------+");
    println!(
        "|            Fibonacci Algorithm Comparison for n = {:<12}           |",
        n
    );
    println!("+---------------------+-------------------------------------------+------------+");
    println!("| Algorithm           | {:>41} | Time       |", "Result");
    println!("+---------------------+-------------------------------------------+------------+");

    // Recursive (only for small n)
    if n <= max_recursive {
        let start = Instant::now();
        let result = recursive::fib_recursive(n);
        let elapsed = start.elapsed();
        println!(
            "| {:19} | {:>41} | {:10?} |",
            "Recursive",
            format_scientific(result),
            elapsed
        );
    } else {
        println!(
            "| {:19} | {:41} | {:10} |",
            "Recursive",
            format!("(skipped - n > {})", max_recursive),
            "N/A"
        );
    }

    // Recursive with memo (skip for large n to avoid stack overflow)
    // The default stack size is usually 2MB, which allows for ~20k-30k stack frames.
    let max_recursive_memo = 20_000;
    if n <= max_recursive_memo {
        let start = Instant::now();
        let result = recursive::fib_recursive_memo(n);
        let elapsed = start.elapsed();
        println!(
            "| {:19} | {:>41} | {:10?} |",
            "Recursive+Memo",
            format_scientific(result),
            elapsed
        );
    } else {
        println!(
            "| {:19} | {:41} | {:10} |",
            "Recursive+Memo",
            format!("(skipped - n > {})", max_recursive_memo),
            "N/A"
        );
    }

    // Iterative
    let start = Instant::now();
    let result = iterative::fib_iterative(n);
    let elapsed = start.elapsed();
    println!(
        "| {:19} | {:>41} | {:10?} |",
        "Iterative",
        format_scientific(result),
        elapsed
    );

    // Iterative branchless
    let start = Instant::now();
    let result = iterative::fib_iterative_branchless(n);
    let elapsed = start.elapsed();
    println!(
        "| {:19} | {:>41} | {:10?} |",
        "Iter. Branchless",
        format_scientific(result),
        elapsed
    );

    // Matrix
    let start = Instant::now();
    let result = matrix::fib_matrix_fast(n);
    let elapsed = start.elapsed();
    println!(
        "| {:19} | {:>41} | {:10?} |",
        "Matrix",
        format_scientific(result),
        elapsed
    );

    // Matrix doubling
    let start = Instant::now();
    let result = matrix::fib_doubling(n);
    let elapsed = start.elapsed();
    println!(
        "| {:19} | {:>41} | {:10?} |",
        "Matrix Doubling",
        format_scientific(result),
        elapsed
    );

    // Binet (with accuracy warning)
    let start = Instant::now();
    let binet_result = closed_form::fib_binet_f64(n);
    let elapsed = start.elapsed();
    if n <= closed_form::MAX_ACCURATE_N {
        println!(
            "| {:19} | {:>41} | {:10?} |",
            "Binet (f64)",
            format!("{:.4e}", binet_result),
            elapsed
        );
    } else {
        let (mantissa, exponent) = closed_form::fib_binet_scientific(n);
        let output = format!("{:.4}e{}", mantissa, exponent);
        println!(
            "| {:19} | {:>41} | {:10?} |",
            "Binet (f64) [!]", output, elapsed
        );
    }

    println!("+---------------------+-------------------------------------------+------------+");

    if n > closed_form::MAX_ACCURATE_N {
        println!(
            "\n(!) Note: Binet formula loses precision for n > {}",
            closed_form::MAX_ACCURATE_N
        );
    }
}
