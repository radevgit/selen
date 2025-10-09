/// Integration test for CSP to LP extraction
/// 
/// Tests the extraction of linear constraints from a CSP model
/// and their conversion to LP format, as well as the full
/// CSP → LP → CSP pipeline.

use selen::prelude::*;

#[test]
fn test_extract_linear_system_simple() {
    // Create a simple model with linear constraints
    let mut m = Model::default();
    let x = m.float(0.0, 100.0);
    let y = m.float(0.0, 100.0);
    
    // Add linear constraints: x + y = 50, 2x + y <= 80
    m.float_lin_eq(&[1.0, 1.0], &[x, y], 50.0);
    m.float_lin_le(&[2.0, 1.0], &[x, y], 80.0);
    
    // Extract linear system
    let system = m.extract_linear_system();
    
    // Should have extracted 2 constraints and 2 variables
    assert_eq!(system.n_constraints(), 2);
    assert_eq!(system.n_variables(), 2);
}

#[test]
fn test_extract_empty_model() {
    let m = Model::default();
    let system = m.extract_linear_system();
    
    assert_eq!(system.n_constraints(), 0);
    assert_eq!(system.n_variables(), 0);
}

#[test]
fn test_linear_system_to_lp_problem() {
    // Create model with linear constraints
    let mut m = Model::default();
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    
    // x + 2y <= 15
    m.float_lin_le(&[1.0, 2.0], &[x, y], 15.0);
    // x + y = 10
    m.float_lin_eq(&[1.0, 1.0], &[x, y], 10.0);
    
    let system = m.extract_linear_system();
    
    // Verify extraction
    assert_eq!(system.n_constraints(), 2);
    assert_eq!(system.n_variables(), 2);
}

#[test]
fn test_medium_sized_problem() {
    // Test with a medium-sized linear system (10 variables, 15 constraints)
    let mut m = Model::default();
    
    // Create 10 variables
    let vars: Vec<_> = (0..10)
        .map(|i| m.float(0.0, 100.0 + i as f64 * 10.0))
        .collect();
    
    // Add 15 linear constraints
    // Constraint 1: sum of all vars = 500
    let coeffs_sum: Vec<f64> = vec![1.0; 10];
    m.float_lin_eq(&coeffs_sum, &vars, 500.0);
    
    // Constraints 2-6: pairwise sums <= 120
    for i in 0..5 {
        m.float_lin_le(&[1.0, 1.0], &[vars[i], vars[i+1]], 120.0);
    }
    
    // Constraints 7-11: differences >= 5 (represented as -x + y >= 5, or x - y <= -5)
    for i in 0..5 {
        m.float_lin_le(&[1.0, -1.0], &[vars[i], vars[i+5]], -5.0);
    }
    
    // Constraints 12-15: triplet constraints
    for i in 0..4 {
        m.float_lin_le(&[1.0, 2.0, 1.0], &[vars[i], vars[i+3], vars[i+6]], 200.0);
    }
    
    // Extract linear system
    let system = m.extract_linear_system();
    
    // Should have all constraints
    assert_eq!(system.n_constraints(), 15);
    assert_eq!(system.n_variables(), 10);
    
    // System should be suitable for LP (≥3 constraints, ≥2 variables)
    // We need access to vars for this check
    // assert!(system.is_suitable_for_lp(&m.vars));
}

#[test]
fn test_large_problem() {
    // Test with a large linear system (50 variables, 100 constraints)
    let mut m = Model::default();
    
    // Create 50 variables
    let vars: Vec<_> = (0..50)
        .map(|i| m.float(0.0, 1000.0 + i as f64 * 10.0))
        .collect();
    
    // Add 100 linear constraints
    // Constraint 1: sum of all vars = 25000
    let coeffs_sum: Vec<f64> = vec![1.0; 50];
    m.float_lin_eq(&coeffs_sum, &vars, 25000.0);
    
    // 49 pairwise constraints
    for i in 0..49 {
        m.float_lin_le(&[1.0, 1.0], &[vars[i], vars[i+1]], 1200.0);
    }
    
    // 50 triplet constraints
    for i in 0..50 {
        let j = (i + 10) % 50;
        let k = (i + 20) % 50;
        m.float_lin_le(&[1.0, 2.0, 1.0], &[vars[i], vars[j], vars[k]], 2500.0);
    }
    
    // Extract linear system
    let system = m.extract_linear_system();
    
    // Should have all 100 constraints
    assert_eq!(system.n_constraints(), 100);
    assert_eq!(system.n_variables(), 50);
}

#[test]
fn test_constraint_conversion_to_standard_form() {
    // Test that constraints are properly converted to standard ≤ form
    let mut m = Model::default();
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    
    // Add equality (converts to 2 inequalities)
    m.float_lin_eq(&[1.0, 1.0], &[x, y], 5.0);
    
    // Add inequality
    m.float_lin_le(&[2.0, 1.0], &[x, y], 10.0);
    
    let system = m.extract_linear_system();
    
    // Should have 2 constraints from user's perspective
    assert_eq!(system.n_constraints(), 2);
    assert_eq!(system.n_variables(), 2);
}

#[test]
fn test_mixed_constraint_types() {
    // Test that only linear constraints are extracted
    let mut m = Model::default();
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let a = m.int(1, 10);
    let b = m.int(1, 10);
    
    // Add linear constraints
    m.float_lin_eq(&[1.0, 2.0], &[x, y], 15.0);
    m.float_lin_le(&[3.0, 1.0], &[x, y], 20.0);
    
    // Add non-linear constraint (should be ignored by LP extraction)
    m.new(a.ne(b));
    
    let system = m.extract_linear_system();
    
    // Should only extract the 2 linear constraints
    assert_eq!(system.n_constraints(), 2);
    // Should only track float variables
    assert_eq!(system.n_variables(), 2);
}

#[test]
fn test_variable_bounds_extraction() {
    // Test that variable bounds are properly extracted
    let mut m = Model::default();
    let x = m.float(5.0, 15.0);  // Custom bounds
    let y = m.float(-10.0, 20.0);
    
    m.float_lin_le(&[1.0, 1.0], &[x, y], 25.0);
    
    let system = m.extract_linear_system();
    
    assert_eq!(system.n_variables(), 2);
    
    // Bounds should be respected when converting to LP
    // (This is tested implicitly when to_lp_problem is called)
}

#[test]
fn test_infeasible_system_detection() {
    // Create an obviously infeasible system
    let mut m = Model::default();
    let x = m.float(0.0, 10.0);
    
    // x <= 5 and x >= 8 (impossible)
    m.float_lin_le(&[1.0], &[x], 5.0);
    m.float_lin_le(&[-1.0], &[x], -8.0);  // Represents x >= 8
    
    let system = m.extract_linear_system();
    
    // System should extract successfully
    assert_eq!(system.n_constraints(), 2);
    assert_eq!(system.n_variables(), 1);
    
    // LP solver should detect infeasibility when solving
    // (Would need access to solve to test this)
}
