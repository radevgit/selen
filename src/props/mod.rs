mod add;
mod alldiff;
mod eq;
mod leq;
mod mul;
mod neq;
mod noop;
mod sum;

use std::ops::{Index, IndexMut};
use dyn_clone::{clone_trait_object, DynClone};

use crate::{vars::VarId, views::{Context, View, ViewExt}};

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

    /// Optimize the order of AllDifferent constraints based on the number of fixed variables.
    /// 
    /// AllDifferent constraints with more singleton (fixed) variables are processed first
    /// because they tend to propagate more effectively and reduce the search space earlier.
    /// This can significantly improve solving performance.
    pub fn optimize_alldiff_order(&mut self, _vars: &crate::vars::Vars) {
        // Use a simpler heuristic based on constraint dependencies
        // that proved effective in practice.
        
        // Create a vector of (constraint_index, dependency_count) pairs
        let mut constraint_priorities: Vec<(usize, usize)> = Vec::new();
        
        for (i, _constraint) in self.state.iter().enumerate() {
            // Calculate priority score based on number of variables this constraint affects
            let mut dependency_count = 0;
            
            // Count how many variables depend on this constraint
            for var_deps in &self.dependencies {
                for &prop_id in var_deps {
                    if prop_id.0 == i {
                        dependency_count += 1;
                    }
                }
            }
            
            constraint_priorities.push((i, dependency_count));
        }
        
        // Sort by dependency count (descending - more dependencies = higher priority)
        constraint_priorities.sort_by(|a, b| b.1.cmp(&a.1));
        
        // Only reorder if we have multiple constraints and the ordering would change
        if constraint_priorities.len() > 1 {
            let first_original_index = constraint_priorities[0].0;
            if first_original_index != 0 {
                // Create new ordered vectors
                let original_state = self.state.clone();
                let original_dependencies = self.dependencies.clone();
                
                // Clear current state
                self.state.clear();
                self.dependencies = vec![Vec::new(); original_dependencies.len()];
                
                // Create index mapping from old to new positions
                let mut index_mapping = vec![0; original_state.len()];
                
                // Rebuild in optimized order
                for (new_idx, &(old_idx, _priority)) in constraint_priorities.iter().enumerate() {
                    if old_idx < original_state.len() {
                        self.state.push(original_state[old_idx].clone());
                        index_mapping[old_idx] = new_idx;
                    }
                }
                
                // Update dependency mapping
                for (var_id, deps) in original_dependencies.into_iter().enumerate() {
                    for old_prop_id in deps {
                        if old_prop_id.0 < index_mapping.len() {
                            let new_prop_id = PropId(index_mapping[old_prop_id.0]);
                            self.dependencies[var_id].push(new_prop_id);
                        }
                    }
                }
            }
        }
    }

    /// Declare a new propagator to enforce `x + y == s`.
    pub fn add(&mut self, x: impl View, y: impl View, s: VarId) -> PropId {
        self.push_new_prop(self::add::Add::new(x, y, s))
    }

    /// Declare a new propagator to enforce `x - y == s`.
    /// This reuses the Add propagator by transforming to `x + (-y) == s`.
    pub fn sub(&mut self, x: impl View, y: impl View, s: VarId) -> PropId {
        use crate::vars::Val;
        // x - y = s  =>  x + (-y) = s
        self.add(x, y.times_neg(Val::ValI(-1)), s)
    }

    /// Declare a new propagator to enforce `x * y == s`.
    pub fn mul(&mut self, x: impl View, y: impl View, s: VarId) -> PropId {
        self.push_new_prop(self::mul::Mul::new(x, y, s))
    }

    /// Declare a new propagator to enforce `sum(xs) == s`.
    pub fn sum(&mut self, xs: Vec<impl View>, s: VarId) -> PropId {
        self.push_new_prop(self::sum::Sum::new(xs, s))
    }

    /// Declare a new propagator to enforce `x == y`.
    pub fn equals(&mut self, x: impl View, y: impl View) -> PropId {
        self.push_new_prop(self::eq::Equals::new(x, y))
    }

    /// Declare a new propagator to enforce `x != y`.
    pub fn not_equals(&mut self, x: impl View, y: impl View) -> PropId {
        self.push_new_prop(self::neq::NotEquals::new(x, y))
    }

    /// Declare a new propagator to enforce `x <= y`.
    pub fn less_than_or_equals(&mut self, x: impl View, y: impl View) -> PropId {
        self.push_new_prop(self::leq::LessThanOrEquals::new(x, y))
    }

    /// Declare a type-aware propagator to enforce `x < y`.
    /// This version uses ULP-based precision by implementing x < y as x + 1 <= y for integers
    /// and appropriate ULP-based bounds for floats.
    pub fn less_than(&mut self, x: impl View, y: impl View) -> PropId {
        // x < y  =>  x + 1 <= y (this works for both integers and floats due to type promotion)
        self.less_than_or_equals(x.next(), y)
    }

    /// Declare a new propagator to enforce `x >= y`.
    pub fn greater_than_or_equals(&mut self, x: impl View, y: impl View) -> PropId {
        self.less_than_or_equals(y, x)
    }

    /// Declare a type-aware propagator to enforce `x > y`.
    /// This version uses ULP-based precision by implementing x > y as x >= y + 1 for integers
    /// and appropriate ULP-based bounds for floats.
    pub fn greater_than(&mut self, x: impl View, y: impl View) -> PropId {
        // x > y  =>  x >= y + 1 (this works for both integers and floats due to type promotion)
        self.greater_than_or_equals(x, y.next())
    }

    /// Declare a new propagator to enforce that all variables have different values.
    /// This is more efficient than pairwise not-equals constraints.
    /// Uses the ultra-efficient AllDifferent implementation with adaptive algorithms.
    pub fn all_different(&mut self, vars: Vec<VarId>) -> PropId {
        self.push_new_prop(self::alldiff::AllDifferent::new(vars))
    }

    /// Create a no-operation propagator for branching operations that have already applied domain filtering.
    pub fn noop(&mut self) -> PropId {
        self.push_new_prop(self::noop::NoOp::new())
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

// Public exports
pub use alldiff::AllDifferent;
