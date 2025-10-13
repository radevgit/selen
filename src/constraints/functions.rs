//! Generic constraint functions for the unified API.
//!
//! This module provides generic constraint functions that work with both
//! integer and float variables, replacing the old type-specific Model methods.
//!
//! Design: Arithmetic operations return ExprBuilder for composition,
//! while constraint functions post directly to the model.
//!
//! Phase 1: Leverage existing runtime API builders
//! Phase 2: Ensure all create AST internally (already done for arithmetic via ExprBuilder)

use crate::model::Model;
use crate::variables::{VarId, Val};
use crate::runtime_api::{ExprBuilder, ModelExt};

// ============================================================================
// Arithmetic Operations (return ExprBuilder for composition)
// ============================================================================

/// Create an addition expression: `x + y`.
///
/// Returns an ExprBuilder that can be further composed or constrained.
/// Works with both integer and float variables.
///
/// # Examples
/// ```
/// use selen::prelude::*;
/// 
/// let mut model = Model::default();
/// let x = model.int(0, 10);
/// let y = model.int(0, 10);
/// let z = model.int(0, 20);
/// 
/// // Post constraint: x + y = z
/// model.new(add(x, y).eq(z));
/// 
/// // Compose: (x + y) + z
/// let result = model.int(0, 30);
/// model.new(add(add(x, y), z).eq(result));
/// ```
pub fn add(x: impl Into<ExprBuilder>, y: impl Into<ExprBuilder>) -> ExprBuilder {
    ExprBuilder::Add(Box::new(x.into()), Box::new(y.into()))
}

/// Create a subtraction expression: `x - y`.
///
/// Returns an ExprBuilder that can be further composed or constrained.
/// Works with both integer and float variables.
///
/// # Examples
/// ```ignore
/// model.new(sub(x, y).eq(z));
/// ```
pub fn sub(x: impl Into<ExprBuilder>, y: impl Into<ExprBuilder>) -> ExprBuilder {
    ExprBuilder::Sub(Box::new(x.into()), Box::new(y.into()))
}

/// Create a multiplication expression: `x * y`.
///
/// Returns an ExprBuilder that can be further composed or constrained.
/// Works with both integer and float variables.
///
/// # Examples
/// ```ignore
/// model.new(mul(x, y).eq(z));
/// ```
pub fn mul(x: impl Into<ExprBuilder>, y: impl Into<ExprBuilder>) -> ExprBuilder {
    ExprBuilder::Mul(Box::new(x.into()), Box::new(y.into()))
}

/// Create a division expression: `x / y`.
///
/// Returns an ExprBuilder that can be further composed or constrained.
/// Works with both integer and float variables.
///
/// # Examples
/// ```ignore
/// model.new(div(x, y).eq(z));
/// ```
pub fn div(x: impl Into<ExprBuilder>, y: impl Into<ExprBuilder>) -> ExprBuilder {
    ExprBuilder::Div(Box::new(x.into()), Box::new(y.into()))
}

/// Post a modulo constraint: `result = x % y`.
///
/// NOTE: Modulo is not yet supported in the ExprBuilder, so this creates
/// a propagator directly via the old API. Will be updated in Phase 2.
///
/// Works with both integer and float variables.
///
/// # Examples
/// ```ignore
/// let result = modulo(&mut model, x, y);
/// ```
pub fn modulo(model: &mut Model, x: VarId, y: VarId) -> VarId {
    model.modulo(x, y)
}

// ============================================================================
// Comparison Constraints
// ============================================================================

/// Post an equality constraint: `x == y`.
///
/// Works with variables, constants, or expressions.
/// Use `int()` or `float()` for explicit constant types.
///
/// # Examples
/// ```ignore
/// eq(&mut model, x, y);           // x == y (two variables)
/// eq(&mut model, x, int(5));      // x == 5 (integer constant)
/// eq(&mut model, x, float(3.14)); // x == 3.14 (float constant)
/// eq(&mut model, add(x, y), z);   // x + y == z (expression)
/// ```
pub fn eq(model: &mut Model, x: impl Into<ExprBuilder>, y: impl Into<ExprBuilder>) {
    model.new(x.into().eq(y));
}

/// Post a not-equal constraint: `x != y`.
///
/// Works with variables, constants, or expressions.
/// Use `int()` or `float()` for explicit constant types.
///
/// # Examples
/// ```ignore
/// ne(&mut model, x, y);        // x != y
/// ne(&mut model, x, int(0));   // x != 0
/// ```
pub fn ne(model: &mut Model, x: impl Into<ExprBuilder>, y: impl Into<ExprBuilder>) {
    model.new(x.into().ne(y));
}

/// Post a less-than constraint: `x < y`.
///
/// Works with variables, constants, or expressions.
/// Use `int()` or `float()` for explicit constant types.
///
/// # Examples
/// ```ignore
/// lt(&mut model, x, y);         // x < y
/// lt(&mut model, x, int(10));   // x < 10
/// ```
pub fn lt(model: &mut Model, x: impl Into<ExprBuilder>, y: impl Into<ExprBuilder>) {
    model.new(x.into().lt(y));
}

/// Post a less-than-or-equal constraint: `x <= y`.
///
/// Works with variables, constants, or expressions.
/// Use `int()` or `float()` for explicit constant types.
///
/// # Examples
/// ```ignore
/// le(&mut model, x, y);          // x <= y
/// le(&mut model, x, int(100));   // x <= 100
/// ```
pub fn le(model: &mut Model, x: impl Into<ExprBuilder>, y: impl Into<ExprBuilder>) {
    model.new(x.into().le(y));
}

/// Post a greater-than constraint: `x > y`.
///
/// Works with variables, constants, or expressions.
/// Use `int()` or `float()` for explicit constant types.
///
/// # Examples
/// ```ignore
/// gt(&mut model, x, y);        // x > y
/// gt(&mut model, x, int(0));   // x > 0
/// ```
pub fn gt(model: &mut Model, x: impl Into<ExprBuilder>, y: impl Into<ExprBuilder>) {
    model.new(x.into().gt(y));
}

/// Post a greater-than-or-equal constraint: `x >= y`.
///
/// Works with variables, constants, or expressions.
/// Use `int()` or `float()` for explicit constant types.
///
/// # Examples
/// ```ignore
/// ge(&mut model, x, y);        // x >= y
/// ge(&mut model, x, int(5));   // x >= 5
/// ```
pub fn ge(model: &mut Model, x: impl Into<ExprBuilder>, y: impl Into<ExprBuilder>) {
    model.new(x.into().ge(y));
}

// ============================================================================
// Linear Constraints (weighted sums)
// ============================================================================

/// Post a linear equality constraint: `sum(coeffs[i] * vars[i]) == constant`.
///
/// Works with both integer and float coefficients via trait overloading.
///
/// # Examples
/// ```ignore
/// // Integer: 2*x + 3*y + z == 10
/// lin_eq(&mut model, &[2, 3, 1], &[x, y, z], 10);
/// 
/// // Float: 2.5*x + 3.7*y == 10.2
/// lin_eq(&mut model, &[2.5, 3.7], &[x, y], 10.2);
/// ```
pub fn lin_eq<T: LinearCoeff>(model: &mut Model, coeffs: &[T], vars: &[VarId], constant: T) {
    T::post_lin_eq(model, coeffs, vars, constant);
}

/// Post a linear less-than-or-equal constraint: `sum(coeffs[i] * vars[i]) <= constant`.
///
/// Works with both integer and float coefficients via trait overloading.
///
/// # Examples
/// ```ignore
/// // Integer: 2*x + 3*y <= 100
/// lin_le(&mut model, &[2, 3], &[x, y], 100);
/// 
/// // Float: 1.5*x + 2.5*y <= 50.0
/// lin_le(&mut model, &[1.5, 2.5], &[x, y], 50.0);
/// ```
pub fn lin_le<T: LinearCoeff>(model: &mut Model, coeffs: &[T], vars: &[VarId], constant: T) {
    T::post_lin_le(model, coeffs, vars, constant);
}

/// Post a linear not-equal constraint: `sum(coeffs[i] * vars[i]) != constant`.
///
/// Works with both integer and float coefficients via trait overloading.
///
/// # Examples
/// ```ignore
/// // Integer: 2*x + y != 5
/// lin_ne(&mut model, &[2, 1], &[x, y], 5);
/// ```
pub fn lin_ne<T: LinearCoeff>(model: &mut Model, coeffs: &[T], vars: &[VarId], constant: T) {
    T::post_lin_ne(model, coeffs, vars, constant);
}

/// Post a reified linear equality constraint: `b <=> sum(coeffs[i] * vars[i]) == constant`.
///
/// Works with both integer and float coefficients via trait overloading.
///
/// # Examples
/// ```ignore
/// let b = model.new_bool_var();
/// lin_eq_reif(&mut model, &[2, 3], &[x, y], 10, b);
/// ```
pub fn lin_eq_reif<T: LinearCoeff>(model: &mut Model, coeffs: &[T], vars: &[VarId], constant: T, b: VarId) {
    T::post_lin_eq_reif(model, coeffs, vars, constant, b);
}

/// Post a reified linear less-than-or-equal constraint: `b <=> sum(coeffs[i] * vars[i]) <= constant`.
///
/// Works with both integer and float coefficients via trait overloading.
///
/// # Examples
/// ```ignore
/// let b = model.new_bool_var();
/// lin_le_reif(&mut model, &[2, 3], &[x, y], 100, b);
/// ```
pub fn lin_le_reif<T: LinearCoeff>(model: &mut Model, coeffs: &[T], vars: &[VarId], constant: T, b: VarId) {
    T::post_lin_le_reif(model, coeffs, vars, constant, b);
}

/// Post a reified linear not-equal constraint: `b <=> sum(coeffs[i] * vars[i]) != constant`.
///
/// Works with both integer and float coefficients via trait overloading.
///
/// # Examples
/// ```ignore
/// let b = model.new_bool_var();
/// lin_ne_reif(&mut model, &[2, 3], &[x, y], 10, b);
/// ```
pub fn lin_ne_reif<T: LinearCoeff>(model: &mut Model, coeffs: &[T], vars: &[VarId], constant: T, b: VarId) {
    T::post_lin_ne_reif(model, coeffs, vars, constant, b);
}

/// Trait for types that can be used as linear coefficients (i32 or f64).
///
/// This enables generic `lin_eq`, `lin_le`, etc. functions to work with both integer and float coefficients.
pub trait LinearCoeff: Copy {
    fn post_lin_eq(model: &mut Model, coeffs: &[Self], vars: &[VarId], constant: Self);
    fn post_lin_le(model: &mut Model, coeffs: &[Self], vars: &[VarId], constant: Self);
    fn post_lin_ne(model: &mut Model, coeffs: &[Self], vars: &[VarId], constant: Self);
    fn post_lin_eq_reif(model: &mut Model, coeffs: &[Self], vars: &[VarId], constant: Self, b: VarId);
    fn post_lin_le_reif(model: &mut Model, coeffs: &[Self], vars: &[VarId], constant: Self, b: VarId);
    fn post_lin_ne_reif(model: &mut Model, coeffs: &[Self], vars: &[VarId], constant: Self, b: VarId);
}

impl LinearCoeff for i32 {
    fn post_lin_eq(model: &mut Model, coeffs: &[Self], vars: &[VarId], constant: Self) {
        // Validate array lengths before creating AST
        if coeffs.len() != vars.len() {
            model.constraint_validation_errors.push(crate::core::SolverError::InvalidConstraint {
                message: format!(
                    "Linear constraint validation error: coefficients and variables must have same length (got {} coefficients but {} variables)",
                    coeffs.len(),
                    vars.len()
                ),
                constraint_name: Some("lin_eq".to_string()),
                variables: None,
            });
            return;
        }
        
        // Phase 2: Create AST node instead of calling Model method directly
        use crate::runtime_api::{ConstraintKind, ComparisonOp};
        let ast = ConstraintKind::LinearInt {
            coeffs: coeffs.to_vec(),
            vars: vars.to_vec(),
            op: ComparisonOp::Eq,
            constant,
        };
        model.pending_constraint_asts.push(ast);
    }
    fn post_lin_le(model: &mut Model, coeffs: &[Self], vars: &[VarId], constant: Self) {
        // Validate array lengths before creating AST
        if coeffs.len() != vars.len() {
            model.constraint_validation_errors.push(crate::core::SolverError::InvalidConstraint {
                message: format!(
                    "Linear constraint validation error: coefficients and variables must have same length (got {} coefficients but {} variables)",
                    coeffs.len(),
                    vars.len()
                ),
                constraint_name: Some("lin_le".to_string()),
                variables: None,
            });
            return;
        }
        
        // Phase 2: Create AST node
        use crate::runtime_api::{ConstraintKind, ComparisonOp};
        let ast = ConstraintKind::LinearInt {
            coeffs: coeffs.to_vec(),
            vars: vars.to_vec(),
            op: ComparisonOp::Le,
            constant,
        };
        model.pending_constraint_asts.push(ast);
    }
    fn post_lin_ne(model: &mut Model, coeffs: &[Self], vars: &[VarId], constant: Self) {
        // Validate array lengths before creating AST
        if coeffs.len() != vars.len() {
            model.constraint_validation_errors.push(crate::core::SolverError::InvalidConstraint {
                message: format!(
                    "Linear constraint validation error: coefficients and variables must have same length (got {} coefficients but {} variables)",
                    coeffs.len(),
                    vars.len()
                ),
                constraint_name: Some("lin_ne".to_string()),
                variables: None,
            });
            return;
        }
        
        // Phase 2: Create AST node
        use crate::runtime_api::{ConstraintKind, ComparisonOp};
        let ast = ConstraintKind::LinearInt {
            coeffs: coeffs.to_vec(),
            vars: vars.to_vec(),
            op: ComparisonOp::Ne,
            constant,
        };
        model.pending_constraint_asts.push(ast);
    }
    fn post_lin_eq_reif(model: &mut Model, coeffs: &[Self], vars: &[VarId], constant: Self, b: VarId) {
        // Phase 2: Create AST node for reified constraint
        use crate::runtime_api::{ConstraintKind, ComparisonOp};
        let ast = ConstraintKind::ReifiedLinearInt {
            coeffs: coeffs.to_vec(),
            vars: vars.to_vec(),
            op: ComparisonOp::Eq,
            constant,
            reif_var: b,
        };
        model.pending_constraint_asts.push(ast);
    }
    fn post_lin_le_reif(model: &mut Model, coeffs: &[Self], vars: &[VarId], constant: Self, b: VarId) {
        // Phase 2: Create AST node for reified constraint
        use crate::runtime_api::{ConstraintKind, ComparisonOp};
        let ast = ConstraintKind::ReifiedLinearInt {
            coeffs: coeffs.to_vec(),
            vars: vars.to_vec(),
            op: ComparisonOp::Le,
            constant,
            reif_var: b,
        };
        model.pending_constraint_asts.push(ast);
    }
    fn post_lin_ne_reif(model: &mut Model, coeffs: &[Self], vars: &[VarId], constant: Self, b: VarId) {
        // Phase 2: Create AST node for reified constraint
        use crate::runtime_api::{ConstraintKind, ComparisonOp};
        let ast = ConstraintKind::ReifiedLinearInt {
            coeffs: coeffs.to_vec(),
            vars: vars.to_vec(),
            op: ComparisonOp::Ne,
            constant,
            reif_var: b,
        };
        model.pending_constraint_asts.push(ast);
    }
}

impl LinearCoeff for f64 {
    fn post_lin_eq(model: &mut Model, coeffs: &[Self], vars: &[VarId], constant: Self) {
        // Validate array lengths before creating AST
        if coeffs.len() != vars.len() {
            model.constraint_validation_errors.push(crate::core::SolverError::InvalidConstraint {
                message: format!(
                    "Linear constraint validation error: coefficients and variables must have same length (got {} coefficients but {} variables)",
                    coeffs.len(),
                    vars.len()
                ),
                constraint_name: Some("lin_eq".to_string()),
                variables: None,
            });
            return;
        }
        
        // Phase 2: Create AST node instead of calling Model method directly
        use crate::runtime_api::{ConstraintKind, ComparisonOp};
        let ast = ConstraintKind::LinearFloat {
            coeffs: coeffs.to_vec(),
            vars: vars.to_vec(),
            op: ComparisonOp::Eq,
            constant,
        };
        model.pending_constraint_asts.push(ast);
    }
    fn post_lin_le(model: &mut Model, coeffs: &[Self], vars: &[VarId], constant: Self) {
        // Validate array lengths before creating AST
        if coeffs.len() != vars.len() {
            model.constraint_validation_errors.push(crate::core::SolverError::InvalidConstraint {
                message: format!(
                    "Linear constraint validation error: coefficients and variables must have same length (got {} coefficients but {} variables)",
                    coeffs.len(),
                    vars.len()
                ),
                constraint_name: Some("lin_le".to_string()),
                variables: None,
            });
            return;
        }
        
        // Phase 2: Create AST node
        use crate::runtime_api::{ConstraintKind, ComparisonOp};
        let ast = ConstraintKind::LinearFloat {
            coeffs: coeffs.to_vec(),
            vars: vars.to_vec(),
            op: ComparisonOp::Le,
            constant,
        };
        model.pending_constraint_asts.push(ast);
    }
    fn post_lin_ne(model: &mut Model, coeffs: &[Self], vars: &[VarId], constant: Self) {
        // Validate array lengths before creating AST
        if coeffs.len() != vars.len() {
            model.constraint_validation_errors.push(crate::core::SolverError::InvalidConstraint {
                message: format!(
                    "Linear constraint validation error: coefficients and variables must have same length (got {} coefficients but {} variables)",
                    coeffs.len(),
                    vars.len()
                ),
                constraint_name: Some("lin_ne".to_string()),
                variables: None,
            });
            return;
        }
        
        // Phase 2: Create AST node
        use crate::runtime_api::{ConstraintKind, ComparisonOp};
        let ast = ConstraintKind::LinearFloat {
            coeffs: coeffs.to_vec(),
            vars: vars.to_vec(),
            op: ComparisonOp::Ne,
            constant,
        };
        model.pending_constraint_asts.push(ast);
    }
    fn post_lin_eq_reif(model: &mut Model, coeffs: &[Self], vars: &[VarId], constant: Self, b: VarId) {
        // Phase 2: Create AST node for reified constraint
        use crate::runtime_api::{ConstraintKind, ComparisonOp};
        let ast = ConstraintKind::ReifiedLinearFloat {
            coeffs: coeffs.to_vec(),
            vars: vars.to_vec(),
            op: ComparisonOp::Eq,
            constant,
            reif_var: b,
        };
        model.pending_constraint_asts.push(ast);
    }
    fn post_lin_le_reif(model: &mut Model, coeffs: &[Self], vars: &[VarId], constant: Self, b: VarId) {
        // Phase 2: Create AST node for reified constraint
        use crate::runtime_api::{ConstraintKind, ComparisonOp};
        let ast = ConstraintKind::ReifiedLinearFloat {
            coeffs: coeffs.to_vec(),
            vars: vars.to_vec(),
            op: ComparisonOp::Le,
            constant,
            reif_var: b,
        };
        model.pending_constraint_asts.push(ast);
    }
    fn post_lin_ne_reif(model: &mut Model, coeffs: &[Self], vars: &[VarId], constant: Self, b: VarId) {
        // Phase 2: Create AST node for reified constraint
        use crate::runtime_api::{ConstraintKind, ComparisonOp};
        let ast = ConstraintKind::ReifiedLinearFloat {
            coeffs: coeffs.to_vec(),
            vars: vars.to_vec(),
            op: ComparisonOp::Ne,
            constant,
            reif_var: b,
        };
        model.pending_constraint_asts.push(ast);
    }
}

// ============================================================================
// Basic Arithmetic Constraints
// ============================================================================

/// Post an all-different constraint: all variables must take different values.
///
/// # Examples
/// ```ignore
/// let vars = vec![x, y, z];
/// alldiff(&mut model, &vars);
/// ```
pub fn alldiff(model: &mut Model, vars: &[VarId]) {
    model.props.all_different(vars.to_vec());
}

/// Post an all-equal constraint: all variables must take the same value.
///
/// # Examples
/// ```ignore
/// let vars = vec![x, y, z];
/// alleq(&mut model, &vars);
/// ```
pub fn alleq(model: &mut Model, vars: &[VarId]) {
    model.props.all_equal(vars.to_vec());
}

/// Post a minimum constraint: `result = min(vars)`.
///
/// Works with both integer and float variables.
///
/// # Examples
/// ```ignore
/// let result = min(&mut model, &[x, y, z]).unwrap();
/// ```
pub fn min(model: &mut Model, vars: &[VarId]) -> crate::core::SolverResult<VarId> {
    model.min(vars)
}

/// Post a maximum constraint: `result = max(vars)`.
///
/// Works with both integer and float variables.
///
/// # Examples
/// ```ignore
/// let result = max(&mut model, &[x, y, z]).unwrap();
/// ```
pub fn max(model: &mut Model, vars: &[VarId]) -> crate::core::SolverResult<VarId> {
    model.max(vars)
}

/// Post a sum constraint: `result = sum(vars)`.
///
/// Works with both integer and float variables.
///
/// # Examples
/// ```ignore
/// let result = sum(&mut model, &[x, y, z]);
/// ```
pub fn sum(model: &mut Model, vars: &[VarId]) -> VarId {
    model.sum(vars)
}

/// Post an absolute value constraint: `result = abs(var)`.
///
/// Works with both integer and float variables.
///
/// # Examples
/// ```ignore
/// let result = abs(&mut model, x);
/// ```
pub fn abs(model: &mut Model, var: VarId) -> VarId {
    model.abs(var)
}

// ============================================================================
// Reified Constraints
// ============================================================================

/// Post a reified equality constraint: `b ⇔ (x == y)`.
///
/// The boolean variable `b` is true if and only if `x` equals `y`.
/// Works with both integer and float variables.
///
/// # Examples
/// ```ignore
/// let b = model.new_bool_var();
/// eq_reif(&mut model, x, y, b);
/// ```
pub fn eq_reif(model: &mut Model, x: VarId, y: VarId, b: VarId) {
    model.props.int_eq_reif(x, y, b);
}

/// Post a reified not-equal constraint: `b ⇔ (x != y)`.
///
/// The boolean variable `b` is true if and only if `x` does not equal `y`.
/// Works with both integer and float variables.
///
/// # Examples
/// ```ignore
/// let b = model.new_bool_var();
/// ne_reif(&mut model, x, y, b);
/// ```
pub fn ne_reif(model: &mut Model, x: VarId, y: VarId, b: VarId) {
    model.props.int_ne_reif(x, y, b);
}

/// Post a reified less-than constraint: `b ⇔ (x < y)`.
///
/// The boolean variable `b` is true if and only if `x` is less than `y`.
/// Works with both integer and float variables.
///
/// # Examples
/// ```ignore
/// let b = model.new_bool_var();
/// lt_reif(&mut model, x, y, b);
/// ```
pub fn lt_reif(model: &mut Model, x: VarId, y: VarId, b: VarId) {
    model.props.int_lt_reif(x, y, b);
}

/// Post a reified less-than-or-equal constraint: `b ⇔ (x <= y)`.
///
/// The boolean variable `b` is true if and only if `x` is less than or equal to `y`.
/// Works with both integer and float variables.
///
/// # Examples
/// ```ignore
/// let b = model.new_bool_var();
/// le_reif(&mut model, x, y, b);
/// ```
pub fn le_reif(model: &mut Model, x: VarId, y: VarId, b: VarId) {
    model.props.int_le_reif(x, y, b);
}

/// Post a reified greater-than constraint: `b ⇔ (x > y)`.
///
/// The boolean variable `b` is true if and only if `x` is greater than `y`.
/// Works with both integer and float variables.
///
/// # Examples
/// ```ignore
/// let b = model.new_bool_var();
/// gt_reif(&mut model, x, y, b);
/// ```
pub fn gt_reif(model: &mut Model, x: VarId, y: VarId, b: VarId) {
    // gt_reif is the same as lt_reif with arguments swapped
    model.props.int_lt_reif(y, x, b);
}

/// Post a reified greater-than-or-equal constraint: `b ⇔ (x >= y)`.
///
/// The boolean variable `b` is true if and only if `x` is greater than or equal to `y`.
/// Works with both integer and float variables.
///
/// # Examples
/// ```ignore
/// let b = model.new_bool_var();
/// ge_reif(&mut model, x, y, b);
/// ```
pub fn ge_reif(model: &mut Model, x: VarId, y: VarId, b: VarId) {
    // ge_reif is the same as le_reif with arguments swapped
    model.props.int_le_reif(y, x, b);
}

// ============================================================================
// Logical Constraints
// ============================================================================

/// Post a logical AND constraint: `result ⇔ (b1 ∧ b2)`.
///
/// # Examples
/// ```ignore
/// let result = and(&mut model, b1, b2);
/// ```
pub fn and(model: &mut Model, b1: VarId, b2: VarId) -> VarId {
    model.bool_and(&[b1, b2])
}

/// Post a logical OR constraint: `result ⇔ (b1 ∨ b2)`.
///
/// # Examples
/// ```ignore
/// let result = or(&mut model, b1, b2);
/// ```
pub fn or(model: &mut Model, b1: VarId, b2: VarId) -> VarId {
    model.bool_or(&[b1, b2])
}

/// Post a logical NOT constraint: `result ⇔ ¬b`.
///
/// # Examples
/// ```ignore
/// let result = not(&mut model, b);
/// ```
pub fn not(model: &mut Model, b: VarId) -> VarId {
    model.bool_not(b)
}

/// Post a logical XOR constraint: `result ⇔ (b1 ⊕ b2)`.
///
/// # Examples
/// ```ignore
/// let result = xor(&mut model, b1, b2);
/// ```
pub fn xor(model: &mut Model, b1: VarId, b2: VarId) -> VarId {
    // XOR can be implemented as (b1 OR b2) AND NOT(b1 AND b2)
    let b1_or_b2 = model.bool_or(&[b1, b2]);
    let b1_and_b2 = model.bool_and(&[b1, b2]);
    let not_both = model.bool_not(b1_and_b2);
    model.bool_and(&[b1_or_b2, not_both])
}

/// Post a logical implication constraint: `b1 → b2` (if b1 then b2).
///
/// Equivalent to: NOT b1 OR b2
///
/// # Examples
/// ```ignore
/// implies(&mut model, b1, b2);
/// ```
pub fn implies(model: &mut Model, b1: VarId, b2: VarId) {
    // b1 => b2 is equivalent to !b1 OR b2
    let not_b1 = model.bool_not(b1);
    let _ = model.bool_or(&[not_b1, b2]);
}

// ============================================================================
// Array/Element Constraints
// ============================================================================

/// Post an element constraint: `result = array[index]`.
///
/// The `index` is 0-based. Works with both integer and float arrays.
///
/// # Examples
/// ```ignore
/// let result = element(&mut model, index, &array);
/// ```
pub fn element(model: &mut Model, array: &[VarId], index: VarId) -> VarId {
    let result = model.int(-1000, 1000); // TODO: compute proper domain
    model.element(array, index, result);
    result
}

// ============================================================================
// Table Constraints
// ============================================================================

/// Post a table constraint: the tuple of variables must match one of the allowed tuples.
///
/// **Note:** This function is a placeholder in Phase 1. Will be properly implemented in Phase 2.
///
/// # Examples
/// ```ignore
/// let allowed = vec![
///     vec![Val::I(1), Val::I(2)],
///     vec![Val::I(2), Val::I(3)],
/// ];
/// table(&mut model, &[x, y], &allowed);
/// ```
pub fn table(_model: &mut Model, _vars: &[VarId], _tuples: &[Vec<Val>]) {
    todo!("table constraint will be implemented in Phase 2")
}

// ============================================================================
// Global Cardinality Constraint
// ============================================================================

/// Post a global cardinality constraint (GCC).
///
/// **Note:** This function is a placeholder in Phase 1. Will be properly implemented in Phase 2.
///
/// For each value `v` in the domain, the number of variables assigned to `v`
/// must be within the specified bounds.
///
/// # Examples
/// ```ignore
/// gcc(&mut model, &vars, &values, &lower_bounds, &upper_bounds);
/// ```
pub fn gcc(
    _model: &mut Model,
    _vars: &[VarId],
    _values: &[Val],
    _lower_bounds: &[i32],
    _upper_bounds: &[i32],
) {
    todo!("gcc constraint will be implemented in Phase 2")
}

// ============================================================================
// Cumulative Constraint
// ============================================================================

/// Post a cumulative constraint for resource scheduling.
///
/// **Note:** This function is a placeholder in Phase 1. Will be properly implemented in Phase 2.
///
/// Ensures that at any point in time, the sum of resource demands of overlapping
/// tasks does not exceed the resource capacity.
///
/// # Examples
/// ```ignore
/// cumulative(&mut model, &starts, &durations, &demands, capacity);
/// ```
pub fn cumulative(
    _model: &mut Model,
    _starts: &[VarId],
    _durations: &[i32],
    _demands: &[i32],
    _capacity: i32,
) {
    todo!("cumulative constraint will be implemented in Phase 2")
}

// ============================================================================
// Type Conversion Constraints
// ============================================================================

/// Convert an integer variable to a float variable.
///
/// **Note:** This function is a placeholder in Phase 1. Will be properly implemented in Phase 2.
///
/// # Examples
/// ```ignore
/// let float_var = to_float(&mut model, int_var);
/// ```
pub fn to_float(_model: &mut Model, _int_var: VarId) -> VarId {
    todo!("to_float will be implemented in Phase 2")
}

/// Post a floor constraint: `result = floor(float_var)`.
///
/// **Note:** This function is a placeholder in Phase 1. Will be properly implemented in Phase 2.
///
/// # Examples
/// ```ignore
/// let result = floor(&mut model, float_var);
/// ```
pub fn floor(_model: &mut Model, _float_var: VarId) -> VarId {
    todo!("floor will be implemented in Phase 2")
}

/// Post a ceiling constraint: `result = ceil(float_var)`.
///
/// **Note:** This function is a placeholder in Phase 1. Will be properly implemented in Phase 2.
///
/// # Examples
/// ```ignore
/// let result = ceil(&mut model, float_var);
/// ```
pub fn ceil(_model: &mut Model, _float_var: VarId) -> VarId {
    todo!("ceil will be implemented in Phase 2")
}

/// Post a rounding constraint: `result = round(float_var)`.
///
/// **Note:** This function is a placeholder in Phase 1. Will be properly implemented in Phase 2.
///
/// # Examples
/// ```ignore
/// let result = round(&mut model, float_var);
/// ```
pub fn round(_model: &mut Model, _float_var: VarId) -> VarId {
    todo!("round will be implemented in Phase 2")
}
