// Constraint posting methods
//
// This module contains the actual implementations of constraint posting methods
// that were moved here to complete the modularization architecture.

use crate::model::core::Model;
use crate::variables::{VarId, View, Val};
use crate::variables::views::ViewRaw;
use crate::core::error::{SolverError, SolverResult};

impl Model {
    // ═══════════════════════════════════════════════════════════════════════
    // 📐 Mathematical Operations
    // ═══════════════════════════════════════════════════════════════════════

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
        
        // Calculate bounds for modulo result
        // This is conservative - the actual bounds depend on the signs of x and y
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

    // ═══════════════════════════════════════════════════════════════════════
    // 🌐 Global Constraints
    // ═══════════════════════════════════════════════════════════════════════

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

    // ═══════════════════════════════════════════════════════════════════════
    // 🔍 Boolean Operations
    // ═══════════════════════════════════════════════════════════════════════

    #[doc(hidden)]
    /// Create a variable representing the boolean AND of multiple operands.
    /// Returns a variable that is 1 if ALL operands are non-zero, 0 otherwise.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let a = m.bool();
    /// let b = m.bool();
    /// let c = m.bool();
    /// let and_result = m.bool_and(&[a, b, c]);
    /// ```
    pub fn bool_and(&mut self, operands: &[VarId]) -> VarId {
        let result = self.bool(); // Create a boolean variable (0 or 1)
        self.props.bool_and(operands.to_vec(), result);
        result
    }

    #[doc(hidden)]
    /// Create a variable representing the boolean OR of multiple operands.
    /// Returns a variable that is 1 if ANY operand is non-zero, 0 otherwise.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let a = m.bool();
    /// let b = m.bool();
    /// let or_result = m.bool_or(&[a, b]);
    /// ```
    pub fn bool_or(&mut self, operands: &[VarId]) -> VarId {
        let result = self.bool(); // Create a boolean variable (0 or 1)
        self.props.bool_or(operands.to_vec(), result);
        result
    }

    #[doc(hidden)]
    /// Create a variable representing the boolean NOT of an operand.
    /// Returns a variable that is 1 if the operand is 0, and 0 if the operand is non-zero.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let a = m.bool();
    /// let not_a = m.bool_not(a);
    /// ```
    pub fn bool_not(&mut self, operand: VarId) -> VarId {
        let result = self.bool(); // Create a boolean variable (0 or 1)
        self.props.bool_not(operand, result);
        result
    }

    // ═══════════════════════════════════════════════════════════════════════
    // 🔄 Reification Support
    // ═══════════════════════════════════════════════════════════════════════

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

    // ═══════════════════════════════════════════════════════════════════════
    // 📊 Linear Constraints (FlatZinc Integration)
    // ═══════════════════════════════════════════════════════════════════════

    /// Post a linear equality constraint: `sum(coeffs[i] * vars[i]) = constant`.
    /// 
    /// This implements the FlatZinc `int_lin_eq` constraint, which represents
    /// a weighted sum of variables equal to a constant value.
    /// 
    /// # Arguments
    /// * `coefficients` - Array of integer coefficients
    /// * `variables` - Array of variables (must have same length as coefficients)
    /// * `constant` - The constant value the weighted sum must equal
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(0, 10);
    /// let y = m.int(0, 10);
    /// let z = m.int(0, 10);
    /// 
    /// // 2x + 3y - z = 10
    /// m.int_lin_eq(&[2, 3, -1], &[x, y, z], 10);
    /// ```
    /// 
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

        // Create scaled variables: coeffs[i] * vars[i]
        // We use actual multiplication to create new variables, not views
        let scaled_vars: Vec<VarId> = coefficients
            .iter()
            .zip(variables.iter())
            .map(|(&coeff, &var)| {
                self.mul(var, Val::ValI(coeff))
            })
            .collect();

        // Create sum of all scaled variables
        let sum_var = self.sum(&scaled_vars);

        // Post equality constraint: sum = constant
        self.props.equals(sum_var, Val::ValI(constant));
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

        // Create scaled variables: coeffs[i] * vars[i]
        // We use actual multiplication to create new variables, not views
        let scaled_vars: Vec<VarId> = coefficients
            .iter()
            .zip(variables.iter())
            .map(|(&coeff, &var)| {
                self.mul(var, Val::ValI(coeff))
            })
            .collect();

        // Create sum of all scaled variables
        let sum_var = self.sum(&scaled_vars);

        // Post less-than-or-equal constraint: sum ≤ constant
        self.props.less_than_or_equals(sum_var, Val::ValI(constant));
    }

    // ═══════════════════════════════════════════════════════════════════════
    // 🔀 Boolean Clause (CNF/SAT Support)
    // ═══════════════════════════════════════════════════════════════════════

    /// Post a boolean clause constraint: `(∨ pos[i]) ∨ (∨ ¬neg[i])`.
    /// 
    /// This implements the FlatZinc `bool_clause` constraint, which represents
    /// a clause in CNF (Conjunctive Normal Form). The clause is satisfied if:
    /// - At least one positive literal is true, OR
    /// - At least one negative literal is false
    /// 
    /// In other words: `pos[0] ∨ pos[1] ∨ ... ∨ ¬neg[0] ∨ ¬neg[1] ∨ ...`
    /// 
    /// # Arguments
    /// * `pos` - Array of positive boolean literals (variables that should be true)
    /// * `neg` - Array of negative boolean literals (variables that should be false)
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let a = m.bool();
    /// let b = m.bool();
    /// let c = m.bool();
    /// 
    /// // At least one of: a is true, b is true, or c is false
    /// // Equivalent to: a ∨ b ∨ ¬c
    /// m.bool_clause(&[a, b], &[c]);
    /// ```
    /// 
    /// # Implementation
    /// 
    /// The clause is decomposed as:
    /// 1. If both arrays are empty, the clause is unsatisfiable (posts false)
    /// 2. Otherwise, we create: `(∨ pos[i]) ∨ (∨ ¬neg[i]) = true`
    ///    - This ensures at least one positive literal is 1, or one negative literal is 0
    pub fn bool_clause(&mut self, pos: &[VarId], neg: &[VarId]) {
        // Empty clause is unsatisfiable
        if pos.is_empty() && neg.is_empty() {
            // Post an unsatisfiable constraint: 0 = 1
            self.props.equals(Val::ValI(0), Val::ValI(1));
            return;
        }

        // Special case: only positive literals
        if neg.is_empty() {
            // At least one positive literal must be true: bool_or(pos) = 1
            let clause_result = self.bool_or(pos);
            self.props.equals(clause_result, Val::ValI(1));
            return;
        }

        // Special case: only negative literals
        if pos.is_empty() {
            // At least one negative literal must be false
            // ¬neg[0] ∨ ¬neg[1] ∨ ... = ¬(neg[0] ∧ neg[1] ∧ ...)
            let all_neg = self.bool_and(neg);
            let not_all_neg = self.bool_not(all_neg);
            self.props.equals(not_all_neg, Val::ValI(1));
            return;
        }

        // General case: both positive and negative literals
        // pos[0] ∨ ... ∨ ¬neg[0] ∨ ...
        // = (pos[0] ∨ ... ∨ pos[n]) ∨ (¬neg[0] ∨ ... ∨ ¬neg[m])
        // = (∨ pos[i]) ∨ ¬(∧ neg[i])
        
        let pos_clause = self.bool_or(pos);
        let all_neg = self.bool_and(neg);
        let not_all_neg = self.bool_not(all_neg);
        
        // At least one side must be true
        let final_clause = self.bool_or(&[pos_clause, not_all_neg]);
        self.props.equals(final_clause, Val::ValI(1));
    }
}
