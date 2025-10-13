//! Unit tests for AllDiff constraint coverage improvement
//! This file improves coverage for src/constraints/props/alldiff.rs (45.45% -> higher)

use selen::prelude::*;

#[test]
fn test_alldiff_small_2_vars() {
    let mut m = Model::default();
    let x = m.int(1, 2);
    let y = m.int(1, 2);
    
    m.alldiff(&[x, y]);
    
    let solution = m.solve().expect("Should find solution");
    assert_ne!(solution[x], solution[y]);
}

#[test]
fn test_alldiff_small_3_vars_tight() {
    let mut m = Model::default();
    let x = m.int(1, 3);
    let y = m.int(1, 3);
    let z = m.int(1, 3);
    
    m.alldiff(&[x, y, z]);
    
    let solution = m.solve().expect("Should find solution");
    let vals: std::collections::HashSet<_> = [x, y, z].iter()
        .map(|&v| solution[v].as_int().unwrap())
        .collect();
    assert_eq!(vals.len(), 3);
}

#[test]
fn test_alldiff_with_assigned_vars() {
    let mut m = Model::default();
    let x = m.int(1, 1);  // x = 1
    let y = m.int(1, 5);
    let z = m.int(1, 5);
    
    m.alldiff(&[x, y, z]);
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[x].as_int().unwrap(), 1);
    assert_ne!(solution[y].as_int().unwrap(), 1);
    assert_ne!(solution[z].as_int().unwrap(), 1);
    assert_ne!(solution[y], solution[z]);
}

#[test]
fn test_alldiff_propagation() {
    let mut m = Model::default();
    let x = m.int(5, 5);  // x = 5
    let y = m.int(5, 6);  // y can be 5 or 6
    let z = m.int(5, 6);  // z can be 5 or 6
    
    m.alldiff(&[x, y, z]);
    
    // Since x=5, y and z must both be 6, but they must be different
    // This should fail
    let result = m.solve();
    assert!(result.is_err());
}

#[test]
fn test_alldiff_large_domain() {
    let mut m = Model::default();
    let vars: Vec<_> = (0..5).map(|_| m.int(1, 100)).collect();
    
    m.alldiff(&vars);
    
    let solution = m.solve().expect("Should find solution");
    let vals: std::collections::HashSet<_> = vars.iter()
        .map(|&v| solution[v].as_int().unwrap())
        .collect();
    assert_eq!(vals.len(), 5);
}

#[test]
fn test_alldiff_exact_domain_size() {
    let mut m = Model::default();
    let vars: Vec<_> = (0..5).map(|_| m.int(1, 5)).collect();
    
    m.alldiff(&vars);
    
    let solution = m.solve().expect("Should find solution");
    let vals: std::collections::HashSet<_> = vars.iter()
        .map(|&v| solution[v].as_int().unwrap())
        .collect();
    assert_eq!(vals.len(), 5);
    // Should use all values from 1 to 5
    for i in 1..=5 {
        assert!(vals.contains(&i));
    }
}

#[test]
fn test_alldiff_negative_values() {
    let mut m = Model::default();
    let x = m.int(-10, -5);
    let y = m.int(-10, -5);
    let z = m.int(-10, -5);
    
    m.alldiff(&[x, y, z]);
    
    let solution = m.solve().expect("Should find solution");
    let vals: std::collections::HashSet<_> = [x, y, z].iter()
        .map(|&v| solution[v].as_int().unwrap())
        .collect();
    assert_eq!(vals.len(), 3);
}

#[test]
fn test_alldiff_mixed_positive_negative() {
    let mut m = Model::default();
    let x = m.int(-2, 2);
    let y = m.int(-2, 2);
    let z = m.int(-2, 2);
    
    m.alldiff(&[x, y, z]);
    
    let solution = m.solve().expect("Should find solution");
    let vals: std::collections::HashSet<_> = [x, y, z].iter()
        .map(|&v| solution[v].as_int().unwrap())
        .collect();
    assert_eq!(vals.len(), 3);
}

#[test]
fn test_alldiff_with_equality_constraint() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    let z = m.int(1, 10);
    let w = m.int(5, 5);  // w = 5
    
    m.new(x.eq(w));  // Force x = 5
    m.alldiff(&[x, y, z]);
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[x].as_int().unwrap(), 5);
    assert_ne!(solution[y].as_int().unwrap(), 5);
    assert_ne!(solution[z].as_int().unwrap(), 5);
    assert_ne!(solution[y], solution[z]);
}

#[test]
fn test_alldiff_many_vars() {
    let mut m = Model::default();
    let vars: Vec<_> = (0..20).map(|_| m.int(1, 20)).collect();
    
    m.alldiff(&vars);
    
    let solution = m.solve().expect("Should find solution");
    let vals: std::collections::HashSet<_> = vars.iter()
        .map(|&v| solution[v].as_int().unwrap())
        .collect();
    assert_eq!(vals.len(), 20);
}

#[test]
fn test_alldiff_two_groups() {
    let mut m = Model::default();
    let group1: Vec<_> = (0..3).map(|_| m.int(1, 10)).collect();
    let group2: Vec<_> = (0..3).map(|_| m.int(1, 10)).collect();
    
    m.alldiff(&group1);
    m.alldiff(&group2);
    
    let solution = m.solve().expect("Should find solution");
    
    // Each group should have all different values
    let vals1: std::collections::HashSet<_> = group1.iter()
        .map(|&v| solution[v].as_int().unwrap())
        .collect();
    let vals2: std::collections::HashSet<_> = group2.iter()
        .map(|&v| solution[v].as_int().unwrap())
        .collect();
    
    assert_eq!(vals1.len(), 3);
    assert_eq!(vals2.len(), 3);
}

#[test]
fn test_alldiff_overlapping_constraints() {
    let mut m = Model::default();
    let x = m.int(1, 5);
    let y = m.int(1, 5);
    let z = m.int(1, 5);
    let w = m.int(1, 5);
    
    m.alldiff(&[x, y, z]);
    m.alldiff(&[y, z, w]);
    
    let solution = m.solve().expect("Should find solution");
    
    // First alldiff ensures x, y, z are all different
    let x_val = solution[x].as_int().unwrap();
    let y_val = solution[y].as_int().unwrap();
    let z_val = solution[z].as_int().unwrap();
    let w_val = solution[w].as_int().unwrap();
    
    assert_ne!(x_val, y_val);
    assert_ne!(x_val, z_val);
    assert_ne!(y_val, z_val);
    
    // Second alldiff ensures y, z, w are all different
    assert_ne!(y_val, w_val);
    assert_ne!(z_val, w_val);
}

#[test]
fn test_alldiff_sparse_domain() {
    let mut m = Model::default();
    // Create variables with gaps in their domains
    let x = m.int(1, 3);
    let y = m.int(1, 3);
    let z = m.int(1, 3);
    
    // Force gaps by excluding middle value
    m.new(x.ne(2));
    
    m.alldiff(&[x, y, z]);
    
    let solution = m.solve().expect("Should find solution");
    let vals: std::collections::HashSet<_> = [x, y, z].iter()
        .map(|&v| solution[v].as_int().unwrap())
        .collect();
    
    assert_eq!(vals.len(), 3);
    assert_ne!(solution[x].as_int().unwrap(), 2);
}

#[test]
fn test_alldiff_sequential_assignment() {
    let mut m = Model::default();
    let v1 = m.int(1, 1);  // v1 = 1
    let v2 = m.int(2, 2);  // v2 = 2
    let v3 = m.int(3, 3);  // v3 = 3
    
    m.alldiff(&[v1, v2, v3]);
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[v1].as_int().unwrap(), 1);
    assert_eq!(solution[v2].as_int().unwrap(), 2);
    assert_eq!(solution[v3].as_int().unwrap(), 3);
}
