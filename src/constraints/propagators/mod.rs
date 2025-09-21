//! Constraint propagators module
//!
//! This module contains constraint propagators organized by category.

// Re-export everything from the props submodule for backward compatibility
pub use crate::constraints::props::*;

// Organized propagator categories (currently organizational only)
pub mod arithmetic;
pub mod comparison;
pub mod logical;
pub mod global;
pub mod mathematical;