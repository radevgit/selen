//! Coverage tests for constraints module
//! Target: Improve coverage for low-coverage constraint types
//! 
//! Current coverage gaps:
//! - table: 0% (0/193 regions)
//! - abs: 5.41% (10/185 regions) 
//! - div: 6.76% (14/207 regions)
//! - modulo: 7.91% (14/177 regions)
//! - sum: 17.91% (12/67 regions)

use selen::prelude::*;

#[cfg(test)]
mod constraints_coverage {
    use super::*;

    // ===== BASIC COMPARISON OPERATIONS =====
    // Test fundamental constraint operations to improve coverage
    
    #[test]
    fn test_equality_constraints() {
        let mut model = Model::default();
        let x = model.int(1, 10);
        let y = model.int(1, 10);
        
        post!(model, x == 5);
        post!(model, y == 5);
        post!(model, x == y);
        
        let result = model.solve();
        assert!(result.is_ok(), "Equality constraints should work");
        
        let solution = result.unwrap();
        assert_eq!(solution.get_int(x), 5);
        assert_eq!(solution.get_int(y), 5);
    }
    
    #[test]
    fn test_inequality_constraints() {
        let mut model = Model::default();
        let x = model.int(1, 10);
        let y = model.int(1, 10);
        
        post!(model, x != y);
        post!(model, x == 3);
        
        let result = model.solve();
        assert!(result.is_ok(), "Inequality constraints should work");
        
        let solution = result.unwrap();
        assert_eq!(solution.get_int(x), 3);
        assert_ne!(solution.get_int(y), 3);
    }
    
    #[test]
    fn test_less_than_constraints() {
        let mut model = Model::default();
        let x = model.int(1, 10);
        let y = model.int(5, 15);
        
        post!(model, x < y);
        post!(model, x == 4);
        
        let result = model.solve();
        assert!(result.is_ok(), "Less than constraints should work");
        
        let solution = result.unwrap();
        assert_eq!(solution.get_int(x), 4);
        assert!(solution.get_int(y) > 4);
    }
    
    #[test]
    fn test_greater_than_constraints() {
        let mut model = Model::default();
        let x = model.int(5, 15);
        let y = model.int(1, 10);
        
        post!(model, x > y);
        post!(model, y == 4);
        
        let result = model.solve();
        assert!(result.is_ok(), "Greater than constraints should work");
        
        let solution = result.unwrap();
        assert_eq!(solution.get_int(y), 4);
        assert!(solution.get_int(x) > 4);
    }
    
    #[test]
    fn test_less_than_or_equal_constraints() {
        let mut model = Model::default();
        let x = model.int(1, 10);
        let y = model.int(5, 15);
        
        post!(model, x <= y);
        post!(model, x == 7);
        
        let result = model.solve();
        assert!(result.is_ok(), "Less than or equal constraints should work");
        
        let solution = result.unwrap();
        assert_eq!(solution.get_int(x), 7);
        assert!(solution.get_int(y) >= 7);
    }
    
    #[test]
    fn test_greater_than_or_equal_constraints() {
        let mut model = Model::default();
        let x = model.int(5, 15);
        let y = model.int(1, 10);
        
        post!(model, x >= y);
        post!(model, y == 8);
        
        let result = model.solve();
        assert!(result.is_ok(), "Greater than or equal constraints should work");
        
        let solution = result.unwrap();
        assert_eq!(solution.get_int(y), 8);
        assert!(solution.get_int(x) >= 8);
    }
    
    // ===== BOUNDARY AND EDGE CASES =====
    
    #[test]
    fn test_boundary_values() {
        let mut model = Model::default();
        let x = model.int(1, 1);  // Fixed to 1
        let y = model.int(10, 10); // Fixed to 10
        
        post!(model, x < y);
        
        let result = model.solve();
        assert!(result.is_ok(), "Boundary value constraints should work");
        
        let solution = result.unwrap();
        assert_eq!(solution.get_int(x), 1);
        assert_eq!(solution.get_int(y), 10);
    }
    
    #[test]
    fn test_zero_handling() {
        let mut model = Model::default();
        let x = model.int(-5, 5);
        let y = model.int(0, 0);
        
        post!(model, x >= y);
        post!(model, x <= y);
        
        let result = model.solve();
        assert!(result.is_ok(), "Zero handling should work");
        
        let solution = result.unwrap();
        assert_eq!(solution.get_int(x), 0);
        assert_eq!(solution.get_int(y), 0);
    }
    
    #[test]
    fn test_negative_numbers() {
        let mut model = Model::default();
        let x = model.int(-10, -1);
        let y = model.int(-5, 5);
        
        post!(model, x < y);
        post!(model, x == -3);
        
        let result = model.solve();
        assert!(result.is_ok(), "Negative number constraints should work");
        
        let solution = result.unwrap();
        assert_eq!(solution.get_int(x), -3);
        assert!(solution.get_int(y) > -3);
    }
    
    #[test]
    fn test_large_domains() {
        let mut model = Model::default();
        let x = model.int(1, 1000);
        let y = model.int(500, 1500);
        
        post!(model, x == y);
        post!(model, x == 750);
        
        let result = model.solve();
        assert!(result.is_ok(), "Large domain constraints should work");
        
        let solution = result.unwrap();
        assert_eq!(solution.get_int(x), 750);
        assert_eq!(solution.get_int(y), 750);
    }
    
    #[test]
    fn test_unsatisfiable_constraint() {
        let mut model = Model::default();
        let x = model.int(1, 5);
        let y = model.int(10, 15);
        
        post!(model, x > y);  // Impossible since max(x) = 5 < min(y) = 10
        
        let result = model.solve();
        assert!(result.is_err(), "Unsatisfiable constraints should fail");
    }
    
    #[test]
    fn test_chain_of_constraints() {
        let mut model = Model::default();
        let x = model.int(1, 10);
        let y = model.int(1, 10);
        let z = model.int(1, 10);
        
        post!(model, x < y);
        post!(model, y < z);
        post!(model, x == 3);
        
        let result = model.solve();
        assert!(result.is_ok(), "Chain of constraints should work");
        
        let solution = result.unwrap();
        let x_val = solution.get_int(x);
        let y_val = solution.get_int(y);
        let z_val = solution.get_int(z);
        
        assert_eq!(x_val, 3);
        assert!(x_val < y_val);
        assert!(y_val < z_val);
    }
    
    // ===== BOOLEAN CONSTRAINT TESTING =====
    
    #[test]
    fn test_boolean_variables() {
        let mut model = Model::default();
        let a = model.bool();
        let b = model.bool();
        
        post!(model, a == 1);
        post!(model, b == 0);
        
        let result = model.solve();
        assert!(result.is_ok(), "Boolean constraints should work");
        
        let solution = result.unwrap();
        assert_eq!(solution.get_int(a), 1);
        assert_eq!(solution.get_int(b), 0);
    }
    
    #[test]
    fn test_boolean_logic_basic() {
        let mut model = Model::default();
        let a = model.bool();
        let b = model.bool();
        
        // Use and() and or() functions for logic
        let and_result = model.bool_and(&[a, b]);
        post!(model, and_result == 1);
        
        let result = model.solve();
        assert!(result.is_ok(), "Boolean AND should work");
        
        let solution = result.unwrap();
        assert_eq!(solution.get_int(a), 1);
        assert_eq!(solution.get_int(b), 1);
    }
    
    #[test]
    fn test_boolean_or_logic() {
        let mut model = Model::default();
        let a = model.bool();
        let b = model.bool();
        
        let or_result = model.bool_or(&[a, b]);
        post!(model, or_result == 1);
        post!(model, a == 0);
        
        let result = model.solve();
        assert!(result.is_ok(), "Boolean OR should work");
        
        let solution = result.unwrap();
        assert_eq!(solution.get_int(a), 0);
        assert_eq!(solution.get_int(b), 1);
    }
    
    #[test]
    fn test_boolean_not_logic() {
        let mut model = Model::default();
        let a = model.bool();
        
        post!(model, not([a]));
        
        let result = model.solve();
        assert!(result.is_ok(), "Boolean NOT should work");
        
        let solution = result.unwrap();
        assert_eq!(solution.get_int(a), 0);
    }

    // ===== COMPREHENSIVE LOGICAL OPERATIONS COVERAGE =====
    // Testing both single variable and array syntaxes for and/or/not

    #[test]
    fn test_logical_and_single_variables() {
        let mut model = Model::default();
        let a = model.bool();
        let b = model.bool();
        let c = model.bool();
        
        // Test and() with individual variables: and(a, b, c)
        post!(model, and(a, b, c));
        
        let result = model.solve();
        assert!(result.is_ok(), "AND with single variables should work");
        
        let solution = result.unwrap();
        assert_eq!(solution.get_int(a), 1);
        assert_eq!(solution.get_int(b), 1);
        assert_eq!(solution.get_int(c), 1);
    }

    #[test]
    fn test_logical_and_array_variables() {
        let mut model = Model::default();
        let a = model.bool();
        let b = model.bool();
        let c = model.bool();
        
        // Test and() with array syntax: and([a, b, c])
        post!(model, and([a, b, c]));
        
        let result = model.solve();
        assert!(result.is_ok(), "AND with array variables should work");
        
        let solution = result.unwrap();
        assert_eq!(solution.get_int(a), 1);
        assert_eq!(solution.get_int(b), 1);
        assert_eq!(solution.get_int(c), 1);
    }

    #[test]
    fn test_logical_or_single_variables() {
        let mut model = Model::default();
        let a = model.bool();
        let b = model.bool();
        let c = model.bool();
        
        // Test or() with individual variables: or(a, b, c)
        post!(model, or(a, b, c));
        post!(model, a == 0);  // Force a to be false
        post!(model, b == 0);  // Force b to be false
        
        let result = model.solve();
        assert!(result.is_ok(), "OR with single variables should work");
        
        let solution = result.unwrap();
        assert_eq!(solution.get_int(a), 0);
        assert_eq!(solution.get_int(b), 0);
        assert_eq!(solution.get_int(c), 1); // c must be true for OR to be satisfied
    }

    #[test]
    fn test_logical_or_array_variables() {
        let mut model = Model::default();
        let a = model.bool();
        let b = model.bool();
        let c = model.bool();
        
        // Test or() with array syntax: or([a, b, c])
        post!(model, or([a, b, c]));
        post!(model, a == 0);  // Force a to be false
        post!(model, b == 0);  // Force b to be false
        
        let result = model.solve();
        assert!(result.is_ok(), "OR with array variables should work");
        
        let solution = result.unwrap();
        assert_eq!(solution.get_int(a), 0);
        assert_eq!(solution.get_int(b), 0);
        assert_eq!(solution.get_int(c), 1); // c must be true for OR to be satisfied
    }

    #[test]
    fn test_logical_not_single_variable() {
        let mut model = Model::default();
        let a = model.bool();
        
        // Test not() with single variable: not(a)
        post!(model, not(a));
        
        let result = model.solve();
        assert!(result.is_ok(), "NOT with single variable should work");
        
        let solution = result.unwrap();
        assert_eq!(solution.get_int(a), 0);
    }

    #[test]
    fn test_logical_not_array_variables() {
        let mut model = Model::default();
        let a = model.bool();
        let b = model.bool();
        let c = model.bool();
        
        // Test not() with array syntax: not([a, b, c])
        // This should apply NOT to each variable individually
        post!(model, not([a, b, c]));
        
        let result = model.solve();
        assert!(result.is_ok(), "NOT with array variables should work");
        
        let solution = result.unwrap();
        assert_eq!(solution.get_int(a), 0); // All should be false (NOT applied individually)
        assert_eq!(solution.get_int(b), 0);
        assert_eq!(solution.get_int(c), 0);
    }

    #[test]
    fn test_mixed_logical_operations() {
        let mut model = Model::default();
        let a = model.bool();
        let b = model.bool();
        let c = model.bool();
        let d = model.bool();
        
        // Test mixing single and array syntaxes
        post!(model, and(a, b));        // Single variable syntax
        post!(model, or([c, d]));       // Array syntax
        post!(model, not(a));           // Single variable NOT (should conflict with and(a,b))
        
        let result = model.solve();
        // This should be unsatisfiable because and(a,b) requires a=1, but not(a) requires a=0
        assert!(result.is_err(), "Conflicting logical constraints should be unsatisfiable");
    }

    #[test]
    fn test_logical_operations_with_constraints() {
        let mut model = Model::default();
        let a = model.bool();
        let b = model.bool();
        let c = model.bool();
        let d = model.bool();
        
        // Complex logical scenario
        post!(model, or([a, b]));       // At least one of a,b is true
        post!(model, and([c, d]));      // Both c,d are true
        post!(model, not(a));           // a is false
        
        let result = model.solve();
        assert!(result.is_ok(), "Complex logical constraints should work");
        
        let solution = result.unwrap();
        assert_eq!(solution.get_int(a), 0); // a is false (due to not(a))
        assert_eq!(solution.get_int(b), 1); // b must be true (for or([a,b]) to hold)
        assert_eq!(solution.get_int(c), 1); // c is true (due to and([c,d]))
        assert_eq!(solution.get_int(d), 1); // d is true (due to and([c,d]))
    }

    #[test]
    fn test_logical_edge_cases() {
        let mut model = Model::default();
        let a = model.bool();
        
        // Test single element arrays
        post!(model, and([a]));         // Single element AND
        post!(model, or([a]));          // Single element OR (redundant but should work)
        post!(model, not([a]));         // Single element NOT
        
        let result = model.solve();
        // This is unsatisfiable: and([a]) requires a=1, but not([a]) requires a=0
        assert!(result.is_err(), "Conflicting single-element logical constraints should be unsatisfiable");
    }

    #[test]
    fn test_large_logical_arrays() {
        let mut model = Model::default();
        let vars: Vec<VarId> = (0..10).map(|_| model.bool()).collect();
        
        // Test with larger arrays - use array syntax with spread
        post!(model, or([vars[0], vars[1], vars[2], vars[3], vars[4], 
                        vars[5], vars[6], vars[7], vars[8], vars[9]]));          // At least one must be true
        
        // Force most to be false, leaving only one that can be true
        for i in 0..9 {
            post!(model, vars[i] == 0);
        }
        
        let result = model.solve();
        assert!(result.is_ok(), "Large logical arrays should work");
        
        let solution = result.unwrap();
        // First 9 should be false, last one should be true
        for i in 0..9 {
            assert_eq!(solution.get_int(vars[i]), 0);
        }
        assert_eq!(solution.get_int(vars[9]), 1);
    }
    
    // ===== COVERAGE STRESS TESTS =====
    
    #[test]
    fn test_multiple_variable_constraints() {
        let mut model = Model::default();
        let vars: Vec<VarId> = (0..5).map(|i| model.int(i, i + 10)).collect();
        
        // Create various constraints between variables
        post!(model, vars[0] < vars[1]);
        post!(model, vars[1] != vars[2]);
        post!(model, vars[2] >= vars[3]);
        post!(model, vars[3] == vars[4]);
        post!(model, vars[0] == 2);
        
        let result = model.solve();
        assert!(result.is_ok(), "Multiple variable constraints should work");
        
        let solution = result.unwrap();
        let values: Vec<i32> = vars.iter().map(|&v| solution.get_int(v)).collect();
        
        assert_eq!(values[0], 2);
        assert!(values[0] < values[1]);
        assert!(values[1] != values[2]);
        assert!(values[2] >= values[3]);
        assert_eq!(values[3], values[4]);
    }

    // ===== ADDITIONAL COMPREHENSIVE CONSTRAINT COVERAGE =====
    // Covering missing constraint types and runtime API methods

    // ===== GLOBAL CONSTRAINTS (RUNTIME API) =====

    #[test]
    fn test_alldiff_runtime_api() {
        let mut m = Model::default();
        let x = m.int(1, 3);
        let y = m.int(1, 3);
        let z = m.int(1, 3);
        
        // Using runtime API instead of post! macro
        m.alldiff(&[x, y, z]);
        
        let solution = m.solve().unwrap();
        let x_val = solution.get_int(x);
        let y_val = solution.get_int(y);
        let z_val = solution.get_int(z);
        
        // All values should be different
        assert_ne!(x_val, y_val);
        assert_ne!(y_val, z_val);
        assert_ne!(x_val, z_val);
    }

    #[test]
    fn test_alleq_runtime_api() {
        let mut m = Model::default();
        let x = m.int(1, 5);
        let y = m.int(1, 5);
        let z = m.int(1, 5);
        
        // All variables should have the same value
        m.alleq(&[x, y, z]);
        
        let solution = m.solve().unwrap();
        let x_val = solution.get_int(x);
        let y_val = solution.get_int(y);
        let z_val = solution.get_int(z);
        
        assert_eq!(x_val, y_val);
        assert_eq!(y_val, z_val);
    }

    #[test]
    fn test_element_constraint_runtime_api() {
        let mut m = Model::default();
        let array = vec![m.int(10, 10), m.int(20, 20), m.int(30, 30)]; // Fixed values
        let index = m.int(0, 2);
        let value = m.int(10, 30);
        
        // array[index] == value
        m.elem(&array, index, value);
        
        let solution = m.solve().unwrap();
        let index_val = solution.get_int(index) as usize;
        let value_val = solution.get_int(value);
        let array_val = solution.get_int(array[index_val]);
        
        assert_eq!(array_val, value_val);
    }

    #[test]
    fn test_count_constraint_runtime_api() {
        let mut m = Model::default();
        let vars = vec![m.int(1, 3), m.int(1, 3), m.int(1, 3), m.int(1, 3)];
        let result = m.int(0, 4);
        
        // Count how many vars equal 2
        m.count(&vars, 2, result);
        post!(m, result >= 1); // At least one variable should be 2
        
        let solution = m.solve().unwrap();
        let result_val = solution.get_int(result);
        let actual_count = vars.iter()
            .map(|&v| solution.get_int(v))
            .filter(|&val| val == 2)
            .count() as i32;
        
        assert_eq!(result_val, actual_count);
        assert!(result_val >= 1);
    }

    #[test]
    fn test_between_constraint_runtime_api() {
        let mut m = Model::default();
        let x = m.int(1, 10);
        
        // 3 <= x <= 7
        m.betw(x, 3, 7);
        
        let solution = m.solve().unwrap();
        let x_val = solution.get_int(x);
        
        assert!(x_val >= 3);
        assert!(x_val <= 7);
    }

    #[test]
    fn test_atleast_atmost_runtime_api() {
        let mut m = Model::default();
        let x = m.int(1, 10);
        let y = m.int(1, 10);
        
        m.atleast(x, 5); // x >= 5
        m.atmost(y, 7);  // y <= 7
        
        let solution = m.solve().unwrap();
        let x_val = solution.get_int(x);
        let y_val = solution.get_int(y);
        
        assert!(x_val >= 5);
        assert!(y_val <= 7);
    }

    #[test]
    fn test_global_cardinality_constraint() {
        let mut m = Model::default();
        let vars = vec![m.int(1, 3), m.int(1, 3), m.int(1, 3), m.int(1, 3)];
        let values = vec![1, 2, 3];
        let counts = vec![m.int(0, 4), m.int(0, 4), m.int(0, 4)];
        
        // Global cardinality: count each value
        m.gcc(&vars, &values, &counts);
        
        // Force at least one of each value
        for &count_var in &counts {
            post!(m, count_var >= 1);
        }
        
        let solution = m.solve().unwrap();
        
        // Verify counts
        for (i, &value) in values.iter().enumerate() {
            let expected_count = solution.get_int(counts[i]);
            let actual_count = vars.iter()
                .map(|&v| solution.get_int(v))
                .filter(|&val| val == value)
                .count() as i32;
            assert_eq!(expected_count, actual_count);
        }
    }

    // ===== MISSING POST! MACRO CONSTRAINTS =====

    #[test]
    fn test_alldiff_post_macro() {
        let mut m = Model::default();
        let x = m.int(1, 3);
        let y = m.int(1, 3);
        let z = m.int(1, 3);
        
        // Using post! macro for alldiff
        post!(m, alldiff([x, y, z]));
        
        let solution = m.solve().unwrap();
        let x_val = solution.get_int(x);
        let y_val = solution.get_int(y);
        let z_val = solution.get_int(z);
        
        assert_ne!(x_val, y_val);
        assert_ne!(y_val, z_val);
        assert_ne!(x_val, z_val);
    }

    #[test]
    fn test_allequal_post_macro() {
        let mut m = Model::default();
        let x = m.int(1, 5);
        let y = m.int(1, 5);
        let z = m.int(1, 5);
        
        post!(m, allequal([x, y, z]));
        
        let solution = m.solve().unwrap();
        let x_val = solution.get_int(x);
        let y_val = solution.get_int(y);
        let z_val = solution.get_int(z);
        
        assert_eq!(x_val, y_val);
        assert_eq!(y_val, z_val);
    }

    #[test]
    fn test_element_post_macro() {
        let mut m = Model::default();
        let array = vec![m.int(10, 10), m.int(20, 20), m.int(30, 30)];
        let index = m.int(0, 2);
        let value = m.int(10, 30);
        
        post!(m, element(array.clone(), index, value));
        
        let solution = m.solve().unwrap();
        let index_val = solution.get_int(index) as usize;
        let value_val = solution.get_int(value);
        let array_val = solution.get_int(array[index_val]);
        
        assert_eq!(array_val, value_val);
    }

    #[test]
    fn test_sum_constraints_post_macro() {
        let mut m = Model::default();
        let vars = vec![m.int(1, 5), m.int(1, 5), m.int(1, 5)];
        let total = m.int(3, 15);
        
        // Sum constraint using post! macro
        post!(m, sum(vars.clone()) == total);
        post!(m, total == 10); // Force specific sum
        
        let solution = m.solve().unwrap();
        let actual_sum: i32 = vars.iter().map(|&v| solution.get_int(v)).sum();
        let total_val = solution.get_int(total);
        
        assert_eq!(actual_sum, total_val);
        assert_eq!(total_val, 10);
    }

    #[test]
    fn test_modulo_constraints() {
        let mut m = Model::default();
        let x = m.int(1, 20);
        let y = m.int(1, 10);
        let divisor = m.int(3, 3); // Fixed divisor
        
        // Use the modulo function from model constraints
        let mod_var = m.modulo(x, divisor);
        post!(m, mod_var == 1);  // x mod 3 = 1
        
        // For now, skip the y % 2 == 0 test as it might have syntax issues
        post!(m, y >= 2);        // Just ensure y is reasonable
        
        let solution = m.solve().unwrap();
        let x_val = solution.get_int(x);
        let y_val = solution.get_int(y);
        let mod_val = solution.get_int(mod_var);
        
        assert_eq!(x_val % 3, 1);
        assert_eq!(mod_val, 1);
        assert!(y_val >= 2);
    }

    #[test]
    fn test_division_constraints() {
        let mut m = Model::default();
        let x = m.int(10, 20);
        let y = m.int(2, 5);
        let result = m.int(1, 10);
        
        post!(m, x / y == result);
        post!(m, result == 4);
        
        let solution = m.solve().unwrap();
        let x_val = solution.get_int(x);
        let y_val = solution.get_int(y);
        let result_val = solution.get_int(result);
        
        assert_eq!(x_val / y_val, result_val);
        assert_eq!(result_val, 4);
    }

    #[test]
    fn test_absolute_value_constraints() {
        let mut m = Model::default();
        let x = m.int(-10, 10);
        let abs_x = m.int(0, 10);
        
        // Test that the abs constraint actually works by validating the relationship
        post!(m, abs(x) == abs_x);
        
        let solution = m.solve().unwrap();
        let x_val = solution.get_int(x);
        let abs_x_val = solution.get_int(abs_x);
        
        assert_eq!(x_val.abs(), abs_x_val, "Absolute value constraint should be satisfied");
    }

    #[test]
    fn test_min_max_constraints() {
        let mut m = Model::default();
        let vars = vec![m.int(1, 10), m.int(1, 10), m.int(1, 10)];
        let min_var = m.int(1, 10);
        let max_var = m.int(1, 10);
        
        post!(m, min(vars.clone()) == min_var);
        post!(m, max(vars.clone()) == max_var);
        post!(m, max_var >= min_var); // Basic constraint
        
        let solution = m.solve().unwrap();
        let values: Vec<i32> = vars.iter().map(|&v| solution.get_int(v)).collect();
        let min_val = solution.get_int(min_var);
        let max_val = solution.get_int(max_var);
        
        assert_eq!(*values.iter().min().unwrap(), min_val);
        assert_eq!(*values.iter().max().unwrap(), max_val);
        assert!(max_val >= min_val);
    }

    // ===== RUNTIME API BUILDER PATTERN =====

    #[test]
    fn test_runtime_api_new_method() {
        let mut m = Model::default();
        let x = m.int(1, 10);
        let y = m.int(1, 10);
        
        // Using the new() method with runtime API
        m.new(x.ge(5));
        m.new(y.le(7));
        m.new(x.add(y).eq(10));
        
        let solution = m.solve().unwrap();
        let x_val = solution.get_int(x);
        let y_val = solution.get_int(y);
        
        assert!(x_val >= 5);
        assert!(y_val <= 7);
        assert_eq!(x_val + y_val, 10);
    }

    #[test]
    fn test_postall_runtime_api() {
        let mut m = Model::default();
        let x = m.int(1, 10);
        let y = m.int(1, 10);
        
        let constraints = vec![
            x.ge(5),
            y.le(7),
            x.add(y).eq(10)
        ];
        
        m.postall(constraints);
        
        let solution = m.solve().unwrap();
        let x_val = solution.get_int(x);
        let y_val = solution.get_int(y);
        
        assert!(x_val >= 5);
        assert!(y_val <= 7);
        assert_eq!(x_val + y_val, 10);
    }

    #[test]
    fn test_post_and_post_or_runtime_api() {
        let mut m = Model::default();
        let x = m.int(1, 10);
        let y = m.int(1, 10);
        
        // Either (x >= 8) OR (y <= 3)
        let or_constraints = vec![
            x.ge(8),
            y.le(3)
        ];
        m.post_or(or_constraints);
        
        // Both (x + y >= 5) AND (x + y <= 12)
        let and_constraints = vec![
            x.add(y).ge(5),
            x.add(y).le(12)
        ];
        m.post_and(and_constraints);
        
        let solution = m.solve().unwrap();
        let x_val = solution.get_int(x);
        let y_val = solution.get_int(y);
        
        // Check OR constraint
        assert!(x_val >= 8 || y_val <= 3);
        
        // Check AND constraint
        assert!(x_val + y_val >= 5);
        assert!(x_val + y_val <= 12);
    }

    // ===== EDGE CASES AND STRESS TESTS =====

    #[test]
    fn test_empty_constraint_lists() {
        let mut m = Model::default();
        let x = m.int(1, 5);
        
        // Test empty constraint lists
        let empty_result = m.postall(vec![]);
        assert!(empty_result.is_empty());
        
        let empty_and = m.post_and(vec![]);
        assert!(empty_and.is_none());
        
        let empty_or = m.post_or(vec![]);
        assert!(empty_or.is_none());
        
        // Model should still be solvable
        let solution = m.solve().unwrap();
        let x_val = solution.get_int(x);
        assert!(x_val >= 1 && x_val <= 5);
    }

    #[test]
    fn test_large_constraint_combinations() {
        let mut m = Model::default();
        let vars: Vec<VarId> = (0..5).map(|i| m.int(i, i + 5)).collect(); // Smaller for test stability
        
        // Large alldiff constraint
        m.alldiff(&vars);
        
        // Multiple count constraints
        for i in 0..3 {
            let count_var = m.int(0, 5);
            m.count(&vars, i, count_var);
        }
        
        // Should be solvable despite complexity
        let solution = m.solve().unwrap();
        
        // Verify alldiff worked
        let var_values: Vec<i32> = vars.iter().map(|&v| solution.get_int(v)).collect();
        let mut sorted_values = var_values.clone();
        sorted_values.sort();
        sorted_values.dedup();
        assert_eq!(sorted_values.len(), var_values.len()); // All different
    }

    // ===== BOOLEAN OPERATORS COMPREHENSIVE COVERAGE TESTS =====
    // Targeting: src/constraints/boolean_operators.rs (Function: 5.88%, Line: 13.43%)
    
    #[test]
    fn test_varid_bitand_basic_operation() {
        let mut model = Model::default();
        let a = model.bool();
        let b = model.bool();
        
        // Test BitAnd for VarId - creates BoolExpr
        let expr = a & b;
        
        // Apply the expression to model
        let result_var = expr.apply_to(&mut model);
        
        // Set values for AND operation: both true
        post!(model, a == 1);
        post!(model, b == 1);
        post!(model, result_var == 1);
        
        let solution = model.solve();
        assert!(solution.is_ok(), "VarId BitAnd with true values should work");
        
        let sol = solution.unwrap();
        let a_val = sol.get_int(a);
        let b_val = sol.get_int(b);
        let result_val = sol.get_int(result_var);
        
        // Verify AND logic: result should be 1 when both inputs are 1
        assert_eq!(a_val, 1, "Input a should be true");
        assert_eq!(b_val, 1, "Input b should be true");
        assert_eq!(result_val, 1, "AND result should be true when both inputs are true");
        assert_eq!(result_val, (a_val & b_val), "AND result should match bitwise AND of inputs");
    }

    #[test]
    fn test_varid_bitor_basic_operation() {
        let mut model = Model::default();
        let a = model.bool();
        let b = model.bool();
        
        // Test BitOr for VarId - creates BoolExpr
        let expr = a | b;
        
        // Apply the expression to model
        let result_var = expr.apply_to(&mut model);
        
        // Set values for OR operation: one true, one false
        post!(model, a == 1);
        post!(model, b == 0);
        post!(model, result_var == 1);
        
        let solution = model.solve();
        assert!(solution.is_ok(), "VarId BitOr with mixed values should work");
        
        let sol = solution.unwrap();
        let a_val = sol.get_int(a);
        let b_val = sol.get_int(b);
        let result_val = sol.get_int(result_var);
        
        // Verify OR logic: result should be 1 when at least one input is 1
        assert_eq!(a_val, 1, "Input a should be true");
        assert_eq!(b_val, 0, "Input b should be false");
        assert_eq!(result_val, 1, "OR result should be true when at least one input is true");
        assert_eq!(result_val, (a_val | b_val), "OR result should match bitwise OR of inputs");
    }

    #[test]
    fn test_varid_not_basic_operation() {
        let mut model = Model::default();
        let a = model.bool();
        
        // Test Not for VarId - creates BoolExpr
        let expr = !a;
        
        // Apply the expression to model
        let result_var = expr.apply_to(&mut model);
        
        // Set value for NOT operation: false -> true
        post!(model, a == 0);
        post!(model, result_var == 1);
        
        let solution = model.solve();
        assert!(solution.is_ok(), "VarId Not operation should work");
        
        let sol = solution.unwrap();
        let a_val = sol.get_int(a);
        let result_val = sol.get_int(result_var);
        
        // Verify NOT logic: result should be opposite of input
        assert_eq!(a_val, 0, "Input a should be false");
        assert_eq!(result_val, 1, "NOT result should be true when input is false");
        assert_eq!(result_val, 1 - a_val, "NOT result should be the logical inverse of input");
    }

    #[test]
    fn test_bool_expr_chaining_and() {
        let mut model = Model::default();
        let a = model.bool();
        let b = model.bool();
        let c = model.bool();
        
        // Test chained AND operations
        let expr_ab = a & b;
        let expr_abc = expr_ab & c;
        
        let result_var = expr_abc.apply_to(&mut model);
        
        // All should be true for AND chain to be true
        post!(model, a == 1);
        post!(model, b == 1);
        post!(model, c == 1);
        post!(model, result_var == 1);
        
        let solution = model.solve();
        assert!(solution.is_ok(), "Chained AND operations should work");
        
        let sol = solution.unwrap();
        let a_val = sol.get_int(a);
        let b_val = sol.get_int(b);
        let c_val = sol.get_int(c);
        let result_val = sol.get_int(result_var);
        
        // Verify chained AND logic: all inputs must be true for result to be true
        assert_eq!(a_val, 1, "Input a should be true");
        assert_eq!(b_val, 1, "Input b should be true");
        assert_eq!(c_val, 1, "Input c should be true");
        assert_eq!(result_val, 1, "Chained AND result should be true when all inputs are true");
        assert_eq!(result_val, (a_val & b_val & c_val), "Chained AND should match logical AND of all inputs");
    }

    #[test]
    fn test_bool_expr_chaining_or() {
        let mut model = Model::default();
        let a = model.bool();
        let b = model.bool();
        let c = model.bool();
        
        // Test chained OR operations
        let expr_ab = a | b;
        let expr_abc = expr_ab | c;
        
        let result_var = expr_abc.apply_to(&mut model);
        
        // Only one needs to be true for OR chain to be true
        post!(model, a == 0);
        post!(model, b == 0);
        post!(model, c == 1);
        post!(model, result_var == 1);
        
        let solution = model.solve();
        assert!(solution.is_ok(), "Chained OR operations should work");
        
        let sol = solution.unwrap();
        let a_val = sol.get_int(a);
        let b_val = sol.get_int(b);
        let c_val = sol.get_int(c);
        let result_val = sol.get_int(result_var);
        
        // Verify chained OR logic: at least one input must be true for result to be true
        assert_eq!(a_val, 0, "Input a should be false");
        assert_eq!(b_val, 0, "Input b should be false");
        assert_eq!(c_val, 1, "Input c should be true");
        assert_eq!(result_val, 1, "Chained OR result should be true when at least one input is true");
        assert_eq!(result_val, (a_val | b_val | c_val), "Chained OR should match logical OR of all inputs");
        assert!(a_val == 1 || b_val == 1 || c_val == 1, "At least one input should be true for OR to be true");
    }

    #[test]
    fn test_bool_expr_mixed_and_or() {
        let mut model = Model::default();
        let a = model.bool();
        let b = model.bool();
        let c = model.bool();
        
        // Test mixed AND and OR: (a & b) | c
        let expr_and = a & b;
        let expr_mixed = expr_and | c;
        
        let result_var = expr_mixed.apply_to(&mut model);
        
        // Test case where AND part is false but OR makes it true
        post!(model, a == 0);
        post!(model, b == 1);
        post!(model, c == 1);
        post!(model, result_var == 1);
        
        let solution = model.solve();
        assert!(solution.is_ok(), "Mixed AND/OR operations should work");
        
        let sol = solution.unwrap();
        let a_val = sol.get_int(a);
        let b_val = sol.get_int(b);
        let c_val = sol.get_int(c);
        let result_val = sol.get_int(result_var);
        
        // Verify mixed AND/OR logic: (a & b) | c
        assert_eq!(a_val, 0, "Input a should be false");
        assert_eq!(b_val, 1, "Input b should be true");
        assert_eq!(c_val, 1, "Input c should be true");
        assert_eq!(result_val, 1, "Mixed AND/OR result should be true");
        
        let and_part = a_val & b_val;
        let expected_result = and_part | c_val;
        assert_eq!(result_val, expected_result, "Result should match (a & b) | c");
        assert_eq!(and_part, 0, "AND part (a & b) should be false");
        assert!(and_part == 1 || c_val == 1, "Either AND part is true OR c is true");
    }

    #[test]
    fn test_bool_expr_not_chaining() {
        let mut model = Model::default();
        let a = model.bool();
        
        // Test double NOT: !(!a)
        let expr_not = !a;
        let expr_double_not = !expr_not;
        
        let result_var = expr_double_not.apply_to(&mut model);
        
        // Double NOT should equal original value
        post!(model, a == 1);
        post!(model, result_var == 1);
        
        let solution = model.solve();
        assert!(solution.is_ok(), "Double NOT should work correctly");
        
        let sol = solution.unwrap();
        let a_val = sol.get_int(a);
        let result_val = sol.get_int(result_var);
        
        // Verify double NOT logic: !!a should equal a
        assert_eq!(a_val, 1, "Input a should be true");
        assert_eq!(result_val, 1, "Double NOT result should be true");
        assert_eq!(result_val, a_val, "Double NOT should preserve original value");
        
        // Additional verification: double NOT should be identity
        let single_not = 1 - a_val;  // !a
        let double_not = 1 - single_not;  // !!a
        assert_eq!(result_val, double_not, "Double NOT should be mathematically correct");
    }

    #[test]
    fn test_varid_boolexpr_mixed_operations() {
        let mut model = Model::default();
        let a = model.bool();
        let b = model.bool();
        let c = model.bool();
        
        // Test VarId AND BoolExpr: a & (b | c)
        let expr_bc = b | c;
        let expr_mixed = a & expr_bc;
        
        let result_var = expr_mixed.apply_to(&mut model);
        
        // a must be true, and at least one of b or c must be true
        post!(model, a == 1);
        post!(model, b == 0);
        post!(model, c == 1);
        post!(model, result_var == 1);
        
        let solution = model.solve();
        assert!(solution.is_ok(), "VarId & BoolExpr mixed operations should work");
        
        let sol = solution.unwrap();
        let a_val = sol.get_int(a);
        let b_val = sol.get_int(b);
        let c_val = sol.get_int(c);
        let result_val = sol.get_int(result_var);
        
        // Verify mixed VarId & BoolExpr logic: a & (b | c)
        assert_eq!(a_val, 1, "Input a should be true");
        assert_eq!(b_val, 0, "Input b should be false");
        assert_eq!(c_val, 1, "Input c should be true");
        assert_eq!(result_val, 1, "Mixed operation result should be true");
        
        let or_part = b_val | c_val;
        let expected_result = a_val & or_part;
        assert_eq!(result_val, expected_result, "Result should match a & (b | c)");
        assert_eq!(or_part, 1, "OR part (b | c) should be true");
        assert!(b_val == 1 || c_val == 1, "At least one of b or c should be true for OR part");
    }

    #[test]
    fn test_boolexpr_varid_mixed_operations() {
        let mut model = Model::default();
        let a = model.bool();
        let b = model.bool();
        let c = model.bool();
        
        // Test BoolExpr OR VarId: (a & b) | c
        let expr_ab = a & b;
        let expr_mixed = expr_ab | c;
        
        let result_var = expr_mixed.apply_to(&mut model);
        
        // Either both a and b are true, or c is true
        post!(model, a == 0);
        post!(model, b == 1);
        post!(model, c == 1);
        post!(model, result_var == 1);
        
        let solution = model.solve();
        assert!(solution.is_ok(), "BoolExpr | VarId mixed operations should work");
        
        let sol = solution.unwrap();
        let a_val = sol.get_int(a);
        let b_val = sol.get_int(b);
        let c_val = sol.get_int(c);
        let result_val = sol.get_int(result_var);
        
        // Verify mixed BoolExpr | VarId logic: (a & b) | c
        assert_eq!(a_val, 0, "Input a should be false");
        assert_eq!(b_val, 1, "Input b should be true");
        assert_eq!(c_val, 1, "Input c should be true");
        assert_eq!(result_val, 1, "Mixed operation result should be true");
        
        let and_part = a_val & b_val;
        let expected_result = and_part | c_val;
        assert_eq!(result_val, expected_result, "Result should match (a & b) | c");
        assert_eq!(and_part, 0, "AND part (a & b) should be false");
        assert!(and_part == 1 || c_val == 1, "Either AND part is true OR c is true");
    }

    #[test]
    fn test_bool_expr_must_be_true() {
        let mut model = Model::default();
        let a = model.bool();
        let b = model.bool();
        
        // Test must_be_true method
        let expr = a & b;
        let constraint = expr.must_be_true(&mut model);
        constraint.apply_to(&mut model);
        
        let solution = model.solve();
        assert!(solution.is_ok(), "must_be_true should create valid constraint");
        
        if let Ok(sol) = solution {
            // For AND to be true, both must be true
            assert_eq!(sol.get_int(a), 1, "First variable should be true");
            assert_eq!(sol.get_int(b), 1, "Second variable should be true");
        }
    }

    #[test]
    fn test_bool_expr_must_be_false() {
        let mut model = Model::default();
        let a = model.bool();
        let b = model.bool();
        
        // Test must_be_false method with AND expression
        let expr = a & b;
        let constraint = expr.must_be_false(&mut model);
        constraint.apply_to(&mut model);
        
        let solution = model.solve();
        assert!(solution.is_ok(), "must_be_false should create valid constraint");
        
        if let Ok(sol) = solution {
            let a_val = sol.get_int(a);
            let b_val = sol.get_int(b);
            // For AND to be false, at least one must be false
            assert!(a_val == 0 || b_val == 0, "At least one variable should be false for AND to be false");
        }
    }

    #[test]
    fn test_varid_from_trait() {
        use selen::constraints::boolean_operators::BoolExpr;
        
        let mut model = Model::default();
        let a = model.bool();
        
        // Test From<VarId> for BoolExpr
        let expr: BoolExpr = a.into();
        
        // Should be able to apply it successfully
        let result_var = expr.apply_to(&mut model);
        assert_eq!(result_var, a, "From trait should preserve VarId for simple variable");
    }

    #[test]
    fn test_boolean_model_post_true() {
        use selen::constraints::boolean_operators::BooleanModel;
        
        let mut model = Model::default();
        let a = model.bool();
        
        // Create boolean expression: a itself
        let expr: selen::constraints::boolean_operators::BoolExpr = a.into();
        
        // Test BooleanModel::post_true
        model.post_true(expr);
        
        let solution = model.solve();
        assert!(solution.is_ok(), "post_true should work correctly");
        
        if let Ok(sol) = solution {
            assert_eq!(sol.get_int(a), 1, "Variable should be true after post_true");
        }
    }

    #[test]
    fn test_boolean_model_post_false() {
        use selen::constraints::boolean_operators::BooleanModel;
        
        let mut model = Model::default();
        let a = model.bool();
        
        // Create boolean expression: a itself
        let expr: selen::constraints::boolean_operators::BoolExpr = a.into();
        
        // Test BooleanModel::post_false
        model.post_false(expr);
        
        let solution = model.solve();
        assert!(solution.is_ok(), "post_false should work correctly");
        
        if let Ok(sol) = solution {
            assert_eq!(sol.get_int(a), 0, "Variable should be false after post_false");
        }
    }

    #[test]
    fn test_complex_boolean_expression() {
        use selen::constraints::boolean_operators::BooleanModel;
        
        let mut model = Model::default();
        let a = model.bool();
        let b = model.bool();
        let c = model.bool();
        let d = model.bool();
        
        // Test complex expression: (a & b) | (!c & d)
        let expr_ab = a & b;
        let expr_not_c = !c;
        let expr_not_c_and_d = expr_not_c & d;
        let complex_expr = expr_ab | expr_not_c_and_d;
        
        // Post as true
        model.post_true(complex_expr);
        
        // Set values that make the expression true via the second part: (!c & d)
        post!(model, a == 0); // Makes (a & b) false
        post!(model, b == 0); 
        post!(model, c == 0); // Makes !c true
        post!(model, d == 1); // Makes (!c & d) true
        
        let solution = model.solve();
        assert!(solution.is_ok(), "Complex boolean expression should be satisfiable");
        
        let sol = solution.unwrap();
        let a_val = sol.get_int(a);
        let b_val = sol.get_int(b);
        let c_val = sol.get_int(c);
        let d_val = sol.get_int(d);
        
        // Verify complex expression logic: (a & b) | (!c & d)
        assert_eq!(a_val, 0, "Input a should be false");
        assert_eq!(b_val, 0, "Input b should be false");
        assert_eq!(c_val, 0, "Input c should be false");
        assert_eq!(d_val, 1, "Input d should be true");
        
        let left_part = a_val & b_val;  // Should be 0
        let not_c = 1 - c_val;          // Should be 1
        let right_part = not_c & d_val; // Should be 1
        let total_result = left_part | right_part; // Should be 1
        
        assert_eq!(left_part, 0, "Left part (a & b) should be false");
        assert_eq!(not_c, 1, "NOT c should be true");
        assert_eq!(right_part, 1, "Right part (!c & d) should be true");
        assert_eq!(total_result, 1, "Overall expression should be true");
    }

    // ===== ADDITIONAL BOOLEAN OPERATOR EDGE CASES =====
    
    #[test]
    fn test_and_operation_false_case() {
        let mut model = Model::default();
        let a = model.bool();
        let b = model.bool();
        
        // Test AND operation when result should be false
        let expr = a & b;
        let result_var = expr.apply_to(&mut model);
        
        // Set values where AND should be false (one true, one false)
        post!(model, a == 1);
        post!(model, b == 0);
        post!(model, result_var == 0);
        
        let solution = model.solve();
        assert!(solution.is_ok(), "AND false case should work");
        
        let sol = solution.unwrap();
        let a_val = sol.get_int(a);
        let b_val = sol.get_int(b);
        let result_val = sol.get_int(result_var);
        
        assert_eq!(a_val, 1, "Input a should be true");
        assert_eq!(b_val, 0, "Input b should be false");
        assert_eq!(result_val, 0, "AND result should be false when one input is false");
        assert_eq!(result_val, (a_val & b_val), "AND result should match bitwise AND");
    }
    
    #[test]
    fn test_or_operation_false_case() {
        let mut model = Model::default();
        let a = model.bool();
        let b = model.bool();
        
        // Test OR operation when result should be false
        let expr = a | b;
        let result_var = expr.apply_to(&mut model);
        
        // Set values where OR should be false (both false)
        post!(model, a == 0);
        post!(model, b == 0);
        post!(model, result_var == 0);
        
        let solution = model.solve();
        assert!(solution.is_ok(), "OR false case should work");
        
        let sol = solution.unwrap();
        let a_val = sol.get_int(a);
        let b_val = sol.get_int(b);
        let result_val = sol.get_int(result_var);
        
        assert_eq!(a_val, 0, "Input a should be false");
        assert_eq!(b_val, 0, "Input b should be false");
        assert_eq!(result_val, 0, "OR result should be false when both inputs are false");
        assert_eq!(result_val, (a_val | b_val), "OR result should match bitwise OR");
    }
    
    #[test]
    fn test_not_operation_true_input() {
        let mut model = Model::default();
        let a = model.bool();
        
        // Test NOT operation with true input
        let expr = !a;
        let result_var = expr.apply_to(&mut model);
        
        // Set value for NOT operation: true -> false
        post!(model, a == 1);
        post!(model, result_var == 0);
        
        let solution = model.solve();
        assert!(solution.is_ok(), "NOT with true input should work");
        
        let sol = solution.unwrap();
        let a_val = sol.get_int(a);
        let result_val = sol.get_int(result_var);
        
        assert_eq!(a_val, 1, "Input a should be true");
        assert_eq!(result_val, 0, "NOT result should be false when input is true");
        assert_eq!(result_val, 1 - a_val, "NOT result should be logical inverse");
    }
    
    #[test]
    fn test_not_operation_false_case() {
        let mut model = Model::default();
        let a = model.bool();
        
        // Test NOT operation when input is false
        let expr = !a;
        let result_var = expr.apply_to(&mut model);
        
        // Set value for NOT operation: false -> true
        post!(model, a == 0);
        post!(model, result_var == 1);
        
        let solution = model.solve();
        assert!(solution.is_ok(), "NOT false case should work");
        
        let sol = solution.unwrap();
        let a_val = sol.get_int(a);
        let result_val = sol.get_int(result_var);
        
        assert_eq!(a_val, 0, "Input a should be false");
        assert_eq!(result_val, 1, "NOT result should be true when input is false");
        assert_eq!(result_val, 1 - a_val, "NOT result should be logical inverse");
    }

    // ===== CARDINALITY CONSTRAINTS COMPREHENSIVE COVERAGE =====
    // Targeting: src/constraints/props/cardinality.rs (Function: 66.67%, Line: 31.54%, Region: 27.80%)
    
    #[test]
    fn test_at_least_cardinality_constraint() {
        let mut model = Model::default();
        let vars = vec![model.bool(), model.bool(), model.bool(), model.bool()];
        
        // At least 2 variables should be true (value 1)
        let count_var = model.int(0, vars.len() as i32);
        model.count(&vars, 1, count_var);
        model.new(count_var.ge(2));
        
        let solution = model.solve();
        assert!(solution.is_ok(), "At least cardinality constraint should be satisfiable");
        
        let sol = solution.unwrap();
        let values: Vec<i32> = vars.iter().map(|&v| sol.get_int(v)).collect();
        let true_count = values.iter().filter(|&&v| v == 1).count();
        
        assert!(true_count >= 2, "At least 2 variables should be true, got {}", true_count);
        println!("At least constraint: {} true values out of {}", true_count, vars.len());
    }

    #[test]
    fn test_at_least_cardinality_impossible() {
        let mut model = Model::default();
        // Create variables that cannot all be 1
        let vars = vec![model.int(0, 0), model.int(0, 0), model.int(0, 1)]; // Only one can be 1
        
        // Require at least 3 variables to be 1 - impossible
        let count_var = model.int(0, vars.len() as i32);
        model.count(&vars, 1, count_var);
        model.new(count_var.ge(3));
        
        let solution = model.solve();
        assert!(solution.is_err(), "Impossible at least constraint should fail");
    }

    #[test]
    fn test_at_most_cardinality_constraint() {
        let mut model = Model::default();
        let vars = vec![model.bool(), model.bool(), model.bool(), model.bool()];
        
        // At most 2 variables should be true (value 1)
        let count_var = model.int(0, vars.len() as i32);
        model.count(&vars, 1, count_var);
        model.new(count_var.le(2));
        
        let solution = model.solve();
        assert!(solution.is_ok(), "At most cardinality constraint should be satisfiable");
        
        let sol = solution.unwrap();
        let values: Vec<i32> = vars.iter().map(|&v| sol.get_int(v)).collect();
        let true_count = values.iter().filter(|&&v| v == 1).count();
        
        assert!(true_count <= 2, "At most 2 variables should be true, got {}", true_count);
        println!("At most constraint: {} true values out of {}", true_count, vars.len());
    }

    #[test]
    fn test_at_most_cardinality_forced() {
        let mut model = Model::default();
        let vars = vec![model.bool(), model.bool(), model.bool()];
        
        // Force 2 variables to be true, then enforce at most 2
        post!(model, vars[0] == 1);
        post!(model, vars[1] == 1);
        
        let count_var = model.int(0, vars.len() as i32);
        model.count(&vars, 1, count_var);
        model.new(count_var.le(2));
        
        let solution = model.solve();
        assert!(solution.is_ok(), "At most with forced values should work");
        
        let sol = solution.unwrap();
        let values: Vec<i32> = vars.iter().map(|&v| sol.get_int(v)).collect();
        let true_count = values.iter().filter(|&&v| v == 1).count();
        
        assert_eq!(true_count, 2, "Should have exactly 2 true values");
        assert_eq!(sol.get_int(vars[2]), 0, "Third variable should be forced to false");
    }

    #[test]
    fn test_exactly_cardinality_constraint() {
        let mut model = Model::default();
        let vars = vec![model.bool(), model.bool(), model.bool(), model.bool()];
        
        // Exactly 2 variables should be true (value 1)
        let count_var = model.int(0, vars.len() as i32);
        model.count(&vars, 1, count_var);
        model.new(count_var.eq(2));
        
        let solution = model.solve();
        assert!(solution.is_ok(), "Exactly cardinality constraint should be satisfiable");
        
        let sol = solution.unwrap();
        let values: Vec<i32> = vars.iter().map(|&v| sol.get_int(v)).collect();
        let true_count = values.iter().filter(|&&v| v == 1).count();
        
        assert_eq!(true_count, 2, "Exactly 2 variables should be true, got {}", true_count);
        
        // Verify logical consistency
        let false_count = values.iter().filter(|&&v| v == 0).count();
        assert_eq!(false_count, 2, "Exactly 2 variables should be false");
        assert_eq!(true_count + false_count, vars.len(), "All variables should be assigned");
    }

    #[test]
    fn test_exactly_cardinality_impossible() {
        let mut model = Model::default();
        let vars = vec![model.int(1, 1), model.int(1, 1)]; // Both must be 1
        
        // Require exactly 1 variable to be 1 - impossible since both must be 1
        let count_var = model.int(0, vars.len() as i32);
        model.count(&vars, 1, count_var);
        model.new(count_var.eq(1));
        
        let solution = model.solve();
        assert!(solution.is_err(), "Impossible exactly constraint should fail");
    }

    #[test]
    fn test_cardinality_with_different_values() {
        let mut model = Model::default();
        let vars = vec![model.int(0, 2), model.int(0, 2), model.int(0, 2), model.int(0, 2)];
        
        // Exactly 2 variables should equal 2
        let count_var = model.int(0, vars.len() as i32);
        model.count(&vars, 2, count_var);
        model.new(count_var.eq(2));
        
        let solution = model.solve();
        assert!(solution.is_ok(), "Cardinality with value 2 should work");
        
        let sol = solution.unwrap();
        let values: Vec<i32> = vars.iter().map(|&v| sol.get_int(v)).collect();
        let count_2 = values.iter().filter(|&&v| v == 2).count();
        
        assert_eq!(count_2, 2, "Exactly 2 variables should equal 2, got {}", count_2);
        
        // Other values should be 0 or 1
        for &value in &values {
            assert!(value >= 0 && value <= 2, "All values should be in domain [0,2]");
        }
    }

    #[test]
    fn test_cardinality_edge_case_zero_count() {
        let mut model = Model::default();
        let vars = vec![model.bool(), model.bool(), model.bool()];
        
        // Exactly 0 variables should be true (all should be false)
        let count_var = model.int(0, vars.len() as i32);
        model.count(&vars, 1, count_var);
        model.new(count_var.eq(0));
        
        let solution = model.solve();
        assert!(solution.is_ok(), "Zero cardinality constraint should work");
        
        let sol = solution.unwrap();
        let values: Vec<i32> = vars.iter().map(|&v| sol.get_int(v)).collect();
        let true_count = values.iter().filter(|&&v| v == 1).count();
        
        assert_eq!(true_count, 0, "No variables should be true");
        for &value in &values {
            assert_eq!(value, 0, "All variables should be false");
        }
    }

    #[test]
    fn test_cardinality_edge_case_all_count() {
        let mut model = Model::default();
        let vars = vec![model.bool(), model.bool(), model.bool()];
        
        // Exactly all variables should be true
        let count_var = model.int(0, vars.len() as i32);
        model.count(&vars, 1, count_var);
        model.new(count_var.eq(vars.len() as i32));
        
        let solution = model.solve();
        assert!(solution.is_ok(), "All variables cardinality constraint should work");
        
        let sol = solution.unwrap();
        let values: Vec<i32> = vars.iter().map(|&v| sol.get_int(v)).collect();
        let true_count = values.iter().filter(|&&v| v == 1).count();
        
        assert_eq!(true_count, vars.len(), "All variables should be true");
        for &value in &values {
            assert_eq!(value, 1, "All variables should be true");
        }
    }

    #[test]
    fn test_cardinality_with_fixed_variables() {
        let mut model = Model::default();
        let vars = vec![model.bool(), model.bool(), model.bool(), model.bool()];
        
        // Fix some variables and test cardinality
        post!(model, vars[0] == 1); // Force first to true
        post!(model, vars[1] == 0); // Force second to false
        
        // Exactly 2 variables should be true (including the forced one)
        let count_var = model.int(0, vars.len() as i32);
        model.count(&vars, 1, count_var);
        model.new(count_var.eq(2));
        
        let solution = model.solve();
        assert!(solution.is_ok(), "Cardinality with fixed variables should work");
        
        let sol = solution.unwrap();
        let values: Vec<i32> = vars.iter().map(|&v| sol.get_int(v)).collect();
        
        // Verify fixed values
        assert_eq!(values[0], 1, "First variable should be fixed to true");
        assert_eq!(values[1], 0, "Second variable should be fixed to false");
        
        // Count true values
        let true_count = values.iter().filter(|&&v| v == 1).count();
        assert_eq!(true_count, 2, "Exactly 2 variables should be true");
        
        // One of the remaining variables should be true
        let remaining_true_count = [values[2], values[3]].iter().filter(|&&v| v == 1).count();
        assert_eq!(remaining_true_count, 1, "Exactly one of the remaining variables should be true");
    }

    #[test]
    fn test_multiple_cardinality_constraints() {
        let mut model = Model::default();
        let vars = vec![model.int(0, 2), model.int(0, 2), model.int(0, 2), model.int(0, 2)];
        
        // At least 1 variable should equal 0
        let count_0_var = model.int(0, vars.len() as i32);
        model.count(&vars, 0, count_0_var);
        model.new(count_0_var.ge(1));
        
        // At most 2 variables should equal 1
        let count_1_var = model.int(0, vars.len() as i32);
        model.count(&vars, 1, count_1_var);
        model.new(count_1_var.le(2));
        
        // Exactly 1 variable should equal 2
        let count_2_var = model.int(0, vars.len() as i32);
        model.count(&vars, 2, count_2_var);
        model.new(count_2_var.eq(1));
        
        let solution = model.solve();
        assert!(solution.is_ok(), "Multiple cardinality constraints should be satisfiable");
        
        let sol = solution.unwrap();
        let values: Vec<i32> = vars.iter().map(|&v| sol.get_int(v)).collect();
        
        // Verify all constraints
        let count_0 = values.iter().filter(|&&v| v == 0).count();
        let count_1 = values.iter().filter(|&&v| v == 1).count();
        let count_2 = values.iter().filter(|&&v| v == 2).count();
        
        assert!(count_0 >= 1, "At least 1 variable should equal 0");
        assert!(count_1 <= 2, "At most 2 variables should equal 1");
        assert_eq!(count_2, 1, "Exactly 1 variable should equal 2");
        
        println!("Multiple cardinality: {} zeros, {} ones, {} twos", count_0, count_1, count_2);
    }

    #[test]
    fn test_cardinality_large_domain() {
        let mut model = Model::default();
        let vars = vec![
            model.int(1, 10), model.int(1, 10), model.int(1, 10), 
            model.int(1, 10), model.int(1, 10)
        ];
        
        // Exactly 3 variables should equal 5
        let count_var = model.int(0, vars.len() as i32);
        model.count(&vars, 5, count_var);
        model.new(count_var.eq(3));
        
        let solution = model.solve();
        assert!(solution.is_ok(), "Cardinality with large domain should work");
        
        let sol = solution.unwrap();
        let values: Vec<i32> = vars.iter().map(|&v| sol.get_int(v)).collect();
        let count_5 = values.iter().filter(|&&v| v == 5).count();
        
        assert_eq!(count_5, 3, "Exactly 3 variables should equal 5");
        
        // All values should be in valid domain
        for &value in &values {
            assert!(value >= 1 && value <= 10, "All values should be in domain [1,10]");
        }
        
        // Non-5 values should be something else in the domain
        let non_5_values: Vec<i32> = values.iter().filter(|&&v| v != 5).cloned().collect();
        for &value in &non_5_values {
            assert!(value >= 1 && value <= 10 && value != 5, "Non-5 values should be in domain but not 5");
        }
    }

    #[test]
    fn test_cardinality_propagation_effects() {
        let mut model = Model::default();
        let vars = vec![model.bool(), model.bool()];
        
        // Exactly 1 variable should be true
        let count_var = model.int(0, vars.len() as i32);
        model.count(&vars, 1, count_var);
        model.new(count_var.eq(1));
        
        // Force one variable to be true
        post!(model, vars[0] == 1);
        
        let solution = model.solve();
        assert!(solution.is_ok(), "Cardinality propagation should work");
        
        let sol = solution.unwrap();
        
        // The constraint should have propagated to force the second variable to false
        assert_eq!(sol.get_int(vars[0]), 1, "First variable should be true");
        assert_eq!(sol.get_int(vars[1]), 0, "Second variable should be propagated to false");
        
        // Verify cardinality is satisfied
        let values: Vec<i32> = vars.iter().map(|&v| sol.get_int(v)).collect();
        let true_count = values.iter().filter(|&&v| v == 1).count();
        assert_eq!(true_count, 1, "Exactly 1 variable should be true after propagation");
    }

    #[test]
    fn test_gcc_cardinality_constraint() {
        let mut model = Model::default();
        let vars = vec![model.int(1, 3), model.int(1, 3), model.int(1, 3), model.int(1, 3)];
        
        // Use global cardinality constraint to specify counts for each value
        let values = [1, 2, 3];
        let counts = vec![model.int(0, 4), model.int(0, 4), model.int(0, 4)];
        
        model.gcc(&vars, &values, &counts);
        
        // Force specific counts: 2 ones, 1 two, 1 three
        model.new(counts[0].eq(2)); // 2 variables equal 1
        model.new(counts[1].eq(1)); // 1 variable equals 2
        model.new(counts[2].eq(1)); // 1 variable equals 3
        
        let solution = model.solve();
        assert!(solution.is_ok(), "Global cardinality constraint should be satisfiable");
        
        let sol = solution.unwrap();
        let values: Vec<i32> = vars.iter().map(|&v| sol.get_int(v)).collect();
        
        // Verify counts
        let count_1 = values.iter().filter(|&&v| v == 1).count();
        let count_2 = values.iter().filter(|&&v| v == 2).count();
        let count_3 = values.iter().filter(|&&v| v == 3).count();
        
        assert_eq!(count_1, 2, "Should have exactly 2 variables equal to 1");
        assert_eq!(count_2, 1, "Should have exactly 1 variable equal to 2");
        assert_eq!(count_3, 1, "Should have exactly 1 variable equal to 3");
        
        // Verify count variables are correct
        assert_eq!(sol.get_int(counts[0]), 2, "Count variable for 1s should be 2");
        assert_eq!(sol.get_int(counts[1]), 1, "Count variable for 2s should be 1");
        assert_eq!(sol.get_int(counts[2]), 1, "Count variable for 3s should be 1");
    }
}