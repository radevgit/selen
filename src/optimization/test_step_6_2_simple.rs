//! Tests for Step 6.2: Variable Partitioning (Simplified)
//!
//! This module contains simplified tests for the variable partitioning functionality.

use crate::optimization::variable_partitioning::{
    VariablePartitioner, SubproblemBuilder, PartitionError
};
use crate::model::Model;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pure_float_problem_no_partitioning() {
        // Test that pure float problems don't need partitioning
        let mut model = Model::with_float_precision(6);
        let x = m.float(0.0, 10.0);
        let y = m.float(5.0, 15.0);
        
        // Add a simple constraint
        m.new(x.le(y));
        
        let result = VariablePartitioner::partition_model(&model);
        
        // Should have only float partition, no integer partition
        assert!(result.float_partition.is_some());
        assert!(result.integer_partition.is_none());
        assert!(result.is_separable); // Pure problems are trivially separable
        assert_eq!(result.total_variables, 2);
        
        let float_partition = result.float_partition.unwrap();
        assert_eq!(float_partition.float_variables.len(), 2);
        assert_eq!(float_partition.integer_variables.len(), 0);
    }
    
    #[test]
    fn test_pure_integer_problem_no_partitioning() {
        // Test that pure integer problems don't need partitioning
        let mut model = Model::with_float_precision(6);
        let x = m.int(0, 10);
        let y = m.int(5, 15);
        
        // Add a simple constraint
        m.new(x.le(y));
        
        let result = VariablePartitioner::partition_model(&model);
        
        // Should have only integer partition, no float partition
        assert!(result.float_partition.is_none());
        assert!(result.integer_partition.is_some());
        assert!(result.is_separable); // Pure problems are trivially separable
        assert_eq!(result.total_variables, 2);
        
        let integer_partition = result.integer_partition.unwrap();
        assert_eq!(integer_partition.float_variables.len(), 0);
        assert_eq!(integer_partition.integer_variables.len(), 2);
    }
    
    #[test]
    fn test_mixed_separable_problem_partitioning() {
        // Test partitioning of a separable mixed problem
        let mut model = Model::with_float_precision(6);
        let float_x = m.float(0.0, 10.0);
        let float_y = m.float(5.0, 15.0);
        let int_a = m.int(0, 10);
        let int_b = m.int(5, 15);
        
        // Add constraints within each type (simulating separable problem)
        m.new(float_x.le(float_y));
        m.new(int_a.le(int_b));
        
        let result = VariablePartitioner::partition_model(&model);
        
        // Should have both partitions
        assert!(result.float_partition.is_some());
        assert!(result.integer_partition.is_some());
        assert_eq!(result.total_variables, 4);
        
        let float_partition = result.float_partition.unwrap();
        assert_eq!(float_partition.float_variables.len(), 2);
        assert_eq!(float_partition.integer_variables.len(), 0);
        
        let integer_partition = result.integer_partition.unwrap();
        assert_eq!(integer_partition.float_variables.len(), 0);
        assert_eq!(integer_partition.integer_variables.len(), 2);
    }
    
    #[test]
    fn test_float_subproblem_creation() {
        // Test creating a float subproblem from a partition
        let mut model = Model::with_float_precision(6);
        let float_x = m.float(0.0, 10.0);
        let float_y = m.float(5.0, 15.0);
        let _int_a = m.int(0, 10);
        
        m.new(float_x.le(float_y));
        
        let partition_result = VariablePartitioner::partition_model(&model);
        
        if let Some(float_partition) = &partition_result.float_partition {
            let subproblem_result = SubproblemBuilder::create_float_subproblem(&model, float_partition);
            
            match subproblem_result {
                Ok(subproblem) => {
                    // Count variables manually since Vars doesn't have len()
                    let mut var_count = 0;
                    for (_, var) in subproblem.get_vars().iter_with_indices() {
                        var_count += 1;
                        // Check that variables are float type
                        match var {
                            crate::variables::Var::VarF(_) => {}, // Expected
                            crate::variables::Var::VarI(_) => panic!("Float subproblem should not contain integer variables"),
                        }
                    }
                    assert_eq!(var_count, 2);
                },
                Err(e) => panic!("Float subproblem creation failed: {}", e),
            }
        } else {
            panic!("Expected float partition to exist");
        }
    }
    
    #[test]
    fn test_partition_error_handling() {
        // Test various error conditions
        let model = Model::with_float_precision(6);
        
        // Create a partition with no variables
        let partition = crate::optimization::variable_partitioning::VariablePartition {
            float_variables: vec![],
            integer_variables: vec![],
            constraint_count: 1,
        };
        
        // Should fail to create float subproblem with no float variables
        let result = SubproblemBuilder::create_float_subproblem(&model, &partition);
        assert_eq!(result.unwrap_err(), PartitionError::NoFloatVariables);
        
        // Should fail to create integer subproblem with no integer variables
        let result = SubproblemBuilder::create_integer_subproblem(&model, &partition);
        assert_eq!(result.unwrap_err(), PartitionError::NoIntegerVariables);
    }
    
    #[test]
    fn test_empty_model() {
        // Test partitioning of an empty model
        let model = Model::with_float_precision(6);
        
        let result = VariablePartitioner::partition_model(&model);
        
        // Empty model should have no partitions
        assert!(result.float_partition.is_none());
        assert!(result.integer_partition.is_none());
        assert_eq!(result.total_variables, 0);
        assert_eq!(result.total_constraints, 0);
    }
}
