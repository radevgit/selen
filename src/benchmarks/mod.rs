//! Benchmarking and performance testing
//!
//! This module contains various benchmarks for testing solver performance,
//! precision validation, and constraint handling.

pub mod benchmark_suite;
pub mod manufacturing_constraints;

// Re-export main benchmark functionality
pub use benchmark_suite::*;
// pub use comprehensive_benchmark::*; // Commented out to fix unused warning