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

// New unified constraint functions (Phase 1 of refactoring)
pub use crate::constraints::functions::{
    // Arithmetic operations
    add, sub, mul, div, modulo,
    // Comparison constraints
    eq, ne, lt, le, gt, ge,
    // Reified constraints
    eq_reif, ne_reif, lt_reif, le_reif, gt_reif, ge_reif,
    // Linear constraints (weighted sums)
    lin_eq, lin_le, lin_ne,
    lin_eq_reif, lin_le_reif, lin_ne_reif,
    LinearCoeff, // Trait for generic linear constraints
    // Basic constraints
    alldiff, alleq, min, max, sum, abs,
    // Logical constraints
    and, or, not, xor, implies,
    // Advanced constraints
    element, table, gcc, cumulative,
    // Type conversion
    to_float, floor, ceil, round,
};

// Convenient constructor functions for common values
/// Create an integer value - shorthand for Val::ValI()
pub const fn int(value: i32) -> crate::variables::Val {
    crate::variables::Val::int(value)
}

/// Create a floating-point value - shorthand for Val::ValF()
pub const fn float(value: f64) -> crate::variables::Val {
    crate::variables::Val::float(value)
}

/// Create a boolean value - shorthand for Val::ValI(0) or Val::ValI(1)
/// 
/// Booleans are represented as integers: false = 0, true = 1
/// 
/// # Examples
/// 
/// ```
/// use selen::prelude::*;
/// 
/// let mut m = Model::default();
/// let b = m.bool();
/// 
/// // Use bool() to create boolean constants
/// m.new(b.eq(bool(true)));  // b == 1
/// m.new(b.ne(bool(false))); // b != 0
/// ```
pub const fn bool(value: bool) -> crate::variables::Val {
    crate::variables::Val::bool(value)
}