//! Tests for integer linear reified constraints (int_lin_*_reif)

use selen::prelude::*;

// ═══════════════════════════════════════════════════════════════════════
// int_lin_eq_reif tests
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn test_int_lin_eq_reif_true() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let b = m.bool();
    
    // b ⇔ (x + y = 10)
    m.lin_eq_reif(&[1, 1], &[x, y], 10, b);
    
    // Force b = 1 (equation must hold)
    m.new(b.eq(1));
    
    // Force x = 3
    m.new(x.eq(3));
    
    let solution = m.solve().expect("Should find solution");
    
    // y must be 7 because b=1 forces x+y=10 and x=3
    assert_eq!(solution[x], Val::ValI(3));
    assert_eq!(solution[y], Val::ValI(7));
    assert_eq!(solution[b], Val::ValI(1));
}

#[test]
fn test_int_lin_eq_reif_false() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let b = m.bool();
    
    // b ⇔ (x + y = 10)
    m.lin_eq_reif(&[1, 1], &[x, y], 10, b);
    
    // Force b = 0 (equation must NOT hold)
    m.new(b.eq(0));
    
    // Force x = 3
    m.new(x.eq(3));
    
    let solution = m.solve().expect("Should find solution");
    
    // y must NOT be 7 because b=0 forces x+y≠10
    assert_eq!(solution[x], Val::ValI(3));
    assert_ne!(solution[y], Val::ValI(7));
    assert_eq!(solution[b], Val::ValI(0));
}

#[test]
fn test_int_lin_eq_reif_infer_true() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let b = m.bool();
    
    // b ⇔ (x + y = 10)
    m.lin_eq_reif(&[1, 1], &[x, y], 10, b);
    
    // Force values that make equation true
    m.new(x.eq(6));
    m.new(y.eq(4));
    
    let solution = m.solve().expect("Should find solution");
    
    // b must be inferred to 1
    assert_eq!(solution[x], Val::ValI(6));
    assert_eq!(solution[y], Val::ValI(4));
    assert_eq!(solution[b], Val::ValI(1));
}

#[test]
fn test_int_lin_eq_reif_infer_false() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let b = m.bool();
    
    // b ⇔ (x + y = 10)
    m.lin_eq_reif(&[1, 1], &[x, y], 10, b);
    
    // Force values that make equation false
    m.new(x.eq(3));
    m.new(y.eq(4));
    
    let solution = m.solve().expect("Should find solution");
    
    // b must be inferred to 0 (3+4=7≠10)
    assert_eq!(solution[x], Val::ValI(3));
    assert_eq!(solution[y], Val::ValI(4));
    assert_eq!(solution[b], Val::ValI(0));
}

#[test]
fn test_int_lin_eq_reif_with_coefficients() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let b = m.bool();
    
    // b ⇔ (2*x + 3*y = 20)
    m.lin_eq_reif(&[2, 3], &[x, y], 20, b);
    
    // Force b = 1
    m.new(b.eq(1));
    m.new(x.eq(4));
    
    let solution = m.solve().expect("Should find solution");
    
    // y must be 4 (2*4 + 3*4 = 8 + 12 = 20)
    assert_eq!(solution[x], Val::ValI(4));
    assert_eq!(solution[y], Val::ValI(4));
    assert_eq!(solution[b], Val::ValI(1));
}

#[test]
fn test_int_lin_eq_reif_negative_coefficients() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let b = m.bool();
    
    // b ⇔ (x - y = 2)
    m.lin_eq_reif(&[1, -1], &[x, y], 2, b);
    
    // Force x = 7, b = 1
    m.new(x.eq(7));
    m.new(b.eq(1));
    
    let solution = m.solve().expect("Should find solution");
    
    // y must be 5 (7 - 5 = 2)
    assert_eq!(solution[x], Val::ValI(7));
    assert_eq!(solution[y], Val::ValI(5));
    assert_eq!(solution[b], Val::ValI(1));
}

#[test]
fn test_int_lin_eq_reif_empty_array() {
    let mut m = Model::default();
    
    let b = m.bool();
    
    // b ⇔ (empty sum = 0)  => should set b = 1
    m.lin_eq_reif(&[], &[], 0, b);
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[b], Val::ValI(1));
}

#[test]
fn test_int_lin_eq_reif_empty_array_nonzero() {
    let mut m = Model::default();
    
    let b = m.bool();
    
    // b ⇔ (empty sum = 5)  => should set b = 0
    m.lin_eq_reif(&[], &[], 5, b);
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[b], Val::ValI(0));
}

// ═══════════════════════════════════════════════════════════════════════
// int_lin_le_reif tests
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn test_int_lin_le_reif_true() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let b = m.bool();
    
    // b ⇔ (x + y ≤ 15)
    m.lin_le_reif(&[1, 1], &[x, y], 15, b);
    
    // Force b = 1 (inequality must hold)
    m.new(b.eq(1));
    
    // Force x = 10
    m.new(x.eq(10));
    
    let solution = m.solve().expect("Should find solution");
    
    // y must be ≤ 5 because b=1 forces x+y≤15 and x=10
    assert_eq!(solution[x], Val::ValI(10));
    let y_val = if let Val::ValI(v) = solution[y] { v } else { panic!("y should be int") };
    assert!(y_val <= 5, "y should be ≤ 5, got {}", y_val);
    assert_eq!(solution[b], Val::ValI(1));
}

#[test]
fn test_int_lin_le_reif_false() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let b = m.bool();
    
    // b ⇔ (x + y ≤ 8)
    m.lin_le_reif(&[1, 1], &[x, y], 8, b);
    
    // Force b = 0 (inequality must NOT hold, meaning x+y > 8)
    m.new(b.eq(0));
    
    // Force x = 5
    m.new(x.eq(5));
    
    let solution = m.solve().expect("Should find solution");
    
    // y must be > 3 because b=0 forces x+y>8 and x=5
    assert_eq!(solution[x], Val::ValI(5));
    let y_val = if let Val::ValI(v) = solution[y] { v } else { panic!("y should be int") };
    assert!(y_val > 3, "y should be > 3, got {}", y_val);
    assert_eq!(solution[b], Val::ValI(0));
}

#[test]
fn test_int_lin_le_reif_infer_true() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let b = m.bool();
    
    // b ⇔ (x + y ≤ 10)
    m.lin_le_reif(&[1, 1], &[x, y], 10, b);
    
    // Force values that satisfy inequality
    m.new(x.eq(3));
    m.new(y.eq(4));
    
    let solution = m.solve().expect("Should find solution");
    
    // b must be inferred to 1 (3+4=7≤10)
    assert_eq!(solution[x], Val::ValI(3));
    assert_eq!(solution[y], Val::ValI(4));
    assert_eq!(solution[b], Val::ValI(1));
}

#[test]
fn test_int_lin_le_reif_infer_false() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let b = m.bool();
    
    // b ⇔ (x + y ≤ 5)
    m.lin_le_reif(&[1, 1], &[x, y], 5, b);
    
    // Force values that violate inequality
    m.new(x.eq(8));
    m.new(y.eq(3));
    
    let solution = m.solve().expect("Should find solution");
    
    // b must be inferred to 0 (8+3=11>5)
    assert_eq!(solution[x], Val::ValI(8));
    assert_eq!(solution[y], Val::ValI(3));
    assert_eq!(solution[b], Val::ValI(0));
}

#[test]
fn test_int_lin_le_reif_with_coefficients() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let b = m.bool();
    
    // b ⇔ (2*x + 3*y ≤ 30)
    m.lin_le_reif(&[2, 3], &[x, y], 30, b);
    
    // Force b = 1, x = 6
    m.new(b.eq(1));
    m.new(x.eq(6));
    
    let solution = m.solve().expect("Should find solution");
    
    // y must be ≤ 6 (2*6 + 3*y ≤ 30 => 12 + 3*y ≤ 30 => y ≤ 6)
    assert_eq!(solution[x], Val::ValI(6));
    let y_val = if let Val::ValI(v) = solution[y] { v } else { panic!("y should be int") };
    assert!(y_val <= 6, "y should be ≤ 6, got {}", y_val);
    assert_eq!(solution[b], Val::ValI(1));
}

#[test]
fn test_int_lin_le_reif_at_boundary() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let b = m.bool();
    
    // b ⇔ (x + y ≤ 10)
    m.lin_le_reif(&[1, 1], &[x, y], 10, b);
    
    // Force x = 5, y = 5 (exactly at boundary)
    m.new(x.eq(5));
    m.new(y.eq(5));
    
    let solution = m.solve().expect("Should find solution");
    
    // b should be 1 (5+5=10≤10)
    assert_eq!(solution[x], Val::ValI(5));
    assert_eq!(solution[y], Val::ValI(5));
    assert_eq!(solution[b], Val::ValI(1));
}

#[test]
fn test_int_lin_le_reif_empty_array() {
    let mut m = Model::default();
    
    let b = m.bool();
    
    // b ⇔ (empty sum ≤ 5)  => 0 ≤ 5 is true, should set b = 1
    m.lin_le_reif(&[], &[], 5, b);
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[b], Val::ValI(1));
}

#[test]
fn test_int_lin_le_reif_empty_array_negative() {
    let mut m = Model::default();
    
    let b = m.bool();
    
    // b ⇔ (empty sum ≤ -5)  => 0 ≤ -5 is false, should set b = 0
    m.lin_le_reif(&[], &[], -5, b);
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[b], Val::ValI(0));
}

// ═══════════════════════════════════════════════════════════════════════
// int_lin_ne_reif tests
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn test_int_lin_ne_reif_true() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let b = m.bool();
    
    // b ⇔ (x + y ≠ 10)
    m.lin_ne_reif(&[1, 1], &[x, y], 10, b);
    
    // Force b = 1 (inequality must hold: x+y≠10)
    m.new(b.eq(1));
    
    // Force x = 3
    m.new(x.eq(3));
    
    let solution = m.solve().expect("Should find solution");
    
    // y must NOT be 7 because b=1 forces x+y≠10
    assert_eq!(solution[x], Val::ValI(3));
    assert_ne!(solution[y], Val::ValI(7));
    assert_eq!(solution[b], Val::ValI(1));
}

#[test]
fn test_int_lin_ne_reif_false() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let b = m.bool();
    
    // b ⇔ (x + y ≠ 10)
    m.lin_ne_reif(&[1, 1], &[x, y], 10, b);
    
    // Force b = 0 (must have x+y=10)
    m.new(b.eq(0));
    
    // Force x = 6
    m.new(x.eq(6));
    
    let solution = m.solve().expect("Should find solution");
    
    // y must be 4 because b=0 forces x+y=10 and x=6
    assert_eq!(solution[x], Val::ValI(6));
    assert_eq!(solution[y], Val::ValI(4));
    assert_eq!(solution[b], Val::ValI(0));
}

#[test]
fn test_int_lin_ne_reif_infer_true() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let b = m.bool();
    
    // b ⇔ (x + y ≠ 10)
    m.lin_ne_reif(&[1, 1], &[x, y], 10, b);
    
    // Force values that make inequality true
    m.new(x.eq(3));
    m.new(y.eq(4));
    
    let solution = m.solve().expect("Should find solution");
    
    // b must be inferred to 1 (3+4=7≠10)
    assert_eq!(solution[x], Val::ValI(3));
    assert_eq!(solution[y], Val::ValI(4));
    assert_eq!(solution[b], Val::ValI(1));
}

#[test]
fn test_int_lin_ne_reif_infer_false() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let b = m.bool();
    
    // b ⇔ (x + y ≠ 10)
    m.lin_ne_reif(&[1, 1], &[x, y], 10, b);
    
    // Force values that make equation true (inequality false)
    m.new(x.eq(6));
    m.new(y.eq(4));
    
    let solution = m.solve().expect("Should find solution");
    
    // b must be inferred to 0 (6+4=10, so NOT not-equal)
    assert_eq!(solution[x], Val::ValI(6));
    assert_eq!(solution[y], Val::ValI(4));
    assert_eq!(solution[b], Val::ValI(0));
}

#[test]
fn test_int_lin_ne_reif_with_coefficients() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let b = m.bool();
    
    // b ⇔ (2*x + 3*y ≠ 20)
    m.lin_ne_reif(&[2, 3], &[x, y], 20, b);
    
    // Force b = 1 (must have 2*x + 3*y ≠ 20)
    m.new(b.eq(1));
    m.new(x.eq(4));
    
    let solution = m.solve().expect("Should find solution");
    
    // y must NOT be 4 (because 2*4 + 3*4 = 8 + 12 = 20)
    assert_eq!(solution[x], Val::ValI(4));
    assert_ne!(solution[y], Val::ValI(4));
    assert_eq!(solution[b], Val::ValI(1));
}

#[test]
fn test_int_lin_ne_reif_three_variables() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let z = m.int(0, 10);
    let b = m.bool();
    
    // b ⇔ (x + y + z ≠ 15)
    m.lin_ne_reif(&[1, 1, 1], &[x, y, z], 15, b);
    
    // Force specific values
    m.new(x.eq(5));
    m.new(y.eq(5));
    m.new(z.eq(5));
    
    let solution = m.solve().expect("Should find solution");
    
    // b must be 0 (5+5+5=15, so NOT not-equal)
    assert_eq!(solution[x], Val::ValI(5));
    assert_eq!(solution[y], Val::ValI(5));
    assert_eq!(solution[z], Val::ValI(5));
    assert_eq!(solution[b], Val::ValI(0));
}

#[test]
fn test_int_lin_ne_reif_empty_array() {
    let mut m = Model::default();
    
    let b = m.bool();
    
    // b ⇔ (empty sum ≠ 5)  => 0 ≠ 5 is true, should set b = 1
    m.lin_ne_reif(&[], &[], 5, b);
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[b], Val::ValI(1));
}

#[test]
fn test_int_lin_ne_reif_empty_array_zero() {
    let mut m = Model::default();
    
    let b = m.bool();
    
    // b ⇔ (empty sum ≠ 0)  => 0 ≠ 0 is false, should set b = 0
    m.lin_ne_reif(&[], &[], 0, b);
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[b], Val::ValI(0));
}

// ═══════════════════════════════════════════════════════════════════════
// Combined/Integration tests
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn test_multiple_int_lin_reif_constraints() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let b1 = m.bool();
    let b2 = m.bool();
    let b3 = m.bool();
    
    // b1 ⇔ (x + y = 10)
    m.lin_eq_reif(&[1, 1], &[x, y], 10, b1);
    
    // b2 ⇔ (x + y ≤ 8)
    m.lin_le_reif(&[1, 1], &[x, y], 8, b2);
    
    // b3 ⇔ (x + y ≠ 7)
    m.lin_ne_reif(&[1, 1], &[x, y], 7, b3);
    
    // Force x = 5, y = 5
    m.new(x.eq(5));
    m.new(y.eq(5));
    
    let solution = m.solve().expect("Should find solution");
    
    // x+y = 10, so:
    // b1 should be 1 (10 = 10)
    // b2 should be 0 (10 > 8)
    // b3 should be 1 (10 ≠ 7)
    assert_eq!(solution[b1], Val::ValI(1));
    assert_eq!(solution[b2], Val::ValI(0));
    assert_eq!(solution[b3], Val::ValI(1));
}

#[test]
fn test_int_lin_reif_mismatched_lengths() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let b = m.bool();
    
    // Mismatched lengths should force b = 0
    m.lin_eq_reif(&[1, 2], &[x], 5, b);
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[b], Val::ValI(0));
}
