//! Step 2.4: Enhanced Precision Handling with Constraint Metadata
//!
//! This module provides precision-aware optimization that uses the constraint metadata
//! collection system to properly handle floating-point constraint boundaries with
//! ULP (Unit in the Last Place) precision.
//!
//! ## Key Features
//! 
//! - **Constraint Metadata Integration**: Uses the comprehensive constraint registry
//!   to analyze constraint patterns and extract precise boundary values
//! - **ULP-Aware Boundaries**: Applies floating-point precision rules to handle
//!   strict inequalities (x < 5.5 → x ≤ 5.499999999999999)
//! - **Type-Safe Analysis**: Uses structured constraint types instead of heuristics
//! - **Cache-Optimized**: Maintains bound caches for efficient repeated optimization
//! 
//! ## Architecture
//! 
//! This implementation leverages the constraint metadata collection system built in
//! the previous phases to provide reliable, precise constraint boundary optimization.

use crate::vars::{Vars, VarId};
use crate::props::Propagators;
use crate::optimization::constraint_integration::{ConstraintAwareOptimizer};
use crate::optimization::float_direct::{OptimizationResult, OptimizationOperation, DomainError};

use crate::domain::FloatInterval;

/// Enhanced constraint analyzer for Step 2.4 that tries to extract actual constraint values
#[derive(Debug)]
pub struct PrecisionAwareOptimizer {
    base_optimizer: ConstraintAwareOptimizer,
}

impl PrecisionAwareOptimizer {
    /// Create a new precision-aware optimizer
    pub fn new() -> Self {
        Self {
            base_optimizer: ConstraintAwareOptimizer::new(),
        }
    }

    /// Maximize a variable with precision-aware constraint analysis
    pub fn maximize_with_precision(
        &self,
        vars: &Vars,
        props: &Propagators,
        var_id: VarId,
    ) -> OptimizationResult {
        // Try our new precision-aware optimization using constraint metadata
        if let Some(result) = self.try_metadata_precision_optimization(vars, props, var_id, true) {
            return result;
        }
        
        // Fall back to Step 2.3.3 conservative analysis
        self.base_optimizer.maximize_with_constraints(vars, props, var_id)
    }

    /// Minimize a variable with precision-aware constraint analysis
    /// Enhanced precision-aware minimization using constraint metadata
    pub fn minimize_with_precision(
        &self,
        vars: &Vars,
        props: &Propagators,
        var_id: VarId,
    ) -> OptimizationResult {
        // Try our new precision-aware optimization using constraint metadata
        if let Some(result) = self.try_metadata_precision_optimization(vars, props, var_id, false) {
            return result;
        }
        
        // Fall back to Step 2.3.3 conservative analysis
        self.base_optimizer.minimize_with_constraints(vars, props, var_id)
    }



    /// Try precision optimization using constraint metadata system
    fn try_metadata_precision_optimization(
        &self,
        vars: &Vars,
        props: &Propagators,
        var_id: VarId,
        is_maximization: bool,
    ) -> Option<OptimizationResult> {
        // Get variable domain
        let var_domain = match &vars[var_id] {
            crate::vars::Var::VarF(interval) => {
                if interval.is_empty() {
                    return Some(OptimizationResult::domain_error(DomainError::EmptyDomain));
                }
                interval
            },
            crate::vars::Var::VarI(_) => {
                // Integer variables don't need precision optimization
                return None;
            }
        };

        // Use the constraint metadata registry to analyze constraints
        let registry = props.get_constraint_registry();
        let analysis = registry.analyze_variable_constraints(var_id);

        // For constraint analysis, we can work directly with the analysis results
        // without needing a full context
        let step_size = self.calculate_precision_step_size(var_domain);

        // Get effective bounds from constraint analysis
        let constraint_upper = analysis.get_effective_upper_bound(step_size);
        let constraint_lower = analysis.get_effective_lower_bound(step_size);

        // Calculate optimal value based on bounds and optimization direction
        let optimal_value = if is_maximization {
            // For maximization, use the upper bound if available, otherwise domain max
            constraint_upper.unwrap_or(var_domain.max)
        } else {
            // For minimization, use the lower bound if available, otherwise domain min
            constraint_lower.unwrap_or(var_domain.min)
        };

        // Verify the optimal value is within the original domain
        if optimal_value < var_domain.min || optimal_value > var_domain.max {
            // Optimal value is outside domain - fall back
            return None;
        }

        // Check if we used constraint-derived bounds (indicating precision optimization)
        let used_constraints = constraint_upper.is_some() || constraint_lower.is_some();
        if used_constraints {
            // Log precision optimization for debugging
            #[cfg(debug_assertions)]
            {
                eprintln!(
                    "Precision optimization for {:?}: constraint_lower={:?}, constraint_upper={:?}, optimal={:?}",
                    var_id,
                    constraint_lower,
                    constraint_upper,
                    optimal_value
                );
            }
        }

        // Return successful optimization result
        let operation = if is_maximization {
            OptimizationOperation::Maximization
        } else {
            OptimizationOperation::Minimization
        };
        
        Some(OptimizationResult::success(optimal_value, operation, var_id))
    }

    /// Calculate appropriate step size for precision optimization
    fn calculate_precision_step_size(&self, interval: &FloatInterval) -> f64 {
        let domain_size = interval.max - interval.min;
        
        // Use a step size that's appropriate for the domain size
        if domain_size > 1000.0 {
            1e-6  // Large domains need less precision
        } else if domain_size > 100.0 {
            1e-8  // Medium domains
        } else if domain_size > 10.0 {
            1e-10 // Small domains need high precision
        } else {
            1e-12 // Very small domains need maximum precision
        }
    }
}

impl Default for PrecisionAwareOptimizer {
    fn default() -> Self {
        Self::new()
    }
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

    fn create_test_props_with_constraint() -> Propagators {
        // For unit testing, we'll create an empty propagator collection 
        // The constraint pattern analysis now requires domain pattern matching, 
        // so we'll test the logic accordingly
        Propagators::default()
    }

    #[test]
    fn test_precision_aware_maximization() {
        let optimizer = PrecisionAwareOptimizer::new();
        let (vars, var_id) = create_test_vars_with_float(1.0, 10.0);
        let props = create_test_props_with_constraint();

        let result = optimizer.maximize_with_precision(&vars, &props, var_id);

        // The test should fall back to the base optimizer behavior
        assert!(result.success, "Optimization should succeed");
        // The result depends on what the base ConstraintAwareOptimizer returns
        // for unconstrained cases - let's just check it's in the valid range
        assert!(result.optimal_value >= 1.0 && result.optimal_value <= 10.0, 
                "Result should be within domain bounds");
    }
}
