//! Constraint builder system for clean API design.
//!
//! This module provides a constraint builder pattern that enables clean,
//! intuitive constraint syntax while maintaining proper separation between
//! constraint creation and model management.

use crate::vars::{VarId, Val};
use crate::model::Model;
use crate::boolean_operators::{BoolExpr, BooleanModel};

/// Represents a constraint that can be added to a model.
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
                let result_var = model.bool_expr(expr);
                model.eq(result_var, Val::ValI(1));
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

/// Extension trait for VarId to provide clean constraint creation API
pub trait VarConstraints {
    /// Create x == value constraint
    fn eq_int(self, value: i32) -> Constraint;
    /// Create x == value constraint
    fn eq_float(self, value: f64) -> Constraint;
    /// Create x == y constraint
    fn eq_var(self, other: VarId) -> Constraint;
    /// Create x != y constraint
    fn ne_var(self, other: VarId) -> Constraint;
    /// Create x <= value constraint
    fn le_int(self, value: i32) -> Constraint;
    /// Create x >= value constraint
    fn ge_int(self, value: i32) -> Constraint;
}

impl VarConstraints for VarId {
    fn eq_int(self, value: i32) -> Constraint {
        Constraint::EqVal(self, Val::ValI(value))
    }
    
    fn eq_float(self, value: f64) -> Constraint {
        Constraint::EqVal(self, Val::ValF(value))
    }
    
    fn eq_var(self, other: VarId) -> Constraint {
        Constraint::Eq(self, other)
    }
    
    fn ne_var(self, other: VarId) -> Constraint {
        Constraint::Ne(self, other)
    }
    
    fn le_int(self, value: i32) -> Constraint {
        Constraint::LeVal(self, Val::ValI(value))
    }
    
    fn ge_int(self, value: i32) -> Constraint {
        Constraint::GeVal(self, Val::ValI(value))
    }
}

/// Extension trait for Model to provide unified constraint posting
pub trait ModelConstraints {
    /// Post a constraint to the model - unified method that accepts multiple input types
    fn post<T: Into<ConstraintInput>>(&mut self, input: T);
    
    /// Post multiple constraints at once (batch operation)
    fn post_all<T: Into<ConstraintInput>>(&mut self, inputs: Vec<T>);
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
    
    fn post_all<T: Into<ConstraintInput>>(&mut self, inputs: Vec<T>) {
        for input in inputs {
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
}
