//! Unit tests for between constraint
//! This file improves coverage for src/constraints/props/between.rs (48.91% -> higher)

use selen::prelude::*;

#[test]
fn test_between_basic_constraint() {
    let mut m = Model::default();
    let lower = m.int(1, 5);
    let middle = m.int(1, 10);
    let upper = m.int(5, 15);
    
    // Add between constraint: lower <= middle <= upper
    m.props.between_constraint(lower, middle, upper);
    
    let solution = m.solve().expect("Should find solution");
    let l = solution[lower].as_int().unwrap();
    let mid = solution[middle].as_int().unwrap();
    let u = solution[upper].as_int().unwrap();
    
    assert!(l <= mid);
    assert!(mid <= u);
}

#[test]
fn test_between_propagation_lower_bound() {
    let mut m = Model::default();
    let lower = m.int(10, 15);  // Lower bound is 10
    let middle = m.int(1, 20);
    let upper = m.int(15, 25);
    
    m.props.between_constraint(lower, middle, upper);
    
    let solution = m.solve().expect("Should find solution");
    let l = solution[lower].as_int().unwrap();
    let mid = solution[middle].as_int().unwrap();
    let u = solution[upper].as_int().unwrap();
    
    // Middle should be >= 10 (from lower bound)
    assert!(mid >= 10);
    assert!(l <= mid);
    assert!(mid <= u);
}

#[test]
fn test_between_propagation_upper_bound() {
    let mut m = Model::default();
    let lower = m.int(1, 10);
    let middle = m.int(1, 20);
    let upper = m.int(5, 10);  // Upper bound is 10
    
    m.props.between_constraint(lower, middle, upper);
    
    let solution = m.solve().expect("Should find solution");
    let l = solution[lower].as_int().unwrap();
    let mid = solution[middle].as_int().unwrap();
    let u = solution[upper].as_int().unwrap();
    
    // Middle should be <= 10 (from upper bound)
    assert!(mid <= 10);
    assert!(l <= mid);
    assert!(mid <= u);
}

#[test]
fn test_between_middle_bound_propagation() {
    let mut m = Model::default();
    let lower = m.int(1, 20);
    let middle = m.int(10, 15);  // Middle is constrained
    let upper = m.int(1, 25);
    
    m.props.between_constraint(lower, middle, upper);
    
    let solution = m.solve().expect("Should find solution");
    let l = solution[lower].as_int().unwrap();
    let mid = solution[middle].as_int().unwrap();
    let u = solution[upper].as_int().unwrap();
    
    // Lower should be <= 15 (from middle max)
    assert!(l <= 15);
    // Upper should be >= 10 (from middle min)
    assert!(u >= 10);
    assert!(l <= mid);
    assert!(mid <= u);
}

#[test]
fn test_between_all_same_value() {
    let mut m = Model::default();
    let lower = m.int(5, 5);   // All fixed to 5
    let middle = m.int(5, 5);
    let upper = m.int(5, 5);
    
    m.props.between_constraint(lower, middle, upper);
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[lower].as_int().unwrap(), 5);
    assert_eq!(solution[middle].as_int().unwrap(), 5);
    assert_eq!(solution[upper].as_int().unwrap(), 5);
}

#[test]
fn test_between_tight_constraints() {
    let mut m = Model::default();
    let lower = m.int(5, 7);
    let middle = m.int(6, 6);  // Middle fixed to 6
    let upper = m.int(6, 10);
    
    m.props.between_constraint(lower, middle, upper);
    
    let solution = m.solve().expect("Should find solution");
    let l = solution[lower].as_int().unwrap();
    let mid = solution[middle].as_int().unwrap();
    let u = solution[upper].as_int().unwrap();
    
    assert_eq!(mid, 6);
    assert!(l <= 6);
    assert!(u >= 6);
}

#[test]
fn test_between_infeasible_lower_greater_than_upper() {
    let mut m = Model::default();
    let lower = m.int(10, 15);
    let middle = m.int(1, 20);
    let upper = m.int(1, 5);  // Upper max < lower min
    
    m.props.between_constraint(lower, middle, upper);
    
    // Should have no solution since lower > upper is impossible
    let result = m.solve();
    assert!(result.is_err());
}

#[test]
fn test_between_with_negative_values() {
    let mut m = Model::default();
    let lower = m.int(-10, -5);
    let middle = m.int(-15, 0);
    let upper = m.int(-3, 5);
    
    m.props.between_constraint(lower, middle, upper);
    
    let solution = m.solve().expect("Should find solution");
    let l = solution[lower].as_int().unwrap();
    let mid = solution[middle].as_int().unwrap();
    let u = solution[upper].as_int().unwrap();
    
    assert!(l <= mid);
    assert!(mid <= u);
}

#[test]
fn test_between_boundary_values() {
    let mut m = Model::default();
    let lower = m.int(1, 3);
    let middle = m.int(3, 5);
    let upper = m.int(5, 7);
    
    // Overlapping at boundaries
    m.props.between_constraint(lower, middle, upper);
    
    let solution = m.solve().expect("Should find solution");
    let l = solution[lower].as_int().unwrap();
    let mid = solution[middle].as_int().unwrap();
    let u = solution[upper].as_int().unwrap();
    
    assert!(l <= mid);
    assert!(mid <= u);
}
