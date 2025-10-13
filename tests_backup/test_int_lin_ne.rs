//! Tests for int_lin_ne constraint

use selen::prelude::*;

#[test]
fn test_int_lin_ne_basic() {
    let mut m = Model::default();
    
    let x = m.int(1, 5);
    let y = m.int(1, 5);
    
    // x + y ≠ 6
    m.lin_ne(&[1, 1], &[x, y], 6);
    
    let solution = m.solve().expect("Should find solution");
    
    if let (Val::ValI(x_val), Val::ValI(y_val)) = (solution[x], solution[y]) {
        // Sum must not equal 6
        assert_ne!(x_val + y_val, 6);
    } else {
        panic!("Expected integer values");
    }
}

#[test]
fn test_int_lin_ne_with_coefficients() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // 2*x + 3*y ≠ 12
    m.lin_ne(&[2, 3], &[x, y], 12);
    
    let solution = m.solve().expect("Should find solution");
    
    if let (Val::ValI(x_val), Val::ValI(y_val)) = (solution[x], solution[y]) {
        // Weighted sum must not equal 12
        assert_ne!(2 * x_val + 3 * y_val, 12);
    } else {
        panic!("Expected integer values");
    }
}

#[test]
fn test_int_lin_ne_negative_coefficients() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // 2*x - y ≠ 5
    m.lin_ne(&[2, -1], &[x, y], 5);
    
    let solution = m.solve().expect("Should find solution");
    
    if let (Val::ValI(x_val), Val::ValI(y_val)) = (solution[x], solution[y]) {
        assert_ne!(2 * x_val - y_val, 5);
    } else {
        panic!("Expected integer values");
    }
}

#[test]
fn test_int_lin_ne_three_variables() {
    let mut m = Model::default();
    
    let x = m.int(1, 3);
    let y = m.int(1, 3);
    let z = m.int(1, 3);
    
    // x + y + z ≠ 6
    m.lin_ne(&[1, 1, 1], &[x, y, z], 6);
    
    let solution = m.solve().expect("Should find solution");
    
    if let (Val::ValI(x_val), Val::ValI(y_val), Val::ValI(z_val)) = 
           (solution[x], solution[y], solution[z]) {
        assert_ne!(x_val + y_val + z_val, 6);
    } else {
        panic!("Expected integer values");
    }
}

#[test]
fn test_int_lin_ne_forced_solution() {
    let mut m = Model::default();
    
    let x = m.int(3, 3);  // Fixed to 3
    let y = m.int(1, 5);
    
    // 2*x + y ≠ 10  =>  6 + y ≠ 10  =>  y ≠ 4
    m.lin_ne(&[2, 1], &[x, y], 10);
    
    let solution = m.solve().expect("Should find solution");
    
    if let Val::ValI(y_val) = solution[y] {
        // y must not be 4
        assert_ne!(y_val, 4);
    } else {
        panic!("Expected integer value");
    }
}

#[test]
fn test_int_lin_ne_empty_array() {
    let mut m = Model::default();
    
    // Empty sum ≠ 0 (this should be unsatisfiable)
    m.lin_ne(&[], &[], 0);
    
    let result = m.solve();
    assert!(result.is_err(), "Empty sum = 0, so ≠ 0 should be unsatisfiable");
}

#[test]
fn test_int_lin_ne_empty_array_satisfiable() {
    let mut m = Model::default();
    
    // Empty sum ≠ 5 (this should be satisfiable since 0 ≠ 5)
    m.lin_ne(&[], &[], 5);
    
    let result = m.solve();
    assert!(result.is_ok(), "Empty sum = 0, so ≠ 5 should be satisfiable");
}

#[test]
fn test_int_lin_ne_large_coefficients() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // 100*x + 50*y ≠ 250
    m.lin_ne(&[100, 50], &[x, y], 250);
    
    let solution = m.solve().expect("Should find solution");
    
    if let (Val::ValI(x_val), Val::ValI(y_val)) = (solution[x], solution[y]) {
        assert_ne!(100 * x_val + 50 * y_val, 250);
    } else {
        panic!("Expected integer values");
    }
}

#[test]
fn test_int_lin_ne_combined_with_eq() {
    let mut m = Model::default();
    
    let x = m.int(1, 5);
    let y = m.int(1, 5);
    let z = m.int(1, 5);
    
    // x + y + z = 10
    m.lin_eq(&[1, 1, 1], &[x, y, z], 10);
    
    // 2*x + y ≠ 8
    m.lin_ne(&[2, 1], &[x, y], 8);
    
    let solution = m.solve().expect("Should find solution");
    
    if let (Val::ValI(x_val), Val::ValI(y_val), Val::ValI(z_val)) = 
           (solution[x], solution[y], solution[z]) {
        assert_eq!(x_val + y_val + z_val, 10);
        assert_ne!(2 * x_val + y_val, 8);
    } else {
        panic!("Expected integer values");
    }
}

#[test]
fn test_int_lin_ne_zero_coefficients() {
    let mut m = Model::default();
    
    let x = m.int(1, 5);
    let y = m.int(1, 5);
    let z = m.int(1, 5);
    
    // 2*x + 0*y + 3*z ≠ 11
    m.lin_ne(&[2, 0, 3], &[x, y, z], 11);
    
    let solution = m.solve().expect("Should find solution");
    
    if let (Val::ValI(x_val), Val::ValI(z_val)) = (solution[x], solution[z]) {
        // y doesn't matter since its coefficient is 0
        assert_ne!(2 * x_val + 3 * z_val, 11);
    } else {
        panic!("Expected integer values");
    }
}

#[test]
fn test_int_lin_ne_single_variable() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    
    // 3*x ≠ 9  =>  x ≠ 3
    m.lin_ne(&[3], &[x], 9);
    
    let solution = m.solve().expect("Should find solution");
    
    if let Val::ValI(x_val) = solution[x] {
        assert_ne!(x_val, 3);
    } else {
        panic!("Expected integer value");
    }
}

#[test]
fn test_int_lin_ne_unsatisfiable() {
    let mut m = Model::default();
    
    let x = m.int(5, 5);  // Fixed to 5
    let y = m.int(3, 3);  // Fixed to 3
    
    // x + y ≠ 8, but x=5 and y=3, so x+y=8
    m.lin_ne(&[1, 1], &[x, y], 8);
    
    let result = m.solve();
    assert!(result.is_err(), "Should be unsatisfiable");
}

#[test]
fn test_int_lin_ne_propagation() {
    let mut m = Model::default();
    
    let x = m.int(1, 3);
    let y = m.int(1, 1);  // Fixed to 1
    
    // x + y ≠ 3  =>  x + 1 ≠ 3  =>  x ≠ 2
    m.lin_ne(&[1, 1], &[x, y], 3);
    
    let solution = m.solve().expect("Should find solution");
    
    if let Val::ValI(x_val) = solution[x] {
        // x should be either 1 or 3, but not 2
        assert_ne!(x_val, 2);
        assert!(x_val == 1 || x_val == 3);
    } else {
        panic!("Expected integer value");
    }
}
