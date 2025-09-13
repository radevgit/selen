//! Test suite for Step 6.3: Subproblem Solving Strategies
//!
//! This module tests the specialized solving strategies for partitioned subproblems.
//! Tests cover float subproblem solving, integer subproblem solving, and coordination.

#[cfg(test)]
mod tests {
    use super::super::subproblem_solving::*;
    use super::super::variable_partitioning::*;
    use crate::model::Model;
    use crate::vars::VarId;
    use std::time::Duration;
    
    fn create_float_model() -> (Model, Vec<VarId>) {
        let mut model = Model::with_float_precision(3);
        
        // Add some float variables
        let var0 = m.float(-10.0, 10.0);
        let var1 = m.float(0.0, 100.0);
        let var2 = m.float(-5.0, 5.0);
        
        (model, vec![var0, var1, var2])
    }
    
    fn create_integer_model() -> (Model, Vec<VarId>) {
        let mut model = Model::with_float_precision(3);
        
        // Add some integer variables
        let var0 = m.int(1, 10);
        let var1 = m.int(-5, 5);
        let var2 = m.int(0, 100);
        
        (model, vec![var0, var1, var2])
    }
    
    fn create_mixed_model() -> (Model, Vec<VarId>) {
        let mut model = Model::with_float_precision(3);
        
        // Add mixed variables
        let var0 = m.float(0.0, 10.0);  // float
        let var1 = m.int(1, 5);         // integer
        let var2 = m.float(-1.0, 1.0);  // float
        let var3 = m.int(10, 20);       // integer
        
        (model, vec![var0, var1, var2, var3])
    }
    
    #[test]
    fn test_float_solver_creation() {
        let _solver = FloatSubproblemSolver::new(3);
        // Test that creation succeeds - precision_digits field is private
        
        let _solver_with_timeout = FloatSubproblemSolver::new(2)
            .with_timeout(Duration::from_millis(500));
        // Test that timeout configuration doesn't crash
    }
    
    #[test]
    fn test_integer_solver_creation() {
        let _solver = IntegerSubproblemSolver::new();
        // Test that creation succeeds - max_depth field is private
        
        let _solver_with_config = IntegerSubproblemSolver::new()
            .with_max_depth(500)
            .with_timeout(Duration::from_millis(2000));
        // Test that configuration doesn't crash
    }
    
    #[test]
    fn test_float_subproblem_solving() {
        let (model, var_ids) = create_float_model();
        let solver = FloatSubproblemSolver::new(3);
        
        // Create a float partition
        let float_partition = VariablePartition {
            float_variables: var_ids.clone(),
            integer_variables: vec![],
            constraint_count: 0,
        };
        
        let result = solver.solve_float_subproblem(&model, &float_partition);
        assert!(result.is_ok());
        
        let solution = result.unwrap();
        assert!(solution.is_solved);
        assert_eq!(solution.variable_count, 3);
        assert_eq!(solution.variable_assignments.len(), 3);
        
        // Check that solutions are within bounds
        for (var_id, value) in &solution.variable_assignments {
            if let SubproblemValue::Float(val) = value {
                // Find which variable this is based on order
                let index = var_ids.iter().position(|v| v == var_id).unwrap();
                match index {
                    0 => assert!(*val >= -10.0 && *val <= 10.0),
                    1 => assert!(*val >= 0.0 && *val <= 100.0),
                    2 => assert!(*val >= -5.0 && *val <= 5.0),
                    _ => panic!("Unexpected variable ID"),
                }
            } else {
                panic!("Expected float value");
            }
        }
    }
    
    #[test]
    fn test_integer_subproblem_solving() {
        let (model, var_ids) = create_integer_model();
        let solver = IntegerSubproblemSolver::new();
        
        // Create an integer partition
        let integer_partition = VariablePartition {
            float_variables: vec![],
            integer_variables: var_ids.clone(),
            constraint_count: 0,
        };
        
        let result = solver.solve_integer_subproblem(&model, &integer_partition);
        assert!(result.is_ok());
        
        let solution = result.unwrap();
        assert!(solution.is_solved);
        assert_eq!(solution.variable_count, 3);
        assert_eq!(solution.variable_assignments.len(), 3);
        
        // Check that solutions are within bounds
        for (var_id, value) in &solution.variable_assignments {
            if let SubproblemValue::Integer(val) = value {
                // Find which variable this is based on order
                let index = var_ids.iter().position(|v| v == var_id).unwrap();
                match index {
                    0 => assert!(*val >= 1 && *val <= 10),
                    1 => assert!(*val >= -5 && *val <= 5),
                    2 => assert!(*val >= 0 && *val <= 100),
                    _ => panic!("Unexpected variable ID"),
                }
            } else {
                panic!("Expected integer value");
            }
        }
    }
    
    #[test]
    fn test_float_solver_empty_partition() {
        let (model, _) = create_float_model();
        let solver = FloatSubproblemSolver::new(3);
        
        // Create an empty float partition
        let empty_partition = VariablePartition {
            float_variables: vec![],
            integer_variables: vec![],
            constraint_count: 0,
        };
        
        let result = solver.solve_float_subproblem(&model, &empty_partition);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), SubproblemSolvingError::FloatSolvingFailed(FloatSolvingError::EmptyPartition)));
    }
    
    #[test]
    fn test_integer_solver_empty_partition() {
        let (model, _) = create_integer_model();
        let solver = IntegerSubproblemSolver::new();
        
        // Create an empty integer partition
        let empty_partition = VariablePartition {
            float_variables: vec![],
            integer_variables: vec![],
            constraint_count: 0,
        };
        
        let result = solver.solve_integer_subproblem(&model, &empty_partition);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), SubproblemSolvingError::IntegerSolvingFailed(IntegerSolvingError::EmptyPartition)));
    }
    
    #[test]
    fn test_coordinator_creation() {
        let _coordinator = SubproblemCoordinator::new(3);
        // We can't access private fields directly, so just test creation succeeds
        
        let _coordinator_with_timeout = SubproblemCoordinator::new(2)
            .with_global_timeout(Duration::from_millis(5000));
        // Test that timeout configuration doesn't crash
    }
    
    #[test]
    fn test_coordinator_solve_float_only_problem() {
        let (model, var_ids) = create_float_model();
        let coordinator = SubproblemCoordinator::new(3);
        
        // Create a partition result with only float variables
        let float_partition = VariablePartition {
            float_variables: var_ids,
            integer_variables: vec![],
            constraint_count: 0,
        };
        
        let partition_result = PartitionResult {
            float_partition: Some(float_partition),
            integer_partition: None,
            is_separable: true,
            total_variables: 3,
            total_constraints: 0,
        };
        
        let result = coordinator.solve_partitioned_problem(&model, &partition_result);
        assert!(result.is_ok());
        
        let combined_solution = result.unwrap();
        assert!(combined_solution.is_complete);
        assert_eq!(combined_solution.all_assignments.len(), 3);
        assert_eq!(combined_solution.subproblem_results.len(), 1);
        assert!(combined_solution.speedup_factor > 1.0);
    }
    
    #[test]
    fn test_coordinator_solve_integer_only_problem() {
        let (model, var_ids) = create_integer_model();
        let coordinator = SubproblemCoordinator::new(3);
        
        // Create a partition result with only integer variables
        let integer_partition = VariablePartition {
            float_variables: vec![],
            integer_variables: var_ids,
            constraint_count: 0,
        };
        
        let partition_result = PartitionResult {
            float_partition: None,
            integer_partition: Some(integer_partition),
            is_separable: true,
            total_variables: 3,
            total_constraints: 0,
        };
        
        let result = coordinator.solve_partitioned_problem(&model, &partition_result);
        assert!(result.is_ok());
        
        let combined_solution = result.unwrap();
        assert!(combined_solution.is_complete);
        assert_eq!(combined_solution.all_assignments.len(), 3);
        assert_eq!(combined_solution.subproblem_results.len(), 1);
        assert!(combined_solution.speedup_factor > 1.0);
    }
    
    #[test]
    fn test_coordinator_solve_mixed_separable_problem() {
        let (model, var_ids) = create_mixed_model();
        let coordinator = SubproblemCoordinator::new(3);
        
        // Create a partition result with both float and integer subproblems
        let float_partition = VariablePartition {
            float_variables: vec![var_ids[0], var_ids[2]], // vars 0 and 2 are float
            integer_variables: vec![],
            constraint_count: 0,
        };
        
        let integer_partition = VariablePartition {
            float_variables: vec![],
            integer_variables: vec![var_ids[1], var_ids[3]], // vars 1 and 3 are integer
            constraint_count: 0,
        };
        
        let partition_result = PartitionResult {
            float_partition: Some(float_partition),
            integer_partition: Some(integer_partition),
            is_separable: true,
            total_variables: 4,
            total_constraints: 0,
        };
        
        let result = coordinator.solve_partitioned_problem(&model, &partition_result);
        assert!(result.is_ok());
        
        let combined_solution = result.unwrap();
        assert!(combined_solution.is_complete);
        assert_eq!(combined_solution.all_assignments.len(), 4);
        assert_eq!(combined_solution.subproblem_results.len(), 2);
        
        // Check that we have both float and integer assignments
        let mut float_count = 0;
        let mut integer_count = 0;
        
        for value in combined_solution.all_assignments.values() {
            match value {
                SubproblemValue::Float(_) => float_count += 1,
                SubproblemValue::Integer(_) => integer_count += 1,
            }
        }
        
        assert_eq!(float_count, 2);
        assert_eq!(integer_count, 2);
        assert!(combined_solution.speedup_factor > 1.0);
    }
    
    #[test]
    fn test_coordinator_no_subproblems() {
        let (model, _) = create_mixed_model();
        let coordinator = SubproblemCoordinator::new(3);
        
        // Create a partition result with no subproblems
        let partition_result = PartitionResult {
            float_partition: None,
            integer_partition: None,
            is_separable: false,
            total_variables: 4,
            total_constraints: 0,
        };
        
        let result = coordinator.solve_partitioned_problem(&model, &partition_result);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), SubproblemSolvingError::NoSubproblems));
    }
    
    #[test]
    fn test_solve_with_partitioning_convenience_function() {
        let (model, var_ids) = create_float_model();
        
        // Create a simple partition result
        let float_partition = VariablePartition {
            float_variables: vec![var_ids[0], var_ids[1]],
            integer_variables: vec![],
            constraint_count: 0,
        };
        
        let partition_result = PartitionResult {
            float_partition: Some(float_partition),
            integer_partition: None,
            is_separable: true,
            total_variables: 2,
            total_constraints: 0,
        };
        
        let result = solve_with_partitioning(&model, &partition_result);
        assert!(result.is_ok());
        
        let combined_solution = result.unwrap();
        assert!(combined_solution.is_complete);
        assert_eq!(combined_solution.all_assignments.len(), 2);
    }
    
    #[test]
    fn test_subproblem_value_equality() {
        let float_val1 = SubproblemValue::Float(3.14);
        let float_val2 = SubproblemValue::Float(3.14);
        let float_val3 = SubproblemValue::Float(2.71);
        
        let int_val1 = SubproblemValue::Integer(42);
        let int_val2 = SubproblemValue::Integer(42);
        let int_val3 = SubproblemValue::Integer(7);
        
        assert_eq!(float_val1, float_val2);
        assert_ne!(float_val1, float_val3);
        assert_eq!(int_val1, int_val2);
        assert_ne!(int_val1, int_val3);
        assert_ne!(float_val1, int_val1);
    }
    
    #[test]
    fn test_error_display() {
        let errors = vec![
            SubproblemSolvingError::FloatSolvingFailed(FloatSolvingError::EmptyPartition),
            SubproblemSolvingError::IntegerSolvingFailed(IntegerSolvingError::EmptyPartition),
            SubproblemSolvingError::TimeoutExceeded,
            SubproblemSolvingError::CombinationFailed(CombinationError::InvalidStructure),
            SubproblemSolvingError::NoSubproblems,
        ];
        
        for error in errors {
            let error_string = format!("{}", error);
            assert!(!error_string.is_empty());
        }
    }
    
    #[test]
    fn test_performance_characteristics() {
        let (model, var_ids) = create_mixed_model();
        let coordinator = SubproblemCoordinator::new(3);
        
        // Create a larger partition to test performance
        let float_partition = VariablePartition {
            float_variables: vec![var_ids[0], var_ids[2]],
            integer_variables: vec![],
            constraint_count: 0,
        };
        
        let integer_partition = VariablePartition {
            float_variables: vec![],
            integer_variables: vec![var_ids[1], var_ids[3]],
            constraint_count: 0,
        };
        
        let partition_result = PartitionResult {
            float_partition: Some(float_partition),
            integer_partition: Some(integer_partition),
            is_separable: true,
            total_variables: 4,
            total_constraints: 0,
        };
        
        let start_time = std::time::Instant::now();
        let result = coordinator.solve_partitioned_problem(&model, &partition_result);
        let elapsed = start_time.elapsed();
        
        assert!(result.is_ok());
        let combined_solution = result.unwrap();
        
        // Performance assertions
        assert!(elapsed < Duration::from_millis(100)); // Should be very fast for small problems
        assert!(combined_solution.total_time < Duration::from_millis(50));
        assert!(combined_solution.speedup_factor >= 1.0);
        
        println!("Step 6.3 Performance:");
        println!("  Total solving time: {:?}", combined_solution.total_time);
        println!("  Estimated speedup: {:.1}x", combined_solution.speedup_factor);
        println!("  Variables solved: {}", combined_solution.all_assignments.len());
        println!("  Subproblems: {}", combined_solution.subproblem_results.len());
    }
}
