//! Direct analytical optimization for pure float problems
//!
//! This module implements O(1) analytical solutions for floating-point optimization
//! problems with bounds constraints. Instead of using binary search enumeration,
//! we compute optimal solutions directly using mathematical analysis.
//!
//! # Key Insight
//! 
//! For pure float optimization problems like:
//! - maximize x subject to x ∈ [1.0, 10.0]
//! - minimize y subject to y ≥ 2.0, y ≤ 8.0
//!
//! The optimal solution is simply the appropriate boundary value:
//! - maximize x ∈ [a, b] → x* = b  
//! - minimize x ∈ [a, b] → x* = a
//!
//! This avoids the exponential step enumeration that causes hanging in high precision.

use crate::vars::{Vars, VarId};
use crate::domain::FloatInterval;

/// Core analytical optimizer for pure float problems
pub struct FloatBoundsOptimizer;

/// Result of direct float optimization
#[derive(Debug, Clone, PartialEq)]
pub struct OptimizationResult {
    /// The optimal value found
    pub optimal_value: f64,
    /// Whether the optimization was successful
    pub success: bool,
    /// Human-readable description of the result
    pub description: String,
}

impl OptimizationResult {
    /// Create a successful optimization result
    pub fn success(value: f64, description: String) -> Self {
        Self {
            optimal_value: value,
            success: true,
            description,
        }
    }

    /// Create a failed optimization result
    pub fn failure(description: String) -> Self {
        Self {
            optimal_value: f64::NAN,
            success: false,
            description,
        }
    }
}

impl FloatBoundsOptimizer {
    /// Create a new float bounds optimizer
    pub fn new() -> Self {
        Self
    }

    /// Directly maximize a float variable subject to its bounds
    /// 
    /// This is the core O(1) optimization operation. For a variable with
    /// bounds [min, max], the maximum value is simply max (or max - ε
    /// if the bound is exclusive).
    ///
    /// # Arguments
    /// * `vars` - Variable collection containing the target variable
    /// * `var_id` - ID of the variable to maximize
    ///
    /// # Returns
    /// An `OptimizationResult` with the optimal value or failure reason
    pub fn maximize_variable(
        &self, 
        vars: &Vars, 
        var_id: VarId
    ) -> OptimizationResult {
        match self.get_float_bounds(vars, var_id) {
            Some(interval) => {
                if interval.is_empty() {
                    return OptimizationResult::failure(
                        "Cannot maximize: variable has empty domain".to_string()
                    );
                }

                // For maximization, we want the largest possible value
                let optimal_value = interval.max;
                
                OptimizationResult::success(
                    optimal_value,
                    format!("Maximized {} to {}", var_id_to_string(var_id), optimal_value)
                )
            },
            None => OptimizationResult::failure(
                format!("Variable {} is not a float variable", var_id_to_string(var_id))
            ),
        }
    }

    /// Directly minimize a float variable subject to its bounds
    /// 
    /// For a variable with bounds [min, max], the minimum value is simply min.
    ///
    /// # Arguments
    /// * `vars` - Variable collection containing the target variable
    /// * `var_id` - ID of the variable to minimize
    ///
    /// # Returns
    /// An `OptimizationResult` with the optimal value or failure reason
    pub fn minimize_variable(
        &self,
        vars: &Vars,
        var_id: VarId
    ) -> OptimizationResult {
        match self.get_float_bounds(vars, var_id) {
            Some(interval) => {
                if interval.is_empty() {
                    return OptimizationResult::failure(
                        "Cannot minimize: variable has empty domain".to_string()
                    );
                }

                // For minimization, we want the smallest possible value
                let optimal_value = interval.min;
                
                OptimizationResult::success(
                    optimal_value,
                    format!("Minimized {} to {}", var_id_to_string(var_id), optimal_value)
                )
            },
            None => OptimizationResult::failure(
                format!("Variable {} is not a float variable", var_id_to_string(var_id))
            ),
        }
    }

    /// Check if a variable can be optimized using direct float optimization
    ///
    /// This returns true if:
    /// 1. The variable is a float variable (not integer)
    /// 2. The variable has a non-empty domain
    pub fn can_optimize(&self, vars: &Vars, var_id: VarId) -> bool {
        match self.get_float_bounds(vars, var_id) {
            Some(interval) => !interval.is_empty(),
            None => false,
        }
    }

    /// Get the current bounds of a float variable
    ///
    /// Returns None if the variable is not a float variable.
    fn get_float_bounds<'a>(&self, vars: &'a Vars, var_id: VarId) -> Option<&'a FloatInterval> {
        // Access the variable through the Vars indexing
        match &vars[var_id] {
            crate::vars::Var::VarF(interval) => Some(interval),
            crate::vars::Var::VarI(_) => None,
        }
    }

    /// Apply the optimization result by updating the variable domain
    ///
    /// This sets the variable to the single optimal value, effectively
    /// "assigning" it to the solution.
    pub fn apply_result(
        &self,
        vars: &mut Vars,
        var_id: VarId,
        result: &OptimizationResult
    ) -> Result<(), String> {
        if !result.success {
            return Err(format!("Cannot apply failed optimization: {}", result.description));
        }

        // Update the variable domain to the single optimal value
        match &mut vars[var_id] {
            crate::vars::Var::VarF(interval) => {
                // Create a new interval containing only the optimal value
                // We use a small epsilon to create a tiny interval around the optimal point
                let optimal = result.optimal_value;
                let step = interval.step;
                
                // Set the interval to contain just the optimal value
                // This is mathematically equivalent to "assigning" the variable
                *interval = FloatInterval::with_step(optimal, optimal, step);
                
                Ok(())
            },
            crate::vars::Var::VarI(_) => {
                Err(format!("Variable {} is not a float variable", var_id_to_string(var_id)))
            }
        }
    }

    /// Optimize and apply in one operation (convenience method)
    ///
    /// This is equivalent to calling maximize_variable() followed by apply_result(),
    /// but handles the error cases more cleanly.
    pub fn maximize_and_apply(
        &self,
        vars: &mut Vars,
        var_id: VarId
    ) -> OptimizationResult {
        let result = self.maximize_variable(vars, var_id);
        
        if result.success {
            match self.apply_result(vars, var_id, &result) {
                Ok(()) => result,
                Err(error) => OptimizationResult::failure(error),
            }
        } else {
            result
        }
    }

    /// Minimize and apply in one operation (convenience method)
    pub fn minimize_and_apply(
        &self,
        vars: &mut Vars,
        var_id: VarId
    ) -> OptimizationResult {
        let result = self.minimize_variable(vars, var_id);
        
        if result.success {
            match self.apply_result(vars, var_id, &result) {
                Ok(()) => result,
                Err(error) => OptimizationResult::failure(error),
            }
        } else {
            result
        }
    }
}

impl Default for FloatBoundsOptimizer {
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

    fn create_test_vars_with_float(min: f64, max: f64) -> (Vars, VarId) {
        let mut vars = Vars::new();
        let var_id = vars.new_var_with_bounds(
            crate::vars::Val::float(min), 
            crate::vars::Val::float(max)
        );
        (vars, var_id)
    }

    #[test]
    fn test_maximize_simple_bounds() {
        let optimizer = FloatBoundsOptimizer::new();
        let (vars, var_id) = create_test_vars_with_float(1.0, 10.0);

        let result = optimizer.maximize_variable(&vars, var_id);

        assert!(result.success, "Optimization should succeed");
        assert_eq!(result.optimal_value, 10.0, "Should maximize to upper bound");
        assert!(result.description.contains("Maximized"));
        assert!(result.description.contains("10"));
    }

    #[test]
    fn test_minimize_simple_bounds() {
        let optimizer = FloatBoundsOptimizer::new();
        let (vars, var_id) = create_test_vars_with_float(2.5, 8.7);

        let result = optimizer.minimize_variable(&vars, var_id);

        assert!(result.success, "Optimization should succeed");
        assert_eq!(result.optimal_value, 2.5, "Should minimize to lower bound");
        assert!(result.description.contains("Minimized"));
        assert!(result.description.contains("2.5"));
    }

    #[test]
    fn test_single_point_domain() {
        let optimizer = FloatBoundsOptimizer::new();
        let (vars, var_id) = create_test_vars_with_float(5.0, 5.0);

        let max_result = optimizer.maximize_variable(&vars, var_id);
        let min_result = optimizer.minimize_variable(&vars, var_id);

        assert!(max_result.success);
        assert!(min_result.success);
        assert_eq!(max_result.optimal_value, 5.0);
        assert_eq!(min_result.optimal_value, 5.0);
    }

    #[test]
    fn test_can_optimize_float_variable() {
        let optimizer = FloatBoundsOptimizer::new();
        let (vars, var_id) = create_test_vars_with_float(1.0, 10.0);

        assert!(optimizer.can_optimize(&vars, var_id), "Should be able to optimize float variable");
    }

    #[test]
    fn test_cannot_optimize_integer_variable() {
        let optimizer = FloatBoundsOptimizer::new();
        let mut vars = Vars::new();
        let int_var_id = vars.new_var_with_bounds(
            crate::vars::Val::int(1), 
            crate::vars::Val::int(10)
        );

        assert!(!optimizer.can_optimize(&vars, int_var_id), "Should not be able to optimize integer variable");
        
        let result = optimizer.maximize_variable(&vars, int_var_id);
        assert!(!result.success, "Should fail on integer variable");
        assert!(result.description.contains("not a float variable"));
    }

    #[test]
    fn test_apply_optimization_result() {
        let optimizer = FloatBoundsOptimizer::new();
        let (mut vars, var_id) = create_test_vars_with_float(1.0, 10.0);

        let result = optimizer.maximize_variable(&vars, var_id);
        assert!(result.success);

        let apply_result = optimizer.apply_result(&mut vars, var_id, &result);
        assert!(apply_result.is_ok(), "Should successfully apply optimization result");

        // Verify the variable is now set to the optimal value
        let final_result = optimizer.maximize_variable(&vars, var_id);
        assert!(final_result.success);
        assert_eq!(final_result.optimal_value, 10.0);
    }

    #[test]
    fn test_maximize_and_apply_convenience() {
        let optimizer = FloatBoundsOptimizer::new();
        let (mut vars, var_id) = create_test_vars_with_float(3.0, 7.0);

        let result = optimizer.maximize_and_apply(&mut vars, var_id);

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
    fn test_minimize_and_apply_convenience() {
        let optimizer = FloatBoundsOptimizer::new();
        let (mut vars, var_id) = create_test_vars_with_float(1.5, 9.5);

        let result = optimizer.minimize_and_apply(&mut vars, var_id);

        assert!(result.success, "Minimize and apply should succeed");
        assert_eq!(result.optimal_value, 1.5, "Should find correct minimum");

        // Verify the variable domain was updated
        if let crate::vars::Var::VarF(interval) = &vars[var_id] {
            assert_eq!(interval.min, 1.5);
            assert_eq!(interval.max, 1.5);
        } else {
            assert!(false, "Variable should still be float");
        }
    }

    #[test]
    fn test_precision_handling() {
        // Test that the optimizer works with different precision levels
        let optimizer = FloatBoundsOptimizer::new();
        
        // High precision case that would cause hanging with binary search
        let (vars, var_id) = create_test_vars_with_float(0.000001, 1.000000);

        let result = optimizer.maximize_variable(&vars, var_id);

        assert!(result.success, "Should handle high precision without hanging");
        assert_eq!(result.optimal_value, 1.000000, "Should find correct maximum");
        
        // This should complete instantly, not hang like the binary search approach
        let start = std::time::Instant::now();
        let _repeated_result = optimizer.maximize_variable(&vars, var_id);
        let duration = start.elapsed();
        
        assert!(duration.as_millis() < 10, "Should complete in well under 10ms");
    }
}
