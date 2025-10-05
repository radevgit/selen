//! Reified comparison constraints
//!
//! This module contains reified constraints where a boolean variable
//! represents whether a comparison holds:
//! - Integer comparisons: int_eq_reif, int_ne_reif, int_lt_reif, int_le_reif, int_gt_reif, int_ge_reif
//! - Float comparisons: float_eq_reif, float_ne_reif, float_lt_reif, float_le_reif, float_gt_reif, float_ge_reif

use crate::model::Model;
use crate::variables::VarId;

impl Model {
    /// Post a reified equality constraint: `b ⇔ (x = y)`.
    /// 
    /// The boolean variable `b` is 1 if and only if `x = y`.
    /// This is useful for FlatZinc integration and conditional constraints.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(1, 10);
    /// let y = m.int(1, 10);
    /// let b = m.bool();
    /// m.int_eq_reif(x, y, b);
    /// // Now b is 1 iff x = y
    /// ```
    pub fn int_eq_reif(&mut self, x: VarId, y: VarId, b: VarId) {
        self.props.int_eq_reif(x, y, b);
    }

    /// Post a reified inequality constraint: `b ⇔ (x ≠ y)`.
    /// 
    /// The boolean variable `b` is 1 if and only if `x ≠ y`.
    /// This is useful for FlatZinc integration and conditional constraints.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(1, 10);
    /// let y = m.int(1, 10);
    /// let b = m.bool();
    /// m.int_ne_reif(x, y, b);
    /// // Now b is 1 iff x ≠ y
    /// ```
    pub fn int_ne_reif(&mut self, x: VarId, y: VarId, b: VarId) {
        self.props.int_ne_reif(x, y, b);
    }

    /// Post a reified less-than constraint: `b ⇔ (x < y)`.
    /// 
    /// The boolean variable `b` is 1 if and only if `x < y`.
    /// This is useful for FlatZinc integration and conditional constraints.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(1, 10);
    /// let y = m.int(1, 10);
    /// let b = m.bool();
    /// m.int_lt_reif(x, y, b);
    /// // Now b is 1 iff x < y
    /// ```
    pub fn int_lt_reif(&mut self, x: VarId, y: VarId, b: VarId) {
        self.props.int_lt_reif(x, y, b);
    }

    /// Post a reified less-than-or-equal constraint: `b ⇔ (x ≤ y)`.
    /// 
    /// The boolean variable `b` is 1 if and only if `x ≤ y`.
    /// This is useful for FlatZinc integration and conditional constraints.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(1, 10);
    /// let y = m.int(1, 10);
    /// let b = m.bool();
    /// m.int_le_reif(x, y, b);
    /// // Now b is 1 iff x ≤ y
    /// ```
    pub fn int_le_reif(&mut self, x: VarId, y: VarId, b: VarId) {
        self.props.int_le_reif(x, y, b);
    }

    /// Post a reified greater-than constraint: `b ⇔ (x > y)`.
    /// 
    /// The boolean variable `b` is 1 if and only if `x > y`.
    /// This is useful for FlatZinc integration and conditional constraints.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(1, 10);
    /// let y = m.int(1, 10);
    /// let b = m.bool();
    /// m.int_gt_reif(x, y, b);
    /// // Now b is 1 iff x > y
    /// ```
    pub fn int_gt_reif(&mut self, x: VarId, y: VarId, b: VarId) {
        self.props.int_gt_reif(x, y, b);
    }

    /// Post a reified greater-than-or-equal constraint: `b ⇔ (x ≥ y)`.
    /// 
    /// The boolean variable `b` is 1 if and only if `x ≥ y`.
    /// This is useful for FlatZinc integration and conditional constraints.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(1, 10);
    /// let y = m.int(1, 10);
    /// let b = m.bool();
    /// m.int_ge_reif(x, y, b);
    /// // Now b is 1 iff x ≥ y
    /// ```
    pub fn int_ge_reif(&mut self, x: VarId, y: VarId, b: VarId) {
        self.props.int_ge_reif(x, y, b);
    }

    /// Post a reified float equality constraint: `b ⇔ (x = y)`.
    /// 
    /// The boolean variable `b` is 1 if and only if `x = y` (for float variables).
    /// This is useful for FlatZinc integration and conditional constraints with floats.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.float(0.0, 10.0);
    /// let y = m.float(0.0, 10.0);
    /// let b = m.bool();
    /// m.float_eq_reif(x, y, b);
    /// // Now b is 1 iff x = y
    /// ```
    pub fn float_eq_reif(&mut self, x: VarId, y: VarId, b: VarId) {
        // Float equality reification uses the same propagator as integers
        // since the comparison logic is type-agnostic at the VarId level
        self.props.int_eq_reif(x, y, b);
    }

    /// Post a reified float not-equal constraint: `b ⇔ (x ≠ y)`.
    /// 
    /// The boolean variable `b` is 1 if and only if `x ≠ y` (for float variables).
    /// This is useful for FlatZinc integration and conditional constraints with floats.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.float(0.0, 10.0);
    /// let y = m.float(0.0, 10.0);
    /// let b = m.bool();
    /// m.float_ne_reif(x, y, b);
    /// // Now b is 1 iff x ≠ y
    /// ```
    pub fn float_ne_reif(&mut self, x: VarId, y: VarId, b: VarId) {
        self.props.int_ne_reif(x, y, b);
    }

    /// Post a reified float less-than constraint: `b ⇔ (x < y)`.
    /// 
    /// The boolean variable `b` is 1 if and only if `x < y` (for float variables).
    /// This is useful for FlatZinc integration and conditional constraints with floats.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.float(0.0, 10.0);
    /// let y = m.float(0.0, 10.0);
    /// let b = m.bool();
    /// m.float_lt_reif(x, y, b);
    /// // Now b is 1 iff x < y
    /// ```
    pub fn float_lt_reif(&mut self, x: VarId, y: VarId, b: VarId) {
        self.props.int_lt_reif(x, y, b);
    }

    /// Post a reified float less-than-or-equal constraint: `b ⇔ (x ≤ y)`.
    /// 
    /// The boolean variable `b` is 1 if and only if `x ≤ y` (for float variables).
    /// This is useful for FlatZinc integration and conditional constraints with floats.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.float(0.0, 10.0);
    /// let y = m.float(0.0, 10.0);
    /// let b = m.bool();
    /// m.float_le_reif(x, y, b);
    /// // Now b is 1 iff x ≤ y
    /// ```
    pub fn float_le_reif(&mut self, x: VarId, y: VarId, b: VarId) {
        self.props.int_le_reif(x, y, b);
    }

    /// Post a reified float greater-than constraint: `b ⇔ (x > y)`.
    /// 
    /// The boolean variable `b` is 1 if and only if `x > y` (for float variables).
    /// This is useful for FlatZinc integration and conditional constraints with floats.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.float(0.0, 10.0);
    /// let y = m.float(0.0, 10.0);
    /// let b = m.bool();
    /// m.float_gt_reif(x, y, b);
    /// // Now b is 1 iff x > y
    /// ```
    pub fn float_gt_reif(&mut self, x: VarId, y: VarId, b: VarId) {
        self.props.int_gt_reif(x, y, b);
    }

    /// Post a reified float greater-than-or-equal constraint: `b ⇔ (x ≥ y)`.
    /// 
    /// The boolean variable `b` is 1 if and only if `x ≥ y` (for float variables).
    /// This is useful for FlatZinc integration and conditional constraints with floats.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.float(0.0, 10.0);
    /// let y = m.float(0.0, 10.0);
    /// let b = m.bool();
    /// m.float_ge_reif(x, y, b);
    /// // Now b is 1 iff x ≥ y
    /// ```
    pub fn float_ge_reif(&mut self, x: VarId, y: VarId, b: VarId) {
        self.props.int_ge_reif(x, y, b);
    }
}
