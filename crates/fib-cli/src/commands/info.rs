//! Info command - display algorithm information

use fib_core::FibMethod;

pub fn run(method: &str) {
    println!("╔═══════════════════════════════════════════════════════════════════╗");
    println!("║              📚 Fibonacci Algorithm Information                   ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝");
    println!();

    if method == "all" {
        let methods = [
            FibMethod::Recursive,
            FibMethod::RecursiveMemo,
            FibMethod::Iterative,
            FibMethod::IterativeBranchless,
            FibMethod::Matrix,
            FibMethod::FastDoubling,
            FibMethod::Binet,
        ];

        println!(
            "┌───────────────────────┬────────────┬────────────┬───────────────────────────────┐"
        );
        println!(
            "│ Algorithm             │ Time       │ Space      │ Notes                         │"
        );
        println!(
            "├───────────────────────┼────────────┼────────────┼───────────────────────────────┤"
        );

        for m in methods {
            let notes = match m {
                FibMethod::Recursive => "Demonstration only",
                FibMethod::RecursiveMemo => "Good for small n",
                FibMethod::Iterative => "General purpose",
                FibMethod::IterativeBranchless => "CPU pipeline optimized",
                FibMethod::Matrix => "Best for large n",
                FibMethod::FastDoubling => "Alternative O(log n)",
                FibMethod::Binet => "n ≤ 78 only",
            };

            println!(
                "│ {:21} │ {:10} │ {:10} │ {:29} │",
                m.name(),
                m.time_complexity(),
                m.space_complexity(),
                notes
            );
        }

        println!(
            "└───────────────────────┴────────────┴────────────┴───────────────────────────────┘"
        );
    } else {
        match method.parse::<FibMethod>() {
            Ok(m) => {
                println!("Algorithm: {}", m.name());
                println!("Time Complexity: {}", m.time_complexity());
                println!("Space Complexity: {}", m.space_complexity());

                println!();
                println!("Description:");
                match m {
                    FibMethod::Recursive => {
                        println!("  The naive recursive implementation directly follows the");
                        println!("  mathematical definition F(n) = F(n-1) + F(n-2).");
                        println!("  It has exponential time complexity O(2^n) because it");
                        println!("  recomputes the same values many times.");
                        println!();
                        println!("  ⚠️  Only suitable for demonstration with n < 30.");
                    }
                    FibMethod::RecursiveMemo => {
                        println!("  Uses memoization to cache computed values, avoiding");
                        println!("  redundant calculations. Achieves O(n) time complexity");
                        println!("  but requires O(n) space for the cache.");
                    }
                    FibMethod::Iterative => {
                        println!("  The standard iterative approach using a simple loop.");
                        println!("  Maintains only two values (a, b) and updates them");
                        println!("  in each iteration. O(n) time with O(1) space.");
                    }
                    FibMethod::IterativeBranchless => {
                        println!("  A variant of iterative that avoids conditional branches");
                        println!("  in the main loop. Can improve performance on modern CPUs");
                        println!("  due to better branch prediction and pipeline utilization.");
                    }
                    FibMethod::Matrix => {
                        println!("  Uses the matrix identity [[1,1],[1,0]]^n to compute");
                        println!("  F(n) in O(log n) time using fast exponentiation.");
                        println!("  This is the fastest method for very large n.");
                    }
                    FibMethod::FastDoubling => {
                        println!("  Uses the fast doubling identities to compute F(n):");
                        println!("  F(2k) = F(k) * (2*F(k+1) - F(k))");
                        println!("  F(2k+1) = F(k)² + F(k+1)²");
                        println!();
                        println!("  Computes F(n) in O(log n) time using recursive doubling.");
                        println!(
                            "  Alternative to matrix exponentiation with similar performance."
                        );
                    }
                    FibMethod::Binet => {
                        println!("  Uses Binet's closed-form formula:");
                        println!("  F(n) = (φ^n - ψ^n) / √5");
                        println!("  where φ = (1+√5)/2 (golden ratio)");
                        println!();
                        println!("  ⚠️  Limited to n ≤ 78 due to floating-point precision.");
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Error: {}", e);
                eprintln!("Available methods: recursive, recursive_memo, iterative, matrix, fast_doubling, binet");
            }
        }
    }
}
