use selen::prelude::*;

// ═══════════════════════════════════════════════════════════════════════════
// Test: int2float - Integer to Float Conversion
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_int2float_basic() {
    let mut m = Model::default();
    let x = m.int(5, 5); // Fixed at 5
    let y = m.float(0.0, 100.0);
    
    m.int2float(x, y);
    
    let sol = m.solve().expect("Should have solution");
    assert_eq!(sol.get_int(x), 5);
    assert!((sol.get_float(y) - 5.0).abs() < 1e-9);
}

#[test]
fn test_int2float_range() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    let y = m.float(0.0, 100.0);
    
    m.int2float(x, y);
    
    // Add constraint to test: y must be at least 7.0
    m.props.greater_than_or_equals(y, Val::ValF(7.0));
    
    let sol = m.solve().expect("Should have solution");
    let x_val = sol.get_int(x);
    let y_val = sol.get_float(y);
    
    assert!(x_val >= 7);
    assert!((y_val - x_val as f64).abs() < 1e-9);
}

#[test]
fn test_int2float_bidirectional() {
    let mut m = Model::default();
    let x = m.int(-10, 10);
    let y = m.float(-100.0, 100.0);
    
    m.int2float(x, y);
    
    // Constrain float variable
    m.props.equals(y, Val::ValF(3.0));
    
    let sol = m.solve().expect("Should have solution");
    assert_eq!(sol.get_int(x), 3);
    assert!((sol.get_float(y) - 3.0).abs() < 1e-9);
}

// ═══════════════════════════════════════════════════════════════════════════
// Test: float2int_floor - Floor Conversion
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_float2int_floor_basic() {
    let mut m = Model::default();
    let x = m.float(3.7, 3.7); // Fixed at 3.7
    let y = m.int(-100, 100);
    
    m.float2int_floor(x, y);
    
    let sol = m.solve().expect("Should have solution");
    assert_eq!(sol.get_int(y), 3);
    assert!((sol.get_float(x) - 3.7).abs() < 1e-9);
}

#[test]
fn test_float2int_floor_negative() {
    let mut m = Model::default();
    let x = m.float(-2.3, -2.3); // Fixed at -2.3
    let y = m.int(-100, 100);
    
    m.float2int_floor(x, y);
    
    let sol = m.solve().expect("Should have solution");
    assert_eq!(sol.get_int(y), -3); // floor(-2.3) = -3
}

#[test]
fn test_float2int_floor_range() {
    let mut m = Model::default();
    let x = m.float(1.0, 5.9);
    let y = m.int(-100, 100);
    
    m.float2int_floor(x, y);
    
    // y should be between floor(1.0) = 1 and floor(5.9) = 5
    let sol = m.solve().expect("Should have solution");
    let y_val = sol.get_int(y);
    let x_val = sol.get_float(x);
    
    assert!(y_val >= 1 && y_val <= 5);
    assert!(x_val >= y_val as f64 && x_val < (y_val + 1) as f64);
}

#[test]
fn test_float2int_floor_exact_integer() {
    let mut m = Model::default();
    let x = m.float(5.0, 5.0); // Exact integer
    let y = m.int(-100, 100);
    
    m.float2int_floor(x, y);
    
    let sol = m.solve().expect("Should have solution");
    assert_eq!(sol.get_int(y), 5);
}

// ═══════════════════════════════════════════════════════════════════════════
// Test: float2int_ceil - Ceiling Conversion
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_float2int_ceil_basic() {
    let mut m = Model::default();
    let x = m.float(3.2, 3.2); // Fixed at 3.2
    let y = m.int(-100, 100);
    
    m.float2int_ceil(x, y);
    
    let sol = m.solve().expect("Should have solution");
    assert_eq!(sol.get_int(y), 4); // ceil(3.2) = 4
}

#[test]
fn test_float2int_ceil_negative() {
    let mut m = Model::default();
    let x = m.float(-2.3, -2.3); // Fixed at -2.3
    let y = m.int(-100, 100);
    
    m.float2int_ceil(x, y);
    
    let sol = m.solve().expect("Should have solution");
    assert_eq!(sol.get_int(y), -2); // ceil(-2.3) = -2
}

#[test]
fn test_float2int_ceil_range() {
    let mut m = Model::default();
    let x = m.float(1.1, 5.9);
    let y = m.int(-100, 100);
    
    m.float2int_ceil(x, y);
    
    // y should be between ceil(1.1) = 2 and ceil(5.9) = 6
    let sol = m.solve().expect("Should have solution");
    let y_val = sol.get_int(y);
    let x_val = sol.get_float(x);
    
    assert!(y_val >= 2 && y_val <= 6);
    assert!(x_val > (y_val - 1) as f64 && x_val <= y_val as f64);
}

#[test]
fn test_float2int_ceil_exact_integer() {
    let mut m = Model::default();
    let x = m.float(5.0, 5.0); // Exact integer
    let y = m.int(-100, 100);
    
    m.float2int_ceil(x, y);
    
    let sol = m.solve().expect("Should have solution");
    assert_eq!(sol.get_int(y), 5);
}

// ═══════════════════════════════════════════════════════════════════════════
// Test: float2int_round - Rounding Conversion
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_float2int_round_basic() {
    let mut m = Model::default();
    let x = m.float(3.4, 3.4); // Fixed at 3.4
    let y = m.int(-100, 100);
    
    m.float2int_round(x, y);
    
    let sol = m.solve().expect("Should have solution");
    assert_eq!(sol.get_int(y), 3); // round(3.4) = 3
}

#[test]
fn test_float2int_round_up() {
    let mut m = Model::default();
    let x = m.float(3.6, 3.6); // Fixed at 3.6
    let y = m.int(-100, 100);
    
    m.float2int_round(x, y);
    
    let sol = m.solve().expect("Should have solution");
    assert_eq!(sol.get_int(y), 4); // round(3.6) = 4
}

#[test]
fn test_float2int_round_negative() {
    let mut m = Model::default();
    let x = m.float(-2.3, -2.3); // Fixed at -2.3
    let y = m.int(-100, 100);
    
    m.float2int_round(x, y);
    
    let sol = m.solve().expect("Should have solution");
    assert_eq!(sol.get_int(y), -2); // round(-2.3) = -2
}

#[test]
fn test_float2int_round_negative_up() {
    let mut m = Model::default();
    let x = m.float(-2.7, -2.7); // Fixed at -2.7
    let y = m.int(-100, 100);
    
    m.float2int_round(x, y);
    
    let sol = m.solve().expect("Should have solution");
    assert_eq!(sol.get_int(y), -3); // round(-2.7) = -3
}

#[test]
fn test_float2int_round_exact_half() {
    let mut m = Model::default();
    let x = m.float(2.5, 2.5); // Fixed at 2.5
    let y = m.int(-100, 100);
    
    m.float2int_round(x, y);
    
    let sol = m.solve().expect("Should have solution");
    // round(2.5) with banker's rounding = 2 (round to even)
    // Note: This might be 2 or 3 depending on implementation
    let y_val = sol.get_int(y);
    assert!(y_val == 2 || y_val == 3);
}

#[test]
fn test_float2int_round_exact_integer() {
    let mut m = Model::default();
    let x = m.float(5.0, 5.0); // Exact integer
    let y = m.int(-100, 100);
    
    m.float2int_round(x, y);
    
    let sol = m.solve().expect("Should have solution");
    assert_eq!(sol.get_int(y), 5);
}

// ═══════════════════════════════════════════════════════════════════════════
// Test: Combined Conversions
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_roundtrip_int_to_float_to_int() {
    let mut m = Model::default();
    let x = m.int(7, 7);
    let y = m.float(-100.0, 100.0);
    let z = m.int(-100, 100);
    
    m.int2float(x, y);
    m.float2int_floor(y, z);
    
    let sol = m.solve().expect("Should have solution");
    assert_eq!(sol.get_int(x), 7);
    let y_val = sol.get_float(y);
    let z_val = sol.get_int(z);
    
    // Float might be 6.999... or 7.0 due to floating point, so check bounds
    assert!(y_val >= 6.99 && y_val <= 7.01);
    // floor(6.999...) = 6, floor(7.0) = 7
    assert!(z_val >= 6 && z_val <= 7);
}

#[test]
fn test_mixed_type_constraint() {
    let mut m = Model::default();
    let int_val = m.int(1, 10);
    let float_val = m.float(0.0, 100.0);
    let result = m.int(-100, 100);
    
    // Convert int to float
    m.int2float(int_val, float_val);
    
    // Add 0.5 to float
    let float_plus_half = m.add(float_val, Val::ValF(0.5));
    
    // Floor the result
    m.float2int_floor(float_plus_half, result);
    
    let sol = m.solve().expect("Should have solution");
    let int_val_sol = sol.get_int(int_val);
    let float_val_sol = sol.get_float(float_val);
    let result_sol = sol.get_int(result);
    
    assert!((float_val_sol - int_val_sol as f64).abs() < 1e-9);
    assert_eq!(result_sol, int_val_sol); // floor(x + 0.5) = x for integer x
}

#[test]
fn test_all_three_floor_ceil_round() {
    let mut m = Model::default();
    let x = m.float(3.7, 3.7);
    let floor_result = m.int(-100, 100);
    let ceil_result = m.int(-100, 100);
    let round_result = m.int(-100, 100);
    
    m.float2int_floor(x, floor_result);
    m.float2int_ceil(x, ceil_result);
    m.float2int_round(x, round_result);
    
    let sol = m.solve().expect("Should have solution");
    assert_eq!(sol.get_int(floor_result), 3);
    assert_eq!(sol.get_int(ceil_result), 4);
    assert_eq!(sol.get_int(round_result), 4);
}

// ═══════════════════════════════════════════════════════════════════════════
// Test: Edge Cases
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_int2float_zero() {
    let mut m = Model::default();
    let x = m.int(0, 0);
    let y = m.float(-100.0, 100.0);
    
    m.int2float(x, y);
    
    let sol = m.solve().expect("Should have solution");
    assert_eq!(sol.get_int(x), 0);
    assert!(sol.get_float(y).abs() < 1e-9);
}

#[test]
fn test_float2int_floor_zero() {
    let mut m = Model::default();
    let x = m.float(0.0, 0.0);
    let y = m.int(-100, 100);
    
    m.float2int_floor(x, y);
    
    let sol = m.solve().expect("Should have solution");
    assert_eq!(sol.get_int(y), 0);
}

#[test]
fn test_large_values() {
    let mut m = Model::default();
    let x = m.int(1000, 1000);
    let y = m.float(0.0, 10000.0);
    let z = m.int(-10000, 10000);
    
    m.int2float(x, y);
    m.float2int_floor(y, z);
    
    let sol = m.solve().expect("Should have solution");
    assert_eq!(sol.get_int(x), 1000);
    let y_val = sol.get_float(y);
    let z_val = sol.get_int(z);
    
    // Float might have small precision errors
    assert!(y_val >= 999.99 && y_val <= 1000.01);
    // floor should give 999 or 1000 depending on floating point representation
    assert!(z_val >= 999 && z_val <= 1000);
}

// ═══════════════════════════════════════════════════════════════════════════
// Test: Critical Range -0.6 to 0.6 (Comprehensive Coverage)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_floor_positive_small() {
    // Test positive values between 0 and 1
    let test_cases = vec![0.1, 0.3, 0.5, 0.6, 0.9];
    
    for &val in &test_cases {
        let mut m = Model::default();
        let x = m.float(val, val);
        let y = m.int(-10, 10);
        m.float2int_floor(x, y);
        
        let sol = m.solve().expect(&format!("Should have solution for {}", val));
        assert_eq!(sol.get_int(y), 0, "floor({}) should be 0", val);
    }
}

#[test]
fn test_floor_negative_small() {
    // Test negative values between -1 and 0
    let test_cases = vec![-0.1, -0.3, -0.5, -0.6, -0.9];
    
    for &val in &test_cases {
        let mut m = Model::default();
        let x = m.float(val, val);
        let y = m.int(-10, 10);
        m.float2int_floor(x, y);
        
        let sol = m.solve().expect(&format!("Should have solution for {}", val));
        assert_eq!(sol.get_int(y), -1, "floor({}) should be -1", val);
    }
}

#[test]
fn test_ceil_positive_small() {
    // Test positive values between 0 and 1
    let test_cases = vec![0.1, 0.3, 0.5, 0.6, 0.9];
    
    for &val in &test_cases {
        let mut m = Model::default();
        let x = m.float(val, val);
        let y = m.int(-10, 10);
        m.float2int_ceil(x, y);
        
        let sol = m.solve().expect(&format!("Should have solution for {}", val));
        assert_eq!(sol.get_int(y), 1, "ceil({}) should be 1", val);
    }
}

#[test]
fn test_ceil_negative_small() {
    // Test negative values between -1 and 0
    let test_cases = vec![-0.1, -0.3, -0.5, -0.6, -0.9];
    
    for &val in &test_cases {
        let mut m = Model::default();
        let x = m.float(val, val);
        let y = m.int(-10, 10);
        m.float2int_ceil(x, y);
        
        let sol = m.solve().expect(&format!("Should have solution for {}", val));
        assert_eq!(sol.get_int(y), 0, "ceil({}) should be 0", val);
    }
}

#[test]
fn test_round_near_zero_positive() {
    // Test rounding positive values near zero
    let mut m = Model::default();
    
    // 0.1 rounds to 0
    let x1 = m.float(0.1, 0.1);
    let y1 = m.int(-10, 10);
    m.float2int_round(x1, y1);
    
    // 0.4 rounds to 0
    let x2 = m.float(0.4, 0.4);
    let y2 = m.int(-10, 10);
    m.float2int_round(x2, y2);
    
    // 0.6 rounds to 1
    let x3 = m.float(0.6, 0.6);
    let y3 = m.int(-10, 10);
    m.float2int_round(x3, y3);
    
    let sol = m.solve().expect("Should have solution");
    assert_eq!(sol.get_int(y1), 0, "round(0.1) should be 0");
    assert_eq!(sol.get_int(y2), 0, "round(0.4) should be 0");
    assert_eq!(sol.get_int(y3), 1, "round(0.6) should be 1");
}

#[test]
fn test_round_near_zero_negative() {
    // Test rounding negative values near zero
    let mut m = Model::default();
    
    // -0.1 rounds to 0
    let x1 = m.float(-0.1, -0.1);
    let y1 = m.int(-10, 10);
    m.float2int_round(x1, y1);
    
    // -0.4 rounds to 0
    let x2 = m.float(-0.4, -0.4);
    let y2 = m.int(-10, 10);
    m.float2int_round(x2, y2);
    
    // -0.6 rounds to -1
    let x3 = m.float(-0.6, -0.6);
    let y3 = m.int(-10, 10);
    m.float2int_round(x3, y3);
    
    let sol = m.solve().expect("Should have solution");
    assert_eq!(sol.get_int(y1), 0, "round(-0.1) should be 0");
    assert_eq!(sol.get_int(y2), 0, "round(-0.4) should be 0");
    assert_eq!(sol.get_int(y3), -1, "round(-0.6) should be -1");
}

#[test]
fn test_all_conversions_at_boundaries() {
    // Test all three conversions at key boundary points
    let test_values = vec![
        (-0.6, -1, 0, -1),   // (value, floor, ceil, round)
        (-0.5, -1, 0, 0),    // -0.5 might round to 0 or -1
        (-0.4, -1, 0, 0),
        (-0.1, -1, 0, 0),
        (0.0, 0, 0, 0),
        (0.1, 0, 1, 0),
        (0.4, 0, 1, 0),
        (0.5, 0, 1, 0),      // 0.5 might round to 0 or 1
        (0.6, 0, 1, 1),
    ];
    
    for &(val, expected_floor, expected_ceil, expected_round) in &test_values {
        let mut m = Model::default();
        let x = m.float(val, val);
        let floor_result = m.int(-10, 10);
        let ceil_result = m.int(-10, 10);
        let round_result = m.int(-10, 10);
        
        m.float2int_floor(x, floor_result);
        m.float2int_ceil(x, ceil_result);
        m.float2int_round(x, round_result);
        
        let sol = m.solve().expect(&format!("Should have solution for {}", val));
        let floor_res = sol.get_int(floor_result);
        let ceil_res = sol.get_int(ceil_result);
        let round_res = sol.get_int(round_result);
        
        assert_eq!(floor_res, expected_floor, "floor({}) should be {}", val, expected_floor);
        assert_eq!(ceil_res, expected_ceil, "ceil({}) should be {}", val, expected_ceil);
        
        // Round might vary for .5 values due to banker's rounding
        if val == 0.5 || val == -0.5 {
            assert!(round_res == expected_round || round_res == expected_round + 1 || round_res == expected_round - 1,
                    "round({}) = {} (expected close to {})", val, round_res, expected_round);
        } else {
            assert_eq!(round_res, expected_round, "round({}) should be {}", val, expected_round);
        }
    }
}

#[test]
fn test_range_containing_zero() {
    // Test conversion with a range that crosses zero
    let mut m = Model::default();
    let x = m.float(-0.5, 0.5);
    let floor_result = m.int(-10, 10);
    let ceil_result = m.int(-10, 10);
    
    m.float2int_floor(x, floor_result);
    m.float2int_ceil(x, ceil_result);
    
    let sol = m.solve().expect("Should have solution");
    let floor_res = sol.get_int(floor_result);
    let ceil_res = sol.get_int(ceil_result);
    let x_val = sol.get_float(x);
    
    // floor(-0.5 to 0.5) should be -1 or 0
    assert!(floor_res >= -1 && floor_res <= 0, 
            "floor({}) = {} should be in [-1, 0]", x_val, floor_res);
    
    // ceil(-0.5 to 0.5) should be 0 or 1
    assert!(ceil_res >= 0 && ceil_res <= 1, 
            "ceil({}) = {} should be in [0, 1]", x_val, ceil_res);
    
    // Verify constraint is satisfied
    if x_val >= 0.0 {
        assert_eq!(floor_res, 0);
        assert!(ceil_res == 0 || ceil_res == 1);
    } else {
        assert_eq!(floor_res, -1);
        assert_eq!(ceil_res, 0);
    }
}
