//! Mathematical constraint syntax with explicit typing
//!
//! This module provides helper functions and types for creating constraints
//! using mathematical notation with explicit type specification.

use crate::vars::{VarId, Val};

/// Represents a mathematical expression that can be used in constraints
#[derive(Debug, Clone)]
pub enum MathExpr {
    /// A variable
    Variable(VarId),
    /// A constant value
    Constant(TypedConstant),
    /// Absolute value: abs(x)
    Abs(Box<MathExpr>),
    /// Maximum of two expressions: max(x, y)
    Max(Box<MathExpr>, Box<MathExpr>),
    /// Minimum of two expressions: min(x, y)
    Min(Box<MathExpr>, Box<MathExpr>),
    /// Addition: x + y
    Add(Box<MathExpr>, Box<MathExpr>),
    /// Subtraction: x - y
    Sub(Box<MathExpr>, Box<MathExpr>),
    /// Multiplication: x * y
    Mul(Box<MathExpr>, Box<MathExpr>),
    /// Division: x / y
    Div(Box<MathExpr>, Box<MathExpr>),
}

/// Represents a constant value with explicit type information
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypedConstant {
    Integer(i32),
    Float(f64),
}

impl TypedConstant {
    /// Convert to internal Val representation
    pub fn to_val(self) -> Val {
        match self {
            TypedConstant::Integer(i) => Val::ValI(i),
            TypedConstant::Float(f) => Val::ValF(f),
        }
    }
    
    /// Get the integer value if it's an integer type
    pub fn as_int(self) -> Option<i32> {
        match self {
            TypedConstant::Integer(i) => Some(i),
            TypedConstant::Float(_) => None,
        }
    }
    
    /// Get the float value if it's a float type
    pub fn as_float(self) -> Option<f64> {
        match self {
            TypedConstant::Float(f) => Some(f),
            TypedConstant::Integer(_) => None,
        }
    }
    
    /// Check if this is an integer constant
    pub fn is_int(self) -> bool {
        matches!(self, TypedConstant::Integer(_))
    }
    
    /// Check if this is a float constant
    pub fn is_float(self) -> bool {
        matches!(self, TypedConstant::Float(_))
    }
}

/// Create an explicit integer constant
/// 
/// # Examples
/// 
/// ```rust
/// use cspsolver::math_syntax::int;
/// 
/// let five = int(5);  // Explicitly typed integer constant
/// ```
pub fn int(value: i32) -> TypedConstant {
    TypedConstant::Integer(value)
}

/// Create an explicit float constant
/// 
/// # Examples
/// 
/// ```rust
/// use cspsolver::math_syntax::float;
/// 
/// let pi = float(3.14159);  // Explicitly typed float constant
/// ```
pub fn float(value: f64) -> TypedConstant {
    TypedConstant::Float(value)
}

/// Mathematical functions for constraint expressions

/// Create absolute value expression: abs(x)
/// 
/// # Examples
/// 
/// ```rust
/// use cspsolver::math_syntax::{abs, int};
/// 
/// // abs(variable) - will be handled by macro
/// // abs(int(5)) creates absolute value of constant
/// let abs_five = abs(int(5));
/// ```
pub fn abs<T: Into<MathExpr>>(expr: T) -> MathExpr {
    MathExpr::Abs(Box::new(expr.into()))
}

/// Create maximum expression: max(x, y)
/// 
/// # Examples
/// 
/// ```rust
/// use cspsolver::math_syntax::{max, int};
/// 
/// // max(x, y) - variables handled by macro
/// // max(int(5), int(10)) creates max of constants
/// let max_val = max(int(5), int(10));
/// ```
pub fn max<T: Into<MathExpr>, U: Into<MathExpr>>(left: T, right: U) -> MathExpr {
    MathExpr::Max(Box::new(left.into()), Box::new(right.into()))
}

/// Create minimum expression: min(x, y)
/// 
/// # Examples
/// 
/// ```rust
/// use cspsolver::math_syntax::{min, int};
/// 
/// // min(x, y) - variables handled by macro
/// // min(int(2), int(7)) creates min of constants
/// let min_val = min(int(2), int(7));
/// ```
pub fn min<T: Into<MathExpr>, U: Into<MathExpr>>(left: T, right: U) -> MathExpr {
    MathExpr::Min(Box::new(left.into()), Box::new(right.into()))
}

// Implement conversions for easy use in mathematical expressions

impl From<VarId> for MathExpr {
    fn from(var: VarId) -> Self {
        MathExpr::Variable(var)
    }
}

impl From<TypedConstant> for MathExpr {
    fn from(constant: TypedConstant) -> Self {
        MathExpr::Constant(constant)
    }
}

impl MathExpr {
    /// Check if this expression is a simple variable
    pub fn is_variable(&self) -> bool {
        matches!(self, MathExpr::Variable(_))
    }
    
    /// Check if this expression is a constant
    pub fn is_constant(&self) -> bool {
        matches!(self, MathExpr::Constant(_))
    }
    
    /// Get the variable if this is a simple variable expression
    pub fn as_variable(&self) -> Option<VarId> {
        match self {
            MathExpr::Variable(var) => Some(*var),
            _ => None,
        }
    }
    
    /// Get the constant if this is a constant expression
    pub fn as_constant(&self) -> Option<TypedConstant> {
        match self {
            MathExpr::Constant(constant) => Some(*constant),
            _ => None,
        }
    }
    
    /// Check if this expression involves only constants (can be evaluated immediately)
    pub fn is_evaluable(&self) -> bool {
        match self {
            MathExpr::Constant(_) => true,
            MathExpr::Variable(_) => false,
            MathExpr::Abs(expr) => expr.is_evaluable(),
            MathExpr::Max(left, right) | MathExpr::Min(left, right) |
            MathExpr::Add(left, right) | MathExpr::Sub(left, right) |
            MathExpr::Mul(left, right) | MathExpr::Div(left, right) => {
                left.is_evaluable() && right.is_evaluable()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_int_constant() {
        let five = int(5);
        assert_eq!(five.as_int(), Some(5));
        assert_eq!(five.to_val(), Val::ValI(5));
        assert!(five.is_int());
        assert!(!five.is_float());
    }
    
    #[test]
    fn test_float_constant() {
        let pi = float(3.14159);
        assert_eq!(pi.as_float(), Some(3.14159));
        assert_eq!(pi.to_val(), Val::ValF(3.14159));
        assert!(pi.is_float());
        assert!(!pi.is_int());
    }
    
    #[test]
    fn test_type_safety() {
        let int_val = int(42);
        let float_val = float(2.71);
        
        // These should work
        assert_eq!(int_val.as_int(), Some(42));
        assert_eq!(float_val.as_float(), Some(2.71));
        
        // Cross-type access should return None
        assert_eq!(int_val.as_float(), None);
        assert_eq!(float_val.as_int(), None);
    }
    
    #[test]
    fn test_type_checking() {
        let int_val = int(42);
        let float_val = float(2.71);
        
        assert!(int_val.is_int());
        assert!(!int_val.is_float());
        
        assert!(float_val.is_float());
        assert!(!float_val.is_int());
    }
    
    #[test]
    fn test_math_expressions() {
        let five = int(5);
        let ten = int(10);
        
        // Test abs function
        let abs_expr = abs(five);
        assert!(matches!(abs_expr, MathExpr::Abs(_)));
        
        // Test max function
        let max_expr = max(five, ten);
        assert!(matches!(max_expr, MathExpr::Max(_, _)));
        
        // Test min function
        let min_expr = min(five, ten);
        assert!(matches!(min_expr, MathExpr::Min(_, _)));
    }
    
    #[test]
    fn test_expr_properties() {
        let five = int(5);
        let constant_expr = MathExpr::from(five);
        
        assert!(constant_expr.is_constant());
        assert!(!constant_expr.is_variable());
        assert!(constant_expr.is_evaluable());
        assert_eq!(constant_expr.as_constant(), Some(five));
        assert_eq!(constant_expr.as_variable(), None);
    }
}