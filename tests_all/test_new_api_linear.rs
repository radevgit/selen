//! Test the new unified API with linear constraints

use selen::prelude::*;

#[test]
fn test_lin_eq_integer() {
    let mut model = Model::default();
    let x = model.int(0, 10);
    let y = model.int(0, 10);
    let z = model.int(0, 10);
    
    // 2*x + 3*y + z == 10
    lin_eq(&mut model, &[2, 3, 1], &[x, y, z], 10);
    
    let solution = model.solve().unwrap();
    let x_val = solution.get_int(x);
    let y_val = solution.get_int(y);
    let z_val = solution.get_int(z);
    
    assert_eq!(2 * x_val + 3 * y_val + z_val, 10);
}

#[test]
fn test_lin_le_integer() {
    let mut model = Model::default();
    let x = model.int(0, 10);
    let y = model.int(0, 10);
    
    // x + y <= 5
    lin_le(&mut model, &[1, 1], &[x, y], 5);
    
    let solution = model.solve().unwrap();
    let x_val = solution.get_int(x);
    let y_val = solution.get_int(y);
    
    assert!(x_val + y_val <= 5);
}

#[test]
fn test_lin_eq_float() {
    let mut model = Model::default();
    let x = model.float(0.0, 10.0);
    let y = model.float(0.0, 10.0);
    
    // 2.5*x + 1.5*y == 10.0
    lin_eq(&mut model, &[2.5, 1.5], &[x, y], 10.0);
    
    let solution = model.solve().unwrap();
    let x_val = solution.get_float(x);
    let y_val = solution.get_float(y);
    
    let sum = 2.5 * x_val + 1.5 * y_val;
    assert!((sum - 10.0).abs() < 0.01, "Expected ~10.0, got {}", sum);
}

#[test]
fn test_lin_eq_reif() {
    let mut model = Model::default();
    let x = model.int(0, 10);
    let y = model.int(0, 10);
    let b = model.bool();
    
    // b <=> (x + y == 5)
    lin_eq_reif(&mut model, &[1, 1], &[x, y], 5, b);
    
    // Force b to be true
    let one = model.int(1, 1);
    eq(&mut model, b, one);
    
    let solution = model.solve().unwrap();
    let x_val = solution.get_int(x);
    let y_val = solution.get_int(y);
    let b_val = solution.get_int(b);
    
    assert_eq!(b_val, 1);
    assert_eq!(x_val + y_val, 5);
}

#[test]
fn test_generic_linear_with_comparison() {
    let mut model = Model::default();
    let x = model.int(1, 10);
    let y = model.int(1, 10);
    let z = model.int(1, 10);
    
    // 2*x + y == z
    lin_eq(&mut model, &[2, 1, -1], &[x, y, z], 0);
    
    // x > 3
    let three = model.int(3, 3);
    gt(&mut model, x, three);
    
    let solution = model.solve().unwrap();
    let x_val = solution.get_int(x);
    let y_val = solution.get_int(y);
    let z_val = solution.get_int(z);
    
    assert!(x_val > 3);
    assert_eq!(2 * x_val + y_val, z_val);
}
