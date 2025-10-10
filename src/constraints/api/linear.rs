//! Linear constraint operations
//!
//! This module contains linear (weighted sum) constraints with a unified generic API:
//! - Generic linear: lin_eq, lin_le, lin_ne (works for both int and float)
//! - Generic linear reified: lin_eq_reif, lin_le_reif, lin_ne_reif (works for both int and float)
//! - Boolean linear: bool_lin_eq, bool_lin_le, bool_lin_ne
//! - Boolean linear reified: bool_lin_eq_reif, bool_lin_le_reif, bool_lin_ne_reif
//!
//! **Note:** The old type-specific methods (int_lin_eq, float_lin_eq, etc.) have been removed  
//! in favor of the new generic lin_eq() method, which automatically infers the type from
//! the coefficient/constant types.

use crate::model::Model;
use crate::variables::VarId;

impl Model {
    // ========== New Generic Linear Constraint API ==========
    // These methods provide a unified interface that works with both int and float
    // by using the LinearCoeff trait. They delegate to the standalone functions.

    /// Post a generic linear equality constraint: `sum(coeffs[i] * vars[i]) = constant`.
    /// 
    /// This works for both integer and float coefficients/constants.
    /// Type is inferred from the coefficient/constant types.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(0, 10);
    /// let y = m.int(0, 10);
    /// 
    /// // Integer: 2x + 3y = 10
    /// m.lin_eq(&[2, 3], &[x, y], 10);
    /// 
    /// // Float: 2.5x + 1.5y = 10.0
    /// let x = m.float(0.0, 10.0);
    /// let y = m.float(0.0, 10.0);
    /// m.lin_eq(&[2.5, 1.5], &[x, y], 10.0);
    /// ```
    pub fn lin_eq<T: crate::constraints::functions::LinearCoeff>(&mut self, coeffs: &[T], vars: &[VarId], constant: T) {
        crate::constraints::functions::lin_eq(self, coeffs, vars, constant);
    }

    /// Post a generic linear less-than-or-equal constraint: `sum(coeffs[i] * vars[i]) ≤ constant`.
    /// 
    /// This works for both integer and float coefficients/constants.
    /// Type is inferred from the coefficient/constant types.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(0, 10);
    /// let y = m.int(0, 10);
    /// 
    /// // Integer: x + y ≤ 15
    /// m.lin_le(&[1, 1], &[x, y], 15);
    /// 
    /// // Float: 1.5x + 2.0y ≤ 20.0
    /// let x = m.float(0.0, 10.0);
    /// let y = m.float(0.0, 10.0);
    /// m.lin_le(&[1.5, 2.0], &[x, y], 20.0);
    /// ```
    pub fn lin_le<T: crate::constraints::functions::LinearCoeff>(&mut self, coeffs: &[T], vars: &[VarId], constant: T) {
        crate::constraints::functions::lin_le(self, coeffs, vars, constant);
    }

    /// Post a generic linear not-equal constraint: `sum(coeffs[i] * vars[i]) ≠ constant`.
    /// 
    /// This works for both integer and float coefficients/constants.
    /// Type is inferred from the coefficient/constant types.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(0, 10);
    /// let y = m.int(0, 10);
    /// 
    /// // Integer: x + y ≠ 7
    /// m.lin_ne(&[1, 1], &[x, y], 7);
    /// ```
    pub fn lin_ne<T: crate::constraints::functions::LinearCoeff>(&mut self, coeffs: &[T], vars: &[VarId], constant: T) {
        crate::constraints::functions::lin_ne(self, coeffs, vars, constant);
    }

    /// Post a reified generic linear equality: `b ⇔ (sum(coeffs[i] * vars[i]) = constant)`.
    /// 
    /// Works for both integer and float types.
    pub fn lin_eq_reif<T: crate::constraints::functions::LinearCoeff>(&mut self, coeffs: &[T], vars: &[VarId], constant: T, b: VarId) {
        crate::constraints::functions::lin_eq_reif(self, coeffs, vars, constant, b);
    }

    /// Post a reified generic linear inequality: `b ⇔ (sum(coeffs[i] * vars[i]) ≤ constant)`.
    /// 
    /// Works for both integer and float types.
    pub fn lin_le_reif<T: crate::constraints::functions::LinearCoeff>(&mut self, coeffs: &[T], vars: &[VarId], constant: T, b: VarId) {
        crate::constraints::functions::lin_le_reif(self, coeffs, vars, constant, b);
    }

    /// Post a reified generic linear not-equal: `b ⇔ (sum(coeffs[i] * vars[i]) ≠ constant)`.
    /// 
    /// Works for both integer and float types.
    pub fn lin_ne_reif<T: crate::constraints::functions::LinearCoeff>(&mut self, coeffs: &[T], vars: &[VarId], constant: T, b: VarId) {
        crate::constraints::functions::lin_ne_reif(self, coeffs, vars, constant, b);
    }

    // ============================================================================
    // Boolean Linear Constraints
    // ============================================================================
    // Note: These are thin wrappers that call the generic lin_* methods.
    // They provide better API discoverability and document the intent that
    // variables should be boolean.

    /// Post a boolean linear equality constraint: `sum(coeffs[i] * bools[i]) = constant`.
    /// 
    /// This implements the MiniZinc/FlatZinc `bool_lin_eq` constraint, which represents
    /// a weighted sum of boolean variables equal to a constant value.
    /// 
    /// Boolean variables are represented as integers with domain {0, 1} where:
    /// - 0 = false
    /// - 1 = true
    /// 
    /// # Arguments
    /// * `coefficients` - Array of integer coefficients
    /// * `variables` - Array of boolean variables (must have same length as coefficients)
    /// * `constant` - The target sum value
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let b1 = m.bool();
    /// let b2 = m.bool();
    /// let b3 = m.bool();
    /// 
    /// // Exactly 2 out of 3 booleans must be true
    /// m.bool_lin_eq(&[1, 1, 1], &[b1, b2, b3], 2);
    /// 
    /// // Weighted sum with coefficients
    /// m.bool_lin_eq(&[2, 3, 1], &[b1, b2, b3], 5);
    /// ```
    pub fn bool_lin_eq(&mut self, coefficients: &[i32], variables: &[VarId], constant: i32) {
        // Delegate to generic lin_eq
        self.lin_eq(coefficients, variables, constant);
    }

    /// Post a boolean linear less-than-or-equal constraint: `sum(coeffs[i] * bools[i]) ≤ constant`.
    /// 
    /// This implements the MiniZinc/FlatZinc `bool_lin_le` constraint.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let b1 = m.bool();
    /// let b2 = m.bool();
    /// let b3 = m.bool();
    /// 
    /// // At most 2 out of 3 booleans can be true
    /// m.bool_lin_le(&[1, 1, 1], &[b1, b2, b3], 2);
    /// ```
    pub fn bool_lin_le(&mut self, coefficients: &[i32], variables: &[VarId], constant: i32) {
        self.lin_le(coefficients, variables, constant);
    }

    /// Post a boolean linear not-equal constraint: `sum(coeffs[i] * bools[i]) ≠ constant`.
    /// 
    /// This implements the MiniZinc/FlatZinc `bool_lin_ne` constraint.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let b1 = m.bool();
    /// let b2 = m.bool();
    /// let b3 = m.bool();
    /// 
    /// // Not exactly 2 true (can be 0, 1, or 3)
    /// m.bool_lin_ne(&[1, 1, 1], &[b1, b2, b3], 2);
    /// ```
    pub fn bool_lin_ne(&mut self, coefficients: &[i32], variables: &[VarId], constant: i32) {
        self.lin_ne(coefficients, variables, constant);
    }

    /// Post a reified boolean linear equality: `b ⇔ (sum(coeffs[i] * bools[i]) = constant)`.
    /// 
    /// This implements the MiniZinc/FlatZinc `bool_lin_eq_reif` constraint.
    /// The boolean variable `b` is 1 if and only if the linear equation holds.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let b1 = m.bool();
    /// let b2 = m.bool();
    /// let b3 = m.bool();
    /// let reif = m.bool();
    /// 
    /// // reif ⇔ (exactly 2 out of 3 are true)
    /// m.bool_lin_eq_reif(&[1, 1, 1], &[b1, b2, b3], 2, reif);
    /// ```
    pub fn bool_lin_eq_reif(&mut self, coefficients: &[i32], variables: &[VarId], constant: i32, reif_var: VarId) {
        self.lin_eq_reif(coefficients, variables, constant, reif_var);
    }

    /// Post a reified boolean linear inequality: `b ⇔ (sum(coeffs[i] * bools[i]) ≤ constant)`.
    /// 
    /// This implements the MiniZinc/FlatZinc `bool_lin_le_reif` constraint.
    /// The boolean variable `b` is 1 if and only if the linear inequality holds.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let b1 = m.bool();
    /// let b2 = m.bool();
    /// let b3 = m.bool();
    /// let reif = m.bool();
    /// 
    /// // reif ⇔ (at most 2 are true)
    /// m.bool_lin_le_reif(&[1, 1, 1], &[b1, b2, b3], 2, reif);
    /// ```
    pub fn bool_lin_le_reif(&mut self, coefficients: &[i32], variables: &[VarId], constant: i32, reif_var: VarId) {
        self.lin_le_reif(coefficients, variables, constant, reif_var);
    }

    /// Post a reified boolean linear not-equal: `b ⇔ (sum(coeffs[i] * bools[i]) ≠ constant)`.
    /// 
    /// This implements the MiniZinc/FlatZinc `bool_lin_ne_reif` constraint.
    /// The boolean variable `b` is 1 if and only if the linear inequality holds.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let b1 = m.bool();
    /// let b2 = m.bool();
    /// let b3 = m.bool();
    /// let reif = m.bool();
    /// 
    /// // reif ⇔ (NOT exactly 2 are true)
    /// m.bool_lin_ne_reif(&[1, 1, 1], &[b1, b2, b3], 2, reif);
    /// ```
    pub fn bool_lin_ne_reif(&mut self, coefficients: &[i32], variables: &[VarId], constant: i32, reif_var: VarId) {
        self.lin_ne_reif(coefficients, variables, constant, reif_var);
    }
}
