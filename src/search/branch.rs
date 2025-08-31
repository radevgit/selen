use crate::props::PropId;
use crate::search::Space;
use crate::vars::{VarId, Val};
use crate::views::Context;

/// Perform a binary split on the first unassigned decision variable.
pub fn split_on_unassigned(space: Space) -> SplitOnUnassigned {
    if let Some(pivot) = space.vars.get_unassigned_var() {
        // Split domain at mid-point of domain
        let mid = space.vars[pivot].mid();

        SplitOnUnassigned {
            branch: Some((space, pivot, mid, true)),
        }
    } else {
        SplitOnUnassigned { branch: None }
    }
}

/// Manual state machine until `gen` keyword is available (edition 2024).
pub struct SplitOnUnassigned {
    branch: Option<(Space, VarId, Val, bool)>,
}

impl Iterator for SplitOnUnassigned {
    type Item = (Space, PropId);

    fn next(&mut self) -> Option<Self::Item> {
        let (space, pivot, mid, is_left) = self.branch.take()?;

        if is_left {
            // Split the provided space using a new propagator, to explore a specific branch.
            let mut space_branch_left = space.clone();
            let p = space_branch_left.props.less_than_or_equals(pivot, mid);

            self.branch = Some((space, pivot, mid, false));

            Some((space_branch_left, p))
        } else {
            let mut space_branch_right = space;
            let mut events = Vec::new();
            let ctx = Context::new(&mut space_branch_right.vars, &mut events);
            let p = space_branch_right.props.greater_than(pivot, mid, &ctx);
            Some((space_branch_right, p))
        }
    }
}
