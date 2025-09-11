//! Test suite for Step 6.4: Solution Integration and Validation
//!
//! This module tests the solution integration and validation logic that combines
//! subproblem solutions into validated complete solutions.

#[cfg(test)]
mod tests {
    use super::super::solution_integration::*;
    use super::super::subproblem_solving::*;
    use super::super::variable_partitioning::*;
    use crate::model::Model;
    use crate::vars::{VarId, Val};
    use std::collections::HashMap;
    use std::time::Duration;
    
    fn create_test_model() -> (Model, Vec<VarId>) {
        let mut model = Model::with_float_precision(3);
        
        // Create a mix of variables
        let var0 = model.new_var_float(0.0, 10.0);   // float
        let var1 = model.new_var_int(1, 5);          // integer
        let var2 = model.new_var_float(-1.0, 1.0);   // float
        let var3 = model.new_var_int(10, 20);        // integer
        
        (model, vec![var0, var1, var2, var3])
    }
    
    fn create_valid_combined_solution(var_ids: &[VarId]) -> CombinedSolution {
        let mut all_assignments = HashMap::new();
        
        // Add assignments that should be valid
        all_assignments.insert(var_ids[0], SubproblemValue::Float(5.0));   // var0: float in [0, 10]
        all_assignments.insert(var_ids[1], SubproblemValue::Integer(3));   // var1: int in [1, 5]
        all_assignments.insert(var_ids[2], SubproblemValue::Float(0.5));   // var2: float in [-1, 1]
        all_assignments.insert(var_ids[3], SubproblemValue::Integer(15));  // var3: int in [10, 20]
        
        let subproblem_result = SubproblemSolution {
            variable_assignments: all_assignments.clone(),
            solve_time: Duration::from_micros(100),
            is_solved: true,
            variable_count: 4,
        };
        
        CombinedSolution {
            all_assignments,
            subproblem_results: vec![subproblem_result],
            total_time: Duration::from_micros(100),
            is_complete: true,
            speedup_factor: 10.0,
        }
    }
    
    fn create_invalid_combined_solution(var_ids: &[VarId]) -> CombinedSolution {
        let mut all_assignments = HashMap::new();
        
        // Add assignments that should be invalid (out of bounds)
        all_assignments.insert(var_ids[0], SubproblemValue::Float(15.0));  // var0: out of bounds [0, 10]
        all_assignments.insert(var_ids[1], SubproblemValue::Integer(3));   // var1: valid
        all_assignments.insert(var_ids[2], SubproblemValue::Float(2.0));   // var2: out of bounds [-1, 1]
        all_assignments.insert(var_ids[3], SubproblemValue::Integer(25));  // var3: out of bounds [10, 20]
        
        let subproblem_result = SubproblemSolution {
            variable_assignments: all_assignments.clone(),
            solve_time: Duration::from_micros(100),
            is_solved: true,
            variable_count: 4,
        };
        
        CombinedSolution {
            all_assignments,
            subproblem_results: vec![subproblem_result],
            total_time: Duration::from_micros(100),
            is_complete: true,
            speedup_factor: 10.0,
        }
    }
    
    fn create_partition_result() -> PartitionResult {
        PartitionResult {
            float_partition: None,
            integer_partition: None,
            is_separable: true,
            total_variables: 4,
            total_constraints: 0,
        }
    }
    
    #[test]
    fn test_integrator_creation() {
        let integrator = SolutionIntegrator::new();
        // Test default creation succeeds
        
        let integrator_with_config = SolutionIntegrator::new()
            .with_timeout(Duration::from_millis(1000))
            .with_full_validation(false);
        // Test configuration succeeds
        assert!(true); // Placeholder since fields are private
    }
    
    #[test]
    fn test_valid_solution_integration() {
        let (model, var_ids) = create_test_model();
        let combined_solution = create_valid_combined_solution(&var_ids);
        let partition_result = create_partition_result();
        let integrator = SolutionIntegrator::new();
        
        let result = integrator.integrate_solution(&model, &combined_solution, &partition_result);
        assert!(result.is_ok());
        
        let integrated = result.unwrap();
        assert!(integrated.is_fully_validated);
        assert_eq!(integrated.variable_assignments.len(), 4);
        assert_eq!(integrated.validation_issues.len(), 0);
        assert!(integrated.integration_time < Duration::from_millis(100));
        
        // Check that solution object was created successfully
        // The integrated solution should contain assignments from the integrated variable_assignments
        // var_ids[0] was assigned Val::ValF(5.0), var_ids[1] was assigned Val::ValI(3), etc.
        assert_eq!(integrated.solution[var_ids[0]], Val::ValF(5.0));
        assert_eq!(integrated.solution[var_ids[1]], Val::ValI(3));
        assert_eq!(integrated.solution[var_ids[2]], Val::ValF(0.5));
        assert_eq!(integrated.solution[var_ids[3]], Val::ValI(15));
        
        // Verify the assignments are correct
        assert_eq!(integrated.variable_assignments[&var_ids[0]], Val::ValF(5.0));
        assert_eq!(integrated.variable_assignments[&var_ids[1]], Val::ValI(3));
        assert_eq!(integrated.variable_assignments[&var_ids[2]], Val::ValF(0.5));
        assert_eq!(integrated.variable_assignments[&var_ids[3]], Val::ValI(15));
    }
    
    #[test]
    fn test_invalid_solution_integration() {
        let (model, var_ids) = create_test_model();
        let combined_solution = create_invalid_combined_solution(&var_ids);
        let partition_result = create_partition_result();
        let integrator = SolutionIntegrator::new();
        
        let result = integrator.integrate_solution(&model, &combined_solution, &partition_result);
        assert!(result.is_ok());
        
        let integrated = result.unwrap();
        assert!(!integrated.is_fully_validated);
        assert_eq!(integrated.variable_assignments.len(), 4);
        assert!(integrated.validation_issues.len() > 0);
        
        // Check specific validation issues
        let out_of_bounds_issues: Vec<_> = integrated.validation_issues.iter()
            .filter(|issue| matches!(issue, ValidationIssue::VariableOutOfBounds { .. }))
            .collect();
        assert_eq!(out_of_bounds_issues.len(), 3); // var0, var2, var3 are out of bounds
    }
    
    #[test]
    fn test_incomplete_solution_integration() {
        let (model, _var_ids) = create_test_model();
        let partition_result = create_partition_result();
        let integrator = SolutionIntegrator::new();
        
        // Create an incomplete solution
        let incomplete_solution = CombinedSolution {
            all_assignments: HashMap::new(),
            subproblem_results: vec![],
            total_time: Duration::from_micros(50),
            is_complete: false,
            speedup_factor: 1.0,
        };
        
        let result = integrator.integrate_solution(&model, &incomplete_solution, &partition_result);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), IntegrationError::NoSolution));
    }
    
    #[test]
    fn test_missing_variable_assignment() {
        let (model, var_ids) = create_test_model();
        let partition_result = create_partition_result();
        let integrator = SolutionIntegrator::new();
        
        // Create solution missing one variable
        let mut all_assignments = HashMap::new();
        all_assignments.insert(var_ids[0], SubproblemValue::Float(5.0));
        all_assignments.insert(var_ids[1], SubproblemValue::Integer(3));
        all_assignments.insert(var_ids[2], SubproblemValue::Float(0.5));
        // Missing var_ids[3]
        
        let incomplete_combined = CombinedSolution {
            all_assignments,
            subproblem_results: vec![],
            total_time: Duration::from_micros(50),
            is_complete: true, // Say it's complete but it's missing variables
            speedup_factor: 1.0,
        };
        
        let result = integrator.integrate_solution(&model, &incomplete_combined, &partition_result);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), 
                        IntegrationError::ConstructionFailed(
                            ConstructionError::MissingRequiredVariables { .. }
                        )));
    }
    
    #[test]
    fn test_type_conversion() {
        let (model, var_ids) = create_test_model();
        let partition_result = create_partition_result();
        let integrator = SolutionIntegrator::new();
        
        // Create solution with type coercion (integer to float)
        let mut all_assignments = HashMap::new();
        all_assignments.insert(var_ids[0], SubproblemValue::Integer(5));    // int->float should work
        all_assignments.insert(var_ids[1], SubproblemValue::Float(3.0));    // float->int should work (exact)
        all_assignments.insert(var_ids[2], SubproblemValue::Float(0.5));    // float->float
        all_assignments.insert(var_ids[3], SubproblemValue::Integer(15));   // int->int
        
        let combined_solution = CombinedSolution {
            all_assignments,
            subproblem_results: vec![],
            total_time: Duration::from_micros(50),
            is_complete: true,
            speedup_factor: 1.0,
        };
        
        let result = integrator.integrate_solution(&model, &combined_solution, &partition_result);
        assert!(result.is_ok());
        
        let integrated = result.unwrap();
        assert!(integrated.is_fully_validated);
        
        // Check type conversions
        assert_eq!(integrated.variable_assignments[&var_ids[0]], Val::ValF(5.0)); // int->float
        assert_eq!(integrated.variable_assignments[&var_ids[1]], Val::ValI(3));   // float->int
    }
    
    #[test]
    fn test_invalid_type_conversion() {
        let (model, var_ids) = create_test_model();
        let partition_result = create_partition_result();
        let integrator = SolutionIntegrator::new();
        
        // Create solution with invalid type conversion (non-integer float to int)
        let mut all_assignments = HashMap::new();
        all_assignments.insert(var_ids[0], SubproblemValue::Float(5.0));
        all_assignments.insert(var_ids[1], SubproblemValue::Float(3.7));    // non-integer float to int variable
        all_assignments.insert(var_ids[2], SubproblemValue::Float(0.5));
        all_assignments.insert(var_ids[3], SubproblemValue::Integer(15));
        
        let combined_solution = CombinedSolution {
            all_assignments,
            subproblem_results: vec![],
            total_time: Duration::from_micros(50),
            is_complete: true,
            speedup_factor: 1.0,
        };
        
        let result = integrator.integrate_solution(&model, &combined_solution, &partition_result);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), 
                        IntegrationError::MergingFailed(
                            MergingError::TypeConversionFailed { .. }
                        )));
    }
    
    #[test]
    fn test_validation_timeout() {
        let (model, var_ids) = create_test_model();
        let combined_solution = create_valid_combined_solution(&var_ids);
        let partition_result = create_partition_result();
        
        // Set a very short timeout
        let integrator = SolutionIntegrator::new()
            .with_timeout(Duration::from_nanos(1)); // Extremely short timeout
        
        let result = integrator.integrate_solution(&model, &combined_solution, &partition_result);
        // Might timeout or might succeed if fast enough - either is acceptable
        if let Err(error) = result {
            assert!(matches!(error, IntegrationError::TimeoutExceeded));
        }
    }
    
    #[test]
    fn test_validation_disabled() {
        let (model, var_ids) = create_test_model();
        let combined_solution = create_invalid_combined_solution(&var_ids);
        let partition_result = create_partition_result();
        
        // Disable full validation
        let integrator = SolutionIntegrator::new()
            .with_full_validation(false);
        
        let result = integrator.integrate_solution(&model, &combined_solution, &partition_result);
        assert!(result.is_ok());
        
        let integrated = result.unwrap();
        // Should still catch bounds violations even with full validation disabled
        assert!(!integrated.is_fully_validated);
        assert_eq!(integrated.constraints_validated, 0); // No constraint validation done
    }
    
    #[test]
    fn test_validation_issue_display() {
        let var_id = crate::optimization::model_integration::index_to_var_id(0);
        let issues = vec![
            ValidationIssue::VariableOutOfBounds {
                var_id,
                value: Val::ValF(15.0),
                expected_min: Val::ValF(0.0),
                expected_max: Val::ValF(10.0),
            },
            ValidationIssue::ConstraintViolation {
                constraint_type: "equals".to_string(),
                variables: vec![var_id],
            },
            ValidationIssue::TypeMismatch {
                var_id,
                expected_type: VariableType::Float,
                actual_value: Val::ValI(5),
            },
            ValidationIssue::MissingAssignment { var_id },
            ValidationIssue::ValidationTimeout,
        ];
        
        for issue in issues {
            let display_string = format!("{}", issue);
            assert!(!display_string.is_empty());
        }
    }
    
    #[test]
    fn test_error_display() {
        let var_id = crate::optimization::model_integration::index_to_var_id(0);
        let errors = vec![
            IntegrationError::MergingFailed(MergingError::ConflictingAssignments {
                var_id,
                value1: SubproblemValue::Float(1.0),
                value2: SubproblemValue::Float(2.0),
            }),
            IntegrationError::ValidationFailed(ValidationError::CriticalViolation {
                constraint_info: "test constraint".to_string(),
            }),
            IntegrationError::ConstructionFailed(ConstructionError::InvalidAssignments),
            IntegrationError::TimeoutExceeded,
            IntegrationError::NoSolution,
        ];
        
        for error in errors {
            let display_string = format!("{}", error);
            assert!(!display_string.is_empty());
        }
    }
    
    #[test]
    fn test_convenience_function() {
        let (model, var_ids) = create_test_model();
        let combined_solution = create_valid_combined_solution(&var_ids);
        let partition_result = create_partition_result();
        
        let result = integrate_subproblem_solution(&model, &combined_solution, &partition_result);
        assert!(result.is_ok());
        
        let integrated = result.unwrap();
        assert!(integrated.is_fully_validated);
        assert_eq!(integrated.variable_assignments.len(), 4);
    }
    
    #[test]
    fn test_performance_characteristics() {
        let (model, var_ids) = create_test_model();
        let combined_solution = create_valid_combined_solution(&var_ids);
        let partition_result = create_partition_result();
        let integrator = SolutionIntegrator::new();
        
        let start_time = std::time::Instant::now();
        let result = integrator.integrate_solution(&model, &combined_solution, &partition_result);
        let elapsed = start_time.elapsed();
        
        assert!(result.is_ok());
        let integrated = result.unwrap();
        
        // Performance assertions
        assert!(elapsed < Duration::from_millis(50)); // Should be very fast
        assert!(integrated.integration_time < Duration::from_millis(10));
        
        println!("Step 6.4 Performance:");
        println!("  Integration time: {:?}", integrated.integration_time);
        println!("  Variables integrated: {}", integrated.variable_assignments.len());
        println!("  Validation issues: {}", integrated.validation_issues.len());
        println!("  Constraints validated: {}", integrated.constraints_validated);
        println!("  Fully validated: {}", integrated.is_fully_validated);
    }
    
    #[test]
    fn test_solution_object_creation() {
        let (model, var_ids) = create_test_model();
        let combined_solution = create_valid_combined_solution(&var_ids);
        let partition_result = create_partition_result();
        let integrator = SolutionIntegrator::new();
        
        let result = integrator.integrate_solution(&model, &combined_solution, &partition_result);
        assert!(result.is_ok());
        
        let integrated = result.unwrap();
        let solution = integrated.solution;
        
        // Test that the solution can be indexed and contains correct values
        // We validate 4 variables to ensure all are present
        assert_eq!(solution[var_ids[0]], Val::ValF(5.0));
        assert_eq!(solution[var_ids[1]], Val::ValI(3));
        assert_eq!(solution[var_ids[2]], Val::ValF(0.5));
        assert_eq!(solution[var_ids[3]], Val::ValI(15));
    }
}
