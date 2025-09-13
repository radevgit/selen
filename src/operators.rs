//! Operator traits and implementations for convenient constraint syntax.
//!
//! This module provides trait-based APIs that allow using standard Rust operators
//! with VarId to create constraints in a more natural and readable way.
//!
//! # Examples
//!
//! ```rust
//! use cspsolver::prelude::*;
//! use cspsolver::operators::*;
//!
//! let mut m = Model::default();
//! let x = m.int(1, 10);
//! let y = m.int(1, 10);
//! 
//! // Comparison operations
//! m.eq_op(x, y);         // x == y
//! m.ne_op(x, y);         // x != y
//! m.lt_op(x, y);         // x < y
//! m.le_op(x, y);         // x <= y
//! m.gt_op(x, y);         // x > y
//! m.ge_op(x, y);         // x >= y
//!
//! // Boolean operations (for boolean variables)
//! let a = m.bool();
//! let b = m.bool();
//! m.and_op(a, b);        // a AND b
//! m.or_op(a, b);         // a OR b
//! m.not_op(a);           // NOT a
//! ```

use crate::vars::VarId;
use crate::model::Model;

/// Trait for comparison operations on variables.
///
/// Provides methods for creating comparison constraints between variables.
pub trait ComparisonOp {
    /// Create an equality constraint: self == other
    fn eq_op(&self, model: &mut Model, other: VarId);
    
    /// Create an inequality constraint: self != other
    fn ne_op(&self, model: &mut Model, other: VarId);
    
    /// Create a less-than constraint: self < other
    fn lt_op(&self, model: &mut Model, other: VarId);
    
    /// Create a less-than-or-equal constraint: self <= other
    fn le_op(&self, model: &mut Model, other: VarId);
    
    /// Create a greater-than constraint: self > other
    fn gt_op(&self, model: &mut Model, other: VarId);
    
    /// Create a greater-than-or-equal constraint: self >= other
    fn ge_op(&self, model: &mut Model, other: VarId);
}

/// Trait for boolean operations on variables.
///
/// Provides methods for creating boolean logic constraints.
pub trait BooleanOp {
    /// Create a boolean AND constraint: self AND other
    fn and_op(&self, model: &mut Model, other: VarId);
    
    /// Create a boolean OR constraint: self OR other
    fn or_op(&self, model: &mut Model, other: VarId);
    
    /// Create a boolean NOT constraint: NOT self
    fn not_op(&self, model: &mut Model);
}

// Implement ComparisonOp for VarId
impl ComparisonOp for VarId {
    fn eq_op(&self, model: &mut Model, other: VarId) {
        model.props.equals(*self, other);
    }
    
    fn ne_op(&self, model: &mut Model, other: VarId) {
        model.props.not_equals(*self, other);
    }
    
    fn lt_op(&self, model: &mut Model, other: VarId) {
        model.props.less_than(*self, other);
    }
    
    fn le_op(&self, model: &mut Model, other: VarId) {
        model.props.less_than_or_equals(*self, other);
    }
    
    fn gt_op(&self, model: &mut Model, other: VarId) {
        model.props.greater_than(*self, other);
    }
    
    fn ge_op(&self, model: &mut Model, other: VarId) {
        model.props.greater_than_or_equals(*self, other);
    }
}

// Implement BooleanOp for VarId
impl BooleanOp for VarId {
    fn and_op(&self, model: &mut Model, other: VarId) {
        let _result = model.bool_and(&[*self, other]);
    }
    
    fn or_op(&self, model: &mut Model, other: VarId) {
        let _result = model.bool_or(&[*self, other]);
    }
    
    fn not_op(&self, model: &mut Model) {
        let _result = model.bool_not(*self);
    }
}

// Extension methods for Model to support operator-based constraint creation
impl Model {
    /// Create equality constraint using operator syntax
    pub fn eq_op(&mut self, left: VarId, right: VarId) {
        self.props.equals(left, right);
    }
    
    /// Create inequality constraint using operator syntax
    pub fn ne_op(&mut self, left: VarId, right: VarId) {
        self.props.not_equals(left, right);
    }
    
    /// Create less-than constraint using operator syntax
    pub fn lt_op(&mut self, left: VarId, right: VarId) {
        self.props.less_than(left, right);
    }
    
    /// Create less-than-or-equal constraint using operator syntax
    pub fn le_op(&mut self, left: VarId, right: VarId) {
        self.props.less_than_or_equals(left, right);
    }
    
    /// Create greater-than constraint using operator syntax
    pub fn gt_op(&mut self, left: VarId, right: VarId) {
        self.props.greater_than(left, right);
    }
    
    /// Create greater-than-or-equal constraint using operator syntax
    pub fn ge_op(&mut self, left: VarId, right: VarId) {
        self.props.greater_than_or_equals(left, right);
    }
    
    /// Create boolean AND constraint using operator syntax
    pub fn and_op(&mut self, left: VarId, right: VarId) {
        let _result = self.bool_and(&[left, right]);
    }
    
    /// Create boolean OR constraint using operator syntax
    pub fn or_op(&mut self, left: VarId, right: VarId) {
        let _result = self.bool_or(&[left, right]);
    }
    
    /// Create boolean NOT constraint using operator syntax
    pub fn not_op(&mut self, var: VarId) {
        let _result = self.bool_not(var);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_comparison_operators() {
        let mut m = Model::default();
        let x = m.int(1, 10);
        let y = m.int(1, 10);
        
        // Test that operator methods work without panicking
        x.eq_op(&mut m, y);
        x.ne_op(&mut m, y);
        x.lt_op(&mut m, y);
        x.le_op(&mut m, y);
        x.gt_op(&mut m, y);
        x.ge_op(&mut m, y);
    }
    
    #[test]
    fn test_boolean_operators() {
        let mut m = Model::default();
        let a = m.int(0, 1); // Boolean variable
        let b = m.int(0, 1); // Boolean variable
        
        // Test that boolean operator methods work without panicking
        a.and_op(&mut m, b);
        a.or_op(&mut m, b);
        a.not_op(&mut m);
    }
    
    #[test]
    fn test_model_operator_extensions() {
        let mut m = Model::default();
        let x = m.int(1, 10);
        let y = m.int(1, 10);
        
        // Test Model extension methods
        m.eq_op(x, y);
        m.ne_op(x, y);
        m.lt_op(x, y);
        m.le_op(x, y);
        m.gt_op(x, y);
        m.ge_op(x, y);
        
        let a = m.int(0, 1); // Boolean variable
        let b = m.int(0, 1); // Boolean variable
        m.and_op(a, b);
        m.or_op(a, b);
        m.not_op(a);
    }
}
