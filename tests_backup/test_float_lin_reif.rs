//! Comprehensive tests for reified float linear constraints
//! Tests for float_lin_eq_reif, float_lin_le_reif, float_lin_ne_reif

use selen::prelude::*;

// ============================================================================
// float_lin_eq_reif - Reified equality
// ============================================================================

#[test]
fn test_float_lin_eq_reif_forces_true() {
    let mut m = Model::default();
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let b = m.bool();
    
    // b <=> (x + y = 7.5)
    m.lin_eq_reif(&[1.0, 1.0], &[x, y], 7.5, b);
    
    // Force b = true
    m.new(b.eq(1));
    m.new(x.eq(3.0));
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[b], Val::ValI(1));
    assert_eq!(solution[x], Val::ValF(3.0));
    assert_eq!(solution[y], Val::ValF(4.5));
}

#[test]
fn test_float_lin_eq_reif_forces_false() {
    let mut m = Model::default();
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let b = m.bool();
    
    // b <=> (x + y = 7.5)
    m.lin_eq_reif(&[1.0, 1.0], &[x, y], 7.5, b);
    
    // Force b = false and fix values that would satisfy
    m.new(b.eq(0));
    m.new(x.eq(3.0));
    m.new(y.eq(4.5)); // This would make sum = 7.5
    
    let result = m.solve();
    // Should be unsatisfiable: b=false but x+y=7.5
    assert!(result.is_err(), "Should be unsatisfiable");
}

#[test]
fn test_float_lin_eq_reif_infers_true() {
    let mut m = Model::default();
    let x = m.float(5.0, 5.0);
    let y = m.float(2.5, 2.5);
    let b = m.bool();
    
    // b <=> (x + y = 7.5)
    m.lin_eq_reif(&[1.0, 1.0], &[x, y], 7.5, b);
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[b], Val::ValI(1)); // Must be true
    assert_eq!(solution[x], Val::ValF(5.0));
    assert_eq!(solution[y], Val::ValF(2.5));
}

#[test]
fn test_float_lin_eq_reif_infers_false() {
    let mut m = Model::default();
    let x = m.float(5.0, 5.0);
    let y = m.float(5.0, 5.0);
    let b = m.bool();
    
    // b <=> (x + y = 7.5)
    // But x = 5, y = 5, so x + y = 10 ≠ 7.5
    m.lin_eq_reif(&[1.0, 1.0], &[x, y], 7.5, b);
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[b], Val::ValI(0)); // Must be false
}

#[test]
fn test_float_lin_eq_reif_with_coefficients() {
    let mut m = Model::default();
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let b = m.bool();
    
    // b <=> (2.0*x + 3.0*y = 18.0)
    m.lin_eq_reif(&[2.0, 3.0], &[x, y], 18.0, b);
    
    m.new(b.eq(1));
    m.new(x.eq(3.0));
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[b], Val::ValI(1));
    assert_eq!(solution[x], Val::ValF(3.0));
    assert_eq!(solution[y], Val::ValF(4.0)); // 2*3 + 3*4 = 18
}

#[test]
fn test_float_lin_eq_reif_negative_coefficients() {
    let mut m = Model::default();
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let b = m.bool();
    
    // b <=> (x - y = 3.0)
    m.lin_eq_reif(&[1.0, -1.0], &[x, y], 3.0, b);
    
    m.new(b.eq(1));
    m.new(x.eq(7.0));
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[b], Val::ValI(1));
    assert_eq!(solution[y], Val::ValF(4.0)); // 7 - 4 = 3
}

#[test]
fn test_float_lin_eq_reif_three_variables() {
    let mut m = Model::default();
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let z = m.float(0.0, 10.0);
    let b = m.bool();
    
    // b <=> (x + y + z = 15.0)
    m.lin_eq_reif(&[1.0, 1.0, 1.0], &[x, y, z], 15.0, b);
    
    m.new(b.eq(1));
    m.new(x.eq(5.0));
    m.new(y.eq(6.0));
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[z], Val::ValF(4.0));
}

#[test]
fn test_float_lin_eq_reif_zero_constant() {
    let mut m = Model::default();
    let x = m.float(-5.0, 5.0);
    let y = m.float(-5.0, 5.0);
    let b = m.bool();
    
    // b <=> (x + y = 0.0)
    m.lin_eq_reif(&[1.0, 1.0], &[x, y], 0.0, b);
    
    m.new(b.eq(1));
    m.new(x.eq(3.0));
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[y], Val::ValF(-3.0));
}

// ============================================================================
// float_lin_le_reif - Reified less-than-or-equal
// ============================================================================

#[test]
fn test_float_lin_le_reif_forces_true() {
    let mut m = Model::default();
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let b = m.bool();
    
    // b <=> (x + y ≤ 10.0)
    m.lin_le_reif(&[1.0, 1.0], &[x, y], 10.0, b);
    
    m.new(b.eq(1));
    m.new(x.eq(6.0));
    
    let solution = m.solve().expect("Should find solution");
    if let Val::ValF(yv) = solution[y] {
        assert!(yv <= 4.0); // Must satisfy x + y ≤ 10
    }
}

#[test]
fn test_float_lin_le_reif_forces_false() {
    let mut m = Model::default();
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let b = m.bool();
    
    // b <=> (x + y ≤ 10.0)
    m.lin_le_reif(&[1.0, 1.0], &[x, y], 10.0, b);
    
    m.new(b.eq(0)); // Force inequality to be false (x + y > 10)
    m.new(x.eq(6.0));
    m.new(y.eq(5.0)); // Force y=5 so x+y=11 > 10
    
    let solution = m.solve().expect("Should find solution");
    if let (Val::ValF(xv), Val::ValF(yv)) = (solution[x], solution[y]) {
        assert!(xv + yv > 10.0); // Must violate the inequality
        assert_eq!(xv, 6.0);
        assert_eq!(yv, 5.0);
    }
}

#[test]
fn test_float_lin_le_reif_infers_true() {
    let mut m = Model::default();
    let x = m.float(0.0, 3.0);
    let y = m.float(0.0, 3.0);
    let b = m.bool();
    
    // b <=> (x + y ≤ 10.0)
    m.lin_le_reif(&[1.0, 1.0], &[x, y], 10.0, b);
    
    // Max possible sum is 6.0, always ≤ 10.0
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[b], Val::ValI(1)); // Must be true
}

#[test]
fn test_float_lin_le_reif_infers_false() {
    let mut m = Model::default();
    let x = m.float(8.0, 10.0);
    let y = m.float(8.0, 10.0);
    let b = m.bool();
    
    // b <=> (x + y ≤ 10.0)
    m.lin_le_reif(&[1.0, 1.0], &[x, y], 10.0, b);
    
    // Min possible sum is 16.0, always > 10.0
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[b], Val::ValI(0)); // Must be false
}

#[test]
fn test_float_lin_le_reif_at_boundary() {
    let mut m = Model::default();
    let x = m.float(5.0, 5.0);
    let y = m.float(5.0, 5.0);
    let b = m.bool();
    
    // b <=> (x + y ≤ 10.0)
    m.lin_le_reif(&[1.0, 1.0], &[x, y], 10.0, b);
    
    // Sum is exactly 10.0, so ≤ 10.0 is true
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[b], Val::ValI(1));
}

#[test]
fn test_float_lin_le_reif_with_coefficients() {
    let mut m = Model::default();
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let b = m.bool();
    
    // b <=> (2.0*x + 3.0*y ≤ 30.0)
    m.lin_le_reif(&[2.0, 3.0], &[x, y], 30.0, b);
    
    m.new(b.eq(1));
    m.new(x.eq(6.0));
    
    let solution = m.solve().expect("Should find solution");
    if let Val::ValF(yv) = solution[y] {
        assert!(2.0 * 6.0 + 3.0 * yv <= 30.0);
    }
}

#[test]
fn test_float_lin_le_reif_negative_coefficients() {
    let mut m = Model::default();
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let b = m.bool();
    
    // b <=> (x - y ≤ 3.0)
    m.lin_le_reif(&[1.0, -1.0], &[x, y], 3.0, b);
    
    m.new(b.eq(1));
    m.new(x.eq(8.0));
    
    let solution = m.solve().expect("Should find solution");
    if let Val::ValF(yv) = solution[y] {
        assert!(8.0 - yv <= 3.0); // yv ≥ 5.0
        assert!(yv >= 5.0);
    }
}

// ============================================================================
// float_lin_ne_reif - Reified not-equal
// ============================================================================

#[test]
fn test_float_lin_ne_reif_forces_true() {
    let mut m = Model::default();
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let b = m.bool();
    
    // b <=> (x + y ≠ 10.0)
    m.lin_ne_reif(&[1.0, 1.0], &[x, y], 10.0, b);
    
    m.new(b.eq(1)); // Force inequality
    m.new(x.eq(5.0));
    
    let solution = m.solve().expect("Should find solution");
    assert_ne!(solution[y], Val::ValF(5.0)); // Can't be 5.0 (would make sum 10)
}

#[test]
fn test_float_lin_ne_reif_forces_false() {
    let mut m = Model::default();
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let b = m.bool();
    
    // b <=> (x + y ≠ 10.0)
    m.lin_ne_reif(&[1.0, 1.0], &[x, y], 10.0, b);
    
    m.new(b.eq(0)); // Force equality (x + y = 10)
    m.new(x.eq(3.0));
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[y], Val::ValF(7.0)); // Must be 7.0
}

#[test]
fn test_float_lin_ne_reif_infers_true() {
    let mut m = Model::default();
    let x = m.float(5.0, 5.0);
    let y = m.float(7.0, 7.0);
    let b = m.bool();
    
    // b <=> (x + y ≠ 10.0)
    m.lin_ne_reif(&[1.0, 1.0], &[x, y], 10.0, b);
    
    // x + y = 12.0 ≠ 10.0
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[b], Val::ValI(1));
}

#[test]
fn test_float_lin_ne_reif_infers_false() {
    let mut m = Model::default();
    let x = m.float(4.0, 4.0);
    let y = m.float(6.0, 6.0);
    let b = m.bool();
    
    // b <=> (x + y ≠ 10.0)
    m.lin_ne_reif(&[1.0, 1.0], &[x, y], 10.0, b);
    
    // x + y = 10.0, so b must be false
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[b], Val::ValI(0));
}

#[test]
fn test_float_lin_ne_reif_with_coefficients() {
    let mut m = Model::default();
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let b = m.bool();
    
    // b <=> (2.0*x + 3.0*y ≠ 18.0)
    m.lin_ne_reif(&[2.0, 3.0], &[x, y], 18.0, b);
    
    m.new(b.eq(1));
    m.new(x.eq(3.0));
    
    let solution = m.solve().expect("Should find solution");
    if let Val::ValF(yv) = solution[y] {
        assert_ne!(2.0 * 3.0 + 3.0 * yv, 18.0);
    }
}

#[test]
fn test_float_lin_ne_reif_three_variables() {
    let mut m = Model::default();
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let z = m.float(0.0, 10.0);
    let b = m.bool();
    
    // b <=> (x + y + z ≠ 15.0)
    m.lin_ne_reif(&[1.0, 1.0, 1.0], &[x, y, z], 15.0, b);
    
    m.new(b.eq(1));
    m.new(x.eq(5.0));
    m.new(y.eq(5.0));
    
    let solution = m.solve().expect("Should find solution");
    assert_ne!(solution[z], Val::ValF(5.0)); // Can't be 5.0
}

// ============================================================================
// Edge cases and combinations
// ============================================================================

#[test]
fn test_float_lin_reif_precision() {
    let mut m = Model::default();
    let x = m.float(0.0, 1.0);
    let y = m.float(0.0, 1.0);
    let b = m.bool();
    
    // b <=> (x + y = 0.3)
    m.lin_eq_reif(&[1.0, 1.0], &[x, y], 0.3, b);
    
    m.new(b.eq(1));
    m.new(x.eq(0.1));
    
    let solution = m.solve().expect("Should find solution");
    if let Val::ValF(yv) = solution[y] {
        // Use 1e-5 tolerance to account for step-size rounding (step = 1e-6)
        assert!((yv - 0.2).abs() < 1e-5); // Should be 0.2 within floating point precision
    }
}

#[test]
fn test_multiple_float_lin_reif() {
    let mut m = Model::default();
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let b1 = m.bool();
    let b2 = m.bool();
    
    // b1 <=> (x + y = 10.0)
    m.lin_eq_reif(&[1.0, 1.0], &[x, y], 10.0, b1);
    
    // b2 <=> (x + y ≤ 8.0)
    m.lin_le_reif(&[1.0, 1.0], &[x, y], 8.0, b2);
    
    m.new(x.eq(4.0));
    m.new(y.eq(4.0));
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[b1], Val::ValI(0)); // 4 + 4 = 8 ≠ 10
    assert_eq!(solution[b2], Val::ValI(1));  // 4 + 4 = 8 ≤ 8
}

#[test]
fn test_float_lin_reif_with_bool_logic() {
    let mut m = Model::default();
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let b1 = m.bool();
    let b2 = m.bool();
    
    // b1 <=> (x + y = 10.0)
    m.lin_eq_reif(&[1.0, 1.0], &[x, y], 10.0, b1);
    
    // b2 <=> (x + y ≠ 10.0)
    m.lin_ne_reif(&[1.0, 1.0], &[x, y], 10.0, b2);
    
    // b1 and b2 should be opposites
    let solution = m.solve().expect("Should find solution");
    let b1_val = solution[b1] == Val::ValI(1);
    let b2_val = solution[b2] == Val::ValI(1);
    assert_ne!(b1_val, b2_val); // Exactly one should be true
}

#[test]
fn test_float_lin_eq_reif_single_variable() {
    let mut m = Model::default();
    let x = m.float(0.0, 10.0);
    let b = m.bool();
    
    // b <=> (2.0*x = 6.0)
    m.lin_eq_reif(&[2.0], &[x], 6.0, b);
    
    m.new(b.eq(1));
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[x], Val::ValF(3.0));
}

#[test]
fn test_float_lin_le_reif_negative_constant() {
    let mut m = Model::default();
    let x = m.float(-10.0, 10.0);
    let y = m.float(-10.0, 10.0);
    let b = m.bool();
    
    // b <=> (x + y ≤ -5.0)
    m.lin_le_reif(&[1.0, 1.0], &[x, y], -5.0, b);
    
    m.new(b.eq(1));
    m.new(x.eq(-3.0));
    
    let solution = m.solve().expect("Should find solution");
    if let Val::ValF(yv) = solution[y] {
        assert!(yv <= -2.0); // Must satisfy constraint
    }
}
