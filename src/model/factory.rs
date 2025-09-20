//! Variable factory methods
//!
//! This module contains methods for creating different types of variables.
//! Currently all implementations are in model_core.rs and will be moved here in a future phase.

use crate::vars::{VarId, Val, VarIdBin};
use crate::model_core::Model;

impl Model {
    // Note: Variable factory methods are currently implemented in model_core.rs
    // They include:
    // - new_var(min, max) -> VarId
    // - new_vars(n, min, max) -> Iterator<VarId>
    // - int_vars(n, min, max) -> Iterator<VarId>
    // - ints(values) -> VarId
    // - float_vars(n, min, max) -> Iterator<VarId>
    // - bool() -> VarId
    // - new_var_binary() -> VarIdBin
    // - new_vars_binary(n) -> Iterator<VarIdBin>
    // - int(min, max) -> VarId
    // - float(min, max) -> VarId
    // - binary() -> VarIdBin
    // - new_var_unchecked(min, max) -> VarId
    //
    // These will be moved to this module in a future phase of the modularization.
}