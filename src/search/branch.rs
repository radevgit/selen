use crate::props::PropId;
use crate::search::Space;
use crate::vars::{VarId, Val};
use crate::views::Context;

/// Perform a binary split on the first unassigned decision variable.
/// Uses efficient SparseSet state management for integer variables.
/// Always uses binary splitting - value enumeration has been eliminated based on empirical evidence.
pub fn split_on_unassigned(space: Space) -> SplitOnUnassigned {
    if let Some(pivot) = space.vars.get_unassigned_var() {
        // Always use binary split (empirical evidence shows no benefit from value enumeration)
        let mid = space.vars[pivot].mid();
        SplitOnUnassigned {
            branch: Some(BranchState::BinarySplit {
                space, 
                pivot, 
                mid, 
                is_left: true
            }),
        }
    } else {
        SplitOnUnassigned { branch: None }
    }
}

/// Manual state machine until `gen` keyword is available (edition 2024).
#[derive(Debug)]
pub struct SplitOnUnassigned {
    branch: Option<BranchState>,
}

#[derive(Debug)]
enum BranchState {
    /// Binary split for domains (always produces exactly 2 branches)
    BinarySplit {
        space: Space,
        pivot: VarId,
        mid: Val,
        is_left: bool,
    },
}

impl SplitOnUnassigned {
    /// Get the current propagation count from the space being explored.
    pub fn get_propagation_count(&self) -> usize {
        match &self.branch {
            Some(BranchState::BinarySplit { space, .. }) => space.get_propagation_count(),
            None => 0,
        }
    }

    /// Get the current node count from the space being explored.
    pub fn get_node_count(&self) -> usize {
        match &self.branch {
            Some(BranchState::BinarySplit { space, .. }) => space.get_node_count(),
            None => 0,
        }
    }
}

impl Iterator for SplitOnUnassigned {
    type Item = (Space, PropId);

    fn next(&mut self) -> Option<Self::Item> {
        let branch_state = self.branch.take()?;

        match branch_state {
            BranchState::BinarySplit { space, pivot, mid, is_left } => {
                if is_left {
                    // Left branch: pivot <= mid
                    let mut space_left = space.clone();
                    space_left.props.increment_node_count();
                    let p = space_left.props.less_than_or_equals(pivot, mid);
                    
                    // Set up for right branch
                    self.branch = Some(BranchState::BinarySplit { 
                        space, 
                        pivot, 
                        mid, 
                        is_left: false 
                    });
                    
                    Some((space_left, p))
                } else {
                    // Right branch: pivot > mid
                    let mut space_right = space;
                    space_right.props.increment_node_count();
                    
                    let mut events = Vec::new();
                    let _ctx = Context::new(&mut space_right.vars, &mut events);
                    let p = space_right.props.greater_than(pivot, mid);
                    
                    // No more branches after this
                    Some((space_right, p))
                }
            }
        }
    }
}
