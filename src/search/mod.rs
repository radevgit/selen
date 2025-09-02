use crate::{prelude::Solution, props::Propagators, search::{agenda::Agenda, branch::{split_on_unassigned, SplitOnUnassigned}, mode::Mode}, vars::Vars, views::Context};

pub mod mode;

pub mod agenda;
pub mod branch;
mod value_branch;
mod hybrid_branch;
mod mcv_branch;

// Re-export the new branching strategies
pub use value_branch::{ValueBasedBranching, split_with_value_assignment};
pub use hybrid_branch::{HybridBranching, split_with_hybrid_strategy};
pub use mcv_branch::MCVBranching;

/// Data required to perform search, copied on branch and discarded on failure.
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
}

/// Perform search, iterating over assignments that satisfy all constraints.
pub fn search<M: Mode>(vars: Vars, props: Propagators, mode: M) -> Search<M> {
    // Schedule all propagators during initial propagation step
    let agenda = Agenda::with_props(props.get_prop_ids_iter());

    // Propagate constraints until search is stalled or a solution is found
    let Some((is_stalled, space)) = propagate(Space { vars, props }, agenda) else {
        return Search::Done(None);
    };

    // Explore space by alternating branching and propagation
    if is_stalled {
        Search::Stalled(Engine::new(space, mode))
    } else {
        Search::Done(Some(space))
    }
}

/// Perform search with custom branching strategy, iterating over assignments that satisfy all constraints.
pub fn search_with_branching<M: Mode, B: Iterator<Item = (Space, crate::props::PropId)> + Clone>(
    vars: Vars, 
    props: Propagators, 
    mode: M, 
    branching_factory: fn(Space) -> B
) -> SearchWithBranching<M, B> {
    // Schedule all propagators during initial propagation step
    let agenda = Agenda::with_props(props.get_prop_ids_iter());

    // Propagate constraints until search is stalled or a solution is found
    let Some((is_stalled, space)) = propagate(Space { vars, props }, agenda) else {
        return SearchWithBranching::Done(None);
    };

    // Explore space by alternating branching and propagation
    if is_stalled {
        SearchWithBranching::Stalled(Engine::new_with_branching(space, mode, branching_factory))
    } else {
        SearchWithBranching::Done(Some(space))
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
}

/// Manual state machine for custom branching strategies.
pub enum SearchWithBranching<M, B> {
    Stalled(Engine<M, B>),
    Done(Option<Space>),
}

impl<M, B: Iterator<Item = (Space, crate::props::PropId)> + Clone> SearchWithBranching<M, B> {
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
}

impl<M: Mode, B: Iterator<Item = (Space, crate::props::PropId)> + Clone> Iterator for SearchWithBranching<M, B> {
    type Item = Solution;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Stalled(engine) => engine.next(),
            Self::Done(space_opt) => space_opt.take().map(|space| space.vars.into_solution()),
        }
    }
}

impl<M: Mode> Iterator for Search<M> {
    type Item = Solution;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Stalled(engine) => engine.next(),
            Self::Done(space_opt) => space_opt.take().map(|space| space.vars.into_solution()),
        }
    }
}

/// Manual state machine until `gen` keyword is available (edition 2024).
pub struct Engine<M, B> {
    branch_iter: B,
    stack: Vec<Space>,
    mode: M,
    branching_factory: fn(Space) -> B,
    current_stats: Option<(usize, usize)>, // (propagation_count, node_count)
}

impl<M, B: Clone> Engine<M, B> {
    pub fn new_with_branching(space: Space, mode: M, branching_factory: fn(Space) -> B) -> Self {
        // Preserve a trail of copies to allow backtracking on failed spaces
        Self {
            branch_iter: branching_factory(space),
            stack: Vec::new(),
            mode,
            branching_factory,
            current_stats: None,
        }
    }
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
}

impl<M: Mode, B: Iterator<Item = (Space, crate::props::PropId)>> Iterator for Engine<M, B> {
    type Item = Solution;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
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
                        // Branch on new space, to explore it further
                        // Push current space to stack for backtracking
                        self.stack.push(space.clone());
                        // Create new branching iterator for the stalled space
                        self.branch_iter = (self.branching_factory)(space);
                        continue; // Continue with new branching iterator
                    } else {
                        // Mode object may update its internal state when new solutions are found
                        self.mode.on_solution(&space.vars);

                        // Extract solution assignment for all decision variables
                        return Some(space.vars.into_solution());
                    }
                }
            }

            // Pop from stack if we have anything there
            if let Some(parent_space) = self.stack.pop() {
                self.branch_iter = (self.branching_factory)(parent_space);
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
        let prop = space.props.get_state_mut(p);

        // Wrap engine objects before passing them to user-controlled propagation logic
        let mut ctx = Context::new(&mut space.vars, &mut events);

        // Prune decision variable domains to enforce constraints
        prop.prune(&mut ctx)?;

        // Schedule propagators that depend on changed variables
        #[allow(clippy::iter_with_drain)]
        for v in events.drain(..) {
            for p in space.props.on_bound_change(v) {
                agenda.schedule(p);
            }
        }

        // Search is over once all decision variables have been assigned
        if space.vars.is_assigned_all() {
            return Some((false, space));
        }
    }

    Some((true, space))
}
