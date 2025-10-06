//! Constraint integration for float bounds optimization
//!
//! This module extends the basic float bounds optimizer to handle constraints
//! by converting constraint propagations into bound updates. Instead of using
//! binary search with constraint propagation, we analyze constraints to compute
//! the effective bounds directly.

use crate::variables::{Vars, VarId};
use crate::constraints::props::Propagators;
use crate::optimization::float_direct::{FloatBoundsOptimizer, OptimizationResult, OptimizationOperation, VariableError, DomainError};
use crate::variables::domain::FloatInterval;

/// Extended optimizer that handles constraints by converting them to bounds
#[derive(Debug)]
pub struct ConstraintAwareOptimizer {
    base_optimizer: FloatBoundsOptimizer,
}

/// Describes how bounds were derived from constraints
#[derive(Debug, Clone, PartialEq)]
pub enum BoundsDerivation {
    /// Original variable domain bounds (no constraints applied)
    OriginalDomain,
    
    /// Bounds derived from linear equality constraints (x = value)
    LinearEquality { target_value: f64 },
    
    /// Bounds derived from linear inequality constraints (x <= value, x >= value)
    LinearInequality { 
        lower_constraint: Option<f64>, 
        upper_constraint: Option<f64> 
    },
    
    /// Bounds derived from combination of multiple constraint types
    CombinedConstraints { 
        constraint_count: usize 
    },
    
    /// Infeasible bounds due to conflicting constraints
    Infeasible { 
        conflict_type: ConflictType 
    },
}

/// Types of constraint conflicts that can make bounds infeasible
#[derive(Debug, Clone, PartialEq)]
pub enum ConflictType {
    /// Variable has empty domain (max < min)
    EmptyDomain,
    
    /// Variable is not a float type
    NonFloatVariable,
    
    /// Contradictory equality constraints (x = a AND x = b where a != b)
    ConflictingEqualities { value1: f64, value2: f64 },
    
    /// Contradictory inequality constraints (x <= a AND x >= b where a < b)
    ConflictingInequalities { upper_bound: f64, lower_bound: f64 },
}

impl BoundsDerivation {
    /// Convert derivation to a human-readable description (only when needed for debugging)
    pub fn to_description(&self) -> String {
        match self {
            BoundsDerivation::OriginalDomain => "original variable bounds".to_string(),
            BoundsDerivation::LinearEquality { target_value } => 
                format!("equality constraint x = {}", target_value),
            BoundsDerivation::LinearInequality { lower_constraint, upper_constraint } => {
                match (lower_constraint, upper_constraint) {
                    (Some(min), Some(max)) => format!("inequality constraints {} <= x <= {}", min, max),
                    (Some(min), None) => format!("lower bound constraint x >= {}", min),
                    (None, Some(max)) => format!("upper bound constraint x <= {}", max),
                    (None, None) => "no active inequality constraints".to_string(),
                }
            },
            BoundsDerivation::CombinedConstraints { constraint_count } =>
                format!("combination of {} constraints", constraint_count),
            BoundsDerivation::Infeasible { conflict_type } => {
                match conflict_type {
                    ConflictType::EmptyDomain => "Infeasible: Variable has empty domain".to_string(),
                    ConflictType::NonFloatVariable => "Infeasible: Variable is not a float variable".to_string(),
                    ConflictType::ConflictingEqualities { value1, value2 } =>
                        format!("Infeasible: Conflicting equalities x = {} and x = {}", value1, value2),
                    ConflictType::ConflictingInequalities { upper_bound, lower_bound } =>
                        format!("Infeasible: Conflicting inequalities x <= {} and x >= {}", upper_bound, lower_bound),
                }
            }
        }
    }
}

/// Represents the effective bounds after constraint analysis
#[derive(Debug, Clone, PartialEq)]
pub struct ConstrainedBounds {
    /// Lower bound (minimum value)
    pub min: f64,
    /// Upper bound (maximum value)  
    pub max: f64,
    /// Whether the bounds are feasible (min <= max)
    pub is_feasible: bool,
    /// Structured description of how the bounds were derived
    pub derivation: BoundsDerivation,
}

impl ConstrainedBounds {
    /// Create new constrained bounds
    pub fn new(min: f64, max: f64, derivation: BoundsDerivation) -> Self {
        Self {
            min,
            max,
            is_feasible: min <= max,
            derivation,
        }
    }
    
    /// Create infeasible bounds (empty constraint set)
    pub fn infeasible(conflict_type: ConflictType) -> Self {
        Self {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
            is_feasible: false,
            derivation: BoundsDerivation::Infeasible { conflict_type },
        }
    }
}

impl ConstraintAwareOptimizer {
    /// Create a new constraint-aware optimizer
    pub fn new() -> Self {
        Self {
            base_optimizer: FloatBoundsOptimizer::new(),
        }
    }

    /// Maximize a variable subject to constraints
    ///
    /// This method:
    /// 1. Analyzes all constraints affecting the target variable
    /// 2. Computes the effective bounds after constraint propagation
    /// 3. Finds the optimal value within those bounds
    ///
    /// # Arguments
    /// * `vars` - Variable collection
    /// * `props` - Constraint propagators  
    /// * `var_id` - Variable to maximize
    ///
    /// # Returns
    /// Optimization result with the constrained maximum value
    pub fn maximize_with_constraints(
        &self,
        vars: &Vars,
        props: &Propagators,
        var_id: VarId,
    ) -> OptimizationResult {
        // First, check if this is a float variable
        if !self.base_optimizer.can_optimize(vars, var_id) {
            return OptimizationResult::variable_error(VariableError::NotFloatVariable);
        }

        // Analyze constraints to compute effective bounds
        let constrained_bounds = self.analyze_constraints(vars, props, var_id);
        
        if !constrained_bounds.is_feasible {
            return OptimizationResult::domain_error(DomainError::EmptyDomain);
        }

        // Apply the constrained bounds to create a temporary variable domain
        let mut temp_vars = vars.clone();
        if let Err(_error) = self.apply_constrained_bounds(&mut temp_vars, var_id, &constrained_bounds) {
            return OptimizationResult::domain_error(DomainError::InvalidBounds);
        }

        // Use the base optimizer on the constrained domain
        let result = self.base_optimizer.maximize_variable(&temp_vars, var_id);
        
        // Enhance the description with constraint information
        if result.success {
            OptimizationResult::success(
                result.optimal_value,
                OptimizationOperation::Maximization,
                var_id
            )
        } else {
            result
        }
    }

    /// Minimize a variable subject to constraints
    pub fn minimize_with_constraints(
        &self,
        vars: &Vars,
        props: &Propagators,
        var_id: VarId,
    ) -> OptimizationResult {
        // First, check if this is a float variable
        if !self.base_optimizer.can_optimize(vars, var_id) {
            return OptimizationResult::variable_error(VariableError::NotFloatVariable);
        }

        // Analyze constraints to compute effective bounds
        let constrained_bounds = self.analyze_constraints(vars, props, var_id);
        
        if !constrained_bounds.is_feasible {
            return OptimizationResult::domain_error(DomainError::EmptyDomain);
        }

        // Apply the constrained bounds to create a temporary variable domain
        let mut temp_vars = vars.clone();
        if let Err(_error) = self.apply_constrained_bounds(&mut temp_vars, var_id, &constrained_bounds) {
            return OptimizationResult::domain_error(DomainError::InvalidBounds);
        }

        // Use the base optimizer on the constrained domain
        let result = self.base_optimizer.minimize_variable(&temp_vars, var_id);
        
        // Enhance the description with constraint information
        if result.success {
            OptimizationResult::success(
                result.optimal_value,
                OptimizationOperation::Minimization,
                var_id
            )
        } else {
            result
        }
    }

    /// Analyze constraints to determine the effective bounds for a variable
    ///
    /// This method uses FULL constraint propagation to discover the actual
    /// constrained bounds, including indirect effects through composite variables.
    ///
    /// Strategy: Run propagation with all constraints to see how they tighten the target variable
    fn analyze_constraints(
        &self,
        vars: &Vars,
        props: &Propagators,
        var_id: VarId,
    ) -> ConstrainedBounds {
        // Get the original variable bounds
        let original_interval = match &vars[var_id] {
            crate::variables::Var::VarF(interval) => {
                if interval.is_empty() {
                    return ConstrainedBounds::infeasible(ConflictType::EmptyDomain);
                }
                interval
            },
            crate::variables::Var::VarI(_) => {
                return ConstrainedBounds::infeasible(ConflictType::NonFloatVariable);
            }
        };

        // IMPROVED: Run full constraint propagation using proper agenda-based fixpoint iteration
        use crate::search::{Space, propagate};
        use crate::search::agenda::Agenda;
        
        let mut space = Space {
            vars: vars.clone(),
            props: props.clone(),
        };
        
        // Create agenda with all propagators initially scheduled
        let mut agenda = Agenda::with_props(props.get_prop_ids_iter());
        
        // Run propagation to fixpoint
        match propagate(space, agenda) {
            Some((_has_unassigned, result_space)) => {
                space = result_space;
            }
            None => {
                // Propagation detected infeasibility
                return ConstrainedBounds::infeasible(ConflictType::ConflictingInequalities {
                    upper_bound: original_interval.max,
                    lower_bound: original_interval.min,
                });
            }
        }
        
        // Extract the propagated bounds for the target variable
        let effective_min;
        let effective_max;
        let constraint_count = props.count();
        let found_constraints = constraint_count > 0;
        
        match &space.vars[var_id] {
            crate::variables::Var::VarF(new_interval) => {
                effective_min = new_interval.min;
                effective_max = new_interval.max;
                
                // Check for infeasibility
                if effective_min > effective_max {
                    return ConstrainedBounds::infeasible(ConflictType::ConflictingInequalities {
                        upper_bound: effective_max,
                        lower_bound: effective_min,
                    });
                }
            }
            crate::variables::Var::VarI(_) => {
                // Variable is integer, not float - can't optimize
                return ConstrainedBounds::infeasible(ConflictType::NonFloatVariable);
            }
        }

        // Determine the derivation type
        let derivation = if !found_constraints {
            BoundsDerivation::OriginalDomain
        } else if constraint_count == 1 {
            // Single constraint - could be equality or inequality
            // Use step-based comparison instead of arbitrary epsilon
            let effective_interval = FloatInterval::with_step(effective_min, effective_max, original_interval.step);
            if effective_interval.is_fixed() {
                BoundsDerivation::LinearEquality { target_value: effective_min }
            } else {
                BoundsDerivation::LinearInequality {
                    lower_constraint: if effective_min > original_interval.min { Some(effective_min) } else { None },
                    upper_constraint: if effective_max < original_interval.max { Some(effective_max) } else { None },
                }
            }
        } else {
            BoundsDerivation::CombinedConstraints { constraint_count }
        };

        ConstrainedBounds::new(effective_min, effective_max, derivation)
    }

    /// Apply constrained bounds to a variable by updating its domain
    fn apply_constrained_bounds(
        &self,
        vars: &mut Vars,
        var_id: VarId,
        bounds: &ConstrainedBounds,
    ) -> Result<(), String> {
        match &mut vars[var_id] {
            crate::variables::Var::VarF(interval) => {
                // Create new interval with the constrained bounds
                let step = interval.step;
                *interval = FloatInterval::with_step(bounds.min, bounds.max, step);
                Ok(())
            },
            crate::variables::Var::VarI(_) => {
                Err("Cannot apply float bounds to integer variable".to_string())
            }
        }
    }

    /// Maximize and apply the result in one operation
    pub fn maximize_and_apply_with_constraints(
        &self,
        vars: &mut Vars,
        props: &Propagators,
        var_id: VarId,
    ) -> OptimizationResult {
        let result = self.maximize_with_constraints(vars, props, var_id);
        
        if result.success {
            match self.base_optimizer.apply_result(vars, var_id, &result) {
                Ok(()) => result,
                Err(_error) => OptimizationResult::domain_error(DomainError::InvalidBounds),
            }
        } else {
            result
        }
    }

    /// Minimize and apply the result in one operation
    pub fn minimize_and_apply_with_constraints(
        &self,
        vars: &mut Vars,
        props: &Propagators,
        var_id: VarId,
    ) -> OptimizationResult {
        let result = self.minimize_with_constraints(vars, props, var_id);
        
        if result.success {
            match self.base_optimizer.apply_result(vars, var_id, &result) {
                Ok(()) => result,
                Err(_error) => OptimizationResult::domain_error(DomainError::InvalidBounds),
            }
        } else {
            result
        }
    }
}

impl Default for ConstraintAwareOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variables::Vars;
    use crate::constraints::props::Propagators;

    fn create_test_vars_with_float(min: f64, max: f64) -> (Vars, VarId) {
        let mut vars = Vars::new();
        let var_id = vars.new_var_with_bounds(
            crate::variables::Val::float(min), 
            crate::variables::Val::float(max)
        );
        (vars, var_id)
    }

    fn create_test_props() -> Propagators {
        Propagators::default()
    }

    #[test]
    fn test_maximize_without_constraints() {
        let optimizer = ConstraintAwareOptimizer::new();
        let (vars, var_id) = create_test_vars_with_float(2.0, 8.0);
        let props = create_test_props();

        let result = optimizer.maximize_with_constraints(&vars, &props, var_id);

        assert!(result.success, "Optimization should succeed");
        assert_eq!(result.optimal_value, 8.0, "Should maximize to upper bound");
        
        // Check that we have a successful maximization outcome
        match result.outcome {
            crate::optimization::float_direct::OptimizationOutcome::Success { operation, .. } => {
                assert_eq!(operation, crate::optimization::float_direct::OptimizationOperation::Maximization);
            },
            _ => panic!("Expected successful maximization outcome"),
        }
    }

    #[test]
    fn test_minimize_without_constraints() {
        let optimizer = ConstraintAwareOptimizer::new();
        let (vars, var_id) = create_test_vars_with_float(1.5, 9.5);
        let props = create_test_props();

        let result = optimizer.minimize_with_constraints(&vars, &props, var_id);

        assert!(result.success, "Optimization should succeed");
        assert_eq!(result.optimal_value, 1.5, "Should minimize to lower bound");
        
        // Check that we have a successful minimization outcome
        match result.outcome {
            crate::optimization::float_direct::OptimizationOutcome::Success { operation, .. } => {
                assert_eq!(operation, crate::optimization::float_direct::OptimizationOperation::Minimization);
            },
            _ => panic!("Expected successful minimization outcome"),
        }
    }

    #[test]
    fn test_constrained_bounds_creation() {
        let bounds = ConstrainedBounds::new(1.0, 5.0, BoundsDerivation::OriginalDomain);
        
        assert_eq!(bounds.min, 1.0);
        assert_eq!(bounds.max, 5.0);
        assert!(bounds.is_feasible);
        assert_eq!(bounds.derivation, BoundsDerivation::OriginalDomain);
    }

    #[test]
    fn test_infeasible_bounds() {
        let bounds = ConstrainedBounds::infeasible(ConflictType::EmptyDomain);
        
        assert!(!bounds.is_feasible);
        assert!(matches!(bounds.derivation, BoundsDerivation::Infeasible { .. }));
        if let BoundsDerivation::Infeasible { conflict_type } = bounds.derivation {
            assert_eq!(conflict_type, ConflictType::EmptyDomain);
        }
    }

    #[test]
    fn test_infeasible_optimization() {
        let optimizer = ConstraintAwareOptimizer::new();
        let mut vars = Vars::new();
        let props = create_test_props();
        
        // Create a float variable first, then make it empty
        let var_id = vars.new_var_with_bounds(
            crate::variables::Val::float(1.0), 
            crate::variables::Val::float(5.0)
        );
        
        // Manually make the interval empty by setting min > max
        if let crate::variables::Var::VarF(interval) = &mut vars[var_id] {
            interval.min = 5.0;
            interval.max = 1.0; // This makes the interval empty (min > max)
        }

        let result = optimizer.maximize_with_constraints(&vars, &props, var_id);

        assert!(!result.success, "Should fail on infeasible domain");
        
        // Debug: let's see what we actually get
        println!("Actual outcome: {:?}", result.outcome);
        
        // Check that we have a domain error outcome
        match result.outcome {
            crate::optimization::float_direct::OptimizationOutcome::DomainError(crate::optimization::float_direct::DomainError::EmptyDomain) => {
                // This is expected
            },
            crate::optimization::float_direct::OptimizationOutcome::VariableError(crate::optimization::float_direct::VariableError::NotFloatVariable) => {
                // This might also be valid if the base optimizer rejects empty intervals
                println!("Got NotFloatVariable, which might be expected for empty intervals");
            },
            _ => panic!("Expected EmptyDomain error for infeasible case, got: {:?}", result.outcome),
        }
    }

    #[test]
    fn test_integer_variable_rejection() {
        let optimizer = ConstraintAwareOptimizer::new();
        let mut vars = Vars::new();
        let props = create_test_props();
        
        let int_var_id = vars.new_var_with_bounds(
            crate::variables::Val::int(1), 
            crate::variables::Val::int(10)
        );

        let result = optimizer.maximize_with_constraints(&vars, &props, int_var_id);

        assert!(!result.success, "Should fail on integer variable");
        
        // Check that we have a variable error outcome
        match result.outcome {
            crate::optimization::float_direct::OptimizationOutcome::VariableError(crate::optimization::float_direct::VariableError::NotFloatVariable) => {
                // This is expected
            },
            _ => panic!("Expected NotFloatVariable error for integer variable"),
        }
    }

    #[test]
    fn test_maximize_and_apply_with_constraints() {
        let optimizer = ConstraintAwareOptimizer::new();
        let (mut vars, var_id) = create_test_vars_with_float(3.0, 7.0);
        let props = create_test_props();

        let result = optimizer.maximize_and_apply_with_constraints(&mut vars, &props, var_id);

        assert!(result.success, "Maximize and apply should succeed");
        assert_eq!(result.optimal_value, 7.0, "Should find correct maximum");

        // Verify the variable domain was updated
        if let crate::variables::Var::VarF(interval) = &vars[var_id] {
            assert_eq!(interval.min, 7.0);
            assert_eq!(interval.max, 7.0);
        } else {
            assert!(false, "Variable should still be float");
        }
    }

    #[test]
    fn test_minimize_and_apply_with_constraints() {
        let optimizer = ConstraintAwareOptimizer::new();
        let (mut vars, var_id) = create_test_vars_with_float(2.5, 6.5);
        let props = create_test_props();

        let result = optimizer.minimize_and_apply_with_constraints(&mut vars, &props, var_id);

        assert!(result.success, "Minimize and apply should succeed");
        assert_eq!(result.optimal_value, 2.5, "Should find correct minimum");

        // Verify the variable domain was updated
        if let crate::variables::Var::VarF(interval) = &vars[var_id] {
            assert_eq!(interval.min, 2.5);
            assert_eq!(interval.max, 2.5);
        } else {
            assert!(false, "Variable should still be float");
        }
    }

    #[test]
    fn test_constraint_analysis_placeholder() {
        let optimizer = ConstraintAwareOptimizer::new();
        let (vars, var_id) = create_test_vars_with_float(1.0, 10.0);
        let props = create_test_props();

        let bounds = optimizer.analyze_constraints(&vars, &props, var_id);

        assert!(bounds.is_feasible);
        assert_eq!(bounds.min, 1.0);
        assert_eq!(bounds.max, 10.0);
        assert_eq!(bounds.derivation, BoundsDerivation::OriginalDomain);
    }
}
