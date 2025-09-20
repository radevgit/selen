//! Logical constraint macros
//!
//! This module provides macro support for logical constraints.
//! Currently re-exports from the main constraint_macros for compatibility.

// All logical-related functionality is currently in constraint_macros.rs
// Re-export the main macros that handle logical constraints
pub use crate::{post, postall};