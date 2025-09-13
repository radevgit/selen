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
            Constraint::Eq(x, y) => { let _p = model.props.equals(x, y); },
            Constraint::Ne(x, y) => { let _p = model.props.not_equals(x, y); },
            Constraint::Lt(x, y) => { let _p = model.props.less_than(x, y); },
            Constraint::Le(x, y) => { let _p = model.props.less_than_or_equals(x, y); },
            Constraint::Gt(x, y) => { let _p = model.props.greater_than(x, y); },
            Constraint::Ge(x, y) => { let _p = model.props.greater_than_or_equals(x, y); },
            Constraint::EqVal(x, val) => { let _p = model.props.equals(x, val); },
            Constraint::LeVal(x, val) => { let _p = model.props.less_than_or_equals(x, val); },
            Constraint::GeVal(x, val) => { let _p = model.props.greater_than_or_equals(x, val); },
            Constraint::GtVal(x, val) => { let _p = model.props.greater_than(x, val); },
            Constraint::LtVal(x, val) => { let _p = model.props.less_than(x, val); },
            Constraint::BoolTrue(expr) => {
                let result_var = model.bool_expr(expr);
                let _p = model.props.equals(result_var, Val::ValI(1));
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
