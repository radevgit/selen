//! Precision-Aware Constraint Boundary Optimization
//! 
//! This module addresses the precision gap in constrained optimization by using
//! constraint metadata to implement ULP-aware boundary handling for floating-point
//! constraints. It ensures that constraints like x < 5.5 properly respect
//! floating-point precision and allow values as close as possible to the boundary.

use crate::vars::{VarId, Vars};
use crate::optimization::constraint_metadata::{
    ConstraintRegistry, VariableConstraintAnalysis
};
use crate::optimization::ulp_utils::UlpUtils;
use std::collections::HashMap;

/// Precision-aware constraint optimizer
pub struct PrecisionOptimizer {
    /// Step size for floating-point domain discretization
    step_size: f64,
    /// Cache for computed precision-aware bounds
    bound_cache: HashMap<VarId, PrecisionBounds>,
}

/// Precision-aware bounds for a variable
#[derive(Debug, Clone)]
pub struct PrecisionBounds {
    /// Effective upper bound considering all constraints and precision
    pub upper_bound: Option<f64>,
    /// Effective lower bound considering all constraints and precision
    pub lower_bound: Option<f64>,
    /// Whether bounds have been adjusted for precision
    pub precision_adjusted: bool,
    /// Original constraint bounds before precision adjustment
    pub original_upper: Option<f64>,
    pub original_lower: Option<f64>,
}

impl PrecisionOptimizer {
    /// Create a new precision optimizer with the given step size
    pub fn new(step_size: f64) -> Self {
        Self {
            step_size,
            bound_cache: HashMap::new(),
        }
    }

    /// Optimize variable bounds using constraint metadata for precision-aware handling
    pub fn optimize_bounds(
        &mut self,
        var_id: VarId,
        registry: &ConstraintRegistry,
        vars: &Vars,
    ) -> Result<PrecisionBounds, String> {
        // Check cache first
        if let Some(cached_bounds) = self.bound_cache.get(&var_id) {
            return Ok(cached_bounds.clone());
        }

        // Analyze constraints for this variable
        let analysis = registry.analyze_variable_constraints(var_id);
        
        // Compute precision-aware bounds
        let bounds = self.compute_precision_bounds(var_id, &analysis, vars)?;
        
        // Cache the result
        self.bound_cache.insert(var_id, bounds.clone());
        
        Ok(bounds)
    }

    /// Compute precision-aware bounds from constraint analysis
    fn compute_precision_bounds(
        &self,
        var_id: VarId,
        analysis: &VariableConstraintAnalysis,
        vars: &Vars,
    ) -> Result<PrecisionBounds, String> {
        // Get current variable domain
        let var_domain = &vars[var_id];
        
        let (current_min, current_max) = match var_domain {
            crate::vars::Var::VarF(interval) => (interval.min, interval.max),
            crate::vars::Var::VarI(sparse_set) => {
                let min_val = sparse_set.min();
                let max_val = sparse_set.max();
                (min_val as f64, max_val as f64)
            }
        };

        // Start with current bounds
        let mut lower_bound = Some(current_min);
        let mut upper_bound = Some(current_max);
        let mut precision_adjusted = false;

        // Process constraint-derived bounds
        let constraint_upper = analysis.get_effective_upper_bound(self.step_size);
        let constraint_lower = analysis.get_effective_lower_bound(self.step_size);

        // Apply constraint bounds while considering precision
        if let Some(constraint_max) = constraint_upper {
            let precision_max = if analysis.strict_upper_bounds.iter().any(|&b| (b - constraint_max).abs() < f64::EPSILON) {
                // This is a strict upper bound (x < constraint_max)
                UlpUtils::strict_upper_bound(constraint_max)
            } else {
                // This is a non-strict upper bound (x <= constraint_max)
                constraint_max
            };
            
            upper_bound = Some(upper_bound.map_or(precision_max, |current| current.min(precision_max)));
            
            if (precision_max - constraint_max).abs() > f64::EPSILON {
                precision_adjusted = true;
            }
        }

        if let Some(constraint_min) = constraint_lower {
            let precision_min = if analysis.strict_lower_bounds.iter().any(|&b| (b - constraint_min).abs() < f64::EPSILON) {
                // This is a strict lower bound (x > constraint_min)
                UlpUtils::strict_lower_bound(constraint_min)
            } else {
                // This is a non-strict lower bound (x >= constraint_min)
                constraint_min
            };
            
            lower_bound = Some(lower_bound.map_or(precision_min, |current| current.max(precision_min)));
            
            if (precision_min - constraint_min).abs() > f64::EPSILON {
                precision_adjusted = true;
            }
        }

        // Handle equality constraints with precision
        if !analysis.equality_values.is_empty() {
            // For equality constraints, use the constraint value exactly
            let eq_value = analysis.equality_values[0]; // Take first equality value
            lower_bound = Some(eq_value);
            upper_bound = Some(eq_value);
            precision_adjusted = false; // Equality constraints are exact
        }

        Ok(PrecisionBounds {
            upper_bound,
            lower_bound,
            precision_adjusted,
            original_upper: constraint_upper,
            original_lower: constraint_lower,
        })
    }

    /// Apply precision-aware bounds to a variable's domain
    /// Note: This is a conceptual method - actual domain modification requires
    /// access to mutable variable structures which aren't available here
    pub fn get_optimized_bounds(
        &self,
        _var_id: VarId,
        bounds: &PrecisionBounds,
    ) -> Result<(Option<f64>, Option<f64>), String> {
        if bounds.lower_bound.is_none() && bounds.upper_bound.is_none() {
            return Ok((None, None)); // No changes needed
        }

        Ok((bounds.lower_bound, bounds.upper_bound))
    }

    /// Get step size used by this optimizer
    pub fn get_step_size(&self) -> f64 {
        self.step_size
    }

    /// Clear the bound cache (useful when constraints change)
    pub fn clear_cache(&mut self) {
        self.bound_cache.clear();
    }

    /// Get optimization statistics
    pub fn get_stats(&self) -> OptimizationStats {
        let precision_adjusted_count = self.bound_cache.values()
            .filter(|bounds| bounds.precision_adjusted)
            .count();
            
        OptimizationStats {
            total_variables_optimized: self.bound_cache.len(),
            precision_adjusted_variables: precision_adjusted_count,
            cache_size: self.bound_cache.len(),
        }
    }
}

/// Statistics about precision optimization
#[derive(Debug, Clone)]
pub struct OptimizationStats {
    /// Total number of variables that have been optimized
    pub total_variables_optimized: usize,
    /// Number of variables where precision adjustments were made
    pub precision_adjusted_variables: usize,
    /// Current size of the bound cache
    pub cache_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ulp_utils() {
        // Test ULP calculation
        let value = 5.5;
        let ulp = UlpUtils::ulp(value);
        assert!(ulp > 0.0);
        assert!(ulp < 1e-10); // Should be very small for this value range

        // Test strict bounds
        let strict_upper = UlpUtils::strict_upper_bound(5.5);
        assert!(strict_upper < 5.5);
        assert!(strict_upper > 5.4); // Should be very close to 5.5

        let strict_lower = UlpUtils::strict_lower_bound(5.5);
        assert!(strict_lower > 5.5);
        assert!(strict_lower < 5.6); // Should be very close to 5.5
    }

    #[test]
    fn test_precision_bounds() {
        let mut optimizer = PrecisionOptimizer::new(1e-10);
        
        // Test step size
        assert_eq!(optimizer.get_step_size(), 1e-10);
        
        // Test cache operations
        optimizer.clear_cache();
        let stats = optimizer.get_stats();
        assert_eq!(stats.cache_size, 0);
        assert_eq!(stats.total_variables_optimized, 0);
    }
}
