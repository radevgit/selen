//! Tests for float comparison reified constraints

use selen::prelude::*;

// ═══════════════════════════════════════════════════════════════════════
// float_eq_reif tests
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn test_float_eq_reif_true() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let b = m.bool();
    
    // Post reified constraint: b ⇔ (x = y)
    m.eq_reif(x, y, b);
    
    // Force b to be true
    m.new(b.eq(1));
    
    // Force x to 5.0
    m.new(x.eq(5.0));
    
    // Solve
    let solution = m.solve().expect("Should find solution");
    
    // y must be 5.0 because b=1 implies x=y
    if let (Val::ValF(x_val), Val::ValF(y_val), Val::ValI(b_val)) = 
           (solution[x], solution[y], solution[b]) {
        assert!((x_val - 5.0).abs() < 1e-6);
        assert!((y_val - 5.0).abs() < 1e-6);
        assert_eq!(b_val, 1);
    } else {
        panic!("Expected float and int values");
    }
}

#[test]
fn test_float_eq_reif_false() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let b = m.bool();
    
    // Post reified constraint: b ⇔ (x = y)
    m.eq_reif(x, y, b);
    
    // Force b to be false
    m.new(b.eq(0));
    
    // Force x to 5.0
    m.new(x.eq(5.0));
    
    // Solve
    let solution = m.solve().expect("Should find solution");
    
    // y must NOT be 5.0 because b=0 implies x≠y
    if let (Val::ValF(x_val), Val::ValF(y_val), Val::ValI(b_val)) = 
           (solution[x], solution[y], solution[b]) {
        assert!((x_val - 5.0).abs() < 1e-6);
        assert!((y_val - x_val).abs() > 1e-9); // y ≠ x
        assert_eq!(b_val, 0);
    } else {
        panic!("Expected float and int values");
    }
}

#[test]
fn test_float_eq_reif_inference_to_true() {
    let mut m = Model::default();
    
    let x = m.float(3.5, 3.5);  // Fixed to 3.5
    let y = m.float(3.5, 3.5);  // Fixed to 3.5
    let b = m.bool();
    
    // Post reified constraint: b ⇔ (x = y)
    m.eq_reif(x, y, b);
    
    let solution = m.solve().expect("Should find solution");
    
    // b must be 1 because x=y
    if let Val::ValI(b_val) = solution[b] {
        assert_eq!(b_val, 1);
    } else {
        panic!("Expected int value");
    }
}

// ═══════════════════════════════════════════════════════════════════════
// float_ne_reif tests
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn test_float_ne_reif_true() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let b = m.bool();
    
    // Post reified constraint: b ⇔ (x ≠ y)
    m.ne_reif(x, y, b);
    
    // Force b to be true
    m.new(b.eq(1));
    
    // Force x to 5.0
    m.new(x.eq(5.0));
    
    let solution = m.solve().expect("Should find solution");
    
    // y must NOT be 5.0 because b=1 implies x≠y
    if let (Val::ValF(x_val), Val::ValF(y_val), Val::ValI(b_val)) = 
           (solution[x], solution[y], solution[b]) {
        assert!((x_val - 5.0).abs() < 1e-6);
        assert!((y_val - x_val).abs() > 1e-9);
        assert_eq!(b_val, 1);
    } else {
        panic!("Expected float and int values");
    }
}

#[test]
fn test_float_ne_reif_false() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let b = m.bool();
    
    // Post reified constraint: b ⇔ (x ≠ y)
    m.ne_reif(x, y, b);
    
    // Force b to be false
    m.new(b.eq(0));
    
    // Force x to 7.5
    m.new(x.eq(7.5));
    
    let solution = m.solve().expect("Should find solution");
    
    // y must be 7.5 because b=0 implies x=y
    if let (Val::ValF(x_val), Val::ValF(y_val), Val::ValI(b_val)) = 
           (solution[x], solution[y], solution[b]) {
        assert!((x_val - 7.5).abs() < 1e-6);
        assert!((y_val - 7.5).abs() < 1e-6);
        assert_eq!(b_val, 0);
    } else {
        panic!("Expected float and int values");
    }
}

// ═══════════════════════════════════════════════════════════════════════
// float_lt_reif tests
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn test_float_lt_reif_true() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let b = m.bool();
    
    // Post reified constraint: b ⇔ (x < y)
    m.lt_reif(x, y, b);
    
    // Force b to be true
    m.new(b.eq(1));
    
    // Force x to 3.0
    m.new(x.eq(3.0));
    
    let solution = m.solve().expect("Should find solution");
    
    // y must be > 3.0 because b=1 implies x<y
    if let (Val::ValF(x_val), Val::ValF(y_val), Val::ValI(b_val)) = 
           (solution[x], solution[y], solution[b]) {
        assert!((x_val - 3.0).abs() < 1e-6);
        assert!(y_val > x_val);
        assert_eq!(b_val, 1);
    } else {
        panic!("Expected float and int values");
    }
}

#[test]
fn test_float_lt_reif_false() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let b = m.bool();
    
    // Post reified constraint: b ⇔ (x < y)
    m.lt_reif(x, y, b);
    
    // Force b to be false
    m.new(b.eq(0));
    
    // Force x to 7.0
    m.new(x.eq(7.0));
    
    let solution = m.solve().expect("Should find solution");
    
    // y must be ≥ 7.0 because b=0 implies x≥y
    if let (Val::ValF(x_val), Val::ValF(y_val), Val::ValI(b_val)) = 
           (solution[x], solution[y], solution[b]) {
        assert!((x_val - 7.0).abs() < 1e-6);
        assert!(y_val <= x_val + 1e-9);
        assert_eq!(b_val, 0);
    } else {
        panic!("Expected float and int values");
    }
}

#[test]
fn test_float_lt_reif_inference() {
    let mut m = Model::default();
    
    let x = m.float(2.0, 2.0);  // Fixed to 2.0
    let y = m.float(5.0, 5.0);  // Fixed to 5.0
    let b = m.bool();
    
    // Post reified constraint: b ⇔ (x < y)
    m.lt_reif(x, y, b);
    
    let solution = m.solve().expect("Should find solution");
    
    // b must be 1 because 2.0 < 5.0
    if let Val::ValI(b_val) = solution[b] {
        assert_eq!(b_val, 1);
    } else {
        panic!("Expected int value");
    }
}

// ═══════════════════════════════════════════════════════════════════════
// float_le_reif tests
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn test_float_le_reif_true() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let b = m.bool();
    
    // Post reified constraint: b ⇔ (x ≤ y)
    m.le_reif(x, y, b);
    
    // Force b to be true
    m.new(b.eq(1));
    
    // Force x to 4.5
    m.new(x.eq(4.5));
    
    let solution = m.solve().expect("Should find solution");
    
    // y must be ≥ 4.5 because b=1 implies x≤y
    if let (Val::ValF(x_val), Val::ValF(y_val), Val::ValI(b_val)) = 
           (solution[x], solution[y], solution[b]) {
        assert!((x_val - 4.5).abs() < 1e-6);
        assert!(y_val >= x_val - 1e-9);
        assert_eq!(b_val, 1);
    } else {
        panic!("Expected float and int values");
    }
}

#[test]
fn test_float_le_reif_false() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let b = m.bool();
    
    // Post reified constraint: b ⇔ (x ≤ y)
    m.le_reif(x, y, b);
    
    // Force b to be false
    m.new(b.eq(0));
    
    // Force x to 6.0
    m.new(x.eq(6.0));
    
    let solution = m.solve().expect("Should find solution");
    
    // y must be < 6.0 because b=0 implies x>y
    if let (Val::ValF(x_val), Val::ValF(y_val), Val::ValI(b_val)) = 
           (solution[x], solution[y], solution[b]) {
        assert!((x_val - 6.0).abs() < 1e-6);
        assert!(y_val < x_val);
        assert_eq!(b_val, 0);
    } else {
        panic!("Expected float and int values");
    }
}

// ═══════════════════════════════════════════════════════════════════════
// float_gt_reif tests
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn test_float_gt_reif_true() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let b = m.bool();
    
    // Post reified constraint: b ⇔ (x > y)
    m.gt_reif(x, y, b);
    
    // Force b to be true
    m.new(b.eq(1));
    
    // Force x to 8.0
    m.new(x.eq(8.0));
    
    let solution = m.solve().expect("Should find solution");
    
    // y must be < 8.0 because b=1 implies x>y
    if let (Val::ValF(x_val), Val::ValF(y_val), Val::ValI(b_val)) = 
           (solution[x], solution[y], solution[b]) {
        assert!((x_val - 8.0).abs() < 1e-6);
        assert!(y_val < x_val);
        assert_eq!(b_val, 1);
    } else {
        panic!("Expected float and int values");
    }
}

#[test]
fn test_float_gt_reif_false() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let b = m.bool();
    
    // Post reified constraint: b ⇔ (x > y)
    m.gt_reif(x, y, b);
    
    // Force b to be false
    m.new(b.eq(0));
    
    // Force x to 2.5
    m.new(x.eq(2.5));
    
    let solution = m.solve().expect("Should find solution");
    
    // y must be ≥ 2.5 because b=0 implies x≤y
    if let (Val::ValF(x_val), Val::ValF(y_val), Val::ValI(b_val)) = 
           (solution[x], solution[y], solution[b]) {
        assert!((x_val - 2.5).abs() < 1e-6);
        assert!(y_val >= x_val - 1e-9);
        assert_eq!(b_val, 0);
    } else {
        panic!("Expected float and int values");
    }
}

// ═══════════════════════════════════════════════════════════════════════
// float_ge_reif tests
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn test_float_ge_reif_true() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let b = m.bool();
    
    // Post reified constraint: b ⇔ (x ≥ y)
    m.ge_reif(x, y, b);
    
    // Force b to be true
    m.new(b.eq(1));
    
    // Force x to 5.5
    m.new(x.eq(5.5));
    
    let solution = m.solve().expect("Should find solution");
    
    // y must be ≤ 5.5 because b=1 implies x≥y
    if let (Val::ValF(x_val), Val::ValF(y_val), Val::ValI(b_val)) = 
           (solution[x], solution[y], solution[b]) {
        assert!((x_val - 5.5).abs() < 1e-6);
        assert!(y_val <= x_val + 1e-9);
        assert_eq!(b_val, 1);
    } else {
        panic!("Expected float and int values");
    }
}

#[test]
fn test_float_ge_reif_false() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let b = m.bool();
    
    // Post reified constraint: b ⇔ (x ≥ y)
    m.ge_reif(x, y, b);
    
    // Force b to be false
    m.new(b.eq(0));
    
    // Force x to 3.0
    m.new(x.eq(3.0));
    
    let solution = m.solve().expect("Should find solution");
    
    // y must be > 3.0 because b=0 implies x<y
    if let (Val::ValF(x_val), Val::ValF(y_val), Val::ValI(b_val)) = 
           (solution[x], solution[y], solution[b]) {
        assert!((x_val - 3.0).abs() < 1e-6);
        assert!(y_val > x_val);
        assert_eq!(b_val, 0);
    } else {
        panic!("Expected float and int values");
    }
}

#[test]
fn test_float_ge_reif_inference() {
    let mut m = Model::default();
    
    let x = m.float(7.0, 7.0);  // Fixed to 7.0
    let y = m.float(7.0, 7.0);  // Fixed to 7.0
    let b = m.bool();
    
    // Post reified constraint: b ⇔ (x ≥ y)
    m.ge_reif(x, y, b);
    
    let solution = m.solve().expect("Should find solution");
    
    // b must be 1 because 7.0 ≥ 7.0
    if let Val::ValI(b_val) = solution[b] {
        assert_eq!(b_val, 1);
    } else {
        panic!("Expected int value");
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Combined tests
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn test_float_reif_combined() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let z = m.float(0.0, 10.0);
    let b1 = m.bool();
    let b2 = m.bool();
    
    // b1 ⇔ (x < y)
    m.lt_reif(x, y, b1);
    
    // b2 ⇔ (y < z)
    m.lt_reif(y, z, b2);
    
    // Force both to be true
    m.new(b1.eq(1));
    m.new(b2.eq(1));
    
    // Force x to 2.0
    m.new(x.eq(2.0));
    
    let solution = m.solve().expect("Should find solution");
    
    // Must have x < y < z, so 2.0 < y < z
    if let (Val::ValF(x_val), Val::ValF(y_val), Val::ValF(z_val)) = 
           (solution[x], solution[y], solution[z]) {
        assert!((x_val - 2.0).abs() < 1e-6);
        assert!(x_val < y_val);
        assert!(y_val < z_val);
    } else {
        panic!("Expected float values");
    }
}
