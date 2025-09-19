use crate::{prelude::Solution, props::Propagators, search::{agenda::Agenda, branch::{split_on_unassigned, SplitOnUnassigned}, mode::Mode}, vars::Vars, views::Context};

pub mod mode;

pub mod agenda;
pub mod branch;

/// Data required to perform search, now uses Clone for efficient backtracking.
#[derive(Clone, Debug)]
pub struct Space {
    pub vars: Vars,
    pub props: Propagators,
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

    /// Estimate memory usage for this space in KB (simple approximation)
    pub fn estimate_memory_kb(&self) -> usize {
        // Simple estimation based on variable count and domain complexity
        // Average variable with domain uses ~50-100 bytes
        let var_memory_kb = (self.vars.count() * 100) / 1024; // ~100 bytes per variable
        
        // Propagators are more complex, estimate ~200-500 bytes each
        let prop_memory_kb = (self.props.count() * 300) / 1024; // ~300 bytes per propagator
        
        // Base overhead for the space structure itself
        let base_memory_kb = 1; // 1KB base
        
        base_memory_kb + var_memory_kb + prop_memory_kb
    }
}

/// Perform search, iterating over assignments that satisfy all constraints.
pub fn search<M: Mode>(vars: Vars, props: Propagators, mode: M) -> Search<M> {
    search_with_timeout(vars, props, mode, None)
}

/// Perform search with timeout support.
pub fn search_with_timeout<M: Mode>(
    vars: Vars, 
    props: Propagators, 
    mode: M, 
    timeout: Option<std::time::Duration>
) -> Search<M> {
    search_with_timeout_and_memory(vars, props, mode, timeout, None)
}

/// Perform search with timeout and memory limit support.
pub fn search_with_timeout_and_memory<M: Mode>(
    vars: Vars, 
    props: Propagators, 
    mode: M, 
    timeout: Option<std::time::Duration>,
    memory_limit_mb: Option<u64>
) -> Search<M> {
    // Schedule all propagators during initial propagation step
    let agenda = Agenda::with_props(props.get_prop_ids_iter());

    // Propagate constraints until search is stalled or a solution is found
    let Some((is_stalled, space)) = propagate(Space { vars, props }, agenda) else {
        return Search::Done(None);
    };

    // Explore space by alternating branching and propagation
    if is_stalled {
        Search::Stalled(DefaultEngine::with_timeout_and_memory(space, mode, timeout, memory_limit_mb))
    } else {
        Search::Done(Some(space))
    }
}

/// Manual state machine until `gen` keyword is available (edition 2024).
pub enum Search<M> {
    Stalled(DefaultEngine<M>),
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
                let stats = crate::solution::SolveStats {
                    propagation_count: space.get_propagation_count(),
                    node_count: space.get_node_count(),
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

impl<M: Mode, B: Iterator<Item = (Space, crate::props::PropId)>> Iterator for Engine<M, B> {
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
                        let stats = crate::solution::SolveStats {
                            propagation_count: space.get_propagation_count(),
                            node_count: space.get_node_count(),
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
pub fn propagate(mut space: Space, mut agenda: Agenda) -> Option<(bool, Space)> {
    // Track which domains got updated, to schedule next propagators in batch
    let mut events = Vec::new();
    
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
