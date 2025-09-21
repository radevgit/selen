//! Operator overloads for VarId to provide clean syntax.
//!
//! This module implements operator overloading for VarId to enable:
//! - Function-style boolean operations: and(a, b), or(a, b), not(a) that work directly with post()
//! - Runtime API boolean operations: constraint.and(), constraint.or(), constraint.not()
//!
//! ## Function-Style API (Macro-based)
//! ```rust
//! use cspsolver::prelude::*;
//! 
//! let mut m = Model::default();
//! let a = m.bool();
//! let b = m.bool();
//! 
//! post!(m, and(a, b));        // Boolean AND
//! post!(m, or(a, b));         // Boolean OR
//! post!(m, not(a));           // Boolean NOT
//! ```
//!
//! ## Runtime API (Programmatic)
//! ```rust
//! use cspsolver::prelude::*;
//! 
//! let mut m = Model::default();
//! let a = m.bool();
//! let b = m.bool();
//! 
//! // Create constraint expressions first, then post them
//! m.new(a.eq(1).and(b.eq(1)));     // (a == 1) && (b == 1)
//! m.new(a.eq(1).or(b.eq(1)));      // (a == 1) || (b == 1)
//! m.new(a.eq(1).not());            // !(a == 1)
//! ```

use crate::variables::VarId;
use crate::model::Model;
use crate::constraints::builder_legacy::Constraint;
use std::ops::{BitAnd, BitOr, Not};

#[doc(hidden)]
/// Represents a boolean expression that can be applied to a model
/// This automatically creates variables when applied
#[derive(Debug, Clone)]
pub struct BoolExpr {
    pub(crate) operation: BoolOperation,
}

#[derive(Debug, Clone)]
pub(crate) enum BoolOperation {
    /// Simple variable
    Var(VarId),
    /// AND operation: left & right
    And(Box<BoolExpr>, Box<BoolExpr>),
    /// OR operation: left | right
    Or(Box<BoolExpr>, Box<BoolExpr>),
    /// NOT operation: !operand
    Not(Box<BoolExpr>),
}

impl BoolExpr {
    /// Apply this boolean expression to the model and return the result variable
    pub fn apply_to(self, model: &mut Model) -> VarId {
        match self.operation {
            BoolOperation::Var(var_id) => var_id,
            BoolOperation::And(left, right) => {
                let left_var = left.apply_to(model);
                let right_var = right.apply_to(model);
                let result = model.bool();
                let mut vars = Vec::with_capacity(2);
                vars.push(left_var);
                vars.push(right_var);
                model.props.bool_and(vars, result);
                result
            }
            BoolOperation::Or(left, right) => {
                let left_var = left.apply_to(model);
                let right_var = right.apply_to(model);
                let result = model.bool();
                let mut vars = Vec::with_capacity(2);
                vars.push(left_var);
                vars.push(right_var);
                model.props.bool_or(vars, result);
                result
            }
            BoolOperation::Not(operand) => {
                let operand_var = operand.apply_to(model);
                let result = model.bool();
                model.props.bool_not(operand_var, result);
                result
            }
        }
    }
    
    /// Create a constraint that this boolean expression must be true (== 1)
    pub fn must_be_true(self, model: &mut Model) -> Constraint {
        let result_var = self.apply_to(model);
        Constraint::EqVal(result_var, crate::variables::Val::ValI(1))
    }
    
    /// Create a constraint that this boolean expression must be false (== 0)
    pub fn must_be_false(self, model: &mut Model) -> Constraint {
        let result_var = self.apply_to(model);
        Constraint::EqVal(result_var, crate::variables::Val::ValI(0))
    }
}

// Implement From trait to automatically convert VarId to BoolExpr
impl From<VarId> for BoolExpr {
    fn from(var_id: VarId) -> Self {
        BoolExpr {
            operation: BoolOperation::Var(var_id)
        }
    }
}

// Implement BitAnd (&) for VarId
impl BitAnd for VarId {
    type Output = BoolExpr;
    
    fn bitand(self, rhs: Self) -> Self::Output {
        BoolExpr {
            operation: BoolOperation::And(
                Box::new(BoolExpr::from(self)),
                Box::new(BoolExpr::from(rhs))
            )
        }
    }
}

// Implement BitOr (|) for VarId  
impl BitOr for VarId {
    type Output = BoolExpr;
    
    fn bitor(self, rhs: Self) -> Self::Output {
        BoolExpr {
            operation: BoolOperation::Or(
                Box::new(BoolExpr::from(self)),
                Box::new(BoolExpr::from(rhs))
            )
        }
    }
}

// Implement Not (!) for VarId
impl Not for VarId {
    type Output = BoolExpr;
    
    fn not(self) -> Self::Output {
        BoolExpr {
            operation: BoolOperation::Not(
                Box::new(BoolExpr::from(self))
            )
        }
    }
}

// Implement BitAnd (&) for BoolExpr (to enable chaining)
impl BitAnd for BoolExpr {
    type Output = BoolExpr;
    
    fn bitand(self, rhs: Self) -> Self::Output {
        BoolExpr {
            operation: BoolOperation::And(Box::new(self), Box::new(rhs))
        }
    }
}

// Implement BitAnd between VarId and BoolExpr
impl BitAnd<BoolExpr> for VarId {
    type Output = BoolExpr;
    
    fn bitand(self, rhs: BoolExpr) -> Self::Output {
        BoolExpr {
            operation: BoolOperation::And(
                Box::new(BoolExpr::from(self)),
                Box::new(rhs)
            )
        }
    }
}

// Implement BitAnd between BoolExpr and VarId
impl BitAnd<VarId> for BoolExpr {
    type Output = BoolExpr;
    
    fn bitand(self, rhs: VarId) -> Self::Output {
        BoolExpr {
            operation: BoolOperation::And(
                Box::new(self),
                Box::new(BoolExpr::from(rhs))
            )
        }
    }
}

// Implement BitOr (|) for BoolExpr (to enable chaining)
impl BitOr for BoolExpr {
    type Output = BoolExpr;
    
    fn bitor(self, rhs: Self) -> Self::Output {
        BoolExpr {
            operation: BoolOperation::Or(Box::new(self), Box::new(rhs))
        }
    }
}

// Implement BitOr between VarId and BoolExpr
impl BitOr<BoolExpr> for VarId {
    type Output = BoolExpr;
    
    fn bitor(self, rhs: BoolExpr) -> Self::Output {
        BoolExpr {
            operation: BoolOperation::Or(
                Box::new(BoolExpr::from(self)),
                Box::new(rhs)
            )
        }
    }
}

// Implement BitOr between BoolExpr and VarId
impl BitOr<VarId> for BoolExpr {
    type Output = BoolExpr;
    
    fn bitor(self, rhs: VarId) -> Self::Output {
        BoolExpr {
            operation: BoolOperation::Or(
                Box::new(self),
                Box::new(BoolExpr::from(rhs))
            )
        }
    }
}

// Implement Not (!) for BoolExpr (to enable chaining)
impl Not for BoolExpr {
    type Output = BoolExpr;
    
    fn not(self) -> Self::Output {
        BoolExpr {
            operation: BoolOperation::Not(Box::new(self))
        }
    }
}

#[doc(hidden)]
/// Extension trait for Model to work with boolean expressions
pub trait BooleanModel {
    /// Post a boolean expression as a constraint that must be true
    fn post_true(&mut self, expr: BoolExpr);
    
    /// Post a boolean expression as a constraint that must be false  
    fn post_false(&mut self, expr: BoolExpr);
}

impl BooleanModel for Model {
    fn post_true(&mut self, expr: BoolExpr) {
        let constraint = expr.must_be_true(self);
        constraint.apply_to(self);
    }
    
    fn post_false(&mut self, expr: BoolExpr) {
        let constraint = expr.must_be_false(self);
        constraint.apply_to(self);
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::post;
    
    #[test]
    fn test_bitwise_boolean_operators() {
        let mut m = Model::default();
        
        let a = m.bool();
        let b = m.bool();
        let c = m.bool();
        
        // Test function-style boolean operations using post! macro
        // Instead of creating result variables, we post constraints directly
        
        // Set up test values
        m.props.equals(a, crate::variables::Val::ValI(1));
        m.props.equals(b, crate::variables::Val::ValI(0)); 
        m.props.equals(c, crate::variables::Val::ValI(0));
        
        // Test that function-style boolean operations work
        // These should be satisfiable with the values above
        post!(m, or(a, b));   // 1 | 0 = 1 (true)
        post!(m, not(c));     // !0 = 1 (true)
        
        let solution = m.solve().unwrap();
        let a_val = if let crate::variables::Val::ValI(v) = solution[a] { v } else { 0 };
        let b_val = if let crate::variables::Val::ValI(v) = solution[b] { v } else { 0 };
        let c_val = if let crate::variables::Val::ValI(v) = solution[c] { v } else { 0 };
        
        assert_eq!(a_val, 1);
        assert_eq!(b_val, 0);
        assert_eq!(c_val, 0);
        
        // Verify the boolean logic manually
        // or(a, b) with a=1, b=0 should be true (1 | 0 = 1)
        // not(c) with c=0 should be true (!0 = 1)
    }
}
