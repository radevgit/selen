//! Precision-Aware Constraint Boundary Propagator
//! 
//! This module provides a propagator that uses constraint metadata and precision
//! optimization to properly handle floating-point constraint boundaries.

use crate::constraints::props::{Prune, Propagate};
use crate::variables::VarId;
use crate::variables::views::Context;
use crate::optimization::precision_optimizer::PrecisionOptimizer;
use crate::optimization::ulp_utils::UlpUtils;
use crate::optimization::constraint_metadata::ConstraintRegistry;
use std::collections::HashSet;

/// A propagator that applies precision-aware constraint boundary optimization
#[derive(Debug, Clone)]
pub struct PrecisionBoundaryPropagator {
    /// Variables that this propagator affects
    variables: Vec<VarId>,
    /// Step size for precision calculations
    step_size: f64,
}

impl PrecisionBoundaryPropagator {
    /// Create a new precision boundary propagator for the given variables
    pub fn new(variables: Vec<VarId>, step_size: f64) -> Self {
        Self {
            variables,
            step_size,
        }
    }

    /// Create a propagator for a single variable
    pub fn for_variable(var_id: VarId, step_size: f64) -> Self {
        Self::new(vec![var_id], step_size)
    }

    /// Apply precision optimization to all variables using constraint metadata
    pub fn apply_precision_optimization(
        &self,
        ctx: &mut Context,
        registry: &ConstraintRegistry,
    ) -> Option<()> {
        let mut optimizer = PrecisionOptimizer::new(self.step_size);
        let mut any_changed = false;

        for &var_id in &self.variables {
            // Optimize bounds for this variable
            let bounds = optimizer.optimize_bounds(var_id, registry, ctx.vars()).ok()?;
            
            // Apply the optimized bounds using Context API
            let changed = self.apply_precision_bounds(var_id, &bounds, ctx).ok()?;
            
            if changed {
                any_changed = true;
                
                // Log precision adjustments for debugging
                if bounds.precision_adjusted {
                    #[cfg(debug_assertions)]
                    {
                        eprintln!(
                            "Precision adjustment for variable {:?}: original_upper={:?}, new_upper={:?}, original_lower={:?}, new_lower={:?}",
                            var_id,
                            bounds.original_upper,
                            bounds.upper_bound,
                            bounds.original_lower,
                            bounds.lower_bound
                        );
                    }
                }
            }
        }

        if any_changed {
            Some(())
        } else {
            Some(())
        }
    }

    /// Apply precision bounds to a variable using the Context API
    fn apply_precision_bounds(
        &self,
        var_id: VarId,
        bounds: &crate::optimization::precision_optimizer::PrecisionBounds,
        ctx: &mut Context,
    ) -> Result<bool, String> {
        let mut changed = false;

        // Apply lower bound if available
        if let Some(min) = bounds.lower_bound {
            if ctx.try_set_min(var_id, min.into()).is_none() {
                return Err("Failed to set minimum bound".to_string());
            }
            changed = true;
        }

        // Apply upper bound if available
        if let Some(max) = bounds.upper_bound {
            if ctx.try_set_max(var_id, max.into()).is_none() {
                return Err("Failed to set maximum bound".to_string());
            }
            changed = true;
        }

        Ok(changed)
    }
}

impl Prune for PrecisionBoundaryPropagator {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        // Simplified precision-aware boundary check
        // This is a basic implementation that focuses on the most common precision issues
        
        for &var_id in &self.variables {
            // Access variable directly through context
            let var = &ctx.vars()[var_id];
            
            // Check if we're dealing with floating-point values
            if let crate::variables::Var::VarF(interval) = var {
                let current_min = interval.min;
                let current_max = interval.max;
                
                // Apply ULP-aware boundary adjustments for values that look like
                // they might be constraint boundaries
                
                // If the max value looks like it might be a strict constraint boundary
                // (e.g., very close to a "round" number like 5.5), suggest precision adjustment
                if self.looks_like_constraint_boundary(current_max) {
                    let precision_max = UlpUtils::strict_upper_bound(current_max);
                    if precision_max < current_max {
                        // Try to set the new maximum using the proper Context API
                        ctx.try_set_max(var_id, crate::variables::Val::ValF(precision_max))?;
                    }
                }
                
                // Similar for min values
                if self.looks_like_constraint_boundary(current_min) {
                    let precision_min = UlpUtils::strict_lower_bound(current_min);
                    if precision_min > current_min {
                        // Try to set the new minimum using the proper Context API
                        ctx.try_set_min(var_id, crate::variables::Val::ValF(precision_min))?;
                    }
                }
            }
        }
        
        Some(())
    }
}

impl PrecisionBoundaryPropagator {
    /// Heuristic to detect if a value looks like a constraint boundary
    fn looks_like_constraint_boundary(&self, value: f64) -> bool {
        // Check if the value is "round" (has few decimal places)
        let rounded = (value * 10.0).round() / 10.0;
        (value - rounded).abs() < f64::EPSILON
    }
}

impl Propagate for PrecisionBoundaryPropagator {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        self.variables.iter().copied()
    }
}

/// Helper function to create precision boundary propagators for all variables
/// that are involved in floating-point constraints
pub fn create_precision_propagators(
    registry: &ConstraintRegistry,
    step_size: f64,
) -> Vec<PrecisionBoundaryPropagator> {
    let mut variable_set = HashSet::new();
    let mut propagators = Vec::new();
    
    // Find all variables involved in floating-point constraints
    for constraint_id in 0..registry.constraint_count() {
        if let Some(metadata) = registry.get_constraint(crate::optimization::constraint_metadata::ConstraintId(constraint_id)) {
            // Check if this constraint involves floating-point operations
            if matches!(
                metadata.constraint_type,
                crate::optimization::constraint_metadata::ConstraintType::LessThan |
                crate::optimization::constraint_metadata::ConstraintType::LessThanOrEquals |
                crate::optimization::constraint_metadata::ConstraintType::GreaterThan |
                crate::optimization::constraint_metadata::ConstraintType::GreaterThanOrEquals
            ) {
                for &var_id in &metadata.variables {
                    variable_set.insert(var_id);
                }
            }
        }
    }
    
    // Create a propagator for each variable involved in floating-point constraints
    for var_id in variable_set {
        propagators.push(PrecisionBoundaryPropagator::for_variable(var_id, step_size));
    }
    
    propagators
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_precision_boundary_propagator() {
        // Test that propagator initializes correctly
        let var_id = VarId::from_index(0); // Use proper constructor
        let _propagator = PrecisionBoundaryPropagator::for_variable(var_id, 1e-10);
        
        // Test that propagator initializes correctly
        // Note: Full context testing would require more complex setup
    }

    #[test]
    fn test_boundary_detection() {
        let var_id = VarId::from_index(0);
        let propagator = PrecisionBoundaryPropagator::for_variable(var_id, 1e-10);
        
        // Test boundary detection heuristic
        assert!(propagator.looks_like_constraint_boundary(5.5)); // Should detect this
        assert!(propagator.looks_like_constraint_boundary(10.0)); // Should detect this
        assert!(!propagator.looks_like_constraint_boundary(5.5000001)); // Should not detect this
    }
}
