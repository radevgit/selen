//! Unit tests for global constraints (alldiff, alleq) and cardinality constraints
//! This file improves coverage for src/constraints/api/global.rs and src/constraints/props/cardinality.rs

use selen::prelude::*;

// ============================================================================
// AllDiff Tests
// ============================================================================

#[test]
fn test_alldiff_basic() {
    let mut m = Model::default();
    let x = m.int(1, 3);
    let y = m.int(1, 3);
    let z = m.int(1, 3);
    
    m.alldiff(&[x, y, z]);
    
    let solution = m.solve().expect("Should find solution");
    let vals: std::collections::HashSet<_> = [x, y, z].iter()
        .map(|&v| solution[v].as_int().unwrap())
        .collect();
    
    assert_eq!(vals.len(), 3); // All different values
}

#[test]
fn test_alldiff_impossible() {
    let mut m = Model::default();
    let x = m.int(1, 2);
    let y = m.int(1, 2);
    let z = m.int(1, 2);
    let w = m.int(1, 2);
    
    // 4 variables in domain of size 2 - impossible to all be different
    m.alldiff(&[x, y, z, w]);
    
    let result = m.solve();
    assert!(result.is_err());
}

#[test]
fn test_alldiff_single_variable() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    
    m.alldiff(&[x]);
    
    let solution = m.solve().expect("Should find solution");
    assert!(solution[x].as_int().unwrap() >= 1);
    assert!(solution[x].as_int().unwrap() <= 10);
}

#[test]
fn test_alldiff_empty_array() {
    let mut m = Model::default();
    
    m.alldiff(&[]);
    
    // Should still solve (empty constraint is trivially satisfied)
    let _solution = m.solve().expect("Should find solution");
}

#[test]
fn test_alldiff_with_larger_array() {
    let mut m = Model::default();
    let vars: Vec<_> = (0..10).map(|_| m.int(1, 10)).collect();
    
    m.alldiff(&vars);
    
    let solution = m.solve().expect("Should find solution");
    let vals: std::collections::HashSet<_> = vars.iter()
        .map(|&v| solution[v].as_int().unwrap())
        .collect();
    
    assert_eq!(vals.len(), 10); // All different
}

// ============================================================================
// AllEq Tests
// ============================================================================

#[test]
fn test_alleq_basic() {
    let mut m = Model::default();
    let x = m.int(1, 5);
    let y = m.int(1, 5);
    let z = m.int(1, 5);
    
    m.alleq(&[x, y, z]);
    
    let solution = m.solve().expect("Should find solution");
    let x_val = solution[x].as_int().unwrap();
    let y_val = solution[y].as_int().unwrap();
    let z_val = solution[z].as_int().unwrap();
    
    assert_eq!(x_val, y_val);
    assert_eq!(y_val, z_val);
}

#[test]
fn test_alleq_with_fixed_value() {
    let mut m = Model::default();
    let x = m.int(1, 5);
    let y = m.int(3, 3);  // Fixed to 3
    let z = m.int(1, 5);
    
    m.alleq(&[x, y, z]);
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[x].as_int().unwrap(), 3);
    assert_eq!(solution[y].as_int().unwrap(), 3);
    assert_eq!(solution[z].as_int().unwrap(), 3);
}

#[test]
fn test_alleq_conflicting_fixed_values() {
    let mut m = Model::default();
    let x = m.int(2, 2);  // Fixed to 2
    let y = m.int(3, 3);  // Fixed to 3
    
    m.alleq(&[x, y]);
    
    // Should have no solution since they must be equal but are fixed to different values
    let result = m.solve();
    assert!(result.is_err());
}

#[test]
fn test_alleq_single_variable() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    
    m.alleq(&[x]);
    
    let solution = m.solve().expect("Should find solution");
    assert!(solution[x].as_int().unwrap() >= 1);
}

// ============================================================================
// Cardinality: at_least Tests
// ============================================================================

#[test]
fn test_at_least_basic() {
    let mut m = Model::default();
    let x = m.int(0, 1);
    let y = m.int(0, 1);
    let z = m.int(0, 1);
    
    // At least 2 variables must equal 1
    m.at_least(&[x, y, z], 1, 2);
    
    let solution = m.solve().expect("Should find solution");
    let count = [x, y, z].iter()
        .filter(|&&v| solution[v].as_int().unwrap() == 1)
        .count();
    
    assert!(count >= 2);
}

#[test]
fn test_at_least_all_required() {
    let mut m = Model::default();
    let x = m.int(0, 1);
    let y = m.int(0, 1);
    let z = m.int(0, 1);
    
    // At least 3 variables must equal 1 (all of them)
    m.at_least(&[x, y, z], 1, 3);
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[x].as_int().unwrap(), 1);
    assert_eq!(solution[y].as_int().unwrap(), 1);
    assert_eq!(solution[z].as_int().unwrap(), 1);
}

#[test]
fn test_at_least_impossible() {
    let mut m = Model::default();
    let x = m.int(0, 1);
    let y = m.int(0, 1);
    
    // Require 3 variables equal 1, but only have 2 variables
    m.at_least(&[x, y], 1, 3);
    
    let result = m.solve();
    assert!(result.is_err());
}

#[test]
fn test_at_least_propagation_forces_values() {
    let mut m = Model::default();
    let x = m.int(0, 1);
    let y = m.int(0, 1);
    let z = m.int(1, 1);  // Already fixed to 1
    
    // At least 3 must equal 1, so x and y must be forced to 1
    m.at_least(&[x, y, z], 1, 3);
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[x].as_int().unwrap(), 1);
    assert_eq!(solution[y].as_int().unwrap(), 1);
    assert_eq!(solution[z].as_int().unwrap(), 1);
}

#[test]
fn test_at_least_with_negative_target() {
    let mut m = Model::default();
    let x = m.int(-5, 5);
    let y = m.int(-5, 5);
    let z = m.int(-5, 5);
    
    // At least 2 variables must equal -2
    m.at_least(&[x, y, z], -2, 2);
    
    let solution = m.solve().expect("Should find solution");
    let count = [x, y, z].iter()
        .filter(|&&v| solution[v].as_int().unwrap() == -2)
        .count();
    
    assert!(count >= 2);
}

// ============================================================================
// Cardinality: at_most Tests
// ============================================================================

#[test]
fn test_at_most_basic() {
    let mut m = Model::default();
    let x = m.int(0, 1);
    let y = m.int(0, 1);
    let z = m.int(0, 1);
    
    // At most 1 variable can equal 1
    m.at_most(&[x, y, z], 1, 1);
    
    let solution = m.solve().expect("Should find solution");
    let count = [x, y, z].iter()
        .filter(|&&v| solution[v].as_int().unwrap() == 1)
        .count();
    
    assert!(count <= 1);
}

#[test]
fn test_at_most_zero() {
    let mut m = Model::default();
    let x = m.int(0, 1);
    let y = m.int(0, 1);
    
    // No variable can equal 1
    m.at_most(&[x, y], 1, 0);
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[x].as_int().unwrap(), 0);
    assert_eq!(solution[y].as_int().unwrap(), 0);
}

#[test]
fn test_at_most_already_violated() {
    let mut m = Model::default();
    let x = m.int(1, 1);  // Fixed to 1
    let y = m.int(1, 1);  // Fixed to 1
    
    // At most 1 variable can equal 1, but both are fixed to 1
    m.at_most(&[x, y], 1, 1);
    
    let result = m.solve();
    assert!(result.is_err());
}

#[test]
fn test_at_most_forces_exclusion() {
    let mut m = Model::default();
    let x = m.int(0, 1);
    let y = m.int(0, 1);
    let z = m.int(1, 1);  // Already fixed to 1
    
    // At most 1 can equal 1, so x and y must be 0
    m.at_most(&[x, y, z], 1, 1);
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[x].as_int().unwrap(), 0);
    assert_eq!(solution[y].as_int().unwrap(), 0);
    assert_eq!(solution[z].as_int().unwrap(), 1);
}

// ============================================================================
// Cardinality: exactly Tests
// ============================================================================

#[test]
fn test_exactly_basic() {
    let mut m = Model::default();
    let x = m.int(0, 1);
    let y = m.int(0, 1);
    let z = m.int(0, 1);
    
    // Exactly 2 variables must equal 1
    m.exactly(&[x, y, z], 1, 2);
    
    let solution = m.solve().expect("Should find solution");
    let count = [x, y, z].iter()
        .filter(|&&v| solution[v].as_int().unwrap() == 1)
        .count();
    
    assert_eq!(count, 2);
}

#[test]
fn test_exactly_one() {
    let mut m = Model::default();
    let x = m.int(0, 1);
    let y = m.int(0, 1);
    let z = m.int(0, 1);
    
    // Exactly 1 variable must equal 1
    m.exactly(&[x, y, z], 1, 1);
    
    let solution = m.solve().expect("Should find solution");
    let count = [x, y, z].iter()
        .filter(|&&v| solution[v].as_int().unwrap() == 1)
        .count();
    
    assert_eq!(count, 1);
}

#[test]
fn test_exactly_impossible() {
    let mut m = Model::default();
    let x = m.int(0, 1);
    let y = m.int(0, 1);
    
    // Exactly 3 variables must equal 1, but only have 2
    m.exactly(&[x, y], 1, 3);
    
    let result = m.solve();
    assert!(result.is_err());
}

#[test]
fn test_exactly_with_wider_domains() {
    let mut m = Model::default();
    let x = m.int(0, 5);
    let y = m.int(0, 5);
    let z = m.int(0, 5);
    let w = m.int(0, 5);
    
    // Exactly 2 variables must equal 3
    m.exactly(&[x, y, z, w], 3, 2);
    
    let solution = m.solve().expect("Should find solution");
    let count = [x, y, z, w].iter()
        .filter(|&&v| solution[v].as_int().unwrap() == 3)
        .count();
    
    assert_eq!(count, 2);
}

#[test]
fn test_exactly_zero() {
    let mut m = Model::default();
    let x = m.int(0, 1);
    let y = m.int(0, 1);
    
    // Exactly 0 variables must equal 1 (same as at_most 0)
    m.exactly(&[x, y], 1, 0);
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[x].as_int().unwrap(), 0);
    assert_eq!(solution[y].as_int().unwrap(), 0);
}
