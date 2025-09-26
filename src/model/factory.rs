//! Public Variable Factory API
//!
//! This module provides the clean, user-friendly API for creating variables in constraint models.
//! This is the ONLY API that end users should use for variable creation.
//!
//! ## Single Variable Creation
//! - `int(min, max)` - Create a single integer variable
//! - `float(min, max)` - Create a single floating-point variable  
//! - `bool()` - Create a single boolean variable (0 or 1)
//! - `intset(values)` - Create a variable from a set of specific integer values
//!
//! ## Multiple Variable Creation
//! - `ints(n, min, max)` - Create n integer variables with the same bounds
//! - `floats(n, min, max)` - Create n floating-point variables with the same bounds
//! - `bools(n)` - Create n boolean variables

use crate::model::core::Model;
use crate::variables::{Val, VarId};

impl Model {
    // ========================================================================
    // SINGLE VARIABLE CREATION - PRIMARY PUBLIC API
    // ========================================================================

    /// Create an integer variable with specified bounds.
    /// 
    /// Creates a variable that can take any integer value between `min` and `max` (inclusive).
    ///
    /// # Arguments
    /// * `min` - Minimum value for the variable (inclusive)
    /// * `max` - Maximum value for the variable (inclusive)
    ///
    /// # Example
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(1, 10);     // Variable from 1 to 10
    /// let y = m.int(-5, 15);    // Variable from -5 to 15
    /// ```
    pub fn int(&mut self, min: i32, max: i32) -> VarId {
        self.new_var_unchecked(Val::ValI(min), Val::ValI(max))
    }

    /// Create a floating-point variable with specified bounds.
    /// 
    /// Creates a variable that can take any floating-point value between `min` and `max` (inclusive).
    /// The precision is controlled by the model's `float_precision_digits` setting.
    ///
    /// # Arguments
    /// * `min` - Minimum value for the variable (inclusive)
    /// * `max` - Maximum value for the variable (inclusive)
    ///
    /// # Example
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.float(0.0, 10.0);    // Variable from 0.0 to 10.0
    /// let y = m.float(-1.5, 3.14);   // Variable from -1.5 to 3.14
    /// ```
    pub fn float(&mut self, min: f64, max: f64) -> VarId {
        self.new_var_unchecked(Val::ValF(min), Val::ValF(max))
    }

    /// Create a boolean variable (0 or 1).
    ///
    /// Creates a variable that can only take values 0 or 1, useful for representing
    /// boolean logic, flags, or binary decisions.
    /// 
    /// # Returns
    /// A `VarId` that can take values 0 (false) or 1 (true)
    ///
    /// # Example
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let flag = m.bool();          // 0 or 1
    /// let enabled = m.bool();       // 0 or 1
    /// 
    /// // Use in constraints
    /// post!(m, flag != enabled);    // Flags must be different
    /// ```
    pub fn bool(&mut self) -> VarId {
        self.int(0, 1)
    }

    /// Create an integer variable from a specific set of values.
    /// 
    /// Creates a variable that can only take values from the provided list.
    /// This is useful for non-contiguous domains, categorical values, or
    /// when you need precise control over allowed values.
    ///
    /// # Arguments
    /// * `values` - Vector of integer values that the variable can take
    ///
    /// # Returns
    /// A `VarId` that can only take values from the provided vector
    ///
    /// # Example
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// 
    /// // Variable that can only be prime numbers
    /// let prime = m.intset(vec![2, 3, 5, 7, 11, 13]);
    /// 
    /// // Variable for days of week (1=Monday, 7=Sunday)  
    /// let weekday = m.intset(vec![1, 2, 3, 4, 5, 6, 7]);
    /// 
    /// // Non-contiguous range
    /// let sparse = m.intset(vec![1, 5, 10, 50, 100]);
    /// 
    /// post!(m, prime != weekday);
    /// ```
    pub fn intset(&mut self, values: Vec<i32>) -> VarId {
        self.props_mut().on_new_var();
        self.vars_mut().new_var_with_values(values)
    }

    // ========================================================================
    // MULTIPLE VARIABLE CREATION - PUBLIC API
    // ========================================================================

    /// Create multiple integer variables with the same bounds.
    /// 
    /// Creates `n` integer variables, each with the same domain bounds.
    /// This is more efficient than calling `int()` multiple times.
    ///
    /// # Arguments
    /// * `n` - Number of variables to create
    /// * `min` - Minimum value for each variable (inclusive)
    /// * `max` - Maximum value for each variable (inclusive)
    ///
    /// # Returns
    /// A `Vec<VarId>` containing all created variables
    ///
    /// # Example
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let vars = m.ints(5, 1, 10);     // 5 variables, each from 1 to 10
    /// let sudoku_row = m.ints(9, 1, 9); // 9 variables for Sudoku row
    /// ```
    pub fn ints(&mut self, n: usize, min: i32, max: i32) -> Vec<VarId> {
        self.int_vars(n, min, max).collect()
    }

    /// Create multiple floating-point variables with the same bounds.
    /// 
    /// Creates `n` floating-point variables, each with the same domain bounds.
    /// This is more efficient than calling `float()` multiple times.
    ///
    /// # Arguments
    /// * `n` - Number of variables to create
    /// * `min` - Minimum value for each variable (inclusive)
    /// * `max` - Maximum value for each variable (inclusive)
    ///
    /// # Returns
    /// A `Vec<VarId>` containing all created variables
    ///
    /// # Example
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let coords = m.floats(3, 0.0, 1.0);   // 3 variables for x, y, z coordinates
    /// let weights = m.floats(10, 0.0, 100.0); // 10 weight variables
    /// ```
    pub fn floats(&mut self, n: usize, min: f64, max: f64) -> Vec<VarId> {
        self.float_vars(n, min, max).collect()
    }

    /// Create multiple boolean variables.
    /// 
    /// Creates `n` boolean variables, each with domain [0, 1].
    /// This is more efficient than calling `bool()` multiple times.
    ///
    /// # Arguments
    /// * `n` - Number of boolean variables to create
    ///
    /// # Returns
    /// A `Vec<VarId>` containing all created boolean variables
    ///
    /// # Example
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let flags = m.bools(8);           // 8 boolean flags
    /// let choices = m.bools(10);        // 10 binary choices
    /// 
    /// // Use in constraints
    /// post!(m, flags[0] != flags[1]);   // Different flags
    /// ```
    pub fn bools(&mut self, n: usize) -> Vec<VarId> {
        self.int_vars(n, 0, 1).collect()
    }
}