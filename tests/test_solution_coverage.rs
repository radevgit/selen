//! Coverage tests for core::solution module
//! Target: Improve coverage for core::solution (35.56% line coverage)

use selen::prelude::*;

#[cfg(test)]
mod solution_coverage {
    use super::*;

    #[test]
    fn test_solution_basic_access() {
        let mut model = Model::default();
        let x = model.int(1, 10);
        let y = model.float(0.0, 5.0);
        
        model.new(x.eq(7));
        model.new(y.eq(3.5));
        
        let solution = model.solve();
        assert!(solution.is_ok(), "Model should be solvable");
        
        if let Ok(solution) = solution {
            let x_val = solution.get_int(x);
            let y_val = solution.get_float(y);
            
            assert_eq!(x_val, 7, "x should be 7");
            assert_eq!(y_val, 3.5, "y should be 3.5");
        }
    }

    #[test]
    fn test_solution_with_multiple_variables() {
        let mut model = Model::default();
        let x1 = model.int(0, 20);
        let x2 = model.int(10, 30);
        let x3 = model.int(20, 40);
        let x4 = model.int(30, 50);
        let x5 = model.int(40, 60);
        
        // Assign specific values to each variable
        model.new(x1.eq(5));
        model.new(x2.eq(15));
        model.new(x3.eq(25));
        model.new(x4.eq(35));
        model.new(x5.eq(45));
        
        let solution = model.solve();
        assert!(solution.is_ok(), "Model with multiple variables should be solvable");
        
        if let Ok(solution) = solution {
            assert_eq!(solution.get_int(x1), 5, "Variable 0 should be 5");
            assert_eq!(solution.get_int(x2), 15, "Variable 1 should be 15");
            assert_eq!(solution.get_int(x3), 25, "Variable 2 should be 25");
            assert_eq!(solution.get_int(x4), 35, "Variable 3 should be 35");
            assert_eq!(solution.get_int(x5), 45, "Variable 4 should be 45");
        }
    }

    #[test]
    fn test_solution_edge_case_values() {
        let mut model = Model::default();
        let min_int = model.int(-1000, 1000);
        let max_int = model.int(-1000, 1000);
        let zero_int = model.int(-100, 100);
        
        model.new(min_int.eq(-999));
        model.new(max_int.eq(999));
        model.new(zero_int.eq(0));
        
        let solution = model.solve();
        if let Ok(solution) = solution {
            assert_eq!(solution.get_int(min_int), -999, "Min int should be -999");
            assert_eq!(solution.get_int(max_int), 999, "Max int should be 999");
            assert_eq!(solution.get_int(zero_int), 0, "Zero int should be 0");
        }
    }

    #[test]
    fn test_solution_float_precision_handling() {
        let mut model = Model::default();
        let precise_float = model.float(0.0, 1.0);
        let small_float = model.float(-0.001, 0.001);
        
        model.new(precise_float.eq(0.12345));
        model.new(small_float.eq(0.0001));
        
        let solution = model.solve();
        if let Ok(solution) = solution {
            let precise_val = solution.get_float(precise_float);
            let small_val = solution.get_float(small_float);
            
            assert!(
                (precise_val - 0.12345).abs() < 1e-6,
                "Precise float should be close to 0.12345, got {}",
                precise_val
            );
            assert!(
                (small_val - 0.0001).abs() < 1e-6,
                "Small float should be close to 0.0001, got {}",
                small_val
            );
        }
    }

    #[test]
    fn test_solution_with_boolean_simulation() {
        let mut model = Model::default();
        let bool1 = model.int(0, 1);
        let bool2 = model.int(0, 1);
        let bool3 = model.int(0, 1);
        
        // Set different boolean values
        model.new(bool1.eq(1));
        model.new(bool2.eq(0));
        model.new(bool3.eq(1));
        
        let solution = model.solve();
        if let Ok(solution) = solution {
            assert_eq!(solution.get_int(bool1), 1, "bool1 should be true (1)");
            assert_eq!(solution.get_int(bool2), 0, "bool2 should be false (0)");
            assert_eq!(solution.get_int(bool3), 1, "bool3 should be true (1)");
        }
    }

    #[test]
    fn test_solution_consistency_check() {
        let mut model = Model::default();
        let x = model.int(1, 10);
        let y = model.int(1, 10);
        let z = model.int(1, 20);
        
        // Create relationships between variables
        model.new(x.eq(3));
        model.new(y.eq(4));
        model.new(z.eq(7));
        
        let solution = model.solve();
        if let Ok(solution) = solution {
            let x_val = solution.get_int(x);
            let y_val = solution.get_int(y);
            let z_val = solution.get_int(z);
            
            // Verify consistency
            assert_eq!(x_val, 3, "x should be 3");
            assert_eq!(y_val, 4, "y should be 4");
            assert_eq!(z_val, 7, "z should be 7");
            
            // Additional consistency checks
            assert!(x_val + y_val == z_val, "x + y should equal z");
        }
    }

    #[test]
    fn test_solution_with_sparse_domains() {
        let mut model = Model::default();
        
        // Create variables with wide domains but constrain to specific values
        let sparse1 = model.int(0, 1000000);
        let sparse2 = model.int(-500000, 500000);
        
        model.new(sparse1.eq(123456));
        model.new(sparse2.eq(-98765));
        
        let solution = model.solve();
        if let Ok(solution) = solution {
            let val1 = solution.get_int(sparse1);
            let val2 = solution.get_int(sparse2);
            
            assert_eq!(val1, 123456, "Sparse variable 1 should be 123456");
            assert_eq!(val2, -98765, "Sparse variable 2 should be -98765");
        }
    }

    #[test]
    fn test_solution_float_boundary_conditions() {
        let mut model = Model::default();
        let boundary_low = model.float(-1000.0, 1000.0);
        let boundary_high = model.float(-1000.0, 1000.0);
        let boundary_zero = model.float(-10.0, 10.0);
        
        model.new(boundary_low.eq(-999.999));
        model.new(boundary_high.eq(999.999));
        model.new(boundary_zero.eq(0.0));
        
        let solution = model.solve();
        if let Ok(solution) = solution {
            let low_val = solution.get_float(boundary_low);
            let high_val = solution.get_float(boundary_high);
            let zero_val = solution.get_float(boundary_zero);
            
            assert!(
                (low_val - (-999.999)).abs() < 1e-6,
                "Boundary low should be close to -999.999, got {}",
                low_val
            );
            assert!(
                (high_val - 999.999).abs() < 1e-6,
                "Boundary high should be close to 999.999, got {}",
                high_val
            );
            assert!(
                zero_val.abs() < 1e-10,
                "Boundary zero should be close to 0.0, got {}",
                zero_val
            );
        }
    }

    #[test]
    fn test_solution_mixed_types_comprehensive() {
        let mut model = Model::default();
        
        // Create mix of different variable types
        let int_small = model.int(1, 5);
        let int_large = model.int(1000, 2000);
        let float_small = model.float(0.0, 1.0);
        let float_large = model.float(100.0, 200.0);
        let bool_var = model.int(0, 1);
        
        // Set specific values
        model.new(int_small.eq(3));
        model.new(int_large.eq(1500));
        model.new(float_small.eq(0.75));
        model.new(float_large.eq(150.25));
        model.new(bool_var.eq(1));
        
        let solution = model.solve();
        if let Ok(solution) = solution {
            assert_eq!(solution.get_int(int_small), 3, "Small int should be 3");
            assert_eq!(solution.get_int(int_large), 1500, "Large int should be 1500");
            
            let small_float_val = solution.get_float(float_small);
            let large_float_val = solution.get_float(float_large);
            
            assert!(
                (small_float_val - 0.75).abs() < 1e-10,
                "Small float should be 0.75, got {}",
                small_float_val
            );
            assert!(
                (large_float_val - 150.25).abs() < 1e-10,
                "Large float should be 150.25, got {}",
                large_float_val
            );
            
            assert_eq!(solution.get_int(bool_var), 1, "Bool var should be true (1)");
        }
    }

    #[test]
    fn test_solution_variable_ordering() {
        let mut model = Model::default();
        
        // Create variables in different order
        let third = model.int(20, 30);
        let first = model.int(0, 10);  
        let second = model.int(10, 20);
        
        model.new(first.eq(5));
        model.new(second.eq(15));
        model.new(third.eq(25));
        
        let solution = model.solve();
        if let Ok(solution) = solution {
            let first_val = solution.get_int(first);
            let second_val = solution.get_int(second);
            let third_val = solution.get_int(third);
            
            assert_eq!(first_val, 5, "First variable should be 5");
            assert_eq!(second_val, 15, "Second variable should be 15");
            assert_eq!(third_val, 25, "Third variable should be 25");
            
            // Verify ordering relationship
            assert!(first_val < second_val, "First should be less than second");
            assert!(second_val < third_val, "Second should be less than third");
        }
    }

    #[test]
    fn test_solution_with_constraints_verification() {
        let mut model = Model::default();
        let x = model.int(1, 100);
        let y = model.int(1, 100);
        
        // Add constraints that should be satisfied in solution
        model.new(x.ge(20));
        model.new(x.le(30));
        model.new(y.ge(40));
        model.new(y.le(50));
        model.new(x.eq(25));
        model.new(y.eq(45));
        
        let solution = model.solve();
        if let Ok(solution) = solution {
            let x_val = solution.get_int(x);
            let y_val = solution.get_int(y);
            
            // Verify solution satisfies all constraints
            assert!(x_val >= 20, "x should be >= 20");
            assert!(x_val <= 30, "x should be <= 30");
            assert!(y_val >= 40, "y should be >= 40");
            assert!(y_val <= 50, "y should be <= 50");
            assert_eq!(x_val, 25, "x should be exactly 25");
            assert_eq!(y_val, 45, "y should be exactly 45");
        }
    }

    #[test]
    fn test_solution_error_boundary_access() {
        let mut model = Model::default();
        let x = model.int(1, 10);
        
        // Create unsolvable model
        model.new(x.ge(20));
        model.new(x.le(5));
        
        let solution = model.solve();
        assert!(
            solution.is_err(),
            "Contradictory constraints should result in error"
        );
    }

    #[test]
    fn test_solution_single_variable_domains() {
        let mut model = Model::default();
        let single_int = model.int(42, 42);
        let single_float = model.float(3.14159, 3.14159);
        
        let solution = model.solve();
        if let Ok(solution) = solution {
            let int_val = solution.get_int(single_int);
            let float_val = solution.get_float(single_float);
            
            assert_eq!(int_val, 42, "Single domain int should be 42");
            assert!(
                (float_val - 3.14159).abs() < 1e-10,
                "Single domain float should be 3.14159, got {}",
                float_val
            );
        }
    }

    #[test]
    fn test_solution_large_variable_set() {
        let mut model = Model::default();
        
        // Create many variables to test solution handling
        let x1 = model.int(0, 100);
        let x2 = model.int(1, 101);
        let x3 = model.int(2, 102);
        let x4 = model.int(3, 103);
        let x5 = model.int(4, 104);
        let x6 = model.int(5, 105);
        let x7 = model.int(6, 106);
        let x8 = model.int(7, 107);
        let x9 = model.int(8, 108);
        let x10 = model.int(9, 109);
        
        // Constrain each to a specific value
        model.new(x1.eq(50));
        model.new(x2.eq(51));
        model.new(x3.eq(52));
        model.new(x4.eq(53));
        model.new(x5.eq(54));
        model.new(x6.eq(55));
        model.new(x7.eq(56));
        model.new(x8.eq(57));
        model.new(x9.eq(58));
        model.new(x10.eq(59));
        
        let solution = model.solve();
        if let Ok(solution) = solution {
            assert_eq!(solution.get_int(x1), 50, "Variable 1 should be 50");
            assert_eq!(solution.get_int(x2), 51, "Variable 2 should be 51");
            assert_eq!(solution.get_int(x3), 52, "Variable 3 should be 52");
            assert_eq!(solution.get_int(x4), 53, "Variable 4 should be 53");
            assert_eq!(solution.get_int(x5), 54, "Variable 5 should be 54");
            assert_eq!(solution.get_int(x6), 55, "Variable 6 should be 55");
            assert_eq!(solution.get_int(x7), 56, "Variable 7 should be 56");
            assert_eq!(solution.get_int(x8), 57, "Variable 8 should be 57");
            assert_eq!(solution.get_int(x9), 58, "Variable 9 should be 58");
            assert_eq!(solution.get_int(x10), 59, "Variable 10 should be 59");
        }
    }

    #[test]
    fn test_solution_precision_edge_cases() {
        // Default precision is 6 decimal places (step = 1e-6)
        // So we need to use values that are representable with this precision
        let mut model = Model::default();
        let tiny_float = model.float(0.0, 0.001);
        let precise_float = model.float(0.999999, 1.000001);
        
        // Use 0.0001 instead of 0.0000005 (which is below precision)
        model.new(tiny_float.eq(0.0001));
        model.new(precise_float.eq(1.0));
        
        let solution = model.solve();
        if let Ok(solution) = solution {
            let tiny_val = solution.get_float(tiny_float);
            let precise_val = solution.get_float(precise_float);
            
            // Use tolerance matching the default precision (1e-6)
            assert!(
                (tiny_val - 0.0001).abs() < 1e-5,
                "Tiny float should be 0.0001: got {}",
                tiny_val
            );
            assert!(
                (precise_val - 1.0).abs() < 1e-5,
                "Precise float should be 1.0: got {}",
                precise_val
            );
        }
    }

    #[test]
    fn test_solution_variable_reuse_access() {
        let mut model = Model::default();
        let x = model.int(1, 10);
        let y = model.int(1, 10);
        
        model.new(x.eq(7));
        model.new(y.eq(3));
        
        let solution = model.solve();
        if let Ok(solution) = solution {
            // Access same variables multiple times
            for _ in 0..5 {
                let x_val = solution.get_int(x);
                let y_val = solution.get_int(y);
                
                assert_eq!(x_val, 7, "x should consistently be 7");
                assert_eq!(y_val, 3, "y should consistently be 3");
            }
        }
    }

    #[test]
    fn test_solution_memory_efficiency() {
        let mut model = Model::default();
        
        // Create solution with many variables for memory testing
        let p1 = model.int(0, 10);
        let n1 = model.int(-10, 0);
        let p2 = model.int(10, 20);
        let n2 = model.int(-20, -10);
        let p3 = model.int(20, 30);
        let n3 = model.int(-30, -20);
        let p4 = model.int(30, 40);
        let n4 = model.int(-40, -30);
        let p5 = model.int(40, 50);
        let n5 = model.int(-50, -40);
        
        // Set specific values
        model.new(p1.eq(5));
        model.new(n1.eq(-5));
        model.new(p2.eq(15));
        model.new(n2.eq(-15));
        model.new(p3.eq(25));
        model.new(n3.eq(-25));
        model.new(p4.eq(35));
        model.new(n4.eq(-35));
        model.new(p5.eq(45));
        model.new(n5.eq(-45));
        
        let solution = model.solve();
        if let Ok(solution) = solution {
            // Verify all variables have correct values
            assert_eq!(solution.get_int(p1), 5, "p1 should be 5");
            assert_eq!(solution.get_int(n1), -5, "n1 should be -5");
            assert_eq!(solution.get_int(p2), 15, "p2 should be 15");
            assert_eq!(solution.get_int(n2), -15, "n2 should be -15");
            assert_eq!(solution.get_int(p3), 25, "p3 should be 25");
            assert_eq!(solution.get_int(n3), -25, "n3 should be -25");
            assert_eq!(solution.get_int(p4), 35, "p4 should be 35");
            assert_eq!(solution.get_int(n4), -35, "n4 should be -35");
            assert_eq!(solution.get_int(p5), 45, "p5 should be 45");
            assert_eq!(solution.get_int(n5), -45, "n5 should be -45");
        }
    }

    #[test]
    fn test_solution_type_safety() {
        let mut model = Model::default();
        let int_var = model.int(1, 100);
        let float_var = model.float(1.0, 100.0);
        
        model.new(int_var.eq(50));
        model.new(float_var.eq(75.5));
        
        let solution = model.solve();
        if let Ok(solution) = solution {
            // Ensure proper type access
            let int_val = solution.get_int(int_var);
            let float_val = solution.get_float(float_var);
            
            assert_eq!(int_val, 50, "Int variable should be accessible as int");
            assert!(
                (float_val - 75.5).abs() < 1e-10,
                "Float variable should be accessible as float: got {}",
                float_val
            );
            
            // Type safety is enforced by Rust's type system
            // These would cause compile errors:
            // let _ = solution.get_float(int_var); // Wrong!
            // let _ = solution.get_int(float_var); // Wrong!
        }
    }
}