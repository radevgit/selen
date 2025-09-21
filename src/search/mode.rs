use crate::constraints::props::PropId;
use crate::search::Space;
use crate::variables::{Vars, Val};
use crate::variables::views::View;

/// Control search behavior when a solution is found.
pub trait Mode: core::fmt::Debug {
    /// List propagators to be scheduled on after branch.
    fn on_branch(&self, _: &mut Space) -> impl Iterator<Item = PropId> {
        core::iter::empty()
    }

    /// Update internal state when new solution is found.
    fn on_solution(&mut self, _vars: &Vars) {}
}

/// Enumerate assignments that satisfy all constraints.
#[derive(Debug)]
pub struct Enumerate;

impl Mode for Enumerate {}

/// Enumerate assignments that satisfy all constraints, and gradually lower objective expression.
#[derive(Debug)]
pub struct Minimize<V> {
    objective: V,
    minimum_opt: Option<Val>,
}

impl<V: View> Minimize<V> {
    pub const fn new(objective: V) -> Self {
        Self {
            objective,
            minimum_opt: None,
        }
    }
}

impl<V: View> Mode for Minimize<V> {
    fn on_branch(&self, space: &mut Space) -> impl Iterator<Item = PropId> {
        // Prune assignments that cannot lower objective expression
        if let Some(minimum) = self.minimum_opt {
            // let mut events = Vec::new();
            // let ctx = Context::new(&mut space.vars, &mut events);
            let prop_id = space.props.less_than(self.objective, minimum);
            Some(prop_id).into_iter()
        } else {
            None.into_iter()
        }
    }

    fn on_solution(&mut self, vars: &Vars) {
        // New objective value is necessarily lower than previous lowest
        self.minimum_opt = Some(self.objective.min_raw(vars));
    }
}
