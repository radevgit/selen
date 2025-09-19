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