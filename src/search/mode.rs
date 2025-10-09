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
    
    /// Extract optimization objective for LP solver (if any)
    /// Returns (variable, minimize=true) or None if not optimizing
    fn lp_objective(&self) -> Option<(crate::variables::VarId, bool)> {
        None
    }
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
    
    fn lp_objective(&self) -> Option<(crate::variables::VarId, bool)> {
        // Check if V is VarId (minimize) or Opposite<VarId> (maximize)
        use std::any::TypeId;
        
        // Get the type ID of the objective
        let type_id = TypeId::of::<V>();
        
        // Check if it's VarId (minimize)
        if type_id == TypeId::of::<crate::variables::VarId>() {
            // SAFETY: We just checked the type
            let var_id = unsafe { *(&self.objective as *const V as *const crate::variables::VarId) };
            return Some((var_id, true)); // minimize=true
        }
        
        // Check if it's Opposite<VarId> (maximize)
        if type_id == TypeId::of::<crate::variables::views::Opposite<crate::variables::VarId>>() {
            // SAFETY: We just checked the type. Opposite is a tuple struct Opposite(VarId)
            // so we can cast the pointer and dereference to get the inner VarId
            let var_id = unsafe { *(&self.objective as *const V as *const crate::variables::VarId) };
            return Some((var_id, false)); // minimize=false (i.e., maximize)
        }
        
        None
    }
}
