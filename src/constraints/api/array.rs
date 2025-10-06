//! Array operation constraints
//!
//! This module contains constraints for array operations:
//! - Integer arrays: array_int_minimum, array_int_maximum, array_int_element
//! - Float arrays: array_float_minimum, array_float_maximum, array_float_element

use crate::model::Model;
use crate::variables::VarId;
use crate::core::error::SolverResult;

impl Model {
    // ============================================================================
    // Integer Array Constraints
    // ============================================================================

    /// Find the minimum value in an integer array.
    /// 
    /// This implements the MiniZinc/FlatZinc `array_int_minimum` constraint.
    /// Constrains `result` to equal the minimum value in `array`.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(1, 10);
    /// let y = m.int(5, 15);
    /// let z = m.int(3, 8);
    /// 
    /// let min_result = m.array_int_minimum(&[x, y, z]).expect("non-empty array");
    /// ```
    /// 
    /// # Errors
    /// Returns `SolverError::InvalidInput` if the array is empty.
    /// 
    /// # Note
    /// This is a convenience wrapper around the generic `min()` method,
    /// provided for MiniZinc/FlatZinc compatibility. The underlying implementation
    /// works for both integer and float variables.
    pub fn array_int_minimum(&mut self, array: &[VarId]) -> SolverResult<VarId> {
        // Delegate to the generic min() implementation which works for both int and float
        self.min(array)
    }

    /// Find the maximum value in an integer array.
    /// 
    /// This implements the MiniZinc/FlatZinc `array_int_maximum` constraint.
    /// Constrains `result` to equal the maximum value in `array`.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(1, 10);
    /// let y = m.int(5, 15);
    /// let z = m.int(3, 8);
    /// 
    /// let max_result = m.array_int_maximum(&[x, y, z]).expect("non-empty array");
    /// ```
    /// 
    /// # Errors
    /// Returns `SolverError::InvalidInput` if the array is empty.
    /// 
    /// # Note
    /// This is a convenience wrapper around the generic `max()` method,
    /// provided for MiniZinc/FlatZinc compatibility. The underlying implementation
    /// works for both integer and float variables.
    pub fn array_int_maximum(&mut self, array: &[VarId]) -> SolverResult<VarId> {
        // Delegate to the generic max() implementation which works for both int and float
        self.max(array)
    }

    /// Access an element from an integer array using a variable index.
    /// 
    /// This implements the MiniZinc/FlatZinc `array_int_element` and 
    /// `array_var_int_element` constraints. Constrains `result = array[index]` 
    /// where `index` is a variable.
    /// 
    /// # Arguments
    /// * `index` - Integer variable representing the array index (0-based)
    /// * `array` - Array of integer variables to index into
    /// * `result` - Integer variable that will equal `array[index]`
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// 
    /// // Array of scores
    /// let scores = vec![
    ///     m.int(10, 10),
    ///     m.int(20, 20),
    ///     m.int(30, 30),
    /// ];
    /// 
    /// let index = m.int(0, 2);
    /// let selected_score = m.int(0, 50);
    /// 
    /// // selected_score = scores[index]
    /// m.array_int_element(index, &scores, selected_score);
    /// ```
    /// 
    /// # Note
    /// This is a convenience wrapper around the generic element constraint,
    /// provided for MiniZinc/FlatZinc compatibility. The underlying `props.element()`
    /// implementation works for both integer and float variables.
    pub fn array_int_element(&mut self, index: VarId, array: &[VarId], result: VarId) {
        // Delegate to the generic element constraint which works for both int and float
        self.props.element(array.to_vec(), index, result);
    }

    // ============================================================================
    // Float Array Constraints
    // ============================================================================

    /// Find the minimum value in a float array.
    /// 
    /// This implements the MiniZinc/FlatZinc `array_float_minimum` constraint.
    pub fn array_float_minimum(&mut self, array: &[VarId]) -> SolverResult<VarId> {
        // Delegate to the generic min() implementation which works for floats
        self.min(array)
    }

    /// Find the maximum value in a float array.
    /// 
    /// This implements the FlatZinc `array_float_maximum` constraint.
    /// Constrains `result` to equal the maximum value in `array`.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.float(1.5, 10.5);
    /// let y = m.float(2.3, 8.7);
    /// let z = m.float(3.1, 6.2);
    /// 
    /// let max_result = m.array_float_maximum(&[x, y, z]).expect("non-empty array");
    /// ```
    /// 
    /// # Errors
    /// Returns `SolverError::InvalidInput` if the array is empty.
    /// 
    /// # Note
    /// This is a convenience wrapper around the generic `max()` method,
    /// provided for FlatZinc compatibility. The underlying implementation
    /// works for both integer and float variables.
    pub fn array_float_maximum(&mut self, array: &[VarId]) -> SolverResult<VarId> {
        // Delegate to the generic max() implementation which works for floats
        self.max(array)
    }

    /// Access an element from a float array using a variable index.
    /// 
    /// This implements the FlatZinc `array_float_element` constraint.
    /// Constrains `result = array[index]` where `index` is a variable.
    /// 
    /// # Arguments
    /// * `index` - Integer variable representing the array index (0-based)
    /// * `array` - Array of float variables to index into
    /// * `result` - Float variable that will equal `array[index]`
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// 
    /// // Array of prices
    /// let prices = vec![
    ///     m.float(10.5, 10.5),
    ///     m.float(12.3, 12.3),
    ///     m.float(15.0, 15.0),
    /// ];
    /// 
    /// let index = m.int(0, 2);
    /// let selected_price = m.float(0.0, 20.0);
    /// 
    /// // selected_price = prices[index]
    /// m.array_float_element(index, &prices, selected_price);
    /// ```
    /// 
    /// # Note
    /// This is a convenience wrapper around the generic element constraint,
    /// provided for FlatZinc compatibility. The underlying `props.element()`
    /// implementation works for both integer and float variables.
    pub fn array_float_element(&mut self, index: VarId, array: &[VarId], result: VarId) {
        // Delegate to the generic element constraint which works for floats
        self.props.element(array.to_vec(), index, result);
    }
}

