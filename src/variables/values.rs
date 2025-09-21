//! Value types and value operations
//!
//! This module contains the Val enum and all operations that can be performed
//! on values including type conversions, arithmetic, and comparisons.
//!
//! Currently all implementations are in vars.rs. This module provides
//! organizational structure for future extraction.

// Re-export value types and operations from vars.rs
pub use crate::variables::vars::Val;

// Note: Val type and operations are currently implemented in vars.rs:
//
// Val enum definition (lines 52-58):
// - ValI(i32) - Integer values
// - ValF(f64) - Floating-point values  
//
// Val trait implementations (lines 59-377):
// - Core methods: int(), float(), is_int(), is_float() (lines 59-142)
// - Type conversions: From<i32>, From<f64> (lines 143-153)
// - Additional methods: safe_mod(), abs(), etc. (lines 155-220)
// - Comparison traits: PartialEq, Eq, PartialOrd, Ord (lines 221-250)
// - Arithmetic traits: Add, Sub, Mul, Div, Rem (lines 251-377)
// - Utility traits: Sum (lines 264-269)
//
// This comprehensive value system could be extracted to this module
// for better organization in a future refactoring phase.