//! Constraint integration for float bounds optimization
//!
//! This module extends the basic float bounds optimizer to handle constraints
//! by converting constraint propagations into bound updates. Instead of using
//! binary search with constraint propagation, we analyze constraints to compute
//! the effective bounds directly.

use crate::vars::{Vars, VarId};
use crate::props::Propagators;
use crate::optimization::float_direct::{FloatBoundsOptimizer, OptimizationResult};
use crate::domain::FloatInterval;

/// Extended optimizer that handles constraints by converting them to bounds
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
            return OptimizationResult::failure(
                format!("Cannot optimize variable {}: not a float variable or empty domain", var_id_to_string(var_id))
            );
        }

        // Analyze constraints to compute effective bounds
        let constrained_bounds = self.analyze_constraints(vars, props, var_id);
        
        if !constrained_bounds.is_feasible {
            return OptimizationResult::failure(constrained_bounds.derivation.to_description());
        }

        // Apply the constrained bounds to create a temporary variable domain
        let mut temp_vars = vars.clone();
        if let Err(error) = self.apply_constrained_bounds(&mut temp_vars, var_id, &constrained_bounds) {
            return OptimizationResult::failure(error);
        }

        // Use the base optimizer on the constrained domain
        let result = self.base_optimizer.maximize_variable(&temp_vars, var_id);
        
        // Enhance the description with constraint information
        if result.success {
            OptimizationResult::success(
                result.optimal_value,
                format!("{} (constrained by: {})", result.description, constrained_bounds.derivation.to_description())
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
            return OptimizationResult::failure(
                format!("Cannot optimize variable {}: not a float variable or empty domain", var_id_to_string(var_id))
            );
        }

        // Analyze constraints to compute effective bounds
        let constrained_bounds = self.analyze_constraints(vars, props, var_id);
        
        if !constrained_bounds.is_feasible {
            return OptimizationResult::failure(constrained_bounds.derivation.to_description());
        }

        // Apply the constrained bounds to create a temporary variable domain
        let mut temp_vars = vars.clone();
        if let Err(error) = self.apply_constrained_bounds(&mut temp_vars, var_id, &constrained_bounds) {
            return OptimizationResult::failure(error);
        }

        // Use the base optimizer on the constrained domain
        let result = self.base_optimizer.minimize_variable(&temp_vars, var_id);
        
        // Enhance the description with constraint information
        if result.success {
            OptimizationResult::success(
                result.optimal_value,
                format!("{} (constrained by: {})", result.description, constrained_bounds.derivation.to_description())
            )
        } else {
            result
        }
    }

    /// Analyze constraints to determine the effective bounds for a variable
    ///
    /// This is where the magic happens - instead of binary search + propagation,
    /// we analyze the constraint structure to compute bounds directly.
    ///
    /// For now, this is a placeholder that returns the original variable bounds.
    /// In the next implementation steps, we'll add analysis for specific constraint types.
    fn analyze_constraints(
        &self,
        vars: &Vars,
        _props: &Propagators,
        var_id: VarId,
    ) -> ConstrainedBounds {
        // Get the original variable bounds
        match &vars[var_id] {
            crate::vars::Var::VarF(interval) => {
                if interval.is_empty() {
                    ConstrainedBounds::infeasible(ConflictType::EmptyDomain)
                } else {
                    // For now, return the original bounds
                    // TODO: In future steps, analyze propagators to find additional constraints
                    ConstrainedBounds::new(
                        interval.min,
                        interval.max,
                        BoundsDerivation::OriginalDomain
                    )
                }
            },
            crate::vars::Var::VarI(_) => {
                ConstrainedBounds::infeasible(ConflictType::NonFloatVariable)
            }
        }
    }

    /// Apply constrained bounds to a variable by updating its domain
    fn apply_constrained_bounds(
        &self,
        vars: &mut Vars,
        var_id: VarId,
        bounds: &ConstrainedBounds,
    ) -> Result<(), String> {
        match &mut vars[var_id] {
            crate::vars::Var::VarF(interval) => {
                // Create new interval with the constrained bounds
                let step = interval.step;
                *interval = FloatInterval::with_step(bounds.min, bounds.max, step);
                Ok(())
            },
            crate::vars::Var::VarI(_) => {
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
                Err(error) => OptimizationResult::failure(error),
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
                Err(error) => OptimizationResult::failure(error),
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

/// Helper function to convert VarId to string for error messages
fn var_id_to_string(var_id: VarId) -> String {
    format!("VarId({:?})", var_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vars::Vars;
    use crate::props::Propagators;

    fn create_test_vars_with_float(min: f64, max: f64) -> (Vars, VarId) {
        let mut vars = Vars::new();
        let var_id = vars.new_var_with_bounds(
            crate::vars::Val::float(min), 
            crate::vars::Val::float(max)
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
        assert!(result.description.contains("original variable bounds"));
    }

    #[test]
    fn test_minimize_without_constraints() {
        let optimizer = ConstraintAwareOptimizer::new();
        let (vars, var_id) = create_test_vars_with_float(1.5, 9.5);
        let props = create_test_props();

        let result = optimizer.minimize_with_constraints(&vars, &props, var_id);

        assert!(result.success, "Optimization should succeed");
        assert_eq!(result.optimal_value, 1.5, "Should minimize to lower bound");
        assert!(result.description.contains("original variable bounds"));
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
            crate::vars::Val::float(1.0), 
            crate::vars::Val::float(5.0)
        );
        
        // Manually make the interval empty by setting min > max
        if let crate::vars::Var::VarF(interval) = &mut vars[var_id] {
            interval.min = 5.0;
            interval.max = 1.0; // This makes the interval empty (min > max)
        }

        let result = optimizer.maximize_with_constraints(&vars, &props, var_id);

        assert!(!result.success, "Should fail on infeasible domain");
        assert!(result.description.contains("empty domain"));
    }

    #[test]
    fn test_integer_variable_rejection() {
        let optimizer = ConstraintAwareOptimizer::new();
        let mut vars = Vars::new();
        let props = create_test_props();
        
        let int_var_id = vars.new_var_with_bounds(
            crate::vars::Val::int(1), 
            crate::vars::Val::int(10)
        );

        let result = optimizer.maximize_with_constraints(&vars, &props, int_var_id);

        assert!(!result.success, "Should fail on integer variable");
        assert!(result.description.contains("not a float variable"));
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
        if let crate::vars::Var::VarF(interval) = &vars[var_id] {
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
        if let crate::vars::Var::VarF(interval) = &vars[var_id] {
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
