use crate::props::PropId;
use crate::search::{Space, SpaceState};
use crate::vars::{VarId, Val, Var};
use crate::views::Context;

/// Efficient branching strategy that uses SparseSet state management to minimize copying
/// This strategy is optimized for problems with many integer variables
pub fn split_with_sparse_state_management(space: Space) -> SparseStateBranching {
    if let Some(pivot) = space.vars.get_unassigned_var() {
        let var_domain = &space.vars[pivot];
        
        // Only use this strategy for integer variables with SparseSet domains
        if let Var::VarI(sparse_set) = var_domain {
            let mid = space.vars[pivot].mid();
            return SparseStateBranching {
                state: Some(SparseState::Initial {
                    space,
                    pivot,
                    mid,
                }),
            };
        }
    }
    
    // If no suitable variable found, return empty branching
    SparseStateBranching { state: None }
}

/// Branching iterator that uses saved states for efficient backtracking
#[derive(Debug)]
pub struct SparseStateBranching {
    state: Option<SparseState>,
}

#[derive(Debug)]
enum SparseState {
    /// Initial state - need to save state and try first branch
    Initial {
        space: Space,
        pivot: VarId,
        mid: Val,
    },
    /// After first branch - have saved state, can restore for second branch
    AfterFirstBranch {
        space: Space,
        saved_state: SpaceState,
        pivot: VarId,
        mid: Val,
    },
}

impl Iterator for SparseStateBranching {
    type Item = (Space, PropId);

    fn next(&mut self) -> Option<Self::Item> {
        let current_state = self.state.take()?;

        match current_state {
            SparseState::Initial { mut space, pivot, mid } => {
                // Save the current state before applying any constraints
                let saved_state = space.save_state();
                
                // Apply first constraint: pivot <= mid
                space.props.increment_node_count();
                let prop_id = space.props.less_than_or_equals(pivot, mid);
                
                // Prepare for second branch
                self.state = Some(SparseState::AfterFirstBranch {
                    space: space.clone(), // Clone once for the second branch
                    saved_state,
                    pivot,
                    mid,
                });
                
                Some((space, prop_id))
            }
            SparseState::AfterFirstBranch { mut space, saved_state, pivot, mid } => {
                // Restore to the saved state
                space.restore_state(&saved_state);
                
                // Apply second constraint: pivot > mid
                space.props.increment_node_count();
                let mut events = Vec::new();
                let _ctx = Context::new(&mut space.vars, &mut events);
                let prop_id = space.props.greater_than(pivot, mid);
                
                // No more branches after this
                self.state = None;
                
                Some((space, prop_id))
            }
        }
    }
}

impl SparseStateBranching {
    /// Get the current propagation count from the space being explored.
    pub fn get_propagation_count(&self) -> usize {
        match &self.state {
            Some(SparseState::Initial { space, .. }) => space.get_propagation_count(),
            Some(SparseState::AfterFirstBranch { space, .. }) => space.get_propagation_count(),
            None => 0,
        }
    }

    /// Get the current node count from the space being explored.
    pub fn get_node_count(&self) -> usize {
        match &self.state {
            Some(SparseState::Initial { space, .. }) => space.get_node_count(),
            Some(SparseState::AfterFirstBranch { space, .. }) => space.get_node_count(),
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vars::{Vars, Val};
    use crate::props::Propagators;

    #[test]
    fn test_sparse_state_branching() {
        let mut vars = Vars::default();
        let _var1 = vars.new_var_with_bounds(Val::ValI(1), Val::ValI(10));
        
        let mut props = Propagators::default();
        props.on_new_var(); // Initialize propagators to know about the variable
        
        let space = Space { vars, props };
        
        let mut branching = split_with_sparse_state_management(space);
        
        // Should produce exactly two branches
        let first_branch = branching.next();
        assert!(first_branch.is_some());
        
        let second_branch = branching.next();
        assert!(second_branch.is_some());
        
        let third_branch = branching.next();
        assert!(third_branch.is_none());
    }
}
