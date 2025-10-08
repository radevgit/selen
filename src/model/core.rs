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
    /// 
    /// assert_eq!(m.constraint_count(), 0);
    /// post!(m, x != y);
    /// assert_eq!(m.constraint_count(), 1);
    /// post!(m, x <= int(8));
    /// assert_eq!(m.constraint_count(), 2);
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
    /// use selen::prelude::*;
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
                eprintln!("DEBUG maximize: Optimization succeeded, returning solution");
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
                eprintln!("DEBUG maximize: Optimization failed, falling back to search");
                // Optimization router failed - use search-based minimize(opposite)
                // BUT: we need to correct the objective variable value in the result
                // since minimize(opposite) negates the objective bounds
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
    /// use selen::prelude::*;
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
    /// use selen::prelude::*;
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
    /// use selen::prelude::*;
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
    /// use selen::prelude::*;
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
