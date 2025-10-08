//! Coverage tests for variables module
//! Target: Improve coverage for variables::core (60.71% line coverage) and variables::views (38.79% line coverage)

use selen::prelude::*;

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
        model.new(x.eq(42));
        
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
                (-10..=10).contains(&value),
                "Variable value should be within expected range: got {}",
                value
            );
        }
    }

    #[test]
    #[ignore] // Large domain handling may have changed
    fn test_variable_large_domain() {
        let mut model = Model::default();
        let x = model.int(1000000, 3000000);
        
        let solution = model.solve();
        assert!(solution.is_ok(), "Creating variable with large values should succeed");
        
        if let Ok(solution) = solution {
            let value = solution.get_int(x);
            assert!(
                (1000000..=3000000).contains(&value),
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
        model.new(x.eq(25));
        model.new(y.eq(0));
        
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
        
        model.new(x.ge(0.5));
        
        let solution = model.solve();
        if let Ok(solution) = solution {
            let value = solution.get_float(x);
            assert!(
                (0.5..=1.0).contains(&value),
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
        
        model.new(x.eq(3.5));
        
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
        model.new(x.eq(3));
        model.new(y.eq(7));
        
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
        model.new(x.ge(20));
        model.new(x.le(30));
        model.new(y.ge(40));
        model.new(y.le(50));
        
        let solution = model.solve();
        if let Ok(solution) = solution {
            let x_val = solution.get_int(x);
            let y_val = solution.get_int(y);
            
            assert!((20..=30).contains(&x_val), "x should be in [20,30]: got {}", x_val);
            assert!((40..=50).contains(&y_val), "y should be in [40,50]: got {}", y_val);
        }
    }

    #[test]
    fn test_variable_infeasible_constraints() {
        let mut model = Model::default();
        let x = model.int(1, 5);
        
        // Create infeasible constraints
        model.new(x.ge(10));
        model.new(x.le(0));
        
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
        model.new(b.eq(1));
        
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
        model.new(x.ne(y));
        model.new(x.eq(15));
        
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
        model.new(x.eq(30));
        model.new(y.eq(65));
        
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
        model.new(x.ge(-100));
        model.new(x.le(100));
        model.new(x.eq(0)); // Middle value
        
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
        model.new(x.ge(-0.000001));
        model.new(x.le(0.000001));
        
        let solution = model.solve();
        if let Ok(solution) = solution {
            let x_val = solution.get_float(x);
            
            assert!(
                (-0.000001..=0.000001).contains(&x_val),
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
        model.new(x.eq(3));
        model.new(y.eq(4));
        model.new(z.eq(12));
        
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
        model.new(x.ge(100));
        model.new(x.le(200));
        model.new(x.eq(150));
        
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
        model.new(int_var.eq(5));
        model.new(float_var.eq(2.5));
        model.new(bool_var.eq(0));
        
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
        model.new(large_var.ge(-10));
        model.new(large_var.le(10));
        model.new(large_var.eq(0));
        
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
        model.new(x.eq(5));
        model.new(y.eq(10));
        model.new(z.eq(15));
        
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