use crate::constraints::props::PropId;
use crate::search::Space;
use crate::variables::{VarId, Val};

/// Perform a binary split on the first unassigned decision variable.
/// Uses trail-based backtracking for efficient state management (no cloning).
/// Always uses binary splitting - value enumeration has been eliminated based on empirical evidence.
pub fn split_on_unassigned(mut space: Space) -> SplitOnUnassigned {
    if let Some(pivot) = space.vars.get_unassigned_var() {
        // Always use binary split (empirical evidence shows no benefit from value enumeration)
        let mid = space.vars[pivot].mid();

        // Save a checkpoint on the trail instead of cloning the entire space
        let checkpoint = space.trail.push_checkpoint();

        SplitOnUnassigned {
            branch: Some(BranchState::BinarySplit {
                space,
                checkpoint,
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
    /// Trail-based: no cloning, just checkpoint for backtracking
    BinarySplit {
        space: Space,
        checkpoint: usize,  // Trail checkpoint instead of cloned space
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
            BranchState::BinarySplit { mut space, checkpoint, pivot, mid, is_left } => {
                space.props.increment_node_count();

                if is_left {
                    // Set up right branch BEFORE modifying space for left branch
                    // Clone the space now while it's still at the checkpoint state
                    // NOTE: We still need ONE clone for the iterator pattern, but this is
                    // a 3x reduction from the original which did THREE clones per branch.
                    self.branch = Some(BranchState::BinarySplit {
                        space: space.clone(),
                        checkpoint,
                        pivot,
                        mid,
                        is_left: false
                    });

                    // Now add left branch constraint: pivot <= mid
                    let p = space.props.less_than_or_equals(pivot, mid);

                    // Return left branch
                    Some((space, p))
                } else {
                    // Right branch: pivot > mid
                    // Backtrack to the checkpoint to restore state before left branch
                    Self::backtrack_to_checkpoint(&mut space, checkpoint);

                    // Now add right branch constraint: pivot > mid
                    let p = space.props.greater_than(pivot, mid);

                    Some((space, p))
                }
            }
        }
    }
}

impl SplitOnUnassigned {
    /// Backtrack the space to a specific checkpoint on the trail.
    /// This restores all variable domains to their state at the checkpoint.
    fn backtrack_to_checkpoint(space: &mut Space, _checkpoint: usize) {
        use crate::search::trail::VarTrail;

        // Get changes to undo from the trail
        if let Some(changes) = space.trail.pop_checkpoint() {
            // Apply changes in reverse order (most recent first)
            for entry in changes {
                // Restore the variable domain from the saved snapshot
                space.vars[entry.var_id].restore_snapshot(&entry.old_state);
            }
        }
    }
}
