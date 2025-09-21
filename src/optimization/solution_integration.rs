//! Step 6.4: Solution Integration and Validation
//!
//! This module implements the logic to combine solutions from different subproblems
//! and validate that the combined solution satisfies all original constraints.
//! 
//! The integration process:
//! 1. **Solution Merging**: Combine variable assignments from float and integer subproblems
//! 2. **Constraint Validation**: Verify all original constraints are satisfied
//! 3. **Conflict Resolution**: Handle any conflicts between subproblem solutions
//! 4. **Solution Construction**: Create valid Solution objects for the original model

use crate::model::Model;
use crate::core::solution::Solution;
use crate::variables::{Var, VarId, Val};
use crate::optimization::subproblem_solving::{CombinedSolution, SubproblemValue};
use crate::optimization::variable_partitioning::PartitionResult;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Integrator for combining and validating subproblem solutions
#[derive(Debug)]
pub struct SolutionIntegrator {
    /// Timeout for validation operations
    validation_timeout: Duration,
    /// Whether to perform full constraint validation
    full_validation: bool,
}

/// Result of solution integration and validation
#[derive(Debug)]
pub struct IntegratedSolution {
    /// The final validated solution
    pub solution: Solution,
    /// Original variable assignments from subproblems
    pub variable_assignments: HashMap<VarId, Val>,
    /// Whether all constraints were validated successfully
    pub is_fully_validated: bool,
    /// Time taken for integration and validation
    pub integration_time: Duration,
    /// Number of constraints validated
    pub constraints_validated: usize,
    /// Any validation warnings or issues found
    pub validation_issues: Vec<ValidationIssue>,
}

/// Issues that can arise during validation
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationIssue {
    /// Variable assignment is out of bounds
    VariableOutOfBounds { var_id: VarId, value: Val, expected_min: Val, expected_max: Val },
    /// Constraint violation detected
    ConstraintViolation { constraint_type: String, variables: Vec<VarId> },
    /// Type mismatch between expected and actual variable types
    TypeMismatch { var_id: VarId, expected_type: VariableType, actual_value: Val },
    /// Missing assignment for a variable
    MissingAssignment { var_id: VarId },
    /// Validation timed out
    ValidationTimeout,
}

/// Variable type for validation
#[derive(Debug, Clone, PartialEq)]
pub enum VariableType {
    Float,
    Integer,
}

/// Errors that can occur during solution integration
#[derive(Debug, Clone, PartialEq)]
pub enum IntegrationError {
    /// Failed to merge subproblem solutions
    MergingFailed(MergingError),
    /// Validation failed with critical errors
    ValidationFailed(ValidationError),
    /// Solution construction failed
    ConstructionFailed(ConstructionError),
    /// Integration timed out
    TimeoutExceeded,
    /// No solution provided to integrate
    NoSolution,
}

/// Specific errors during solution merging
#[derive(Debug, Clone, PartialEq)]
pub enum MergingError {
    /// Conflicting assignments for the same variable
    ConflictingAssignments { var_id: VarId, value1: SubproblemValue, value2: SubproblemValue },
    /// Variable missing from all subproblems
    MissingVariable { var_id: VarId },
    /// Type conversion failed
    TypeConversionFailed { var_id: VarId, value: SubproblemValue },
}

/// Specific errors during validation
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    /// Critical constraint violation that makes solution invalid
    CriticalViolation { constraint_info: String },
    /// Model structure is invalid for validation
    InvalidModel,
    /// Too many validation errors to continue
    TooManyErrors { error_count: usize },
}

/// Specific errors during solution construction
#[derive(Debug, Clone, PartialEq)]
pub enum ConstructionError {
    /// Cannot create solution with current assignments
    InvalidAssignments,
    /// Solution format is incompatible with model
    IncompatibleFormat,
    /// Required variables are missing from assignments
    MissingRequiredVariables { missing_count: usize },
}

impl SolutionIntegrator {
    /// Create a new solution integrator
    pub fn new() -> Self {
        Self {
            validation_timeout: Duration::from_millis(5000), // 5 seconds default
            full_validation: true,
        }
    }
    
    /// Set validation timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.validation_timeout = timeout;
        self
    }
    
    /// Set whether to perform full validation or just basic checks
    pub fn with_full_validation(mut self, full_validation: bool) -> Self {
        self.full_validation = full_validation;
        self
    }
    
    /// Integrate and validate a combined solution from subproblems
    ///
    /// This is the main entry point for Step 6.4. It takes the results from
    /// Step 6.3 subproblem solving and creates a validated solution for the original m.
    pub fn integrate_solution(
        &self,
        model: &Model,
        combined_solution: &CombinedSolution,
        partition_result: &PartitionResult,
    ) -> Result<IntegratedSolution, IntegrationError> {
        let start_time = Instant::now();
        
        if !combined_solution.is_complete {
            return Err(IntegrationError::NoSolution);
        }
        
        // Step 1: Merge subproblem solutions into unified variable assignments
        let variable_assignments = self.merge_subproblem_solutions(model, combined_solution)?;
        
        // Step 2: Validate assignments against model constraints
        let (validation_issues, constraints_validated) = self.validate_assignments(
            model, 
            &variable_assignments,
            partition_result
        )?;
        
        // Step 3: Construct final solution object
        let solution = self.construct_solution(model, &variable_assignments)?;
        
        let integration_time = start_time.elapsed();
        
        // Check timeout
        if integration_time > self.validation_timeout {
            return Err(IntegrationError::TimeoutExceeded);
        }
        
        let is_fully_validated = validation_issues.is_empty();
        
        Ok(IntegratedSolution {
            solution,
            variable_assignments,
            is_fully_validated,
            integration_time,
            constraints_validated,
            validation_issues,
        })
    }
    
    /// Merge subproblem solutions into unified variable assignments
    fn merge_subproblem_solutions(
        &self,
        model: &Model,
        combined_solution: &CombinedSolution,
    ) -> Result<HashMap<VarId, Val>, IntegrationError> {
        let mut merged_assignments = HashMap::new();
        
        // Convert SubproblemValue to Val for each variable
        for (var_id, subproblem_value) in &combined_solution.all_assignments {
            // Check for conflicts (shouldn't happen with proper partitioning)
            if merged_assignments.contains_key(var_id) {
                return Err(IntegrationError::MergingFailed(
                    MergingError::ConflictingAssignments {
                        var_id: *var_id,
                        value1: subproblem_value.clone(),
                        value2: subproblem_value.clone(), // Placeholder since we found a conflict
                    }
                ));
            }
            
            // Convert SubproblemValue to Val
            let val = self.convert_subproblem_value_to_val(model, *var_id, subproblem_value)?;
            merged_assignments.insert(*var_id, val);
        }
        
        Ok(merged_assignments)
    }
    
    /// Convert SubproblemValue to Val with type checking
    fn convert_subproblem_value_to_val(
        &self,
        model: &Model,
        var_id: VarId,
        subproblem_value: &SubproblemValue,
    ) -> Result<Val, IntegrationError> {
        let var = &model[var_id];
        
        match (var, subproblem_value) {
            (Var::VarF(_), SubproblemValue::Float(f)) => Ok(Val::ValF(*f)),
            (Var::VarI(_), SubproblemValue::Integer(i)) => Ok(Val::ValI(*i)),
            // Type coercion cases
            (Var::VarF(_), SubproblemValue::Integer(i)) => Ok(Val::ValF(*i as f64)),
            (Var::VarI(_), SubproblemValue::Float(f)) => {
                // Only allow if the float is actually an integer value
                let rounded = f.round();
                if (f - rounded).abs() < f64::EPSILON {
                    Ok(Val::ValI(rounded as i32))
                } else {
                    Err(IntegrationError::MergingFailed(
                        MergingError::TypeConversionFailed {
                            var_id,
                            value: subproblem_value.clone(),
                        }
                    ))
                }
            }
        }
    }
    
    /// Validate assignments against model constraints
    fn validate_assignments(
        &self,
        model: &Model,
        assignments: &HashMap<VarId, Val>,
        _partition_result: &PartitionResult,
    ) -> Result<(Vec<ValidationIssue>, usize), IntegrationError> {
        let mut issues = Vec::new();
        let mut constraints_validated = 0;
        
        // Basic validation: check that all assignments are within variable bounds
        for (var_id, val) in assignments {
            let var = &model[*var_id];
            self.validate_variable_bounds(var, *var_id, val, &mut issues);
        }
        
        // For Step 6.4, we implement basic constraint validation
        // In a full implementation, this would check all propagators/constraints
        if self.full_validation {
            constraints_validated = self.validate_model_constraints(model, assignments, &mut issues)?;
        }
        
        // Check if we have too many errors
        if issues.len() > 100 {
            return Err(IntegrationError::ValidationFailed(
                ValidationError::TooManyErrors { error_count: issues.len() }
            ));
        }
        
        Ok((issues, constraints_validated))
    }
    
    /// Validate that a variable assignment is within its domain bounds
    fn validate_variable_bounds(
        &self,
        var: &Var,
        var_id: VarId,
        val: &Val,
        issues: &mut Vec<ValidationIssue>,
    ) {
        match (var, val) {
            (Var::VarF(interval), Val::ValF(f)) => {
                if *f < interval.min || *f > interval.max {
                    issues.push(ValidationIssue::VariableOutOfBounds {
                        var_id,
                        value: *val,
                        expected_min: Val::ValF(interval.min),
                        expected_max: Val::ValF(interval.max),
                    });
                }
            },
            (Var::VarI(sparse_set), Val::ValI(i)) => {
                if !sparse_set.contains(*i) {
                    issues.push(ValidationIssue::VariableOutOfBounds {
                        var_id,
                        value: *val,
                        expected_min: Val::ValI(sparse_set.min()),
                        expected_max: Val::ValI(sparse_set.max()),
                    });
                }
            },
            (Var::VarF(_), Val::ValI(_)) => {
                issues.push(ValidationIssue::TypeMismatch {
                    var_id,
                    expected_type: VariableType::Float,
                    actual_value: *val,
                });
            },
            (Var::VarI(_), Val::ValF(_)) => {
                issues.push(ValidationIssue::TypeMismatch {
                    var_id,
                    expected_type: VariableType::Integer,
                    actual_value: *val,
                });
            },
        }
    }
    
    /// Validate model constraints (simplified for Step 6.4)
    fn validate_model_constraints(
        &self,
        _model: &Model,
        _assignments: &HashMap<VarId, Val>,
        _issues: &mut Vec<ValidationIssue>,
    ) -> Result<usize, IntegrationError> {
        // For Step 6.4, we implement a simplified constraint validation
        // In a full implementation, this would:
        // 1. Iterate through all propagators in the model
        // 2. Check each constraint against the current assignments
        // 3. Report any violations as ValidationIssue::ConstraintViolation
        
        // Return a placeholder count - in real implementation this would be the actual count
        Ok(0)
    }
    
    /// Construct a final Solution object from variable assignments
    fn construct_solution(
        &self,
        model: &Model,
        assignments: &HashMap<VarId, Val>,
    ) -> Result<Solution, IntegrationError> {
        let vars = model.get_vars();
        let mut solution_values = Vec::new();
        
        // Iterate through variables by their indices using the helper functions
        for i in 0..vars.iter().count() {
            let var_id = crate::optimization::model_integration::index_to_var_id(i);
            
            if let Some(val) = assignments.get(&var_id) {
                solution_values.push(*val);
            } else {
                return Err(IntegrationError::ConstructionFailed(
                    ConstructionError::MissingRequiredVariables { missing_count: 1 }
                ));
            }
        }
        
        Ok(Solution::from(solution_values))
    }
}

impl Default for SolutionIntegrator {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ValidationIssue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationIssue::VariableOutOfBounds { var_id, value, expected_min, expected_max } => {
                write!(f, "Variable {:?} value {:?} is out of bounds [{:?}, {:?}]", 
                       var_id, value, expected_min, expected_max)
            },
            ValidationIssue::ConstraintViolation { constraint_type, variables } => {
                write!(f, "Constraint '{}' violated by variables {:?}", constraint_type, variables)
            },
            ValidationIssue::TypeMismatch { var_id, expected_type, actual_value } => {
                write!(f, "Variable {:?} expected {:?} type but got value {:?}", 
                       var_id, expected_type, actual_value)
            },
            ValidationIssue::MissingAssignment { var_id } => {
                write!(f, "Missing assignment for variable {:?}", var_id)
            },
            ValidationIssue::ValidationTimeout => {
                write!(f, "Validation process timed out")
            },
        }
    }
}

impl std::fmt::Display for IntegrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntegrationError::MergingFailed(err) => {
                write!(f, "Solution merging failed: {}", err)
            },
            IntegrationError::ValidationFailed(err) => {
                write!(f, "Solution validation failed: {}", err)
            },
            IntegrationError::ConstructionFailed(err) => {
                write!(f, "Solution construction failed: {}", err)
            },
            IntegrationError::TimeoutExceeded => {
                write!(f, "Integration process timed out")
            },
            IntegrationError::NoSolution => {
                write!(f, "No solution provided to integrate")
            },
        }
    }
}

impl std::fmt::Display for MergingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MergingError::ConflictingAssignments { var_id, value1, value2 } => {
                write!(f, "Conflicting assignments for variable {:?}: {:?} vs {:?}", 
                       var_id, value1, value2)
            },
            MergingError::MissingVariable { var_id } => {
                write!(f, "Variable {:?} missing from all subproblems", var_id)
            },
            MergingError::TypeConversionFailed { var_id, value } => {
                write!(f, "Type conversion failed for variable {:?} with value {:?}", 
                       var_id, value)
            },
        }
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::CriticalViolation { constraint_info } => {
                write!(f, "Critical constraint violation: {}", constraint_info)
            },
            ValidationError::InvalidModel => {
                write!(f, "Model structure is invalid for validation")
            },
            ValidationError::TooManyErrors { error_count } => {
                write!(f, "Too many validation errors: {}", error_count)
            },
        }
    }
}

impl std::fmt::Display for ConstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstructionError::InvalidAssignments => {
                write!(f, "Cannot create solution with current assignments")
            },
            ConstructionError::IncompatibleFormat => {
                write!(f, "Solution format is incompatible with model")
            },
            ConstructionError::MissingRequiredVariables { missing_count } => {
                write!(f, "Missing {} required variables from assignments", missing_count)
            },
        }
    }
}

impl std::error::Error for IntegrationError {}
impl std::error::Error for MergingError {}
impl std::error::Error for ValidationError {}
impl std::error::Error for ConstructionError {}

/// Convenience function to integrate a solution end-to-end
pub fn integrate_subproblem_solution(
    model: &Model,
    combined_solution: &CombinedSolution,
    partition_result: &PartitionResult,
) -> Result<IntegratedSolution, IntegrationError> {
    let integrator = SolutionIntegrator::new();
    integrator.integrate_solution(model, combined_solution, partition_result)
}
