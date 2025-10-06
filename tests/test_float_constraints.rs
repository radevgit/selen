//! Tests for float linear constraints (FlatZinc integration)

use selen::prelude::*;

// ═══════════════════════════════════════════════════════════════════════
// float_lin_eq tests
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn test_float_lin_eq_simple() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    
    // x + y = 7.5
    m.float_lin_eq(&[1.0, 1.0], &[x, y], 7.5);
    
    // Force x = 3.0
    m.new(x.eq(3.0));
    
    let solution = m.solve().expect("Should find solution");
    
    if let (Val::ValF(x_val), Val::ValF(y_val)) = (solution[x], solution[y]) {
        assert!((x_val - 3.0).abs() < 1e-6);
        assert!((y_val - 4.5).abs() < 1e-6); // y must be 4.5 to make x + y = 7.5
        assert!((x_val + y_val - 7.5).abs() < 1e-6);
    } else {
        panic!("Expected float values");
    }
}

#[test]
fn test_float_lin_eq_with_coefficients() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    
    // 2.5*x + 3.7*y = 18.5
    m.float_lin_eq(&[2.5, 3.7], &[x, y], 18.5);
    
    // Force x = 2.0 using builder API (should work with float_lin_eq)
    m.new(x.eq(2.0));
    
    let solution = m.solve().expect("Should find solution");
    
    if let (Val::ValF(x_val), Val::ValF(y_val)) = (solution[x], solution[y]) {
        assert!((x_val - 2.0).abs() < 1e-6);
        // 2.5*2.0 + 3.7*y = 18.5 => 5.0 + 3.7*y = 18.5 => 3.7*y = 13.5 => y ≈ 3.648648...
        let expected_y = (18.5 - 2.5 * x_val) / 3.7;
        assert!((y_val - expected_y).abs() < 1e-6);
        // Use 1e-5 tolerance to account for accumulated precision errors in propagation
        assert!((2.5 * x_val + 3.7 * y_val - 18.5).abs() < 1e-5);
    } else {
        panic!("Expected float values");
    }
}

#[test]
fn test_float_lin_eq_negative_coefficient() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    
    // 5.0*x - 2.0*y = 6.0
    m.float_lin_eq(&[5.0, -2.0], &[x, y], 6.0);
    
    // Force x = 4.0
    m.new(x.eq(4.0));
    
    let solution = m.solve().expect("Should find solution");
    
    if let (Val::ValF(x_val), Val::ValF(y_val)) = (solution[x], solution[y]) {
        assert!((x_val - 4.0).abs() < 1e-6);
        assert!((y_val - 7.0).abs() < 1e-6); // 5*4 - 2*7 = 20 - 14 = 6
        assert!((5.0 * x_val - 2.0 * y_val - 6.0).abs() < 1e-6);
    } else {
        panic!("Expected float values");
    }
}

#[test]
fn test_float_lin_eq_three_variables() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let z = m.float(0.0, 10.0);
    
    // 2.0*x + 3.0*y - z = 10.0
    m.float_lin_eq(&[2.0, 3.0, -1.0], &[x, y, z], 10.0);
    
    // Force x = 2.0 and y = 3.0
    m.new(x.eq(2.0));
    m.new(y.eq(3.0));
    
    let solution = m.solve().expect("Should find solution");
    
    if let (Val::ValF(x_val), Val::ValF(y_val), Val::ValF(z_val)) = 
        (solution[x], solution[y], solution[z]) {
        assert!((x_val - 2.0).abs() < 1e-6);
        assert!((y_val - 3.0).abs() < 1e-6);
        assert!((z_val - 3.0).abs() < 1e-6); // 2*2 + 3*3 - 3 = 4 + 9 - 3 = 10
        assert!((2.0 * x_val + 3.0 * y_val - z_val - 10.0).abs() < 1e-6);
    } else {
        panic!("Expected float values");
    }
}

// ═══════════════════════════════════════════════════════════════════════
// float_lin_le tests
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn test_float_lin_le_simple() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    
    // x + y ≤ 10.5
    m.float_lin_le(&[1.0, 1.0], &[x, y], 10.5);
    
    // Force x = 8.0
    m.new(x.eq(8.0));
    
    let solution = m.solve().expect("Should find solution");
    
    if let (Val::ValF(x_val), Val::ValF(y_val)) = (solution[x], solution[y]) {
        assert!((x_val - 8.0).abs() < 1e-6);
        // y must be ≤ 2.5 to satisfy x + y ≤ 10.5
        assert!(y_val <= 2.5 + 1e-6);
        assert!(x_val + y_val <= 10.5 + 1e-6);
    } else {
        panic!("Expected float values");
    }
}

#[test]
fn test_float_lin_le_with_coefficients() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    
    // 2.0*x + 3.0*y ≤ 20.0
    m.float_lin_le(&[2.0, 3.0], &[x, y], 20.0);
    
    // Force x = 5.0
    m.new(x.eq(5.0));
    
    let solution = m.solve().expect("Should find solution");
    
    if let (Val::ValF(x_val), Val::ValF(y_val)) = (solution[x], solution[y]) {
        assert!((x_val - 5.0).abs() < 1e-6);
        // 2*5 + 3*y ≤ 20 => 10 + 3*y ≤ 20 => 3*y ≤ 10 => y ≤ 3.333...
        assert!(y_val <= 10.0/3.0 + 1e-6);
        assert!(2.0 * x_val + 3.0 * y_val <= 20.0 + 1e-6);
    } else {
        panic!("Expected float values");
    }
}

#[test]
fn test_float_lin_le_negative_coefficient() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    
    // x - y ≤ 5.0
    m.float_lin_le(&[1.0, -1.0], &[x, y], 5.0);
    
    // Force x = 8.0
    m.new(x.eq(8.0));
    
    let solution = m.solve().expect("Should find solution");
    
    if let (Val::ValF(x_val), Val::ValF(y_val)) = (solution[x], solution[y]) {
        assert!((x_val - 8.0).abs() < 1e-6);
        // 8 - y ≤ 5 => y ≥ 3
        assert!(y_val >= 3.0 - 1e-6);
        assert!(x_val - y_val <= 5.0 + 1e-6);
    } else {
        panic!("Expected float values");
    }
}

// ═══════════════════════════════════════════════════════════════════════
// float_lin_ne tests
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn test_float_lin_ne_simple() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    
    // x + y ≠ 5.0
    m.float_lin_ne(&[1.0, 1.0], &[x, y], 5.0);
    
    // Force x = 2.0
    m.new(x.eq(2.0));
    
    let solution = m.solve().expect("Should find solution");
    
    if let (Val::ValF(x_val), Val::ValF(y_val)) = (solution[x], solution[y]) {
        assert!((x_val - 2.0).abs() < 1e-6);
        // y can be anything except 3.0 (which would make x + y = 5.0)
        assert!((y_val - 3.0).abs() > 1e-6);
        assert!((x_val + y_val - 5.0).abs() > 1e-6);
    } else {
        panic!("Expected float values");
    }
}

#[test]
fn test_float_lin_ne_with_coefficients() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    
    // 2.0*x + 3.0*y ≠ 12.0
    m.float_lin_ne(&[2.0, 3.0], &[x, y], 12.0);
    
    // Force x = 3.0
    m.new(x.eq(3.0));
    
    let solution = m.solve().expect("Should find solution");
    
    if let (Val::ValF(x_val), Val::ValF(y_val)) = (solution[x], solution[y]) {
        assert!((x_val - 3.0).abs() < 1e-6);
        // y can be anything except 2.0 (which would make 2*3 + 3*2 = 12)
        assert!((y_val - 2.0).abs() > 1e-6);
        assert!((2.0 * x_val + 3.0 * y_val - 12.0).abs() > 1e-6);
    } else {
        panic!("Expected float values");
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Edge cases
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn test_float_lin_eq_single_variable() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 10.0);
    
    // 3.5*x = 7.0
    m.float_lin_eq(&[3.5], &[x], 7.0);
    
    let solution = m.solve().expect("Should find solution");
    
    if let Val::ValF(x_val) = solution[x] {
        assert!((x_val - 2.0).abs() < 1e-6); // x must be 2.0
        assert!((3.5 * x_val - 7.0).abs() < 1e-6);
    } else {
        panic!("Expected float value");
    }
}

#[test]
fn test_float_lin_le_single_variable() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 10.0);
    
    // 2.0*x ≤ 10.0
    m.float_lin_le(&[2.0], &[x], 10.0);
    
    let solution = m.solve().expect("Should find solution");
    
    // x must be ≤ 5.0
    if let Val::ValF(x_val) = solution[x] {
        assert!(x_val <= 5.0 + 1e-6);
        assert!(2.0 * x_val <= 10.0 + 1e-6);
    } else {
        panic!("Expected float value");
    }
}

#[test]
fn test_float_lin_eq_infeasible() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 5.0);
    let y = m.float(0.0, 5.0);
    
    // x + y = 20.0 (impossible with given domains)
    m.float_lin_eq(&[1.0, 1.0], &[x, y], 20.0);
    
    let result = m.solve();
    
    assert!(result.is_err(), "Should not find solution");
}

#[test]
fn test_float_lin_eq_mismatched_lengths() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    
    // Mismatched lengths should create an unsatisfiable constraint
    m.float_lin_eq(&[1.0, 2.0, 3.0], &[x, y], 10.0);
    
    let result = m.solve();
    assert!(result.is_err(), "Should not find solution due to mismatched lengths");
}

#[test]
fn test_float_lin_le_mismatched_lengths() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    
    // Mismatched lengths should create an unsatisfiable constraint
    m.float_lin_le(&[1.0], &[x, y], 10.0);
    
    let result = m.solve();
    assert!(result.is_err(), "Should not find solution due to mismatched lengths");
}

// ═══════════════════════════════════════════════════════════════════════
// Real-world example from SELEN_MISSING_FEATURES.md
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn test_loan_example() {
    // Example: loan amount calculation
    // 1.05*principal + 1.03*interest - 1.0*payment = balance
    let mut m = Model::default();
    
    let principal = m.float(1000.0, 10000.0);
    let interest = m.float(0.0, 1000.0);
    let payment = m.float(0.0, 5000.0);
    let balance = m.float(0.0, 15000.0);
    
    // Linear equation: 1.05*principal + 1.03*interest - 1.0*payment = balance
    m.float_lin_eq(&[1.05, 1.03, -1.0, -1.0], 
                   &[principal, interest, payment, balance], 
                   0.0);
    
    // Set known values
    m.new(principal.eq(5000.0));  // $5000 principal
    m.new(interest.eq(250.0));    // $250 interest
    m.new(payment.eq(1000.0));    // $1000 payment
    
    let solution = m.solve().expect("Should find solution");
    
    if let Val::ValF(balance_val) = solution[balance] {
        // Expected: 1.05*5000 + 1.03*250 - 1000 = 5250 + 257.5 - 1000 = 4507.5
        let expected_balance = 1.05 * 5000.0 + 1.03 * 250.0 - 1000.0;
        assert!((balance_val - expected_balance).abs() < 1e-6);
    } else {
        panic!("Expected float value for balance");
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Reified float linear constraint tests
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn test_float_lin_eq_reif_true() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let b = m.bool();
    
    // b ⇔ (2.0*x + 3.0*y = 12.0)
    m.float_lin_eq_reif(&[2.0, 3.0], &[x, y], 12.0, b);
    
    // Force x = 3.0, y = 2.0 (which makes 2*3 + 3*2 = 12)
    m.new(x.eq(3.0));
    m.new(y.eq(2.0));
    
    let solution = m.solve().expect("Should find solution");
    
    assert_eq!(solution[b], Val::ValI(1), "b should be 1 when constraint holds");
}

#[test]
fn test_float_lin_eq_reif_false() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let b = m.bool();
    
    // b ⇔ (2.0*x + 3.0*y = 12.0)
    m.float_lin_eq_reif(&[2.0, 3.0], &[x, y], 12.0, b);
    
    // Force x = 1.0, y = 1.0 (which makes 2*1 + 3*1 = 5, not 12)
    m.new(x.eq(1.0));
    m.new(y.eq(1.0));
    
    let solution = m.solve().expect("Should find solution");
    
    assert_eq!(solution[b], Val::ValI(0), "b should be 0 when constraint doesn't hold");
}

#[test]
fn test_float_lin_eq_reif_force_true() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let b = m.bool();
    
    // b ⇔ (x + y = 7.5)
    m.float_lin_eq_reif(&[1.0, 1.0], &[x, y], 7.5, b);
    
    // Force b = 1 and x = 3.0
    m.new(b.eq(1));
    m.new(x.eq(3.0));
    
    let solution = m.solve().expect("Should find solution");
    
    if let Val::ValF(y_val) = solution[y] {
        assert!((y_val - 4.5).abs() < 1e-6, "y should be 4.5 when b=1 and x=3.0");
    }
}

#[test]
fn test_float_lin_eq_reif_force_false() {
    let mut m = Model::default();
    
    let x = m.float(2.0, 4.0);
    let y = m.float(2.0, 4.0);
    let b = m.bool();
    
    // b ⇔ (x + y = 5.0)
    m.float_lin_eq_reif(&[1.0, 1.0], &[x, y], 5.0, b);
    
    // Force b = 0 (constraint must not hold)
    m.new(b.eq(0));
    
    let solution = m.solve().expect("Should find solution");
    
    if let (Val::ValF(x_val), Val::ValF(y_val)) = (solution[x], solution[y]) {
        assert!((x_val + y_val - 5.0).abs() > 1e-6, "x + y should not equal 5.0 when b=0");
    }
}

#[test]
fn test_float_lin_le_reif_true() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let b = m.bool();
    
    // b ⇔ (x + y ≤ 10.0)
    m.float_lin_le_reif(&[1.0, 1.0], &[x, y], 10.0, b);
    
    // Force x = 4.0, y = 5.0 (which makes 4 + 5 = 9 ≤ 10)
    m.new(x.eq(4.0));
    m.new(y.eq(5.0));
    
    let solution = m.solve().expect("Should find solution");
    
    assert_eq!(solution[b], Val::ValI(1), "b should be 1 when constraint holds");
}

#[test]
fn test_float_lin_le_reif_false() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 20.0);
    let y = m.float(0.0, 20.0);
    let b = m.bool();
    
    // b ⇔ (x + y ≤ 10.0)
    m.float_lin_le_reif(&[1.0, 1.0], &[x, y], 10.0, b);
    
    // Force x = 8.0, y = 5.0 (which makes 8 + 5 = 13 > 10)
    m.new(x.eq(8.0));
    m.new(y.eq(5.0));
    
    let solution = m.solve().expect("Should find solution");
    
    assert_eq!(solution[b], Val::ValI(0), "b should be 0 when constraint doesn't hold");
}

#[test]
fn test_float_lin_le_reif_force_true() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let b = m.bool();
    
    // b ⇔ (2.0*x + 3.0*y ≤ 20.0)
    m.float_lin_le_reif(&[2.0, 3.0], &[x, y], 20.0, b);
    
    // Force b = 1 and x = 5.0
    m.new(b.eq(1));
    m.new(x.eq(5.0));
    
    let solution = m.solve().expect("Should find solution");
    
    if let (Val::ValF(x_val), Val::ValF(y_val)) = (solution[x], solution[y]) {
        assert!(2.0 * x_val + 3.0 * y_val <= 20.0 + 1e-6, 
                "2*x + 3*y should be ≤ 20.0 when b=1");
    }
}

#[test]
fn test_float_lin_ne_reif_true() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let b = m.bool();
    
    // b ⇔ (x + y ≠ 5.0)
    m.float_lin_ne_reif(&[1.0, 1.0], &[x, y], 5.0, b);
    
    // Force x = 2.0, y = 1.0 (which makes 2 + 1 = 3 ≠ 5)
    m.new(x.eq(2.0));
    m.new(y.eq(1.0));
    
    let solution = m.solve().expect("Should find solution");
    
    assert_eq!(solution[b], Val::ValI(1), "b should be 1 when constraint holds");
}

#[test]
fn test_float_lin_ne_reif_false() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let b = m.bool();
    
    // b ⇔ (x + y ≠ 5.0)
    m.float_lin_ne_reif(&[1.0, 1.0], &[x, y], 5.0, b);
    
    // Force x = 2.0, y = 3.0 (which makes 2 + 3 = 5, so NOT not-equal)
    m.new(x.eq(2.0));
    m.new(y.eq(3.0));
    
    let solution = m.solve().expect("Should find solution");
    
    assert_eq!(solution[b], Val::ValI(0), "b should be 0 when values are equal");
}

#[test]
fn test_float_lin_ne_reif_force_true() {
    let mut m = Model::default();
    
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    let b = m.bool();
    
    // b ⇔ (2.0*x + 3.0*y ≠ 12.0)
    m.float_lin_ne_reif(&[2.0, 3.0], &[x, y], 12.0, b);
    
    // Force b = 1 and x = 3.0
    m.new(b.eq(1));
    m.new(x.eq(3.0));
    
    let solution = m.solve().expect("Should find solution");
    
    if let (Val::ValF(x_val), Val::ValF(y_val)) = (solution[x], solution[y]) {
        assert!((2.0 * x_val + 3.0 * y_val - 12.0).abs() > 1e-6,
                "2*x + 3*y should not equal 12.0 when b=1");
    }
}
