//! Linear constraint operations
//!
//! This module contains linear (weighted sum) constraints for booleans, integers and floats:
//! - Boolean linear: bool_lin_eq, bool_lin_le, bool_lin_ne
//! - Boolean linear reified: bool_lin_eq_reif, bool_lin_le_reif, bool_lin_ne_reif
//! - Integer linear: int_lin_eq, int_lin_le, int_lin_ne
//! - Integer linear reified: int_lin_eq_reif, int_lin_le_reif, int_lin_ne_reif
//! - Float linear: float_lin_eq, float_lin_le, float_lin_ne
//! - Float linear reified: float_lin_eq_reif, float_lin_le_reif, float_lin_ne_reif

use crate::model::Model;
use crate::variables::{VarId, Val};
use crate::variables::views::ViewRaw;

impl Model {
    pub fn int_lin_eq(&mut self, coefficients: &[i32], variables: &[VarId], constant: i32) {
        // Handle mismatched lengths - will be detected as unsatisfiable during solving
        if coefficients.len() != variables.len() {
            // Create an unsatisfiable constraint: 0 = 1
            self.props.equals(Val::ValI(0), Val::ValI(1));
            return;
        }

        if variables.is_empty() {
            // Empty sum = constant
            // This is satisfiable only if constant == 0, otherwise unsatisfiable
            self.props.equals(Val::ValI(0), Val::ValI(constant));
            return;
        }

        // Use dedicated propagator for linear equality
        self.props.int_lin_eq(coefficients.to_vec(), variables.to_vec(), constant);
    }

    /// Post a linear less-than-or-equal constraint: `sum(coeffs[i] * vars[i]) ≤ constant`.
    /// 
    /// This implements the FlatZinc `int_lin_le` constraint, which represents
    /// a weighted sum of variables less than or equal to a constant value.
    /// 
    /// # Arguments
    /// * `coefficients` - Array of integer coefficients
    /// * `variables` - Array of variables (must have same length as coefficients)
    /// * `constant` - The upper bound for the weighted sum
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(0, 10);
    /// let y = m.int(0, 10);
    /// let z = m.int(0, 10);
    /// 
    /// // x + y + z ≤ 20
    /// m.int_lin_le(&[1, 1, 1], &[x, y, z], 20);
    /// ```
    /// 
    pub fn int_lin_le(&mut self, coefficients: &[i32], variables: &[VarId], constant: i32) {
        // Handle mismatched lengths - will be detected as unsatisfiable during solving
        if coefficients.len() != variables.len() {
            // Create an unsatisfiable constraint: 0 = 1
            self.props.equals(Val::ValI(0), Val::ValI(1));
            return;
        }

        if variables.is_empty() {
            // Empty sum ≤ constant
            // This is satisfiable only if 0 ≤ constant, otherwise unsatisfiable
            self.props.less_than_or_equals(Val::ValI(0), Val::ValI(constant));
            return;
        }

        // Use dedicated propagator for linear inequality
        self.props.int_lin_le(coefficients.to_vec(), variables.to_vec(), constant);
    }

    /// Post a linear not-equal constraint: `sum(coeffs[i] * vars[i]) ≠ constant`.
    /// 
    /// This implements the FlatZinc `int_lin_ne` constraint, which represents
    /// a weighted sum of variables not equal to a constant value.
    /// 
    /// # Arguments
    /// * `coefficients` - Array of integer coefficients
    /// * `variables` - Array of variables (must have same length as coefficients)
    /// * `constant` - The value that the weighted sum must not equal
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(0, 10);
    /// let y = m.int(0, 10);
    /// let z = m.int(0, 10);
    /// 
    /// // x + y + z ≠ 15
    /// m.int_lin_ne(&[1, 1, 1], &[x, y, z], 15);
    /// ```
    /// 
    pub fn int_lin_ne(&mut self, coefficients: &[i32], variables: &[VarId], constant: i32) {
        // Handle mismatched lengths - will be detected as unsatisfiable during solving
        if coefficients.len() != variables.len() {
            // Create an unsatisfiable constraint: 0 = 1
            self.props.equals(Val::ValI(0), Val::ValI(1));
            return;
        }

        if variables.is_empty() {
            // Empty sum ≠ constant
            // This is satisfiable only if 0 ≠ constant, otherwise unsatisfiable
            self.props.not_equals(Val::ValI(0), Val::ValI(constant));
            return;
        }

        // Use dedicated propagator for linear not-equal
        self.props.int_lin_ne(coefficients.to_vec(), variables.to_vec(), constant);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // 🔢 Integer Linear Reified Constraints (FlatZinc Integration)
    // ═══════════════════════════════════════════════════════════════════════

    /// Post a reified integer linear equality: `b ⇔ (sum(coeffs[i] * vars[i]) = constant)`.
    /// 
    /// This implements the FlatZinc `int_lin_eq_reif` constraint.
    /// The boolean variable `b` is 1 if and only if the linear equation holds.
    /// 
    /// # Arguments
    /// * `coefficients` - Array of integer coefficients
    /// * `variables` - Array of variables (must have same length as coefficients)
    /// * `constant` - The constant value
    /// * `reif_var` - Boolean variable for reification (0 or 1)
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(0, 10);
    /// let y = m.int(0, 10);
    /// let b = m.bool();
    /// 
    /// // b ⇔ (x + y = 7)
    /// m.int_lin_eq_reif(&[1, 1], &[x, y], 7, b);
    /// ```
    pub fn int_lin_eq_reif(&mut self, coefficients: &[i32], variables: &[VarId], constant: i32, reif_var: VarId) {
        if coefficients.len() != variables.len() {
            self.props.equals(reif_var, Val::ValI(0));
            return;
        }
        if variables.is_empty() {
            if constant == 0 {
                self.props.equals(reif_var, Val::ValI(1));
            } else {
                self.props.equals(reif_var, Val::ValI(0));
            }
            return;
        }
        // Use dedicated propagator for reified linear equality
        self.props.int_lin_eq_reif(coefficients.to_vec(), variables.to_vec(), constant, reif_var);
    }

    /// Post a reified integer linear less-than-or-equal: `b ⇔ (sum(coeffs[i] * vars[i]) ≤ constant)`.
    /// 
    /// This implements the FlatZinc `int_lin_le_reif` constraint.
    /// The boolean variable `b` is 1 if and only if the linear inequality holds.
    /// 
    /// # Arguments
    /// * `coefficients` - Array of integer coefficients
    /// * `variables` - Array of variables (must have same length as coefficients)
    /// * `constant` - The constant value
    /// * `reif_var` - Boolean variable for reification (0 or 1)
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(0, 10);
    /// let y = m.int(0, 10);
    /// let b = m.bool();
    /// 
    /// // b ⇔ (x + y ≤ 20)
    /// m.int_lin_le_reif(&[1, 1], &[x, y], 20, b);
    /// ```
    pub fn int_lin_le_reif(&mut self, coefficients: &[i32], variables: &[VarId], constant: i32, reif_var: VarId) {
        if coefficients.len() != variables.len() {
            self.props.equals(reif_var, Val::ValI(0));
            return;
        }
        if variables.is_empty() {
            if constant >= 0 {
                self.props.equals(reif_var, Val::ValI(1));
            } else {
                self.props.equals(reif_var, Val::ValI(0));
            }
            return;
        }
        // Use dedicated propagator for reified linear inequality
        self.props.int_lin_le_reif(coefficients.to_vec(), variables.to_vec(), constant, reif_var);
    }

    /// Post a reified integer linear not-equal: `b ⇔ (sum(coeffs[i] * vars[i]) ≠ constant)`.
    /// 
    /// This implements the FlatZinc `int_lin_ne_reif` constraint.
    /// The boolean variable `b` is 1 if and only if the linear inequality holds.
    /// 
    /// # Arguments
    /// * `coefficients` - Array of integer coefficients
    /// * `variables` - Array of variables (must have same length as coefficients)
    /// * `constant` - The constant value
    /// * `reif_var` - Boolean variable for reification (0 or 1)
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(0, 10);
    /// let y = m.int(0, 10);
    /// let b = m.bool();
    /// 
    /// // b ⇔ (x + y ≠ 5)
    /// m.int_lin_ne_reif(&[1, 1], &[x, y], 5, b);
    /// ```
    pub fn int_lin_ne_reif(&mut self, coefficients: &[i32], variables: &[VarId], constant: i32, reif_var: VarId) {
        if coefficients.len() != variables.len() {
            self.props.equals(reif_var, Val::ValI(0));
            return;
        }
        if variables.is_empty() {
            if constant != 0 {
                self.props.equals(reif_var, Val::ValI(1));
            } else {
                self.props.equals(reif_var, Val::ValI(0));
            }
            return;
        }
        // Use dedicated propagator for reified linear not-equal
        self.props.int_lin_ne_reif(coefficients.to_vec(), variables.to_vec(), constant, reif_var);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // � Float Linear Constraints (FlatZinc Integration)
    // ═══════════════════════════════════════════════════════════════════════

    /// Post a float linear equality constraint: `sum(coeffs[i] * vars[i]) = constant`.
    /// 
    /// This implements the FlatZinc `float_lin_eq` constraint.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.float(0.0, 10.0);
    /// let y = m.float(0.0, 10.0);
    /// 
    /// // 2.5*x + 3.7*y = 18.5
    /// m.float_lin_eq(&[2.5, 3.7], &[x, y], 18.5);
    /// ```
    pub fn float_lin_eq(&mut self, coefficients: &[f64], variables: &[VarId], constant: f64) {
        if coefficients.len() != variables.len() {
            self.props.equals(Val::ValF(0.0), Val::ValF(1.0));
            return;
        }
        if variables.is_empty() {
            self.props.equals(Val::ValF(0.0), Val::ValF(constant));
            return;
        }
        // Use dedicated propagator for float linear equality
        self.props.float_lin_eq(coefficients.to_vec(), variables.to_vec(), constant);
    }

    /// Post a float linear less-than-or-equal constraint: `sum(coeffs[i] * vars[i]) ≤ constant`.
    /// 
    /// This implements the FlatZinc `float_lin_le` constraint.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.float(0.0, 10.0);
    /// let y = m.float(0.0, 10.0);
    /// 
    /// // x + y ≤ 20.5
    /// m.float_lin_le(&[1.0, 1.0], &[x, y], 20.5);
    /// ```
    pub fn float_lin_le(&mut self, coefficients: &[f64], variables: &[VarId], constant: f64) {
        if coefficients.len() != variables.len() {
            self.props.equals(Val::ValF(0.0), Val::ValF(1.0));
            return;
        }
        if variables.is_empty() {
            self.props.less_than_or_equals(Val::ValF(0.0), Val::ValF(constant));
            return;
        }
        // Use dedicated propagator for float linear inequality
        self.props.float_lin_le(coefficients.to_vec(), variables.to_vec(), constant);
    }

    /// Post a float linear not-equal constraint: `sum(coeffs[i] * vars[i]) ≠ constant`.
    /// 
    /// This implements the FlatZinc `float_lin_ne` constraint.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.float(0.0, 10.0);
    /// let y = m.float(0.0, 10.0);
    /// 
    /// // x + y ≠ 5.0
    /// m.float_lin_ne(&[1.0, 1.0], &[x, y], 5.0);
    /// ```
    pub fn float_lin_ne(&mut self, coefficients: &[f64], variables: &[VarId], constant: f64) {
        if coefficients.len() != variables.len() {
            self.props.equals(Val::ValF(0.0), Val::ValF(1.0));
            return;
        }
        if variables.is_empty() {
            self.props.not_equals(Val::ValF(0.0), Val::ValF(constant));
            return;
        }
        // Use dedicated propagator for float linear not-equal
        self.props.float_lin_ne(coefficients.to_vec(), variables.to_vec(), constant);
    }

    /// Post a reified float linear equality: `b ⇔ (sum(coeffs[i] * vars[i]) = constant)`.
    /// 
    /// This implements the FlatZinc `float_lin_eq_reif` constraint.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.float(0.0, 10.0);
    /// let y = m.float(0.0, 10.0);
    /// let b = m.bool();
    /// 
    /// // b ⇔ (x + y = 7.5)
    /// m.float_lin_eq_reif(&[1.0, 1.0], &[x, y], 7.5, b);
    /// ```
    pub fn float_lin_eq_reif(&mut self, coefficients: &[f64], variables: &[VarId], constant: f64, reif_var: VarId) {
        if coefficients.len() != variables.len() {
            self.props.equals(reif_var, Val::ValI(0));
            return;
        }
        if variables.is_empty() {
            if constant.abs() < 1e-10 {
                self.props.equals(reif_var, Val::ValI(1));
            } else {
                self.props.equals(reif_var, Val::ValI(0));
            }
            return;
        }
        // Use dedicated propagator for reified float linear equality
        self.props.float_lin_eq_reif(coefficients.to_vec(), variables.to_vec(), constant, reif_var);
    }

    /// Post a reified float linear less-than-or-equal: `b ⇔ (sum(coeffs[i] * vars[i]) ≤ constant)`.
    /// 
    /// This implements the FlatZinc `float_lin_le_reif` constraint.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.float(0.0, 10.0);
    /// let y = m.float(0.0, 10.0);
    /// let b = m.bool();
    /// 
    /// // b ⇔ (x + y ≤ 20.5)
    /// m.float_lin_le_reif(&[1.0, 1.0], &[x, y], 20.5, b);
    /// ```
    pub fn float_lin_le_reif(&mut self, coefficients: &[f64], variables: &[VarId], constant: f64, reif_var: VarId) {
        if coefficients.len() != variables.len() {
            self.props.equals(reif_var, Val::ValI(0));
            return;
        }
        if variables.is_empty() {
            if constant >= -1e-10 {
                self.props.equals(reif_var, Val::ValI(1));
            } else {
                self.props.equals(reif_var, Val::ValI(0));
            }
            return;
        }
        // Use dedicated propagator for reified float linear inequality
        self.props.float_lin_le_reif(coefficients.to_vec(), variables.to_vec(), constant, reif_var);
    }

    /// Post a reified float linear not-equal: `b ⇔ (sum(coeffs[i] * vars[i]) ≠ constant)`.
    /// 
    /// This implements the FlatZinc `float_lin_ne_reif` constraint.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.float(0.0, 10.0);
    /// let y = m.float(0.0, 10.0);
    /// let b = m.bool();
    /// 
    /// // b ⇔ (x + y ≠ 5.0)
    /// m.float_lin_ne_reif(&[1.0, 1.0], &[x, y], 5.0, b);
    /// ```
    pub fn float_lin_ne_reif(&mut self, coefficients: &[f64], variables: &[VarId], constant: f64, reif_var: VarId) {
        if coefficients.len() != variables.len() {
            self.props.equals(reif_var, Val::ValI(0));
            return;
        }
        if variables.is_empty() {
            if constant.abs() > 1e-10 {
                self.props.equals(reif_var, Val::ValI(1));
            } else {
                self.props.equals(reif_var, Val::ValI(0));
            }
            return;
        }
        // Use dedicated propagator for reified float linear not-equal
        self.props.float_lin_ne_reif(coefficients.to_vec(), variables.to_vec(), constant, reif_var);
    }

    // ============================================================================
    // Boolean Linear Constraints
    // ============================================================================
    // Note: These are thin wrappers over int_lin_* constraints since boolean
    // variables are represented as integers with domain {0, 1}. The same
    // propagators work for both. These methods provide better API discoverability
    // and document the intent that variables should be boolean.

    /// Post a boolean linear equality constraint: `sum(coeffs[i] * bools[i]) = constant`.
    /// 
    /// This implements the MiniZinc/FlatZinc `bool_lin_eq` constraint, which represents
    /// a weighted sum of boolean variables equal to a constant value.
    /// 
    /// Boolean variables are represented as integers with domain {0, 1} where:
    /// - 0 = false
    /// - 1 = true
    /// 
    /// This method is a wrapper over `int_lin_eq` but documents that variables should be boolean.
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
        // Delegate to integer linear constraint - same propagator works for booleans
        // Boolean variables are represented as integers with domain {0, 1}
        self.int_lin_eq(coefficients, variables, constant);
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
        self.int_lin_le(coefficients, variables, constant);
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
        self.int_lin_ne(coefficients, variables, constant);
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
        self.int_lin_eq_reif(coefficients, variables, constant, reif_var);
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
        self.int_lin_le_reif(coefficients, variables, constant, reif_var);
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
        self.int_lin_ne_reif(coefficients, variables, constant, reif_var);
    }

}
