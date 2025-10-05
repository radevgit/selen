//! Comprehensive tests for boolean linear constraints
//! Tests for bool_lin_eq, bool_lin_le, bool_lin_ne and their reified versions

use selen::prelude::*;

// ============================================================================
// bool_lin_eq - Cardinality and weighted sums
// ============================================================================

#[test]
fn test_bool_lin_eq_exactly_k_out_of_n() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    let b3 = m.bool();
    
    // Exactly 2 out of 3 must be true
    m.bool_lin_eq(&[1, 1, 1], &[b1, b2, b3], 2);
    
    let solution = m.solve().expect("Should find solution");
    
    // Count how many are true
    let count = [b1, b2, b3].iter()
        .filter(|&&var| solution[var] == Val::ValI(1))
        .count();
    
    assert_eq!(count, 2);
}

#[test]
fn test_bool_lin_eq_all_true() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    let b3 = m.bool();
    
    // All must be true
    m.bool_lin_eq(&[1, 1, 1], &[b1, b2, b3], 3);
    
    let solution = m.solve().expect("Should find solution");
    
    assert_eq!(solution[b1], Val::ValI(1));
    assert_eq!(solution[b2], Val::ValI(1));
    assert_eq!(solution[b3], Val::ValI(1));
}

#[test]
fn test_bool_lin_eq_all_false() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    let b3 = m.bool();
    
    // All must be false
    m.bool_lin_eq(&[1, 1, 1], &[b1, b2, b3], 0);
    
    let solution = m.solve().expect("Should find solution");
    
    assert_eq!(solution[b1], Val::ValI(0));
    assert_eq!(solution[b2], Val::ValI(0));
    assert_eq!(solution[b3], Val::ValI(0));
}

#[test]
fn test_bool_lin_eq_weighted_sum() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    let b3 = m.bool();
    
    // 2*b1 + 3*b2 + 1*b3 = 5
    m.bool_lin_eq(&[2, 3, 1], &[b1, b2, b3], 5);
    
    let solution = m.solve().expect("Should find solution");
    
    if let (Val::ValI(v1), Val::ValI(v2), Val::ValI(v3)) = 
        (solution[b1], solution[b2], solution[b3]) {
        assert_eq!(2 * v1 + 3 * v2 + v3, 5);
    }
}

#[test]
fn test_bool_lin_eq_single_variable() {
    let mut m = Model::default();
    let b = m.bool();
    
    // b must be true
    m.bool_lin_eq(&[1], &[b], 1);
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[b], Val::ValI(1));
}

#[test]
fn test_bool_lin_eq_unsatisfiable() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    
    // Sum can only be 0, 1, or 2, not 5
    m.bool_lin_eq(&[1, 1], &[b1, b2], 5);
    
    let result = m.solve();
    assert!(result.is_err(), "Should be unsatisfiable");
}

#[test]
fn test_bool_lin_eq_negative_coefficients() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    
    // 2*b1 - b2 = 1
    // Possible: b1=1, b2=1 -> 2-1=1 ✓
    m.bool_lin_eq(&[2, -1], &[b1, b2], 1);
    
    let solution = m.solve().expect("Should find solution");
    
    if let (Val::ValI(v1), Val::ValI(v2)) = (solution[b1], solution[b2]) {
        assert_eq!(2 * v1 - v2, 1);
    }
}

// ============================================================================
// bool_lin_le - At-most-k constraints
// ============================================================================

#[test]
fn test_bool_lin_le_at_most_k() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    let b3 = m.bool();
    
    // At most 2 out of 3 can be true
    m.bool_lin_le(&[1, 1, 1], &[b1, b2, b3], 2);
    
    let solution = m.solve().expect("Should find solution");
    
    let count = [b1, b2, b3].iter()
        .filter(|&&var| solution[var] == Val::ValI(1))
        .count();
    
    assert!(count <= 2);
}

#[test]
fn test_bool_lin_le_at_boundary() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    
    // Sum ≤ 2 (always satisfied for 2 booleans)
    m.bool_lin_le(&[1, 1], &[b1, b2], 2);
    
    // Force both true
    m.new(b1.eq(1));
    m.new(b2.eq(1));
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[b1], Val::ValI(1));
    assert_eq!(solution[b2], Val::ValI(1));
}

#[test]
fn test_bool_lin_le_tight_bound() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    let b3 = m.bool();
    
    // At most 1 can be true
    m.bool_lin_le(&[1, 1, 1], &[b1, b2, b3], 1);
    
    let solution = m.solve().expect("Should find solution");
    
    let count = [b1, b2, b3].iter()
        .filter(|&&var| solution[var] == Val::ValI(1))
        .count();
    
    assert!(count <= 1);
}

#[test]
fn test_bool_lin_le_weighted() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    let b3 = m.bool();
    
    // 2*b1 + 3*b2 + 1*b3 ≤ 4
    m.bool_lin_le(&[2, 3, 1], &[b1, b2, b3], 4);
    
    let solution = m.solve().expect("Should find solution");
    
    if let (Val::ValI(v1), Val::ValI(v2), Val::ValI(v3)) = 
        (solution[b1], solution[b2], solution[b3]) {
        assert!(2 * v1 + 3 * v2 + v3 <= 4);
    }
}

#[test]
fn test_bool_lin_le_unsatisfiable() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    let b3 = m.bool();
    
    // Sum must be ≤ -1 (impossible for booleans)
    m.bool_lin_le(&[1, 1, 1], &[b1, b2, b3], -1);
    
    let result = m.solve();
    assert!(result.is_err(), "Should be unsatisfiable");
}

// ============================================================================
// bool_lin_ne - Not-exactly-k constraints
// ============================================================================

#[test]
fn test_bool_lin_ne_not_exactly_k() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    let b3 = m.bool();
    
    // NOT exactly 2 out of 3 (can be 0, 1, or 3)
    m.bool_lin_ne(&[1, 1, 1], &[b1, b2, b3], 2);
    
    let solution = m.solve().expect("Should find solution");
    
    let count = [b1, b2, b3].iter()
        .filter(|&&var| solution[var] == Val::ValI(1))
        .count();
    
    assert_ne!(count, 2);
}

#[test]
fn test_bool_lin_ne_weighted() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    
    // 2*b1 + 3*b2 ≠ 3
    m.bool_lin_ne(&[2, 3], &[b1, b2], 3);
    
    let solution = m.solve().expect("Should find solution");
    
    if let (Val::ValI(v1), Val::ValI(v2)) = (solution[b1], solution[b2]) {
        assert_ne!(2 * v1 + 3 * v2, 3);
    }
}

#[test]
fn test_bool_lin_ne_forces_specific_value() {
    let mut m = Model::default();
    let b = m.bool();
    
    // b ≠ 0, so b must be 1
    m.bool_lin_ne(&[1], &[b], 0);
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[b], Val::ValI(1));
}

// ============================================================================
// bool_lin_eq_reif - Reified cardinality
// ============================================================================

#[test]
fn test_bool_lin_eq_reif_forces_true() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    let b3 = m.bool();
    let reif = m.bool();
    
    // reif ⇔ (exactly 2 are true)
    m.bool_lin_eq_reif(&[1, 1, 1], &[b1, b2, b3], 2, reif);
    
    // Force reif = true
    m.new(reif.eq(1));
    
    let solution = m.solve().expect("Should find solution");
    
    let count = [b1, b2, b3].iter()
        .filter(|&&var| solution[var] == Val::ValI(1))
        .count();
    
    assert_eq!(count, 2);
}

#[test]
fn test_bool_lin_eq_reif_forces_false() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    let b3 = m.bool();
    let reif = m.bool();
    
    // reif ⇔ (exactly 2 are true)
    m.bool_lin_eq_reif(&[1, 1, 1], &[b1, b2, b3], 2, reif);
    
    // Force reif = false
    m.new(reif.eq(0));
    
    let solution = m.solve().expect("Should find solution");
    
    let count = [b1, b2, b3].iter()
        .filter(|&&var| solution[var] == Val::ValI(1))
        .count();
    
    assert_ne!(count, 2); // Must NOT be exactly 2
}

#[test]
fn test_bool_lin_eq_reif_infers_true() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    let b3 = m.bool();
    let reif = m.bool();
    
    // reif ⇔ (exactly 2 are true)
    m.bool_lin_eq_reif(&[1, 1, 1], &[b1, b2, b3], 2, reif);
    
    // Fix values to make exactly 2 true
    m.new(b1.eq(1));
    m.new(b2.eq(1));
    m.new(b3.eq(0));
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[reif], Val::ValI(1));
}

#[test]
fn test_bool_lin_eq_reif_infers_false() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    let b3 = m.bool();
    let reif = m.bool();
    
    // reif ⇔ (exactly 2 are true)
    m.bool_lin_eq_reif(&[1, 1, 1], &[b1, b2, b3], 2, reif);
    
    // Fix all to true (sum = 3, not 2)
    m.new(b1.eq(1));
    m.new(b2.eq(1));
    m.new(b3.eq(1));
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[reif], Val::ValI(0));
}

#[test]
fn test_bool_lin_eq_reif_weighted() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    let reif = m.bool();
    
    // reif ⇔ (2*b1 + 3*b2 = 5)
    m.bool_lin_eq_reif(&[2, 3], &[b1, b2], 5, reif);
    
    m.new(reif.eq(1));
    
    let solution = m.solve().expect("Should find solution");
    
    // Only solution: b1=1, b2=1 (2+3=5)
    assert_eq!(solution[b1], Val::ValI(1));
    assert_eq!(solution[b2], Val::ValI(1));
}

// ============================================================================
// bool_lin_le_reif - Reified at-most-k
// ============================================================================

#[test]
fn test_bool_lin_le_reif_forces_true() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    let b3 = m.bool();
    let reif = m.bool();
    
    // reif ⇔ (at most 2 are true)
    m.bool_lin_le_reif(&[1, 1, 1], &[b1, b2, b3], 2, reif);
    
    m.new(reif.eq(1));
    
    let solution = m.solve().expect("Should find solution");
    
    let count = [b1, b2, b3].iter()
        .filter(|&&var| solution[var] == Val::ValI(1))
        .count();
    
    assert!(count <= 2);
}

#[test]
fn test_bool_lin_le_reif_forces_false() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    let b3 = m.bool();
    let reif = m.bool();
    
    // reif ⇔ (at most 1 is true)
    m.bool_lin_le_reif(&[1, 1, 1], &[b1, b2, b3], 1, reif);
    
    // Force reif = false (so > 1 must be true)
    m.new(reif.eq(0));
    
    let solution = m.solve().expect("Should find solution");
    
    let count = [b1, b2, b3].iter()
        .filter(|&&var| solution[var] == Val::ValI(1))
        .count();
    
    assert!(count > 1); // Must violate ≤ 1
}

#[test]
fn test_bool_lin_le_reif_infers_true() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    let reif = m.bool();
    
    // reif ⇔ (b1 + b2 ≤ 2)
    m.bool_lin_le_reif(&[1, 1], &[b1, b2], 2, reif);
    
    // Always true for 2 booleans
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[reif], Val::ValI(1));
}

#[test]
fn test_bool_lin_le_reif_infers_false() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    let b3 = m.bool();
    let reif = m.bool();
    
    // reif ⇔ (b1 + b2 + b3 ≤ 1)
    m.bool_lin_le_reif(&[1, 1, 1], &[b1, b2, b3], 1, reif);
    
    // Force all true (sum = 3 > 1)
    m.new(b1.eq(1));
    m.new(b2.eq(1));
    m.new(b3.eq(1));
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[reif], Val::ValI(0));
}

// ============================================================================
// bool_lin_ne_reif - Reified not-exactly-k
// ============================================================================

#[test]
fn test_bool_lin_ne_reif_forces_true() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    let reif = m.bool();
    
    // reif ⇔ (b1 + b2 ≠ 1)
    m.bool_lin_ne_reif(&[1, 1], &[b1, b2], 1, reif);
    
    m.new(reif.eq(1));
    
    let solution = m.solve().expect("Should find solution");
    
    if let (Val::ValI(v1), Val::ValI(v2)) = (solution[b1], solution[b2]) {
        assert_ne!(v1 + v2, 1); // Must be 0 or 2
    }
}

#[test]
fn test_bool_lin_ne_reif_forces_false() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    let reif = m.bool();
    
    // reif ⇔ (b1 + b2 ≠ 1)
    m.bool_lin_ne_reif(&[1, 1], &[b1, b2], 1, reif);
    
    m.new(reif.eq(0)); // Force equality
    
    let solution = m.solve().expect("Should find solution");
    
    if let (Val::ValI(v1), Val::ValI(v2)) = (solution[b1], solution[b2]) {
        assert_eq!(v1 + v2, 1); // Must equal 1
    }
}

#[test]
fn test_bool_lin_ne_reif_infers_true() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    let reif = m.bool();
    
    // reif ⇔ (b1 + b2 ≠ 1)
    m.bool_lin_ne_reif(&[1, 1], &[b1, b2], 1, reif);
    
    // Force both false (sum = 0 ≠ 1)
    m.new(b1.eq(0));
    m.new(b2.eq(0));
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[reif], Val::ValI(1));
}

#[test]
fn test_bool_lin_ne_reif_infers_false() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    let reif = m.bool();
    
    // reif ⇔ (b1 + b2 ≠ 1)
    m.bool_lin_ne_reif(&[1, 1], &[b1, b2], 1, reif);
    
    // Force sum = 1
    m.new(b1.eq(1));
    m.new(b2.eq(0));
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[reif], Val::ValI(0));
}

// ============================================================================
// Combined constraints and edge cases
// ============================================================================

#[test]
fn test_bool_lin_multiple_constraints() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    let b3 = m.bool();
    let b4 = m.bool();
    
    // At least 2 must be true
    m.bool_lin_le(&[-1, -1, -1, -1], &[b1, b2, b3, b4], -2);
    
    // At most 3 must be true
    m.bool_lin_le(&[1, 1, 1, 1], &[b1, b2, b3, b4], 3);
    
    let solution = m.solve().expect("Should find solution");
    
    let count = [b1, b2, b3, b4].iter()
        .filter(|&&var| solution[var] == Val::ValI(1))
        .count();
    
    assert!(count >= 2 && count <= 3);
}

#[test]
fn test_bool_lin_empty_array() {
    let mut m = Model::default();
    
    // Empty sum = 0 (satisfiable)
    m.bool_lin_eq(&[], &[], 0);
    
    let solution = m.solve();
    assert!(solution.is_ok(), "Empty sum = 0 should be satisfiable");
}

#[test]
fn test_bool_lin_empty_array_unsatisfiable() {
    let mut m = Model::default();
    
    // Empty sum = 5 (unsatisfiable)
    m.bool_lin_eq(&[], &[], 5);
    
    let result = m.solve();
    assert!(result.is_err(), "Empty sum = 5 should be unsatisfiable");
}

#[test]
fn test_bool_lin_large_coefficients() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    
    // 100*b1 + 50*b2 = 150
    m.bool_lin_eq(&[100, 50], &[b1, b2], 150);
    
    let solution = m.solve().expect("Should find solution");
    
    // Only solution: b1=1, b2=1
    assert_eq!(solution[b1], Val::ValI(1));
    assert_eq!(solution[b2], Val::ValI(1));
}
