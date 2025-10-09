//! Tests for linear constraint helpers (FlatZinc integration)

use selen::prelude::*;

#[test]
fn test_int_lin_eq_simple() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // x + y = 7
    m.int_lin_eq(&[1, 1], &[x, y], 7);
    
    // Force x = 3
    m.new(x.eq(3));
    
    let solution = m.solve().expect("Should find solution");
    
    assert_eq!(solution[x], Val::ValI(3));
    assert_eq!(solution[y], Val::ValI(4)); // y must be 4 to make x + y = 7
}

#[test]
fn test_int_lin_eq_with_coefficients() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // 2x + 3y = 12
    m.int_lin_eq(&[2, 3], &[x, y], 12);
    
    // Force x = 3
    m.new(x.eq(3));
    
    let solution = m.solve().expect("Should find solution");
    
    assert_eq!(solution[x], Val::ValI(3));
    assert_eq!(solution[y], Val::ValI(2)); // 2*3 + 3*2 = 6 + 6 = 12
}

#[test]
fn test_int_lin_eq_negative_coefficient() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // 5x - 2y = 6
    m.int_lin_eq(&[5, -2], &[x, y], 6);
    
    // Force x = 4
    m.new(x.eq(4));
    
    let solution = m.solve().expect("Should find solution");
    
    assert_eq!(solution[x], Val::ValI(4));
    assert_eq!(solution[y], Val::ValI(7)); // 5*4 - 2*7 = 20 - 14 = 6
}

#[test]
fn test_int_lin_eq_three_variables() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let z = m.int(0, 10);
    
    // 2x + 3y - z = 10
    m.int_lin_eq(&[2, 3, -1], &[x, y, z], 10);
    
    // Force x = 2 and y = 3
    m.new(x.eq(2));
    m.new(y.eq(3));
    
    let solution = m.solve().expect("Should find solution");
    
    assert_eq!(solution[x], Val::ValI(2));
    assert_eq!(solution[y], Val::ValI(3));
    assert_eq!(solution[z], Val::ValI(3)); // 2*2 + 3*3 - 3 = 4 + 9 - 3 = 10
}

#[test]
fn test_int_lin_le_simple() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // x + y ≤ 10
    m.int_lin_le(&[1, 1], &[x, y], 10);
    
    // Force x = 8
    m.new(x.eq(8));
    
    let solution = m.solve().expect("Should find solution");
    
    assert_eq!(solution[x], Val::ValI(8));
    // y must be ≤ 2 to satisfy x + y ≤ 10
    if let Val::ValI(y_val) = solution[y] {
        assert!(y_val <= 2);
    } else {
        panic!("Expected integer value for y");
    }
}

#[test]
fn test_int_lin_le_with_coefficients() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // 2x + 3y ≤ 20
    m.int_lin_le(&[2, 3], &[x, y], 20);
    
    // Force x = 5
    m.new(x.eq(5));
    
    let solution = m.solve().expect("Should find solution");
    
    assert_eq!(solution[x], Val::ValI(5));
    // 2*5 + 3*y ≤ 20 => 10 + 3*y ≤ 20 => 3*y ≤ 10 => y ≤ 3.33 => y ≤ 3
    if let Val::ValI(y_val) = solution[y] {
        assert!(y_val <= 3);
    } else {
        panic!("Expected integer value for y");
    }
}

#[test]
fn test_int_lin_le_negative_coefficient() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // x - y ≤ 5
    m.int_lin_le(&[1, -1], &[x, y], 5);
    
    // Force x = 8
    m.new(x.eq(8));
    
    let solution = m.solve().expect("Should find solution");
    
    assert_eq!(solution[x], Val::ValI(8));
    // 8 - y ≤ 5 => y ≥ 3
    if let Val::ValI(y_val) = solution[y] {
        assert!(y_val >= 3);
    } else {
        panic!("Expected integer value for y");
    }
}

#[test]
fn test_int_lin_eq_multiple_solutions() {
    let mut m = Model::default();
    
    let x = m.int(0, 3);
    let y = m.int(0, 3);
    
    // x + y = 3
    m.int_lin_eq(&[1, 1], &[x, y], 3);
    
    // Just verify that we can find at least one solution
    let solution = m.solve().expect("Should find solution");
    
    if let (Val::ValI(x_val), Val::ValI(y_val)) = (solution[x], solution[y]) {
        assert_eq!(x_val + y_val, 3);
    } else {
        panic!("Expected integer values");
    }
}

#[test]
fn test_int_lin_eq_infeasible() {
    let mut m = Model::default();
    
    let x = m.int(0, 5);
    let y = m.int(0, 5);
    
    // x + y = 20 (impossible with given domains)
    m.int_lin_eq(&[1, 1], &[x, y], 20);
    
    let result = m.solve();
    
    assert!(result.is_err(), "Should not find solution");
}

#[test]
fn test_int_lin_le_boundary() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let z = m.int(0, 10);
    
    // x + y + z ≤ 15
    m.int_lin_le(&[1, 1, 1], &[x, y, z], 15);
    
    // Force x = 5 and y = 5
    m.new(x.eq(5));
    m.new(y.eq(5));
    
    let solution = m.solve().expect("Should find solution");
    
    assert_eq!(solution[x], Val::ValI(5));
    assert_eq!(solution[y], Val::ValI(5));
    // z must be ≤ 5 to satisfy x + y + z ≤ 15
    if let Val::ValI(z_val) = solution[z] {
        assert!(z_val <= 5);
    } else {
        panic!("Expected integer value for z");
    }
}

#[test]
fn test_int_lin_eq_single_variable() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    
    // 3x = 9
    m.int_lin_eq(&[3], &[x], 9);
    
    let solution = m.solve().expect("Should find solution");
    
    assert_eq!(solution[x], Val::ValI(3)); // x must be 3
}

#[test]
fn test_int_lin_le_single_variable() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    
    // 2x ≤ 10
    m.int_lin_le(&[2], &[x], 10);
    
    let solution = m.solve().expect("Should find solution");
    
    // x must be ≤ 5
    if let Val::ValI(x_val) = solution[x] {
        assert!(x_val <= 5);
    } else {
        panic!("Expected integer value for x");
    }
}

#[test]
fn test_int_lin_eq_mismatched_lengths() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // Mismatched lengths should create an unsatisfiable constraint
    m.int_lin_eq(&[1, 2, 3], &[x, y], 10);
    
    let result = m.solve();
    assert!(result.is_err(), "Should not find solution due to mismatched lengths");
}

#[test]
fn test_int_lin_le_mismatched_lengths() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // Mismatched lengths should create an unsatisfiable constraint
    m.int_lin_le(&[1], &[x, y], 10);
    
    let result = m.solve();
    assert!(result.is_err(), "Should not find solution due to mismatched lengths");
}
