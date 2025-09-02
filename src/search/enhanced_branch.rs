use crate::props::PropId;
use crate::search::Space;
use crate::utils::{float_next, float_prev};
use crate::vars::{VarId, Val};
use crate::views::{Context, ViewType};
use std::collections::HashMap;

/// Enhanced branching strategy that can split around forbidden values
/// to create "holes" in domains through branching.
pub struct SplitAroundForbiddenValues {
    /// Current branching state
    state: BranchState,
}

#[derive(Debug)]
enum BranchState {
    /// Find a variable to split around forbidden values
    FindVariable(Space),
    /// Split around a specific forbidden value
    SplitValue {
        space: Space,
        var: VarId,
        forbidden_value: Val,
        left_done: bool,
    },
    /// Fall back to normal mid-point splitting
    FallbackSplit {
        space: Space,
        var: VarId,
        mid: Val,
        left_done: bool,
    },
    /// No more branches
    Done,
}

impl SplitAroundForbiddenValues {
    /// Create a new enhanced branching strategy
    pub fn new(space: Space) -> Self {
        Self {
            state: BranchState::FindVariable(space),
        }
    }
    
    /// Find a variable that has forbidden values and would benefit from splitting
    fn find_variable_with_forbidden_values(&self, space: &Space) -> Option<(VarId, Val)> {
        // Look for unassigned variables that have forbidden values
        // This is a simplified approach - in a full implementation, we'd need
        // access to the constraint store to find forbidden values
        
        // For now, let's find any unassigned variable and use mid-point splitting
        // In a full implementation, we'd track forbidden values globally
        if let Some(var) = space.vars.get_unassigned_var() {
            let var_domain = &space.vars[var];
            let mid = var_domain.mid();
            Some((var, mid))
        } else {
            None
        }
    }
    
    /// Get the current propagation count from the space being explored.
    pub fn get_propagation_count(&self) -> usize {
        match &self.state {
            BranchState::FindVariable(space) => space.get_propagation_count(),
            BranchState::SplitValue { space, .. } => space.get_propagation_count(),
            BranchState::FallbackSplit { space, .. } => space.get_propagation_count(),
            BranchState::Done => 0,
        }
    }

    /// Get the current node count from the space being explored.
    pub fn get_node_count(&self) -> usize {
        match &self.state {
            BranchState::FindVariable(space) => space.get_node_count(),
            BranchState::SplitValue { space, .. } => space.get_node_count(),
            BranchState::FallbackSplit { space, .. } => space.get_node_count(),
            BranchState::Done => 0,
        }
    }
}

impl Iterator for SplitAroundForbiddenValues {
    type Item = (Space, PropId);

    fn next(&mut self) -> Option<Self::Item> {
        match std::mem::replace(&mut self.state, BranchState::Done) {
            BranchState::FindVariable(space) => {
                // Try to find a variable with forbidden values to split around
                if let Some((var, forbidden_value)) = self.find_variable_with_forbidden_values(&space) {
                    // Start splitting around this forbidden value
                    self.state = BranchState::SplitValue {
                        space,
                        var,
                        forbidden_value,
                        left_done: false,
                    };
                    self.next() // Recursive call to handle the split
                } else {
                    // No variables to split
                    None
                }
            }
            BranchState::SplitValue { space, var, forbidden_value, left_done } => {
                if !left_done {
                    // Create left branch: var < forbidden_value
                    let mut left_space = space.clone();
                    left_space.props.increment_node_count();
                    
                    // Add constraint: var <= forbidden_value.prev() (i.e., var < forbidden_value)
                    let upper_bound = match forbidden_value {
                        Val::ValI(fv) => Val::ValI(fv - 1),
                        Val::ValF(fv) => Val::ValF(float_prev(fv)),
                    };
                    
                    let prop_id = left_space.props.less_than_or_equals(var, upper_bound);
                    
                    // Prepare for right branch
                    self.state = BranchState::SplitValue {
                        space,
                        var,
                        forbidden_value,
                        left_done: true,
                    };
                    
                    Some((left_space, prop_id))
                } else {
                    // Create right branch: var > forbidden_value
                    let mut right_space = space;
                    right_space.props.increment_node_count();
                    
                    // Add constraint: var >= forbidden_value.next() (i.e., var > forbidden_value)
                    let lower_bound = match forbidden_value {
                        Val::ValI(fv) => Val::ValI(fv + 1),
                        Val::ValF(fv) => Val::ValF(float_next(fv)),
                    };
                    
                    let mut events = Vec::new();
                    let _ctx = Context::new(&mut right_space.vars, &mut events);
                    let prop_id = right_space.props.greater_than(var, lower_bound);
                    
                    Some((right_space, prop_id))
                }
            }
            BranchState::FallbackSplit { space, var, mid, left_done } => {
                // Standard mid-point splitting (fallback)
                if !left_done {
                    // Left branch: var <= mid
                    let mut left_space = space.clone();
                    left_space.props.increment_node_count();
                    
                    let prop_id = left_space.props.less_than_or_equals(var, mid);
                    
                    self.state = BranchState::FallbackSplit {
                        space,
                        var,
                        mid,
                        left_done: true,
                    };
                    
                    Some((left_space, prop_id))
                } else {
                    // Right branch: var > mid
                    let mut right_space = space;
                    right_space.props.increment_node_count();
                    
                    let mut events = Vec::new();
                    let _ctx = Context::new(&mut right_space.vars, &mut events);
                    let prop_id = right_space.props.greater_than(var, mid);
                    
                    Some((right_space, prop_id))
                }
            }
            BranchState::Done => None,
        }
    }
}

/// Create an enhanced branching strategy that handles forbidden values
pub fn split_with_forbidden_values(space: Space) -> SplitAroundForbiddenValues {
    SplitAroundForbiddenValues::new(space)
}
