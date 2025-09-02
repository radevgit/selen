use std::mem::replace;

use crate::{prelude::Solution, props::Propagators, search::{agenda::Agenda, branch::{split_on_unassigned, SplitOnUnassigned}, mode::Mode}, vars::Vars, views::Context};

pub mod mode;

mod agenda;
mod branch;
mod enhanced_branch;

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

/// Manual state machine until `gen` keyword is available (edition 2024).
pub enum Search<M> {
    Stalled(Engine<M>),
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
pub struct Engine<M> {
    branch_iter: SplitOnUnassigned,
    stack: Vec<SplitOnUnassigned>,
    mode: M,
}

impl<M> Engine<M> {
    fn new(space: Space, mode: M) -> Self {
        // Preserve a trail of copies to allow backtracking on failed spaces
        Self {
            branch_iter: split_on_unassigned(space),
            stack: Vec::new(),
            mode,
        }
    }

    /// Get the current propagation count from the engine's current state.
    pub fn get_propagation_count(&self) -> usize {
        // Try to get the count from the current branch iterator
        let current_count = self.branch_iter.get_propagation_count();
        
        // If that's 0, try to get it from the stack
        if current_count == 0 && !self.stack.is_empty() {
            // Get the count from the last item in the stack
            self.stack.last()
                .map(|split| split.get_propagation_count())
                .unwrap_or(0)
        } else {
            current_count
        }
    }

    /// Get the current node count from the engine's current state.
    pub fn get_node_count(&self) -> usize {
        // Try to get the count from the current branch iterator
        let current_count = self.branch_iter.get_node_count();
        
        // If that's 0, try to get it from the stack
        if current_count == 0 && !self.stack.is_empty() {
            // Get the count from the last item in the stack
            self.stack.last()
                .map(|split| split.get_node_count())
                .unwrap_or(0)
        } else {
            current_count
        }
    }
}

impl<M: Mode> Iterator for Engine<M> {
    type Item = Solution;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            while let Some((mut space, p)) = self.branch_iter.next() {
                // Schedule propagator triggered by the branch
                let agenda =
                    Agenda::with_props(self.mode.on_branch(&mut space).chain(core::iter::once(p)));

                // Failed spaces are discarded, fixed points get explored further (depth-first search)
                if let Some((is_stalled, space)) = propagate(space, agenda) {
                    if is_stalled {
                        // Branch on new space, to explore it further
                        let parent = replace(&mut self.branch_iter, split_on_unassigned(space));

                        // Save where search will resume if sub-space gets failed
                        self.stack.push(parent);
                    } else {
                        // Mode object may update its internal state when new solutions are found
                        self.mode.on_solution(&space.vars);

                        // Extract solution assignment for all decision variables
                        return Some(space.vars.into_solution());
                    }
                }
            }

            self.branch_iter = self.stack.pop()?;
        }
    }
}

/// Apply scheduled propagators, pruning domains until space is failed, stalled, or assigned.
fn propagate(mut space: Space, mut agenda: Agenda) -> Option<(bool, Space)> {
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
