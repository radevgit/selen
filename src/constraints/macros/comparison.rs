//! Comparison constraint macros
//!
//! This module provides macro support for comparison constraints.
//! Currently re-exports from the main constraint_macros for compatibility.

// All comparison-related functionality is currently in constraint_macros.rs
// Re-export the main macros that handle comparison constraints
pub use crate::{post, postall};