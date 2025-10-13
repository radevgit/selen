//! Integration tests for Phase 2 FlatZinc constraints
//!
//! Tests the interaction between:
//! - Linear constraints (int_lin_eq, int_lin_le)
//! - Boolean clause constraints (bool_clause)
//! - Reification constraints (int_eq_reif, int_ne_reif from Phase 1)
//!
//! These tests verify that combinations of constraint types work correctly together,
//! which is essential for real FlatZinc models that mix constraint types.

use selen::prelude::*;

/// Test linear equality with bool_clause
/// x + y + z = 10, and at least one must equal 5
#[test]
fn test_linear_eq_with_bool_clause() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let z = m.int(0, 10);
    
    // x + y + z = 10
    m.lin_eq(&[1, 1, 1], &[x, y, z], 10);
    
    // At least one must equal 5
    let x_eq_5 = m.bool();
    let y_eq_5 = m.bool();
    let z_eq_5 = m.bool();
    
    let five = m.int(5, 5);
    m.eq_reif(x, five, x_eq_5);
    m.eq_reif(y, five, y_eq_5);
    m.eq_reif(z, five, z_eq_5);
    
    m.bool_clause(&[x_eq_5, y_eq_5, z_eq_5], &[]);
    
    let solution = m.solve().expect("Should find solution");
    
    // Verify at least one is 5
    let x_val = if let Val::ValI(v) = solution[x] { v } else { 0 };
    let y_val = if let Val::ValI(v) = solution[y] { v } else { 0 };
    let z_val = if let Val::ValI(v) = solution[z] { v } else { 0 };
    
    assert_eq!(x_val + y_val + z_val, 10);
    assert!(x_val == 5 || y_val == 5 || z_val == 5, "At least one must be 5");
}

/// Test linear inequality with bool_clause
/// 2x + 3y <= 15 AND (x=0 OR y=0)
#[test]
fn test_linear_le_with_bool_clause() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // 2x + 3y <= 15
    m.lin_le(&[2, 3], &[x, y], 15);
    
    // At least one must be zero
    let x_eq_0 = m.bool();
    let y_eq_0 = m.bool();
    
    let zero = m.int(0, 0);
    m.eq_reif(x, zero, x_eq_0);
    m.eq_reif(y, zero, y_eq_0);
    
    m.bool_clause(&[x_eq_0, y_eq_0], &[]);
    
    let solution = m.solve().expect("Should find solution");
    
    let x_val = if let Val::ValI(v) = solution[x] { v } else { 0 };
    let y_val = if let Val::ValI(v) = solution[y] { v } else { 0 };
    
    assert!(2 * x_val + 3 * y_val <= 15);
    assert!(x_val == 0 || y_val == 0, "At least one must be 0");
}

/// Test CNF (multiple bool_clause) with linear equality
/// (a OR b) AND (NOT a OR c) AND (a + b + c = 2)
#[test]
fn test_cnf_with_linear_eq() {
    let mut m = Model::default();
    
    let a = m.bool();
    let b = m.bool();
    let c = m.bool();
    
    // CNF clauses
    m.bool_clause(&[a, b], &[]); // a OR b
    m.bool_clause(&[c], &[a]); // NOT a OR c (a -> c)
    
    // Exactly 2 must be true
    m.lin_eq(&[1, 1, 1], &[a, b, c], 2);
    
    let solution = m.solve().expect("Should find solution");
    
    let a_val = if solution[a] == Val::ValI(1) { 1 } else { 0 };
    let b_val = if solution[b] == Val::ValI(1) { 1 } else { 0 };
    let c_val = if solution[c] == Val::ValI(1) { 1 } else { 0 };
    
    // Verify
    assert!(a_val == 1 || b_val == 1, "a OR b");
    assert!(a_val == 0 || c_val == 1, "a -> c");
    assert_eq!(a_val + b_val + c_val, 2, "Exactly 2 true");
}

/// Test CNF with linear inequality
/// (a OR b OR c) AND (NOT a OR NOT b) AND (a + b + c <= 2)
#[test]
fn test_cnf_with_linear_le() {
    let mut m = Model::default();
    
    let a = m.bool();
    let b = m.bool();
    let c = m.bool();
    
    // CNF clauses
    m.bool_clause(&[a, b, c], &[]); // a OR b OR c (at least one true)
    m.bool_clause(&[], &[a, b]); // NOT a OR NOT b (not both true)
    
    // At most 2 can be true
    m.lin_le(&[1, 1, 1], &[a, b, c], 2);
    
    let solution = m.solve().expect("Should find solution");
    
    let a_val = if solution[a] == Val::ValI(1) { 1 } else { 0 };
    let b_val = if solution[b] == Val::ValI(1) { 1 } else { 0 };
    let c_val = if solution[c] == Val::ValI(1) { 1 } else { 0 };
    
    // Verify
    assert!(a_val + b_val + c_val >= 1, "At least one true");
    assert!(!(a_val == 1 && b_val == 1), "Not both a and b");
    assert!(a_val + b_val + c_val <= 2, "At most 2 true");
}

/// Test reified linear equation
/// b <-> (2x + 3y = 12)
#[test]
fn test_reified_linear_eq() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let b = m.bool();
    
    // Create sum: 2x + 3y
    let x2 = m.mul(x, Val::ValI(2));
    let y3 = m.mul(y, Val::ValI(3));
    let sum = m.sum(&[x2, y3]);
    
    let twelve = m.int(12, 12);
    
    // b <-> (sum = 12)
    m.eq_reif(sum, twelve, b);
    
    // Force b = true
    m.new(b.eq(1));
    
    let solution = m.solve().expect("Should find solution");
    
    let x_val = if let Val::ValI(v) = solution[x] { v } else { 0 };
    let y_val = if let Val::ValI(v) = solution[y] { v } else { 0 };
    
    assert_eq!(solution[b], Val::ValI(1));
    assert_eq!(2 * x_val + 3 * y_val, 12);
}

/// Test reified linear equation (negative case)
/// b <-> (2x + 3y = 12), with b = false
#[test]
fn test_reified_linear_eq_false() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let b = m.bool();
    
    // Create sum: 2x + 3y
    let x2 = m.mul(x, Val::ValI(2));
    let y3 = m.mul(y, Val::ValI(3));
    let sum = m.sum(&[x2, y3]);
    
    let twelve = m.int(12, 12);
    
    // b <-> (sum = 12)
    m.eq_reif(sum, twelve, b);
    
    // Force b = false
    m.new(b.eq(0));
    
    let solution = m.solve().expect("Should find solution");
    
    let x_val = if let Val::ValI(v) = solution[x] { v } else { 0 };
    let y_val = if let Val::ValI(v) = solution[y] { v } else { 0 };
    
    assert_eq!(solution[b], Val::ValI(0));
    assert_ne!(2 * x_val + 3 * y_val, 12, "Should NOT equal 12");
}

/// Test multiple linear constraints with bool_clause
/// x + y = 5, 2x + y = 8, and (x=3 OR y=2)
#[test]
fn test_multiple_linear_with_clause() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // Two linear equations
    m.lin_eq(&[1, 1], &[x, y], 5);
    m.lin_eq(&[2, 1], &[x, y], 8);
    
    // At least one specific value
    let x_eq_3 = m.bool();
    let y_eq_2 = m.bool();
    
    let three = m.int(3, 3);
    let two = m.int(2, 2);
    
    m.eq_reif(x, three, x_eq_3);
    m.eq_reif(y, two, y_eq_2);
    
    m.bool_clause(&[x_eq_3, y_eq_2], &[]);
    
    let solution = m.solve().expect("Should find solution");
    
    // From x + y = 5 and 2x + y = 8, we get x = 3, y = 2
    assert_eq!(solution[x], Val::ValI(3));
    assert_eq!(solution[y], Val::ValI(2));
    assert_eq!(solution[x_eq_3], Val::ValI(1));
    assert_eq!(solution[y_eq_2], Val::ValI(1));
}

/// Test negative coefficients with bool_clause
/// 3x - 2y = 4, and (x=2 OR y=1)
#[test]
fn test_negative_coeff_with_clause() {
    let mut m = Model::default();
    
    let x = m.int(-5, 5);
    let y = m.int(-5, 5);
    
    // 3x - 2y = 4
    m.lin_eq(&[3, -2], &[x, y], 4);
    
    // At least one specific value
    let x_eq_2 = m.bool();
    let y_eq_1 = m.bool();
    
    let two = m.int(2, 2);
    let one = m.int(1, 1);
    
    m.eq_reif(x, two, x_eq_2);
    m.eq_reif(y, one, y_eq_1);
    
    m.bool_clause(&[x_eq_2, y_eq_1], &[]);
    
    let solution = m.solve().expect("Should find solution");
    
    let x_val = if let Val::ValI(v) = solution[x] { v } else { 0 };
    let y_val = if let Val::ValI(v) = solution[y] { v } else { 0 };
    
    assert_eq!(3 * x_val - 2 * y_val, 4);
    assert!(x_val == 2 || y_val == 1);
}

/// Test chained reifications with linear constraints
/// b1 <-> (x + y = 10), b2 <-> (y + z = 15), (b1 OR b2)
#[test]
fn test_chained_reifications() {
    let mut m = Model::default();
    
    let x = m.int(0, 20);
    let y = m.int(0, 20);
    let z = m.int(0, 20);
    let b1 = m.bool();
    let b2 = m.bool();
    
    // Create sums
    let sum_xy = m.sum(&[x, y]);
    let sum_yz = m.sum(&[y, z]);
    
    let ten = m.int(10, 10);
    let fifteen = m.int(15, 15);
    
    // Reify the conditions
    m.eq_reif(sum_xy, ten, b1);
    m.eq_reif(sum_yz, fifteen, b2);
    
    // At least one must be true
    m.bool_clause(&[b1, b2], &[]);
    
    let solution = m.solve().expect("Should find solution");
    
    let x_val = if let Val::ValI(v) = solution[x] { v } else { 0 };
    let y_val = if let Val::ValI(v) = solution[y] { v } else { 0 };
    let z_val = if let Val::ValI(v) = solution[z] { v } else { 0 };
    
    let b1_val = solution[b1] == Val::ValI(1);
    let b2_val = solution[b2] == Val::ValI(1);
    
    assert!(b1_val || b2_val, "At least one condition must hold");
    
    if b1_val {
        assert_eq!(x_val + y_val, 10, "b1 -> x+y=10");
    }
    if b2_val {
        assert_eq!(y_val + z_val, 15, "b2 -> y+z=15");
    }
}

/// Test 3-SAT with linear constraint
/// (a OR b OR c) AND (NOT a OR b) AND (a OR NOT c) AND (a + b + c = 2)
#[test]
fn test_3sat_with_linear() {
    let mut m = Model::default();
    
    let a = m.bool();
    let b = m.bool();
    let c = m.bool();
    
    // 3-SAT clauses
    m.bool_clause(&[a, b, c], &[]);
    m.bool_clause(&[b], &[a]);
    m.bool_clause(&[a], &[c]);
    
    // Exactly 2 must be true
    m.lin_eq(&[1, 1, 1], &[a, b, c], 2);
    
    let solution = m.solve().expect("Should find solution");
    
    let a_val = if solution[a] == Val::ValI(1) { 1 } else { 0 };
    let b_val = if solution[b] == Val::ValI(1) { 1 } else { 0 };
    let c_val = if solution[c] == Val::ValI(1) { 1 } else { 0 };
    
    // Verify clauses
    assert!(a_val + b_val + c_val >= 1, "Clause 1");
    assert!(a_val == 0 || b_val == 1, "Clause 2");
    assert!(a_val == 1 || c_val == 0, "Clause 3");
    
    // Verify linear
    assert_eq!(a_val + b_val + c_val, 2);
}

/// Test reified inequality with bool_clause
/// b <-> (x != y), and (b OR x=5)
#[test]
fn test_reified_ne_with_clause() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let b = m.bool();
    
    // b <-> (x != y)
    m.ne_reif(x, y, b);
    
    // b OR x=5
    let x_eq_5 = m.bool();
    let five = m.int(5, 5);
    m.eq_reif(x, five, x_eq_5);
    
    m.bool_clause(&[b, x_eq_5], &[]);
    
    let solution = m.solve().expect("Should find solution");
    
    let x_val = if let Val::ValI(v) = solution[x] { v } else { 0 };
    let y_val = if let Val::ValI(v) = solution[y] { v } else { 0 };
    let b_val = solution[b] == Val::ValI(1);
    
    assert!(b_val || x_val == 5, "At least one condition");
    
    if b_val {
        assert_ne!(x_val, y_val, "b=true means x!=y");
    } else {
        assert_eq!(x_val, y_val, "b=false means x=y");
    }
}

/// Test large integration: 5 variables, mixed constraints
#[test]
fn test_large_integration() {
    let mut m = Model::default();
    
    let v1 = m.int(0, 10);
    let v2 = m.int(0, 10);
    let v3 = m.int(0, 10);
    let v4 = m.int(0, 10);
    let v5 = m.int(0, 10);
    
    // Linear constraints
    m.lin_eq(&[1, 1, 1, 1, 1], &[v1, v2, v3, v4, v5], 20);
    m.lin_le(&[2, 2, 1, 1, 1], &[v1, v2, v3, v4, v5], 25);
    
    // Boolean clauses
    let v1_eq_5 = m.bool();
    let v2_eq_5 = m.bool();
    
    let five = m.int(5, 5);
    m.eq_reif(v1, five, v1_eq_5);
    m.eq_reif(v2, five, v2_eq_5);
    
    // At least one equals 5
    m.bool_clause(&[v1_eq_5, v2_eq_5], &[]);
    
    // v3 != v4
    let v3_ne_v4 = m.bool();
    m.ne_reif(v3, v4, v3_ne_v4);
    m.new(v3_ne_v4.eq(1));
    
    let solution = m.solve().expect("Should find solution");
    
    let v1_val = if let Val::ValI(v) = solution[v1] { v } else { 0 };
    let v2_val = if let Val::ValI(v) = solution[v2] { v } else { 0 };
    let v3_val = if let Val::ValI(v) = solution[v3] { v } else { 0 };
    let v4_val = if let Val::ValI(v) = solution[v4] { v } else { 0 };
    let v5_val = if let Val::ValI(v) = solution[v5] { v } else { 0 };
    
    // Verify all constraints
    assert_eq!(v1_val + v2_val + v3_val + v4_val + v5_val, 20);
    assert!(2 * v1_val + 2 * v2_val + v3_val + v4_val + v5_val <= 25);
    assert!(v1_val == 5 || v2_val == 5);
    assert_ne!(v3_val, v4_val);
}
