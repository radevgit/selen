//! Prelude module for CSP solver
//!
//! This module re-exports from the new modular API structure for backward compatibility.

// Re-export everything from the new API prelude
pub use crate::api::prelude::*;

// Re-export FlatZinc types and utilities
pub use crate::flatzinc::{FlatZincError, FlatZincResult, format_solution};

