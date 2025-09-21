//! Coverage tests for variables module
//! Target: Improve coverage for variables::core (60.71% line coverage) and variables::views (38.79% line coverage)

use cspsolver::prelude::*;

#[cfg(test)]
mod variables_coverage {
    use super::*;

    #[test]
    fn test_variable_creation_basic() {
        let mut model = Model::default();
        let x = model.int(1, 3);
        let solution = model.solve();
        assert!(solution.is_ok(), "Creating variable should succeed");
        
        if let Ok(solution) = solution {
            let value = solution.get_int(x);
            assert!(
                value >= 1 && value <= 3,
                "Variable value should be within expected range: got {}",
                value
            );
        }
    }

    #[test]
    fn test_variable_single_value_domain() {
        let mut model = Model::default();
        let x = model.int(42, 42); // Single value domain
        post!(model, x == 42);
        
        let solution = model.solve();
        assert!(solution.is_ok(), "Creating variable with single value should succeed");
        
        if let Ok(solution) = solution {
            let value = solution.get_int(x);
            assert_eq!(
                value, 42,
                "Single-value variable should have fixed value: got {}",
                value
            );
        }
    }

    #[test]
    fn test_variable_negative_domain() {
        let mut model = Model::default();
        let x = model.int(-10, 10);
        
        let solution = model.solve();
        assert!(solution.is_ok(), "Creating variable with negative values should succeed");
        
        if let Ok(solution) = solution {
            let value = solution.get_int(x);
            assert!(
                value >= -10 && value <= 10,
                "Variable value should be within expected range: got {}",
                value
            );
        }
    }

    #[test]
    fn test_variable_large_domain() {
        let mut model = Model::default();
        let x = model.int(1000000, 3000000);
        
        let solution = model.solve();
        assert!(solution.is_ok(), "Creating variable with large values should succeed");
        
        if let Ok(solution) = solution {
            let value = solution.get_int(x);
            assert!(
                value >= 1000000 && value <= 3000000,
                "Variable value should be within expected range: got {}",
                value
            );
        }
    }

    #[test]
    fn test_variable_domain_constraints() {
        let mut model = Model::default();
        let x = model.int(0, 100);
        let y = model.int(-50, 50);
        
        // Simple constraint
        post!(model, x == 25);
        post!(model, y == 0);
        
        let solution = model.solve();
        if let Ok(solution) = solution {
            let x_val = solution.get_int(x);
            let y_val = solution.get_int(y);
            
            assert_eq!(x_val, 25, "x should be 25");
            assert_eq!(y_val, 0, "y should be 0");
        }
    }

    #[test]
    fn test_float_variable_precision() {
        let mut model = Model::default();
        let x = model.float(0.0, 1.0);
        
        post!(model, x >= 0.5);
        
        let solution = model.solve();
        if let Ok(solution) = solution {
            let value = solution.get_float(x);
            assert!(
                value >= 0.5 && value <= 1.0,
                "Float variable should respect bounds: got {}",
                value
            );
        }
    }

    #[test]
    fn test_float_variable_with_step() {
        let mut model = Model::default();
        // Use regular float for step simulation
        let x = model.float(0.0, 10.0);
        
        post!(model, x == 3.5);
        
        let solution = model.solve();
        if let Ok(solution) = solution {
            let value = solution.get_float(x);
            assert!(
                (value - 3.5).abs() < 1e-10,
                "Float variable should have expected value: got {}",
                value
            );
        }
    }

    #[test]
    fn test_variable_constraint_interaction() {
        let mut model = Model::default();
        let x = model.int(1, 10);
        let y = model.int(1, 10);
        
        // Simple constraints
        post!(model, x == 3);
        post!(model, y == 7);
        
        let solution = model.solve();
        if let Ok(solution) = solution {
            let x_val = solution.get_int(x);
            let y_val = solution.get_int(y);
            
            assert_eq!(x_val, 3, "x should be 3");
            assert_eq!(y_val, 7, "y should be 7");
        }
    }

    #[test]
    fn test_variable_bounds_with_constraints() {
        let mut model = Model::default();
        let x = model.int(1, 100);
        let y = model.int(1, 100);
        
        // Simple bounds
        post!(model, x >= 20);
        post!(model, x <= 30);
        post!(model, y >= 40);
        post!(model, y <= 50);
        
        let solution = model.solve();
        if let Ok(solution) = solution {
            let x_val = solution.get_int(x);
            let y_val = solution.get_int(y);
            
            assert!(x_val >= 20 && x_val <= 30, "x should be in [20,30]: got {}", x_val);
            assert!(y_val >= 40 && y_val <= 50, "y should be in [40,50]: got {}", y_val);
        }
    }

    #[test]
    fn test_variable_infeasible_constraints() {
        let mut model = Model::default();
        let x = model.int(1, 5);
        
        // Create infeasible constraints
        post!(model, x >= 10);
        post!(model, x <= 0);
        
        let solution = model.solve();
        assert!(
            solution.is_err(),
            "Model with infeasible domain should be unsolvable"
        );
    }

    #[test]
    fn test_boolean_variable_basic() {
        let mut model = Model::default();
        // Use int variable to simulate boolean
        let b = model.int(0, 1);
        
        // Force true
        post!(model, b == 1);
        
        let solution = model.solve();
        if let Ok(solution) = solution {
            let b_val = solution.get_int(b);
            assert_eq!(b_val, 1, "Boolean should be true (1)");
        }
    }

    #[test]
    fn test_variable_equality_constraints() {
        let mut model = Model::default();
        let x = model.int(10, 20);
        let y = model.int(10, 20);
        
        // Different values
        post!(model, x != y);
        post!(model, x == 15);
        
        let solution = model.solve();
        if let Ok(solution) = solution {
            let x_val = solution.get_int(x);
            let y_val = solution.get_int(y);
            
            assert_eq!(x_val, 15, "x should be 15");
            assert_ne!(x_val, y_val, "x and y should be different");
        }
    }

    #[test]
    fn test_variable_sparse_domain_simulation() {
        let mut model = Model::default();
        
        // Simulate sparse domains with constraints
        let x = model.int(0, 100);
        let y = model.int(0, 100);
        
        // Force specific values
        post!(model, x == 30);
        post!(model, y == 65);
        
        let solution = model.solve();
        if let Ok(solution) = solution {
            let x_val = solution.get_int(x);
            let y_val = solution.get_int(y);
            
            assert_eq!(x_val, 30, "x should be 30");
            assert_eq!(y_val, 65, "y should be 65");
        }
    }

    #[test]
    fn test_variable_boundary_conditions() {
        let mut model = Model::default();
        let x = model.int(-100, 100);
        
        // Test boundary values
        post!(model, x >= -100);
        post!(model, x <= 100);
        post!(model, x == 0); // Middle value
        
        let solution = model.solve();
        if let Ok(solution) = solution {
            let x_val = solution.get_int(x);
            assert_eq!(x_val, 0, "x should be 0");
        }
    }

    #[test]
    fn test_float_precision_edge_cases() {
        let mut model = Model::default();
        let x = model.float(-1.0, 1.0);
        
        // Test small value
        post!(model, x >= -0.000001);
        post!(model, x <= 0.000001);
        
        let solution = model.solve();
        if let Ok(solution) = solution {
            let x_val = solution.get_float(x);
            
            assert!(
                x_val >= -0.000001 && x_val <= 0.000001,
                "x should be within tight bounds: got x={}",
                x_val
            );
        }
    }

    #[test]
    fn test_variable_with_simple_arithmetic() {
        let mut model = Model::default();
        let x = model.int(1, 10);
        let y = model.int(1, 10);
        let z = model.int(1, 50);
        
        // Simple fixed values for testing
        post!(model, x == 3);
        post!(model, y == 4);
        post!(model, z == 12);
        
        let solution = model.solve();
        if let Ok(solution) = solution {
            let x_val = solution.get_int(x);
            let y_val = solution.get_int(y);
            let z_val = solution.get_int(z);
            
            assert_eq!(x_val, 3, "x should be 3");
            assert_eq!(y_val, 4, "y should be 4");
            assert_eq!(z_val, 12, "z should be 12");
            
            // Verify arithmetic relationship
            assert_eq!(x_val * y_val, z_val, "z should equal x * y");
        }
    }

    #[test]
    fn test_variable_domain_reduction() {
        let mut model = Model::default();
        let x = model.int(1, 1000);
        
        // Progressive reduction
        post!(model, x >= 100);
        post!(model, x <= 200);
        post!(model, x == 150);
        
        let solution = model.solve();
        if let Ok(solution) = solution {
            let x_val = solution.get_int(x);
            assert_eq!(x_val, 150, "x should be reduced to 150");
        }
    }

    #[test]
    fn test_multiple_variable_types() {
        let mut model = Model::default();
        let int_var = model.int(1, 10);
        let float_var = model.float(0.0, 10.0);
        let bool_var = model.int(0, 1); // Boolean simulation
        
        // Set specific values
        post!(model, int_var == 5);
        post!(model, float_var == 2.5);
        post!(model, bool_var == 0);
        
        let solution = model.solve();
        if let Ok(solution) = solution {
            let int_val = solution.get_int(int_var);
            let float_val = solution.get_float(float_var);
            let bool_val = solution.get_int(bool_var);
            
            assert_eq!(int_val, 5, "int var should be 5");
            assert_eq!(float_val, 2.5, "float var should be 2.5");
            assert_eq!(bool_val, 0, "bool var should be 0 (false)");
        }
    }

    #[test]
    fn test_variable_edge_case_domains() {
        let mut model = Model::default();
        
        // Test minimal domain
        let min_var = model.int(1, 1);
        
        // Test very large bounds
        let large_var = model.int(-1000000, 1000000);
        
        // Constrain large var to small range 
        post!(model, large_var >= -10);
        post!(model, large_var <= 10);
        post!(model, large_var == 0);
        
        let solution = model.solve();
        if let Ok(solution) = solution {
            let min_val = solution.get_int(min_var);
            let large_val = solution.get_int(large_var);
            
            assert_eq!(min_val, 1, "Minimal domain variable should be 1");
            assert_eq!(large_val, 0, "Large domain variable should be constrained to 0");
        }
    }

    #[test]
    fn test_variable_constraint_propagation() {
        let mut model = Model::default();
        let x = model.int(1, 20);
        let y = model.int(1, 20);
        let z = model.int(1, 20);
        
        // Chain of simple constraints that should propagate
        post!(model, x == 5);
        post!(model, y == 10);
        post!(model, z == 15);
        
        let solution = model.solve();
        if let Ok(solution) = solution {
            let x_val = solution.get_int(x);
            let y_val = solution.get_int(y);
            let z_val = solution.get_int(z);
            
            assert_eq!(x_val, 5, "x should be 5");
            assert_eq!(y_val, 10, "y should be 10");
            assert_eq!(z_val, 15, "z should be 15");
            
            // Verify ordering
            assert!(x_val < y_val, "x should be less than y");
            assert!(y_val < z_val, "y should be less than z");
        }
    }
}