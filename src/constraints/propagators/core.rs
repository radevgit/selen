//! Core propagator types and traits for constraint propagation system

// Framework-only imports commented out
// use std::ops::{Index, IndexMut};
use std::rc::Rc;

use crate::variables::VarId;
// Framework-only imports commented out
// use crate::variables::views::{Context, View, ViewExt};

// Type aliases for cleaner Rc-based sharing
pub type PropagatorBox = Box<dyn Prune>;
pub type SharedPropagator = Rc<PropagatorBox>;

/// Enforce a specific constraint by pruning domain of decision variables.
pub trait Prune: core::fmt::Debug {
    /// Perform pruning based on variable domains and internal state.
    fn prune(&self, ctx: &mut Context) -> Option<()>;
}

/// Isolate methods that prevent propagator from being used as a trait-object.
pub trait Propagate: Prune + 'static {
    /// List variables that schedule the propagator when their domain changes.
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId>;
}

/// Store internal state for each propagators, along with dependencies for when to schedule each.
#[doc(hidden)]
#[derive(Clone, Debug, Default)]
pub struct Propagators {
    pub state: Vec<SharedPropagator>,
    pub dependencies: Vec<Vec<PropId>>,
    /// Counter for the number of propagation steps performed
    pub propagation_steps: u64,
}

/// Unique ID for propagators, allowing them to be scheduled for propagation.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PropId(pub(crate) usize);

// Types are already public, no need for re-export
