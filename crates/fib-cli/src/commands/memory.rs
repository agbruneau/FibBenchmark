//! Memory analysis command

use crate::ALLOCATOR;
use fib_core::FibMethod;
use std::str::FromStr;

pub fn run(n: u64, method_str: &str) {
    let method = match FibMethod::from_str(method_str) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    println!("🧠 Memory Analysis: {} (n={})", method.name(), n);
    println!("==========================================");

    // Reset allocator stats
    ALLOCATOR.reset();
    let initial_allocs = ALLOCATOR.get_allocation_count();
    let initial_bytes = ALLOCATOR.get_current_usage();

    // Run calculation
    let result = method.calculate(n);

    let final_allocs = ALLOCATOR.get_allocation_count();
    let final_bytes = ALLOCATOR.get_current_usage(); // This might be back to 0 if everything was dropped

    // For recursive memo, the vector represents peak usage basically.
    // Since we don't have peak tracking yet, we rely on "total allocations made".

    let allocs_made = final_allocs - initial_allocs;
    let net_bytes = final_bytes.saturating_sub(initial_bytes);

    println!("Result: {}", result);
    println!();
    println!("📊 Statistics:");
    println!("  Total Allocations: {}", allocs_made);
    println!("  Net Bytes Leaked/Held: {}", net_bytes);

    println!();
    println!("📋 Theoretical Complexity:");
    println!("  Space: {}", method.space_complexity());
    if method == FibMethod::RecursiveMemo {
        let theoretical = (n + 1) * 16;
        println!("  Expected Peak (Memo): ~{} bytes", theoretical);
    }
}
