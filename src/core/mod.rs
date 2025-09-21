//! Core functionality
//!
//! This module contains fundamental types used throughout the CSP solver:
//! - Error types and result handling
//! - Solution representation
//! - Validation utilities

pub mod error;
pub mod solution;
pub mod validation;

// Re-export everything from submodules
pub use error::*;
pub use solution::*;
pub use validation::*;