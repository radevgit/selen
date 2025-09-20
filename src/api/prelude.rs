//! Prelude module for CSP solver
//!
//! This module re-exports the most commonly used types and traits from the CSP solver library.
//! Users can import everything they need with a single `use csp::prelude::*;` statement.

#[doc(hidden)]
pub use crate::utils::*;
pub use crate::vars::*;
pub use crate::views::*;
pub use crate::model_core::*;
pub use crate::solution::*;
pub use crate::config::*;
pub use crate::error::*;
pub use crate::props::*;
pub use crate::search::*;

// Runtime constraint API
pub use crate::runtime_api::{ExprBuilder, Constraint, Builder, VarIdExt, ModelExt, ConstraintVecExt, and_all, or_all, all_of, any_of};

// Mathematical constraint macros
pub use crate::{post, postall};

// Convenient constructor functions for common values
/// Create an integer value - shorthand for Val::ValI()
pub const fn int(value: i32) -> crate::vars::Val {
    crate::vars::Val::int(value)
}

/// Create a floating-point value - shorthand for Val::ValF()
pub const fn float(value: f64) -> crate::vars::Val {
    crate::vars::Val::float(value)
}