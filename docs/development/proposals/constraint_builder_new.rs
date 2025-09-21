//! Constraint builder system for clean API design.
//!
//! This module provides a constraint builder pattern that enables clean,
//! intuitive constraint syntax while maintaining proper separation between
//! constraint creation and model management.

use crate::vars::{VarId, Val};
use crate::model::Model;
use crate::boolean_operators::{BoolExpr, BooleanModel};

/// Represents a constraint that can be added to a m.
#[derive(Debug, Clone)]
pub enum Constraint {
    /// Equality constraint: x == y
    Eq(VarId, VarId),
    /// Inequality constraint: x != y  
    Ne(VarId, VarId),
    /// Less than constraint: x < y
    Lt(VarId, VarId),
    /// Less than or equal constraint: x <= y
    Le(VarId, VarId),
    /// Greater than constraint: x > y
    Gt(VarId, VarId),
    /// Greater than or equal constraint: x >= y
    Ge(VarId, VarId),
    /// Equality with constant: x == value
    EqVal(VarId, Val),
    /// Less than or equal with constant: x <= value
    LeVal(VarId, Val),
    /// Greater than or equal with constant: x >= value
    GeVal(VarId, Val),
    /// Greater than with constant: x > value
    GtVal(VarId, Val),
    /// Less than with constant: x < value
    LtVal(VarId, Val),
    /// Boolean expression must be true
    BoolTrue(BoolExpr),
}

impl Constraint {
    /// Apply this constraint to a model.
    pub fn apply_to(self, model: &mut Model) {
        match self {
            Constraint::Eq(x, y) => model.eq(x, y),
            Constraint::Ne(x, y) => model.ne(x, y),
            Constraint::Lt(x, y) => model.lt(x, y),
            Constraint::Le(x, y) => model.le(x, y),
            Constraint::Gt(x, y) => model.gt(x, y),
            Constraint::Ge(x, y) => model.ge(x, y),
            Constraint::EqVal(x, val) => model.eq(x, val),
            Constraint::LeVal(x, val) => model.le(x, val),
            Constraint::GeVal(x, val) => model.ge(x, val),
            Constraint::GtVal(x, val) => model.gt(x, val),
            Constraint::LtVal(x, val) => model.lt(x, val),
            Constraint::BoolTrue(expr) => {
                // Use post_true instead of bool_expr to avoid creating intermediate variables
                model.post_true(expr);
            }
        }
    }
}

/// Different types of inputs that can be converted to constraints for post()
#[derive(Debug, Clone)]
pub enum ConstraintInput {
    /// A direct constraint
    Direct(Constraint),
    /// A boolean expression that should be true
    Boolean(BoolExpr),
}

impl From<Constraint> for ConstraintInput {
    fn from(constraint: Constraint) -> Self {
        ConstraintInput::Direct(constraint)
    }
}

impl From<BoolExpr> for ConstraintInput {
    fn from(expr: BoolExpr) -> Self {
        ConstraintInput::Boolean(expr)
    }
}

#[doc(hidden)]
/// Extension trait for Model to provide unified constraint posting
pub trait ModelConstraints {
    /// Post a constraint to the model - unified method that accepts multiple input types
    fn post<T: Into<ConstraintInput>>(&mut self, input: T);
}

impl ModelConstraints for Model {
    fn post<T: Into<ConstraintInput>>(&mut self, input: T) {
        let constraint_input = input.into();
        match constraint_input {
            ConstraintInput::Direct(constraint) => {
                constraint.apply_to(self);
            }
            ConstraintInput::Boolean(expr) => {
                let constraint = Constraint::BoolTrue(expr);
                constraint.apply_to(self);
            }
        }
    }
}
