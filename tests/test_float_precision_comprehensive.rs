use cspsolver::prelude::*;

/// Test suite for comprehensive floating-point precision handling
/// Tests various constraint types with different floating-point constants

#[test]
fn test_less_than_precision_simple() {
    let mut model = Model::with_float_precision(4);
    
    let x = model.new_var_float(1.0, 10.0);
    model.lt(x, float(5.5));
    
    let solution = model.maximize(x).expect("Should have solution");
    
    let Val::ValF(x_val) = solution[x] else { panic!("Expected float") };
    assert!(x_val < 5.5);
    // With ULP-based precision, should be prev_float(5.5)
    assert!((x_val - 5.499999999999999).abs() < 1e-15);
}

#[test]
fn test_less_than_precision_complex() {
    let mut model = Model::with_float_precision(4);
    
    let x = model.new_var_float(1.0, 10.0);
    model.lt(x, float(5.132415232356));
    
    let solution = model.maximize(x).expect("Should have solution");
    
    let Val::ValF(x_val) = solution[x] else { panic!("Expected float") };
    assert!(x_val < 5.132415232356);
    // Should be the largest representable float less than 5.132415232356
    let expected = 5.132415232355999; // prev_float(5.132415232356)
    assert!((x_val - expected).abs() < 1e-14);
}

#[test]
fn test_less_than_precision_very_small() {
    let mut model = Model::with_float_precision(8);
    
    let x = model.new_var_float(0.0, 1.0);
    model.lt(x, float(0.0000012345));
    
    let solution = model.maximize(x).expect("Should have solution");
    
    let Val::ValF(x_val) = solution[x] else { panic!("Expected float") };
    assert!(x_val < 0.0000012345);
    // Should be prev_float(0.0000012345)
    let expected = 0.0000012344999999999998;
    assert!((x_val - expected).abs() < 1e-18);
}

#[test]
fn test_greater_than_precision_simple() {
    let mut model = Model::with_float_precision(4);
    
    let x = model.new_var_float(1.0, 10.0);
    model.gt(x, float(3.7));
    
    let solution = model.minimize(x).expect("Should have solution");
    
    let Val::ValF(x_val) = solution[x] else { panic!("Expected float") };
    assert!(x_val > 3.7);
    // With ULP-based precision, should be next_float(3.7)
    let expected = 3.7000000000000006; // next_float(3.7)
    assert!((x_val - expected).abs() < 1e-15);
}

#[test]
fn test_greater_than_precision_complex() {
    let mut model = Model::with_float_precision(4);
    
    let x = model.new_var_float(1.0, 10.0);
    model.gt(x, float(2.987654321098765));
    
    let solution = model.minimize(x).expect("Should have solution");
    
    let Val::ValF(x_val) = solution[x] else { panic!("Expected float") };
    assert!(x_val > 2.987654321098765);
    // Should be the smallest representable float greater than 2.987654321098765
    let expected = 2.9876543210987654; // next_float(2.987654321098765)
    assert!((x_val - expected).abs() < 1e-14);
}

#[test]
fn test_greater_than_precision_near_zero() {
    let mut model = Model::with_float_precision(8);
    
    let x = model.new_var_float(-1.0, 1.0);
    model.gt(x, float(0.00000789123));
    
    let solution = model.minimize(x).expect("Should have solution");
    
    let Val::ValF(x_val) = solution[x] else { panic!("Expected float") };
    assert!(x_val > 0.00000789123);
    // Should be next_float(0.00000789123)
    let expected = 0.000007891230000000001;
    assert!((x_val - expected).abs() < 1e-17);
}

#[test]
fn test_less_than_or_equals_precision() {
    let mut model = Model::with_float_precision(4);
    
    let x = model.new_var_float(1.0, 10.0);
    model.le(x, float(4.333333333333333));
    
    let solution = model.maximize(x).expect("Should have solution");
    
    let Val::ValF(x_val) = solution[x] else { panic!("Expected float") };
    assert!(x_val <= 4.333333333333333);
    // For <=, should be exactly the bound (no ULP adjustment needed)
    assert!((x_val - 4.333333333333333).abs() < 1e-15);
}

#[test]
fn test_greater_than_or_equals_precision() {
    let mut model = Model::with_float_precision(4);
    
    let x = model.new_var_float(1.0, 10.0);
    model.ge(x, float(6.777777777777778));
    
    let solution = model.minimize(x).expect("Should have solution");
    
    let Val::ValF(x_val) = solution[x] else { panic!("Expected float") };
    assert!(x_val >= 6.777777777777778);
    // For >=, should be exactly the bound (no ULP adjustment needed)
    assert!((x_val - 6.777777777777778).abs() < 1e-15);
}

#[test]
fn test_precision_chain_constraints() {
    let mut model = Model::with_float_precision(4);
    
    let x = model.new_var_float(5.12345678, 5.12345679);  // Narrow initial domain
    // Create a narrow window using both < and >
    model.gt(x, float(5.123456789));
    model.lt(x, float(5.123456790));
    
    let solution = model.maximize(x).expect("Should have solution");
    
    let Val::ValF(x_val) = solution[x] else { panic!("Expected float") };
    assert!(x_val > 5.123456789);
    assert!(x_val < 5.123456790);
    // Should be prev_float(5.123456790)
    let expected = 5.123456789999999;
    assert!((x_val - expected).abs() < 1e-14);
}

#[test]
fn test_precision_multiple_variables() {
    // Test x maximization
    let mut model1 = Model::with_float_precision(4);
    let x = model1.new_var_float(1.0, 10.0);
    model1.lt(x, float(3.14159265359));
    
    let solution_x = model1.maximize(x).expect("Should have solution");
    let Val::ValF(x_val) = solution_x[x] else { panic!("Expected float") };
    
    // Test y minimization with separate model
    let mut model2 = Model::with_float_precision(4);
    let y = model2.new_var_float(1.0, 10.0);
    model2.gt(y, float(2.71828182846));
    
    let solution_y = model2.minimize(y).expect("Should have solution");
    let Val::ValF(y_val) = solution_y[y] else { panic!("Expected float") };
    
    assert!(x_val < 3.14159265359);
    assert!(y_val > 2.71828182846);
    
    // Check precision-optimal values
    let expected_x = 3.1415926535899996; // prev_float(3.14159265359)
    let expected_y = 2.7182818284600005; // next_float(2.71828182846)
    
    assert!((x_val - expected_x).abs() < 1e-14);
    assert!((y_val - expected_y).abs() < 1e-14);
}

#[test]
fn test_precision_edge_cases() {
    let mut model = Model::with_float_precision(4);
    
    let x = model.new_var_float(0.0, 1.0);
    
    // Test with very small numbers near machine epsilon
    model.lt(x, float(std::f64::EPSILON * 2.0));
    
    let solution = model.maximize(x).expect("Should have solution");
    
    let Val::ValF(x_val) = solution[x] else { panic!("Expected float") };
    assert!(x_val < std::f64::EPSILON * 2.0);
    
    // Should handle very small numbers correctly
    assert!(x_val > 0.0);
}

#[test]
fn test_precision_large_numbers() {
    let mut model = Model::with_float_precision(4);
    
    let x = model.new_var_float(1e6, 1e8);
    model.lt(x, float(1234567.89012345));
    
    let solution = model.maximize(x).expect("Should have solution");
    
    let Val::ValF(x_val) = solution[x] else { panic!("Expected float") };
    assert!(x_val < 1234567.89012345);
    
    // Even with large numbers, precision should be maintained
    let expected = 1234567.8901234498; // prev_float(1234567.89012345)
    assert!((x_val - expected).abs() < 1e-9);
}

#[test]
fn test_precision_negative_numbers() {
    let mut model = Model::with_float_precision(4);
    
    let x = model.new_var_float(-10.0, 0.0);
    model.gt(x, float(-3.14159265359));
    
    let solution = model.minimize(x).expect("Should have solution");
    
    let Val::ValF(x_val) = solution[x] else { panic!("Expected float") };
    assert!(x_val > -3.14159265359);
    
    // Should be next_float(-3.14159265359)
    let expected = -3.1415926535899996;
    assert!((x_val - expected).abs() < 1e-14);
}
