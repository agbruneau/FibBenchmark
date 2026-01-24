use fib_core::FibMethod;
use num_bigint::BigUint;

#[test]
#[should_panic(expected = "overflow")]
#[cfg(debug_assertions)]
fn test_fib_iterative_overflow() {
    // F(186) fits in u128, F(187) overflows
    // fib_iterative likely panics on add overflow if debug assertions are on,
    // or we might need to check how it's implemented.
    // If it uses wrapping_add, it won't panic.
    // Let's assume standard add behavior in Rust (panics in debug, wraps in release)
    // unless explicit wrapping/saturating methods are used.
    // Checking iterative.rs: `let temp = a + b;` -> this panics in debug.
    fib_core::fib_iterative(187);
}

#[test]
#[cfg(not(debug_assertions))]
fn test_fib_iterative_overflow_release() {
    // In release mode, overflow wraps, so it shouldn't panic
    let result = std::panic::catch_unwind(|| {
        fib_core::fib_iterative(187);
    });
    assert!(result.is_ok(), "Should not panic in release mode due to overflow wrapping");
}

#[test]
fn test_fib_calculate_bigint_transition() {
    let n_safe = 186;
    let n_overflow = 187;

    // Verify u128 calculation works for max
    let val_186 = fib_core::fib_iterative(n_safe);

    // Verify BigUint matches u128 for max
    let method = FibMethod::Iterative;
    let val_186_big = method.calculate_bigint(n_safe);
    assert_eq!(BigUint::from(val_186), val_186_big);

    // Verify BigUint works for overflow case
    let val_187_big = method.calculate_bigint(n_overflow);
    assert!(val_187_big > val_186_big);

    // Check property F(187) = F(186) + F(185)
    let val_185_big = method.calculate_bigint(185);
    assert_eq!(val_187_big, val_186_big + val_185_big);
}

#[test]
fn test_bigint_consistency() {
    let n = 200;
    let method_iter = FibMethod::Iterative;
    let method_matrix = FibMethod::Matrix;

    let res_iter = method_iter.calculate_bigint(n);
    let res_matrix = method_matrix.calculate_bigint(n);

    assert_eq!(res_iter, res_matrix);
}
