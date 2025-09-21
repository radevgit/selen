//! Benchmarking and performance testing
//!
//! This module contains various benchmarks for testing solver performance,
//! precision validation, and constraint handling.

pub mod benchmark_suite;
pub mod comprehensive_benchmark; 
pub mod manufacturing_constraints;
pub mod medium_scale_proposals;
pub mod precision_validation;
pub mod quick_benchmark;
pub mod solver_limits;
pub mod step_2_4_performance_benchmarks;
pub mod step_2_4_performance_benchmarks_simple;

// Re-export main benchmark functionality
pub use benchmark_suite::*;
// pub use comprehensive_benchmark::*; // Commented out to fix unused warning