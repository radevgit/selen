//! Tests for bool() constant function

use selen::prelude::*;

#[test]
fn test_bool_true_constant() {
    let mut m = Model::default();
    let b = m.bool();
    
    // Use bool(true) to constrain boolean to true
    m.new(b.eq(bool(true)));
    
    let result = m.solve().unwrap();
    assert_eq!(result.get_bool(b).unwrap(), true);
}

#[test]
fn test_bool_false_constant() {
    let mut m = Model::default();
    let b = m.bool();
    
    // Use bool(false) to constrain boolean to false
    m.new(b.eq(bool(false)));
    
    let result = m.solve().unwrap();
    assert_eq!(result.get_bool(b).unwrap(), false);
}

#[test]
fn test_bool_not_equal() {
    let mut m = Model::default();
    let b = m.bool();
    
    // Use bool(false) with inequality
    m.new(b.ne(bool(false)));
    
    let result = m.solve().unwrap();
    assert_eq!(result.get_bool(b).unwrap(), true);
}

#[test]
fn test_bool_multiple_constraints() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    
    // b1 must be true, b2 must be false
    m.new(b1.eq(bool(true)));
    m.new(b2.eq(bool(false)));
    
    let result = m.solve().unwrap();
    assert_eq!(result.get_bool(b1).unwrap(), true);
    assert_eq!(result.get_bool(b2).unwrap(), false);
}

#[test]
fn test_bool_with_arithmetic() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    let sum = m.int(0, 2);
    
    // sum = b1 + b2, and sum must equal 1
    m.new(b1.add(b2).eq(sum));
    m.new(sum.eq(bool(true))); // sum == 1
    
    let result = m.solve().unwrap();
    let v1 = result.get_bool(b1).unwrap() as i32;
    let v2 = result.get_bool(b2).unwrap() as i32;
    assert_eq!(v1 + v2, 1);
}

#[test]
fn test_bool_constant_values() {
    // Test that bool() creates the correct Val values
    assert_eq!(bool(true), Val::ValI(1));
    assert_eq!(bool(false), Val::ValI(0));
}

#[test]
fn test_bool_consistency_with_int() {
    let mut m = Model::default();
    let b = m.bool();
    
    // These should be equivalent
    m.new(b.eq(bool(true)));
    
    let result = m.solve().unwrap();
    assert_eq!(result.get_int(b), 1);
    
    // Reset and try with int(1)
    let mut m2 = Model::default();
    let b2 = m2.bool();
    m2.new(b2.eq(int(1)));
    
    let result2 = m2.solve().unwrap();
    assert_eq!(result2.get_int(b2), 1);
    
    // Both should produce the same result
    assert_eq!(result.get_int(b), result2.get_int(b2));
}

#[test]
fn test_bool_with_comparison() {
    let mut m = Model::default();
    let b = m.bool();
    
    // b > false means b must be true
    m.new(b.gt(bool(false)));
    
    let result = m.solve().unwrap();
    assert_eq!(result.get_bool(b).unwrap(), true);
}

#[test]
fn test_bool_less_than() {
    let mut m = Model::default();
    let b = m.bool();
    
    // b < true means b must be false (since 0 < 1)
    m.new(b.lt(bool(true)));
    
    let result = m.solve().unwrap();
    assert_eq!(result.get_bool(b).unwrap(), false);
}

#[test]
fn test_bool_alldiff_with_constants() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    
    // Both must be different
    m.alldiff(&[b1, b2]);
    
    // b1 must be true
    m.new(b1.eq(bool(true)));
    
    let result = m.solve().unwrap();
    
    // b1 is true, b2 must be false (because alldiff)
    assert_eq!(result.get_bool(b1).unwrap(), true);
    assert_eq!(result.get_bool(b2).unwrap(), false);
}

#[test]
fn test_try_get_bool_success() {
    let mut m = Model::default();
    let b = m.bool();
    m.new(b.eq(bool(true)));
    
    let result = m.solve().unwrap();
    assert_eq!(result.try_get_bool(b).unwrap(), true);
}

#[test]
fn test_try_get_bool_failure() {
    let mut m = Model::default();
    let x = m.int(5, 10);
    m.new(x.eq(7));
    
    let result = m.solve().unwrap();
    // Trying to get a boolean from an integer variable with value 7 should fail
    assert!(result.try_get_bool(x).is_err());
}

#[test]
fn test_as_bool_success() {
    let mut m = Model::default();
    let b = m.bool();
    m.new(b.eq(bool(false)));
    
    let result = m.solve().unwrap();
    assert_eq!(result.as_bool(b), Some(false));
}

#[test]
fn test_as_bool_failure() {
    let mut m = Model::default();
    let x = m.int(2, 5);
    m.new(x.eq(3));
    
    let result = m.solve().unwrap();
    // as_bool should return None for non-boolean values
    assert_eq!(result.as_bool(x), None);
}

#[test]
fn test_get_bool_error_on_invalid() {
    let mut m = Model::default();
    let x = m.int(5, 10);
    m.new(x.eq(7));
    
    let result = m.solve().unwrap();
    // This should return an error
    assert!(result.get_bool(x).is_err());
}
