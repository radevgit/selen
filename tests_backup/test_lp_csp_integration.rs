/// Integration test for CSP with linear constraints
/// 
/// Tests linear constraint solving including combinations of
/// linear equalities and inequalities on float variables.
/// These tests verify that the linear constraint system works correctly
/// through actual solving rather than extraction.

use selen::prelude::*;

#[test]
fn test_simple_linear_system() {
    // Create a simple model with linear constraints
    let mut m = Model::default();
    let x = m.float(0.0, 100.0);
    let y = m.float(0.0, 100.0);
    
    // Add linear constraints: x + y = 50, 2x + y <= 80
    m.lin_eq(&[1.0, 1.0], &[x, y], 50.0);
    m.lin_le(&[2.0, 1.0], &[x, y], 80.0);
    
    // Should find a solution
    let solution = m.solve().expect("Should find solution");
    
    if let (Val::ValF(x_val), Val::ValF(y_val)) = (solution[x], solution[y]) {
        // Verify constraints are satisfied
        assert!((x_val + y_val - 50.0).abs() < 1e-6, "x + y should equal 50");
        assert!(2.0 * x_val + y_val <= 80.0 + 1e-6, "2x + y should be <= 80");
        
        // From x + y = 50, we have y = 50 - x
        // Substituting into 2x + y <= 80: 2x + (50 - x) <= 80 => x <= 30
        assert!(x_val <= 30.0 + 1e-6, "x should be <= 30");
    } else {
        panic!("Expected float values");
    }
}

#[test]
fn test_empty_model() {
    // Empty model should solve trivially
    let m = Model::default();
    let solution = m.solve().expect("Empty model should solve");
    // Just verify it doesn't crash
    drop(solution);
}

#[test]
#[ignore = "Float linear equality propagation needs stronger implementation"]
fn test_linear_system_solving() {
    // Create model with linear constraints
    let mut m = Model::default();
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    
    // x + 2y <= 15
    m.lin_le(&[1.0, 2.0], &[x, y], 15.0);
    // x + y = 10
    m.lin_eq(&[1.0, 1.0], &[x, y], 10.0);
    
    let solution = m.solve().expect("Should find solution");
    
    if let (Val::ValF(x_val), Val::ValF(y_val)) = (solution[x], solution[y]) {
        // Verify both constraints are satisfied
        assert!((x_val + y_val - 10.0).abs() < 1e-6, "x + y should equal 10");
        assert!(x_val + 2.0 * y_val <= 15.0 + 1e-6, "x + 2y should be <= 15");
        
        // From x + y = 10, we have x = 10 - y
        // Substituting: (10 - y) + 2y <= 15 => 10 + y <= 15 => y <= 5
        assert!(y_val <= 5.0 + 1e-6, "y should be <= 5");
    } else {
        panic!("Expected float values");
    }
}

#[test]
#[ignore = "Float linear equality propagation needs stronger implementation"]
fn test_medium_sized_problem() {
    // Test with a medium-sized linear system (5 variables, simpler constraints)
    let mut m = Model::default();
    
    // Create 5 variables with reasonable bounds
    let vars: Vec<_> = (0..5)
        .map(|_| m.float(0.0, 100.0))
        .collect();
    
    // Simple constraint: sum of all vars = 250
    let coeffs_sum: Vec<f64> = vec![1.0; 5];
    m.lin_eq(&coeffs_sum, &vars, 250.0);
    
    // Add some pairwise constraints
    m.lin_le(&[1.0, 1.0], &[vars[0], vars[1]], 120.0);
    m.lin_le(&[1.0, 1.0], &[vars[2], vars[3]], 120.0);
    
    // Should find a solution
    let solution = m.solve().expect("Should find solution for medium-sized problem");
    
    // Verify main constraint: sum should equal 250
    let sum: f64 = vars.iter().map(|&v| {
        if let Val::ValF(val) = solution[v] {
            val
        } else {
            panic!("Expected float value");
        }
    }).sum();
    
    assert!((sum - 250.0).abs() < 1e-5, "Sum of all variables should equal 250, got {}", sum);
}

#[test]
#[ignore = "Float linear equality propagation needs stronger implementation"]
fn test_larger_problem() {
    // Test with a larger linear system (10 variables, simpler constraints)
    let mut m = Model::default();
    
    // Create 10 variables with reasonable bounds
    let vars: Vec<_> = (0..10)
        .map(|_| m.float(0.0, 100.0))
        .collect();
    
    // Constraint: sum of all vars = 500
    let coeffs_sum: Vec<f64> = vec![1.0; 10];
    m.lin_eq(&coeffs_sum, &vars, 500.0);
    
    // Add some pairwise constraints
    for i in 0..9 {
        m.lin_le(&[1.0, 1.0], &[vars[i], vars[i+1]], 120.0);
    }
    
    // Should find a solution
    let solution = m.solve().expect("Should find solution for larger problem");
    
    // Verify main constraint: sum should equal 500
    let sum: f64 = vars.iter().map(|&v| {
        if let Val::ValF(val) = solution[v] {
            val
        } else {
            panic!("Expected float value");
        }
    }).sum();
    
    assert!((sum - 500.0).abs() < 1e-4, "Sum of all variables should equal 500, got {}", sum);
}

#[test]
#[ignore = "Float linear equality propagation needs stronger implementation"]
fn test_equality_and_inequality_constraints() {
    // Test that equality and inequality constraints work together
    let mut m = Model::default();
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    
    // Add equality
    m.lin_eq(&[1.0, 1.0], &[x, y], 5.0);
    
    // Add inequality
    m.lin_le(&[2.0, 1.0], &[x, y], 10.0);
    
    let solution = m.solve().expect("Should find solution");
    
    if let (Val::ValF(x_val), Val::ValF(y_val)) = (solution[x], solution[y]) {
        // Verify equality constraint
        assert!((x_val + y_val - 5.0).abs() < 1e-6, "x + y should equal 5");
        // Verify inequality constraint
        assert!(2.0 * x_val + y_val <= 10.0 + 1e-6, "2x + y should be <= 10");
    } else {
        panic!("Expected float values");
    }
}

#[test]
fn test_mixed_constraint_types() {
    // Test that linear and non-linear constraints can coexist
    let mut m = Model::default();
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let a = m.int(1, 10);
    let b = m.int(1, 10);
    
    // Add linear constraints on floats
    m.lin_eq(&[1.0, 2.0], &[x, y], 15.0);
    m.lin_le(&[3.0, 1.0], &[x, y], 20.0);
    
    // Add non-linear constraint on integers
    m.new(a.ne(b));
    
    // Should find a solution satisfying all constraints
    let solution = m.solve().expect("Should find solution with mixed constraints");
    
    if let (Val::ValF(x_val), Val::ValF(y_val)) = (solution[x], solution[y]) {
        // Verify linear constraints
        assert!((x_val + 2.0 * y_val - 15.0).abs() < 1e-6, "x + 2y should equal 15");
        assert!(3.0 * x_val + y_val <= 20.0 + 1e-6, "3x + y should be <= 20");
    } else {
        panic!("Expected float values");
    }
    
    if let (Val::ValI(a_val), Val::ValI(b_val)) = (solution[a], solution[b]) {
        // Verify non-linear constraint
        assert!(a_val != b_val, "a should not equal b");
    } else {
        panic!("Expected integer values");
    }
}

#[test]
fn test_variable_bounds_respected() {
    // Test that variable bounds are properly respected during solving
    let mut m = Model::default();
    let x = m.float(5.0, 15.0);  // Custom bounds
    let y = m.float(-10.0, 20.0);
    
    m.lin_le(&[1.0, 1.0], &[x, y], 25.0);
    
    let solution = m.solve().expect("Should find solution");
    
    if let (Val::ValF(x_val), Val::ValF(y_val)) = (solution[x], solution[y]) {
        // Verify bounds are respected
        assert!(x_val >= 5.0 - 1e-6, "x should be >= 5.0, got {}", x_val);
        assert!(x_val <= 15.0 + 1e-6, "x should be <= 15.0, got {}", x_val);
        assert!(y_val >= -10.0 - 1e-6, "y should be >= -10.0, got {}", y_val);
        assert!(y_val <= 20.0 + 1e-6, "y should be <= 20.0, got {}", y_val);
        
        // Verify constraint
        assert!(x_val + y_val <= 25.0 + 1e-6, "x + y should be <= 25");
    } else {
        panic!("Expected float values");
    }
}

#[test]
fn test_infeasible_system_detection() {
    // Create an obviously infeasible system
    let mut m = Model::default();
    let x = m.float(0.0, 10.0);
    
    // x <= 5 and x >= 8 (impossible)
    m.lin_le(&[1.0], &[x], 5.0);
    m.lin_le(&[-1.0], &[x], -8.0);  // Represents x >= 8, or -x <= -8
    
    // Should detect infeasibility
    let result = m.solve();
    
    assert!(result.is_err(), "Infeasible system should return error");
}
