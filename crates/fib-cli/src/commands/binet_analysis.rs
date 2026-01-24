//! Binet formula accuracy analysis

use fib_core::{closed_form, iterative};

pub fn run(max_n: u64) {
    println!("--- Binet Formula Accuracy Analysis ---");
    println!();
    println!("Analyzing accuracy of Binet formula F(n) = (phi^n - psi^n) / sqrt(5)");
    println!();

    println!("+--------+---------------------------+---------------------------+---------------+---------------+");
    println!("| n      | Exact F(n)                | Binet F(n)                | Abs Error     | Rel Error     |");
    println!("+--------+---------------------------+---------------------------+---------------+---------------+");

    let mut first_error_n: Option<u64> = None;

    for n in (0..=max_n).step_by(10) {
        let exact = iterative::fib_iterative(n);
        let binet = closed_form::fib_binet_f64(n);
        let binet_rounded = closed_form::fib_binet_rounded(n);

        let (abs_error, rel_error) = closed_form::binet_error_analysis(n);

        let (error_marker, _is_error) = if binet_rounded != exact {
            first_error_n.get_or_insert(n);
            ("[X]", true)
        } else {
            ("[V]", false)
        };

        println!(
            "| {:6} | {:25} | {:25.2} | {:13.2e} | {:9.2e} {:3} |",
            n, exact, binet, abs_error, rel_error, error_marker
        );
    }

    println!("+--------+---------------------------+---------------------------+---------------+---------------+");
    println!();

    // Find exact limit
    let accuracy_limit = closed_form::find_binet_accuracy_limit();
    println!("SUMMARY:");
    println!("   - Binet formula is exact for n <= {}", accuracy_limit);
    println!(
        "   - Maximum recommended n: {}",
        closed_form::MAX_ACCURATE_N
    );

    if let Some(first_err) = first_error_n {
        println!("   - First error observed at n = {}", first_err);
    }

    println!();
    println!("NOTE: IEEE 754 double precision has ~15-17 significant decimal digits.");
    println!("   For larger n, use iterative or matrix methods.");
}
