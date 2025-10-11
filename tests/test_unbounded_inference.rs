/// Tests for unbounded variable inference from constraint ASTs
/// 
/// This test suite validates that the new deferred inference phase correctly
/// extracts bounds from constraints and applies them to unbounded variables.

use selen::prelude::*;

#[test]
fn test_inference_binary_comparison_less_than() {
    // Test: x < 10 where x is initially unbounded
    let mut m = Model::default();
    let x = m.new_var(int(i32::MIN), int(i32::MAX));
    
    // Add constraint: x < 10
    lt(&mut m, x, int(10));
    
    // Inference should apply bound x <= 9 during solve
    let solution = m.solve().expect("Should find solution");
    let val = solution.get_int(x);
    assert!(val < 10, "Solution should satisfy x < 10");
}

#[test]
fn test_inference_binary_comparison_greater_equal() {
    // Test: x >= 0 where x is initially unbounded
    let mut m = Model::default();
    let x = m.new_var(int(i32::MIN), int(i32::MAX));
    
    // Add constraint: x >= 0
    ge(&mut m, x, int(0));
    
    let solution = m.solve().expect("Should find solution");
    let val = solution.get_int(x);
    assert!(val >= 0, "Solution should satisfy x >= 0");
}

#[test]
fn test_inference_both_bounds() {
    // Test: 0 <= x < 100 where x is initially unbounded
    let mut m = Model::default();
    let x = m.new_var(int(i32::MIN), int(i32::MAX));
    
    // Add constraints: x >= 0 and x < 100
    ge(&mut m, x, int(0));
    lt(&mut m, x, int(100));
    
    let solution = m.solve().expect("Should find solution");
    let val = solution.get_int(x);
    assert!(val >= 0 && val < 100, "Solution should satisfy 0 <= x < 100");
}

#[test]
fn test_inference_equality_constraint() {
    // Test: x == 42 where x is initially unbounded
    let mut m = Model::default();
    let x = m.new_var(int(i32::MIN), int(i32::MAX));
    
    // Add constraint: x == 42
    eq(&mut m, x, int(42));
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution.get_int(x), 42, "Solution should be x = 42");
}

#[test]
fn test_inference_transitive_bounds() {
    // Test: x < y, y is bounded [0, 10] → x should be < 10
    let mut m = Model::default();
    let x = m.new_var(int(i32::MIN), int(i32::MAX));
    let y = m.new_var(int(0), int(10));
    
    // Add constraint: x < y
    lt(&mut m, x, y);
    
    let solution = m.solve().expect("Should find solution");
    let x_val = solution.get_int(x);
    let y_val = solution.get_int(y);
    assert!(x_val < y_val, "Solution should satisfy x < y");
    assert!(x_val < 10, "x should be bounded by y's upper bound");
}

#[test]
fn test_inference_element_constraint() {
    // Test: array[x] where array has 5 elements → x ∈ [0, 4]
    let mut m = Model::default();
    let x = m.new_var(int(i32::MIN), int(i32::MAX));
    
    let array = m.ints(5, 10, 50);
    
    // Add constraint: array[x] = result
    let _result = element(&mut m, &array, x);
    
    let solution = m.solve().expect("Should find solution");
    let x_val = solution.get_int(x);
    assert!(x_val >= 0 && x_val < 5, "Index x should be in [0, 4]");
}

#[test]
fn test_inference_order_independence() {
    // Test that inference works regardless of variable/constraint order
    
    // Model 1: constraints in one order
    let mut m1 = Model::default();
    let x1 = m1.new_var(int(i32::MIN), int(i32::MAX));
    ge(&mut m1, x1, int(0));
    lt(&mut m1, x1, int(100));
    let sol1 = m1.solve().expect("Model 1 should solve");
    
    // Model 2: same constraints, different order
    let mut m2 = Model::default();
    let x2 = m2.new_var(int(i32::MIN), int(i32::MAX));
    lt(&mut m2, x2, int(100));
    ge(&mut m2, x2, int(0));
    let sol2 = m2.solve().expect("Model 2 should solve");
    
    // Both should find valid solutions
    let v1 = sol1.get_int(x1);
    let v2 = sol2.get_int(x2);
    assert!(v1 >= 0 && v1 < 100, "Model 1 solution should be in [0, 100)");
    assert!(v2 >= 0 && v2 < 100, "Model 2 solution should be in [0, 100)");
}

#[test]
fn test_inference_multiple_unbounded_vars() {
    // Test that inference handles multiple unbounded variables
    let mut m = Model::default();
    let x = m.new_var(int(i32::MIN), int(i32::MAX));
    let y = m.new_var(int(i32::MIN), int(i32::MAX));
    
    // Add constraints
    ge(&mut m, x, int(0));
    lt(&mut m, x, int(10));
    ge(&mut m, y, int(5));
    lt(&mut m, y, int(15));
    
    // Add relational constraint: x < y
    lt(&mut m, x, y);
    
    let solution = m.solve().expect("Should find solution");
    let x_val = solution.get_int(x);
    let y_val = solution.get_int(y);
    assert!(x_val >= 0 && x_val < 10, "x should be in [0, 10)");
    assert!(y_val >= 5 && y_val < 15, "y should be in [5, 15)");
    assert!(x_val < y_val, "Should satisfy x < y");
}

#[test]
fn test_inference_alldifferent_bounds() {
    // Test that AllDifferent provides weak bounds
    let mut m = Model::default();
    let x = m.new_var(int(i32::MIN), int(i32::MAX));
    let y = m.new_var(int(0), int(5));
    let z = m.new_var(int(3), int(8));
    
    // Add AllDifferent constraint
    alldiff(&mut m, &[x, y, z]);
    
    let solution = m.solve().expect("Should find solution");
    let x_val = solution.get_int(x);
    let y_val = solution.get_int(y);
    let z_val = solution.get_int(z);
    // All values should be different
    assert!(x_val != y_val && x_val != z_val && y_val != z_val, 
            "All values should be different");
}

#[test]
fn test_inference_no_constraints() {
    // Test fallback when unbounded variable has no relevant constraints
    let mut m = Model::default();
    let x = m.new_var(int(i32::MIN), int(i32::MAX));
    
    // No constraints on x - should use fallback bounds
    let solution = m.solve().expect("Should find solution");
    // Should find some solution using fallback bounds - just check it doesn't panic
    let _val = solution.get_int(x);
}

#[test]
fn test_inference_conflicting_bounds() {
    // Test that conflicting constraints are detected
    let mut m = Model::default();
    let x = m.new_var(int(i32::MIN), int(i32::MAX));
    
    // Add conflicting constraints: x < 5 and x > 10
    lt(&mut m, x, int(5));
    gt(&mut m, x, int(10));
    
    // Should detect conflict during validation or search
    let result = m.solve();
    assert!(result.is_err(), "Should detect conflicting constraints");
}


#[test]
fn test_inference_tight_bounds() {
    // Test that inference computes tight bounds from multiple constraints
    let mut m = Model::default();
    let x = m.new_var(int(i32::MIN), int(i32::MAX));
    
    // Multiple constraints that progressively tighten bounds
    ge(&mut m, x, int(-100));
    lt(&mut m, x, int(200));
    ge(&mut m, x, int(0));  // Tighter lower bound
    lt(&mut m, x, int(50));  // Tighter upper bound
    ge(&mut m, x, int(10));  // Even tighter lower bound
    
    let solution = m.solve().expect("Should find solution");
    let val = solution.get_int(x);
    assert!(val >= 10 && val < 50, "Should use tightest bounds [10, 50)");
}

#[test]
fn test_inference_preserves_existing_bounds() {
    // Test that inference doesn't weaken existing bounds
    let mut m = Model::default();
    // Create variable with some initial bounds (but still wide)
    let x = m.new_var(int(i32::MIN), int(1000));
    
    // Add tighter constraint
    lt(&mut m, x, int(50));
    
    let solution = m.solve().expect("Should find solution");
    let val = solution.get_int(x);
    assert!(val < 50, "Should respect tighter constraint");
}
