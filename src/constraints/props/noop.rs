use crate::variables::views::Context;
use crate::variables::VarId;
use crate::constraints::props::{Prune};

/// A no-operation propagator that does nothing when invoked.
/// Used for branching operations that have already applied domain filtering directly.
#[derive(Clone, Debug)]
pub struct NoOp;

impl NoOp {
    /// Create a new no-operation propagator.
    pub fn new() -> Self {
        Self
    }
}

impl Prune for NoOp {
    fn prune(&self, _ctx: &mut Context) -> Option<()> {
        // Do nothing - domain filtering was already applied during branching
        Some(())
    }
}

impl crate::constraints::props::Propagate for NoOp {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        // No dependencies - this propagator never needs to be re-scheduled
        core::iter::empty()
    }
}
