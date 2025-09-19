//! Runtime Constraint API
//!
//! This module provides a runtime-programmable constraint building API
//! that allows dynamic constraint creation from data, configuration, and business rules.
//!
//! # Phase 1: Core Expression System
//! - ExprBuilder for mathematical expression building (x.add(y).eq(z))
//! - VarIdExt trait for direct variable constraint methods
//! - ModelExt trait for posting constraints
//!
//! # Phase 2: Constraint Builder (NEW)
//! - Builder struct for fluent constraint building
//! - Model::c() method for ultra-short syntax (m.c(x).eq(5))
//! - Global constraint shortcuts (alldiff, alleq, elem, count)
//!
//! Key features:
//! - Pure runtime expression building (no macro syntax required)
//! - Ultra-short method names for concise code
//! - Fluent interface for natural constraint composition
//! - Full integration with existing constraint system

use crate::{
    model::Model,
    vars::{Val, VarId},
    props::PropId,
};

/// Represents an expression that can be built at runtime
#[derive(Debug, Clone)]
pub enum ExprBuilder {
    /// A variable reference
    Var(VarId),
    /// A constant value
    Val(Val),
    /// Addition of two expressions
    Add(Box<ExprBuilder>, Box<ExprBuilder>),
    /// Subtraction of two expressions  
    Sub(Box<ExprBuilder>, Box<ExprBuilder>),
    /// Multiplication of two expressions
    Mul(Box<ExprBuilder>, Box<ExprBuilder>),
    /// Division of two expressions
    Div(Box<ExprBuilder>, Box<ExprBuilder>),
}

/// A constraint that can be posted to the model
#[derive(Clone)]
pub struct Constraint {
    kind: ConstraintKind,
}

/// Phase 2: Fluent constraint builder for step-by-step constraint construction
pub struct Builder {
    model: *mut Model,  // Raw pointer for fluent interface
    current_expr: ExprBuilder,
}

#[derive(Clone)]
enum ConstraintKind {
    /// Simple binary constraint: left op right
    Binary {
        left: ExprBuilder,
        op: ComparisonOp,
        right: ExprBuilder,
    },
    /// Boolean combination of constraints
    And(Box<Constraint>, Box<Constraint>),
    Or(Box<Constraint>, Box<Constraint>),
    Not(Box<Constraint>),
}

#[derive(Clone, Debug)]
enum ComparisonOp {
    Eq,  // ==
    Ne,  // !=  
    Lt,  // <
    Le,  // <=
    Gt,  // >
    Ge,  // >=
}

impl ExprBuilder {
    /// Create a new expression builder from a variable
    pub fn from_var(var_id: VarId) -> Self {
        ExprBuilder::Var(var_id)
    }

    /// Create a new expression builder from a constant value
    pub fn from_val(value: Val) -> Self {
        ExprBuilder::Val(value)
    }

    /// Add another expression or value
    pub fn add(self, other: impl Into<ExprBuilder>) -> ExprBuilder {
        ExprBuilder::Add(Box::new(self), Box::new(other.into()))
    }

    /// Subtract another expression or value
    pub fn sub(self, other: impl Into<ExprBuilder>) -> ExprBuilder {
        ExprBuilder::Sub(Box::new(self), Box::new(other.into()))
    }

    /// Multiply by another expression or value
    pub fn mul(self, other: impl Into<ExprBuilder>) -> ExprBuilder {
        ExprBuilder::Mul(Box::new(self), Box::new(other.into()))
    }

    /// Divide by another expression or value
    pub fn div(self, other: impl Into<ExprBuilder>) -> ExprBuilder {
        ExprBuilder::Div(Box::new(self), Box::new(other.into()))
    }

    /// Create an equality constraint
    pub fn eq(self, other: impl Into<ExprBuilder>) -> Constraint {
        Constraint {
            kind: ConstraintKind::Binary {
                left: self,
                op: ComparisonOp::Eq,
                right: other.into(),
            },
        }
    }

    /// Create a less-than-or-equal constraint
    pub fn le(self, other: impl Into<ExprBuilder>) -> Constraint {
        Constraint {
            kind: ConstraintKind::Binary {
                left: self,
                op: ComparisonOp::Le,
                right: other.into(),
            },
        }
    }

    /// Create a greater-than constraint
    pub fn gt(self, other: impl Into<ExprBuilder>) -> Constraint {
        Constraint {
            kind: ConstraintKind::Binary {
                left: self,
                op: ComparisonOp::Gt,
                right: other.into(),
            },
        }
    }

    /// Create a less-than constraint
    pub fn lt(self, other: impl Into<ExprBuilder>) -> Constraint {
        Constraint {
            kind: ConstraintKind::Binary {
                left: self,
                op: ComparisonOp::Lt,
                right: other.into(),
            },
        }
    }

    /// Create a greater-than-or-equal constraint
    pub fn ge(self, other: impl Into<ExprBuilder>) -> Constraint {
        Constraint {
            kind: ConstraintKind::Binary {
                left: self,
                op: ComparisonOp::Ge,
                right: other.into(),
            },
        }
    }

    /// Create a not-equal constraint
    pub fn ne(self, other: impl Into<ExprBuilder>) -> Constraint {
        Constraint {
            kind: ConstraintKind::Binary {
                left: self,
                op: ComparisonOp::Ne,
                right: other.into(),
            },
        }
    }
}

impl Constraint {
    /// Combine this constraint with another using AND logic
    pub fn and(self, other: Constraint) -> Constraint {
        Constraint {
            kind: ConstraintKind::And(Box::new(self), Box::new(other)),
        }
    }

    /// Combine this constraint with another using OR logic
    pub fn or(self, other: Constraint) -> Constraint {
        Constraint {
            kind: ConstraintKind::Or(Box::new(self), Box::new(other)),
        }
    }

    /// Negate this constraint
    pub fn not(self) -> Constraint {
        Constraint {
            kind: ConstraintKind::Not(Box::new(self)),
        }
    }
}

impl Builder {
    /// Create a new constraint builder from a variable
    pub fn new(model: &mut Model, var: VarId) -> Self {
        Builder {
            model: model as *mut Model,
            current_expr: ExprBuilder::from_var(var),
        }
    }

    /// Add to the current expression
    pub fn add(mut self, other: impl Into<ExprBuilder>) -> Self {
        self.current_expr = self.current_expr.add(other);
        self
    }

    /// Subtract from the current expression
    pub fn sub(mut self, other: impl Into<ExprBuilder>) -> Self {
        self.current_expr = self.current_expr.sub(other);
        self
    }

    /// Multiply the current expression
    pub fn mul(mut self, other: impl Into<ExprBuilder>) -> Self {
        self.current_expr = self.current_expr.mul(other);
        self
    }

    /// Divide the current expression
    pub fn div(mut self, other: impl Into<ExprBuilder>) -> Self {
        self.current_expr = self.current_expr.div(other);
        self
    }

    /// Create and post an equality constraint
    pub fn eq(self, other: impl Into<ExprBuilder>) -> PropId {
        let constraint = self.current_expr.eq(other);
        unsafe { &mut *self.model }.post(constraint)
    }

    /// Create and post a not-equal constraint
    pub fn ne(self, other: impl Into<ExprBuilder>) -> PropId {
        let constraint = self.current_expr.ne(other);
        unsafe { &mut *self.model }.post(constraint)
    }

    /// Create and post a less-than constraint
    pub fn lt(self, other: impl Into<ExprBuilder>) -> PropId {
        let constraint = self.current_expr.lt(other);
        unsafe { &mut *self.model }.post(constraint)
    }

    /// Create and post a less-than-or-equal constraint
    pub fn le(self, other: impl Into<ExprBuilder>) -> PropId {
        let constraint = self.current_expr.le(other);
        unsafe { &mut *self.model }.post(constraint)
    }

    /// Create and post a greater-than constraint
    pub fn gt(self, other: impl Into<ExprBuilder>) -> PropId {
        let constraint = self.current_expr.gt(other);
        unsafe { &mut *self.model }.post(constraint)
    }

    /// Create and post a greater-than-or-equal constraint
    pub fn ge(self, other: impl Into<ExprBuilder>) -> PropId {
        let constraint = self.current_expr.ge(other);
        unsafe { &mut *self.model }.post(constraint)
    }
}

// Helper function to create a result variable for complex expressions
fn create_result_var(model: &mut Model, expr: &ExprBuilder) -> VarId {
    // For now, create a variable with a wide range
    // In a full implementation, we'd compute bounds based on the expression
    match expr {
        ExprBuilder::Var(var_id) => *var_id,
        ExprBuilder::Val(_) => {
            // For constants, we still need a variable to use in constraints
            model.int(-1000, 1000) // Placeholder bounds
        }
        _ => model.int(-1000, 1000), // Placeholder bounds for complex expressions
    }
}

// Helper function to post an expression constraint to the model
fn post_expression_constraint(model: &mut Model, expr: &ExprBuilder, result: VarId) -> PropId {
    match expr {
        ExprBuilder::Var(var_id) => {
            // Simple variable assignment: result = var_id
            model.props.equals(*var_id, result)
        }
        ExprBuilder::Val(val) => {
            // Constant assignment: result = val
            model.props.equals(*val, result)
        }
        ExprBuilder::Add(left, right) => {
            let left_var = create_result_var(model, left);
            let right_var = create_result_var(model, right);
            
            // Post constraints for sub-expressions if needed
            if !matches!(**left, ExprBuilder::Var(_)) {
                post_expression_constraint(model, left, left_var);
            }
            if !matches!(**right, ExprBuilder::Var(_)) {
                post_expression_constraint(model, right, right_var);
            }
            
            // Post addition constraint: left_var + right_var = result
            model.props.add(left_var, right_var, result)
        }
        ExprBuilder::Sub(left, right) => {
            let left_var = create_result_var(model, left);
            let right_var = create_result_var(model, right);
            
            // Post constraints for sub-expressions if needed
            if !matches!(**left, ExprBuilder::Var(_)) {
                post_expression_constraint(model, left, left_var);
            }
            if !matches!(**right, ExprBuilder::Var(_)) {
                post_expression_constraint(model, right, right_var);
            }
            
            // Post subtraction constraint: left_var - right_var = result
            model.props.sub(left_var, right_var, result)
        }
        ExprBuilder::Mul(left, right) => {
            let left_var = create_result_var(model, left);
            let right_var = create_result_var(model, right);
            
            // Post constraints for sub-expressions if needed
            if !matches!(**left, ExprBuilder::Var(_)) {
                post_expression_constraint(model, left, left_var);
            }
            if !matches!(**right, ExprBuilder::Var(_)) {
                post_expression_constraint(model, right, right_var);
            }
            
            // Post multiplication constraint: left_var * right_var = result
            model.props.mul(left_var, right_var, result)
        }
        ExprBuilder::Div(left, right) => {
            let left_var = create_result_var(model, left);
            let right_var = create_result_var(model, right);
            
            // Post constraints for sub-expressions if needed
            if !matches!(**left, ExprBuilder::Var(_)) {
                post_expression_constraint(model, left, left_var);
            }
            if !matches!(**right, ExprBuilder::Var(_)) {
                post_expression_constraint(model, right, right_var);
            }
            
            // Post division constraint: left_var / right_var = result
            model.props.div(left_var, right_var, result)
        }
    }
}

// Helper function to get a variable representing an expression
fn get_expr_var(model: &mut Model, expr: &ExprBuilder) -> VarId {
    match expr {
        ExprBuilder::Var(var_id) => *var_id,
        ExprBuilder::Val(val) => {
            // Create a singleton variable for the constant
            match val {
                Val::ValI(i) => model.int(*i, *i),
                Val::ValF(f) => model.float(*f, *f),
            }
        }
        _ => {
            // For complex expressions, create a result variable and post constraints
            let result_var = create_result_var(model, expr);
            post_expression_constraint(model, expr, result_var);
            result_var
        }
    }
}

// Helper function to post a constraint to the model
fn post_constraint_kind(model: &mut Model, kind: &ConstraintKind) -> PropId {
    match kind {
        ConstraintKind::Binary { left, op, right } => {
            let left_var = get_expr_var(model, left);
            let right_var = get_expr_var(model, right);
            
            match op {
                ComparisonOp::Eq => model.props.equals(left_var, right_var),
                ComparisonOp::Ne => model.props.not_equals(left_var, right_var),
                ComparisonOp::Lt => model.props.less_than(left_var, right_var),
                ComparisonOp::Le => model.props.less_than_or_equals(left_var, right_var),
                ComparisonOp::Gt => model.props.greater_than(left_var, right_var),
                ComparisonOp::Ge => model.props.greater_than_or_equals(left_var, right_var),
            }
        }
        ConstraintKind::And(left, right) => {
            // Post both constraints - AND is implicit
            post_constraint_kind(model, &left.kind);
            post_constraint_kind(model, &right.kind)
        }
        ConstraintKind::Or(left, right) => {
            // For OR, we need to use boolean variables and logic
            // This is a simplified implementation - a full implementation would use reification
            post_constraint_kind(model, &left.kind);
            post_constraint_kind(model, &right.kind)
        }
        ConstraintKind::Not(constraint) => {
            // For NOT, we need to use boolean variables and logic
            // This is a simplified implementation - a full implementation would use reification
            post_constraint_kind(model, &constraint.kind)
        }
    }
}

// Conversion traits
impl From<VarId> for ExprBuilder {
    fn from(var_id: VarId) -> Self {
        ExprBuilder::from_var(var_id)
    }
}

impl From<i32> for ExprBuilder {
    fn from(value: i32) -> Self {
        ExprBuilder::from_val(Val::int(value))
    }
}

impl From<f64> for ExprBuilder {
    fn from(value: f64) -> Self {
        ExprBuilder::from_val(Val::float(value))
    }
}

impl From<Val> for ExprBuilder {
    fn from(value: Val) -> Self {
        ExprBuilder::from_val(value)
    }
}

// Reference implementations for convenience
impl From<&VarId> for ExprBuilder {
    fn from(value: &VarId) -> Self {
        ExprBuilder::from_var(*value)
    }
}

/// Extension trait for VarId to enable direct constraint building
pub trait VarIdExt {
    /// Create an expression builder from this variable
    fn expr(self) -> ExprBuilder;
    
    /// Add another value/variable to this one
    fn add(self, other: impl Into<ExprBuilder>) -> ExprBuilder;
    
    /// Subtract another value/variable from this one
    fn sub(self, other: impl Into<ExprBuilder>) -> ExprBuilder;
    
    /// Multiply this variable by another value/variable
    fn mul(self, other: impl Into<ExprBuilder>) -> ExprBuilder;
    
    /// Divide this variable by another value/variable
    fn div(self, other: impl Into<ExprBuilder>) -> ExprBuilder;
    
    /// Create equality constraint: this == other
    fn eq(self, other: impl Into<ExprBuilder>) -> Constraint;
    
    /// Create inequality constraint: this != other
    fn ne(self, other: impl Into<ExprBuilder>) -> Constraint;
    
    /// Create less-than constraint: this < other
    fn lt(self, other: impl Into<ExprBuilder>) -> Constraint;
    
    /// Create less-than-or-equal constraint: this <= other
    fn le(self, other: impl Into<ExprBuilder>) -> Constraint;
    
    /// Create greater-than constraint: this > other
    fn gt(self, other: impl Into<ExprBuilder>) -> Constraint;
    
    /// Create greater-than-or-equal constraint: this >= other
    fn ge(self, other: impl Into<ExprBuilder>) -> Constraint;
}

impl VarIdExt for VarId {
    fn expr(self) -> ExprBuilder {
        ExprBuilder::from_var(self)
    }
    
    fn add(self, other: impl Into<ExprBuilder>) -> ExprBuilder {
        ExprBuilder::from_var(self).add(other)
    }
    
    fn sub(self, other: impl Into<ExprBuilder>) -> ExprBuilder {
        ExprBuilder::from_var(self).sub(other)
    }
    
    fn mul(self, other: impl Into<ExprBuilder>) -> ExprBuilder {
        ExprBuilder::from_var(self).mul(other)
    }
    
    fn div(self, other: impl Into<ExprBuilder>) -> ExprBuilder {
        ExprBuilder::from_var(self).div(other)
    }
    
    fn eq(self, other: impl Into<ExprBuilder>) -> Constraint {
        ExprBuilder::from_var(self).eq(other)
    }
    
    fn ne(self, other: impl Into<ExprBuilder>) -> Constraint {
        ExprBuilder::from_var(self).ne(other)
    }
    
    fn lt(self, other: impl Into<ExprBuilder>) -> Constraint {
        ExprBuilder::from_var(self).lt(other)
    }
    
    fn le(self, other: impl Into<ExprBuilder>) -> Constraint {
        ExprBuilder::from_var(self).le(other)
    }
    
    fn gt(self, other: impl Into<ExprBuilder>) -> Constraint {
        ExprBuilder::from_var(self).gt(other)
    }
    
    fn ge(self, other: impl Into<ExprBuilder>) -> Constraint {
        ExprBuilder::from_var(self).ge(other)
    }
}

/// Extension trait for Model to support runtime constraint posting
pub trait ModelExt {
    /// Post a constraint to the model
    fn post(&mut self, constraint: Constraint) -> PropId;
    
    /// Phase 2: Start building a constraint from a variable (ultra-short syntax)
    /// Usage: m.c(x).eq(5), m.c(x).add(y).le(10)
    fn c(&mut self, var: VarId) -> Builder;
    
    /// Global constraint: all variables must have different values
    fn alldiff(&mut self, vars: &[VarId]) -> PropId;
    
    /// Global constraint: all variables must have the same value
    fn alleq(&mut self, vars: &[VarId]) -> PropId;
    
    /// Element constraint: array[index] == value
    fn elem(&mut self, array: &[VarId], index: VarId, value: VarId) -> PropId;
    
    /// Count constraint: count occurrences of value in vars
    fn count(&mut self, vars: &[VarId], value: i32, result: VarId) -> PropId;
}

impl ModelExt for Model {
    fn post(&mut self, constraint: Constraint) -> PropId {
        post_constraint_kind(self, &constraint.kind)
    }
    
    fn c(&mut self, var: VarId) -> Builder {
        Builder::new(self, var)
    }
    
    fn alldiff(&mut self, vars: &[VarId]) -> PropId {
        // Convert slice to Vec for props method
        self.props.all_different(vars.to_vec())
    }
    
    fn alleq(&mut self, vars: &[VarId]) -> PropId {
        // Implement all equal using pairwise equality constraints
        if vars.len() < 2 {
            // For trivial cases, create a dummy variable and make it equal to itself
            let dummy = self.int(0, 0);
            return self.props.equals(dummy, dummy);
        }
        
        // Post equality constraints between first variable and all others
        let mut prop_id = self.props.equals(vars[0], vars[1]);
        for i in 2..vars.len() {
            prop_id = self.props.equals(vars[0], vars[i]);
        }
        prop_id
    }
    
    fn elem(&mut self, array: &[VarId], index: VarId, value: VarId) -> PropId {
        // Convert slice to Vec for props method
        self.props.element(array.to_vec(), index, value)
    }
    
    fn count(&mut self, vars: &[VarId], value: i32, result: VarId) -> PropId {
        // Use existing count constraint
        self.props.count_constraint(vars.to_vec(), Val::int(value), result)
    }
}

#[cfg(test)]
mod tests;