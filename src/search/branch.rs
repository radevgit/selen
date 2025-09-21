use crate::constraints::props::PropId;
use crate::search::Space;
use crate::variables::{VarId, Val};

/// Perform a binary split on the first unassigned decision variable.
/// Uses efficient clone-based state management.
/// Always uses binary splitting - value enumeration has been eliminated based on empirical evidence.
pub fn split_on_unassigned(space: Space) -> SplitOnUnassigned {
    if let Some(pivot) = space.vars.get_unassigned_var() {
        // Always use binary split (empirical evidence shows no benefit from value enumeration)
        let mid = space.vars[pivot].mid();
        let saved_space = space.clone(); // Save entire space for backtracking
        SplitOnUnassigned {
            branch: Some(BranchState::BinarySplit {
                space, 
                saved_space,
                pivot, 
                mid, 
                is_left: true
            }),
        }
    } else {
        SplitOnUnassigned { branch: None }
    }
}

#[derive(Debug)]
pub struct SplitOnUnassigned {
    branch: Option<BranchState>,
}

#[derive(Debug)]
enum BranchState {
    /// Binary split for domains (always produces exactly 2 branches)
    BinarySplit {
        space: Space,
        saved_space: Space, // Save entire space for backtracking via clone
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
            BranchState::BinarySplit { mut space, saved_space, pivot, mid, is_left } => {
                space.props.increment_node_count();
                
                if is_left {
                    // Left branch: pivot <= mid
                    // Add left branch constraint: pivot <= mid
                    let p = space.props.less_than_or_equals(pivot, mid);
                    
                    // Set up right branch using saved space
                    self.branch = Some(BranchState::BinarySplit { 
                        space: saved_space.clone(),
                        saved_space,
                        pivot, 
                        mid, 
                        is_left: false 
                    });
                    
                    // Return left branch
                    Some((space, p))
                } else {
                    // Right branch: pivot > mid
                    // Use the saved space for the right branch  
                    let mut right_space = saved_space;
                    let p = right_space.props.greater_than(pivot, mid);
                    
                    Some((right_space, p))
                }
            }
        }
    }
}
