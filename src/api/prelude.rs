//! Prelude module for CSP solver
//!
//! This module re-exports the most commonly used types and traits from the CSP solver library.
//! Users can import everything they need with a single `use csp::prelude::*;` statement.

#[doc(hidden)]
pub use crate::utils::*;
pub use crate::variables::*;
pub use crate::model::*;
pub use crate::core::*;
pub use crate::constraints::*;
pub use crate::search::*;

// Runtime constraint API
pub use crate::runtime_api::{ExprBuilder, Constraint, Builder, VarIdExt, ModelExt, ConstraintVecExt, and_all, or_all, all_of, any_of};

// Convenient constructor functions for common values
/// Create an integer value - shorthand for Val::ValI()
pub const fn int(value: i32) -> crate::variables::Val {
    crate::variables::Val::int(value)
}

/// Create a floating-point value - shorthand for Val::ValF()
pub const fn float(value: f64) -> crate::variables::Val {
    crate::variables::Val::float(value)
}