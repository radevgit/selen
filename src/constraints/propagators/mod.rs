//! Constraint propagators module
//!
//! This module contains constraint propagators organized by category.

// Re-export everything from the original props module for backward compatibility
pub use crate::props::*;

// Organized propagator categories (currently organizational only)
pub mod arithmetic;
pub mod comparison;
pub mod logical;
pub mod global;
pub mod mathematical;