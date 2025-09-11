use crate::prelude::*;
use crate::domain::float_interval::{DEFAULT_FLOAT_PRECISION_DIGITS, precision_to_step_size};
use crate::optimization::model_integration::{OptimizationRouter, OptimizationAttempt};
use std::ops::Index;

#[derive(Debug)]
pub struct Model {
    vars: Vars,
    props: Propagators,
    /// Precision for float variables (decimal places)
    pub float_precision_digits: i32,
    /// Optimization router for efficient algorithm selection
    optimization_router: OptimizationRouter,
}

impl Default for Model {
    fn default() -> Self {
        Self::with_float_precision(DEFAULT_FLOAT_PRECISION_DIGITS)
    }
}

impl Model {
    /// Create a new model with custom float precision
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::with_float_precision(4); // 4 decimal places
    /// let var = model.new_var_float(0.0, 1.0);
    /// ```
    pub fn with_float_precision(precision_digits: i32) -> Self {
        Self {
            vars: Vars::default(),
            props: Propagators::default(),
            float_precision_digits: precision_digits,
            optimization_router: OptimizationRouter::new(),
        }
    }

    /// Get the current float precision setting
    pub fn float_precision_digits(&self) -> i32 {
        self.float_precision_digits
    }

    /// Get the step size for the current float precision
    pub fn float_step_size(&self) -> f64 {
        precision_to_step_size(self.float_precision_digits)
    }

    /// Get access to constraint registry for debugging/analysis
    pub fn get_constraint_registry(&self) -> &crate::optimization::constraint_metadata::ConstraintRegistry {
        self.props.get_constraint_registry()
    }

    /// Create a new decision variable, with the provided domain bounds.
    ///
    /// Both lower and upper bounds are included in the domain.
    /// In case `max < min` the bounds will be swapped.
    /// We don't want to deal with "unwrap" every time
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let var = model.new_var(Val::int(1), Val::int(10));
    /// ```
    pub fn new_var(&mut self, min: Val, max: Val) -> VarId {
        if min < max {
            self.new_var_unchecked(min, max)
        } else {
            self.new_var_unchecked(max, min)
        }
    }

    /// Create new decision variables, with the provided domain bounds.
    ///
    /// All created variables will have the same starting domain bounds.
    /// Both lower and upper bounds are included in the domain.
    /// In case `max < min` the bounds will be swapped.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let vars: Vec<_> = model.new_vars(3, Val::int(0), Val::int(5)).collect();
    /// ```
    pub fn new_vars(&mut self, n: usize, min: Val, max: Val) -> impl Iterator<Item = VarId> + '_ {
        let (actual_min, actual_max) = if min < max { (min, max) } else { (max, min) };
        core::iter::repeat_with(move || self.new_var_unchecked(actual_min, actual_max)).take(n)
    }

    /// Create a new integer decision variable with the provided domain bounds.
    ///
    /// Both lower and upper bounds are included in the domain.
    /// In case `max < min` the bounds will be swapped.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let var = model.new_var_int(1, 10);
    /// ```
    pub fn new_var_int(&mut self, min: i32, max: i32) -> VarId {
        self.new_var(Val::ValI(min), Val::ValI(max))
    }

    /// Create new integer decision variables, with the provided domain bounds.
    /// Both lower and upper bounds are included in the domain.
    /// In case `max < min` the bounds will be swapped.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let vars: Vec<_> = model.new_vars_int(5, 0, 9).collect();
    /// ```
    pub fn new_vars_int(
        &mut self,
        n: usize,
        min: i32,
        max: i32,
    ) -> impl Iterator<Item = VarId> + '_ {
        self.new_vars(n, Val::ValI(min), Val::ValI(max))
    }

    /// Create a new integer decision variable from a vector of specific values.
    /// This is useful for creating variables with non-contiguous domains.
    ///
    /// # Arguments
    /// * `values` - Vector of integer values that the variable can take
    ///
    /// # Returns
    /// A new VarId for the created variable
    ///
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let var = model.new_var_with_values(vec![2, 4, 6, 8]); // Even numbers only
    /// ```
    pub fn new_var_with_values(&mut self, values: Vec<i32>) -> VarId {
        self.props.on_new_var();
        self.vars.new_var_with_values(values)
    }

    /// Create a new float decision variable with the provided domain bounds.
    ///
    /// Both lower and upper bounds are included in the domain.
    /// In case `max < min` the bounds will be swapped.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let var = model.new_var_float(0.0, 10.5);
    /// ```
    pub fn new_var_float(&mut self, min: f64, max: f64) -> VarId {
        self.new_var(Val::ValF(min), Val::ValF(max))
    }

    /// Create new float decision variables, with the provided domain bounds.
    ///
    /// Both lower and upper bounds are included in the domain.
    /// In case `max < min` the bounds will be swapped.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let vars: Vec<_> = model.new_vars_float(3, 0.0, 1.0).collect();
    /// ```
    pub fn new_vars_float(
        &mut self,
        n: usize,
        min: f64,
        max: f64,
    ) -> impl Iterator<Item = VarId> + '_ {
        self.new_vars(n, Val::ValF(min), Val::ValF(max))
    }

    /// Create a new binary decision variable.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let var = model.new_var_binary();
    /// ```
    pub fn new_var_binary(&mut self) -> VarIdBin {
        VarIdBin(self.new_var_unchecked(Val::ValI(0), Val::ValI(1)))
    }

    /// Create new binary decision variables.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let vars: Vec<_> = model.new_vars_binary(4).collect();
    /// ```
    pub fn new_vars_binary(&mut self, n: usize) -> impl Iterator<Item = VarIdBin> + '_ {
        core::iter::repeat_with(|| self.new_var_binary()).take(n)
    }

    /// Create a new integer decision variable, with the provided domain bounds.
    ///
    /// Both lower and upper bounds are included in the domain.
    ///
    /// This function assumes that `min < max`.
    fn new_var_unchecked(&mut self, min: Val, max: Val) -> VarId {
        self.props.on_new_var();
        let step_size = self.float_step_size();
        self.vars.new_var_with_bounds_and_step(min, max, step_size)
    }

    /// Create an expression of two views added together.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(1, 5);
    /// let y = model.new_var_int(2, 8);
    /// let sum = model.add(x, y);
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
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(5, 10);
    /// let y = model.new_var_int(2, 4);
    /// let diff = model.sub(x, y);
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
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(3, 5);
    /// let y = model.new_var_int(2, 4);
    /// let product = model.mul(x, y);
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

    /// Create a new variable that holds the result of `x % y` (modulo operation).
    ///
    /// For the modulo operation `x % y = result`:
    /// - If y > 0: result is in range [0, y-1]  
    /// - If y < 0: result is in range [y+1, 0]
    /// - If y contains 0, the constraint may fail during solving
    ///
    /// # Examples
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(10, 20);
    /// let y = model.new_var_int(3, 7);
    /// let remainder = model.modulo(x, y);
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
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(-10, 5);
    /// let abs_x = model.abs(x);
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
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(1, 10);
    /// let y = model.new_var_int(5, 15);
    /// let z = model.new_var_int(3, 8);
    /// let minimum = model.min(&[x, y, z]);
    /// ```
    pub fn min(&mut self, vars: &[VarId]) -> VarId {
        if vars.is_empty() {
            panic!("Cannot compute minimum of empty variable list");
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

        let result_min = min_of_mins.unwrap();
        let result_max = min_of_maxs.unwrap();

        let result = self.new_var_unchecked(result_min, result_max);
        let _p = self.props.min(vars.to_vec(), result);
        result
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
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(1, 10);
    /// let y = model.new_var_int(5, 15);
    /// let z = model.new_var_int(3, 8);
    /// let maximum = model.max(&[x, y, z]);
    /// ```
    pub fn max(&mut self, vars: &[VarId]) -> VarId {
        if vars.is_empty() {
            panic!("Cannot compute maximum of empty variable list");
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

        let result_min = max_of_mins.unwrap();
        let result_max = max_of_maxs.unwrap();

        let result = self.new_var_unchecked(result_min, result_max);
        let _p = self.props.max(vars.to_vec(), result);
        result
    }

    /// Create a new variable that holds the result of `x / y` (division).
    ///
    /// For the division operation `x / y = result`:
    /// - If y contains 0, the constraint may fail during solving
    /// - Division by values very close to 0 is also avoided for numerical stability
    /// - The result may be converted to float even for integer inputs
    ///
    /// # Examples
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(10, 20);
    /// let y = model.new_var_int(2, 5);
    /// let quotient = model.div(x, y);
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

    /// Create an expression of the sum of a slice of views.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let vars: Vec<_> = model.new_vars_int(3, 1, 10).collect();
    /// let total = model.sum(&vars);
    /// ```
    pub fn sum(&mut self, xs: &[impl View]) -> VarId {
        self.sum_iter(xs.iter().copied())
    }

    /// Create an expression of the sum of an iterator of views.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let vars: Vec<_> = model.new_vars_int(3, 1, 10).collect();
    /// let total = model.sum_iter(vars.iter().copied());
    /// ```
    pub fn sum_iter(&mut self, xs: impl IntoIterator<Item = impl View>) -> VarId {
        let xs: Vec<_> = xs.into_iter().collect();

        let min: Val = xs.iter().map(|x| x.min_raw(&self.vars)).sum();
        let max: Val = xs.iter().map(|x| x.max_raw(&self.vars)).sum();
        let s = self.new_var_unchecked(min, max);

        let _p = self.props.sum(xs, s);

        s
    }

    /// Declare two expressions to be equal.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(1, 10);
    /// let y = model.new_var_int(1, 10);
    /// model.equals(x, y);
    /// ```
    pub fn equals(&mut self, x: impl View, y: impl View) {
        let _p = self.props.equals(x, y);
    }

    /// Short name for equals constraint: `==`
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(1, 10);
    /// let y = model.new_var_int(1, 10);
    /// model.eq(x, y);
    /// ```
    pub fn eq(&mut self, x: impl View, y: impl View) {
        self.equals(x, y);
    }

    /// Declare two expressions to be not equal.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(1, 10);
    /// let y = model.new_var_int(1, 10);
    /// model.ne(x, y);
    /// ```
    pub fn ne(&mut self, x: impl View, y: impl View) {
        let _p = self.props.not_equals(x, y);
    }

    /// Declare constraint `x <= y`.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(1, 10);
    /// let y = model.new_var_int(5, 15);
    /// model.le(x, y);
    /// ```
    pub fn le(&mut self, x: impl View, y: impl View) {
        let _p = self.props.less_than_or_equals(x, y);
    }

    /// Declare constraint `x < y`.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(1, 10);
    /// model.lt(x, Val::int(5));
    /// ```
    pub fn lt(&mut self, x: impl View, y: impl View) {
        //let mut events = Vec::new();
        //let ctx = Context::new(&mut self.vars, &mut events);
        let _p = self.props.less_than(x, y);
    }

    /// Declare constraint `x >= y`.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(5, 15);
    /// let y = model.new_var_int(1, 10);
    /// model.ge(x, y);
    /// ```
    pub fn ge(&mut self, x: impl View, y: impl View) {
        let _p = self.props.greater_than_or_equals(x, y);
    }

    /// Declare constraint `x > y`.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(1, 10);
    /// model.gt(x, float(2.5));
    /// ```
    pub fn gt(&mut self, x: impl View, y: impl View) {
        //let mut events = Vec::new();
        //let ctx = Context::new(&mut self.vars, &mut events);
        let _p = self.props.greater_than(x, y);
    }

    /// Declare all-different constraint: all variables must have distinct values.
    /// This is more efficient than adding pairwise not-equals constraints.
    /// 
    /// **Note**: This constraint is designed for integer variables with discrete domains.
    /// Using it with floating-point variables is not recommended due to precision issues
    /// and the continuous nature of float domains.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let vars: Vec<_> = model.new_vars_int(4, 1, 4).collect();
    /// model.all_different(vars);
    /// ```
    pub fn all_different(&mut self, vars: Vec<VarId>) {
        let _p = self.props.all_different(vars);
    }

    /// Create a new variable that holds the result of a boolean AND operation.
    /// 
    /// The boolean AND operation computes `result = a AND b AND c AND ...`:
    /// - All variables are treated as boolean: 0 = false, non-zero = true
    /// - Result is 1 if and only if all operands are non-zero
    /// - Result is 0 if any operand is 0
    ///
    /// # Examples
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let a = model.new_var_int(0, 1);
    /// let b = model.new_var_int(0, 1);
    /// let c = model.new_var_int(0, 1);
    /// let result = model.bool_and(&[a, b, c]);
    /// ```
    pub fn bool_and(&mut self, operands: &[VarId]) -> VarId {
        if operands.is_empty() {
            // Empty AND is typically true
            return self.new_var_int(1, 1);
        }

        // Result is boolean (0 or 1)
        let result = self.new_var_int(0, 1);
        let _p = self.props.bool_and(operands.to_vec(), result);
        result
    }

    /// Create a new variable that holds the result of a boolean OR operation.
    /// 
    /// The boolean OR operation computes `result = a OR b OR c OR ...`:
    /// - All variables are treated as boolean: 0 = false, non-zero = true
    /// - Result is 1 if any operand is non-zero
    /// - Result is 0 if and only if all operands are 0
    ///
    /// # Examples
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let a = model.new_var_int(0, 1);
    /// let b = model.new_var_int(0, 1);
    /// let c = model.new_var_int(0, 1);
    /// let result = model.bool_or(&[a, b, c]);
    /// ```
    pub fn bool_or(&mut self, operands: &[VarId]) -> VarId {
        if operands.is_empty() {
            // Empty OR is typically false
            return self.new_var_int(0, 0);
        }

        // Result is boolean (0 or 1)
        let result = self.new_var_int(0, 1);
        let _p = self.props.bool_or(operands.to_vec(), result);
        result
    }

    /// Create a new variable that holds the result of a boolean NOT operation.
    /// 
    /// The boolean NOT operation computes `result = NOT operand`:
    /// - Variables are treated as boolean: 0 = false, non-zero = true
    /// - Result is 1 if operand is 0
    /// - Result is 0 if operand is non-zero
    ///
    /// # Examples
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let a = model.new_var_int(0, 1);
    /// let result = model.bool_not(a);
    /// ```
    pub fn bool_not(&mut self, operand: VarId) -> VarId {
        // Result is boolean (0 or 1)
        let result = self.new_var_int(0, 1);
        let _p = self.props.bool_not(operand, result);
        result
    }

    /// Find assignment that minimizes objective expression while satisfying all constraints.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(1, 10);
    /// model.gt(x, Val::int(3));
    /// let solution = model.minimize(x);
    /// ```
    #[must_use]
    pub fn minimize(self, objective: impl View) -> Option<Solution> {
        self.minimize_and_iterate(objective).last()
    }

    /// Find assignment that minimizes objective expression with callback to capture solving statistics.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(1, 10);
    /// let solution = model.minimize_with_callback(x, |stats| {
    ///     println!("Propagations: {}", stats.propagation_count);
    /// });
    /// ```
    #[must_use]
    pub fn minimize_with_callback<F>(self, objective: impl View, callback: F) -> Option<Solution>
    where
        F: FnOnce(&crate::solution::SolveStats),
    {
        // First try specialized optimization (Step 2.4 precision handling)
        match self.try_optimization_minimize(&objective) {
            Some(solution) => {
                // Optimization succeeded - report zero search statistics since no search was needed
                let stats = crate::solution::SolveStats {
                    propagation_count: 0,
                    node_count: 0,
                };
                callback(&stats);
                Some(solution)
            }
            None => {
                // Optimization failed or not applicable - fall back to traditional search
                let (vars, props) = self.prepare_for_search();

                let mut search_iter = search(vars, props, mode::Minimize::new(objective));
                let mut last_solution = None;
                let mut current_count = 0;

                // Iterate through all solutions to find the optimal one
                while let Some(solution) = search_iter.next() {
                    last_solution = Some(solution);
                    // Capture the count each iteration, as it might get lost when iterator is consumed
                    current_count = search_iter.get_propagation_count();
                }

                let stats = crate::solution::SolveStats {
                    propagation_count: current_count,
                    node_count: search_iter.get_node_count(),
                };

                callback(&stats);
                last_solution
            }
        }
    }

    /// Enumerate assignments that satisfy all constraints, while minimizing objective expression.
    ///
    /// The order in which assignments are yielded is not stable.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(1, 5);
    /// let solutions: Vec<_> = model.minimize_and_iterate(x).collect();
    /// ```
    pub fn minimize_and_iterate(self, objective: impl View) -> impl Iterator<Item = Solution> {
        // First try specialized optimization before falling back to search
        match self.try_optimization_minimize(&objective) {
            Some(solution) => {
                // Optimization succeeded - return a single-element iterator with the optimal solution
                Box::new(std::iter::once(solution)) as Box<dyn Iterator<Item = Solution>>
            }
            None => {
                // Optimization failed or not applicable - fall back to traditional search
                let (vars, props) = self.prepare_for_search();
                Box::new(search(vars, props, mode::Minimize::new(objective))) as Box<dyn Iterator<Item = Solution>>
            }
        }
    }

    /// Enumerate assignments that satisfy all constraints, while minimizing objective expression, with callback.
    ///
    /// The callback is called with final statistics after all solutions are found.
    /// Returns a vector of all solutions found during the search.
    pub fn minimize_and_iterate_with_callback<F>(
        self,
        objective: impl View,
        callback: F,
    ) -> Vec<Solution>
    where
        F: FnOnce(&crate::solution::SolveStats),
    {
        // First try specialized optimization (Step 2.4 precision handling)
        match self.try_optimization_minimize(&objective) {
            Some(solution) => {
                // Optimization succeeded - report zero search statistics since no search was needed
                let stats = crate::solution::SolveStats {
                    propagation_count: 0,
                    node_count: 0,
                };
                callback(&stats);
                vec![solution]
            }
            None => {
                // Optimization failed or not applicable - fall back to traditional search
                let (vars, props) = self.prepare_for_search();

                let mut search_iter = search(vars, props, mode::Minimize::new(objective));
                let mut solutions = Vec::new();
                let mut current_count = 0;

                // Collect all solutions manually and capture count during iteration
                while let Some(solution) = search_iter.next() {
                    solutions.push(solution);
                    // Capture the count each iteration, as it might get lost when iterator is consumed
                    current_count = search_iter.get_propagation_count();
                }

                let stats = crate::solution::SolveStats {
                    propagation_count: current_count,
                    node_count: search_iter.get_node_count(),
                };

                callback(&stats);
                solutions
            }
        }
    }

    /// Find assignment that maximizes objective expression while satisfying all constraints.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(1, 10);
    /// model.lt(x, Val::int(8));
    /// let solution = model.maximize(x);
    /// ```
    #[must_use]
    pub fn maximize(self, objective: impl View) -> Option<Solution> {
        // First try specialized optimization before falling back to opposite+minimize pattern
        match self.try_optimization_maximize(&objective) {
            Some(solution) => Some(solution),
            None => self.minimize(objective.opposite()),
        }
    }

    /// Find assignment that maximizes objective expression with callback to capture solving statistics.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(1, 10);
    /// let solution = model.maximize_with_callback(x, |stats| {
    ///     println!("Nodes explored: {}", stats.node_count);
    /// });
    /// ```
    #[must_use]
    pub fn maximize_with_callback<F>(self, objective: impl View, callback: F) -> Option<Solution>
    where
        F: FnOnce(&crate::solution::SolveStats),
    {
        // First try specialized optimization (Step 2.4 precision handling)
        match self.try_optimization_maximize(&objective) {
            Some(solution) => {
                // Optimization succeeded - report zero search statistics since no search was needed
                let stats = crate::solution::SolveStats {
                    propagation_count: 0,
                    node_count: 0,
                };
                callback(&stats);
                Some(solution)
            }
            None => {
                // Optimization failed or not applicable - fall back to traditional search
                self.minimize_with_callback(objective.opposite(), callback)
            }
        }
    }

    /// Enumerate assignments that satisfy all constraints, while maximizing objective expression.
    ///
    /// The order in which assignments are yielded is not stable.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(1, 5);
    /// let solutions: Vec<_> = model.maximize_and_iterate(x).collect();
    /// ```
    pub fn maximize_and_iterate(self, objective: impl View) -> impl Iterator<Item = Solution> {
        // First try specialized optimization before falling back to search
        match self.try_optimization_maximize(&objective) {
            Some(solution) => {
                // Optimization succeeded - return a single-element iterator with the optimal solution
                Box::new(std::iter::once(solution)) as Box<dyn Iterator<Item = Solution>>
            }
            None => {
                // Optimization failed or not applicable - fall back to traditional search
                Box::new(self.minimize_and_iterate(objective.opposite())) as Box<dyn Iterator<Item = Solution>>
            }
        }
    }

    /// Enumerate assignments that satisfy all constraints, while maximizing objective expression, with callback.
    ///
    /// The callback is called with final statistics after all solutions are found.
    /// Returns a vector of all solutions found during the search.
    pub fn maximize_and_iterate_with_callback<F>(
        self,
        objective: impl View,
        callback: F,
    ) -> Vec<Solution>
    where
        F: FnOnce(&crate::solution::SolveStats),
    {
        self.minimize_and_iterate_with_callback(objective.opposite(), callback)
    }

    /// Get reference to variables for analysis (used by optimization module)
    #[doc(hidden)]
    pub fn get_vars(&self) -> &Vars {
        &self.vars
    }

    /// Get reference to propagators for analysis (used by optimization module)
    #[doc(hidden)]
    pub fn get_props(&self) -> &Propagators {
        &self.props
    }

    /// Validate that all integer variable domains fit within the u16 optimization range.
    ///
    /// This method checks that all integer variables have domains that can be represented
    /// using u16 optimization (domain size ≤ 65535). Since we've already replaced VarI
    /// with VarSparse in the new_var_with_bounds method, this validation mainly serves
    /// as a safety check and provides clear error messages for invalid domain sizes.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if validation succeeds, or `Err(String)` with error details if validation fails.
    pub fn validate(&self) -> Result<(), String> {
        for (i, var) in self.vars.iter_with_indices() {
            match var {
                Var::VarI(sparse_set) => {
                    let domain_size = sparse_set.universe_size();
                    if domain_size > 65535 {
                        return Err(format!(
                            "Variable {} has domain size {} which exceeds the maximum of 65535 for u16 optimization. \
                            Consider using smaller domains or splitting large domains into multiple variables.",
                            i, domain_size
                        ));
                    }

                    // Additional validation: check if domain range is reasonable
                    let min_val = sparse_set.min_universe_value();
                    let max_val = sparse_set.max_universe_value();
                    let actual_range = max_val - min_val + 1;

                    if actual_range < 0 || actual_range > 65535 {
                        return Err(format!(
                            "Variable {} has invalid domain range [{}, {}] which results in {} values. \
                            Domain range must be positive and ≤ 65535.",
                            i, min_val, max_val, actual_range
                        ));
                    }
                }
                Var::VarF { .. } => {
                    // Float variables use interval representation, no validation needed
                }
            }
        }
        Ok(())
    }

    #[doc(hidden)]
    /// Optimize constraint processing order based on constraint characteristics.
    ///
    /// This method analyzes constraints (particularly AllDifferent) and reorders them
    /// to prioritize constraints with more fixed values, which tend to propagate more effectively.
    /// This can significantly improve solving performance by doing more effective propagation earlier.
    ///
    /// Should be called after all constraints are added but before solving.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let vars: Vec<_> = model.new_vars_int(4, 1, 4).collect();
    /// model.all_different(vars);
    /// model.optimize_constraint_order();
    /// ```
    pub fn optimize_constraint_order(&mut self) -> &mut Self {
        // Since we can't downcast trait objects easily, we'll implement this optimization
        // at the Propagators level by adding a method there
        self.props.optimize_alldiff_order(&self.vars);
        self
    }

    /// Search for assignment that satisfies all constraints within bounds of decision variables.
    /// 
    /// This method automatically tries optimization algorithms for suitable problems
    /// before falling back to traditional constraint propagation search.
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(1, 10);
    /// let y = model.new_var_int(1, 10);
    /// model.ne(x, y);
    /// let solution = model.solve();
    /// ```
    #[must_use]
    pub fn solve(self) -> Option<Solution> {
        // Step 6.5: Try hybrid optimization for mixed problems first
        match self.try_hybrid_solve() {
            Some(solution) => Some(solution),
            None => {
                // Fall back to traditional constraint propagation search
                self.enumerate().next()
            }
        }
    }
    
    /// Step 6.5: Try hybrid optimization approach for constraint satisfaction
    /// Returns Some(solution) if hybrid solver succeeds, None if should fall back to search
    fn try_hybrid_solve(&self) -> Option<Solution> {
        // Create a dummy objective (we're not optimizing, just solving constraints)
        // Use the first variable if available, otherwise return None
        let first_var = match self.vars.iter().next() {
            Some(_) => {
                // Create VarId for the first variable
                let first_var_id = crate::optimization::model_integration::index_to_var_id(0);
                first_var_id
            },
            None => return None, // No variables to solve
        };
        
        // Try optimization with the dummy objective
        match self.optimization_router.try_minimize(&self.vars, &self.props, &first_var) {
            OptimizationAttempt::Success(solution) => Some(solution),
            OptimizationAttempt::Fallback(_) => None, // Fall back to search
            OptimizationAttempt::Infeasible(_) => None, // No solution exists
        }
    }

    /// Search for assignment with a callback to capture solving statistics.
    ///
    /// The callback receives the solving statistics when the search completes.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(1, 10);
    /// let solution = model.solve_with_callback(|stats| {
    ///     println!("Search completed with {} propagations", stats.propagation_count);
    /// });
    /// ```
    #[must_use]
    pub fn solve_with_callback<F>(self, callback: F) -> Option<Solution>
    where
        F: FnOnce(&crate::solution::SolveStats),
    {
        // Run the solving process
        let (vars, props) = self.prepare_for_search();

        // Create a search and run it to completion to capture final stats
        let mut search_iter = search(vars, props, mode::Enumerate);
        let result = search_iter.next();

        // Get the final stats from the search
        let final_propagation_count = search_iter.get_propagation_count();
        let final_node_count = search_iter.get_node_count();

        let stats = crate::solution::SolveStats {
            propagation_count: final_propagation_count,
            node_count: final_node_count,
        };

        callback(&stats);
        result
    }

    #[doc(hidden)]
    /// Internal helper that automatically optimizes constraints before search.
    /// This ensures all solving methods benefit from constraint optimization.
    fn prepare_for_search(mut self) -> (crate::vars::Vars, crate::props::Propagators) {
        // Automatically optimize constraint order for better performance
        self.optimize_constraint_order();
        (self.vars, self.props)
    }

    /// Try to solve minimization using specialized optimization algorithms
    /// Returns Some(solution) if optimization succeeds, None if should fall back to search
    fn try_optimization_minimize(&self, objective: &impl View) -> Option<Solution> {
        // Attempt optimization using the router
        match self.optimization_router.try_minimize(&self.vars, &self.props, objective) {
            OptimizationAttempt::Success(solution) => Some(solution),
            OptimizationAttempt::Fallback(_reason) => {
                // Optimization not applicable - let search handle it
                None
            },
            OptimizationAttempt::Infeasible(_reason) => {
                // Problem is infeasible - no solution exists
                None
            },
        }
    }

    /// Try to solve maximization using specialized optimization algorithms  
    /// Returns Some(solution) if optimization succeeds, None if should fall back to search
    fn try_optimization_maximize(&self, objective: &impl View) -> Option<Solution> {
        // Attempt optimization using the router
        match self.optimization_router.try_maximize(&self.vars, &self.props, objective) {
            OptimizationAttempt::Success(solution) => Some(solution),
            OptimizationAttempt::Fallback(_reason) => {
                // Optimization not applicable - let search handle it
                None
            },
            OptimizationAttempt::Infeasible(_reason) => {
                // Problem is infeasible - no solution exists
                None
            },
        }
    }

    /// Extract a single float variable from the objective (Step 2.3.1 heuristic)
    fn extract_single_float_variable(&self, _objective: &impl View) -> Option<VarId> {
        // Simple heuristic for Step 2.3.1: if exactly one float variable exists, use it
        // TODO: In Step 2.3.2, implement proper View analysis to extract variable from objective
        
        let mut float_vars = Vec::new();
        for (index, var) in self.vars.iter_with_indices() {
            if matches!(var, crate::vars::Var::VarF(_)) {
                // Convert index to VarId using the helper from model_integration
                let var_id = crate::optimization::model_integration::index_to_var_id(index);
                float_vars.push(var_id);
            }
        }
        
        // Simple heuristic: if exactly one float variable, optimize it
        if float_vars.len() == 1 {
            Some(float_vars[0])
        } else {
            None // Fallback to search for complex cases
        }
    }

    /// Create a Solution from an OptimizationResult
    fn create_solution_from_optimization(&self, result: crate::optimization::float_direct::OptimizationResult) -> Result<Solution, String> {
        // Create a solution vector with all variable values
        let mut solution_values = Vec::new();
        
        for (index, var) in self.vars.iter_with_indices() {
            match var {
                crate::vars::Var::VarF(interval) => {
                    // For the optimized variable, use the optimal value
                    // For others, use their current bound (this is a simplification)
                    if result.success {
                        let var_id = crate::optimization::model_integration::index_to_var_id(index);
                        match &result.outcome {
                            crate::optimization::float_direct::OptimizationOutcome::Success { variable_id, .. } => {
                                // Compare VarIds by converting both to usize for comparison
                                let result_var_index = crate::optimization::model_integration::var_id_to_index(*variable_id);
                                if index == result_var_index {
                                    solution_values.push(crate::vars::Val::float(result.optimal_value));
                                } else {
                                    solution_values.push(crate::vars::Val::float(interval.min));
                                }
                            },
                            _ => solution_values.push(crate::vars::Val::float(interval.min)),
                        }
                    } else {
                        solution_values.push(crate::vars::Val::float(interval.min));
                    }
                },
                crate::vars::Var::VarI(domain) => {
                    solution_values.push(crate::vars::Val::int(domain.min()));
                }
            }
        }
        
        Ok(Solution::from(solution_values))
    }

    /// Enumerate all assignments that satisfy all constraints.
    ///
    /// The order in which assignments are yielded is not stable.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(1, 3);
    /// let y = model.new_var_int(1, 3);
    /// model.ne(x, y);
    /// let solutions: Vec<_> = model.enumerate().collect();
    /// ```
    pub fn enumerate(self) -> impl Iterator<Item = Solution> {
        let (vars, props) = self.prepare_for_search();
        search(vars, props, mode::Enumerate)
    }

    /// Enumerate all assignments that satisfy all constraints with callback to capture solving statistics.
    ///
    /// The callback is called with final statistics after all solutions are found.
    /// Returns a vector of all solutions found during the search.
    pub fn enumerate_with_callback<F>(self, callback: F) -> Vec<Solution>
    where
        F: FnOnce(&crate::solution::SolveStats),
    {
        let (vars, props) = self.prepare_for_search();

        let mut search_iter = search(vars, props, mode::Enumerate);
        let mut solutions = Vec::new();

        // CRITICAL: Get the stats BEFORE calling any next() methods,
        // because Search::Done(Some(space)) becomes Search::Done(None) after the first next()
        let final_count = search_iter.get_propagation_count();
        let final_node_count = search_iter.get_node_count();

        // Collect all solutions
        while let Some(solution) = search_iter.next() {
            solutions.push(solution);
        }

        let stats = crate::solution::SolveStats {
            propagation_count: final_count,
            node_count: final_node_count,
        };

        callback(&stats);
        solutions
    }
}

impl Index<VarId> for Model {
    type Output = Var;

    fn index(&self, index: VarId) -> &Self::Output {
        &self.vars[index]
    }
}
