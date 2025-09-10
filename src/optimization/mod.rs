//! Efficient optimization algorithms for constraint satisfaction problems
//!
//! This module implements problem classification and specialized solving algorithms
//! based on constraint patterns and variable types. The key insight is that different
//! problem types require fundamentally different algorithmic approaches:
//!
//! - **Pure float problems**: Use direct bounds optimization (O(1) solutions)
//! - **Pure integer problems**: Use existing binary search (works well for discrete domains)
//! - **Mixed problems**: Use hybrid approaches (MINLP techniques)

pub mod classification;

pub use classification::*;
