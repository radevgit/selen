//! Arithmetic constraint macros
//!
//! This module provides macro support for arithmetic constraints.
//! Currently re-exports from the main constraint_macros for compatibility.

// All arithmetic-related functionality is currently in constraint_macros.rs
// Re-export the main macros that handle arithmetic constraints
pub use crate::{post, postall};