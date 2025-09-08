//! Prelude module for CSP solver
//!
//! This module re-exports the most commonly used types and traits from the CSP solver library.
//! Users can import everything they need with a single `use csp::prelude::*;` statement.

// Re-export modules
// pub use crate::constraints;
// pub use crate::domain;
// pub use crate::propagation;
// pub use crate::search;
// pub use crate::solver;
// pub use crate::variable;


// Re-export commonly used types
// pub use crate::constraints::*;
// pub use crate::domain::*;
// pub use crate::propagation::*;
// pub use crate::search::*;
// pub use crate::solver::*;
// pub use crate::variable::*;

#[doc(hidden)]
pub use crate::utils::*;
pub use crate::vars::*;
pub use crate::views::*;
pub use crate::model::*;
pub use crate::solution::*;
pub use crate::props::*;
pub use crate::search::*;

// Convenient constructor functions for common values
/// Create an integer value - shorthand for Val::ValI()
pub const fn int(value: i32) -> crate::vars::Val {
    crate::vars::Val::int(value)
}

/// Create a floating-point value - shorthand for Val::ValF()
pub const fn float(value: f32) -> crate::vars::Val {
    crate::vars::Val::float(value)
}

