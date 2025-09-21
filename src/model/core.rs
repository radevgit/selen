use crate::prelude::*;
use crate::variables::domain::float_interval::{DEFAULT_FLOAT_PRECISION_DIGITS, precision_to_step_size};
use crate::optimization::model_integration::{OptimizationRouter, OptimizationAttempt};
use crate::core::error::{SolverError, SolverResult};
use std::ops::Index;

#[doc(hidden)]
#[derive(Debug)]
pub struct Model {
    pub vars: Vars,
    pub props: Propagators,
    /// Precision for float variables (decimal places)
    pub float_precision_digits: i32,
    /// Optimization router for efficient algorithm selection
    optimization_router: OptimizationRouter,
    /// Configuration for solver behavior
    config: crate::utils::config::SolverConfig,
    /// Memory tracking: estimated memory used by variables and constraints (in bytes)
    estimated_memory_bytes: u64,
    /// Memory limit exceeded flag
    memory_limit_exceeded: bool,
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
    /// let mut m = Model::with_float_precision(4); // 4 decimal places
    /// let var = m.float(0.0, 1.0);
    /// ```
    #[must_use]
    pub fn with_float_precision(precision_digits: i32) -> Self {
        let config = crate::utils::config::SolverConfig::default()
            .with_float_precision(precision_digits);
        Self {
            vars: Vars::default(),
            props: Propagators::default(),
            float_precision_digits: precision_digits,
            optimization_router: OptimizationRouter::new(),
            config,
            estimated_memory_bytes: 0,
            memory_limit_exceeded: false,
        }
    }

    /// Create a new model with a configuration
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let config = SolverConfig::default().with_float_precision(4);
    /// let mut m = Model::with_config(config);
    /// let var = m.float(0.0, 1.0);
    /// ```
    #[must_use]
    pub fn with_config(config: crate::utils::config::SolverConfig) -> Self {
        Self {
            vars: Vars::default(),
            props: Propagators::default(),
            float_precision_digits: config.float_precision_digits,
            optimization_router: OptimizationRouter::new(),
            config,
            estimated_memory_bytes: 0,
            memory_limit_exceeded: false,
        }
    }

    /// Get the current float precision setting.
    ///
    /// Returns the number of decimal places used for floating-point precision
    /// in this model. This affects the granularity of float variable domains
    /// and optimization algorithms.
    ///
    /// # Returns
    /// Number of decimal places for float precision (e.g., 3 = 0.001 granularity)
    ///
    /// # Example
    /// ```
    /// use cspsolver::prelude::*;
    /// let m = Model::with_float_precision(4);
    /// assert_eq!(m.float_precision_digits(), 4);
    /// ```
    pub fn float_precision_digits(&self) -> i32 {
        self.float_precision_digits
    }

    /// Get the step size corresponding to the current float precision.
    ///
    /// Returns the minimum representable difference between float values
    /// based on the model's precision setting. For example, with 3 decimal
    /// places, the step size is 0.001.
    ///
    /// # Returns
    /// The step size as a floating-point value
    ///
    /// # Example
    /// ```
    /// use cspsolver::prelude::*;
    /// let m = Model::with_float_precision(2);
    /// assert_eq!(m.float_step_size(), 0.01);
    /// ```
    pub fn float_step_size(&self) -> f64 {
        precision_to_step_size(self.float_precision_digits)
    }

    /// Get the solver configuration.
    ///
    /// Returns a reference to the current solver configuration, which contains
    /// settings for timeouts, memory limits, precision, and other solver behavior.
    ///
    /// # Returns
    /// Reference to the `SolverConfig` for this model
    ///
    /// # Example
    /// ```
    /// use cspsolver::prelude::*;
    /// let m = Model::default();
    /// let config = m.config();
    /// println!("Float precision: {}", config.float_precision_digits);
    /// ```
    pub fn config(&self) -> &crate::utils::config::SolverConfig {
        &self.config
    }

    /// Get timeout as Duration for search operations
    fn timeout_duration(&self) -> Option<std::time::Duration> {
        self.config.timeout_seconds.map(std::time::Duration::from_secs)
    }

    /// Get memory limit in MB for search operations
    fn memory_limit_mb(&self) -> Option<u64> {
        self.config.max_memory_mb
    }
    
    /// Get the current estimated memory usage in bytes
    pub fn estimated_memory_bytes(&self) -> u64 {
        self.estimated_memory_bytes
    }
    
    /// Get the current estimated memory usage in MB
    pub fn estimated_memory_mb(&self) -> f64 {
        self.estimated_memory_bytes as f64 / (1024.0 * 1024.0)
    }
    
    /// Check if memory limit has been exceeded
    pub fn memory_limit_exceeded(&self) -> bool {
        self.memory_limit_exceeded
    }
    
    /// Add to estimated memory usage and check limits
    fn add_memory_usage(&mut self, bytes: u64) -> Result<(), SolverError> {
        self.estimated_memory_bytes += bytes;
        
        if let Some(limit_mb) = self.config.max_memory_mb {
            let limit_bytes = limit_mb * 1024 * 1024;
            if self.estimated_memory_bytes > limit_bytes {
                self.memory_limit_exceeded = true;
                return Err(SolverError::MemoryLimit {
                    usage_mb: Some(self.estimated_memory_mb() as usize),
                    limit_mb: Some(limit_mb as usize),
                });
            }
        }
        
        Ok(())
    }
    
    /// Get detailed memory breakdown for analysis  
    pub fn memory_breakdown(&self) -> String {
        format!(
            "Memory Breakdown:\n\
             - Variables: {:.3} MB\n\
             - Total estimated: {:.3} MB\n\
             - Limit: {:?} MB\n\
             - Note: Constraint overhead not included in estimates",
            self.estimated_memory_bytes as f64 / (1024.0 * 1024.0),
            self.estimated_memory_mb(),
            self.config.max_memory_mb
        )
    }
    
    /// Estimate memory usage for a variable with improved accuracy
    fn estimate_variable_memory(&self, min: Val, max: Val) -> u64 {
        match (min, max) {
            (Val::ValI(min_i), Val::ValI(max_i)) => {
                // Integer variable: SparseSet structure overhead + domain representation
                if min_i > max_i {
                    // Invalid range - return minimal memory estimate
                    return 96; // Just base overhead
                }
                
                let domain_size = (max_i - min_i + 1) as u64;
                
                // Base SparseSet structure overhead (dense/sparse arrays, metadata)
                let base_cost = 96; // More realistic estimate including Vec overhead
                
                let domain_cost = if domain_size > 1000 {
                    // Large domains use sparse representation
                    // Two Vec<u32> with capacity approximately equal to domain size
                    let vec_overhead = 24 * 2; // Vec metadata for dense/sparse arrays
                    let data_cost = domain_size.saturating_mul(4).saturating_mul(2); // Prevent overflow
                    vec_overhead + data_cost / 8 // Amortized for typical sparsity
                } else {
                    // Small domains use dense representation
                    let vec_overhead = 24 * 2;
                    let data_cost = domain_size.saturating_mul(4).saturating_mul(2); // Prevent overflow
                    vec_overhead + data_cost
                };
                
                base_cost + domain_cost
            }
            (Val::ValF(_), Val::ValF(_)) => {
                // Float variable: FloatInterval structure
                // Contains: min (f64), max (f64), step (f64) = 24 bytes
                // Plus wrapper overhead and alignment
                let base_cost = 32; // FloatInterval struct
                let wrapper_cost = 32; // Var enum wrapper + alignment
                base_cost + wrapper_cost
            }
            _ => {
                // Mixed types: treated as float variable
                64
            }
        }
    }

    #[doc(hidden)]
    /// Get access to constraint registry for debugging/analysis
    pub fn get_constraint_registry(&self) -> &crate::optimization::constraint_metadata::ConstraintRegistry {
        self.props.get_constraint_registry()
    }

    #[doc(hidden)]
    /// Create a new decision variable, with the provided domain bounds.
    ///
    /// Both lower and upper bounds are included in the domain.
    /// In case `max < min` the bounds will be swapped.
    /// We don't want to deal with "unwrap" every time
    /// 
    /// **Note**: This is a low-level method. Use `int()`, `float()`, or `bool()` instead.
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut m = Model::default();
    /// let var = m.new_var(Val::int(1), Val::int(10));
    /// ```
    pub fn new_var(&mut self, min: Val, max: Val) -> VarId {
        if min < max {
            self.new_var_unchecked(min, max)
        } else {
            self.new_var_unchecked(max, min)
        }
    }

    #[doc(hidden)]
    /// Create new decision variables, with the provided domain bounds.
    ///
    /// All created variables will have the same starting domain bounds.
    /// Both lower and upper bounds are included in the domain.
    /// In case `max < min` the bounds will be swapped.
    /// 
    /// **Note**: This is a low-level method. Use specific variable creation methods instead.
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut m = Model::default();
    /// let vars: Vec<_> = m.new_vars(3, Val::int(0), Val::int(5)).collect();
    /// ```
    pub fn new_vars(&mut self, n: usize, min: Val, max: Val) -> impl Iterator<Item = VarId> + '_ {
        let (actual_min, actual_max) = if min < max { (min, max) } else { (max, min) };
        std::iter::repeat_with(move || self.new_var_unchecked(actual_min, actual_max)).take(n)
    }

    #[doc(hidden)]
    /// Create new integer decision variables, with the provided domain bounds.
    ///
    /// Both lower and upper bounds are included in the domain.
    /// In case `max < min` the bounds will be swapped.
    /// 
    /// # Examples
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut m = Model::default();
    /// let vars: Vec<_> = m.int_vars(5, 0, 9).collect();
    /// ```
    pub fn int_vars(
        &mut self,
        n: usize,
        min: i32,
        max: i32,
    ) -> impl Iterator<Item = VarId> + '_ {
        self.new_vars(n, Val::ValI(min), Val::ValI(max))
    }

    /// Create an integer variable with a custom domain from specific values.
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
    /// use cspsolver::prelude::*;
    /// let mut m = Model::default();
    /// 
    /// // Variable that can only be prime numbers
    /// let prime = m.ints(vec![2, 3, 5, 7, 11, 13]);
    /// 
    /// // Variable for days of week (1=Monday, 7=Sunday)  
    /// let weekday = m.ints(vec![1, 2, 3, 4, 5, 6, 7]);
    /// 
    /// // Non-contiguous range
    /// let sparse = m.ints(vec![1, 5, 10, 50, 100]);
    /// 
    /// post!(m, prime != weekday);
    /// ```
    pub fn ints(&mut self, values: Vec<i32>) -> VarId {
        self.props.on_new_var();
        self.vars.new_var_with_values(values)
    }

    #[doc(hidden)]
    /// Create new float decision variables, with the provided domain bounds.
    ///
    /// Both lower and upper bounds are included in the domain.
    /// In case `max < min` the bounds will be swapped.
    /// 
    /// # Examples
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut m = Model::default();
    /// let vars: Vec<_> = m.float_vars(3, 0.0, 1.0).collect();
    /// ```
    pub fn float_vars(
        &mut self,
        n: usize,
        min: f64,
        max: f64,
    ) -> impl Iterator<Item = VarId> + '_ {
        self.new_vars(n, Val::ValF(min), Val::ValF(max))
    }

    /// Create a boolean variable (0 or 1).
    ///
    /// Creates a variable that can only take values 0 or 1, useful for representing
    /// boolean logic, flags, or binary decisions. Equivalent to `m.int(0, 1)` but
    /// more semantically clear for boolean use cases.
    /// 
    /// # Returns
    /// A `VarId` that can take values 0 (false) or 1 (true)
    ///
    /// # Example
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut m = Model::default();
    /// let flag = m.bool();          // 0 or 1
    /// let enabled = m.bool();       // 0 or 1
    /// 
    /// // Use in constraints
    /// post!(m, flag != enabled);    // Flags must be different
    /// 
    /// // Boolean logic (using model methods)
    /// let result = m.bool_and(&[flag, enabled]);  // result = flag AND enabled
    /// ```
    pub fn bool(&mut self) -> VarId {
        self.int(0, 1)
    }

    #[doc(hidden)]
    /// Create a new binary decision variable.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut m = Model::default();
    /// let var = m.new_var_binary();
    /// ```
    pub fn new_var_binary(&mut self) -> VarIdBin {
        VarIdBin(self.new_var_unchecked(Val::ValI(0), Val::ValI(1)))
    }

    #[doc(hidden)]
    /// Create new binary decision variables.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut m = Model::default();
    /// let vars: Vec<_> = m.new_vars_binary(4).collect();
    /// ```
    pub fn new_vars_binary(&mut self, n: usize) -> impl Iterator<Item = VarIdBin> + '_ {
        std::iter::repeat_with(|| self.new_var_binary()).take(n)
    }

    // === SHORT VARIABLE CREATION METHODS ===
    
    /// Create an integer variable with specified bounds.
    /// 
    /// Creates a variable that can take any integer value between `min` and `max` (inclusive).
    ///
    /// # Arguments
    /// * `min` - Minimum value for the variable (inclusive)
    /// * `max` - Maximum value for the variable (inclusive)
    ///
    /// # Note
    /// If `min > max`, an invalid variable will be created that will be caught during validation.
    ///
    /// # Example
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(1, 10);     // Variable from 1 to 10
    /// let y = m.int(-5, 15);    // Variable from -5 to 15
    /// ```
    pub fn int(&mut self, min: i32, max: i32) -> VarId {
        // Create the variable with the bounds as given (don't auto-swap)
        // If min > max, this will create an invalid variable that validation will catch
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
    /// # Note
    /// If `min > max`, an invalid variable will be created that will be caught during validation.
    ///
    /// # Example
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.float(0.0, 10.0);    // Variable from 0.0 to 10.0
    /// let y = m.float(-1.5, 3.14);   // Variable from -1.5 to 3.14
    /// ```
    pub fn float(&mut self, min: f64, max: f64) -> VarId {
        // Create the variable with the bounds as given (don't auto-swap)
        // If min > max, this will create an invalid variable that validation will catch
        self.new_var_unchecked(Val::ValF(min), Val::ValF(max))
    }

    /// Create a binary variable (0 or 1).
    /// 
    /// Creates a boolean variable that can only take values 0 or 1.
    /// Equivalent to `m.int(0, 1)` but optimized for binary constraints.
    ///
    /// # Example
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut m = Model::default();
    /// let flag = m.binary();    // Variable that is 0 or 1
    /// ```
    pub fn binary(&mut self) -> VarIdBin {
        self.new_var_binary()
    }

    #[doc(hidden)]
    /// Create a new integer decision variable, with the provided domain bounds.
    ///
    /// Both lower and upper bounds are included in the domain.
    ///
    /// This function assumes that `min < max`.
    pub fn new_var_unchecked(&mut self, min: Val, max: Val) -> VarId {
        match self.new_var_checked(min, max) {
            Ok(var_id) => var_id,
            Err(error) => {
                // Memory limit exceeded during variable creation
                // Mark the model as invalid and panic to prevent memory explosion
                self.memory_limit_exceeded = true;
                
                panic!("Memory limit exceeded during variable creation: {}. \
                       Configured limit: {:?} MB. \
                       Current usage: {} MB. \
                       This prevents system memory exhaustion.", 
                       error, 
                       self.config.max_memory_mb,
                       self.estimated_memory_mb());
            }
        }
    }
    
    /// Create a new variable with memory limit checking
    fn new_var_checked(&mut self, min: Val, max: Val) -> Result<VarId, SolverError> {
        // Check if memory limit was already exceeded
        if self.memory_limit_exceeded {
            return Err(SolverError::MemoryLimit {
                usage_mb: Some(self.estimated_memory_mb() as usize),
                limit_mb: self.config.max_memory_mb.map(|x| x as usize),
            });
        }
        
        // Estimate memory needed for this variable
        let estimated_memory = self.estimate_variable_memory(min, max);
        
        // Check if adding this variable would exceed the limit
        self.add_memory_usage(estimated_memory)?;
        
        // Create the variable
        self.props.on_new_var();
        let step_size = self.float_step_size();
        let var_id = self.vars.new_var_with_bounds_and_step(min, max, step_size);
        
        Ok(var_id)
    }

    // ========================================================================
    // CONSTRAINT POSTING METHODS
    // ========================================================================
    //
    // This section contains all methods for posting constraints to the model.
    // These methods create new variables representing constraint results and
    // add the corresponding propagators to enforce the constraints.
    //
    // Methods are organized into logical groups:
    // - Mathematical Operations (add, sub, mul, div, modulo, abs)
    // - Global Constraints (min, max, sum, alldiff)  
    // - Boolean Operations (bool_and, bool_or, bool_not)

    // ------------------------------------------------------------------------
    // Mathematical Operations
    // ------------------------------------------------------------------------
    // Mathematical constraint methods (add, sub, mul, div, modulo, abs) have been 
    // moved to model/constraints.rs as part of the modularization effort.
    // Use: model.add(x, y), model.sub(x, y), model.mul(x, y), etc.

    // ------------------------------------------------------------------------
    // Global Constraints
    // ------------------------------------------------------------------------
    // Global constraint methods (min, max, sum, sum_iter) have been moved to 
    // model/constraints.rs as part of the modularization effort.
    // Use: model.min(&vars), model.max(&vars), model.sum(&vars), etc.

    // ------------------------------------------------------------------------
    // Boolean Operations
    // ------------------------------------------------------------------------

    // ═══════════════════════════════════════════════════════════════════════
    // 🔍 Boolean Operations → Moved to model/constraints.rs
    // ═══════════════════════════════════════════════════════════════════════
    
    // The following methods have been moved to model/constraints.rs:
    // - bool_and(&[VarId]) -> VarId  
    // - bool_or(&[VarId]) -> VarId
    // - bool_not(VarId) -> VarId

    // ========================================================================
    // SOLVING METHODS
    // ========================================================================
    //
    // This section contains methods for solving the constraint model:
    // - Optimization (minimize, maximize)
    // - Solution enumeration (solve, enumerate)
    // - Model analysis and validation

    /// Find assignment that minimizes objective expression while satisfying all constraints.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(1, 10);
    /// post!(m, x > 3);
    /// let solution = m.minimize(x);
    /// ```
    #[must_use]
    pub fn minimize(self, objective: impl View) -> SolverResult<Solution> {
        // First try specialized optimization (Step 2.4 precision handling)
        match self.try_optimization_minimize(&objective) {
            Some(mut solution) => {
                // Optimization succeeded - update with minimal stats since no search was performed
                solution.stats = crate::core::solution::SolveStats {
                    propagation_count: 0,
                    node_count: 0,
                    solve_time: std::time::Duration::ZERO,
                    variable_count: solution.stats.variable_count, // Preserve if already set
                    constraint_count: solution.stats.constraint_count, // Preserve if already set
                    peak_memory_mb: solution.stats.peak_memory_mb, // Preserve from optimization
                };
                Ok(solution)
            }
            None => {
                // Optimization failed or not applicable - fall back to traditional search
                let timeout = self.timeout_duration();
                let memory_limit = self.memory_limit_mb();
                let (vars, props) = self.prepare_for_search()?;

                // Capture counts before moving to search
                let var_count = vars.count();
                let constraint_count = props.count();

                let mut search_iter = search_with_timeout_and_memory(vars, props, mode::Minimize::new(objective), timeout, memory_limit);
                let mut last_solution = None;
                let mut current_count = 0;

                // Iterate through all solutions to find the optimal one
                while let Some(solution) = search_iter.next() {
                    last_solution = Some(solution);
                    // Capture the count each iteration, as it might get lost when iterator is consumed
                    current_count = search_iter.get_propagation_count();
                }

                let stats = crate::core::solution::SolveStats {
                    propagation_count: current_count,
                    node_count: search_iter.get_node_count(),
                    solve_time: search_iter.elapsed_time(),
                    variable_count: var_count,
                    constraint_count,
                    peak_memory_mb: search_iter.get_memory_usage_mb(), // Direct MB usage
                };
                
                // Check if search terminated due to timeout
                if search_iter.is_timed_out() {
                    let elapsed = search_iter.elapsed_time().as_secs_f64();
                    return Err(SolverError::timeout_with_context(elapsed, "optimization search"));
                }
                
                // Check if search terminated due to memory limit
                if search_iter.is_memory_limit_exceeded() {
                    let usage_mb = search_iter.get_memory_usage_mb();
                    let limit_mb = memory_limit.unwrap_or(0) as usize;
                    return Err(SolverError::memory_limit_with_context(usage_mb, limit_mb));
                }
                
                match last_solution {
                    Some(mut solution) => {
                        solution.stats = stats;
                        Ok(solution)
                    }
                    None => Err(SolverError::no_solution()),
                }
            }
        }
    }

    /// Enumerate assignments that minimize objective expression.
    /// 
    /// Enumerate assignments that satisfy all constraints, while minimizing objective expression.
    /// Each yielded solution includes embedded statistics.
    ///
    /// The order in which assignments are yielded is not stable.
    /// 
    /// # Example
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(1, 5);
    /// let solutions: Vec<_> = m.minimize_and_iterate(x).collect();
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
                let timeout = self.timeout_duration();
                match self.prepare_for_search() {
                    Ok((vars, props)) => {
                        Box::new(search_with_timeout(vars, props, mode::Minimize::new(objective), timeout)) as Box<dyn Iterator<Item = Solution>>
                    }
                    Err(_) => {
                        // Validation failed - return empty iterator
                        Box::new(std::iter::empty()) as Box<dyn Iterator<Item = Solution>>
                    }
                }
            }
        }
    }

    /// Find assignment that maximizes objective expression while satisfying all constraints.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(1, 10);
    /// post!(m, x < 8);
    /// let solution = m.maximize(x);
    /// ```
    #[must_use]
    pub fn maximize(self, objective: impl View) -> SolverResult<Solution> {
        // First try specialized optimization before falling back to opposite+minimize pattern
        match self.try_optimization_maximize(&objective) {
            Some(mut solution) => {
                // Optimization succeeded - update with minimal stats since no search was performed
                solution.stats = crate::core::solution::SolveStats {
                    propagation_count: 0,
                    node_count: 0,
                    solve_time: std::time::Duration::ZERO,
                    variable_count: solution.stats.variable_count, // Preserve if already set
                    constraint_count: solution.stats.constraint_count, // Preserve if already set
                    peak_memory_mb: solution.stats.peak_memory_mb, // Preserve from optimization
                };
                Ok(solution)
            }
            None => self.minimize(objective.opposite()),
        }
    }

    /// Find assignment that maximizes objective expression.
    /// 
    /// Find assignment that maximizes objective expression and return both solution and statistics.
    ///
    /// This method provides the same functionality as `minimize()` but for maximization,
    /// and returns both the solution and solving statistics in a single call.
    ///
    /// # Returns
    /// A `SolutionWithStats` containing both the optimal solution and solving statistics
    ///
    /// # Example
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(1, 10);
    /// let y = m.int(1, 10);
    /// post!(m, x + y <= int(15));
    /// let sum = m.add(x, y);
    /// let result = m.maximize(sum);
    /// ```
    /// 
    /// Enumerate assignments that satisfy all constraints, while maximizing objective expression.
    ///
    /// The order in which assignments are yielded is not stable.
    /// 
    /// # Example
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(1, 5);
    /// let solutions: Vec<_> = m.maximize_and_iterate(x).collect();
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

    #[doc(hidden)]
    /// Get reference to variables for analysis (used by optimization module)
    #[doc(hidden)]
    pub fn get_vars(&self) -> &Vars {
        &self.vars
    }

    #[doc(hidden)]
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
        use crate::core::validation::ModelValidator;
        
        // Use the comprehensive validation system
        let validator = ModelValidator::new(&self.vars, &self.props);
        match validator.validate() {
            Ok(()) => Ok(()),
            Err(solver_error) => Err(format!("{}", solver_error)),
        }
    }

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
    /// let mut m = Model::default();
    /// let vars: Vec<_> = m.int_vars(4, 1, 4).collect();
    /// post!(m, alldiff(vars));
    /// m.optimize_constraint_order();
    /// ```
    pub fn optimize_constraint_order(&mut self) -> &mut Self {
        // Universal constraint optimization that works for all constraint types
        self.props.optimize_universal_constraint_order(&self.vars);
        self
    }

    #[doc(hidden)]
    /// Create a search engine for this model that allows direct control over search.
    ///
    /// This provides access to lower-level search functionality including resource
    /// cleanup callbacks, custom iteration, and manual search control.
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(1, 10);
    /// let y = m.int(1, 10);
    /// post!(m, x != y);
    /// let mut engine = m.engine();
    /// engine.register_cleanup(Box::new(|| println!("Cleanup executed!")));
    /// let solution = engine.solve_any();
    /// ```
    pub fn engine(self) -> EngineWrapper {
        EngineWrapper::new(self)
    }

    /// Find a solution that satisfies all constraints.
    /// 
    /// Searches for any assignment to variables that satisfies all posted constraints.
    /// Uses hybrid optimization techniques when applicable before falling back to 
    /// traditional constraint propagation search.
    ///
    /// # Returns
    /// * `Ok(Solution)` - A valid solution if one exists
    /// * `Err(SolverError)` - No solution exists, timeout occurred, or other error
    ///
    /// # Example
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(1, 10);
    /// let y = m.int(1, 10);
    /// post!(m, x != y);
    /// 
    /// match m.solve() {
    ///     Ok(solution) => println!("Found: x={:?}, y={:?}", solution[x], solution[y]),
    ///     Err(e) => println!("No solution: {}", e),
    /// }
    /// ```
    #[must_use]
    pub fn solve(self) -> SolverResult<Solution> {
        // Check if memory limit was exceeded during model building
        if self.memory_limit_exceeded {
            return Err(SolverError::MemoryLimit {
                usage_mb: Some(self.estimated_memory_mb() as usize),
                limit_mb: self.config.max_memory_mb.map(|x| x as usize),
            });
        }
        
        // For pure constraint satisfaction (no optimization objective), go directly to search
        let timeout = self.timeout_duration();
        let memory_limit = self.memory_limit_mb();
        let (vars, props) = self.prepare_for_search()?;
        
        // Capture counts before moving to search
        let var_count = vars.count();
        let constraint_count = props.count();
        
        let mut search_iter = search_with_timeout_and_memory(vars, props, mode::Enumerate, timeout, memory_limit);
        
        let result = search_iter.next();
        
        // Capture statistics after search
        let stats = crate::core::solution::SolveStats {
            propagation_count: search_iter.get_propagation_count(),
            node_count: search_iter.get_node_count(),
            solve_time: search_iter.elapsed_time(),
            variable_count: var_count,
            constraint_count,
            peak_memory_mb: search_iter.get_memory_usage_mb(), // Direct MB usage
        };
        
        // Check if search terminated due to timeout
        if search_iter.is_timed_out() {
            let elapsed = search_iter.elapsed_time().as_secs_f64();
            return Err(SolverError::timeout_with_context(elapsed, "constraint satisfaction"));
        }
        
        // Check if search terminated due to memory limit
        if search_iter.is_memory_limit_exceeded() {
            let usage_mb = search_iter.get_memory_usage_mb();
            let limit_mb = memory_limit.unwrap_or(0) as usize;
            return Err(SolverError::memory_limit_with_context(usage_mb, limit_mb));
        }
        
        match result {
            Some(mut solution) => {
                solution.stats = stats;
                Ok(solution)
            },
            None => Err(SolverError::no_solution()),
        }
    }
    
    /// Step 6.5: Try hybrid optimization approach for constraint satisfaction
    /// Returns Some(solution) if hybrid solver succeeds, None if should fall back to search
    // fn try_hybrid_solve(&self) -> Option<Solution> {
    //     // Create a dummy objective (we're not optimizing, just solving constraints)
    //     // Use the first variable if available, otherwise return None
    //     let first_var = match self.vars.iter().next() {
    //         Some(_) => {
    //             // Create VarId for the first variable
    //             let first_var_id = crate::optimization::model_integration::index_to_var_id(0);
    //             first_var_id
    //         },
    //         None => return None, // No variables to solve
    //     };
        
    //     // Try optimization with the dummy objective
    //     match self.optimization_router.try_minimize(&self.vars, &self.props, &first_var) {
    //         OptimizationAttempt::Success(solution) => Some(solution),
    //         OptimizationAttempt::Fallback(_) => None, // Fall back to search
    //         OptimizationAttempt::Infeasible(_) => None, // No solution exists
    //     }
    // }

    #[doc(hidden)]
    /// Internal helper that validates the model and optimizes constraints before search.
    /// This ensures all solving methods benefit from validation and constraint optimization.
    fn prepare_for_search(mut self) -> Result<(crate::variables::Vars, crate::constraints::props::Propagators), crate::core::error::SolverError> {
        // First, validate the model for common errors
        let validator = crate::core::validation::ModelValidator::new(&self.vars, &self.props);
        validator.validate()?;
        
        // Then optimize constraint order for better performance
        self.optimize_constraint_order();
        Ok((self.vars, self.props))
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

    /// Find all solutions that satisfy the constraints.
    ///
    /// Returns an iterator over all valid assignments to variables that satisfy
    /// all posted constraints. The order of solutions is not guaranteed to be stable.
    /// 
    /// # Returns
    /// An iterator over `Solution` objects. Each solution represents one valid
    /// assignment to all variables.
    ///
    /// # Example
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(1, 3);
    /// let y = m.int(1, 3);
    /// post!(m, x != y);  // x and y must be different
    /// 
    /// // Collect all solutions
    /// let solutions: Vec<_> = m.enumerate().collect();
    /// println!("Found {} solutions", solutions.len());
    /// 
    /// for solution in solutions {
    ///     println!("x={:?}, y={:?}", solution[x], solution[y]);
    /// }
    /// ```
    pub fn enumerate(self) -> impl Iterator<Item = Solution> {
        let timeout = self.timeout_duration();
        let memory_limit = self.memory_limit_mb();
        match self.prepare_for_search() {
            Ok((vars, props)) => {
                Box::new(search_with_timeout_and_memory(vars, props, mode::Enumerate, timeout, memory_limit)) as Box<dyn Iterator<Item = Solution>>
            }
            Err(_) => {
                // Validation failed - return empty iterator
                Box::new(std::iter::empty()) as Box<dyn Iterator<Item = Solution>>
            }
        }
    }

    /// Find all solutions with embedded statistics.
    ///
    /// Returns all valid assignments to variables that satisfy all posted constraints.
    /// Each solution includes embedded statistics about the solving process.
    /// 
    /// # Returns
    /// A vector containing all found solutions with their embedded statistics
    ///
    /// # Example
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(1, 3);
    /// let solutions = m.enumerate();
    /// for solution in solutions {
    ///     println!("Solution found! Propagations: {}", solution.stats.propagation_count);
    /// }
    /// ```
    /// 
    /// Find all solutions and return both solutions and statistics.
    ///
    /// This method provides comprehensive solution enumeration with embedded statistics,
    /// eliminating the need for external callbacks or manual statistics collection.
    ///
    /// # Returns
    /// A tuple containing a vector of all solutions and the solving statistics
    ///
    /// # Example
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(1, 3);
    /// let y = m.int(1, 3);
    /// post!(m, x != y);
    /// 
    /// let (solutions, stats) = m.enumerate_with_stats();
    /// 
    /// println!("Found {} solutions", solutions.len());
    /// println!("Search explored {} nodes", stats.node_count);
    /// println!("Performed {} propagations", stats.propagation_count);
    /// 
    /// for solution in solutions {
    ///     println!("x={:?}, y={:?}", solution[x], solution[y]);
    /// }
    /// ```
    pub fn enumerate_with_stats(self) -> (Vec<Solution>, crate::core::solution::SolveStats) {
        let timeout = self.timeout_duration();
        let memory_limit = self.memory_limit_mb();
        let (vars, props) = match self.prepare_for_search() {
            Ok(result) => result,
            Err(_) => {
                // Validation failed - report error stats and return empty vector
                let stats = crate::core::solution::SolveStats {
                    propagation_count: 0,
                    node_count: 0,
                    solve_time: std::time::Duration::ZERO,
                    variable_count: 0, // Unknown due to validation failure
                    constraint_count: 0, // Unknown due to validation failure
                    peak_memory_mb: 0, // No memory used if validation failed
                };
                return (Vec::new(), stats);
            }
        };

        // Capture counts before moving to search
        let var_count = vars.count();
        let constraint_count = props.count();

        let mut search_iter = search_with_timeout_and_memory(vars, props, mode::Enumerate, timeout, memory_limit);
        let mut solutions = Vec::with_capacity(8); // Start with reasonable capacity for solution collection

        // Collect all solutions - the search iterator will track statistics as it goes
        while let Some(solution) = search_iter.next() {
            solutions.push(solution);
        }

        // Get the final statistics after enumeration is complete
        let stats = crate::core::solution::SolveStats {
            propagation_count: search_iter.get_propagation_count(),
            node_count: search_iter.get_node_count(),
            solve_time: search_iter.elapsed_time(),
            variable_count: var_count,
            constraint_count,
            peak_memory_mb: search_iter.get_memory_usage_mb(), // Direct MB usage
        };
        
        // Note: If timeout occurred, we return partial solutions found before timeout
        // The timeout condition can be checked via search_iter.is_timed_out() if needed
        
        (solutions, stats)
    }
}

impl Index<VarId> for Model {
    type Output = Var;

    fn index(&self, index: VarId) -> &Self::Output {
        &self.vars[index]
    }
}

#[doc(hidden)]
/// Wrapper around search engine that provides clean API for resource management
pub struct EngineWrapper {
    model: Option<Model>,
    callbacks: Vec<Box<dyn FnOnce() + Send>>,
}

impl EngineWrapper {
    fn new(model: Model) -> Self {
        Self { 
            model: Some(model),
            callbacks: Vec::new(),
        }
    }

    /// Configure the engine with custom settings
    #[must_use]
    pub fn with_config(mut self, config: crate::utils::config::SolverConfig) -> Self {
        if let Some(ref mut model) = self.model {
            model.config = config;
        }
        self
    }

    /// Register a cleanup callback that will be called when search is interrupted
    /// or when the engine is dropped
    pub fn register_cleanup(&mut self, callback: Box<dyn FnOnce() + Send>) {
        self.callbacks.push(callback);
    }

    /// Solve for any valid solution
    #[must_use]
    pub fn solve_any(&mut self) -> SolverResult<crate::core::solution::Solution> {
        if let Some(model) = self.model.take() {
            match model.solve() {
                Ok(solution) => Ok(solution),
                Err(err) => {
                    // Execute cleanup callbacks on error
                    self.trigger_cleanup();
                    Err(err)
                }
            }
        } else {
            Err(SolverError::InternalError {
                message: "Model has already been consumed".to_string(),
                location: Some("solve_any()".to_string()),
                debug_info: Some("The model can only be solved once".to_string()),
            })
        }
    }

    /// Trigger all registered cleanup callbacks
    fn trigger_cleanup(&mut self) {
        for callback in self.callbacks.drain(..) {
            callback();
        }
    }
}

impl Drop for EngineWrapper {
    fn drop(&mut self) {
        self.trigger_cleanup();
    }
}
