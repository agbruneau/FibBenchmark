//! Build script for fib-go
//!
//! This script compiles the Go code into a static library and links it with Rust.
//! If CGO is not available (e.g., no GCC on Windows), it uses a Rust-based stub.

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Declare the custom cfg for check-cfg
    println!("cargo::rustc-check-cfg=cfg(use_rust_stub)");
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let go_dir = PathBuf::from(&manifest_dir).join("go");
    let out_dir = env::var("OUT_DIR").unwrap();

    let lib_name = "libfibgo.a";
    let lib_path = PathBuf::from(&out_dir).join(lib_name);

    println!("cargo:rerun-if-changed=go/fib.go");
    println!("cargo:rerun-if-changed=build.rs");

    // Check if Go is available
    let go_version = Command::new("go").arg("version").output();

    let go_available = match go_version {
        Ok(output) if output.status.success() => {
            println!(
                "cargo:warning=Go found: {}",
                String::from_utf8_lossy(&output.stdout).trim()
            );
            true
        }
        _ => {
            println!("cargo:warning=Go not found");
            false
        }
    };

    // Check if GCC is available (required for CGO on Windows)
    let gcc_available = Command::new("gcc")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if !gcc_available {
        println!("cargo:warning=GCC not found (required for CGO), using Rust stub");
        println!("cargo:rustc-cfg=use_rust_stub");
        return;
    }

    if !go_available {
        println!("cargo:rustc-cfg=use_rust_stub");
        return;
    }

    // Build the Go library using CGO
    let status = Command::new("go")
        .current_dir(&go_dir)
        .env("CGO_ENABLED", "1")
        .args([
            "build",
            "-buildmode=c-archive",
            "-o",
            lib_path.to_str().unwrap(),
            "fib.go",
        ])
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("cargo:warning=Successfully compiled Go library");
        }
        Ok(s) => {
            println!(
                "cargo:warning=Go build failed with status: {}, using Rust stub",
                s
            );
            println!("cargo:rustc-cfg=use_rust_stub");
            return;
        }
        Err(e) => {
            println!("cargo:warning=Go build error: {}, using Rust stub", e);
            println!("cargo:rustc-cfg=use_rust_stub");
            return;
        }
    }

    // Link the Go library
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=fibgo");

    // Platform-specific linking
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=dylib=ws2_32");
        println!("cargo:rustc-link-lib=dylib=userenv");
        println!("cargo:rustc-link-lib=dylib=ntdll");
        println!("cargo:rustc-link-lib=dylib=bcrypt");
    } else if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dylib=pthread");
        println!("cargo:rustc-link-lib=dylib=dl");
    } else if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=Security");
    }
}
