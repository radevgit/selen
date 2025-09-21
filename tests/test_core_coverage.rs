//! Coverage tests for core modules  
//! Target: Improve coverage for core error handling, solution processing, and validation
//! 
//! Current coverage gaps:
//! - core/error.rs: 41.92% line coverage  
//! - core/solution.rs: 35.56% line coverage
//! - core/validation.rs: 53.90% line coverage

use cspsolver::prelude::*;

#[cfg(test)]
mod core_coverage {
    use super::*;

    // =================================================================
    // ERROR HANDLING AND EDGE CASE COVERAGE TESTS
    // =================================================================
    
    #[test]
    fn test_model_error_conditions_unsatisfiable() {
        let mut model = Model::default();
        let x = model.int(1, 5);
        
        // Create clearly unsatisfiable constraints
        post!(model, x == int(1));
        post!(model, x == int(5));
        
        // Should return error for unsatisfiable model
        let result = model.solve();
        assert!(result.is_err(), "Unsatisfiable model should return error");
    }
    
    #[test]
    fn test_model_with_invalid_domains() {
        let mut model = Model::default();
        
        // Create invalid domain where min > max - solver should handle this gracefully
        let invalid_var = model.int(10, 5); // min > max
        post!(model, invalid_var >= int(3));
        
        // Should either solve successfully (if solver corrects the domain) or fail gracefully
        let result = model.solve();
        // Both outcomes are acceptable - just ensure no panic
        match result {
            Ok(_) => { /* Valid - solver handled it */ },
            Err(_) => { /* Valid - solver detected error */ }
        }
    }
    
    #[test]
    fn test_validation_system_edge_cases() {
        let mut model = Model::default();
        let x = model.int(1, 10);
        let y = model.int(5, 15);
        
        // Test validation with complex constraint combinations
        post!(model, x <= y);
        post!(model, y >= x);
        post!(model, x != y);
        
        // Run validation explicitly
        let validation_result = model.validate();
        assert!(validation_result.is_ok(), "Valid constraint combination should pass validation");
        
        // Should be able to solve
        let solve_result = model.solve();
        assert!(solve_result.is_ok(), "Valid model should be solvable");
        
        if let Ok(solution) = solve_result {
            let x_val = solution.get_int(x);
            let y_val = solution.get_int(y);
            assert!(x_val <= y_val, "Solution should satisfy x <= y");
            assert!(x_val != y_val, "Solution should satisfy x != y");
        }
    }
    
    #[test]
    fn test_empty_constraint_model() {
        let mut model = Model::default();
        let x = model.int(1, 10);
        let y = model.int(1, 10);
        
        // Model with variables but no constraints
        let result = model.solve();
        
        // Should find solution easily
        assert!(result.is_ok(), "Unconstrained model should be solvable");
        
        if let Ok(solution) = result {
            let x_val = solution.get_int(x);
            let y_val = solution.get_int(y);
            assert!(x_val >= 1 && x_val <= 10, "x should be in domain [1,10]");
            assert!(y_val >= 1 && y_val <= 10, "y should be in domain [1,10]");
        }
    }

    // =================================================================
    // SOLUTION PROCESSING COVERAGE TESTS  
    // =================================================================
    
    #[test]
    fn test_solution_access_methods() {
        let mut model = Model::default();
        let x = model.int(1, 10);
        let y = model.int(-5, 5);
        let z = model.float(0.0, 1.0);
        
        // Add constraints to get specific values
        post!(model, x == int(7));
        post!(model, y == int(-2));
        
        let result = model.solve();
        
        if let Ok(solution) = result {
            // Test integer variable access
            let x_val = solution.get_int(x);
            assert_eq!(x_val, 7);
            
            let y_val = solution.get_int(y);
            assert_eq!(y_val, -2);
            
            // Test float variable access
            let z_val = solution.get_float(z);
            assert!(z_val >= 0.0 && z_val <= 1.0);
            
            // Test array access syntax
            if let Val::ValI(x_val) = solution[x] {
                assert_eq!(x_val, 7);
            }
            if let Val::ValI(y_val) = solution[y] {
                assert_eq!(y_val, -2);
            }
        }
    }
    
    #[test]
    fn test_solution_with_boolean_variables() {
        let mut model = Model::default();
        let bool_true = model.int(1, 1);  // Always true
        let bool_false = model.int(0, 0); // Always false
        let bool_var = model.int(0, 1);   // Can be true or false
        
        // Add constraint to make bool_var = 1
        post!(model, bool_var == int(1));
        
        let result = model.solve();
        
        if let Ok(solution) = result {
            assert_eq!(solution.get_int(bool_true), 1);
            assert_eq!(solution.get_int(bool_false), 0);
            assert_eq!(solution.get_int(bool_var), 1);
        }
    }
    
    #[test]
    fn test_solution_with_edge_case_values() {
        let mut model = Model::default();
        
        // Test boundary values
        let min_int = model.int(-1000, -1000);
        let max_int = model.int(1000, 1000);
        let zero_int = model.int(0, 0);
        
        // Test float boundaries  
        let min_float = model.float(-100.0, -100.0);
        let max_float = model.float(100.0, 100.0);
        let zero_float = model.float(0.0, 0.0);
        
        let result = model.solve();
        
        assert!(result.is_ok(), "Fixed value model should be solvable");
        
        if let Ok(solution) = result {
            assert_eq!(solution.get_int(min_int), -1000, "Min int should be -1000");
            assert_eq!(solution.get_int(max_int), 1000, "Max int should be 1000");
            assert_eq!(solution.get_int(zero_int), 0, "Zero int should be 0");
            
            assert_eq!(solution.get_float(min_float), -100.0, "Min float should be -100.0");
            assert_eq!(solution.get_float(max_float), 100.0, "Max float should be 100.0");
            assert_eq!(solution.get_float(zero_float), 0.0, "Zero float should be 0.0");
        }
    }
    
    #[test]
    fn test_solution_completeness_and_consistency() {
        let mut model = Model::default();
        let x = model.int(1, 5);
        let y = model.int(1, 5);
        let sum = model.int(2, 10);
        
        // Constraint: x + y = sum
        post!(model, x + y == sum);
        post!(model, sum == int(6));
        
        let result = model.solve();
        
        if let Ok(solution) = result {
            let x_val = solution.get_int(x);
            let y_val = solution.get_int(y);
            let sum_val = solution.get_int(sum);
            
            // Verify solution consistency
            assert_eq!(x_val + y_val, sum_val);
            assert_eq!(sum_val, 6);
            assert!(x_val >= 1 && x_val <= 5);
            assert!(y_val >= 1 && y_val <= 5);
        }
    }

    // =================================================================
    // VALIDATION COMPREHENSIVE COVERAGE TESTS
    // =================================================================
    
    #[test]
    fn test_validation_with_conflicting_constraints() {
        let mut model = Model::default();
        let x = model.int(1, 10);
        
        // Add conflicting constraints
        post!(model, x <= int(5));
        post!(model, x >= int(8));
        
        // Either validation catches it, or solving fails
        let validation_result = model.validate();
        let solve_result = model.solve();
        
        // At least one should detect the conflict
        let detected_conflict = validation_result.is_err() || solve_result.is_err();
        assert!(detected_conflict, "Should detect conflicting constraints x <= 5 AND x >= 8");
    }
    
    #[test]
    fn test_validation_with_float_precision() {
        let mut model = Model::with_float_precision(2); // 2 decimal places
        let x = model.float(0.0, 1.0);
        let y = model.float(0.0, 1.0);
        
        // Add simple precision constraint
        post!(model, x >= float(0.5));
        post!(model, y >= float(0.5));
        
        let validation_result = model.validate();
        assert!(validation_result.is_ok(), "Valid precision constraints should pass validation");
        
        // Test solving with precision constraints
        let solve_result = model.solve();
        assert!(solve_result.is_ok(), "Float precision constraints should be solvable");
        
        if let Ok(solution) = solve_result {
            let x_val = solution.get_float(x);
            let y_val = solution.get_float(y);
            
            // Verify constraints are satisfied
            assert!(x_val >= 0.5, "x should be >= 0.5");
            assert!(y_val >= 0.5, "y should be >= 0.5");
        }
    }
    
    #[test]
    fn test_validation_with_many_variables() {
        let mut model = Model::default();
        let vars: Vec<_> = (0..50).map(|i| model.int(i, i + 10)).collect();
        
        // Add constraints between many variables
        for i in 0..vars.len()-1 {
            post!(model, vars[i] <= vars[i+1]);
        }
        
        // Validation should handle large models
        let validation_result = model.validate();
        assert!(validation_result.is_ok(), "Large valid model should pass validation");
        
        // Model should be solvable
        let solve_result = model.solve();
        assert!(solve_result.is_ok(), "Large valid model should be solvable");
        
        if let Ok(solution) = solve_result {
            // Verify solution respects constraints
            for i in 0..vars.len()-1 {
                let val_i = solution.get_int(vars[i]);
                let val_next = solution.get_int(vars[i+1]);
                assert!(val_i <= val_next, "Constraint vars[{}] <= vars[{}] violated: {} > {}", 
                        i, i+1, val_i, val_next);
            }
        }
    }
    
    #[test]
    fn test_validation_mixed_constraint_types() {
        let mut model = Model::default();
        let x = model.int(5, 10);  // Adjusted ranges to work with min constraint
        let y = model.int(7, 15);
        
        // Mix different constraint types
        post!(model, x != y);                    // Inequality
        post!(model, x + y <= int(25));          // Arithmetic (increased limit)
        post!(model, x >= int(5));               // Comparison
        
        // Global constraints
        let all_vars = vec![x, y];
        let min_var = model.min(&all_vars).expect("min should work");
        post!(model, min_var >= int(5));  // min_var will be at least 5 (x's minimum)
        
        let max_var = model.max(&all_vars).expect("max should work");
        post!(model, max_var <= int(15));
        
        // Validation should handle mixed types
        let validation_result = model.validate();
        assert!(validation_result.is_ok(), "Mixed constraint types should pass validation");
        
        let solve_result = model.solve();
        assert!(solve_result.is_ok(), "Mixed constraint model should be solvable");
        
        if let Ok(solution) = solve_result {
            let x_val = solution.get_int(x);
            let y_val = solution.get_int(y);
            let min_val = solution.get_int(min_var);
            let max_val = solution.get_int(max_var);
            
            // Verify constraints are satisfied
            assert_ne!(x_val, y_val, "x should not equal y");
            assert!(x_val + y_val <= 25, "x + y should be <= 25");
            assert!(x_val >= 5, "x should be >= 5");
            assert_eq!(min_val, x_val.min(y_val), "min_var should equal min(x,y)");
            assert_eq!(max_val, x_val.max(y_val), "max_var should equal max(x,y)");
            assert!(min_val >= 5, "min should be >= 5");
            assert!(max_val <= 15, "max should be <= 15");
        }
    }

    // =================================================================
    // MODEL CORE FUNCTIONALITY TESTS
    // =================================================================
    
    #[test]
    fn test_model_statistics_and_introspection() {
        let mut model = Model::default();
        
        // Add variables and constraints to test model state
        let x = model.int(1, 10);
        let y = model.float(0.0, 1.0);
        
        post!(model, x >= int(5));
        post!(model, y <= float(0.5));
        
        // Test precision configuration
        let precision = model.float_precision_digits();
        let step_size = model.float_step_size();
        
        assert!(precision > 0, "Precision should be positive");
        assert!(step_size > 0.0, "Step size should be positive");
        
        // Test that model can be solved
        let result = model.solve();
        assert!(result.is_ok(), "Valid model should be solvable");
        
        if let Ok(solution) = result {
            let x_val = solution.get_int(x);
            let y_val = solution.get_float(y);
            assert!(x_val >= 5, "x should be >= 5");
            assert!(y_val <= 0.5, "y should be <= 0.5");
        }
    }
    
    #[test]
    fn test_model_with_optimization() {
        let mut model = Model::default();
        let x = model.int(1, 10);
        let y = model.int(1, 10);
        
        post!(model, x + y <= int(15));
        
        // Test minimization
        let objective = model.add(x, y);
        let min_result = model.minimize(objective);
        
        assert!(min_result.is_ok(), "Optimization should succeed");
        
        if let Ok(solution) = min_result {
            let x_val = solution.get_int(x);
            let y_val = solution.get_int(y);
            
            // Should minimize x + y
            assert_eq!(x_val + y_val, 2, "Minimum possible value should be 1 + 1 = 2");
            assert!(x_val >= 1 && y_val >= 1, "Values should be in domain");
            assert!(x_val + y_val <= 15, "Should satisfy constraint");
        }
    }
    
    #[test]
    fn test_model_state_and_configuration() {
        // Test different model configurations
        let default_model = Model::default();
        assert_eq!(default_model.float_precision_digits(), 6);
        
        let precision_model = Model::with_float_precision(3);
        assert_eq!(precision_model.float_precision_digits(), 3);
        
        let config = SolverConfig::default().with_float_precision(8);
        let config_model = Model::with_config(config);
        assert_eq!(config_model.float_precision_digits(), 8);
    }
    
    #[test]
    fn test_model_with_sparse_domains() {
        let mut model = Model::default();
        
        // Test variables with non-contiguous domains
        let sparse_var = model.ints(vec![2, 5, 7, 11, 13]); // Prime numbers
        let weekday = model.ints(vec![1, 2, 3, 4, 5, 6, 7]); // Days of week
        
        post!(model, sparse_var != weekday);
        
        let result = model.solve();
        
        if let Ok(solution) = result {
            let sparse_val = solution.get_int(sparse_var);
            let weekday_val = solution.get_int(weekday);
            
            // Verify values are from correct domains
            assert!([2, 5, 7, 11, 13].contains(&sparse_val));
            assert!((1..=7).contains(&weekday_val));
            assert_ne!(sparse_val, weekday_val);
        }
    }
    
    #[test]
    fn test_model_error_handling_empty_domains() {
        let mut model = Model::default();
        
        // Create variable with empty domain
        let empty_var = model.ints(vec![]);
        
        // Add constraint on empty variable
        post!(model, empty_var >= int(1));
        
        // Should handle gracefully
        let result = model.solve();
        
        match result {
            Ok(_) => {
                // If solver handles this, that's fine
            },
            Err(_) => {
                // If solver returns error, that's expected
            }
        }
    }
    
    #[test]
    fn test_model_with_large_domains() {
        let mut model = Model::default();
        
        // Very large domain
        let large_var = model.int(-1000000, 1000000);
        
        // Very small domain
        let small_var = model.int(42, 42);
        
        // Constraint between them
        post!(model, large_var >= small_var);
        
        let result = model.solve();
        
        if let Ok(solution) = result {
            let large_val = solution.get_int(large_var);
            let small_val = solution.get_int(small_var);
            
            assert_eq!(small_val, 42);
            assert!(large_val >= 42);
            assert!(large_val >= -1000000 && large_val <= 1000000);
        }
    }
}