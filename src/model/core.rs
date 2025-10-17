use crate::prelude::*;
use crate::variables::domain::float_interval::{DEFAULT_FLOAT_PRECISION_DIGITS, precision_to_step_size};
use crate::optimization::model_integration::{OptimizationRouter, OptimizationAttempt};
use crate::core::error::{SolverError, SolverResult};
use std::ops::Index;

#[derive(Debug)]
pub struct Model {
    #[doc(hidden)]
    pub vars: Vars,
    #[doc(hidden)]
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
    /// LP constraints extracted from runtime API AST before materialization
    /// These are collected during constraint posting and used at search root for LP solving
    #[doc(hidden)]
    pub pending_lp_constraints: Vec<crate::lpsolver::csp_integration::LinearConstraint>,
    /// Pending constraint ASTs that haven't been materialized yet
    /// We delay materialization to avoid creating duplicate propagators
    #[doc(hidden)]
    pub pending_constraint_asts: Vec<crate::runtime_api::ConstraintKind>,
    /// Validation errors detected during constraint posting
    /// These will be returned as errors during solve()
    #[doc(hidden)]
    pub constraint_validation_errors: Vec<crate::core::SolverError>,
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
    /// use selen::prelude::*;
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
            pending_lp_constraints: Vec::new(),
            pending_constraint_asts: Vec::new(),
            constraint_validation_errors: Vec::new(),
        }
    }

    /// Create a new model with a configuration
    ///
    /// ```
    /// use selen::prelude::*;
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
            pending_lp_constraints: Vec::new(),
            pending_constraint_asts: Vec::new(),
            constraint_validation_errors: Vec::new(),
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
    /// use selen::prelude::*;
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
    /// use selen::prelude::*;
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
    /// use selen::prelude::*;
    /// let m = Model::default();
    /// let config = m.config();
    /// println!("Float precision: {}", config.float_precision_digits);
    /// ```
    pub fn config(&self) -> &crate::utils::config::SolverConfig {
        &self.config
    }

    /// Get timeout as Duration for search operations
    fn timeout_duration(&self) -> Option<std::time::Duration> {
        self.config.timeout_ms.map(std::time::Duration::from_millis)
    }

    /// Get memory limit in MB for search operations
    fn memory_limit_mb(&self) -> Option<u64> {
        self.config.max_memory_mb
    }
    
    /// Materialize pending constraint ASTs into actual propagators
    /// This is called after LP extraction and solving to create the propagators
    /// needed for the CSP search phase.
    pub(crate) fn materialize_pending_asts(&mut self) {
        // STEP 0: Pre-process equality constraints to apply immediate bounds
        // This ensures that Var==Var constraints are applied to variable domains
        // BEFORE other constraints (like modulo) are materialized and create result variables
        // based on operand bounds
        self.apply_immediate_var_eq_bounds();
        
        // Take ownership of the pending ASTs to avoid borrow checker issues
        let asts = std::mem::take(&mut self.pending_constraint_asts);
        
        // Materialize each AST into propagators
        for ast in asts {
            crate::runtime_api::materialize_constraint_kind(self, &ast);
        }
    }
    
    /// Pre-process pending ASTs to apply immediate bounds for Var==Var equality constraints
    /// This ensures variables are constrained BEFORE other constraints see their bounds
    fn apply_immediate_var_eq_bounds(&mut self) {
        use crate::runtime_api::{ExprBuilder, ConstraintKind, ComparisonOp};
        
        // Collect all Var==Var equality constraints
        let mut eq_constraints = Vec::new();
        for ast in self.pending_constraint_asts.iter() {
            if let ConstraintKind::Binary { left, op, right } = ast {
                if matches!(op, ComparisonOp::Eq) {
                    if let (ExprBuilder::Var(var1), ExprBuilder::Var(var2)) = (left, right) {
                        eq_constraints.push((*var1, *var2));
                    }
                }
            }
        }
        
        // Apply bounds for each Var==Var constraint
        for (var1, var2) in eq_constraints {
            if var1.to_index() < self.vars.count() && var2.to_index() < self.vars.count() {
                // Collect bounds from both variables
                let (var1_min, var1_max, is_var1_int) = match &self.vars[var1] {
                    crate::variables::Var::VarI(ss) => (ss.min(), ss.max(), true),
                    crate::variables::Var::VarF(_) => (0, 0, false),
                };
                
                let (var2_min, var2_max, is_var2_int) = match &self.vars[var2] {
                    crate::variables::Var::VarI(ss) => (ss.min(), ss.max(), true),
                    crate::variables::Var::VarF(_) => (0, 0, false),
                };
                
                // Only apply for integer variables
                if is_var1_int && is_var2_int {
                    // Compute intersection bounds
                    let intersection_min = if var1_min > var2_min { var1_min } else { var2_min };
                    let intersection_max = if var1_max < var2_max { var1_max } else { var2_max };
                    
                    if intersection_min <= intersection_max {
                        // Collect values to remove BEFORE any mutable borrows
                        let var1_to_remove: Vec<i32> = if let crate::variables::Var::VarI(ss) = &self.vars[var1] {
                            ss.iter()
                                .filter(|val| *val < intersection_min || *val > intersection_max)
                                .collect()
                        } else {
                            Vec::new()
                        };
                        
                        let var2_to_remove: Vec<i32> = if let crate::variables::Var::VarI(ss) = &self.vars[var2] {
                            ss.iter()
                                .filter(|val| *val < intersection_min || *val > intersection_max)
                                .collect()
                        } else {
                            Vec::new()
                        };
                        
                        // Now remove values from var1
                        if let crate::variables::Var::VarI(sparse_set) = &mut self.vars[var1] {
                            for val in var1_to_remove {
                                sparse_set.remove(val);
                            }
                        }
                        
                        // Remove values from var2
                        if let crate::variables::Var::VarI(sparse_set) = &mut self.vars[var2] {
                            for val in var2_to_remove {
                                sparse_set.remove(val);
                            }
                        }
                    }
                }
            }
        }
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
    
    /// Set the memory limit exceeded flag (used internally by factory methods)
    pub(crate) fn set_memory_limit_exceeded(&mut self) {
        self.memory_limit_exceeded = true;
    }
    
    /// Add to estimated memory usage (used internally by factory methods)
    pub(crate) fn add_estimated_memory(&mut self, bytes: u64) {
        self.estimated_memory_bytes += bytes;
    }
    
    /// Get mutable access to vars (used internally by factory methods)
    pub(crate) fn vars_mut(&mut self) -> &mut crate::variables::Vars {
        &mut self.vars
    }
    
    /// Get mutable access to props (used internally by factory methods)
    pub(crate) fn props_mut(&mut self) -> &mut crate::constraints::props::Propagators {
        &mut self.props
    }
    
    /// Get the current number of variables in the model
    /// 
    /// This can be called at any time during model construction to check
    /// how many variables have been created.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// 
    /// assert_eq!(m.variable_count(), 0);
    /// let x = m.int(1, 10);
    /// assert_eq!(m.variable_count(), 1);
    /// let y = m.float(0.0, 1.0);
    /// assert_eq!(m.variable_count(), 2);
    /// ```
    pub fn variable_count(&self) -> usize {
        self.vars.count()
    }
    
    /// Get the current number of constraints in the model
    /// 
    /// This can be called at any time during model construction to check
    /// how many constraints have been posted.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(1, 10);
    /// let y = m.int(1, 10);
    /// let z = m.int(1, 10);
    /// 
    /// assert_eq!(m.constraint_count(), 0);
    /// m.alldiff(&[x, y, z]);
    /// let count_after_first = m.constraint_count();
    /// assert!(count_after_first > 0);
    /// m.alleq(&[x, y]);
    /// assert!(m.constraint_count() > count_after_first);
    /// ```
    pub fn constraint_count(&self) -> usize {
        self.props.count()
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

    #[doc(hidden)]
    /// Get access to constraint registry for debugging/analysis
    pub fn get_constraint_registry(&self) -> &crate::optimization::constraint_metadata::ConstraintRegistry {
        self.props.get_constraint_registry()
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
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(1, 10);
    /// m.new(x.gt(3));
    /// let solution = m.minimize(x);
    /// ```
    #[must_use]
    pub fn minimize(self, objective: impl View) -> SolverResult<Solution> {
        // Check for constraint validation errors first
        if !self.constraint_validation_errors.is_empty() {
            return Err(self.constraint_validation_errors[0].clone());
        }
        
        // Record start time for initialization time tracking
        let init_start = std::time::Instant::now();
        
        // Capture variable type counts from the model before optimization/search
        let int_var_count = self.vars.int_var_count;
        let bool_var_count = self.vars.bool_var_count;
        let float_var_count = self.vars.float_var_count;
        let set_var_count = self.vars.set_var_count;
        
        // First try specialized optimization (Step 2.4 precision handling)
        match self.try_optimization_minimize(&objective) {
            Some(mut solution) => {
                // Optimization succeeded - update with minimal stats since no search was performed
                // Note: propagators_count will be set after prepare_for_search in fallback path
                solution.stats = crate::core::solution::SolveStats {
                    propagation_count: 0,
                    node_count: 0,
                    solve_time: std::time::Duration::ZERO,
                    variables: solution.stats.variables, // Preserve if already set
                    constraint_count: solution.stats.constraint_count, // Preserve if already set
                    peak_memory_mb: solution.stats.peak_memory_mb, // Preserve from optimization
                    lp_solver_used: false,
                    lp_constraint_count: 0,
                    lp_variable_count: 0,
                    lp_stats: None,
                    bool_variables: bool_var_count,
                    float_variables: float_var_count,
                    init_time: init_start.elapsed(),  // Time to run optimization
                    int_variables: int_var_count,
                    objective: 0.0,
                    objective_bound: 0.0,
                    propagators: 0,  // Would be available after prepare_for_search in fallback
                    set_variables: set_var_count,
                };
                Ok(solution)
            }
            None => {
                // Optimization failed or not applicable - fall back to traditional search
                let timeout = self.timeout_duration();
                let memory_limit = self.memory_limit_mb();
                let float_precision = self.float_precision_digits;
                let (vars, props, pending_lp) = self.prepare_for_search()?;

                // Capture counts AFTER prepare_for_search (which materializes all constraints into propagators)
                let var_count = vars.count();
                let constraint_count = props.count();
                let propagators_count = props.count();  // Same as constraint_count after materialization
                
                // Record initialization time (time spent setting up model, materializing constraints, etc.)
                let init_time = init_start.elapsed();

                let mut search_iter = search_with_timeout_and_memory(vars, props, mode::Minimize::new(objective), timeout, memory_limit, pending_lp, float_precision);
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
                    variables: var_count,
                    constraint_count,
                    peak_memory_mb: search_iter.get_memory_usage_mb(), // Direct MB usage
                    lp_solver_used: false,
                    lp_constraint_count: 0,
                    lp_variable_count: 0,
                    lp_stats: None,
                    bool_variables: bool_var_count,
                    float_variables: float_var_count,
                    init_time: init_time,  // Use captured initialization time
                    int_variables: int_var_count,
                    objective: 0.0,
                    objective_bound: 0.0,
                    propagators: propagators_count,
                    set_variables: set_var_count,
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
    /// use selen::prelude::*;
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
                let float_precision = self.float_precision_digits;
                match self.prepare_for_search() {
                    Ok((vars, props, pending_lp)) => {
                        Box::new(search_with_timeout(vars, props, mode::Minimize::new(objective), timeout, pending_lp, float_precision)) as Box<dyn Iterator<Item = Solution>>
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
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(1, 10);
    /// m.new(x.lt(8));
    /// let solution = m.maximize(x);
    /// ```
    #[must_use]
    pub fn maximize(self, objective: impl View) -> SolverResult<Solution> {
        // Check for constraint validation errors first
        if !self.constraint_validation_errors.is_empty() {
            return Err(self.constraint_validation_errors[0].clone());
        }
        
        // Record start time for initialization time tracking
        let init_start = std::time::Instant::now();
        
        // Capture variable type counts from the model before optimization/search
        let int_var_count = self.vars.int_var_count;
        let bool_var_count = self.vars.bool_var_count;
        let float_var_count = self.vars.float_var_count;
        let set_var_count = self.vars.set_var_count;
        
        // First try specialized optimization before falling back to opposite+minimize pattern
        match self.try_optimization_maximize(&objective) {
            Some(mut solution) => {
                // Optimization succeeded - update with minimal stats since no search was performed
                solution.stats = crate::core::solution::SolveStats {
                    propagation_count: 0,
                    node_count: 0,
                    solve_time: std::time::Duration::ZERO,
                    variables: solution.stats.variables, // Preserve if already set
                    constraint_count: solution.stats.constraint_count, // Preserve if already set
                    peak_memory_mb: solution.stats.peak_memory_mb, // Preserve from optimization
                    lp_solver_used: false,
                    lp_constraint_count: 0,
                    lp_variable_count: 0,
                    lp_stats: None,
                    bool_variables: bool_var_count,
                    float_variables: float_var_count,
                    init_time: init_start.elapsed(),  // Time to run optimization
                    int_variables: int_var_count,
                    objective: 0.0,
                    objective_bound: 0.0,
                    propagators: 0,  // Would be available after prepare_for_search in fallback
                    set_variables: set_var_count,
                };
                Ok(solution)
            }
            None => {
                // Optimization router failed - use search-based minimize(opposite)
                // The variable type counts will be preserved through the minimize() call
                match self.minimize(objective.opposite()) {
                    Ok(solution) => {

                        // FIXED: Solution extraction consistency for maximize(objective) → minimize(opposite)
                        // The minimize(opposite) approach correctly finds constraint-respecting values for 
                        // decision variables. The main optimization bug is in the optimization router 
                        // bypassing constraint propagation entirely, not in the minimize(opposite) transform.
                        // Decision variable values are correct; any composite objective inconsistencies
                        // are due to the router's constraint-ignoring behavior, addressed separately.
                        Ok(solution)
                    }
                    Err(e) => Err(e),
                }
            }
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
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(1, 10);
    /// let y = m.int(1, 10);
    /// let sum = m.add(x, y);
    /// m.new(sum.le(15));
    /// let result = m.maximize(sum);
    /// ```
    /// 
    /// Enumerate assignments that satisfy all constraints, while maximizing objective expression.
    ///
    /// The order in which assignments are yielded is not stable.
    /// 
    /// # Example
    /// ```
    /// use selen::prelude::*;
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

    /// Infer bounds for unbounded variables by analyzing constraint ASTs.
    ///
    /// This method scans all pending constraint ASTs (before materialization) to extract
    /// bounds information for variables that were created with unbounded domains (i32::MIN/MAX).
    /// 
    /// The inference analyzes:
    /// - Binary comparisons: x < y, x <= c, etc.
    /// - Linear constraints: 2x + 3y <= 10
    /// - Element constraints: array[x] bounds x by array size
    /// - AllDifferent: bounds variables by constraint size
    ///
    /// This provides much better bounds than the simple variable-context inference
    /// at creation time, since all constraints are available for analysis.
    ///
    /// **SPECIAL CASE**: Also applies simple binary equals constraints to ALL variables
    /// (even bounded ones) to ensure that deferred equality constraints like `x.eq(47)`
    /// are honored before other constraints (like modulo) are posted.
    ///
    /// # Returns
    /// Ok(()) if inference succeeds, Err if unbounded variables cannot be reasonably bounded.
    fn infer_unbounded_from_asts(&mut self) -> Result<(), SolverError> {
        use std::collections::HashMap;
        
        // **SPECIAL HANDLING FOR EQUALS CONSTRAINTS**
        // Apply simple binary equals constraints (Var == Val) to tighten bounds
        // This must happen BEFORE materializing other constraints that might depend on bounds
        for constraint_ast in &self.pending_constraint_asts {
            if let crate::runtime_api::ConstraintKind::Binary { left, op, right } = constraint_ast {
                if matches!(op, crate::runtime_api::ComparisonOp::Eq) {
                    // Pattern: Var == Constant
                    if let (crate::runtime_api::ExprBuilder::Var(var_id), crate::runtime_api::ExprBuilder::Val(val)) = (left, right) {
                        // Apply this equality constraint immediately
                        match &mut self.vars[*var_id] {
                            crate::variables::Var::VarI(sparse_set) => {
                                if let Val::ValI(i) = val {
                                    sparse_set.remove_all_but(*i);
                                }
                            }
                            crate::variables::Var::VarF(interval) => {
                                if let Val::ValF(f) = val {
                                    interval.min = *f;
                                    interval.max = *f;
                                }
                            }
                        }
                    }
                    // Pattern: Constant == Var
                    else if let (crate::runtime_api::ExprBuilder::Val(val), crate::runtime_api::ExprBuilder::Var(var_id)) = (left, right) {
                        // Apply this equality constraint immediately
                        match &mut self.vars[*var_id] {
                            crate::variables::Var::VarI(sparse_set) => {
                                if let Val::ValI(i) = val {
                                    sparse_set.remove_all_but(*i);
                                }
                            }
                            crate::variables::Var::VarF(interval) => {
                                if let Val::ValF(f) = val {
                                    interval.min = *f;
                                    interval.max = *f;
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // PHASE 1: Identify unbounded variables
        let unbounded_vars = self.identify_unbounded_variables();
        if unbounded_vars.is_empty() {
            return Ok(()); // No unbounded variables, nothing to do
        }
        
        // PHASE 2: Extract bounds from constraint ASTs
        let mut bounds_map: HashMap<VarId, (Option<Val>, Option<Val>)> = HashMap::new();
        for var_id in &unbounded_vars {
            bounds_map.insert(*var_id, (None, None));
        }
        
        for constraint_ast in &self.pending_constraint_asts {
            self.extract_bounds_from_constraint(&constraint_ast, &unbounded_vars, &mut bounds_map);
        }
        
        // PHASE 3: Aggregate and apply bounds
        for var_id in &unbounded_vars {
            if let Some((min_bound, max_bound)) = bounds_map.get(var_id) {
                self.apply_inferred_bounds(*var_id, min_bound, max_bound)?;
            }
        }
        
        // PHASE 4: Fallback for still-unbounded variables
        for var_id in &unbounded_vars {
            if self.is_still_unbounded(*var_id) {
                self.apply_fallback_bounds(*var_id)?;
            }
        }
        
        Ok(())
    }
    
    /// Phase 1: Identify variables with unbounded domains
    fn identify_unbounded_variables(&self) -> Vec<VarId> {
        let mut unbounded = Vec::new();
        
        for var_idx in 0..self.vars.count() {
            let var_id = VarId::from_index(var_idx);
            match &self.vars[var_id] {
                crate::variables::Var::VarI(sparse_set) => {
                    let min_val = sparse_set.min_universe_value();
                    let max_val = sparse_set.max_universe_value();
                    
                    // Check if unbounded (uses sentinel values)
                    if min_val == i32::MIN || max_val == i32::MAX {
                        unbounded.push(var_id);
                    }
                }
                crate::variables::Var::VarF(interval) => {
                    // Check if float is unbounded
                    if interval.min == f64::NEG_INFINITY || interval.max == f64::INFINITY {
                        unbounded.push(var_id);
                    }
                }
            }
        }
        
        unbounded
    }
    
    /// Infer bounds for an expression recursively
    /// 
    /// This method computes the bounds of an expression based on the bounds of its operands.
    /// For variables, it looks up their current bounds. For constants, it uses the constant's value.
    /// For arithmetic expressions, it computes bounds based on the operation and operand bounds.
    fn infer_expr_bounds(&self, expr: &crate::runtime_api::ExprBuilder) -> Option<(i32, i32)> {
        use crate::runtime_api::ExprBuilder;
        
        match expr {
            ExprBuilder::Var(var_id) => {
                // Get bounds of the variable directly
                self.get_variable_bounds(*var_id)
            }
            ExprBuilder::Val(val) => {
                // For constants, return a point interval
                match val {
                    Val::ValI(i) => Some((*i, *i)),
                    Val::ValF(f) => {
                        // For float constants, convert to int bounds (lossy)
                        Some((f.floor() as i32, f.ceil() as i32))
                    }
                }
            }
            ExprBuilder::Add(left, right) => {
                // a + b: bounds are [a.min + b.min, a.max + b.max]
                let left_bounds = self.infer_expr_bounds(left)?;
                let right_bounds = self.infer_expr_bounds(right)?;
                Some((left_bounds.0 + right_bounds.0, left_bounds.1 + right_bounds.1))
            }
            ExprBuilder::Sub(left, right) => {
                // a - b: bounds are [a.min - b.max, a.max - b.min]
                let left_bounds = self.infer_expr_bounds(left)?;
                let right_bounds = self.infer_expr_bounds(right)?;
                Some((left_bounds.0 - right_bounds.1, left_bounds.1 - right_bounds.0))
            }
            ExprBuilder::Mul(left, right) => {
                // a * b: check all corners since multiplication can flip bounds
                let left_bounds = self.infer_expr_bounds(left)?;
                let right_bounds = self.infer_expr_bounds(right)?;
                
                let products = [
                    left_bounds.0 * right_bounds.0,
                    left_bounds.0 * right_bounds.1,
                    left_bounds.1 * right_bounds.0,
                    left_bounds.1 * right_bounds.1,
                ];
                
                let min = *products.iter().min().unwrap_or(&0);
                let max = *products.iter().max().unwrap_or(&0);
                Some((min, max))
            }
            ExprBuilder::Div(left, right) => {
                // a / b: be conservative - similar to multiplication
                let left_bounds = self.infer_expr_bounds(left)?;
                let right_bounds = self.infer_expr_bounds(right)?;
                
                // Avoid division by zero
                if right_bounds.0 <= 0 && right_bounds.1 >= 0 {
                    return None; // Can't infer bounds if divisor includes 0
                }
                
                // Compute all possible quotients
                let mut quotients = Vec::new();
                if right_bounds.0 != 0 {
                    quotients.push(left_bounds.0 / right_bounds.0);
                    quotients.push(left_bounds.1 / right_bounds.0);
                }
                if right_bounds.1 != 0 {
                    quotients.push(left_bounds.0 / right_bounds.1);
                    quotients.push(left_bounds.1 / right_bounds.1);
                }
                
                if quotients.is_empty() {
                    return None;
                }
                
                let min = *quotients.iter().min().unwrap_or(&0);
                let max = *quotients.iter().max().unwrap_or(&0);
                Some((min, max))
            }
            ExprBuilder::Modulo(left, right) => {
                // a % b: result is in [0, max(|b.min|, |b.max|) - 1]
                let _left_bounds = self.infer_expr_bounds(left)?;
                let right_bounds = self.infer_expr_bounds(right)?;
                
                // Modulo result is always in range [0, |divisor| - 1] (for positive divisor)
                // or more generally, [-(|divisor|-1), |divisor|-1]
                let divisor_abs = right_bounds.0.abs().max(right_bounds.1.abs());
                
                if divisor_abs <= 0 {
                    return None; // Invalid divisor
                }
                
                Some((-(divisor_abs - 1), divisor_abs - 1))
            }
        }
    }
    
    /// Phase 2: Extract bounds from a single constraint AST
    fn extract_bounds_from_constraint(
        &self,
        constraint: &crate::runtime_api::ConstraintKind,
        unbounded_vars: &[VarId],
        bounds_map: &mut std::collections::HashMap<VarId, (Option<Val>, Option<Val>)>,
    ) {
        use crate::runtime_api::ConstraintKind;
        
        match constraint {
            // Binary comparisons: x op y or x op constant
            ConstraintKind::Binary { left, op, right } => {
                self.extract_binary_bounds(left, op, right, unbounded_vars, bounds_map);
            }
            
            // Linear constraints: sum(coeffs * vars) op constant
            ConstraintKind::LinearInt { coeffs, vars, op, constant } => {
                self.extract_linear_int_bounds(coeffs, vars, op, *constant, unbounded_vars, bounds_map);
            }
            
            ConstraintKind::LinearFloat { coeffs, vars, op, constant } => {
                self.extract_linear_float_bounds(coeffs, vars, op, *constant, unbounded_vars, bounds_map);
            }
            
            // Element constraint: array[index] = value
            ConstraintKind::Element { index, array, .. } => {
                if unbounded_vars.contains(index) {
                    // Index must be in [0, array.len() - 1]
                    self.update_bounds(bounds_map, *index, Some(Val::ValI(0)), Some(Val::ValI(array.len() as i32 - 1)));
                }
            }
            
            // AllDifferent: n variables need at least n distinct values
            ConstraintKind::AllDifferent { vars } => {
                // For each unbounded variable in this constraint, we know it needs
                // to be distinct from the others. This provides weak bounds.
                // If other vars are bounded [a, b], unbounded var should be roughly [a, b] too
                for var_id in vars {
                    if unbounded_vars.contains(var_id) {
                        self.extract_alldiff_bounds(*var_id, vars, bounds_map);
                    }
                }
            }
            
            _ => {
                // Other constraint types don't provide useful bounds yet
                // TODO: Add support for more constraint types (Sum, Min, Max, etc.)
            }
        }
    }
    
    /// Extract bounds from binary comparison constraints
    fn extract_binary_bounds(
        &self,
        left: &crate::runtime_api::ExprBuilder,
        op: &crate::runtime_api::ComparisonOp,
        right: &crate::runtime_api::ExprBuilder,
        unbounded_vars: &[VarId],
        bounds_map: &mut std::collections::HashMap<VarId, (Option<Val>, Option<Val>)>,
    ) {
        use crate::runtime_api::{ExprBuilder, ComparisonOp};
        
        // Pattern: Var op Constant
        if let (ExprBuilder::Var(var_id), ExprBuilder::Val(val)) = (left, right) {
            if unbounded_vars.contains(var_id) {
                match op {
                    ComparisonOp::Lt => {
                        // x < c  →  x <= c-1 (for integers)
                        let upper = match val {
                            Val::ValI(i) => Some(Val::ValI(i - 1)),
                            Val::ValF(f) => Some(Val::ValF(*f)),
                        };
                        self.update_bounds(bounds_map, *var_id, None, upper);
                    }
                    ComparisonOp::Le => {
                        // x <= c
                        self.update_bounds(bounds_map, *var_id, None, Some(*val));
                    }
                    ComparisonOp::Gt => {
                        // x > c  →  x >= c+1 (for integers)
                        let lower = match val {
                            Val::ValI(i) => Some(Val::ValI(i + 1)),
                            Val::ValF(f) => Some(Val::ValF(*f)),
                        };
                        self.update_bounds(bounds_map, *var_id, lower, None);
                    }
                    ComparisonOp::Ge => {
                        // x >= c
                        self.update_bounds(bounds_map, *var_id, Some(*val), None);
                    }
                    ComparisonOp::Eq => {
                        // x == c  →  x ∈ [c, c]
                        self.update_bounds(bounds_map, *var_id, Some(*val), Some(*val));
                    }
                    ComparisonOp::Ne => {
                        // x != c doesn't provide useful bounds for inference
                    }
                }
            }
        }
        
        // Pattern: Constant op Var (reverse)
        if let (ExprBuilder::Val(val), ExprBuilder::Var(var_id)) = (left, right) {
            if unbounded_vars.contains(var_id) {
                match op {
                    ComparisonOp::Lt => {
                        // c < x  →  x >= c+1
                        let lower = match val {
                            Val::ValI(i) => Some(Val::ValI(i + 1)),
                            Val::ValF(f) => Some(Val::ValF(*f)),
                        };
                        self.update_bounds(bounds_map, *var_id, lower, None);
                    }
                    ComparisonOp::Le => {
                        // c <= x  →  x >= c
                        self.update_bounds(bounds_map, *var_id, Some(*val), None);
                    }
                    ComparisonOp::Gt => {
                        // c > x  →  x <= c-1
                        let upper = match val {
                            Val::ValI(i) => Some(Val::ValI(i - 1)),
                            Val::ValF(f) => Some(Val::ValF(*f)),
                        };
                        self.update_bounds(bounds_map, *var_id, None, upper);
                    }
                    ComparisonOp::Ge => {
                        // c >= x  →  x <= c
                        self.update_bounds(bounds_map, *var_id, None, Some(*val));
                    }
                    ComparisonOp::Eq => {
                        // c == x  →  x ∈ [c, c]
                        self.update_bounds(bounds_map, *var_id, Some(*val), Some(*val));
                    }
                    ComparisonOp::Ne => {
                        // c != x doesn't provide useful bounds
                    }
                }
            }
        }
        
        // Pattern: Var op Var (transitive bounds)
        if let (ExprBuilder::Var(left_var), ExprBuilder::Var(right_var)) = (left, right) {
            // If one is bounded and the other is unbounded, we can infer bounds
            if unbounded_vars.contains(left_var) && !unbounded_vars.contains(right_var) {
                // Get bounds of right_var
                if let Some(right_bounds) = self.get_variable_bounds(*right_var) {
                    match op {
                        ComparisonOp::Lt => {
                            // x < y, y bounded  →  x < y.max
                            self.update_bounds(bounds_map, *left_var, None, Some(Val::ValI(right_bounds.1 - 1)));
                        }
                        ComparisonOp::Le => {
                            // x <= y, y bounded  →  x <= y.max
                            self.update_bounds(bounds_map, *left_var, None, Some(Val::ValI(right_bounds.1)));
                        }
                        ComparisonOp::Gt => {
                            // x > y, y bounded  →  x > y.min
                            self.update_bounds(bounds_map, *left_var, Some(Val::ValI(right_bounds.0 + 1)), None);
                        }
                        ComparisonOp::Ge => {
                            // x >= y, y bounded  →  x >= y.min
                            self.update_bounds(bounds_map, *left_var, Some(Val::ValI(right_bounds.0)), None);
                        }
                        _ => {}
                    }
                }
            }
        }
        
        // Pattern: Var op Expression (e.g., remainder = modulo(number, divisor))
        if let ExprBuilder::Var(var_id) = left {
            if unbounded_vars.contains(var_id) {
                match op {
                    ComparisonOp::Eq => {
                        // Try to infer bounds from the right-hand expression
                        if let Some(expr_bounds) = self.infer_expr_bounds(right) {
                            self.update_bounds(
                                bounds_map,
                                *var_id,
                                Some(Val::ValI(expr_bounds.0)),
                                Some(Val::ValI(expr_bounds.1)),
                            );
                        }
                    }
                    _ => {} // Other operators don't help with bounds inference
                }
            }
        }
    }
    
    /// Extract bounds from integer linear constraints
    fn extract_linear_int_bounds(
        &self,
        coeffs: &[i32],
        vars: &[VarId],
        op: &crate::runtime_api::ComparisonOp,
        constant: i32,
        unbounded_vars: &[VarId],
        bounds_map: &mut std::collections::HashMap<VarId, (Option<Val>, Option<Val>)>,
    ) {
        use crate::runtime_api::ComparisonOp;
        
        // Simple case: single variable linear constraint (e.g., 2x <= 10)
        if vars.len() == 1 && unbounded_vars.contains(&vars[0]) {
            let coeff = coeffs[0];
            let var_id = vars[0];
            
            if coeff == 0 {
                return; // Degenerate constraint
            }
            
            match op {
                ComparisonOp::Le => {
                    // coeff * x <= constant  →  x <= constant / coeff (adjust for sign)
                    if coeff > 0 {
                        let upper = constant / coeff;
                        self.update_bounds(bounds_map, var_id, None, Some(Val::ValI(upper)));
                    } else {
                        let lower = constant / coeff;
                        self.update_bounds(bounds_map, var_id, Some(Val::ValI(lower)), None);
                    }
                }
                ComparisonOp::Ge => {
                    // coeff * x >= constant  →  x >= constant / coeff (adjust for sign)
                    if coeff > 0 {
                        let lower = (constant + coeff - 1) / coeff; // Ceiling division
                        self.update_bounds(bounds_map, var_id, Some(Val::ValI(lower)), None);
                    } else {
                        let upper = constant / coeff;
                        self.update_bounds(bounds_map, var_id, None, Some(Val::ValI(upper)));
                    }
                }
                ComparisonOp::Eq => {
                    // coeff * x == constant  →  x == constant / coeff (if divisible)
                    if constant % coeff == 0 {
                        let value = constant / coeff;
                        self.update_bounds(bounds_map, var_id, Some(Val::ValI(value)), Some(Val::ValI(value)));
                    }
                }
                _ => {}
            }
        }
        
        // TODO: Handle multi-variable linear constraints where only one variable is unbounded
        // Example: x + y <= 10, y ∈ [0, 5]  →  x ∈ [-∞, 10 - 0] = [-∞, 10]
    }
    
    /// Extract bounds from float linear constraints
    fn extract_linear_float_bounds(
        &self,
        coeffs: &[f64],
        vars: &[VarId],
        op: &crate::runtime_api::ComparisonOp,
        constant: f64,
        unbounded_vars: &[VarId],
        bounds_map: &mut std::collections::HashMap<VarId, (Option<Val>, Option<Val>)>,
    ) {
        use crate::runtime_api::ComparisonOp;
        
        // Simple case: single variable linear constraint
        if vars.len() == 1 && unbounded_vars.contains(&vars[0]) {
            let coeff = coeffs[0];
            let var_id = vars[0];
            
            if coeff.abs() < 1e-10 {
                return; // Degenerate constraint
            }
            
            match op {
                ComparisonOp::Le => {
                    if coeff > 0.0 {
                        let upper = constant / coeff;
                        self.update_bounds(bounds_map, var_id, None, Some(Val::ValF(upper)));
                    } else {
                        let lower = constant / coeff;
                        self.update_bounds(bounds_map, var_id, Some(Val::ValF(lower)), None);
                    }
                }
                ComparisonOp::Ge => {
                    if coeff > 0.0 {
                        let lower = constant / coeff;
                        self.update_bounds(bounds_map, var_id, Some(Val::ValF(lower)), None);
                    } else {
                        let upper = constant / coeff;
                        self.update_bounds(bounds_map, var_id, None, Some(Val::ValF(upper)));
                    }
                }
                ComparisonOp::Eq => {
                    let value = constant / coeff;
                    self.update_bounds(bounds_map, var_id, Some(Val::ValF(value)), Some(Val::ValF(value)));
                }
                _ => {}
            }
        }
    }
    
    /// Extract bounds from AllDifferent constraints
    fn extract_alldiff_bounds(
        &self,
        unbounded_var: VarId,
        all_vars: &[VarId],
        bounds_map: &mut std::collections::HashMap<VarId, (Option<Val>, Option<Val>)>,
    ) {
        // Find bounds of other variables in the AllDifferent constraint
        let mut global_min: Option<i32> = None;
        let mut global_max: Option<i32> = None;
        
        for &var_id in all_vars {
            if var_id != unbounded_var {
                if let Some((min, max)) = self.get_variable_bounds(var_id) {
                    global_min = Some(global_min.map_or(min, |gmin| gmin.min(min)));
                    global_max = Some(global_max.map_or(max, |gmax| gmax.max(max)));
                }
            }
        }
        
        // Use the range of other variables as a hint for the unbounded variable
        if let (Some(min), Some(max)) = (global_min, global_max) {
            // Expand slightly to account for AllDifferent constraint
            let expansion = (max - min).max(all_vars.len() as i32);
            self.update_bounds(
                bounds_map,
                unbounded_var,
                Some(Val::ValI(min - expansion / 2)),
                Some(Val::ValI(max + expansion / 2)),
            );
        }
    }
    
    /// Helper: Update bounds for a variable (taking tightest bounds)
    fn update_bounds(
        &self,
        bounds_map: &mut std::collections::HashMap<VarId, (Option<Val>, Option<Val>)>,
        var_id: VarId,
        new_min: Option<Val>,
        new_max: Option<Val>,
    ) {
        let entry = bounds_map.entry(var_id).or_insert((None, None));
        
        // Update minimum (take maximum of all lower bounds)
        if let Some(new_min_val) = new_min {
            entry.0 = Some(match entry.0 {
                None => new_min_val,
                Some(existing_min) => {
                    if new_min_val > existing_min {
                        new_min_val
                    } else {
                        existing_min
                    }
                }
            });
        }
        
        // Update maximum (take minimum of all upper bounds)
        if let Some(new_max_val) = new_max {
            entry.1 = Some(match entry.1 {
                None => new_max_val,
                Some(existing_max) => {
                    if new_max_val < existing_max {
                        new_max_val
                    } else {
                        existing_max
                    }
                }
            });
        }
    }
    
    /// Helper: Get current bounds of a variable
    fn get_variable_bounds(&self, var_id: VarId) -> Option<(i32, i32)> {
        match &self.vars[var_id] {
            crate::variables::Var::VarI(sparse_set) => {
                if !sparse_set.is_empty() {
                    Some((sparse_set.min(), sparse_set.max()))
                } else {
                    None
                }
            }
            crate::variables::Var::VarF(_) => None, // TODO: Handle float bounds
        }
    }
    
    /// Phase 3: Apply inferred bounds to a variable
    fn apply_inferred_bounds(
        &mut self,
        var_id: VarId,
        min_bound: &Option<Val>,
        max_bound: &Option<Val>,
    ) -> Result<(), SolverError> {
        match &mut self.vars[var_id] {
            crate::variables::Var::VarI(sparse_set) => {
                let current_min = sparse_set.min_universe_value();
                let current_max = sparse_set.max_universe_value();
                
                // Determine new bounds
                let new_min = if let Some(Val::ValI(min_val)) = min_bound {
                    if *min_val > current_min {
                        *min_val
                    } else {
                        current_min
                    }
                } else {
                    current_min
                };
                
                let new_max = if let Some(Val::ValI(max_val)) = max_bound {
                    if *max_val < current_max {
                        *max_val
                    } else {
                        current_max
                    }
                } else {
                    current_max
                };
                
                // Only update if bounds actually changed and are tighter
                if new_min > current_min || new_max < current_max {
                    // Reconstruct the variable with tighter bounds
                    let new_var = crate::variables::domain::SparseSet::new(new_min, new_max);
                    *sparse_set = new_var;
                }
            }
            crate::variables::Var::VarF(interval) => {
                // TODO: Apply bounds to float variables
                // For now, just update if we have float bounds
                if let Some(Val::ValF(min_val)) = min_bound {
                    if *min_val > interval.min {
                        interval.min = *min_val;
                    }
                }
                if let Some(Val::ValF(max_val)) = max_bound {
                    if *max_val < interval.max {
                        interval.max = *max_val;
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Check if a variable is still unbounded after inference
    fn is_still_unbounded(&self, var_id: VarId) -> bool {
        match &self.vars[var_id] {
            crate::variables::Var::VarI(sparse_set) => {
                let min_val = sparse_set.min_universe_value();
                let max_val = sparse_set.max_universe_value();
                min_val == i32::MIN || max_val == i32::MAX
            }
            crate::variables::Var::VarF(interval) => {
                interval.min == f64::NEG_INFINITY || interval.max == f64::INFINITY
            }
        }
    }
    
    /// Phase 4: Apply fallback bounds for variables that remain unbounded
    fn apply_fallback_bounds(&mut self, var_id: VarId) -> Result<(), SolverError> {
        // Use configured defaults or reasonable fallback
        let default_int_min = -1_000_000;
        let default_int_max = 1_000_000;
        let default_float_min = -1e6;
        let default_float_max = 1e6;
        
        match &mut self.vars[var_id] {
            crate::variables::Var::VarI(sparse_set) => {
                let current_min = sparse_set.min_universe_value();
                let current_max = sparse_set.max_universe_value();
                
                let new_min = if current_min == i32::MIN { default_int_min } else { current_min };
                let new_max = if current_max == i32::MAX { default_int_max } else { current_max };
                
                if new_min != current_min || new_max != current_max {
                    let new_var = crate::variables::domain::SparseSet::new(new_min, new_max);
                    *sparse_set = new_var;
                }
            }
            crate::variables::Var::VarF(interval) => {
                if interval.min == f64::NEG_INFINITY {
                    interval.min = default_float_min;
                }
                if interval.max == f64::INFINITY {
                    interval.max = default_float_max;
                }
            }
        }
        
        Ok(())
    }

    /// Optimize constraint processing order based on constraint characteristics.
    ///
    /// This method analyzes constraints (particularly AllDifferent) and reorders them
    /// to prioritize constraints with more fixed values, which tend to propagate more effectively.

    // Removed: optimize_constraint_order (no longer functional, see benchmarks and discussion)
    #[doc(hidden)]
    /// Create a search engine for this model that allows direct control over search.
    ///
    /// This provides access to lower-level search functionality including resource
    /// cleanup callbacks, custom iteration, and manual search control.
    ///
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(1, 10);
    /// let y = m.int(1, 10);
    /// m.new(x.ne(y));
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
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(1, 10);
    /// let y = m.int(1, 10);
    /// m.new(x.ne(y));
    /// 
    /// match m.solve() {
    ///     Ok(solution) => println!("Found: x={:?}, y={:?}", solution[x], solution[y]),
    ///     Err(e) => println!("No solution: {}", e),
    /// }
    /// ```
    #[must_use]
    pub fn solve(self) -> SolverResult<Solution> {
        // Check for constraint validation errors first
        if !self.constraint_validation_errors.is_empty() {
            return Err(self.constraint_validation_errors[0].clone());
        }
        
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
        let float_precision = self.float_precision_digits;
        let (vars, props, pending_lp) = self.prepare_for_search()?;
        
        // Capture counts before moving to search
        let var_count = vars.count();
        let constraint_count = props.count();
        
        let mut search_iter = search_with_timeout_and_memory(vars, props, mode::Enumerate, timeout, memory_limit, pending_lp, float_precision);
        
        let result = search_iter.next();
        
        // Capture statistics after search
        let stats = crate::core::solution::SolveStats {
            propagation_count: search_iter.get_propagation_count(),
            node_count: search_iter.get_node_count(),
            solve_time: search_iter.elapsed_time(),
            variables: var_count,
            constraint_count,
            peak_memory_mb: search_iter.get_memory_usage_mb(), // Direct MB usage
            lp_solver_used: false,
            lp_constraint_count: 0,
                    lp_variable_count: 0,
                    lp_stats: None,
                    bool_variables: 0,
                    float_variables: 0,
                    init_time: std::time::Duration::ZERO,
                    int_variables: 0,
                    objective: 0.0,
                    objective_bound: 0.0,
                    propagators: 0,
                    set_variables: 0,
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
    
    // Step 6.5: Try hybrid optimization approach for constraint satisfaction
    // Returns Some(solution) if hybrid solver succeeds, None if should fall back to search
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

    /// Extract linear constraints from the model for LP solving.
    ///
    /// Scans all propagators in the model and extracts float linear equality
    /// and inequality constraints into a LinearConstraintSystem that can be
    /// passed to the LP solver.
    ///
    /// # Returns
    /// A `LinearConstraintSystem` containing all linear constraints found.
    /// Use `is_suitable_for_lp()` to check if the system is worth solving with LP.
    ///
    /// # Example
    /// ```ignore
    /// use selen::prelude::*;
    /// 
    /// let mut m = Model::default();
    /// let x = m.float(0.0, 10.0);
    /// let y = m.float(0.0, 10.0);
    /// m.float_lin_eq(&[1.0, 2.0], &[x, y], 15.0);
    /// m.float_lin_le(&[3.0, 1.0], &[x, y], 20.0);
    /// 
    /// let linear_system = m.extract_linear_system();
    /// if linear_system.is_suitable_for_lp() {
    ///     // System has enough constraints to benefit from LP
    /// }
    /// ```
    pub fn extract_linear_system(&self) -> crate::lpsolver::LinearConstraintSystem {
        self.props.extract_linear_system()
    }

    /// Solve linear constraints using the LP solver and apply results to CSP domains.
    ///
    /// This method extracts linear constraints from the model, solves them using
    /// the LP solver, and tightens variable bounds based on the LP solution.
    /// This can significantly prune the search space before or during CSP search.
    ///
    /// # Arguments
    /// * `ctx` - Mutable context for updating variable domains
    ///
    /// # Returns
    /// * `Some(())` - LP solving succeeded and bounds were updated
    /// * `None` - LP solving failed or bounds update caused inconsistency
    ///
    /// # Example
    /// ```ignore
    /// use selen::prelude::*;
    /// 
    /// let mut m = Model::default();
    /// let x = m.float(0.0, 100.0);
    /// let y = m.float(0.0, 100.0);
    /// m.float_lin_eq(&[1.0, 1.0], &[x, y], 50.0);
    /// m.float_lin_le(&[2.0, 1.0], &[x, y], 80.0);
    /// 
    /// // During propagation, try LP solving
    /// let mut ctx = Context::new(&mut m.vars);
    /// if let Some(()) = m.solve_with_lp(&mut ctx) {
    ///     // LP solution successfully tightened bounds
    /// }
    /// ```
    pub fn solve_with_lp(&self, ctx: &mut crate::variables::views::Context) -> Option<()> {
        use crate::lpsolver::solve;
        
        // Extract linear constraints
        let linear_system = self.extract_linear_system();
        
        // Check if worth solving with LP
        if !linear_system.is_suitable_for_lp(ctx.vars()) {
            return Some(()); // Not enough constraints, skip LP
        }
        
        // Convert to LP problem format
        let lp_problem = linear_system.to_lp_problem(ctx.vars());
        
        // Solve LP problem
        let solution = match solve(&lp_problem) {
            Ok(sol) => sol,
            Err(_) => return Some(()), // LP solving failed, skip
        };
        
        // Apply LP solution to CSP domains
        crate::lpsolver::apply_lp_solution(&linear_system, &solution, ctx)
    }

    #[doc(hidden)]
    /// Internal helper that validates the model and optimizes constraints before search.
    /// This ensures all solving methods benefit from validation and constraint optimization.
    fn prepare_for_search(mut self) -> Result<(crate::variables::Vars, crate::constraints::props::Propagators, Vec<crate::lpsolver::csp_integration::LinearConstraint>), crate::core::error::SolverError> {
        // STEP 0: Infer bounds for unbounded variables using constraint AST analysis
        // This happens BEFORE materialization so we can analyze all constraints
        // and extract better bounds than the simple variable-context inference at creation time
        self.infer_unbounded_from_asts()?;
        
        // STEP 1: Materialize all pending constraint ASTs into propagators
        // This converts runtime API constraints (stored as AST) into CSP propagators
        // NOTE: We pass pending_lp_constraints separately so LP can use AST-extracted constraints
        // without scanning the materialized propagators (avoiding duplication)
        self.materialize_pending_asts();
        
        // STEP 2: Validate the model for common errors
        let validator = crate::core::validation::ModelValidator::new(&self.vars, &self.props);
        validator.validate()?;
        
    // STEP 3: Constraint ordering optimization removed (see benchmarks and discussion)
        
        Ok((self.vars, self.props, self.pending_lp_constraints))
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
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(1, 3);
    /// let y = m.int(1, 3);
    /// m.new(x.ne(y));  // x and y must be different
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
        let float_precision = self.float_precision_digits;
        match self.prepare_for_search() {
            Ok((vars, props, pending_lp)) => {
                Box::new(search_with_timeout_and_memory(vars, props, mode::Enumerate, timeout, memory_limit, pending_lp, float_precision)) as Box<dyn Iterator<Item = Solution>>
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
    /// use selen::prelude::*;
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
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(1, 3);
    /// let y = m.int(1, 3);
    /// m.new(x.ne(y));
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
        let float_precision = self.float_precision_digits;
        let (vars, props, pending_lp) = match self.prepare_for_search() {
            Ok(result) => result,
            Err(_) => {
                // Validation failed - report error stats and return empty vector
                let stats = crate::core::solution::SolveStats {
                    propagation_count: 0,
                    node_count: 0,
                    solve_time: std::time::Duration::ZERO,
                    variables: 0, // Unknown due to validation failure
                    constraint_count: 0, // Unknown due to validation failure
                    peak_memory_mb: 0, // No memory used if validation failed
                    lp_solver_used: false,
                    lp_constraint_count: 0,
                    lp_variable_count: 0,
                    lp_stats: None,
                    bool_variables: 0,
                    float_variables: 0,
                    init_time: std::time::Duration::ZERO,
                    int_variables: 0,
                    objective: 0.0,
                    objective_bound: 0.0,
                    propagators: 0,
                    set_variables: 0,
                };
                return (Vec::new(), stats);
            }
        };

        // Capture counts before moving to search
        let var_count = vars.count();
        let constraint_count = props.count();

        let mut search_iter = search_with_timeout_and_memory(vars, props, mode::Enumerate, timeout, memory_limit, pending_lp, float_precision);
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
            variables: var_count,
            constraint_count,
            peak_memory_mb: search_iter.get_memory_usage_mb(), // Direct MB usage
            lp_solver_used: false,
            lp_constraint_count: 0,
                    lp_variable_count: 0,
                    lp_stats: None,
                    bool_variables: 0,
                    float_variables: 0,
                    init_time: std::time::Duration::ZERO,
                    int_variables: 0,
                    objective: 0.0,
                    objective_bound: 0.0,
                    propagators: 0,
                    set_variables: 0,
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
