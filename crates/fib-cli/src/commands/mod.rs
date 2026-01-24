//! CLI command implementations

pub mod bench;
pub mod binet_analysis;
pub mod calc;
pub mod compare;
pub mod compare_go;
pub mod info;
pub mod memory;
pub mod report;
pub mod sequence;

#[cfg(feature = "simd")]
pub mod simd;
