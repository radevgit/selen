//! Arithmetic constraint operations
//!
//! This module contains mathematical operations and aggregate constraints:
//! - Binary operations: add, sub, mul, div, modulo
//! - Unary operations: abs
//! - Aggregate operations: min, max, sum

use crate::model::Model;
use crate::variables::{VarId, View, Val};
use crate::variables::views::ViewRaw;
use crate::core::error::{SolverError, SolverResult};

impl Model {
    /// Create an expression representing the sum of two views: `x + y`.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(1, 10);
    /// let y = m.int(5, 15);
    /// let sum = m.add(x, y);
    /// ```
    pub fn add(&mut self, x: impl View, y: impl View) -> VarId {
        let min = x.min_raw(&self.vars) + y.min_raw(&self.vars);
        let max = x.max_raw(&self.vars) + y.max_raw(&self.vars);
        let s = self.new_var_unchecked(min, max);

        let _p = self.props.add(x, y, s);

        s
    }

    /// Create an expression representing the difference of two views: `x - y`.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(5, 10);
    /// let y = m.int(2, 4);
    /// let diff = m.sub(x, y);
    /// ```
    pub fn sub(&mut self, x: impl View, y: impl View) -> VarId {
        let min = x.min_raw(&self.vars) - y.max_raw(&self.vars);
        let max = x.max_raw(&self.vars) - y.min_raw(&self.vars);
        let s = self.new_var_unchecked(min, max);

        let _p = self.props.sub(x, y, s);

        s
    }

    /// Create an expression of the multiplication of two views.
    /// 
    /// # Example
    ///
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(3, 5);
    /// let y = m.int(2, 4);
    /// let product = m.mul(x, y);
    /// ```
    pub fn mul(&mut self, x: impl View, y: impl View) -> VarId {
        let x_min = x.min_raw(&self.vars);
        let x_max = x.max_raw(&self.vars);
        let y_min = y.min_raw(&self.vars);
        let y_max = y.max_raw(&self.vars);
        
        // Calculate all possible products at the corners
        let products = [
            x_min * y_min,
            x_min * y_max,
            x_max * y_min,
            x_max * y_max,
        ];
        
        // Find min and max
        let min = products.iter().fold(products[0], |acc, &x| if x < acc { x } else { acc });
        let max = products.iter().fold(products[0], |acc, &x| if x > acc { x } else { acc });
        
        // Create intermediate variable for the multiplication result
        // Use standard step size to ensure accurate value representation
        let s = self.new_var_unchecked(min, max);

        let _p = self.props.mul(x, y, s);

        s
    }

    /// Create a new variable that holds the result of `x / y` (division).
    ///
    /// Division operations require special care with domain boundaries and 
    /// potential division by zero. The solver will ensure y ≠ 0.
    ///
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(10, 20);
    /// let y = m.int(2, 5);
    /// let quotient = m.div(x, y);
    /// ```
    pub fn div(&mut self, x: impl View, y: impl View) -> VarId {
        let x_min = x.min_raw(&self.vars);
        let x_max = x.max_raw(&self.vars);
        let y_min = y.min_raw(&self.vars);
        let y_max = y.max_raw(&self.vars);
        
        // Calculate bounds for division result
        let mut min = Val::ValF(f64::INFINITY);
        let mut max = Val::ValF(f64::NEG_INFINITY);
        
        // Sample corner values to estimate bounds (similar to multiplication)
        let x_samples = [x_min, x_max];
        let y_samples = [y_min, y_max];
        
        for &x_val in &x_samples {
            for &y_val in &y_samples {
                if let Some(div_result) = x_val.safe_div(y_val) {
                    match div_result {
                        Val::ValF(f) if f.is_finite() => {
                            if div_result < min { min = div_result; }
                            if div_result > max { max = div_result; }
                        },
                        Val::ValI(i) => {
                            let f_val = Val::ValF(i as f64);
                            if f_val < min { min = f_val; }
                            if f_val > max { max = f_val; }
                        },
                        _ => {} // Skip infinite or NaN results
                    }
                }
            }
        }
        
        // If we couldn't calculate any valid division results, use conservative bounds
        if min == Val::ValF(f64::INFINITY) || max == Val::ValF(f64::NEG_INFINITY) {
            min = Val::ValF(-1000.0); // Very conservative
            max = Val::ValF(1000.0);
        }
        
        let s = self.new_var_unchecked(min, max);
        let _p = self.props.div(x, y, s);
        s
    }

    /// Create a new variable that holds the result of `x % y` (modulo operation).
    ///
    /// For the modulo operation `x % y = result`:
    /// - If y > 0: result is in range [0, y-1]  
    /// - If y < 0: result is in range [y+1, 0]
    /// - If y contains 0, the constraint may fail during solving
    ///
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(10, 20);
    /// let y = m.int(3, 7);
    /// let remainder = m.modulo(x, y);
    /// ```
    pub fn modulo(&mut self, x: impl View, y: impl View) -> VarId {
        let x_min = x.min_raw(&self.vars);
        let x_max = x.max_raw(&self.vars);
        let y_min = y.min_raw(&self.vars);
        let y_max = y.max_raw(&self.vars);
        
        // IMPORTANT: We must create the result variable with bounds that account for
        // potential pending deferred constraints. Since we can't know what those are,
        // we use CONSERVATIVE bounds that encompass all possible modulo results.
        
        // For modulo, the result is always in range [0, |divisor|-1] when divisor > 0
        // So we need to find the range that could result from ANY x in its range
        // and ANY y in its range that isn't zero.
        
        let mut min = Val::ValI(i32::MAX);
        let mut max = Val::ValI(i32::MIN);
        
        // Sample corner values to estimate bounds
        let x_samples = [x_min, x_max];
        let y_samples = [y_min, y_max];
        
        for &x_val in &x_samples {
            for &y_val in &y_samples {
                if let Some(mod_result) = x_val.safe_mod(y_val) {
                    if mod_result < min { min = mod_result; }
                    if mod_result > max { max = mod_result; }
                }
            }
        }
        
        // If we couldn't calculate any valid modulo results, use conservative bounds
        if min == Val::ValI(i32::MAX) || max == Val::ValI(i32::MIN) {
            // Conservative estimate: result can be as large as the largest divisor
            let y_abs_max = match (y_min, y_max) {
                (Val::ValI(min_i), Val::ValI(max_i)) => {
                    let abs_min = min_i.abs();
                    let abs_max = max_i.abs();
                    Val::ValI(if abs_min > abs_max { abs_min } else { abs_max })
                },
                (Val::ValF(min_f), Val::ValF(max_f)) => {
                    let abs_min = min_f.abs();
                    let abs_max = max_f.abs();
                    Val::ValF(if abs_min > abs_max { abs_min } else { abs_max })
                },
                _ => Val::ValI(100), // Very conservative fallback
            };
            
            min = match y_abs_max {
                Val::ValI(i) => Val::ValI(-i),
                Val::ValF(f) => Val::ValF(-f),
            };
            max = y_abs_max;
        } else {
            // CRITICAL FIX: Even with sampled bounds, we need to be MORE conservative
            // to handle deferred constraints that might widen the operand domains.
            // Expand the bounds to cover all possible modulo results.
            match (min, max) {
                (Val::ValI(_min_i), Val::ValI(_max_i)) => {
                    // For integers, check what the worst-case modulo result could be
                    // given the range of divisors
                    if let (Val::ValI(y_min_i), Val::ValI(y_max_i)) = (y_min, y_max) {
                        // The maximum modulo result magnitude is (max(|y|) - 1)
                        let y_abs_max = if y_min_i.abs() > y_max_i.abs() {
                            y_min_i.abs()
                        } else {
                            y_max_i.abs()
                        };
                        
                        if y_abs_max > 0 {
                            // Result can be [-(y_abs_max-1), y_abs_max-1]
                            // Only expand if needed to encompass computed range
                            let new_min = Val::ValI(-(y_abs_max - 1));
                            let new_max = Val::ValI(y_abs_max - 1);
                            
                            if new_min < min {
                                min = new_min;
                            }
                            if new_max > max {
                                max = new_max;
                            }
                        }
                    }
                }
                _ => {} // For floats, keep as-is
            }
        }
        
        let s = self.new_var_unchecked(min, max);
        let _p = self.props.modulo(x, y, s);
        s
    }

    /// Create a new variable that holds the result of `|x|` (absolute value).
    ///
    /// The absolute value operation always produces a non-negative result:
    /// - If x >= 0: |x| = x
    /// - If x < 0: |x| = -x
    ///
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(-10, 5);
    /// let abs_x = m.abs(x);
    /// ```
    pub fn abs(&mut self, x: impl View) -> VarId {
        let x_min = x.min_raw(&self.vars);
        let x_max = x.max_raw(&self.vars);
        
        // Calculate bounds for absolute value result
        // |x| is always >= 0
        let min = match (x_min, x_max) {
            (min_val, _) if min_val >= Val::ValI(0) => {
                // x is entirely non-negative, so |x| = x
                min_val
            },
            (_, max_val) if max_val <= Val::ValI(0) => {
                // x is entirely non-positive, so |x| = -x
                match max_val {
                    Val::ValI(i) => Val::ValI(-i),
                    Val::ValF(f) => Val::ValF(-f),
                }
            },
            (_, _) => {
                // x spans both positive and negative, so min |x| = 0
                match x_min {
                    Val::ValI(_) => Val::ValI(0),
                    Val::ValF(_) => Val::ValF(0.0),
                }
            }
        };
        
        // Maximum absolute value is the larger of |x_min| and |x_max|
        let max = match (x_min, x_max) {
            (Val::ValI(min_i), Val::ValI(max_i)) => {
                let abs_min = min_i.abs();
                let abs_max = max_i.abs();
                Val::ValI(if abs_min > abs_max { abs_min } else { abs_max })
            },
            (Val::ValF(min_f), Val::ValF(max_f)) => {
                let abs_min = min_f.abs();
                let abs_max = max_f.abs();
                Val::ValF(if abs_min > abs_max { abs_min } else { abs_max })
            },
            (Val::ValI(min_i), Val::ValF(max_f)) => {
                let abs_min = (min_i as f64).abs();
                let abs_max = max_f.abs();
                Val::ValF(if abs_min > abs_max { abs_min } else { abs_max })
            },
            (Val::ValF(min_f), Val::ValI(max_i)) => {
                let abs_min = min_f.abs();
                let abs_max = (max_i as f64).abs();
                Val::ValF(if abs_min > abs_max { abs_min } else { abs_max })
            },
        };
        
        let s = self.new_var_unchecked(min, max);
        let _p = self.props.abs(x, s);
        s
    }

    /// Create a new variable that holds the minimum value of the given variables.
    ///
    /// The minimum operation finds the smallest value among all input variables:
    /// - `result = min(vars[0], vars[1], ..., vars[n])`
    /// - At least one variable must be able to achieve the minimum value
    /// - All variables must be >= result
    ///
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(1, 10);
    /// let y = m.int(5, 15);
    /// let z = m.int(3, 8);
    /// let minimum = m.min(&[x, y, z]).expect("non-empty variable list");
    /// ```
    ///
    /// # Errors
    /// Returns `SolverError::InvalidInput` if the variable list is empty.
    pub fn min(&mut self, vars: &[VarId]) -> SolverResult<VarId> {
        // Check for empty input
        if vars.is_empty() {
            return Err(SolverError::InvalidInput {
                message: "Cannot compute minimum of empty variable list".to_string(),
                function_name: Some("min".to_string()),
                expected: Some("At least one variable".to_string()),
            });
        }

        // Calculate bounds for minimum result
        let mut min_of_mins = None;
        let mut min_of_maxs = None;

        for &var in vars {
            let var_min = var.min_raw(&self.vars);
            let var_max = var.max_raw(&self.vars);

            // Update minimum of minimums (lower bound for result)
            min_of_mins = Some(match min_of_mins {
                None => var_min,
                Some(current) => if var_min < current { var_min } else { current },
            });

            // Update minimum of maximums (upper bound for result)
            min_of_maxs = Some(match min_of_maxs {
                None => var_max,
                Some(current) => if var_max < current { var_max } else { current },
            });
        }

        // These unwraps are safe because we already checked for empty vars at the beginning
        // However, for better error handling practices, we can use expect with context
        let result_min = min_of_mins.expect("internal error: min_of_mins should be Some after empty check");
        let result_max = min_of_maxs.expect("internal error: min_of_maxs should be Some after empty check");

        let result = self.new_var_unchecked(result_min, result_max);
        let _p = self.props.min(vars.to_vec(), result);
        Ok(result)
    }

    /// Create a new variable that holds the maximum value of the given variables.
    ///
    /// The maximum operation finds the largest value among all input variables:
    /// - `result = max(vars[0], vars[1], ..., vars[n])`
    /// - At least one variable must be able to achieve the maximum value
    /// - All variables must be <= result
    ///
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(1, 10);
    /// let y = m.int(5, 15);
    /// let z = m.int(3, 8);
    /// let maximum = m.max(&[x, y, z]).expect("non-empty variable list");
    /// ```
    ///
    /// # Errors
    /// Returns `SolverError::InvalidInput` if the variable list is empty.
    pub fn max(&mut self, vars: &[VarId]) -> SolverResult<VarId> {
        // Check for empty input
        if vars.is_empty() {
            return Err(SolverError::InvalidInput {
                message: "Cannot compute maximum of empty variable list".to_string(),
                function_name: Some("max".to_string()),
                expected: Some("At least one variable".to_string()),
            });
        }

        // Calculate bounds for maximum result
        let mut max_of_mins = None;
        let mut max_of_maxs = None;

        for &var in vars {
            let var_min = var.min_raw(&self.vars);
            let var_max = var.max_raw(&self.vars);

            // Update maximum of minimums (lower bound for result)
            max_of_mins = Some(match max_of_mins {
                None => var_min,
                Some(current) => if var_min > current { var_min } else { current },
            });

            // Update maximum of maximums (upper bound for result)
            max_of_maxs = Some(match max_of_maxs {
                None => var_max,
                Some(current) => if var_max > current { var_max } else { current },
            });
        }

        // These unwraps are safe because we already checked for empty vars at the beginning
        // However, for better error handling practices, we can use expect with context  
        let result_min = max_of_mins.expect("internal error: max_of_mins should be Some after empty check");
        let result_max = max_of_maxs.expect("internal error: max_of_maxs should be Some after empty check");

        let result = self.new_var_unchecked(result_min, result_max);
        let _p = self.props.max(vars.to_vec(), result);
        Ok(result)
    }

    /// Create an expression of the sum of a slice of views.
    /// 
    ///
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let vars: Vec<_> = m.int_vars(3, 1, 10).collect();
    /// let total = m.sum(&vars);
    /// ```
    pub fn sum(&mut self, xs: &[impl View]) -> VarId {
        self.sum_iter(xs.iter().copied())
    }

    #[doc(hidden)]
    /// Create an expression of the sum of an iterator of views.
    /// 
    ///
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let vars: Vec<_> = m.int_vars(3, 1, 10).collect();
    /// let total = m.sum_iter(vars.iter().copied());
    /// ```
    pub fn sum_iter(&mut self, xs: impl IntoIterator<Item = impl View>) -> VarId {
        let xs: Vec<_> = xs.into_iter().collect();

        let min: Val = xs.iter().map(|x| x.min_raw(&self.vars)).sum();
        let max: Val = xs.iter().map(|x| x.max_raw(&self.vars)).sum();
        let s = self.new_var_unchecked(min, max);

        let _p = self.props.sum(xs, s);

        s
    }
}
