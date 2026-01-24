//! Sequence command - generate Fibonacci sequences

use fib_core::iterative::fib_iterative;

pub fn run(count: u64, start: u64) {
    println!(
        "📐 Fibonacci Sequence (F({}) to F({}))",
        start,
        start + count - 1
    );
    println!();

    let max_digits = format!("{}", fib_iterative(start + count - 1)).len();

    // Optimize by maintaining state instead of recalculating from scratch
    let mut current = fib_iterative(start);
    let mut next = if count > 1 {
        fib_iterative(start + 1)
    } else {
        0
    };
    let mut prev_for_ratio: Option<u128> = None;

    for i in 0..count {
        let n = start + i;
        let fib = current;

        // Show the golden ratio approximation for n > 0
        // We need F(n-1) which corresponds to prev_for_ratio if we are iterating
        // Or if this is the first iteration (i=0) and start > 0, we need to calculate it
        let ratio_str = if n > 0 {
            let prev_val = if i > 0 {
                // Use cached previous value
                prev_for_ratio.unwrap_or(0) // Should be set if i > 0
            } else {
                // First iteration, calculate F(start-1) if needed
                fib_iterative(n - 1)
            };

            if prev_val > 0 {
                format!("{:.10}", fib as f64 / prev_val as f64)
            } else {
                "∞".to_string()
            }
        } else {
            "-".to_string()
        };

        println!(
            "  F({:4}) = {:>width$}    φ ≈ {}",
            n,
            fib,
            ratio_str,
            width = max_digits
        );

        // Update state for next iteration
        if i < count - 1 {
            let new_next = current + next;
            prev_for_ratio = Some(current);
            current = next;
            next = new_next;
        }
    }

    println!();
    println!("φ (golden ratio) = 1.6180339887...");
}
