//! Comprehensive tests for non-reified integer linear constraints
//! Tests for int_lin_eq, int_lin_le, int_lin_ne

use selen::prelude::*;

// ============================================================================
// int_lin_eq - Equality constraints
// ============================================================================

#[test]
fn test_int_lin_eq_simple_propagation() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // x + y = 7
    m.lin_eq(&[1, 1], &[x, y], 7);
    
    // Force x = 3, should propagate y = 4
    m.new(x.eq(3));
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[x], Val::ValI(3));
    assert_eq!(solution[y], Val::ValI(4));
}

#[test]
fn test_int_lin_eq_bounds_propagation() {
    let mut m = Model::default();
    let x = m.int(0, 100);
    let y = m.int(0, 100);
    let z = m.int(0, 100);
    
    // x + y + z = 20
    m.lin_eq(&[1, 1, 1], &[x, y, z], 20);
    
    // Fix x = 5, y = 8
    m.new(x.eq(5));
    m.new(y.eq(8));
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[z], Val::ValI(7)); // Must be 7 to satisfy equation
}

#[test]
fn test_int_lin_eq_large_coefficients() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // 100x + 50y = 600
    m.lin_eq(&[100, 50], &[x, y], 600);
    
    let solution = m.solve().expect("Should find solution");
    if let (Val::ValI(x_val), Val::ValI(y_val)) = (solution[x], solution[y]) {
        assert_eq!(100 * x_val + 50 * y_val, 600);
    }
}

#[test]
fn test_int_lin_eq_negative_constant() {
    let mut m = Model::default();
    let x = m.int(-10, 10);
    let y = m.int(-10, 10);
    
    // x + y = -5
    m.lin_eq(&[1, 1], &[x, y], -5);
    
    let solution = m.solve().expect("Should find solution");
    if let (Val::ValI(x_val), Val::ValI(y_val)) = (solution[x], solution[y]) {
        assert_eq!(x_val + y_val, -5);
    }
}

#[test]
fn test_int_lin_eq_zero_coefficient() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let z = m.int(0, 10);
    
    // x + 0*y + z = 10
    m.lin_eq(&[1, 0, 1], &[x, y, z], 10);
    
    m.new(x.eq(4));
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[z], Val::ValI(6));
    // y can be anything since coefficient is 0
}

#[test]
fn test_int_lin_eq_unsatisfiable() {
    let mut m = Model::default();
    let x = m.int(0, 5);
    let y = m.int(0, 5);
    
    // x + y = 20 (impossible with given domains)
    m.lin_eq(&[1, 1], &[x, y], 20);
    
    let result = m.solve();
    assert!(result.is_err(), "Should be unsatisfiable");
}

#[test]
fn test_int_lin_eq_four_variables() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    let z = m.int(1, 10);
    let w = m.int(1, 10);
    
    // x + y + z + w = 20
    m.lin_eq(&[1, 1, 1, 1], &[x, y, z, w], 20);
    
    let solution = m.solve().expect("Should find solution");
    if let (Val::ValI(xv), Val::ValI(yv), Val::ValI(zv), Val::ValI(wv)) = 
        (solution[x], solution[y], solution[z], solution[w]) {
        assert_eq!(xv + yv + zv + wv, 20);
    }
}

#[test]
fn test_int_lin_eq_single_variable() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    
    // 3*x = 12
    m.lin_eq(&[3], &[x], 12);
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[x], Val::ValI(4));
}

// ============================================================================
// int_lin_le - Less-than-or-equal constraints
// ============================================================================

#[test]
fn test_int_lin_le_at_boundary() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // x + y ≤ 10
    m.lin_le(&[1, 1], &[x, y], 10);
    
    m.new(x.eq(5));
    m.new(y.eq(5));
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[x], Val::ValI(5));
    assert_eq!(solution[y], Val::ValI(5));
}

#[test]
fn test_int_lin_le_below_boundary() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // x + y ≤ 10
    m.lin_le(&[1, 1], &[x, y], 10);
    
    m.new(x.eq(3));
    m.new(y.eq(4));
    
    let solution = m.solve().expect("Should find solution");
    if let (Val::ValI(xv), Val::ValI(yv)) = (solution[x], solution[y]) {
        assert!(xv + yv <= 10);
    }
}

#[test]
fn test_int_lin_le_propagates_upper_bounds() {
    let mut m = Model::default();
    let x = m.int(0, 100);
    let y = m.int(0, 100);
    let z = m.int(0, 100);
    
    // x + y + z ≤ 20
    m.lin_le(&[1, 1, 1], &[x, y, z], 20);
    
    m.new(x.eq(10));
    m.new(y.eq(8));
    
    let solution = m.solve().expect("Should find solution");
    if let Val::ValI(zv) = solution[z] {
        assert!(zv <= 2); // z can be at most 2
    }
}

#[test]
fn test_int_lin_le_with_negative_coefficients() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // x - y ≤ 5
    m.lin_le(&[1, -1], &[x, y], 5);
    
    m.new(x.eq(8));
    
    let solution = m.solve().expect("Should find solution");
    if let (Val::ValI(xv), Val::ValI(yv)) = (solution[x], solution[y]) {
        assert!(xv - yv <= 5);
    }
}

#[test]
fn test_int_lin_le_large_positive_constant() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // x + y ≤ 1000 (always satisfied)
    m.lin_le(&[1, 1], &[x, y], 1000);
    
    let solution = m.solve().expect("Should find solution");
    
    // Verify the constraint is satisfied
    if let (Val::ValI(xv), Val::ValI(yv)) = (solution[x], solution[y]) {
        assert!(xv + yv <= 1000, "Constraint x + y ≤ 1000 should be satisfied");
        // Also check domains are respected
        assert!(xv >= 0 && xv <= 10);
        assert!(yv >= 0 && yv <= 10);
    }
}

#[test]
fn test_int_lin_le_unsatisfiable() {
    let mut m = Model::default();
    let x = m.int(5, 10);
    let y = m.int(5, 10);
    
    // x + y ≤ 8 (impossible since min is 10)
    m.lin_le(&[1, 1], &[x, y], 8);
    
    let result = m.solve();
    assert!(result.is_err(), "Should be unsatisfiable");
}

#[test]
fn test_int_lin_le_zero_constant() {
    let mut m = Model::default();
    let x = m.int(-5, 5);
    let y = m.int(-5, 5);
    
    // x + y ≤ 0
    m.lin_le(&[1, 1], &[x, y], 0);
    
    let solution = m.solve().expect("Should find solution");
    if let (Val::ValI(xv), Val::ValI(yv)) = (solution[x], solution[y]) {
        assert!(xv + yv <= 0);
    }
}

// ============================================================================
// int_lin_ne - Not-equal constraints
// ============================================================================

#[test]
fn test_int_lin_ne_excludes_value() {
    let mut m = Model::default();
    let x = m.int(1, 3);
    let y = m.int(1, 3);
    
    // x + y ≠ 4
    m.lin_ne(&[1, 1], &[x, y], 4);
    
    m.new(x.eq(2));
    
    let solution = m.solve().expect("Should find solution");
    assert_ne!(solution[y], Val::ValI(2)); // y can't be 2 (would make sum = 4)
}

#[test]
fn test_int_lin_ne_with_coefficients() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // 2x + 3y ≠ 12
    m.lin_ne(&[2, 3], &[x, y], 12);
    
    let solution = m.solve().expect("Should find solution");
    if let (Val::ValI(xv), Val::ValI(yv)) = (solution[x], solution[y]) {
        assert_ne!(2 * xv + 3 * yv, 12);
    }
}

#[test]
fn test_int_lin_ne_propagation() {
    let mut m = Model::default();
    let x = m.int(5, 5); // Fixed to 5
    let y = m.int(3, 3); // Fixed to 3
    
    // x + y ≠ 8 (but x=5, y=3 makes sum=8, so unsatisfiable)
    m.lin_ne(&[1, 1], &[x, y], 8);
    
    let result = m.solve();
    assert!(result.is_err(), "Should be unsatisfiable");
}

#[test]
fn test_int_lin_ne_multiple_solutions() {
    let mut m = Model::default();
    let x = m.int(0, 5);
    let y = m.int(0, 5);
    
    // x + y ≠ 10 (should have many solutions)
    m.lin_ne(&[1, 1], &[x, y], 10);
    
    let solution = m.solve().expect("Should find solution");
    if let (Val::ValI(xv), Val::ValI(yv)) = (solution[x], solution[y]) {
        assert_ne!(xv + yv, 10);
    }
}

#[test]
fn test_int_lin_ne_negative_constant() {
    let mut m = Model::default();
    let x = m.int(-10, 10);
    let y = m.int(-10, 10);
    
    // x + y ≠ -5
    m.lin_ne(&[1, 1], &[x, y], -5);
    
    let solution = m.solve().expect("Should find solution");
    if let (Val::ValI(xv), Val::ValI(yv)) = (solution[x], solution[y]) {
        assert_ne!(xv + yv, -5);
    }
}

#[test]
fn test_int_lin_ne_three_variables() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let z = m.int(0, 10);
    
    // x + y + z ≠ 15
    m.lin_ne(&[1, 1, 1], &[x, y, z], 15);
    
    let solution = m.solve().expect("Should find solution");
    if let (Val::ValI(xv), Val::ValI(yv), Val::ValI(zv)) = 
        (solution[x], solution[y], solution[z]) {
        assert_ne!(xv + yv + zv, 15);
    }
}

// ============================================================================
// Combined constraints
// ============================================================================

#[test]
fn test_multiple_int_lin_constraints() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let z = m.int(0, 10);
    
    // x + y + z = 15
    m.lin_eq(&[1, 1, 1], &[x, y, z], 15);
    
    // 2x + y ≤ 20
    m.lin_le(&[2, 1, 0], &[x, y, z], 20);
    
    // x + 2y ≠ 10
    m.lin_ne(&[1, 2, 0], &[x, y, z], 10);
    
    let solution = m.solve().expect("Should find solution");
    if let (Val::ValI(xv), Val::ValI(yv), Val::ValI(zv)) = 
        (solution[x], solution[y], solution[z]) {
        assert_eq!(xv + yv + zv, 15);
        assert!(2 * xv + yv <= 20);
        assert_ne!(xv + 2 * yv, 10);
    }
}

#[test]
fn test_int_lin_eq_and_le_interaction() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // x + y = 10
    m.lin_eq(&[1, 1], &[x, y], 10);
    
    // x ≤ 6 (via linear constraint)
    m.lin_le(&[1, 0], &[x, y], 6);
    
    let solution = m.solve().expect("Should find solution");
    if let (Val::ValI(xv), Val::ValI(yv)) = (solution[x], solution[y]) {
        assert_eq!(xv + yv, 10);
        assert!(xv <= 6);
        assert!(yv >= 4); // Must be at least 4
    }
}

#[test]
fn test_int_lin_overconstrained_satisfiable() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // x + y = 10
    m.lin_eq(&[1, 1], &[x, y], 10);
    
    // x + y ≤ 15 (redundant, already satisfied by equality)
    m.lin_le(&[1, 1], &[x, y], 15);
    
    let solution = m.solve().expect("Should find solution");
    if let (Val::ValI(xv), Val::ValI(yv)) = (solution[x], solution[y]) {
        assert_eq!(xv + yv, 10);
    }
}

#[test]
fn test_int_lin_overconstrained_unsatisfiable() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // x + y = 10
    m.lin_eq(&[1, 1], &[x, y], 10);
    
    // x + y ≤ 8 (conflicts with equality)
    m.lin_le(&[1, 1], &[x, y], 8);
    
    let result = m.solve();
    assert!(result.is_err(), "Should be unsatisfiable due to conflict");
}
