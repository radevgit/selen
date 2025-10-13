//! Direct unit tests for neq propagator internals
//! These tests aim to trigger the actual propagation logic in neq.rs

use selen::prelude::*;

// Tests that should trigger actual neq propagation

#[test]
fn test_neq_forces_propagation_with_tight_domain() {
    let mut m = Model::default();
    let x = m.int(1, 2);  // Very tight domain
    let y = m.int(1, 2);
    let z = m.int(1, 2);
    
    // Force x = 1
    m.new(x.eq(1));
    // y != x, so y must be 2
    m.new(y.ne(x));
    // z != x, so z must be 2  
    m.new(z.ne(x));
    // But y != z is impossible now since both must be 2
    m.new(y.ne(z));
    
    let result = m.solve();
    assert!(result.is_err(), "Should be unsatisfiable");
}

#[test]
fn test_neq_propagation_chain() {
    let mut m = Model::default();
    let x = m.int(1, 3);
    let y = m.int(1, 3);
    let z = m.int(1, 3);
    
    // x = 2
    m.new(x.eq(2));
    // y != 2 (due to x)
    m.new(y.ne(x));
    // z = y
    m.new(z.eq(y));
    // This means z != 2 as well
    
    let solution = m.solve().expect("Should find solution");
    let x_val = solution[x].as_int().unwrap();
    let y_val = solution[y].as_int().unwrap();
    let z_val = solution[z].as_int().unwrap();
    
    assert_eq!(x_val, 2);
    assert_ne!(y_val, 2);
    assert_eq!(y_val, z_val);
}

#[test]
fn test_neq_with_domain_reduction() {
    let mut m = Model::default();
    let x = m.int(5, 10);
    let y = m.int(5, 10);
    
    // Force x >= 9 (reduces domain to [9,10])
    m.new(x.ge(9));
    // Force y <= 6 (reduces domain to [5,6])
    m.new(y.le(6));
    // x != y should be trivially satisfied (non-overlapping domains)
    m.new(x.ne(y));
    
    let solution = m.solve().expect("Should find solution");
    assert_ne!(solution[x], solution[y]);
}

#[test]
fn test_neq_singleton_propagation() {
    let mut m = Model::default();
    let x = m.int(5, 5);  // x = 5 (singleton)
    let y = m.int(4, 6);   // y can be 4, 5, or 6
    
    // y != 5 should be propagated
    m.new(y.ne(x));
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[x].as_int().unwrap(), 5);
    let y_val = solution[y].as_int().unwrap();
    assert!(y_val == 4 || y_val == 6);
}

#[test]
fn test_neq_both_singletons_equal() {
    let mut m = Model::default();
    let x = m.int(7, 7);  // x = 7
    let y = m.int(7, 7);  // y = 7
    
    // Both are 7, so x != y should fail
    m.new(x.ne(y));
    
    let result = m.solve();
    assert!(result.is_err(), "Should fail when both singletons are equal");
}

#[test]
fn test_neq_both_singletons_different() {
    let mut m = Model::default();
    let x = m.int(7, 7);  // x = 7
    let y = m.int(8, 8);  // y = 8
    
    // Both assigned to different values
    m.new(x.ne(y));
    
    let solution = m.solve().expect("Should succeed when singletons differ");
    assert_ne!(solution[x], solution[y]);
}

#[test]
fn test_neq_value_at_min_bound() {
    let mut m = Model::default();
    let x = m.int(10, 10);  // x = 10 (at minimum of y's domain)
    let y = m.int(10, 15);   // y starts at 10
    
    // Should exclude 10 from y's domain, forcing y >= 11
    m.new(y.ne(x));
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[x].as_int().unwrap(), 10);
    let y_val = solution[y].as_int().unwrap();
    assert!(y_val >= 11 && y_val <= 15);
}

#[test]
fn test_neq_value_at_max_bound() {
    let mut m = Model::default();
    let x = m.int(15, 15);  // x = 15 (at maximum of y's domain)
    let y = m.int(10, 15);   // y ends at 15
    
    // Should exclude 15 from y's domain, forcing y <= 14
    m.new(y.ne(x));
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[x].as_int().unwrap(), 15);
    let y_val = solution[y].as_int().unwrap();
    assert!(y_val >= 10 && y_val <= 14);
}

#[test]
fn test_neq_with_multiple_propagations() {
    let mut m = Model::default();
    let a = m.int(1, 5);
    let b = m.int(1, 5);
    let c = m.int(1, 5);
    let d = m.int(1, 5);
    
    // Create a chain that forces propagation
    m.new(a.eq(3));       // a = 3
    m.new(b.ne(a));       // b != 3
    m.new(c.eq(b));       // c = b
    m.new(d.ne(c));       // d != c (and therefore d != b)
    m.new(b.eq(2));       // Force b = 2
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[a].as_int().unwrap(), 3);
    assert_eq!(solution[b].as_int().unwrap(), 2);
    assert_eq!(solution[c].as_int().unwrap(), 2);
    assert_ne!(solution[d].as_int().unwrap(), 2);
}

#[test]
fn test_neq_float_singleton_exclusion() {
    let mut m = Model::default();
    let x = m.float(2.5, 2.5);  // x = 2.5
    let y = m.float(2.0, 3.0);
    
    m.new(y.ne(x));
    
    let solution = m.solve().expect("Should find solution");
    let x_val = solution[x].as_float().unwrap();
    let y_val = solution[y].as_float().unwrap();
    
    assert!((x_val - 2.5).abs() < 1e-9);
    assert!((y_val - 2.5).abs() > 1e-5);
}

#[test]
fn test_neq_triggers_on_domain_update() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    let z = m.int(5, 5);
    
    // x = z = 5
    m.new(x.eq(z));
    // y != x, so y != 5
    m.new(y.ne(x));
    // Additional constraint to force y into tight domain
    m.new(y.ge(4));
    m.new(y.le(6));
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[x].as_int().unwrap(), 5);
    let y_val = solution[y].as_int().unwrap();
    assert!(y_val == 4 || y_val == 6);
}

#[test]
fn test_neq_impossible_after_propagation() {
    let mut m = Model::default();
    let x = m.int(5, 6);
    let y = m.int(5, 6);
    let z = m.int(5, 5);
    
    // x = 5
    m.new(x.eq(z));
    // y = 5 (due to some other constraint chain)
    m.new(y.eq(z));
    // But x != y is impossible
    m.new(x.ne(y));
    
    let result = m.solve();
    assert!(result.is_err(), "Should be unsatisfiable");
}

#[test]
fn test_neq_with_negative_bounds() {
    let mut m = Model::default();
    let x = m.int(-5, -5);  // x = -5
    let y = m.int(-6, -4);
    
    m.new(y.ne(x));
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[x].as_int().unwrap(), -5);
    let y_val = solution[y].as_int().unwrap();
    assert!(y_val == -6 || y_val == -4);
}
