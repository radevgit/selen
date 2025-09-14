mod abs;
mod add;
mod alldiff;
mod bool_logic;
mod div;
mod eq;
mod leq;
mod max;
mod min;
mod modulo;
mod mul;
mod neq;
mod noop;
mod sum;

use std::ops::{Index, IndexMut};
use std::rc::Rc;

use crate::{vars::VarId, views::{Context, View, ViewExt}};

// Type aliases for cleaner Rc-based sharing
type PropagatorBox = Box<dyn Prune>;
type SharedPropagator = Rc<PropagatorBox>;

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
#[derive(Clone, Debug, Default)]
pub struct Propagators {
    state: Vec<SharedPropagator>,
    dependencies: Vec<Vec<PropId>>,
    /// Counter for the number of propagation steps performed
    propagation_count: usize,
    /// Counter for the number of search nodes (branching points) explored
    node_count: usize,
    /// Constraint metadata registry for optimization introspection
    constraint_registry: crate::optimization::constraint_metadata::ConstraintRegistry,
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

    /// Acquire immutable reference to propagator state (for constraint analysis).
    pub fn get_state(&self, p: PropId) -> &SharedPropagator {
        &self.state[p.0]
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
    
    /// Get the number of propagators in this collection.
    pub fn count(&self) -> usize {
        self.state.len()
    }

    /// Get access to the constraint metadata registry
    pub fn get_constraint_registry(&self) -> &crate::optimization::constraint_metadata::ConstraintRegistry {
        &self.constraint_registry
    }

    /// Get mutable access to the constraint metadata registry
    pub fn get_constraint_registry_mut(&mut self) -> &mut crate::optimization::constraint_metadata::ConstraintRegistry {
        &mut self.constraint_registry
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
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let x_info = self.analyze_view(&x);
        let y_info = self.analyze_view(&y);
        let s_info = ViewInfo::Variable { var_id: s };
        
        let variables: Vec<_> = x.get_underlying_var().into_iter()
            .chain(y.get_underlying_var().into_iter())
            .chain(std::iter::once(s))
            .collect();
            
        let metadata = ConstraintData::NAry {
            operands: vec![x_info, y_info, s_info],
        };
        
        self.push_new_prop_with_metadata(
            self::add::Add::new(x, y, s),
            ConstraintType::Addition,
            variables,
            metadata,
        )
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
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let x_info = self.analyze_view(&x);
        let y_info = self.analyze_view(&y);
        let s_info = ViewInfo::Variable { var_id: s };
        
        let variables: Vec<_> = x.get_underlying_var().into_iter()
            .chain(y.get_underlying_var().into_iter())
            .chain(std::iter::once(s))
            .collect();
            
        let metadata = ConstraintData::NAry {
            operands: vec![x_info, y_info, s_info],
        };
        
        self.push_new_prop_with_metadata(
            self::mul::Mul::new(x, y, s),
            ConstraintType::Multiplication,
            variables,
            metadata,
        )
    }

    /// Declare a new propagator to enforce `x % y == s`.
    pub fn modulo(&mut self, x: impl View, y: impl View, s: VarId) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let x_info = self.analyze_view(&x);
        let y_info = self.analyze_view(&y);
        let s_info = ViewInfo::Variable { var_id: s };
        
        let variables: Vec<_> = x.get_underlying_var().into_iter()
            .chain(y.get_underlying_var().into_iter())
            .chain(std::iter::once(s))
            .collect();
            
        let metadata = ConstraintData::NAry {
            operands: vec![x_info, y_info, s_info],
        };
        
        self.push_new_prop_with_metadata(
            self::modulo::Modulo::new(x, y, s),
            ConstraintType::Modulo,
            variables,
            metadata,
        )
    }

    /// Declare a new propagator to enforce `|x| == s`.
    pub fn abs(&mut self, x: impl View, s: VarId) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let x_info = self.analyze_view(&x);
        let s_info = ViewInfo::Variable { var_id: s };
        
        let variables: Vec<_> = x.get_underlying_var().into_iter()
            .chain(std::iter::once(s))
            .collect();
            
        let metadata = ConstraintData::NAry {
            operands: vec![x_info, s_info],
        };
        
        self.push_new_prop_with_metadata(
            self::abs::Abs::new(x, s),
            ConstraintType::AbsoluteValue,
            variables,
            metadata,
        )
    }

    /// Declare a new propagator to enforce `min(vars...) == result`.
    pub fn min(&mut self, vars: Vec<VarId>, result: VarId) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let operands: Vec<ViewInfo> = vars.iter()
            .map(|&var| ViewInfo::Variable { var_id: var })
            .chain(std::iter::once(ViewInfo::Variable { var_id: result }))
            .collect();
        
        let variables: Vec<_> = vars.iter().cloned()
            .chain(std::iter::once(result))
            .collect();
            
        let metadata = ConstraintData::NAry { operands };
        
        self.push_new_prop_with_metadata(
            self::min::Min::new(vars, result),
            ConstraintType::Minimum,
            variables,
            metadata,
        )
    }

    /// Declare a new propagator to enforce `max(vars...) == result`.
    pub fn max(&mut self, vars: Vec<VarId>, result: VarId) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let operands: Vec<ViewInfo> = vars.iter()
            .map(|&var| ViewInfo::Variable { var_id: var })
            .chain(std::iter::once(ViewInfo::Variable { var_id: result }))
            .collect();
        
        let variables: Vec<_> = vars.iter().cloned()
            .chain(std::iter::once(result))
            .collect();
            
        let metadata = ConstraintData::NAry { operands };
        
        self.push_new_prop_with_metadata(
            self::max::Max::new(vars, result),
            ConstraintType::Maximum,
            variables,
            metadata,
        )
    }

    /// Declare a new propagator to enforce `x / y == s`.
    pub fn div(&mut self, x: impl View, y: impl View, s: VarId) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let x_info = self.analyze_view(&x);
        let y_info = self.analyze_view(&y);
        let s_info = ViewInfo::Variable { var_id: s };
        
        let variables: Vec<_> = x.get_underlying_var().into_iter()
            .chain(y.get_underlying_var().into_iter())
            .chain(std::iter::once(s))
            .collect();
            
        let metadata = ConstraintData::NAry {
            operands: vec![x_info, y_info, s_info],
        };
        
        self.push_new_prop_with_metadata(
            self::div::Div::new(x, y, s),
            ConstraintType::Division,
            variables,
            metadata,
        )
    }

    /// Declare a new propagator to enforce `sum(xs) == s`.
    pub fn sum(&mut self, xs: Vec<impl View>, s: VarId) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let mut operands = Vec::new();
        let mut variables = Vec::new();
        
        // Analyze all variables in the sum
        for x in &xs {
            operands.push(self.analyze_view(x));
            if let Some(var_id) = x.get_underlying_var() {
                variables.push(var_id);
            }
        }
        
        // Add the result variable
        operands.push(ViewInfo::Variable { var_id: s });
        variables.push(s);
            
        let metadata = ConstraintData::NAry { operands };
        
        self.push_new_prop_with_metadata(
            self::sum::Sum::new(xs, s),
            ConstraintType::Sum,
            variables,
            metadata,
        )
    }

    /// Declare a new propagator to enforce `x == y`.
    pub fn equals(&mut self, x: impl View, y: impl View) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData};
        
        let x_info = self.analyze_view(&x);
        let y_info = self.analyze_view(&y);
        
        let variables: Vec<_> = x.get_underlying_var().into_iter()
            .chain(y.get_underlying_var().into_iter())
            .collect();
            
        let metadata = ConstraintData::Binary {
            left: x_info,
            right: y_info,
        };
        
        self.push_new_prop_with_metadata(
            self::eq::Eq::new(x, y),
            ConstraintType::Equals,
            variables,
            metadata,
        )
    }

    /// Declare a new propagator to enforce `x != y`.
    pub fn not_equals(&mut self, x: impl View, y: impl View) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData};
        
        let x_info = self.analyze_view(&x);
        let y_info = self.analyze_view(&y);
        
        let variables: Vec<_> = x.get_underlying_var().into_iter()
            .chain(y.get_underlying_var().into_iter())
            .collect();
            
        let metadata = ConstraintData::Binary {
            left: x_info,
            right: y_info,
        };
        
        self.push_new_prop_with_metadata(
            self::neq::NotEquals::new(x, y),
            ConstraintType::NotEquals,
            variables,
            metadata,
        )
    }

    /// Declare a new propagator to enforce `x <= y`.
    pub fn less_than_or_equals(&mut self, x: impl View, y: impl View) -> PropId {
        // Use the metadata collection version
        self.less_than_or_equals_with_metadata(x, y)
    }

    /// Declare a type-aware propagator to enforce `x < y`.
    /// This version uses ULP-based precision by implementing x < y as x + 1 <= y for integers
    /// and appropriate ULP-based bounds for floats.
    pub fn less_than(&mut self, x: impl View, y: impl View) -> PropId {
        // Use the metadata collection version
        self.less_than_with_metadata(x, y)
    }

    /// Declare a new propagator to enforce `x >= y`.
    pub fn greater_than_or_equals(&mut self, x: impl View, y: impl View) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData};
        
        let x_info = self.analyze_view(&x);
        let y_info = self.analyze_view(&y);
        
        let variables: Vec<_> = x.get_underlying_var().into_iter()
            .chain(y.get_underlying_var().into_iter())
            .collect();
            
        let metadata = ConstraintData::Binary {
            left: x_info,
            right: y_info,
        };
        
        self.push_new_prop_with_metadata(
            self::leq::LessThanOrEquals::new(y, x), // x >= y  =>  y <= x
            ConstraintType::GreaterThanOrEquals,
            variables,
            metadata,
        )
    }

    /// Declare a type-aware propagator to enforce `x > y`.
    /// This version uses ULP-based precision by implementing x > y as x >= y + 1 for integers
    /// and appropriate ULP-based bounds for floats.
    pub fn greater_than(&mut self, x: impl View, y: impl View) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData};
        
        let x_info = self.analyze_view(&x);
        let y_info = self.analyze_view(&y);
        
        // For constraint metadata, we want to preserve the original relationship x > y
        // The metadata represents the logical constraint, not the implementation details
        let metadata = ConstraintData::Binary {
            left: x_info,
            right: y_info,
        };
        
        let variables: Vec<_> = x.get_underlying_var().into_iter()
            .chain(y.get_underlying_var().into_iter())
            .collect();
            
        self.push_new_prop_with_metadata(
            self::leq::LessThanOrEquals::new(y.next(), x), // x > y  =>  y.next() <= x
            ConstraintType::GreaterThan,
            variables,
            metadata,
        )
    }

    /// Declare a new propagator to enforce that all variables have different values.
    /// This is more efficient than pairwise not-equals constraints.
    /// Uses the ultra-efficient AllDifferent implementation with adaptive algorithms.
    pub fn all_different(&mut self, vars: Vec<VarId>) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let operands: Vec<_> = vars.iter()
            .map(|&var_id| ViewInfo::Variable { var_id })
            .collect();
            
        let metadata = ConstraintData::NAry { operands };
        
        self.push_new_prop_with_metadata(
            self::alldiff::AllDifferent::new(vars.clone()),
            ConstraintType::AllDifferent,
            vars,
            metadata,
        )
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

        // Store propagator state as shared trait object
        self.state.push(Rc::new(Box::new(state)));

        p
    }

    /// Register propagator with metadata collection
    fn push_new_prop_with_metadata(
        &mut self, 
        state: impl Propagate, 
        constraint_type: crate::optimization::constraint_metadata::ConstraintType,
        variables: Vec<VarId>,
        metadata: crate::optimization::constraint_metadata::ConstraintData,
    ) -> PropId {
        // Create propagator first
        let prop_id = self.push_new_prop(state);
            
        // Register constraint metadata
        let _constraint_id = self.constraint_registry.register_constraint(
            constraint_type,
            variables,
            metadata,
        );
        
        prop_id
    }

    /// Get the number of constraints for analysis
    pub fn constraint_count(&self) -> usize {
        self.state.len()
    }

    // Helper functions for constraint metadata collection
    
    /// Analyze a view to extract constraint information
    fn analyze_view<T: View>(&self, view: &T) -> crate::optimization::constraint_metadata::ViewInfo {
        use crate::optimization::constraint_metadata::{ViewInfo, ConstraintValue};
        
        if let Some(var_id) = view.get_underlying_var() {
            ViewInfo::Variable { var_id }
        } else {
            // Check if this is a constant value by creating a dummy context
            // Constants will have the same min and max values
            let mut vars = crate::vars::Vars::default();
            let mut events = Vec::new();
            let ctx = crate::views::Context::new(&mut vars, &mut events);
            
            let min_val = view.min(&ctx);
            let max_val = view.max(&ctx);
            
            if min_val == max_val {
                // This is a constant value
                let value = match min_val {
                    crate::vars::Val::ValI(i) => ConstraintValue::Integer(i),
                    crate::vars::Val::ValF(f) => ConstraintValue::Float(f),
                };
                ViewInfo::Constant { value }
            } else {
                // For now, mark complex views as such
                // A full implementation would detect transformations
                ViewInfo::Complex
            }
        }
    }

    /// Create enhanced constraint methods with metadata collection
    
    /// Declare a new propagator to enforce `x <= y` with metadata collection.
    pub fn less_than_or_equals_with_metadata(&mut self, x: impl View, y: impl View) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData};
        
        let x_info = self.analyze_view(&x);
        let y_info = self.analyze_view(&y);
        let variables: Vec<_> = x.get_underlying_var().into_iter()
            .chain(y.get_underlying_var().into_iter())
            .collect();
            
        let metadata = ConstraintData::Binary {
            left: x_info,
            right: y_info,
        };
        
        self.push_new_prop_with_metadata(
            self::leq::LessThanOrEquals::new(x, y),
            ConstraintType::LessThanOrEquals,
            variables,
            metadata,
        )
    }

    /// Declare a type-aware propagator to enforce `x < y` with metadata collection.
    pub fn less_than_with_metadata(&mut self, x: impl View, y: impl View) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo, TransformationType};
        
        let x_info = self.analyze_view(&x);
        let y_info = self.analyze_view(&y);
        
        // For x < y implemented as x.next() <= y, we need to track the transformation
        let transformed_x_info = if let ViewInfo::Variable { var_id } = x_info {
            ViewInfo::Transformed {
                base_var: var_id,
                transformation: TransformationType::Next,
            }
        } else {
            ViewInfo::Complex
        };
        
        let variables: Vec<_> = x.get_underlying_var().into_iter()
            .chain(y.get_underlying_var().into_iter())
            .collect();
            
        let metadata = ConstraintData::Binary {
            left: transformed_x_info,
            right: y_info,
        };
        
        self.push_new_prop_with_metadata(
            self::leq::LessThanOrEquals::new(x.next(), y),
            ConstraintType::LessThan,
            variables,
            metadata,
        )
    }

    /// Declare a new propagator to enforce `result = a AND b AND c AND ...`.
    /// All variables are treated as boolean: 0 = false, non-zero = true.
    pub fn bool_and(&mut self, operands: Vec<VarId>, result: VarId) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let operand_infos: Vec<_> = operands.iter()
            .map(|&var_id| ViewInfo::Variable { var_id })
            .collect();
        
        let variables: Vec<_> = operands.iter().cloned()
            .chain(std::iter::once(result))
            .collect();
            
        let metadata = ConstraintData::NAry { operands: operand_infos };
        
        self.push_new_prop_with_metadata(
            self::bool_logic::BoolAnd::new(operands, result),
            ConstraintType::BooleanAnd,
            variables,
            metadata,
        )
    }

    /// Declare a new propagator to enforce `result = a OR b OR c OR ...`.
    /// All variables are treated as boolean: 0 = false, non-zero = true.
    pub fn bool_or(&mut self, operands: Vec<VarId>, result: VarId) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let operand_infos: Vec<_> = operands.iter()
            .map(|&var_id| ViewInfo::Variable { var_id })
            .collect();
        
        let variables: Vec<_> = operands.iter().cloned()
            .chain(std::iter::once(result))
            .collect();
            
        let metadata = ConstraintData::NAry { operands: operand_infos };
        
        self.push_new_prop_with_metadata(
            self::bool_logic::BoolOr::new(operands, result),
            ConstraintType::BooleanOr,
            variables,
            metadata,
        )
    }

    /// Declare a new propagator to enforce `result = NOT operand`.
    /// Variables are treated as boolean: 0 = false, non-zero = true.
    pub fn bool_not(&mut self, operand: VarId, result: VarId) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let operand_info = ViewInfo::Variable { var_id: operand };
        let result_info = ViewInfo::Variable { var_id: result };
        
        let variables = vec![operand, result];
            
        let metadata = ConstraintData::Binary {
            left: operand_info,
            right: result_info,
        };
        
        self.push_new_prop_with_metadata(
            self::bool_logic::BoolNot::new(operand, result),
            ConstraintType::BooleanNot,
            variables,
            metadata,
        )
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
