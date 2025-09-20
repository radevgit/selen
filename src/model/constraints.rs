//! Constraint posting methods
//!
//! This module contains methods for posting constraints to the model.
//! Currently all implementations are in model_core.rs and will be moved here in a future phase.

use crate::model_core::Model;
use crate::vars::VarId;

impl Model {
    // Note: Constraint posting methods are currently implemented in model_core.rs
    // They include:
    // - Mathematical operations: add(), sub(), mul(), div(), modulo(), abs()
    // - Comparison operations: equals(), not_equals(), less_than(), etc.
    // - Global constraints: alldiff(), allequal(), element()
    // - Boolean operations: bool_and(), bool_or(), bool_not()
    // - Arithmetic expressions with domain inference
    //
    // These will be moved to this module in a future phase of the modularization.
}