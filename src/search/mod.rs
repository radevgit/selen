use crate::{prelude::Solution, constraints::props::Propagators, search::{agenda::Agenda, branch::{split_on_unassigned, SplitOnUnassigned}, mode::Mode}, variables::Vars, variables::views::Context};

/// Debug flag - set to false to disable LP solver debug output
const LP_DEBUG: bool = false;

#[doc(hidden)]
pub mod mode;

#[doc(hidden)]
pub mod agenda;
#[doc(hidden)]
pub mod branch;

/// Data required to perform search, now uses Clone for efficient backtracking.
#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct Space {
    pub vars: Vars,
    pub props: Propagators,
    /// Whether LP solver was used at root node
    pub lp_solver_used: bool,
    /// Number of linear constraints extracted for LP
    pub lp_constraint_count: usize,
    /// Number of variables used in LP solver
    pub lp_variable_count: usize,
    /// Statistics from LP solver (if used)
    pub lp_stats: Option<crate::lpsolver::types::LpStats>,
}

impl Space {
    /// Get the current propagation count from this space.
    pub fn get_propagation_count(&self) -> usize {
        self.props.get_propagation_count()
    }

    /// Get the current node count from this space.
    pub fn get_node_count(&self) -> usize {
        self.props.get_node_count()
    }

    /// Estimate memory usage for this space in MB (simple approximation)
    pub fn estimate_memory_mb(&self) -> usize {
        // Simple estimation based on variable count and domain complexity
        // Average variable with domain uses ~50-100 bytes
        let var_memory_bytes = self.vars.count() * 100;
        
        // Propagators are more complex, estimate ~200-500 bytes each
        let prop_memory_bytes = self.props.count() * 300;
        
        // Base overhead for the space structure itself
        let base_memory_bytes = 1024; // 1KB base
        
        // CSP memory in MB
        let csp_memory_mb = (base_memory_bytes + var_memory_bytes + prop_memory_bytes) / (1024 * 1024);
        
        // Add LP memory if LP solver was used
        let lp_memory_mb = if let Some(ref lp_stats) = self.lp_stats {
            lp_stats.peak_memory_mb as usize
        } else {
            0
        };
        
        // Total memory in MB (minimum 1MB for rounding)
        (csp_memory_mb + lp_memory_mb).max(1)
    }
}

/// Perform search, iterating over assignments that satisfy all constraints.
#[doc(hidden)]
pub fn search<M: Mode>(vars: Vars, props: Propagators, mode: M) -> Search<M> {
    search_with_timeout(vars, props, mode, None, vec![], crate::variables::domain::float_interval::DEFAULT_FLOAT_PRECISION_DIGITS)
}

/// Perform search with timeout support.
#[doc(hidden)]
pub fn search_with_timeout<M: Mode>(
    vars: Vars, 
    props: Propagators, 
    mode: M, 
    timeout: Option<std::time::Duration>,
    pending_lp_constraints: Vec<crate::lpsolver::csp_integration::LinearConstraint>,
    float_precision_digits: i32
) -> Search<M> {
    search_with_timeout_and_memory(vars, props, mode, timeout, None, pending_lp_constraints, float_precision_digits)
}

/// Perform search with timeout and memory limit support.
///
/// **LP Solver Integration:**
/// Before starting CSP search, this function automatically:
/// 1. Extracts linear constraints (FloatLinEq, FloatLinLe) from the propagator set
/// 2. Checks if LP solving is suitable (requires ≥2 constraints AND ≥2 variables)
/// 3. If suitable, solves the LP relaxation at the root node
/// 4. Tightens variable domains based on the LP solution
/// 5. Returns infeasible immediately if LP detects inconsistency
///
/// This happens **only once** at the root node, not during branch-and-bound search,
/// to avoid performance overhead. Non-linear problems are unaffected (zero overhead).
#[doc(hidden)]
pub fn search_with_timeout_and_memory<M: Mode>(
    vars: Vars, 
    props: Propagators, 
    mode: M, 
    timeout: Option<std::time::Duration>,
    memory_limit_mb: Option<u64>,
    pending_lp_constraints: Vec<crate::lpsolver::csp_integration::LinearConstraint>,
    float_precision_digits: i32
) -> Search<M> {
    // ===== LP SOLVER INTEGRATION (Root Node Only) =====
    // Try LP solving at root node if suitable linear system exists
    let (mut vars, props) = (vars, props);
    
    // Initialize LP tracking fields for Space
    let mut lp_solver_used = false;
    let mut lp_constraint_count = 0;
    let mut lp_variable_count = 0;
    let mut lp_stats_opt = None;
    
    {
        // Build linear system from TWO sources:
        // 1. AST-extracted constraints (runtime API: x.add(y).eq(z))
        //    - Extracted BEFORE materialization in post_constraint_kind()
        //    - No propagators created yet (delayed materialization)
        // 2. Propagator-scanned constraints (old API: m.add(x,y), m.mul(x,y))
        //    - Old API creates propagators immediately (no AST)
        //    - Must scan propagators to find linear relationships
        // Result: Both APIs work together without duplication
        // Build linear system from TWO sources:
        // 1. AST-extracted constraints (runtime API: x.add(y).eq(z))
        //    - Extracted BEFORE materialization in post_constraint_kind()
        //    - No propagators created yet (delayed materialization)
        // 2. Propagator-scanned constraints (old API: m.add(x,y), m.mul(x,y))
        //    - Old API creates propagators immediately (no AST)
        //    - Must scan propagators to find linear relationships
        // Result: Both APIs work together without duplication
        
        if LP_DEBUG {
            eprintln!("LP: Starting with {} AST-extracted constraints (runtime API)", pending_lp_constraints.len());
        }
        
        // Always scan propagators for old API constraints (m.add, m.mul, etc.)
        // These are created directly as propagators, not through AST
        let prop_system = props.extract_linear_system();
        if LP_DEBUG {
            eprintln!("LP: Found {} propagator constraints (old API)", prop_system.constraints.len());
        }
        
        // Build linear system from BOTH sources
        let mut linear_system = crate::lpsolver::csp_integration::LinearConstraintSystem::new();
        
        // Add AST-extracted constraints (runtime API)
        for constraint in pending_lp_constraints {
            linear_system.add_constraint(constraint);
        }
        
        // Add propagator-extracted constraints (old API)
        for constraint in prop_system.constraints {
            linear_system.add_constraint(constraint);
        }
        
        if LP_DEBUG {
            eprintln!("LP: Extracted {} total constraints, {} variables", 
                linear_system.constraints.len(), 
                linear_system.variables.len());
        }
        
        // Extract objective from Mode for LP solver
        let lp_has_objective = if let Some((var_id, minimize)) = mode.lp_objective() {
            if LP_DEBUG {
                eprintln!("LP: Extracted objective: variable {:?}, minimize={}", var_id, minimize);
            }
            
            // Find the index of this variable in the linear system
            if let Some(idx) = linear_system.variables.iter().position(|&v| v == var_id) {
                // Create objective vector: all zeros except 1.0 at the variable's position
                let mut coeffs = vec![0.0; linear_system.variables.len()];
                coeffs[idx] = 1.0;
                linear_system.set_objective(coeffs, minimize);
                if LP_DEBUG {
                    eprintln!("LP: Set objective for variable at index {} (minimize={})", idx, minimize);
                }
                true
            } else {
                if LP_DEBUG {
                    eprintln!("LP: Warning - objective variable {:?} not found in linear system (non-linear problem)", var_id);
                }
                false
            }
        } else {
            if LP_DEBUG {
                eprintln!("LP: No simple variable objective found (might be complex expression)");
            }
            false
        };
        
        let is_suitable = linear_system.is_suitable_for_lp(&vars);
        if LP_DEBUG {
            eprintln!("LP: is_suitable_for_lp() = {}, lp_has_objective = {}", is_suitable, lp_has_objective);
        }
        
        // Only use LP if: (1) suitable AND (2) has objective in linear system
        // Without objective in linear system, LP can't help optimize
        if is_suitable && lp_has_objective {
            if LP_DEBUG {
                eprintln!("LP: System is suitable for LP with objective, solving...");
            }
            // Convert to LP problem and solve
            let lp_problem = linear_system.to_lp_problem(&vars);
            if LP_DEBUG {
                eprintln!("LP: Problem has {} vars, {} constraints", lp_problem.n_vars, lp_problem.n_constraints);
            }
            
            // Capture LP statistics before solving
            lp_constraint_count = linear_system.constraints.len();
            lp_variable_count = linear_system.variables.len();
            
            // Create LP config with tolerance based on float precision
            // Also propagate timeout and memory limits from solver config
            let tolerance = crate::variables::domain::float_interval::precision_to_step_size(float_precision_digits);
            let lp_config = crate::lpsolver::LpConfig {
                feasibility_tol: tolerance,
                optimality_tol: tolerance,
                timeout_ms: timeout.map(|d| d.as_millis() as u64),
                max_memory_mb: memory_limit_mb,
                ..Default::default()
            };
            
            match crate::lpsolver::solve_with_config(&lp_problem, &lp_config) {
                Ok(solution) => {
                    if LP_DEBUG {
                        eprintln!("LP: Solution status = {:?}", solution.status);
                    }
                    
                    // Capture LP statistics from solution
                    lp_solver_used = true;
                    lp_stats_opt = Some(solution.stats.clone());
                    
                    // Check if LP found the problem infeasible
                    if solution.status == crate::lpsolver::LpStatus::Infeasible {
                        if LP_DEBUG {
                            eprintln!("LP: Problem is infeasible");
                        }
                        return Search::Done(None);
                    }
                    
                    // LP found optimal solution
                    if LP_DEBUG {
                        eprintln!("LP: Solution status = {:?}, objective = {}", solution.status, solution.objective);
                    }
                    
                    // Apply LP solution to tighten variable bounds
                    use crate::variables::views::Context;
                    let mut events = Vec::new();
                    let mut vars_mut = vars;
                    {
                        let mut ctx = Context::new(&mut vars_mut, &mut events);
                        if crate::lpsolver::csp_integration::apply_lp_solution(&linear_system, &solution, &mut ctx).is_none() {
                            if LP_DEBUG {
                                eprintln!("LP: Failed to apply solution (propagation failure)");
                            }
                            return Search::Done(None);
                        }
                    }
                    vars = vars_mut;
                    if LP_DEBUG {
                        eprintln!("LP: Successfully applied LP bounds");
                    }
                }
                Err(e) => {
                    if LP_DEBUG {
                        eprintln!("LP: Solver returned error: {:?}", e);
                    }
                    // Continue with CSP search without LP bounds
                }
            }
        }
    }
    // ===== End LP Integration =====

    // Schedule all propagators during initial propagation step
    let agenda = Agenda::with_props(props.get_prop_ids_iter());

    // Propagate constraints until search is stalled or a solution is found
    if LP_DEBUG {
        eprintln!("LP: Starting initial propagation...");
    }
    let Some((is_stalled, space)) = propagate(Space { 
        vars, 
        props,
        lp_solver_used,
        lp_constraint_count,
        lp_variable_count,
        lp_stats: lp_stats_opt,
    }, agenda) else {
        if LP_DEBUG {
            eprintln!("LP: Initial propagation returned None (infeasible)");
        }
        return Search::Done(None);
    };
    if LP_DEBUG {
        eprintln!("LP: Initial propagation succeeded, is_stalled={}", is_stalled);
    }

    // Explore space by alternating branching and propagation
    if is_stalled {
        Search::Stalled(Box::new(DefaultEngine::with_timeout_and_memory(space, mode, timeout, memory_limit_mb)))
    } else {
        Search::Done(Some(space))
    }
}

/// Manual state machine until `gen` keyword is available (edition 2024).
pub enum Search<M> {
    Stalled(Box<DefaultEngine<M>>),
    Done(Option<Space>),
}

impl<M> Search<M> {
    /// Get the current propagation count from the search state.
    pub fn get_propagation_count(&self) -> usize {
        match self {
            Self::Stalled(engine) => engine.get_propagation_count(),
            Self::Done(Some(space)) => space.get_propagation_count(),
            Self::Done(None) => 0, // Failed search, no space available
        }
    }

    /// Get the current node count from the search state.
    pub fn get_node_count(&self) -> usize {
        match self {
            Self::Stalled(engine) => engine.get_node_count(),
            Self::Done(Some(space)) => space.get_node_count(),
            Self::Done(None) => 0, // Failed search, no space available
        }
    }

    /// Check if the search has timed out
    pub fn is_timed_out(&self) -> bool {
        match self {
            Self::Stalled(engine) => engine.is_timed_out(),
            Self::Done(_) => false, // Completed searches cannot timeout
        }
    }

    /// Get the elapsed time since search started
    pub fn elapsed_time(&self) -> std::time::Duration {
        match self {
            Self::Stalled(engine) => engine.elapsed_time(),
            Self::Done(_) => std::time::Duration::from_secs(0), // Completed searches don't track time
        }
    }

    /// Check if memory limit has been exceeded
    pub fn is_memory_limit_exceeded(&self) -> bool {
        match self {
            Self::Stalled(engine) => engine.is_memory_limit_exceeded(),
            Self::Done(_) => false, // Completed searches cannot exceed memory
        }
    }

    /// Get current memory usage estimate in MB
    pub fn get_memory_usage_mb(&self) -> usize {
        match self {
            Self::Stalled(engine) => engine.get_memory_usage_mb(),
            Self::Done(_) => 0, // Completed searches don't track memory
        }
    }
}

impl<M: Mode> Iterator for Search<M> {
    type Item = Solution;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Stalled(engine) => engine.next(),
            Self::Done(space_opt) => space_opt.take().map(|space| {
                let stats = crate::core::solution::SolveStats {
                    propagation_count: space.get_propagation_count(),
                    node_count: space.get_node_count(),
                    solve_time: std::time::Duration::ZERO, // TODO: Track solve time in Space
                    variables: space.vars.count(),
                    constraint_count: space.props.count(),
                    peak_memory_mb: space.estimate_memory_mb(),
                    int_variables: space.vars.int_var_count,
                    bool_variables: space.vars.bool_var_count,
                    float_variables: space.vars.float_var_count,
                    set_variables: space.vars.set_var_count,
                    propagators: space.props.count(),
                    lp_solver_used: space.lp_solver_used,
                    lp_constraint_count: space.lp_constraint_count,
                    lp_variable_count: space.lp_variable_count,
                    lp_stats: space.lp_stats,
                    init_time: std::time::Duration::ZERO,
                    objective: 0.0,
                    objective_bound: 0.0,
                };
                space.vars.into_solution_with_stats(stats)
            }),
        }
    }
}

/// Manual state machine until `gen` keyword is available (edition 2024).
pub struct Engine<M, B> {
    branch_iter: B,
    stack: Vec<B>, // Store branching iterators instead of spaces
    mode: M,
    branching_factory: fn(Space) -> B,
    current_stats: Option<(usize, usize)>, // (propagation_count, node_count)
    // Timeout support
    start_time: std::time::Instant,
    timeout_duration: Option<std::time::Duration>,
    // Memory limit support
    memory_limit_mb: Option<u64>,
    // Optimization: only check timeout/memory periodically
    iteration_count: usize,
    timeout_check_interval: usize,
    // Resource cleanup support
    cleanup_callbacks: Vec<Box<dyn FnOnce() + Send>>,
    is_interrupted: bool,
}

/// Default Engine with SplitOnUnassigned for backwards compatibility
pub type DefaultEngine<M> = Engine<M, SplitOnUnassigned>;

impl<M> DefaultEngine<M> {
    pub fn new(space: Space, mode: M) -> Self {
        // Preserve a trail of copies to allow backtracking on failed spaces
        Self {
            branch_iter: split_on_unassigned(space),
            stack: Vec::new(),
            mode,
            branching_factory: split_on_unassigned,
            current_stats: None,
            start_time: std::time::Instant::now(),
            timeout_duration: None,
            memory_limit_mb: None,
            iteration_count: 0,
            timeout_check_interval: 10000, // Check every 10K iterations for minimal overhead
            cleanup_callbacks: Vec::new(),
            is_interrupted: false,
        }
    }

    pub fn with_timeout(space: Space, mode: M, timeout: Option<std::time::Duration>) -> Self {
        // Preserve a trail of copies to allow backtracking on failed spaces
        Self {
            branch_iter: split_on_unassigned(space),
            stack: Vec::new(),
            mode,
            branching_factory: split_on_unassigned,
            current_stats: None,
            start_time: std::time::Instant::now(),
            timeout_duration: timeout,
            memory_limit_mb: None,
            iteration_count: 0,
            timeout_check_interval: 10000, // Check every 10K iterations for minimal overhead
            cleanup_callbacks: Vec::new(),
            is_interrupted: false,
        }
    }

    pub fn with_timeout_and_memory(
        space: Space, 
        mode: M, 
        timeout: Option<std::time::Duration>,
        memory_limit_mb: Option<u64>
    ) -> Self {
        // Preserve a trail of copies to allow backtracking on failed spaces
        Self {
            branch_iter: split_on_unassigned(space),
            stack: Vec::new(),
            mode,
            branching_factory: split_on_unassigned,
            current_stats: None,
            start_time: std::time::Instant::now(),
            timeout_duration: timeout,
            memory_limit_mb,
            iteration_count: 0,
            timeout_check_interval: 10000, // Check every 10K iterations for minimal overhead
            cleanup_callbacks: Vec::new(),
            is_interrupted: false,
        }
    }
}

// Automatic cleanup when Engine is dropped
impl<M, B> Drop for Engine<M, B> {
    fn drop(&mut self) {
        // Only trigger cleanup if we haven't already been interrupted
        if !self.is_interrupted {
            self.trigger_cleanup();
        }
    }
}

impl<M, B> Engine<M, B> {
    /// Get the current propagation count from the engine's current state.
    pub fn get_propagation_count(&self) -> usize {
        // Return the tracked statistics if available
        self.current_stats.map(|(prop_count, _)| prop_count).unwrap_or(0)
    }

    /// Get the current node count from the engine's current state.
    pub fn get_node_count(&self) -> usize {
        // Return the tracked statistics if available  
        self.current_stats.map(|(_, node_count)| node_count).unwrap_or(0)
    }

    /// Check if the search has timed out
    pub fn is_timed_out(&self) -> bool {
        if let Some(timeout_duration) = self.timeout_duration {
            self.start_time.elapsed() >= timeout_duration
        } else {
            false
        }
    }

    /// Get the elapsed time since search started
    pub fn elapsed_time(&self) -> std::time::Duration {
        self.start_time.elapsed()
    }

    /// Check if memory limit has been exceeded
    pub fn is_memory_limit_exceeded(&self) -> bool {
        if let Some(limit_mb) = self.memory_limit_mb {
            self.get_memory_usage_mb() > limit_mb as usize
        } else {
            false
        }
    }

    /// Get current memory usage estimate in MB (simple approximation)
    pub fn get_memory_usage_mb(&self) -> usize {
        // Base memory for the engine itself and initial structures
        let base_memory_kb = 512; // ~0.5MB base overhead
        
        // Stack memory (each stack frame contains Space which has vars + props)
        // Use actual Space memory estimation when possible
        let stack_memory_kb = self.stack.len() * 3; // Conservative 3KB per frame
        
        // Current branching iterator state (approximate)
        let current_memory_kb = 2; // ~2KB for current iterator state
        
        // Iteration overhead (accumulated search state and temporary allocations)
        // Very conservative estimate: ~5KB per 10K iterations
        let iteration_memory_kb = (self.iteration_count / 10000) * 5;
        
        // Convert to MB with minimum of 1MB
        ((base_memory_kb + stack_memory_kb + current_memory_kb + iteration_memory_kb) / 1024).max(1)
    }
    
    /// Register a cleanup callback to be called when solving is interrupted
    pub fn register_cleanup<F>(&mut self, cleanup: F) 
    where 
        F: FnOnce() + Send + 'static
    {
        self.cleanup_callbacks.push(Box::new(cleanup));
    }
    
    /// Mark the engine as interrupted and trigger cleanup
    fn trigger_cleanup(&mut self) {
        if !self.is_interrupted {
            self.is_interrupted = true;
            
            // Execute all cleanup callbacks
            for cleanup in self.cleanup_callbacks.drain(..) {
                cleanup();
            }
        }
    }
    
    /// Check if the engine has been interrupted
    pub fn is_interrupted(&self) -> bool {
        self.is_interrupted
    }
}

impl<M: Mode, B: Iterator<Item = (Space, crate::constraints::props::PropId)>> Iterator for Engine<M, B> {
    type Item = Solution;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Periodically check timeout and memory limits to reduce overhead
            self.iteration_count += 1;
            if self.iteration_count % self.timeout_check_interval == 0 {
                // Check timeout
                if let Some(timeout_duration) = self.timeout_duration {
                    if self.start_time.elapsed() >= timeout_duration {
                        // Timeout exceeded - trigger cleanup before returning
                        self.trigger_cleanup();
                        return None;
                    }
                }
                
                // Check memory limit
                if let Some(limit_mb) = self.memory_limit_mb {
                    if self.get_memory_usage_mb() > limit_mb as usize {
                        // Memory limit exceeded - trigger cleanup before returning
                        self.trigger_cleanup();
                        return None;
                    }
                }
            }

            while let Some((mut space, p)) = self.branch_iter.next() {
                // Increment node count when exploring a new branch
                space.props.increment_node_count();
                
                // Schedule propagator triggered by the branch
                let agenda =
                    Agenda::with_props(self.mode.on_branch(&mut space).chain(core::iter::once(p)));

                // Failed spaces are discarded, fixed points get explored further (depth-first search)
                if let Some((is_stalled, space)) = propagate(space, agenda) {
                    // Update statistics tracking
                    self.current_stats = Some((space.get_propagation_count(), space.get_node_count()));
                    
                    if is_stalled {
                        // Save the current iterator state before branching deeper
                        let current_iter = std::mem::replace(&mut self.branch_iter, (self.branching_factory)(space));
                        self.stack.push(current_iter);
                        continue; // Continue with new branching iterator
                    } else {
                        // Mode object may update its internal state when new solutions are found
                        self.mode.on_solution(&space.vars);

                        // Extract solution assignment for all decision variables with current statistics
                        let stats = crate::core::solution::SolveStats {
                            propagation_count: space.get_propagation_count(),
                            node_count: space.get_node_count(),
                            solve_time: std::time::Duration::ZERO, // TODO: Track solve time in Engine
                            variables: space.vars.count(),
                            constraint_count: space.props.count(),
                            peak_memory_mb: space.estimate_memory_mb(),
                            int_variables: space.vars.int_var_count,
                            bool_variables: space.vars.bool_var_count,
                            float_variables: space.vars.float_var_count,
                            set_variables: space.vars.set_var_count,
                            propagators: space.props.count(),
                            lp_solver_used: space.lp_solver_used,
                            lp_constraint_count: space.lp_constraint_count,
                            lp_variable_count: space.lp_variable_count,
                            lp_stats: space.lp_stats,
                            init_time: std::time::Duration::ZERO,
                            objective: 0.0,
                            objective_bound: 0.0,
                        };
                        return Some(space.vars.into_solution_with_stats(stats));
                    }
                }
            }

            // Pop from stack if we have anything there
            if let Some(parent_iter) = self.stack.pop() {
                self.branch_iter = parent_iter;
            } else {
                return None;
            }
        }
    }
}

/// Apply scheduled propagators, pruning domains until space is failed, stalled, or assigned.
#[doc(hidden)]
pub fn propagate(mut space: Space, mut agenda: Agenda) -> Option<(bool, Space)> {
    // Track which domains got updated, to schedule next propagators in batch
    let mut events = Vec::with_capacity(16);
    
    // Agenda establishes the order in which scheduled propagators get run
    while let Some(p) = agenda.pop() {
        
        // Increment the propagation step counter
        space.props.increment_propagation_count();

        // Acquire trait object for propagator, which points to both code and inner state
        let prop = space.props.get_state(p);

        // Wrap engine objects before passing them to user-controlled propagation logic
        let mut ctx = Context::new(&mut space.vars, &mut events);
        
        // Prune decision variable domains to enforce constraints
        let result = prop.as_ref().prune(&mut ctx);
        result?;
        
        // Schedule propagators that depend on changed variables
        #[allow(clippy::iter_with_drain)]
        for v in events.drain(..) {
            for p in space.props.on_bound_change(v) {
                agenda.schedule(p);
            }
        }
    }

    // Search is over once all decision variables have been assigned AND agenda is empty
    // (meaning all constraints have been checked on the final assignment)
    if space.vars.is_assigned_all() {
        Some((false, space))
    } else {
        Some((true, space))
    }
}
