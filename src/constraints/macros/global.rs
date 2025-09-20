//! Global constraint macros
//!
//! This module provides macro support for global constraints.
//! Currently re-exports from the main constraint_macros for compatibility.

// All global constraint functionality is currently in constraint_macros.rs
// Re-export the main macros that handle global constraints
pub use crate::{post, postall};