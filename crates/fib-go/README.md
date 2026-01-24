# fib-go

A crate that acts as a bridge between Rust and Go for performance comparison purposes. It bundles a Go implementation of Fibonacci algorithms and links it via FFI (Foreign Function Interface) to Rust.

## Prerequisites

*   **Go Compiler**: Go 1.20+ must be installed and available in your `PATH`.

## How it works

1.  `build.rs` detects the Go installation.
2.  It compiles the Go code in `go/fib.go` into a static archive (`libfib.a`).
3.  The Rust library links against this archive.
4.  Rust code calls the Go functions via `extern "C"`.

## Usage

This crate is primarily used by the `fib-cli` `compare-go` command.

```rust
// Internal usage in fib-cli
extern "C" {
    fn FibIterative(n: u64) -> u64;
}
```
