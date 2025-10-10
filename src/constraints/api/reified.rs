//! Reified constraint methods for Model
//!
//! This module provides convenient methods on the Model struct for posting
//! reified constraints. These methods delegate to the generic functions from
//! the constraints::functions module.
//!
//! ## Available Methods
//!
//! - `model.eq_reif(x, y, b)` - b ⇔ (x == y)
//! - `model.ne_reif(x, y, b)` - b ⇔ (x != y)
//! - `model.lt_reif(x, y, b)` - b ⇔ (x < y)
//! - `model.le_reif(x, y, b)` - b ⇔ (x <= y)
//! - `model.gt_reif(x, y, b)` - b ⇔ (x > y)
//! - `model.ge_reif(x, y, b)` - b ⇔ (x >= y)
//!
//! All methods work with both integer and float variables.

use crate::model::Model;
use crate::variables::VarId;

impl Model {
    /// Post a reified equality constraint: `b ⇔ (x == y)`.
    ///
    /// The boolean variable `b` is true if and only if `x` equals `y`.
    /// Works with both integer and float variables.
    ///
    /// # Arguments
    /// * `x` - First variable
    /// * `y` - Second variable
    /// * `b` - Boolean variable that reflects the result
    ///
    /// # Examples
    /// ```
    /// # use selen::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.int(0, 10);
    /// let y = model.int(0, 10);
    /// let b = model.bool();
    /// model.eq_reif(x, y, b); // b is true iff x == y
    /// ```
    pub fn eq_reif(&mut self, x: VarId, y: VarId, b: VarId) {
        crate::constraints::functions::eq_reif(self, x, y, b);
    }

    /// Post a reified not-equal constraint: `b ⇔ (x != y)`.
    ///
    /// The boolean variable `b` is true if and only if `x` does not equal `y`.
    /// Works with both integer and float variables.
    ///
    /// # Arguments
    /// * `x` - First variable
    /// * `y` - Second variable
    /// * `b` - Boolean variable that reflects the result
    ///
    /// # Examples
    /// ```
    /// # use selen::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.int(0, 10);
    /// let y = model.int(0, 10);
    /// let b = model.bool();
    /// model.ne_reif(x, y, b); // b is true iff x != y
    /// ```
    pub fn ne_reif(&mut self, x: VarId, y: VarId, b: VarId) {
        crate::constraints::functions::ne_reif(self, x, y, b);
    }

    /// Post a reified less-than constraint: `b ⇔ (x < y)`.
    ///
    /// The boolean variable `b` is true if and only if `x` is less than `y`.
    /// Works with both integer and float variables.
    ///
    /// # Arguments
    /// * `x` - First variable
    /// * `y` - Second variable
    /// * `b` - Boolean variable that reflects the result
    ///
    /// # Examples
    /// ```
    /// # use selen::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.int(0, 10);
    /// let y = model.int(0, 10);
    /// let b = model.bool();
    /// model.lt_reif(x, y, b); // b is true iff x < y
    /// ```
    pub fn lt_reif(&mut self, x: VarId, y: VarId, b: VarId) {
        crate::constraints::functions::lt_reif(self, x, y, b);
    }

    /// Post a reified less-than-or-equal constraint: `b ⇔ (x <= y)`.
    ///
    /// The boolean variable `b` is true if and only if `x` is less than or equal to `y`.
    /// Works with both integer and float variables.
    ///
    /// # Arguments
    /// * `x` - First variable
    /// * `y` - Second variable
    /// * `b` - Boolean variable that reflects the result
    ///
    /// # Examples
    /// ```
    /// # use selen::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.int(0, 10);
    /// let y = model.int(0, 10);
    /// let b = model.bool();
    /// model.le_reif(x, y, b); // b is true iff x <= y
    /// ```
    pub fn le_reif(&mut self, x: VarId, y: VarId, b: VarId) {
        crate::constraints::functions::le_reif(self, x, y, b);
    }

    /// Post a reified greater-than constraint: `b ⇔ (x > y)`.
    ///
    /// The boolean variable `b` is true if and only if `x` is greater than `y`.
    /// Works with both integer and float variables.
    ///
    /// # Arguments
    /// * `x` - First variable
    /// * `y` - Second variable
    /// * `b` - Boolean variable that reflects the result
    ///
    /// # Examples
    /// ```
    /// # use selen::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.int(0, 10);
    /// let y = model.int(0, 10);
    /// let b = model.bool();
    /// model.gt_reif(x, y, b); // b is true iff x > y
    /// ```
    pub fn gt_reif(&mut self, x: VarId, y: VarId, b: VarId) {
        crate::constraints::functions::gt_reif(self, x, y, b);
    }

    /// Post a reified greater-than-or-equal constraint: `b ⇔ (x >= y)`.
    ///
    /// The boolean variable `b` is true if and only if `x` is greater than or equal to `y`.
    /// Works with both integer and float variables.
    ///
    /// # Arguments
    /// * `x` - First variable
    /// * `y` - Second variable
    /// * `b` - Boolean variable that reflects the result
    ///
    /// # Examples
    /// ```
    /// # use selen::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.int(0, 10);
    /// let y = model.int(0, 10);
    /// let b = model.bool();
    /// model.ge_reif(x, y, b); // b is true iff x >= y
    /// ```
    pub fn ge_reif(&mut self, x: VarId, y: VarId, b: VarId) {
        crate::constraints::functions::ge_reif(self, x, y, b);
    }
}
