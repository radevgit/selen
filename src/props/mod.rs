mod add;
mod eq;
mod leq;
mod sum;

use std::ops::{Index, IndexMut};
use dyn_clone::{clone_trait_object, DynClone};

use crate::{prelude::VAR_EPSILON, vars::{Val, VarId}, views::{Context, View, ViewExt, ViewType}};

/// Enforce a specific constraint by pruning domain of decision variables.
pub trait Prune: core::fmt::Debug + DynClone {
    /// Perform pruning based on variable domains and internal state.
    fn prune(&mut self, ctx: &mut Context) -> Option<()>;
}

/// Isolate methods that prevent propagator from being used as a trait-object.
pub trait Propagate: Prune + 'static {
    /// List variables that schedule the propagator when their domain changes.
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId>;
}

// ? State of propagators is cloned during search, but trait objects cannot be `Clone` by default
clone_trait_object!(Prune);

/// Store internal state for each propagators, along with dependencies for when to schedule each.
#[derive(Clone, Debug, Default)]
pub struct Propagators {
    state: Vec<Box<dyn Prune>>,
    dependencies: Vec<Vec<PropId>>,
    /// Counter for the number of propagation steps performed
    propagation_count: usize,
    /// Counter for the number of search nodes (branching points) explored
    node_count: usize,
}

impl Propagators {
    /// Extend dependencies matrix with a row for the new decision variable.
    pub fn on_new_var(&mut self) {
        self.dependencies.push(Vec::new());
    }

    /// List ids of all registered propagators.
    pub fn get_prop_ids_iter(&self) -> impl Iterator<Item = PropId> {
        (0..self.state.len()).map(PropId)
    }

    /// Acquire mutable reference to propagator state.
    pub fn get_state_mut(&mut self, p: PropId) -> &mut Box<dyn Prune> {
        &mut self.state[p]
    }

    /// Get list of propagators that should be scheduled when a bound of variable `v` changes.
    pub fn on_bound_change(&self, v: VarId) -> impl Iterator<Item = PropId> + '_ {
        self.dependencies[v].iter().copied()
    }

    /// Get the number of propagation steps performed so far.
    pub fn get_propagation_count(&self) -> usize {
        self.propagation_count
    }

    /// Increment the propagation step counter.
    pub fn increment_propagation_count(&mut self) {
        self.propagation_count += 1;
    }

    /// Get the number of search nodes explored so far.
    pub fn get_node_count(&self) -> usize {
        self.node_count
    }

    /// Increment the search node counter.
    pub fn increment_node_count(&mut self) {
        self.node_count += 1;
    }

    /// Declare a new propagator to enforce `x + y == s`.
    pub fn add(&mut self, x: impl View, y: impl View, s: VarId) -> PropId {
        self.push_new_prop(self::add::Add::new(x, y, s))
    }

    /// Declare a new propagator to enforce `sum(xs) == s`.
    pub fn sum(&mut self, xs: Vec<impl View>, s: VarId) -> PropId {
        self.push_new_prop(self::sum::Sum::new(xs, s))
    }

    /// Declare a new propagator to enforce `x == y`.
    pub fn equals(&mut self, x: impl View, y: impl View) -> PropId {
        self.push_new_prop(self::eq::Equals::new(x, y))
    }

    /// Declare a new propagator to enforce `x <= y`.
    pub fn less_than_or_equals(&mut self, x: impl View, y: impl View) -> PropId {
        self.push_new_prop(self::leq::LessThanOrEquals::new(x, y))
    }

    /// Declare a simple propagator to enforce `x < y` using fixed epsilon.
    /// For backward compatibility - uses floating-point epsilon for all constraints.
    pub fn less_than_simple(&mut self, x: impl View, y: impl View) -> PropId {
        // For strict inequality x < y, we need x <= y - delta
        // Use a small epsilon for floating-point to handle precision correctly
        // This will work for mixed integer/float constraints like v0 * 1.5 < 5.0
        self.less_than_or_equals(x, y.minus(Val::ValF(VAR_EPSILON)))
    }

    /// Declare a type-aware propagator to enforce `x < y`.
    /// This version uses ViewType analysis to determine the appropriate delta.
    pub fn less_than(&mut self, x: impl View, y: impl View, ctx: &Context) -> PropId {
        // Determine the appropriate delta based on the view types
        let x_type = x.result_type(ctx);
        let y_type = y.result_type(ctx);
        
        // Use floating-point epsilon if either side involves floats, 
        // otherwise use integer delta of 1
        let delta = match (x_type, y_type) {
            (ViewType::Float, _) | (_, ViewType::Float) => Val::ValF(VAR_EPSILON),
            (ViewType::Integer, ViewType::Integer) => Val::ValI(1),
        };
        
        // x < y  =>  x <= y - delta
        self.less_than_or_equals(x, y.minus(delta))
    }

    /// Declare a new propagator to enforce `x >= y`.
    pub fn greater_than_or_equals(&mut self, x: impl View, y: impl View) -> PropId {
        self.less_than_or_equals(y, x)
    }

    /// Declare a type-aware propagator to enforce `x > y`.
    /// This version uses ViewType analysis to determine the appropriate delta.
    pub fn greater_than(&mut self, x: impl View, y: impl View, ctx: &Context) -> PropId {
        // Determine the appropriate delta based on the view types
        let x_type = x.result_type(ctx);
        let y_type = y.result_type(ctx);
        
        // Use floating-point epsilon if either side involves floats, 
        // otherwise use integer delta of 1
        let delta = match (x_type, y_type) {
            (ViewType::Float, _) | (_, ViewType::Float) => Val::ValF(1e-6),
            (ViewType::Integer, ViewType::Integer) => Val::ValI(1),
        };
        
        // x > y  =>  x >= y + delta
        self.greater_than_or_equals(x, y.plus(delta))
    }

    /// Register propagator dependencies and store its state as a trait object.
    fn push_new_prop(&mut self, state: impl Propagate) -> PropId {
        // Create new handle to refer to propagator state and dependencies
        let p = PropId(self.state.len());

        // Register dependencies listed by trait implementor
        for v in state.list_trigger_vars() {
            self.dependencies[v].push(p);
        }

        // Store propagator state as trait object
        self.state.push(Box::new(state));

        p
    }
}

/// Propagator handle that is not bound to a specific memory location.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PropId(usize);

impl Index<PropId> for Vec<Box<dyn Prune>> {
    type Output = Box<dyn Prune>;

    fn index(&self, index: PropId) -> &Self::Output {
        &self[index.0]
    }
}

impl IndexMut<PropId> for Vec<Box<dyn Prune>> {
    fn index_mut(&mut self, index: PropId) -> &mut Self::Output {
        &mut self[index.0]
    }
}
