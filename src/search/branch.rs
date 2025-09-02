use crate::props::PropId;
use crate::search::Space;
use crate::vars::{VarId, Val, Var};
use crate::views::Context;

/// Perform a binary split on the first unassigned decision variable.
/// For small integer domains, enumerate individual values instead.
pub fn split_on_unassigned(space: Space) -> SplitOnUnassigned {
    if let Some(pivot) = space.vars.get_unassigned_var() {
        let var_domain = &space.vars[pivot];
        
        // For small integer domains, use value-by-value enumeration
        if let Var::VarI { min, max } = var_domain {
            let domain_size = max - min + 1;
            if domain_size <= 100 && domain_size > 0 {
                // Capture values before moving space
                let min_val = *min;
                let max_val = *max;
                // Use value enumeration for small integer domains
                return SplitOnUnassigned {
                    branch: Some(BranchState::ValueEnumeration {
                        space,
                        pivot,
                        current_value: min_val,
                        max_value: max_val,
                    }),
                };
            }
        }
        
        // Use binary split for large domains or float domains
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
    /// Binary split for large domains
    BinarySplit {
        space: Space,
        pivot: VarId,
        mid: Val,
        is_left: bool,
    },
    /// Value enumeration for small integer domains
    ValueEnumeration {
        space: Space,
        pivot: VarId,
        current_value: i32,
        max_value: i32,
    },
}

impl SplitOnUnassigned {
    /// Get the current propagation count from the space being explored.
    pub fn get_propagation_count(&self) -> usize {
        match &self.branch {
            Some(BranchState::BinarySplit { space, .. }) => space.get_propagation_count(),
            Some(BranchState::ValueEnumeration { space, .. }) => space.get_propagation_count(),
            None => 0,
        }
    }

    /// Get the current node count from the space being explored.
    pub fn get_node_count(&self) -> usize {
        match &self.branch {
            Some(BranchState::BinarySplit { space, .. }) => space.get_node_count(),
            Some(BranchState::ValueEnumeration { space, .. }) => space.get_node_count(),
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
                    // Split the provided space using a new propagator, to explore a specific branch.
                    let mut space_branch_left = space.clone();
                    
                    // Increment node counter when creating a new search node (left branch)
                    space_branch_left.props.increment_node_count();
                    
                    let p = space_branch_left.props.less_than_or_equals(pivot, mid);

                    self.branch = Some(BranchState::BinarySplit { 
                        space, 
                        pivot, 
                        mid, 
                        is_left: false 
                    });

                    Some((space_branch_left, p))
                } else {
                    let mut space_branch_right = space;
                    
                    // Increment node counter when creating a new search node (right branch)
                    space_branch_right.props.increment_node_count();
                    
                    let mut events = Vec::new();
                    let _ctx = Context::new(&mut space_branch_right.vars, &mut events);
                    let p = space_branch_right.props.greater_than(pivot, mid);
                    Some((space_branch_right, p))
                }
            }
            BranchState::ValueEnumeration { space, pivot, current_value, max_value } => {
                if current_value <= max_value {
                    // Try assigning pivot = current_value
                    let mut assign_space = space.clone();
                    assign_space.props.increment_node_count();
                    
                    let prop_id = assign_space.props.equals(pivot, Val::ValI(current_value));
                    
                    // Prepare for next value
                    if current_value < max_value {
                        self.branch = Some(BranchState::ValueEnumeration {
                            space,
                            pivot,
                            current_value: current_value + 1,
                            max_value,
                        });
                    }
                    
                    Some((assign_space, prop_id))
                } else {
                    // All values tried
                    None
                }
            }
        }
    }
}
