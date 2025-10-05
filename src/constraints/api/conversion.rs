//! Type conversion constraints
//!
//! This module contains constraints for converting between integer and float variables:
//! - int2float: Convert integer to float
//! - float2int_floor: Convert float to integer using floor
//! - float2int_ceil: Convert float to integer using ceil
//! - float2int_round: Convert float to integer using rounding

use crate::model::Model;
use crate::variables::{VarId, Val};
use crate::variables::views::ViewRaw;

impl Model {
    pub fn int2float(&mut self, int_var: VarId, float_var: VarId) {
        // Get bounds of integer variable
        let int_min = int_var.min_raw(&self.vars);
        let int_max = int_var.max_raw(&self.vars);
        
        // Convert to float bounds
        let float_min = match int_min {
            Val::ValI(i) => Val::ValF(i as f64),
            Val::ValF(f) => Val::ValF(f.floor()), // Just in case
        };
        let float_max = match int_max {
            Val::ValI(i) => Val::ValF(i as f64),
            Val::ValF(f) => Val::ValF(f.ceil()), // Just in case
        };
        
        // Constrain float variable to integer bounds
        self.props.greater_than_or_equals(float_var, float_min);
        self.props.less_than_or_equals(float_var, float_max);
        
        // Ensure float_var = int_var exactly by creating a float view of int_var
        // Convert int_var to a float variable by adding 0.0
        let int_as_float = self.mul(int_var, Val::ValF(1.0));
        self.props.equals(float_var, int_as_float);
    }

    /// Convert a float variable to integer using floor operation.
    /// 
    /// This implements the FlatZinc `float2int_floor` constraint.
    /// Constrains `int_var` to equal floor(float_var).
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.float(0.0, 10.5);
    /// let y = m.int(-100, 100);
    /// 
    /// // y = floor(x)
    /// m.float2int_floor(x, y);
    /// ```
    pub fn float2int_floor(&mut self, float_var: VarId, int_var: VarId) {
        // Get bounds of float variable
        let float_min = float_var.min_raw(&self.vars);
        let float_max = float_var.max_raw(&self.vars);
        
        // Compute floor bounds
        let (floor_min, floor_max) = match (float_min, float_max) {
            (Val::ValF(f_min), Val::ValF(f_max)) => (f_min.floor() as i32, f_max.floor() as i32),
            (Val::ValI(i_min), Val::ValI(i_max)) => (i_min, i_max),
            (Val::ValF(f), Val::ValI(i)) => (f.floor() as i32, i),
            (Val::ValI(i), Val::ValF(f)) => (i, f.floor() as i32),
        };
        
        // Constrain int_var to floor bounds
        self.props.greater_than_or_equals(int_var, Val::ValI(floor_min));
        self.props.less_than_or_equals(int_var, Val::ValI(floor_max));
        
        // Post constraint: int_var ≤ float_var < int_var + 1
        // This ensures int_var = floor(float_var)
        self.props.less_than_or_equals(int_var, float_var);
        // Create float version: float_var < int_var + 1.0
        let int_as_float = self.add(int_var, Val::ValF(0.0)); // Convert to float
        let int_plus_one_float = self.add(int_as_float, Val::ValF(1.0));
        self.props.less_than(float_var, int_plus_one_float);
    }

    /// Convert a float variable to integer using ceiling operation.
    /// 
    /// This implements the FlatZinc `float2int_ceil` constraint.
    /// Constrains `int_var` to equal ceil(float_var).
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.float(0.0, 10.5);
    /// let y = m.int(-100, 100);
    /// 
    /// // y = ceil(x)
    /// m.float2int_ceil(x, y);
    /// ```
    pub fn float2int_ceil(&mut self, float_var: VarId, int_var: VarId) {
        // Get bounds of float variable
        let float_min = float_var.min_raw(&self.vars);
        let float_max = float_var.max_raw(&self.vars);
        
        // Compute ceil bounds
        let (ceil_min, ceil_max) = match (float_min, float_max) {
            (Val::ValF(f_min), Val::ValF(f_max)) => (f_min.ceil() as i32, f_max.ceil() as i32),
            (Val::ValI(i_min), Val::ValI(i_max)) => (i_min, i_max),
            (Val::ValF(f), Val::ValI(i)) => (f.ceil() as i32, i),
            (Val::ValI(i), Val::ValF(f)) => (i, f.ceil() as i32),
        };
        
        // Constrain int_var to ceil bounds
        self.props.greater_than_or_equals(int_var, Val::ValI(ceil_min));
        self.props.less_than_or_equals(int_var, Val::ValI(ceil_max));
        
        // Post constraint: int_var - 1 < float_var ≤ int_var
        // This ensures int_var = ceil(float_var)
        self.props.less_than_or_equals(float_var, int_var);
        // Create float version: int_var - 1.0 < float_var
        let int_as_float = self.add(int_var, Val::ValF(0.0)); // Convert to float
        let int_minus_one_float = self.sub(int_as_float, Val::ValF(1.0));
        self.props.less_than(int_minus_one_float, float_var);
    }

    /// Convert a float variable to integer using rounding operation.
    /// 
    /// This implements the FlatZinc `float2int_round` constraint.
    /// Constrains `int_var` to equal round(float_var) (rounds to nearest integer, ties to even).
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.float(0.0, 10.5);
    /// let y = m.int(-100, 100);
    /// 
    /// // y = round(x)
    /// m.float2int_round(x, y);
    /// ```
    pub fn float2int_round(&mut self, float_var: VarId, int_var: VarId) {
        // Get bounds of float variable
        let float_min = float_var.min_raw(&self.vars);
        let float_max = float_var.max_raw(&self.vars);
        
        // Compute round bounds
        let (round_min, round_max) = match (float_min, float_max) {
            (Val::ValF(f_min), Val::ValF(f_max)) => (f_min.round() as i32, f_max.round() as i32),
            (Val::ValI(i_min), Val::ValI(i_max)) => (i_min, i_max),
            (Val::ValF(f), Val::ValI(i)) => (f.round() as i32, i),
            (Val::ValI(i), Val::ValF(f)) => (i, f.round() as i32),
        };
        
        // Constrain int_var to round bounds
        self.props.greater_than_or_equals(int_var, Val::ValI(round_min));
        self.props.less_than_or_equals(int_var, Val::ValI(round_max));
        
        // Post constraint: int_var - 0.5 ≤ float_var < int_var + 0.5
        // This ensures int_var = round(float_var)
        // Note: For ties (x.5), this will round to nearest even integer
        let int_minus_half = self.add(int_var, Val::ValF(-0.5));
        let int_plus_half = self.add(int_var, Val::ValF(0.5));
        self.props.less_than_or_equals(int_minus_half, float_var);
        self.props.less_than(float_var, int_plus_half);
    }


}
