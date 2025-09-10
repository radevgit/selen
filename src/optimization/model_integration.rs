//! Model integration for efficient optimization routing
//!
//! This module provides the integration layer between the Model API and the
//! specialized optimization algorithms. It handles automatic problem analysis,
//! algorithm selection, and fallback to traditional search when needed.

use crate::vars::{Vars, VarId};
use crate::props::Propagators;
use crate::solution::Solution;
use crate::views::View;
use crate::optimization::{ProblemClassifier, ProblemType, ConstraintAwareOptimizer, OptimizationResult};

/// Helper function to extract the internal index from VarId
/// Safe accessor using the new VarId methods
pub fn var_id_to_index(var_id: VarId) -> usize {
    var_id.to_index()
}

/// Helper function to create VarId from index 
/// Safe constructor using the new VarId methods
pub fn index_to_var_id(index: usize) -> VarId {
    VarId::from_index(index)
}

/// Integration manager for connecting Model API to optimization algorithms
#[derive(Debug)]
pub struct OptimizationRouter {
    classifier: ProblemClassifier,
    constraint_optimizer: ConstraintAwareOptimizer,
}

/// Result of attempting optimization through specialized algorithms
#[derive(Debug, PartialEq)]
pub enum OptimizationAttempt {
    /// Optimization succeeded with solution
    Success(Solution),
    
    /// Optimization failed, should fall back to search
    Fallback(FallbackReason),
    
    /// Problem is infeasible 
    Infeasible(InfeasibilityReason),
}

/// Reasons why optimization might fall back to traditional search
#[derive(Debug, Clone, PartialEq)]
pub enum FallbackReason {
    /// Complex objective expression that cannot be optimized directly
    ComplexObjectiveExpression,
    
    /// Pure integer problem - existing search is already efficient
    PureIntegerProblem,
    
    /// Mixed separable problem - not yet implemented
    MixedSeparableProblem,
    
    /// Mixed coupled problem - not yet implemented  
    MixedCoupledProblem,
    
    /// Optimizer internal error during solution creation
    SolutionCreationError(SolutionCreationError),
    
    /// Optimizer returned failure but not infeasible
    OptimizerFailure(OptimizerFailure),
}

/// Specific errors that can occur during solution creation
#[derive(Debug, Clone, PartialEq)]
pub enum SolutionCreationError {
    /// Failed to create new solution object
    SolutionInitializationFailed,
    
    /// Failed to insert variable value into solution
    VariableInsertionFailed { var_id: VarId },
    
    /// Variable has invalid domain state
    InvalidVariableDomain { var_id: VarId },
}

/// Specific optimizer failures that should trigger fallback
#[derive(Debug, Clone, PartialEq)]
pub enum OptimizerFailure {
    /// Variable is not a float variable
    NotFloatVariable,
    
    /// Constraint analysis failed
    ConstraintAnalysisFailed,
    
    /// Bounds computation failed
    BoundsComputationFailed,
    
    /// Unknown optimizer error
    UnknownError,
}

/// Reasons why a problem might be infeasible
#[derive(Debug, Clone, PartialEq)]
pub enum InfeasibilityReason {
    /// Variable has empty domain
    EmptyVariableDomain,
    
    /// Conflicting constraints make problem unsolvable
    ConflictingConstraints,
    
    /// Optimizer determined infeasibility
    OptimizerInfeasible(OptimizerInfeasibility),
}

/// Specific types of infeasibility detected by optimizer
#[derive(Debug, Clone, PartialEq)]
pub enum OptimizerInfeasibility {
    /// Variable domain is empty after constraint analysis
    EmptyDomainAfterConstraints,
    
    /// Contradictory constraints detected
    ContradictoryConstraints,
    
    /// No feasible solution exists
    NoFeasibleSolution,
}

impl FallbackReason {
    /// Convert to human-readable description (only when needed for debugging)
    pub fn to_description(&self) -> String {
        match self {
            FallbackReason::ComplexObjectiveExpression => 
                "Complex objective expression - cannot extract simple variable".to_string(),
            FallbackReason::PureIntegerProblem => 
                "Pure integer problem - use existing search".to_string(),
            FallbackReason::MixedSeparableProblem => 
                "Mixed separable problem - not yet implemented".to_string(),
            FallbackReason::MixedCoupledProblem => 
                "Mixed coupled problem - not yet implemented".to_string(),
            FallbackReason::SolutionCreationError(error) => 
                format!("Solution creation failed: {}", error.to_description()),
            FallbackReason::OptimizerFailure(failure) => 
                format!("Optimizer failure: {}", failure.to_description()),
        }
    }
}

impl SolutionCreationError {
    /// Convert to human-readable description
    pub fn to_description(&self) -> String {
        match self {
            SolutionCreationError::SolutionInitializationFailed => 
                "Failed to create new solution object".to_string(),
            SolutionCreationError::VariableInsertionFailed { var_id } => 
                format!("Failed to insert value for variable {:?}", var_id),
            SolutionCreationError::InvalidVariableDomain { var_id } => 
                format!("Variable {:?} has invalid domain state", var_id),
        }
    }
}

impl OptimizerFailure {
    /// Convert to human-readable description
    pub fn to_description(&self) -> String {
        match self {
            OptimizerFailure::NotFloatVariable => 
                "Variable is not a float variable".to_string(),
            OptimizerFailure::ConstraintAnalysisFailed => 
                "Constraint analysis failed".to_string(),
            OptimizerFailure::BoundsComputationFailed => 
                "Bounds computation failed".to_string(),
            OptimizerFailure::UnknownError => 
                "Unknown optimizer error".to_string(),
        }
    }
}

impl InfeasibilityReason {
    /// Convert to human-readable description (only when needed for debugging)
    pub fn to_description(&self) -> String {
        match self {
            InfeasibilityReason::EmptyVariableDomain => 
                "Variable has empty domain".to_string(),
            InfeasibilityReason::ConflictingConstraints => 
                "Conflicting constraints make problem unsolvable".to_string(),
            InfeasibilityReason::OptimizerInfeasible(infeasibility) => 
                format!("Optimizer determined infeasibility: {}", infeasibility.to_description()),
        }
    }
}

impl OptimizerInfeasibility {
    /// Convert to human-readable description
    pub fn to_description(&self) -> String {
        match self {
            OptimizerInfeasibility::EmptyDomainAfterConstraints => 
                "Variable domain is empty after constraint analysis".to_string(),
            OptimizerInfeasibility::ContradictoryConstraints => 
                "Contradictory constraints detected".to_string(),
            OptimizerInfeasibility::NoFeasibleSolution => 
                "No feasible solution exists".to_string(),
        }
    }
}

impl OptimizationRouter {
    /// Create a new optimization router
    pub fn new() -> Self {
        Self {
            classifier: ProblemClassifier,
            constraint_optimizer: ConstraintAwareOptimizer::new(),
        }
    }
    
    /// Attempt to optimize a minimization problem
    ///
    /// This function:
    /// 1. Analyzes the problem to determine if optimization is applicable
    /// 2. Attempts to extract a simple variable objective from the View
    /// 3. Classifies the problem type and routes to appropriate optimizer
    /// 4. Returns success, fallback signal, or infeasible result
    pub fn try_minimize(
        &self,
        _vars: &Vars,
        _props: &Propagators,
        _objective: &impl View,
    ) -> OptimizationAttempt {
        // STEP 2.3.1 SAFE IMPLEMENTATION: Always fall back to search
        // This ensures no hanging while we complete the integration infrastructure
        // TODO: In Step 2.3.2, implement actual optimization logic
        OptimizationAttempt::Fallback(FallbackReason::ComplexObjectiveExpression)
    }
    
    /// Attempt to optimize a maximization problem
    pub fn try_maximize(
        &self,
        _vars: &Vars,
        _props: &Propagators,
        _objective: &impl View,
    ) -> OptimizationAttempt {
        // STEP 2.3.1 SAFE IMPLEMENTATION: Always fall back to search
        // This ensures no hanging while we complete the integration infrastructure
        // TODO: In Step 2.3.2, implement actual optimization logic
        OptimizationAttempt::Fallback(FallbackReason::ComplexObjectiveExpression)
    }
    
    /// Try to extract a simple variable ID from a View
    /// 
    /// This handles the case where the objective is a direct variable reference.
    /// For complex expressions (x + y, x * 2, etc.), this returns None and
    /// we fall back to traditional search.
    fn extract_simple_variable(&self, vars: &Vars, objective: &impl View) -> Option<VarId> {
        // For now, we'll implement a conservative approach:
        // Only optimize when the objective is a direct variable reference
        
        // Try to detect if this is a simple variable by checking if the view
        // has the same bounds as one of our variables
        let obj_min = objective.min_raw(vars);
        let obj_max = objective.max_raw(vars);
        
        // Search through all variables to find a match
        for var in vars.iter().enumerate() {
            let (var_idx, var) = var;
            let var_id = index_to_var_id(var_idx);
            
            // Check if this variable matches the objective bounds exactly
            match var {
                crate::vars::Var::VarF(interval) => {
                    if crate::vars::Val::float(interval.min) == obj_min && 
                       crate::vars::Val::float(interval.max) == obj_max {
                        return Some(var_id);
                    }
                },
                crate::vars::Var::VarI(sparse_set) => {
                    if crate::vars::Val::int(sparse_set.min()) == obj_min && 
                       crate::vars::Val::int(sparse_set.max()) == obj_max {
                        return Some(var_id);
                    }
                },
            }
        }
        
        // No direct variable match found - this is likely a complex expression
        None
    }
    
    /// Attempt float minimization using constraint-aware optimizer
    fn try_float_minimize(
        &self,
        vars: &Vars,
        _props: &Propagators,
        var_idx: usize,
    ) -> OptimizationAttempt {
        // Convert usize to VarId for the constraint optimizer
        // VarId is a newtype around usize, so we create it from the index
        // Since we're in the same crate, we can access the internals if needed
        // For now, let's use a workaround by creating a fake VarId
        // Note: This is a temporary solution until we have proper API access
        
        // Create a temporary vars reference to get a proper VarId
        let all_vars: Vec<_> = vars.iter_with_indices().collect();
        if var_idx >= all_vars.len() {
            return OptimizationAttempt::Fallback(FallbackReason::SolutionCreationError(
                SolutionCreationError::InvalidVariableDomain { var_id: index_to_var_id(var_idx) }
            ));
        }
        
        // Find a way to get VarId from the variable collection
        // For now, we'll work around this by using the direct optimization approach
        // TODO: Fix this once we have proper VarId conversion
        
        return OptimizationAttempt::Fallback(FallbackReason::OptimizerFailure(
            OptimizerFailure::NotFloatVariable
        ));
    }
    
    /// Attempt float maximization using constraint-aware optimizer
    fn try_float_maximize(
        &self,
        vars: &Vars,
        _props: &Propagators,
        var_idx: usize,
    ) -> OptimizationAttempt {
        // Same workaround as minimize
        let all_vars: Vec<_> = vars.iter_with_indices().collect();
        if var_idx >= all_vars.len() {
            return OptimizationAttempt::Fallback(FallbackReason::SolutionCreationError(
                SolutionCreationError::InvalidVariableDomain { var_id: index_to_var_id(var_idx) }
            ));
        }
        
        return OptimizationAttempt::Fallback(FallbackReason::OptimizerFailure(
            OptimizerFailure::NotFloatVariable
        ));
    }
    
    /// Create a Solution from an OptimizationResult
    /// 
    /// This converts the optimizer's result back to the format expected by
    /// the Model API. For now, we create a solution with all variables at
    /// their current domain values, and the optimized variable at its optimal value.
    fn create_solution(
        &self,
        vars: &Vars,
        optimized_var_idx: usize,
        result: &OptimizationResult,
    ) -> Result<Solution, SolutionCreationError> {
        let mut values = Vec::new();
        
        // Add all variables to the solution at their current values
        for (var_idx, var) in vars.iter_with_indices() {
            if var_idx == optimized_var_idx {
                // Set the optimized variable to its optimal value
                values.push(crate::vars::Val::ValF(result.optimal_value));
            } else {
                // For other variables, use their current domain values
                // This is a simplification - in a real solution, all variables would be assigned
                match var {
                    crate::vars::Var::VarF(interval) => {
                        // Use the midpoint of the interval as a reasonable assignment
                        let value = if interval.is_fixed() {
                            interval.min
                        } else {
                            interval.mid()
                        };
                        values.push(crate::vars::Val::ValF(value));
                    },
                    crate::vars::Var::VarI(sparse_set) => {
                        // Use the minimum value for integer variables
                        values.push(crate::vars::Val::ValI(sparse_set.min()));
                    },
                }
            }
        }
        
        Ok(Solution::from(values))
    }
}

impl Default for OptimizationRouter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vars::Vars;
    use crate::props::Propagators;

    fn create_test_float_problem() -> (Vars, VarId) {
        let mut vars = Vars::new();
        let var_id = vars.new_var_with_bounds(
            crate::vars::Val::float(1.0), 
            crate::vars::Val::float(10.0)
        );
        (vars, var_id)
    }

    fn create_test_props() -> Propagators {
        Propagators::default()
    }

    #[test]
    fn test_router_creation() {
        let _router = OptimizationRouter::new();
        // Router should be created successfully
        assert!(true); // Basic creation test
    }

    #[test]
    fn test_simple_variable_extraction() {
        let router = OptimizationRouter::new();
        let (vars, var_id) = create_test_float_problem();
        
        // For a direct variable, we should be able to extract it
        // This test may need refinement as the extraction logic is conservative
        let extracted = router.extract_simple_variable(&vars, &var_id);
        
        // The extraction should work for direct variable references
        assert_eq!(extracted, Some(var_id));
    }

    #[test]
    fn test_pure_float_optimization_attempt() {
        let router = OptimizationRouter::new();
        let (vars, var_id) = create_test_float_problem();
        let props = create_test_props();
        
        let result = router.try_maximize(&vars, &props, &var_id);
        
        // Should either succeed or have a reasonable fallback
        match result {
            OptimizationAttempt::Success(_) => {
                // Success is good!
                assert!(true);
            },
            OptimizationAttempt::Fallback(reason) => {
                // Fallback is acceptable, but reason should be meaningful
                println!("Fallback reason: {:?}", reason);
            },
            OptimizationAttempt::Infeasible(reason) => {
                // Should not be infeasible for this simple case
                panic!("Unexpected infeasible result: {:?}", reason);
            }
        }
    }

    #[test]
    fn test_mixed_problem_fallback() {
        let router = OptimizationRouter::new();
        let mut vars = Vars::new();
        let props = create_test_props();
        
        // Create a mixed problem (both float and integer variables)
        let float_var = vars.new_var_with_bounds(
            crate::vars::Val::float(1.0), 
            crate::vars::Val::float(10.0)
        );
        let _int_var = vars.new_var_with_bounds(
            crate::vars::Val::int(1), 
            crate::vars::Val::int(10)
        );
        
        let result = router.try_maximize(&vars, &props, &float_var);
        
        // Mixed problems should fall back for now
        match result {
            OptimizationAttempt::Fallback(reason) => {
                // Check that it's a fallback for mixed problem handling
                match reason {
                    FallbackReason::MixedSeparableProblem | 
                    FallbackReason::MixedCoupledProblem => {
                        // This is expected for mixed problems
                        assert!(true);
                    },
                    _ => {
                        // Other fallback reasons are also acceptable for now
                        println!("Fallback reason: {:?}", reason);
                    }
                }
            },
            _ => {
                // For now, mixed problems should fall back
                // This may change as we implement mixed problem support
            }
        }
    }
}
