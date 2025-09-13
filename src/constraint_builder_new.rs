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
    /// Apply this constraint to a m.
    pub fn apply_to(self, model: &mut Model) {
        match self {
            Constraint::Eq(x, y) => m.eq(x, y),
            Constraint::Ne(x, y) => m.ne(x, y),
            Constraint::Lt(x, y) => m.lt(x, y),
            Constraint::Le(x, y) => m.le(x, y),
            Constraint::Gt(x, y) => m.gt(x, y),
            Constraint::Ge(x, y) => m.ge(x, y),
            Constraint::EqVal(x, val) => m.eq(x, val),
            Constraint::LeVal(x, val) => m.le(x, val),
            Constraint::GeVal(x, val) => m.ge(x, val),
            Constraint::GtVal(x, val) => m.gt(x, val),
            Constraint::LtVal(x, val) => m.lt(x, val),
            Constraint::BoolTrue(expr) => {
                let result_var = m.bool_expr(expr);
                m.eq(result_var, Val::ValI(1));
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
