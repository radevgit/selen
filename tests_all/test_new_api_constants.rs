//! Test the new unified API with explicit constant types

use selen::prelude::*;

#[test]
fn test_eq_with_int_constant() {
    let mut model = Model::default();
    let x = model.int(0, 10);
    
    // x == 5 (using explicit int() constructor)
    eq(&mut model, x, int(5));
    
    let solution = model.solve().unwrap();
    let x_val = solution.get_int(x);
    
    assert_eq!(x_val, 5);
}

#[test]
fn test_eq_with_float_constant() {
    let mut model = Model::default();
    let x = model.float(0.0, 10.0);
    
    // x == 3.14 (using explicit float() constructor)
    eq(&mut model, x, float(3.14));
    
    let solution = model.solve().unwrap();
    let x_val = solution.get_float(x);
    
    assert!((x_val - 3.14).abs() < 0.01);
}

#[test]
fn test_comparison_with_constants() {
    let mut model = Model::default();
    let x = model.int(0, 100);
    let y = model.int(0, 100);
    
    // x >= 10
    ge(&mut model, x, int(10));
    
    // y <= 50
    le(&mut model, y, int(50));
    
    // x + y == 60
    eq(&mut model, add(x, y), int(60));
    
    let solution = model.solve().unwrap();
    let x_val = solution.get_int(x);
    let y_val = solution.get_int(y);
    
    assert!(x_val >= 10);
    assert!(y_val <= 50);
    assert_eq!(x_val + y_val, 60);
}

#[test]
fn test_expression_with_constant() {
    let mut model = Model::default();
    let x = model.int(1, 10);
    let y = model.int(1, 10);
    
    // 2*x + 3 == y
    eq(&mut model, add(mul(x, int(2)), int(3)), y);
    
    let solution = model.solve().unwrap();
    let x_val = solution.get_int(x);
    let y_val = solution.get_int(y);
    
    assert_eq!(2 * x_val + 3, y_val);
}

#[test]
fn test_ne_with_constant() {
    let mut model = Model::default();
    let x = model.int(0, 5);
    
    // x != 3
    ne(&mut model, x, int(3));
    
    let solution = model.solve().unwrap();
    let x_val = solution.get_int(x);
    
    assert_ne!(x_val, 3);
}

#[test]
fn test_range_with_constants() {
    let mut model = Model::default();
    let x = model.int(0, 100);
    
    // 20 <= x <= 30
    ge(&mut model, x, int(20));
    le(&mut model, x, int(30));
    
    let solution = model.solve().unwrap();
    let x_val = solution.get_int(x);
    
    assert!(x_val >= 20);
    assert!(x_val <= 30);
}

// Note: Complex float expressions may have issues with current propagator implementation
// Use runtime API directly for complex float arithmetic:
// model.new(x.mul(2.5).add(1.5).eq(10.0));
