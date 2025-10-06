mod abs;
mod add;
mod alldiff;
mod allequal;
pub mod between;
mod bool_logic;
pub mod cardinality;
pub mod conditional;
mod count;
mod div;
mod element;
mod eq;
mod leq;
mod linear;
mod max;
mod min;
mod modulo;
mod mul;
mod neq;
mod noop;
mod reification;
mod sum;
mod table;

use std::ops::{Index, IndexMut};
use std::rc::Rc;

use crate::{variables::VarId, variables::views::{Context, View, ViewExt}};

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
#[doc(hidden)]
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

#[doc(hidden)]
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

    /// Optimize the order of AllDifferent constraints using multiple universal heuristics.
    /// 
    /// Uses a combination of three heuristics to determine priority:
    /// 1. Domain tightness: Constraints with smaller average domain sizes (closer to failure)
    /// 2. Variable connectivity: Constraints affecting variables with higher connectivity
    /// 3. Constraint saturation: Constraints with higher ratio of fixed variables
    /// 
    /// This approach is universal and works for any constraint satisfaction problem.
    pub fn optimize_alldiff_order(&mut self, vars: &crate::variables::Vars) {
        use crate::optimization::constraint_metadata::ConstraintType;
        
        // Get all AllDifferent constraints from the registry
        let alldiff_constraint_ids = self.constraint_registry.get_constraints_by_type(&ConstraintType::AllDifferent);
        
        if alldiff_constraint_ids.len() <= 1 {
            return; // Nothing to optimize with 0 or 1 AllDifferent constraints
        }
        
        // Pre-calculate variable connectivity map
        let variable_connectivity = self.calculate_variable_connectivity_map();
        
        // Create a vector of (prop_id, priority_scores) for AllDifferent constraints only
        let mut alldiff_priorities: Vec<(usize, (f64, f64, f64))> = Vec::new();
        
        for constraint_id in alldiff_constraint_ids {
            if let Some(metadata) = self.constraint_registry.get_constraint(constraint_id) {
                // Extract variables from this AllDifferent constraint
                let constraint_vars = match &metadata.data {
                    crate::optimization::constraint_metadata::ConstraintData::NAry { operands } => {
                        operands.iter().filter_map(|op| match op {
                            crate::optimization::constraint_metadata::ViewInfo::Variable { var_id } => Some(*var_id),
                            _ => None,
                        }).collect::<Vec<_>>()
                    },
                    _ => continue, // Skip non-NAry constraints
                };
                
                // Calculate domain tightness score (smaller average domain = higher priority)
                let tightness_score = self.calculate_domain_tightness(&constraint_vars, vars);
                
                // Calculate variable connectivity score (higher connectivity = higher priority)
                let connectivity_score = self.calculate_variable_connectivity(&constraint_vars, &variable_connectivity);
                
                // Calculate constraint saturation score (higher fixed ratio = higher priority)
                let saturation_score = self.calculate_constraint_saturation(&constraint_vars, vars);
                
                // Map constraint_id to propagator index
                let prop_index = constraint_id.0;
                if prop_index < self.state.len() {
                    alldiff_priorities.push((prop_index, (tightness_score, connectivity_score, saturation_score)));
                }
            }
        }
        
        if alldiff_priorities.is_empty() {
            return; // No AllDifferent constraints found
        }
        
        // Sort AllDifferent constraints by priority (lexicographic ordering):
        // Primary: domain tightness (ascending - smaller domains = higher priority)
        // Secondary: constraint saturation (descending - higher fixed ratio = higher priority)
        // Tertiary: variable connectivity (descending - higher connectivity = higher priority)
        alldiff_priorities.sort_by(|a, b| {
            // First compare by domain tightness (ascending)
            let tightness_cmp = a.1.0.partial_cmp(&b.1.0).unwrap_or(std::cmp::Ordering::Equal);
            if tightness_cmp != std::cmp::Ordering::Equal {
                return tightness_cmp;
            }
            // Then by constraint saturation (descending)
            let saturation_cmp = b.1.2.partial_cmp(&a.1.2).unwrap_or(std::cmp::Ordering::Equal);
            if saturation_cmp != std::cmp::Ordering::Equal {
                return saturation_cmp;
            }
            // Finally by variable connectivity (descending)
            b.1.1.partial_cmp(&a.1.1).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        // Check if reordering would actually change anything
        let mut needs_reordering = false;
        for (i, &(prop_idx, _)) in alldiff_priorities.iter().enumerate() {
            if i < alldiff_priorities.len() - 1 {
                let next_prop_idx = alldiff_priorities[i + 1].0;
                if prop_idx > next_prop_idx {
                    needs_reordering = true;
                    break;
                }
            }
        }
        
        if !needs_reordering {
            return; // Already in optimal order
        }
        
        // Create new ordered vectors for reordering
        let original_state = self.state.clone();
        let original_dependencies = self.dependencies.clone();
        
        // Extract AllDifferent constraint indices for reordering
        let alldiff_indices: std::collections::HashSet<usize> = alldiff_priorities.iter().map(|&(idx, _)| idx).collect();
        
        // Build the new state with AllDifferent constraints in optimized order
        self.state.clear();
        let mut index_mapping = vec![0; original_state.len()];
        let mut new_idx = 0;
        
        // First, add AllDifferent constraints in priority order
        for &(old_idx, _) in &alldiff_priorities {
            if old_idx < original_state.len() {
                self.state.push(original_state[old_idx].clone());
                index_mapping[old_idx] = new_idx;
                new_idx += 1;
            }
        }
        
        // Then, add all non-AllDifferent constraints in their original order
        for (old_idx, constraint) in original_state.iter().enumerate() {
            if !alldiff_indices.contains(&old_idx) {
                self.state.push(constraint.clone());
                index_mapping[old_idx] = new_idx;
                new_idx += 1;
            }
        }
        
        // Update dependency mapping with new indices
        self.dependencies = vec![Vec::new(); original_dependencies.len()];
        for (var_id, deps) in original_dependencies.into_iter().enumerate() {
            for old_prop_id in deps {
                if old_prop_id.0 < index_mapping.len() {
                    let new_prop_id = PropId(index_mapping[old_prop_id.0]);
                    self.dependencies[var_id].push(new_prop_id);
                }
            }
        }
    }

    /// Universal constraint optimization that works for all constraint types.
    /// 
    /// This function analyzes ALL constraint types (not just AllDifferent) and prioritizes
    /// them based on constraint-specific optimization strategies:
    /// - AllDifferent: Prioritizes domain tightness + connectivity + saturation
    /// - Arithmetic (=, ≤, +, *): Prioritizes saturation (more fixed variables = easier propagation)
    /// - Boolean (AND, OR, NOT): Prioritizes connectivity (affecting more variables = higher impact)
    /// - Complex constraints: Use hybrid scoring based on constraint characteristics
    pub fn optimize_universal_constraint_order(&mut self, vars: &crate::variables::Vars) {
        use crate::optimization::constraint_metadata::ConstraintType;
        
        // Get all constraint types we want to optimize
        let constraint_types_to_optimize = [
            ConstraintType::AllDifferent,
            ConstraintType::Addition,
            ConstraintType::Multiplication,
            ConstraintType::Modulo,
            ConstraintType::Division,
            ConstraintType::Sum,
            ConstraintType::Equals,
            ConstraintType::NotEquals,
            ConstraintType::LessThanOrEquals,
            ConstraintType::LessThan,
            ConstraintType::GreaterThanOrEquals,
            ConstraintType::GreaterThan,
            ConstraintType::BooleanAnd,
            ConstraintType::BooleanOr,
            ConstraintType::BooleanNot,
            ConstraintType::Count,
            // Reification constraints
            ConstraintType::EqualityReified,
            ConstraintType::InequalityReified,
            ConstraintType::LessThanReified,
            ConstraintType::LessEqualReified,
            ConstraintType::GreaterThanReified,
            ConstraintType::GreaterEqualReified,
        ];
        
        // Pre-calculate variable connectivity map for all constraint types
        let variable_connectivity = self.calculate_variable_connectivity_map();
        
        // Create a vector of (prop_id, priority_score, constraint_type) for all constraints
        let mut constraint_priorities: Vec<(usize, f64, ConstraintType)> = Vec::new();
        
        for constraint_type in &constraint_types_to_optimize {
            for constraint_id in self.constraint_registry.get_constraints_by_type(constraint_type) {
                if let Some(metadata) = self.constraint_registry.get_constraint(constraint_id) {
                    // Extract variables from this constraint
                    let constraint_vars = &metadata.variables;
                    
                    if constraint_vars.is_empty() {
                        continue; // Skip constraints with no variables
                    }
                    
                    // Calculate constraint-specific priority score
                    let priority_score = self.calculate_universal_constraint_priority(
                        constraint_type, 
                        constraint_vars, 
                        vars, 
                        &variable_connectivity
                    );
                    
                    // Map constraint_id to propagator index
                    let prop_index = constraint_id.0;
                    if prop_index < self.state.len() {
                        constraint_priorities.push((prop_index, priority_score, constraint_type.clone()));
                    }
                }
            }
        }
        
        if constraint_priorities.len() <= 1 {
            return; // Nothing to optimize
        }
        
        // Sort constraints by priority score (descending - higher scores = higher priority)
        constraint_priorities.sort_by(|a, b| {
            b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        // Check if reordering would actually change anything
        let mut needs_reordering = false;
        for (i, &(prop_idx, _, _)) in constraint_priorities.iter().enumerate() {
            if i < constraint_priorities.len() - 1 {
                let next_prop_idx = constraint_priorities[i + 1].0;
                if prop_idx > next_prop_idx {
                    needs_reordering = true;
                    break;
                }
            }
        }
        
        if !needs_reordering {
            return; // Already in optimal order
        }
        
        // Create new ordered vectors for reordering
        let original_state = self.state.clone();
        let original_dependencies = self.dependencies.clone();
        
        // Create index mapping: old_index -> new_index
        let mut index_mapping = vec![0; original_state.len()];
        for (new_index, &(old_index, _, _)) in constraint_priorities.iter().enumerate() {
            index_mapping[old_index] = new_index;
        }
        
        // Reorder propagators based on priority
        self.state.clear();
        for &(prop_idx, _, _) in &constraint_priorities {
            self.state.push(original_state[prop_idx].clone());
        }
        
        // Reorder dependencies to maintain consistency
        self.dependencies = vec![Vec::new(); original_dependencies.len()];
        for (var_id, deps) in original_dependencies.into_iter().enumerate() {
            for old_prop_id in deps {
                if old_prop_id.0 < index_mapping.len() {
                    let new_prop_id = PropId(index_mapping[old_prop_id.0]);
                    self.dependencies[var_id].push(new_prop_id);
                }
            }
        }
    }

    /// Calculate universal constraint priority based on constraint type and characteristics.
    /// Higher scores indicate higher priority for propagation.
    fn calculate_universal_constraint_priority(
        &self,
        constraint_type: &crate::optimization::constraint_metadata::ConstraintType,
        constraint_vars: &[VarId],
        vars: &crate::variables::Vars,
        variable_connectivity: &std::collections::HashMap<VarId, usize>
    ) -> f64 {
        use crate::optimization::constraint_metadata::ConstraintType;
        
        match constraint_type {
            // AllDifferent constraints: Use comprehensive scoring (domain tightness + connectivity + saturation)
            ConstraintType::AllDifferent => {
                let tightness_score = 1.0 / self.calculate_domain_tightness(constraint_vars, vars).max(1.0);
                let connectivity_score = self.calculate_variable_connectivity(constraint_vars, variable_connectivity);
                let saturation_score = self.calculate_constraint_saturation(constraint_vars, vars);
                
                // Weight: tightness (40%) + saturation (35%) + connectivity (25%)
                0.4 * tightness_score + 0.35 * saturation_score + 0.25 * connectivity_score
            },
            
            // Arithmetic constraints: Prioritize saturation (fixed variables make propagation easier)
            ConstraintType::Addition | ConstraintType::Multiplication | 
            ConstraintType::Modulo | ConstraintType::Division | 
            ConstraintType::Sum => {
                let saturation_score = self.calculate_constraint_saturation(constraint_vars, vars);
                let connectivity_score = self.calculate_variable_connectivity(constraint_vars, variable_connectivity);
                
                // Weight: saturation (70%) + connectivity (30%)
                0.7 * saturation_score + 0.3 * connectivity_score
            },
            
            // Comparison constraints: Balance saturation and connectivity
            ConstraintType::Equals | ConstraintType::NotEquals |
            ConstraintType::LessThanOrEquals | ConstraintType::LessThan |
            ConstraintType::GreaterThanOrEquals | ConstraintType::GreaterThan => {
                let saturation_score = self.calculate_constraint_saturation(constraint_vars, vars);
                let connectivity_score = self.calculate_variable_connectivity(constraint_vars, variable_connectivity);
                
                // Weight: saturation (60%) + connectivity (40%)
                0.6 * saturation_score + 0.4 * connectivity_score
            },
            
            // Boolean constraints: Prioritize connectivity (affecting many variables = high impact)
            ConstraintType::BooleanAnd | ConstraintType::BooleanOr | ConstraintType::BooleanNot => {
                let connectivity_score = self.calculate_variable_connectivity(constraint_vars, variable_connectivity);
                let saturation_score = self.calculate_constraint_saturation(constraint_vars, vars);
                
                // Weight: connectivity (60%) + saturation (40%)
                0.6 * connectivity_score + 0.4 * saturation_score
            },
            
            // Complex constraints: Use hybrid scoring
            ConstraintType::Complex { .. } => {
                let tightness_score = 1.0 / self.calculate_domain_tightness(constraint_vars, vars).max(1.0);
                let connectivity_score = self.calculate_variable_connectivity(constraint_vars, variable_connectivity);
                let saturation_score = self.calculate_constraint_saturation(constraint_vars, vars);
                
                // Balanced weight: all three factors equally
                (tightness_score + connectivity_score + saturation_score) / 3.0
            },
            
            // Default case: Use balanced scoring
            _ => {
                let connectivity_score = self.calculate_variable_connectivity(constraint_vars, variable_connectivity);
                let saturation_score = self.calculate_constraint_saturation(constraint_vars, vars);
                
                // Weight: connectivity (50%) + saturation (50%)
                0.5 * connectivity_score + 0.5 * saturation_score
            }
        }
    }

    /// Calculate domain tightness score for a constraint.
    /// Lower scores indicate tighter constraints (smaller domains) which should be prioritized.
    fn calculate_domain_tightness(&self, constraint_vars: &[VarId], vars: &crate::variables::Vars) -> f64 {
        if constraint_vars.is_empty() {
            return f64::INFINITY; // Invalid constraint, lowest priority
        }
        
        let total_domain_size: f64 = constraint_vars.iter()
            .map(|&var_id| self.calculate_variable_domain_size(var_id, vars))
            .sum();
        
        // Return average domain size (smaller = tighter = higher priority)
        total_domain_size / (constraint_vars.len() as f64)
    }
    
    /// Calculate the effective domain size for a variable.
    /// For integer variables, this is the actual domain size.
    /// For float variables, this estimates discrete steps within the interval.
    fn calculate_variable_domain_size(&self, var_id: VarId, vars: &crate::variables::Vars) -> f64 {
        match &vars[var_id] {
            crate::variables::Var::VarI(sparse_set) => {
                sparse_set.size() as f64
            },
            crate::variables::Var::VarF(interval) => {
                // For float variables, estimate the number of discrete steps
                let range = interval.max - interval.min;
                if range <= 0.0 {
                    return 1.0; // Single value
                }
                
                // Estimate discrete domain size based on step size
                let estimated_steps = (range / interval.step).ceil();
                estimated_steps.max(1.0) // At least 1 step
            }
        }
    }

    /// Calculate variable connectivity map - how many constraints each variable participates in.
    /// This builds a connectivity graph to understand variable importance in the constraint network.
    fn calculate_variable_connectivity_map(&self) -> std::collections::HashMap<VarId, usize> {
        let mut connectivity_map = std::collections::HashMap::new();
        
        // Since we don't have direct access to all constraints, we'll iterate through
        // all constraint types and collect their constraint IDs
        use crate::optimization::constraint_metadata::ConstraintType;
        
        let constraint_types = [
            ConstraintType::AllDifferent,
            ConstraintType::Addition,
            ConstraintType::Multiplication,
            ConstraintType::Modulo,
            ConstraintType::Division,
            ConstraintType::AbsoluteValue,
            ConstraintType::Minimum,
            ConstraintType::Maximum,
            ConstraintType::Sum,
            ConstraintType::Equals,
            ConstraintType::NotEquals,
            ConstraintType::LessThanOrEquals,
            ConstraintType::LessThan,
            ConstraintType::GreaterThanOrEquals,
            ConstraintType::GreaterThan,
            ConstraintType::BooleanAnd,
            ConstraintType::BooleanOr,
            ConstraintType::BooleanNot,
        ];
        
        // Iterate through all constraint types to count variable participation
        for constraint_type in &constraint_types {
            for constraint_id in self.constraint_registry.get_constraints_by_type(constraint_type) {
                if let Some(metadata) = self.constraint_registry.get_constraint(constraint_id) {
                    // Extract variables from this constraint and increment their connectivity
                    for var_id in &metadata.variables {
                        *connectivity_map.entry(*var_id).or_insert(0) += 1;
                    }
                }
            }
        }
        
        connectivity_map
    }

    /// Calculate average variable connectivity for a constraint.
    /// Higher connectivity indicates variables that are more constrained and likely to propagate.
    fn calculate_variable_connectivity(
        &self, 
        constraint_vars: &[VarId], 
        connectivity_map: &std::collections::HashMap<VarId, usize>
    ) -> f64 {
        if constraint_vars.is_empty() {
            return 0.0; // No variables, lowest connectivity
        }
        
        let total_connectivity: usize = constraint_vars.iter()
            .map(|var_id| connectivity_map.get(var_id).copied().unwrap_or(0))
            .sum();
        
        // Return average connectivity (higher = more constrained = higher priority)
        total_connectivity as f64 / constraint_vars.len() as f64
    }

    /// Calculate constraint saturation score - ratio of fixed variables to total variables.
    /// Higher saturation indicates constraints closer to completion and likely to propagate effectively.
    fn calculate_constraint_saturation(&self, constraint_vars: &[VarId], vars: &crate::variables::Vars) -> f64 {
        if constraint_vars.is_empty() {
            return 0.0; // No variables, lowest saturation
        }
        
        let fixed_variables = constraint_vars.iter()
            .filter(|&&var_id| {
                match &vars[var_id] {
                    crate::variables::Var::VarI(sparse_set) => sparse_set.size() == 1,
                    crate::variables::Var::VarF(interval) => {
                        // For float variables, consider fixed if the range is very small
                        let range = interval.max - interval.min;
                        range <= interval.step
                    }
                }
            })
            .count();
        
        // Return saturation ratio (higher = more fixed = higher priority)
        fixed_variables as f64 / constraint_vars.len() as f64
    }

    /// Declare a new propagator to enforce `x + y == s`.
    pub fn add(&mut self, x: impl View, y: impl View, s: VarId) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let x_info = self.analyze_view(&x);
        let y_info = self.analyze_view(&y);
        let s_info = ViewInfo::Variable { var_id: s };
        
        let variables: Vec<_> = x.get_underlying_var().into_iter()
            .chain(y.get_underlying_var())
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
        use crate::variables::Val;
        // x - y = s  =>  x + (-y) = s
        self.add(x, y.times_neg(Val::ValI(-1)), s)
    }

    /// Declare a new propagator to enforce `x * y == s`.
    pub fn mul(&mut self, x: impl View, y: impl View, s: VarId) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo, ConstraintValue};
        
        // Special handling for TypedConstant detection
        fn analyze_view_for_constants<T: View>(view: &T) -> ViewInfo {
            if let Some(var_id) = view.get_underlying_var() {
                ViewInfo::Variable { var_id }
            } else {
                // This view has no underlying variable - it might be a constant
                // For TypedConstant, we can detect this by checking min == max
                // Since both are the constant value
                let min_val = view.min_raw(&crate::variables::Vars::default());
                let max_val = view.max_raw(&crate::variables::Vars::default());
                
                if min_val == max_val {
                    // This is likely a constant value
                    let const_val = match min_val {
                        crate::variables::Val::ValI(i) => ConstraintValue::Integer(i),
                        crate::variables::Val::ValF(f) => ConstraintValue::Float(f),
                    };
                    ViewInfo::Constant { value: const_val }
                } else {
                    ViewInfo::Complex
                }
            }
        }
        
        let x_info = analyze_view_for_constants(&x);
        let y_info = analyze_view_for_constants(&y);
        let s_info = ViewInfo::Variable { var_id: s };
        
        let variables: Vec<_> = x.get_underlying_var().into_iter()
            .chain(y.get_underlying_var())
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
            .chain(y.get_underlying_var())
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
            .chain(y.get_underlying_var())
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
            .chain(y.get_underlying_var())
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
            .chain(y.get_underlying_var())
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
            .chain(y.get_underlying_var())
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
            .chain(y.get_underlying_var())
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
            self::alldiff::AllDiff::new(vars.clone()),
            ConstraintType::AllDifferent,
            vars,
            metadata,
        )
    }

    /// Declare a new propagator to enforce that all variables have the same value.
    /// This is the complement of AllDifferent constraint.
    /// Uses efficient domain intersection for value propagation.
    pub fn all_equal(&mut self, vars: Vec<VarId>) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let operands: Vec<_> = vars.iter()
            .map(|&var_id| ViewInfo::Variable { var_id })
            .collect();
            
        let metadata = ConstraintData::NAry { operands };
        
        self.push_new_prop_with_metadata(
            self::allequal::AllEqual::new(vars.clone()),
            ConstraintType::AllEqual,
            vars,
            metadata,
        )
    }

    /// Declare a new propagator to enforce that exactly count_var variables in vars equal target_value.
    /// This is the count constraint: count(vars, target_value, count_var).
    pub fn count_constraint(&mut self, vars: Vec<VarId>, target_value: crate::variables::Val, count_var: VarId) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo, ConstraintValue};
        use crate::variables::Val;
        
        let count_instance = count::Count::new(vars.clone(), target_value, count_var);
        
        let mut operands: Vec<ViewInfo> = vars.iter()
            .map(|&var_id| ViewInfo::Variable { var_id })
            .collect();
        operands.push(ViewInfo::Variable { var_id: count_var });
        
        let value = match target_value {
            Val::ValI(i) => ConstraintValue::Integer(i),
            Val::ValF(f) => ConstraintValue::Float(f),
        };
        operands.push(ViewInfo::Constant { value });
        
        let metadata = ConstraintData::NAry { operands };
        
        let mut all_vars = vars.clone();
        all_vars.push(count_var);
        
        self.push_new_prop_with_metadata(
            count_instance,
            ConstraintType::Count,
            all_vars,
            metadata,
        )
    }

    /// Declare a new propagator to enforce that array[index] == value.
    /// This is the element constraint for array indexing operations.
    /// Supports both constant and variable indices with bidirectional propagation.
    pub fn element(&mut self, array: Vec<VarId>, index: VarId, value: VarId) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let mut operands: Vec<ViewInfo> = array.iter()
            .map(|&var_id| ViewInfo::Variable { var_id })
            .collect();
        operands.push(ViewInfo::Variable { var_id: index });
        operands.push(ViewInfo::Variable { var_id: value });
            
        let metadata = ConstraintData::NAry { operands };
        let trigger_vars = {
            let mut vars = array.clone();
            vars.push(index);
            vars.push(value);
            vars
        };
        
        self.push_new_prop_with_metadata(
            self::element::Element::new(array, index, value),
            ConstraintType::Element,
            trigger_vars,
            metadata,
        )
    }

    /// Declare a new propagator to enforce a between constraint: lower <= middle <= upper
    pub fn between_constraint(&mut self, lower: VarId, middle: VarId, upper: VarId) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let variables = vec![lower, middle, upper];
        let operands = vec![
            ViewInfo::Variable { var_id: lower },
            ViewInfo::Variable { var_id: middle },
            ViewInfo::Variable { var_id: upper },
        ];
            
        let metadata = ConstraintData::NAry { operands };
        
        self.push_new_prop_with_metadata(
            self::between::BetweenConstraint::new(lower, middle, upper),
            ConstraintType::Between,
            variables,
            metadata,
        )
    }

    /// Declare a new propagator to enforce "at least N" cardinality constraint
    pub fn at_least_constraint(&mut self, vars: Vec<VarId>, target_value: i32, count: i32) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let operands: Vec<ViewInfo> = vars.iter()
            .map(|&var_id| ViewInfo::Variable { var_id })
            .collect();
            
        let metadata = ConstraintData::NAry { operands };
        
        self.push_new_prop_with_metadata(
            self::cardinality::CardinalityConstraint::at_least(vars.clone(), target_value, count),
            ConstraintType::AtLeast,
            vars,
            metadata,
        )
    }

    /// Declare a new propagator to enforce "at most N" cardinality constraint
    pub fn at_most_constraint(&mut self, vars: Vec<VarId>, target_value: i32, count: i32) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let operands: Vec<ViewInfo> = vars.iter()
            .map(|&var_id| ViewInfo::Variable { var_id })
            .collect();
            
        let metadata = ConstraintData::NAry { operands };
        
        self.push_new_prop_with_metadata(
            self::cardinality::CardinalityConstraint::at_most(vars.clone(), target_value, count),
            ConstraintType::AtMost,
            vars,
            metadata,
        )
    }

    /// Declare a new propagator to enforce "exactly N" cardinality constraint
    pub fn exactly_constraint(&mut self, vars: Vec<VarId>, target_value: i32, count: i32) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let operands: Vec<ViewInfo> = vars.iter()
            .map(|&var_id| ViewInfo::Variable { var_id })
            .collect();
            
        let metadata = ConstraintData::NAry { operands };
        
        self.push_new_prop_with_metadata(
            self::cardinality::CardinalityConstraint::exactly(vars.clone(), target_value, count),
            ConstraintType::Exactly,
            vars,
            metadata,
        )
    }

    /// Declare a new propagator to enforce if-then-else constraint
    pub fn if_then_else_constraint(
        &mut self, 
        condition: self::conditional::Condition,
        then_constraint: self::conditional::SimpleConstraint,
        else_constraint: Option<self::conditional::SimpleConstraint>
    ) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let constraint = self::conditional::IfThenElseConstraint::new(condition, then_constraint, else_constraint);
        let variables = constraint.variables();
        
        let operands: Vec<ViewInfo> = variables.iter()
            .map(|&var_id| ViewInfo::Variable { var_id })
            .collect();
            
        let metadata = ConstraintData::NAry { operands };
        
        self.push_new_prop_with_metadata(
            constraint,
            ConstraintType::IfThenElse,
            variables,
            metadata,
        )
    }

    /// Declare a new propagator to enforce a table constraint.
    /// Variables must take values that correspond to tuples in the allowed table.
    /// This constraint is useful for expressing complex relationships between variables
    /// by explicitly listing all valid combinations.
    pub fn table_constraint(&mut self, vars: Vec<VarId>, tuples: Vec<Vec<crate::variables::Val>>) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let operands: Vec<ViewInfo> = vars.iter()
            .map(|&var_id| ViewInfo::Variable { var_id })
            .collect();
            
        let metadata = ConstraintData::NAry { operands };
        
        self.push_new_prop_with_metadata(
            self::table::Table::new(vars.clone(), tuples),
            ConstraintType::Table,
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
            // Ensure the dependencies matrix is large enough
            while self.dependencies.len() <= v.to_index() {
                self.dependencies.push(Vec::new());
            }
            self.dependencies[v].push(p);
        }

        // Store propagator state as shared trait object
        let boxed = Box::new(state);
        self.state.push(Rc::new(boxed));

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
            let mut vars = crate::variables::Vars::default();
            let mut events = Vec::new();
            let ctx = crate::variables::views::Context::new(&mut vars, &mut events);
            
            let min_val = view.min(&ctx);
            let max_val = view.max(&ctx);
            
            if min_val == max_val {
                // This is a constant value
                let value = match min_val {
                    crate::variables::Val::ValI(i) => ConstraintValue::Integer(i),
                    crate::variables::Val::ValF(f) => ConstraintValue::Float(f),
                };
                ViewInfo::Constant { value }
            } else {
                // For now, mark complex views as such
                // A full implementation would detect transformations
                ViewInfo::Complex
            }
        }
    }

    // Create enhanced constraint methods with metadata collection
    
    /// Declare a new propagator to enforce `x <= y` with metadata collection.
    pub fn less_than_or_equals_with_metadata(&mut self, x: impl View, y: impl View) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData};
        
        let x_info = self.analyze_view(&x);
        let y_info = self.analyze_view(&y);
        let variables: Vec<_> = x.get_underlying_var().into_iter()
            .chain(y.get_underlying_var())
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
            .chain(y.get_underlying_var())
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

    // ═══════════════════════════════════════════════════════════════════════
    // 🔄 Reification Constraints
    // ═══════════════════════════════════════════════════════════════════════

    /// Declare a reified equality constraint: `b ⇔ (x = y)`.
    /// 
    /// The boolean variable `b` is 1 if and only if `x = y`.
    /// - If b = 1, then x must equal y
    /// - If b = 0, then x must not equal y
    /// - If x = y, then b must be 1
    /// - If x ≠ y, then b must be 0
    pub fn int_eq_reif(&mut self, x: VarId, y: VarId, b: VarId) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let x_info = ViewInfo::Variable { var_id: x };
        let y_info = ViewInfo::Variable { var_id: y };
        
        let variables = vec![x, y, b];
        
        let metadata = ConstraintData::Binary {
            left: x_info,
            right: y_info,
        };
        
        self.push_new_prop_with_metadata(
            self::reification::IntEqReif::new(x, y, b),
            ConstraintType::EqualityReified,
            variables,
            metadata,
        )
    }

    /// Declare a reified inequality constraint: `b ⇔ (x ≠ y)`.
    /// 
    /// The boolean variable `b` is 1 if and only if `x ≠ y`.
    /// - If b = 1, then x must not equal y
    /// - If b = 0, then x must equal y
    /// - If x ≠ y, then b must be 1
    /// - If x = y, then b must be 0
    pub fn int_ne_reif(&mut self, x: VarId, y: VarId, b: VarId) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let x_info = ViewInfo::Variable { var_id: x };
        let y_info = ViewInfo::Variable { var_id: y };
        
        let variables = vec![x, y, b];
        
        let metadata = ConstraintData::Binary {
            left: x_info,
            right: y_info,
        };
        
        self.push_new_prop_with_metadata(
            self::reification::IntNeReif::new(x, y, b),
            ConstraintType::InequalityReified,
            variables,
            metadata,
        )
    }

    /// Declare a reified less-than constraint: `b ⇔ (x < y)`.
    /// 
    /// The boolean variable `b` is 1 if and only if `x < y`.
    pub fn int_lt_reif(&mut self, x: VarId, y: VarId, b: VarId) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let x_info = ViewInfo::Variable { var_id: x };
        let y_info = ViewInfo::Variable { var_id: y };
        
        let variables = vec![x, y, b];
        
        let metadata = ConstraintData::Binary {
            left: x_info,
            right: y_info,
        };
        
        self.push_new_prop_with_metadata(
            self::reification::IntLtReif::new(x, y, b),
            ConstraintType::LessThanReified,
            variables,
            metadata,
        )
    }

    /// Declare a reified less-than-or-equal constraint: `b ⇔ (x ≤ y)`.
    /// 
    /// The boolean variable `b` is 1 if and only if `x ≤ y`.
    pub fn int_le_reif(&mut self, x: VarId, y: VarId, b: VarId) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let x_info = ViewInfo::Variable { var_id: x };
        let y_info = ViewInfo::Variable { var_id: y };
        
        let variables = vec![x, y, b];
        
        let metadata = ConstraintData::Binary {
            left: x_info,
            right: y_info,
        };
        
        self.push_new_prop_with_metadata(
            self::reification::IntLeReif::new(x, y, b),
            ConstraintType::LessEqualReified,
            variables,
            metadata,
        )
    }

    /// Declare a reified greater-than constraint: `b ⇔ (x > y)`.
    /// 
    /// The boolean variable `b` is 1 if and only if `x > y`.
    pub fn int_gt_reif(&mut self, x: VarId, y: VarId, b: VarId) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let x_info = ViewInfo::Variable { var_id: x };
        let y_info = ViewInfo::Variable { var_id: y };
        
        let variables = vec![x, y, b];
        
        let metadata = ConstraintData::Binary {
            left: x_info,
            right: y_info,
        };
        
        self.push_new_prop_with_metadata(
            self::reification::IntGtReif::new(x, y, b),
            ConstraintType::GreaterThanReified,
            variables,
            metadata,
        )
    }

    /// Declare a reified greater-than-or-equal constraint: `b ⇔ (x ≥ y)`.
    /// 
    /// The boolean variable `b` is 1 if and only if `x ≥ y`.
    pub fn int_ge_reif(&mut self, x: VarId, y: VarId, b: VarId) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let x_info = ViewInfo::Variable { var_id: x };
        let y_info = ViewInfo::Variable { var_id: y };
        
        let variables = vec![x, y, b];
        
        let metadata = ConstraintData::Binary {
            left: x_info,
            right: y_info,
        };
        
        self.push_new_prop_with_metadata(
            self::reification::IntGeReif::new(x, y, b),
            ConstraintType::GreaterEqualReified,
            variables,
            metadata,
        )
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Linear Constraint Propagators
    // ═══════════════════════════════════════════════════════════════════════

    /// Declare an integer linear equality constraint: `sum(coeffs[i] * vars[i]) = constant`.
    pub fn int_lin_eq(&mut self, coefficients: Vec<i32>, variables: Vec<VarId>, constant: i32) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let operands: Vec<ViewInfo> = variables.iter()
            .map(|&v| ViewInfo::Variable { var_id: v })
            .collect();
        
        let metadata = ConstraintData::NAry { operands };
        
        self.push_new_prop_with_metadata(
            self::linear::IntLinEq::new(coefficients, variables.clone(), constant),
            ConstraintType::Equals,
            variables,
            metadata,
        )
    }

    /// Declare an integer linear less-or-equal constraint: `sum(coeffs[i] * vars[i]) ≤ constant`.
    pub fn int_lin_le(&mut self, coefficients: Vec<i32>, variables: Vec<VarId>, constant: i32) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let operands: Vec<ViewInfo> = variables.iter()
            .map(|&v| ViewInfo::Variable { var_id: v })
            .collect();
        
        let metadata = ConstraintData::NAry { operands };
        
        self.push_new_prop_with_metadata(
            self::linear::IntLinLe::new(coefficients, variables.clone(), constant),
            ConstraintType::LessThanOrEquals,
            variables,
            metadata,
        )
    }

    /// Declare an integer linear not-equal constraint: `sum(coeffs[i] * vars[i]) ≠ constant`.
    pub fn int_lin_ne(&mut self, coefficients: Vec<i32>, variables: Vec<VarId>, constant: i32) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let operands: Vec<ViewInfo> = variables.iter()
            .map(|&v| ViewInfo::Variable { var_id: v })
            .collect();
        
        let metadata = ConstraintData::NAry { operands };
        
        self.push_new_prop_with_metadata(
            self::linear::IntLinNe::new(coefficients, variables.clone(), constant),
            ConstraintType::NotEquals,
            variables,
            metadata,
        )
    }

    /// Declare a reified integer linear equality constraint: `b ⟺ sum(coeffs[i] * vars[i]) = constant`.
    pub fn int_lin_eq_reif(&mut self, coefficients: Vec<i32>, variables: Vec<VarId>, constant: i32, reif_var: VarId) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let mut operands: Vec<ViewInfo> = variables.iter()
            .map(|&v| ViewInfo::Variable { var_id: v })
            .collect();
        operands.push(ViewInfo::Variable { var_id: reif_var });
        
        let metadata = ConstraintData::NAry { operands };
        
        let mut all_vars = variables.clone();
        all_vars.push(reif_var);
        
        self.push_new_prop_with_metadata(
            self::linear::IntLinEqReif::new(coefficients, variables, constant, reif_var),
            ConstraintType::EqualityReified,
            all_vars,
            metadata,
        )
    }

    /// Declare a reified integer linear less-or-equal constraint: `b ⟺ sum(coeffs[i] * vars[i]) ≤ constant`.
    pub fn int_lin_le_reif(&mut self, coefficients: Vec<i32>, variables: Vec<VarId>, constant: i32, reif_var: VarId) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let mut operands: Vec<ViewInfo> = variables.iter()
            .map(|&v| ViewInfo::Variable { var_id: v })
            .collect();
        operands.push(ViewInfo::Variable { var_id: reif_var });
        
        let metadata = ConstraintData::NAry { operands };
        
        let mut all_vars = variables.clone();
        all_vars.push(reif_var);
        
        self.push_new_prop_with_metadata(
            self::linear::IntLinLeReif::new(coefficients, variables, constant, reif_var),
            ConstraintType::InequalityReified,
            all_vars,
            metadata,
        )
    }

    /// Declare a reified integer linear not-equal constraint: `b ⟺ sum(coeffs[i] * vars[i]) ≠ constant`.
    pub fn int_lin_ne_reif(&mut self, coefficients: Vec<i32>, variables: Vec<VarId>, constant: i32, reif_var: VarId) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let mut operands: Vec<ViewInfo> = variables.iter()
            .map(|&v| ViewInfo::Variable { var_id: v })
            .collect();
        operands.push(ViewInfo::Variable { var_id: reif_var });
        
        let metadata = ConstraintData::NAry { operands };
        
        let mut all_vars = variables.clone();
        all_vars.push(reif_var);
        
        self.push_new_prop_with_metadata(
            self::linear::IntLinNeReif::new(coefficients, variables, constant, reif_var),
            ConstraintType::InequalityReified,
            all_vars,
            metadata,
        )
    }

    /// Declare a float linear equality constraint: `sum(coeffs[i] * vars[i]) = constant`.
    pub fn float_lin_eq(&mut self, coefficients: Vec<f64>, variables: Vec<VarId>, constant: f64) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let operands: Vec<ViewInfo> = variables.iter()
            .map(|&v| ViewInfo::Variable { var_id: v })
            .collect();
        
        let metadata = ConstraintData::NAry { operands };
        
        self.push_new_prop_with_metadata(
            self::linear::FloatLinEq::new(coefficients, variables.clone(), constant),
            ConstraintType::Equals,
            variables,
            metadata,
        )
    }

    /// Declare a float linear less-or-equal constraint: `sum(coeffs[i] * vars[i]) ≤ constant`.
    pub fn float_lin_le(&mut self, coefficients: Vec<f64>, variables: Vec<VarId>, constant: f64) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let operands: Vec<ViewInfo> = variables.iter()
            .map(|&v| ViewInfo::Variable { var_id: v })
            .collect();
        
        let metadata = ConstraintData::NAry { operands };
        
        self.push_new_prop_with_metadata(
            self::linear::FloatLinLe::new(coefficients, variables.clone(), constant),
            ConstraintType::LessThanOrEquals,
            variables,
            metadata,
        )
    }

    /// Declare a float linear not-equal constraint: `sum(coeffs[i] * vars[i]) ≠ constant`.
    pub fn float_lin_ne(&mut self, coefficients: Vec<f64>, variables: Vec<VarId>, constant: f64) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let operands: Vec<ViewInfo> = variables.iter()
            .map(|&v| ViewInfo::Variable { var_id: v })
            .collect();
        
        let metadata = ConstraintData::NAry { operands };
        
        self.push_new_prop_with_metadata(
            self::linear::FloatLinNe::new(coefficients, variables.clone(), constant),
            ConstraintType::NotEquals,
            variables,
            metadata,
        )
    }

    /// Declare a reified float linear equality constraint: `b ⟺ sum(coeffs[i] * vars[i]) = constant`.
    pub fn float_lin_eq_reif(&mut self, coefficients: Vec<f64>, variables: Vec<VarId>, constant: f64, reif_var: VarId) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let mut operands: Vec<ViewInfo> = variables.iter()
            .map(|&v| ViewInfo::Variable { var_id: v })
            .collect();
        operands.push(ViewInfo::Variable { var_id: reif_var });
        
        let metadata = ConstraintData::NAry { operands };
        
        let mut all_vars = variables.clone();
        all_vars.push(reif_var);
        
        self.push_new_prop_with_metadata(
            self::linear::FloatLinEqReif::new(coefficients, variables, constant, reif_var),
            ConstraintType::EqualityReified,
            all_vars,
            metadata,
        )
    }

    /// Declare a reified float linear less-or-equal constraint: `b ⟺ sum(coeffs[i] * vars[i]) ≤ constant`.
    pub fn float_lin_le_reif(&mut self, coefficients: Vec<f64>, variables: Vec<VarId>, constant: f64, reif_var: VarId) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let mut operands: Vec<ViewInfo> = variables.iter()
            .map(|&v| ViewInfo::Variable { var_id: v })
            .collect();
        operands.push(ViewInfo::Variable { var_id: reif_var });
        
        let metadata = ConstraintData::NAry { operands };
        
        let mut all_vars = variables.clone();
        all_vars.push(reif_var);
        
        self.push_new_prop_with_metadata(
            self::linear::FloatLinLeReif::new(coefficients, variables, constant, reif_var),
            ConstraintType::InequalityReified,
            all_vars,
            metadata,
        )
    }

    /// Declare a reified float linear not-equal constraint: `b ⟺ sum(coeffs[i] * vars[i]) ≠ constant`.
    pub fn float_lin_ne_reif(&mut self, coefficients: Vec<f64>, variables: Vec<VarId>, constant: f64, reif_var: VarId) -> PropId {
        use crate::optimization::constraint_metadata::{ConstraintType, ConstraintData, ViewInfo};
        
        let mut operands: Vec<ViewInfo> = variables.iter()
            .map(|&v| ViewInfo::Variable { var_id: v })
            .collect();
        operands.push(ViewInfo::Variable { var_id: reif_var });
        
        let metadata = ConstraintData::NAry { operands };
        
        let mut all_vars = variables.clone();
        all_vars.push(reif_var);
        
        self.push_new_prop_with_metadata(
            self::linear::FloatLinNeReif::new(coefficients, variables, constant, reif_var),
            ConstraintType::InequalityReified,
            all_vars,
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
#[doc(hidden)]
pub use alldiff::AllDiff;
#[doc(hidden)]
pub use allequal::AllEqual;
#[doc(hidden)]
pub use count::Count;
