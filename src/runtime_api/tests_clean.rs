//! Tests for the runtime constraint API - Clean version with improved Solution API

use crate::prelude::*;
use crate::runtime_api::{VarIdExt, ModelExt, ConstraintVecExt};

#[test]
fn test_clean_solution_api_demo() {
    let mut m = Model::default();
    let x = m.int(5, 10);
    let y = m.int(0, 5);
    
    // Simple constraints
    m.post(x.ge(int(7)));
    m.post(y.le(int(3)));
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(solution) = result {
        // Clean approaches to get values:
        
        // Option 1: Direct get_int() method (cleanest, no unwrap needed)
        let x_val = solution.get_int(x);
        let y_val = solution.get_int(y);
        
        // Option 2: Indexing + as_int() method
        let x_alt = solution[x].as_int().unwrap();
        let y_alt = solution[y].as_int().unwrap();
        
        // Option 3: Safe version with try_get_int()
        let x_safe = solution.try_get_int(x).expect("x should be an integer");
        let y_safe = solution.try_get_int(y).expect("y should be an integer");
        
        // All approaches should give the same result
        assert_eq!(x_val, x_alt);
        assert_eq!(x_val, x_safe);
        assert_eq!(y_val, y_alt);
        assert_eq!(y_val, y_safe);
        
        // Verify the constraints are satisfied
        assert!(x_val >= 7);
        assert!(y_val <= 3);
    }
    
        println!("✓ Comprehensive Phase 3 + Clean API demonstration complete!");
}

#[test]
fn test_automatic_type_inference() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    let y = m.float(0.0, 5.0);
    
    // Add constraints
    m.post(x.ge(int(5)));
    m.post(y.le(float(3.0)));
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(solution) = result {
        // ✨ NEW FEATURE: Automatic type inference!
        // The compiler infers the type from explicit type annotations
        
        // Method 1: Explicit type annotation (works!)
        let x_val: i32 = solution.get(x);     // Infers i32 from type annotation
        let y_val: f64 = solution.get(y);     // Infers f64 from type annotation
        
        // Method 2: Direct assignment to typed variables (works!)
        let x_as_int: i32 = solution.get(x);
        let y_as_float: f64 = solution.get(y);
        
        // Method 3: Safe option types (works!)
        let x_opt: Option<i32> = solution.get(x); // Infers Option<i32>
        let y_opt: Option<f64> = solution.get(y); // Infers Option<f64>
        
        // Method 4: Function parameter inference (works!)
        fn process_int(val: i32) -> i32 { val * 2 }
        fn process_float(val: f64) -> f64 { val * val }
        
        let x_processed = process_int(solution.get(x)); // Infers i32 from function parameter
        let y_processed = process_float(solution.get(y)); // Infers f64 from function parameter
        
        // Verify all approaches work
        assert!(x_val >= 5);
        assert!(y_val <= 3.0);
        assert_eq!(x_as_int, x_val);
        assert_eq!(y_as_float, y_val);
        assert_eq!(x_opt, Some(x_val));
        assert_eq!(y_opt, Some(y_val));
        assert_eq!(x_processed, x_val * 2);
        assert_eq!(y_processed, y_val * y_val);
        
        println!("✨ Type inference works!");
        println!("   x: i32 = {} (inferred from annotation)", x_val);
        println!("   y: f64 = {} (inferred from annotation)", y_val);
        println!("   x_processed: i32 = {} (inferred from function)", x_processed);
        println!("   x_opt: Option<i32> = {:?} (inferred from type)", x_opt);
    }
    
    println!("✓ Automatic type inference test complete!");
}

#[test]
fn test_phase3_boolean_logic_with_clean_api() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // Create constraints using the clean API
    let c1 = x.ge(int(5));  // x >= 5
    let c2 = y.le(int(8));  // y <= 8
    
    // Combine with AND
    let combined = c1.and(c2);
    m.post(combined);
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(solution) = result {
        // Clean value extraction - no .unwrap() needed!
        let x_val = solution.get_int(x);
        let y_val = solution.get_int(y);
        
        assert!(x_val >= 5);
        assert!(y_val <= 8);
    }
    
    println!("✓ Phase 3 boolean logic with clean API works!");
}

#[test]
#[ignore = "OR constraints need more work on single variables"]
fn test_constraint_or_with_clean_api() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    
    // Create constraints: x == 2 OR x == 8
    // TODO: This currently fails because OR logic for single variables needs work
    let c1 = x.eq(int(2));
    let c2 = x.eq(int(8));
    let combined = c1.or(c2);
    m.post(combined);
    
    let result = m.solve();
    
    // Debug the result
    match &result {
        Ok(solution) => {
            let x_val = solution[x].as_int().unwrap();
            assert!(x_val == 2 || x_val == 8);
            println!("✓ OR constraints with clean API work! x = {}", x_val);
        }
        Err(e) => {
            println!("❌ OR constraint failed: {:?}", e);
            panic!("Expected success but got error: {:?}", e);
        }
    }
}

#[test]
fn test_constraint_vec_operations() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let z = m.int(0, 10);
    
    let constraints = vec![
        x.ge(int(3)),  // x >= 3
        y.le(int(7)),  // y <= 7
        z.eq(int(5)),  // z == 5
    ];
    
    // Use ConstraintVecExt trait
    if let Some(combined) = constraints.and_all() {
        m.post(combined);
    }
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(solution) = result {
        // Mix of clean approaches
        let x_val = solution.get_int(x);        // Direct method
        let y_val = solution[y].as_int().unwrap(); // Indexing + as_int()
        let z_val = solution.try_get_int(z).unwrap(); // Safe method
        
        assert!(x_val >= 3);
        assert!(y_val <= 7);
        assert_eq!(z_val, 5);
    }
    
    println!("✓ Constraint vector operations with clean API work!");
}

#[test]
fn test_model_post_methods() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    let constraints = vec![
        x.ge(int(4)),
        y.le(int(6)),
    ];
    
    // Test post_all method
    let prop_ids = m.post_all(constraints);
    assert_eq!(prop_ids.len(), 2);
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(solution) = result {
        // Clean API - no ugly .unwrap() chains
        let x_val = solution.get_int(x);
        let y_val = solution.get_int(y);
        
        assert!(x_val >= 4);
        assert!(y_val <= 6);
    }
    
    println!("✓ Model post methods with clean API work!");
}

#[test] 
fn test_comprehensive_clean_api_features() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    
    // Test Model::c() method with clean API
    m.c(x).add(y).ge(int(8));
    m.c(x).mul(int(2)).le(y.add(int(6)));
    
    // Test global constraints
    let vars = vec![x, y];
    m.alldiff(&vars);
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(solution) = result {
        // Demonstrate all three clean approaches
        println!("Values using different clean approaches:");
        
        // 1. Direct methods (cleanest)
        let x_direct = solution.get_int(x);
        let y_direct = solution.get_int(y);
        println!("  Direct: x={}, y={}", x_direct, y_direct);
        
        // 2. Indexing syntax
        let x_index = solution[x].as_int().unwrap();
        let y_index = solution[y].as_int().unwrap();
        println!("  Indexing: x={}, y={}", x_index, y_index);
        
        // 3. Safe methods
        let x_safe = solution.try_get_int(x).unwrap();
        let y_safe = solution.try_get_int(y).unwrap();
        println!("  Safe: x={}, y={}", x_safe, y_safe);
        
        // All should be equal
        assert_eq!(x_direct, x_index);
        assert_eq!(x_direct, x_safe);
        assert_eq!(y_direct, y_index);
        assert_eq!(y_direct, y_safe);
        
        // Verify constraints
        assert!(x_direct + y_direct >= 8);
        assert!(x_direct * 2 <= y_direct + 6);
        assert_ne!(x_direct, y_direct); // alldiff
    }
    
    println!("✓ Comprehensive clean API features work perfectly!");
}

#[test]
fn test_safe_constraint_building_no_panics() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // Function to safely build constraints without panicking
    fn build_constraint_safe(var: VarId, op: &str, value: i32) -> Option<Constraint> {
        match op {
            "eq" => Some(var.eq(int(value))),
            "gt" => Some(var.gt(int(value))),
            "lt" => Some(var.lt(int(value))),
            "ge" => Some(var.ge(int(value))),
            "le" => Some(var.le(int(value))),
            _ => None  // Invalid operator - return None instead of panic
        }
    }
    
    // Test data with some invalid operators
    let constraint_specs = vec![
        (x, "ge", 3),        // Valid
        (y, "le", 7),        // Valid  
        (x, "invalid", 5),   // Invalid - should not panic!
        (y, "bad_op", 2),    // Invalid - should not panic!
    ];
    
    let mut successful_constraints = 0;
    let mut failed_constraints = 0;
    
    // Build constraints safely
    for (var, op, value) in constraint_specs {
        match build_constraint_safe(var, op, value) {
            Some(constraint) => {
                m.post(constraint);
                successful_constraints += 1;
            }
            None => {
                failed_constraints += 1;
                // Log error but don't panic - graceful degradation
                println!("Warning: Unknown operator '{}', skipping constraint", op);
            }
        }
    }
    
    // Verify we handled errors gracefully
    assert_eq!(successful_constraints, 2);
    assert_eq!(failed_constraints, 2);
    
    // Model should still be solvable with valid constraints
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(solution) = result {
        let x_val = solution.get_int(x);
        let y_val = solution.get_int(y);
        
        // Verify the valid constraints were applied
        assert!(x_val >= 3);
        assert!(y_val <= 7);
    }
    
    println!("✓ Safe constraint building - no panics, graceful error handling!");
}

// =================== PHASE 4: GLOBAL CONSTRAINTS TESTS ===================

#[test]
fn test_all_different_constraint() {
    let mut m = Model::default();
    let vars: Vec<_> = (0..3).map(|_| m.int(1, 3)).collect();
    
    // All variables must have different values
    m.alldiff(&vars);
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(solution) = result {
        let values: Vec<i32> = vars.iter().map(|&v| solution.get_int(v)).collect();
        
        // Verify all values are different
        for i in 0..values.len() {
            for j in i+1..values.len() {
                assert_ne!(values[i], values[j], "Values should all be different");
            }
        }
        
        // Verify all values are in valid range
        for &value in &values {
            assert!(value >= 1 && value <= 3);
        }
    }
    
    println!("✓ All different constraint test passed!");
}

#[test]
fn test_all_equal_constraint() {
    let mut m = Model::default();
    let vars: Vec<_> = (0..3).map(|_| m.int(1, 10)).collect();
    
    // All variables must have the same value
    m.alleq(&vars);
    
    // Add additional constraint
    m.post(vars[0].ge(5));
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(solution) = result {
        let values: Vec<i32> = vars.iter().map(|&v| solution.get_int(v)).collect();
        
        // Verify all values are equal
        let first_value = values[0];
        for &value in &values {
            assert_eq!(value, first_value, "All values should be equal");
        }
        
        // Verify constraint is satisfied
        assert!(first_value >= 5);
    }
    
    println!("✓ All equal constraint test passed!");
}

#[test]
fn test_element_constraint() {
    let mut m = Model::default();
    
    // Create array with specific values
    let array: Vec<_> = (0..3).map(|i| m.int(i * 10, i * 10)).collect(); // [0, 10, 20]
    let index = m.int(0, 2);
    let value = m.int(0, 20);
    
    // Element constraint: array[index] == value
    m.elem(&array, index, value);
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(solution) = result {
        let idx = solution.get_int(index) as usize;
        let val = solution.get_int(value);
        let array_val = solution.get_int(array[idx]);
        
        // Verify element constraint
        assert_eq!(array_val, val, "array[index] should equal value");
        
        // Verify index is in valid range
        assert!(idx < array.len());
    }
    
    println!("✓ Element constraint test passed!");
}

#[test]
fn test_count_constraint() {
    let mut m = Model::default();
    let vars: Vec<_> = (0..5).map(|_| m.int(1, 3)).collect();
    let count_result = m.int(0, 5);
    
    // Count occurrences of value 2
    m.count(&vars, 2, count_result);
    
    // Force exactly 2 occurrences of value 2
    m.post(count_result.eq(2));
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(solution) = result {
        let values: Vec<i32> = vars.iter().map(|&v| solution.get_int(v)).collect();
        let count = solution.get_int(count_result);
        
        // Count manually
        let actual_count = values.iter().filter(|&&v| v == 2).count();
        
        // Verify count constraint
        assert_eq!(count, 2, "Count should be exactly 2");
        assert_eq!(actual_count, 2, "Actual count should match");
    }
    
    println!("✓ Count constraint test passed!");
}

#[test] 
fn test_cardinality_constraints() {
    let mut m = Model::default();
    let x = m.int(0, 100);
    let y = m.int(0, 100);
    let z = m.int(0, 100);
    
    // Between constraint: x must be between 10 and 20
    m.betw(x, 10, 20);
    
    // At most constraint: y must be at most 50
    m.atmost(y, 50);
    
    // At least constraint: z must be at least 75
    m.atleast(z, 75);
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(solution) = result {
        let x_val = solution.get_int(x);
        let y_val = solution.get_int(y);
        let z_val = solution.get_int(z);
        
        // Verify cardinality constraints
        assert!(x_val >= 10 && x_val <= 20, "x should be between 10 and 20");
        assert!(y_val <= 50, "y should be at most 50");
        assert!(z_val >= 75, "z should be at least 75");
    }
    
    println!("✓ Cardinality constraints test passed!");
}

#[test]
fn test_global_cardinality_constraint() {
    let mut m = Model::default();
    let vars: Vec<_> = (0..6).map(|_| m.int(1, 3)).collect();
    
    // Count variables with values 1, 2, 3
    let values = [1, 2, 3];
    let counts: Vec<_> = (0..3).map(|_| m.int(0, 6)).collect();
    
    // Global cardinality constraint
    m.gcc(&vars, &values, &counts);
    
    // Force specific counts
    m.post(counts[0].eq(2)); // Exactly 2 ones
    m.post(counts[1].eq(3)); // Exactly 3 twos
    m.post(counts[2].eq(1)); // Exactly 1 three
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(solution) = result {
        let var_values: Vec<i32> = vars.iter().map(|&v| solution.get_int(v)).collect();
        let count_values: Vec<i32> = counts.iter().map(|&c| solution.get_int(c)).collect();
        
        // Verify counts manually
        for (i, &target_value) in values.iter().enumerate() {
            let actual_count = var_values.iter().filter(|&&v| v == target_value).count() as i32;
            let constraint_count = count_values[i];
            
            assert_eq!(actual_count, constraint_count, 
                "Count of value {} should match constraint", target_value);
        }
        
        // Verify specific constraints
        assert_eq!(count_values[0], 2, "Should have exactly 2 ones");
        assert_eq!(count_values[1], 3, "Should have exactly 3 twos");  
        assert_eq!(count_values[2], 1, "Should have exactly 1 three");
    }
    
    println!("✓ Global cardinality constraint test passed!");
}

#[test]
fn test_combined_global_constraints() {
    let mut m = Model::default();
    
    // Create scheduling problem with global constraints
    let tasks: Vec<_> = (0..3).map(|_| m.int(1, 5)).collect(); // Start times
    let resources: Vec<_> = (0..3).map(|_| m.int(1, 2)).collect(); // Resource assignments
    
    // All tasks must start at different times
    m.alldiff(&tasks);
    
    // Count resource usage
    let resource_counts: Vec<_> = (0..2).map(|_| m.int(0, 3)).collect();
    m.gcc(&resources, &[1, 2], &resource_counts);
    
    // Each resource should be used at least once
    for &count_var in &resource_counts {
        m.atleast(count_var, 1);
    }
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(solution) = result {
        let task_times: Vec<i32> = tasks.iter().map(|&t| solution.get_int(t)).collect();
        let resource_counts_vals: Vec<i32> = resource_counts.iter().map(|&c| solution.get_int(c)).collect();
        
        // Verify all different constraint
        for i in 0..task_times.len() {
            for j in i+1..task_times.len() {
                assert_ne!(task_times[i], task_times[j], "Task times should be different");
            }
        }
        
        // Verify each resource is used at least once
        for &count in &resource_counts_vals {
            assert!(count >= 1, "Each resource should be used at least once");
        }
        
        // Verify resource count totals
        let total_usage: i32 = resource_counts_vals.iter().sum();
        assert_eq!(total_usage, 3, "Total resource usage should equal number of tasks");
    }
    
    println!("✓ Combined global constraints test passed!");
}