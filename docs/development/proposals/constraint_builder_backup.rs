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

/// Extension trait for VarId to provide clean constraint creation API.
pub trait VarConstraints {
    /// Create x <= y constraint
    fn le(self, other: Self) -> Constraint;
    /// Create x < y constraint  
    fn lt(self, other: Self) -> Constraint;
    /// Create x >= y constraint
    fn ge(self, other: Self) -> Constraint;
    /// Create x > y constraint
    fn gt(self, other: Self) -> Constraint;
    /// Create x == y constraint
    fn eq(self, other: Self) -> Constraint;
    /// Create x != y constraint
    fn ne(self, other: Self) -> Constraint;
    
    /// Create x <= value constraint
    fn le_val(self, value: Val) -> Constraint;
    /// Create x == value constraint  
    fn eq_val(self, value: Val) -> Constraint;
    
    // Syntactic sugar for common value types
    /// Create x == integer constraint
    fn eq_int(self, value: i32) -> Constraint;
    /// Create x == float constraint
    fn eq_float(self, value: f64) -> Constraint;
    /// Create x <= integer constraint
    fn le_int(self, value: i32) -> Constraint;
    /// Create x <= float constraint
    fn le_float(self, value: f64) -> Constraint;
    /// Create x >= integer constraint
    fn ge_int(self, value: i32) -> Constraint;
    /// Create x >= float constraint
    fn ge_float(self, value: f64) -> Constraint;
    /// Create x > integer constraint
    fn gt_int(self, value: i32) -> Constraint;
    /// Create x > float constraint
    fn gt_float(self, value: f64) -> Constraint;
    /// Create x < integer constraint
    fn lt_int(self, value: i32) -> Constraint;
    /// Create x < float constraint
    fn lt_float(self, value: f64) -> Constraint;
    
    // Convenience methods for common values to reduce visual complexity
    /// Create x == 0 constraint
    fn eq_zero(self) -> Constraint;
    /// Create x == 1 constraint
    fn eq_one(self) -> Constraint;
    /// Create x <= 0 constraint
    fn le_zero(self) -> Constraint;
    /// Create x >= 0 constraint (non-negative)
    fn ge_zero(self) -> Constraint;
    /// Create x > 0 constraint (positive)
    fn gt_zero(self) -> Constraint;
    /// Create x < 0 constraint (negative)
    fn lt_zero(self) -> Constraint;
    
    // Option B: Short boolean method names (backup to bitwise operators)
    /// Create x AND y constraint - clean boolean syntax
    fn and(self, other: Self) -> BooleanResult;
    /// Create x OR y constraint - clean boolean syntax
    fn or(self, other: Self) -> BooleanResult;
    /// Create NOT x constraint - clean boolean syntax
    fn not(self) -> BooleanResult;
    
    // Option C: Direct arithmetic methods for clean batch operations
    /// Create |x| operation - returns ArithmeticResult for batch operations
    fn abs(self) -> ArithmeticResult;
    /// Create x + y operation - returns ArithmeticResult for batch operations  
    fn add(self, other: Self) -> ArithmeticResult;
    /// Create x - y operation - returns ArithmeticResult for batch operations
    fn sub(self, other: Self) -> ArithmeticResult;
    /// Create x * y operation - returns ArithmeticResult for batch operations
    fn mul(self, other: Self) -> ArithmeticResult;
    /// Create x / y operation - returns ArithmeticResult for batch operations
    fn div(self, other: Self) -> ArithmeticResult;
    /// Create x % y operation - returns ArithmeticResult for batch operations
    fn modulo(self, other: Self) -> ArithmeticResult;
}

/// Result of an arithmetic operation that can be applied to a model
#[derive(Debug, Clone)]
pub struct ArithmeticResult {
    pub(crate) operation: ArithmeticOperation,
}

#[derive(Debug, Clone)]
pub(crate) enum ArithmeticOperation {
    Abs(VarId),
    Add(VarId, VarId),
    Sub(VarId, VarId),
    Mul(VarId, VarId),
    Div(VarId, VarId),
    Modulo(VarId, VarId),
}

impl ArithmeticResult {
    /// Apply this arithmetic operation to a model and return the result variable
    pub fn apply_to(self, model: &mut Model) -> VarId {
        match self.operation {
            ArithmeticOperation::Abs(a) => m.abs(a),
            ArithmeticOperation::Add(a, b) => m.add(a, b),
            ArithmeticOperation::Sub(a, b) => m.sub(a, b),
            ArithmeticOperation::Mul(a, b) => m.mul(a, b),
            ArithmeticOperation::Div(a, b) => m.div(a, b),
            ArithmeticOperation::Modulo(a, b) => m.modulo(a, b),
        }
    }
}

/// Result of a boolean operation that can be applied to a model
#[derive(Debug, Clone)]
pub struct BooleanResult {
    pub(crate) operation: BooleanOperation,
}

#[derive(Debug, Clone)]
pub(crate) enum BooleanOperation {
    And(VarId, VarId),
    Or(VarId, VarId),
    Not(VarId),
}

impl BooleanResult {
    /// Apply this boolean operation to a model and return the result variable
    pub fn apply_to(self, model: &mut Model) -> VarId {
        match self.operation {
            BooleanOperation::And(a, b) => m.bool_and(&[a, b]),
            BooleanOperation::Or(a, b) => m.bool_or(&[a, b]),
            BooleanOperation::Not(a) => m.bool_not(a),
        }
    }
}

impl VarConstraints for VarId {
    fn le(self, other: Self) -> Constraint {
        Constraint::Le(self, other)
    }
    
    fn lt(self, other: Self) -> Constraint {
        Constraint::Lt(self, other)
    }
    
    fn ge(self, other: Self) -> Constraint {
        Constraint::Ge(self, other)
    }
    
    fn gt(self, other: Self) -> Constraint {
        Constraint::Gt(self, other)
    }
    
    fn eq(self, other: Self) -> Constraint {
        Constraint::Eq(self, other)
    }
    
    fn ne(self, other: Self) -> Constraint {
        Constraint::Ne(self, other)
    }
    
    fn le_val(self, value: Val) -> Constraint {
        Constraint::LeVal(self, value)
    }
    
    fn eq_val(self, value: Val) -> Constraint {
        Constraint::EqVal(self, value)
    }
    
    // Syntactic sugar for common value types
    fn eq_int(self, value: i32) -> Constraint {
        Constraint::EqVal(self, Val::ValI(value))
    }
    
    fn eq_float(self, value: f64) -> Constraint {
        Constraint::EqVal(self, Val::ValF(value))
    }
    
    fn le_int(self, value: i32) -> Constraint {
        Constraint::LeVal(self, Val::ValI(value))
    }
    
    fn le_float(self, value: f64) -> Constraint {
        Constraint::LeVal(self, Val::ValF(value))
    }
    
    fn ge_int(self, value: i32) -> Constraint {
        Constraint::GeVal(self, Val::ValI(value))
    }
    
    fn ge_float(self, value: f64) -> Constraint {
        Constraint::GeVal(self, Val::ValF(value))
    }
    
    fn gt_int(self, value: i32) -> Constraint {
        Constraint::GtVal(self, Val::ValI(value))
    }
    
    fn gt_float(self, value: f64) -> Constraint {
        Constraint::GtVal(self, Val::ValF(value))
    }
    
    fn lt_int(self, value: i32) -> Constraint {
        Constraint::LtVal(self, Val::ValI(value))
    }
    
    fn lt_float(self, value: f64) -> Constraint {
        Constraint::LtVal(self, Val::ValF(value))
    }
    
    // Convenience methods for common values
    fn eq_zero(self) -> Constraint {
        Constraint::EqVal(self, Val::ValI(0))
    }
    
    fn eq_one(self) -> Constraint {
        Constraint::EqVal(self, Val::ValI(1))
    }
    
    fn le_zero(self) -> Constraint {
        Constraint::LeVal(self, Val::ValI(0))
    }
    
    fn ge_zero(self) -> Constraint {
        Constraint::GeVal(self, Val::ValI(0))
    }
    
    fn gt_zero(self) -> Constraint {
        Constraint::GtVal(self, Val::ValI(0))
    }
    
    fn lt_zero(self) -> Constraint {
        Constraint::LtVal(self, Val::ValI(0))
    }
    
    // Option B: Short boolean method implementations
    fn and(self, other: Self) -> BooleanResult {
        BooleanResult {
            operation: BooleanOperation::And(self, other),
        }
    }
    
    fn or(self, other: Self) -> BooleanResult {
        BooleanResult {
            operation: BooleanOperation::Or(self, other),
        }
    }
    
    fn not(self) -> BooleanResult {
        BooleanResult {
            operation: BooleanOperation::Not(self),
        }
    }
    
    // Option C: Direct arithmetic method implementations
    fn abs(self) -> ArithmeticResult {
        ArithmeticResult {
            operation: ArithmeticOperation::Abs(self),
        }
    }
    
    fn add(self, other: Self) -> ArithmeticResult {
        ArithmeticResult {
            operation: ArithmeticOperation::Add(self, other),
        }
    }
    
    fn sub(self, other: Self) -> ArithmeticResult {
        ArithmeticResult {
            operation: ArithmeticOperation::Sub(self, other),
        }
    }
    
    fn mul(self, other: Self) -> ArithmeticResult {
        ArithmeticResult {
            operation: ArithmeticOperation::Mul(self, other),
        }
    }
    
    fn div(self, other: Self) -> ArithmeticResult {
        ArithmeticResult {
            operation: ArithmeticOperation::Div(self, other),
        }
    }
    
    fn modulo(self, other: Self) -> ArithmeticResult {
        ArithmeticResult {
            operation: ArithmeticOperation::Modulo(self, other),
        }
    }
}

/// Input type for the post() method - can be single constraint or multiple
pub enum ConstraintInput {
    Single(Constraint),
    Multiple(Vec<Constraint>),
    Boolean(crate::boolean_operators::BoolExpr),
    Arithmetic(ArithmeticResult),
}

impl From<Constraint> for ConstraintInput {
    fn from(constraint: Constraint) -> Self {
        ConstraintInput::Single(constraint)
    }
}

impl From<Vec<Constraint>> for ConstraintInput {
    fn from(constraints: Vec<Constraint>) -> Self {
        ConstraintInput::Multiple(constraints)
    }
}

impl From<crate::boolean_operators::BoolExpr> for ConstraintInput {
    fn from(expr: crate::boolean_operators::BoolExpr) -> Self {
        ConstraintInput::Boolean(expr)
    }
}

impl From<ArithmeticResult> for ConstraintInput {
    fn from(result: ArithmeticResult) -> Self {
        ConstraintInput::Arithmetic(result)
    }
}

/// Extension trait for Model to easily add constraints.
pub trait ModelConstraints {
    /// Post constraint(s) to the model - works with single constraint or Vec<Constraint>
    fn post(&mut self, constraint: impl Into<ConstraintInput>);
    
    /// Create a new variable from an expression or apply a constraint
    /// This unified method can handle both expressions (returns VarId) and constraints (returns ())
    fn add(&mut self, input: impl Into<ConstraintInput>) -> Option<VarId>;
    
    /// Apply a boolean operation and return the result variable
    fn bool_result(&mut self, result: BooleanResult) -> VarId;
    
    /// Apply an arithmetic operation and return the result variable
    fn arith_result(&mut self, result: ArithmeticResult) -> VarId;
    
    /// Add a single constraint to the model (verbose form for backwards compatibility)
    fn add_constraint(&mut self, constraint: Constraint);
    
    /// Add multiple constraints to the model (verbose form for backwards compatibility)
    fn add_constraints(&mut self, constraints: Vec<Constraint>);
}

impl ModelConstraints for Model {
    fn post(&mut self, constraint: impl Into<ConstraintInput>) {
        match constraint.into() {
            ConstraintInput::Single(c) => c.apply_to(self),
            ConstraintInput::Multiple(constraints) => {
                for c in constraints {
                    c.apply_to(self);
                }
            }
            ConstraintInput::Boolean(expr) => {
                let _result = expr.apply_to(self);
                // Boolean expressions are applied but we don't need to return the result
                // since post() doesn't return anything
            }
            ConstraintInput::Arithmetic(result) => {
                let _result = result.apply_to(self);
                // Arithmetic results are applied but we don't need to return the result
                // since post() doesn't return anything  
            }
        }
    }
    
    fn bool_result(&mut self, result: BooleanResult) -> VarId {
        result.apply_to(self)
    }
    
    fn arith_result(&mut self, result: ArithmeticResult) -> VarId {
        result.apply_to(self)
    }
    
    fn add_constraint(&mut self, constraint: Constraint) {
        constraint.apply_to(self);
    }
    
    fn add_constraints(&mut self, constraints: Vec<Constraint>) {
        for constraint in constraints {
            constraint.apply_to(self);
        }
    }
}

/// Macro for creating constraints with natural syntax.
/// 
/// # Examples
/// 
/// ```rust
/// use cspsolver::prelude::*;
/// use cspsolver::constraint_builder::*;
/// 
/// let mut m = Model::default();
/// let x = m.int(0, 10);
/// let y = m.int(0, 10);
/// 
/// // Clean constraint creation with syntactic sugar
/// m.post(x.le(y));           // x <= y
/// m.post(x.eq_int(5));       // x == 5 (much cleaner!)
/// m.post(y.ge_float(3.14));  // y >= 3.14
/// 
/// // Or batch addition
/// m.post(vec![
///     x.le(y),
///     x.ne(y),
///     x.eq_int(5),               // Clean integer constraint
///     y.le_float(9.5),           // Clean float constraint
/// ]);
/// ```
#[macro_export]
macro_rules! constraints {
    ($($constraint:expr),* $(,)?) => {
        vec![$($constraint),*]
    };
}
