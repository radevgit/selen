use cspsolver::prelude::*;

#[test]
fn test_basic_greater_than() {
    let mut model = Model::default();
    
    let x = model.new_var_int(1, 10);
    model.greater_than(x, int(5));
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    assert!(x_val > 5);
    assert!(x_val >= 6); // Since x > 5 and x is integer, x >= 6
}

#[test]
fn test_greater_than_minimize() {
    let mut model = Model::default();
    
    let x = model.new_var_int(1, 10);
    model.greater_than(x, int(5));
    
    let solution = model.minimize(x).expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    assert_eq!(x_val, 6); // minimum value > 5
}

#[test]
fn test_greater_than_maximize() {
    let mut model = Model::default();
    
    let x = model.new_var_int(1, 10);
    model.greater_than(x, int(5));
    
    let solution = model.maximize(x).expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    assert_eq!(x_val, 10); // maximum value in range
}

#[test]
fn test_type_aware_greater_than_mixed_types() {
    // Test the type-aware greater_than method with mixed types
    let mut model = Model::default();

    let v0 = model.new_var_int(1, 10);

    // Mixed constraint: v0 > 2.5 (should result in v0 >= 3)
    model.greater_than(v0, float(2.5));
    
    let solution = model.minimize(v0).expect("Should have solution");
    let Val::ValI(x) = solution[v0] else { panic!("Expected integer value") };

    // Should find v0 = 3 since v0 > 2.5
    assert_eq!(x, 3);
}

#[test]
fn test_greater_than_with_floats() {
    let mut model = Model::default();
    
    let x = model.new_var_float(1.0, 10.0);
    model.greater_than(x, float(5.5));
    
    let solution = model.minimize(x).expect("Should have solution");
    
    let Val::ValF(x_val) = solution[x] else { panic!("Expected float") };
    assert!(x_val > 5.5);
    // Should be just slightly above 5.5 due to minimization
    assert!(x_val < 5.6);
}

#[test]
fn test_greater_than_float_vs_int() {
    let mut model = Model::default();
    
    let x = model.new_var_float(2.0, 4.0);
    model.greater_than(x, int(3));
    
    let solution = model.minimize(x).expect("Should have solution");
    
    let Val::ValF(x_val) = solution[x] else { panic!("Expected float") };
    assert!(x_val > 3.0);
    // Should be just slightly above 3.0
    assert!(x_val < 3.1);
}

#[test]
fn test_greater_than_negative_numbers() {
    let mut model = Model::default();
    
    let x = model.new_var_int(-10, 5);
    model.greater_than(x, int(-3));
    
    let solution = model.minimize(x).expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    assert_eq!(x_val, -2); // minimum value > -3
}

#[test]
fn test_greater_than_impossible() {
    let mut model = Model::default();
    
    let x = model.new_var_int(1, 5);
    model.greater_than(x, int(10)); // Impossible: no value in [1,5] > 10
    
    let solution = model.solve();
    assert!(solution.is_none(), "Should have no solution");
}

#[test]
fn test_greater_than_boundary() {
    let mut model = Model::default();
    
    let x = model.new_var_int(1, 5);
    model.greater_than(x, int(5)); // x > 5, but max is 5
    
    let solution = model.solve();
    assert!(solution.is_none(), "Should have no solution");
}

#[test]
fn test_greater_than_chaining() {
    let mut model = Model::default();
    
    let x = model.new_var_int(1, 20);
    let y = model.new_var_int(1, 20);
    let z = model.new_var_int(1, 20);
    
    model.greater_than(y, x); // y > x
    model.greater_than(z, y); // z > y, so z > y > x
    
    // Fix x to test propagation
    model.equals(x, int(5));
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    let Val::ValI(y_val) = solution[y] else { panic!("Expected integer") };
    let Val::ValI(z_val) = solution[z] else { panic!("Expected integer") };
    
    assert_eq!(x_val, 5);
    assert!(y_val > x_val); // y > 5
    assert!(z_val > y_val); // z > y
    assert!(y_val >= 6);    // Since y > 5 and integer
    assert!(z_val >= 7);    // Since z > y >= 6 and integer
}

#[test]
fn test_greater_than_with_specific_values() {
    let mut model = Model::default();
    
    let x = model.new_var_with_values(vec![1, 5, 10, 15, 20]);
    model.greater_than(x, int(7));
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    
    // Only 10, 15, 20 are > 7
    assert!(vec![10, 15, 20].contains(&x_val));
}

#[test]
fn test_greater_than_precision() {
    let mut model = Model::with_float_precision(4); // 1e-4 precision
    
    let x = model.new_var_float(1.0, 2.0);
    model.greater_than(x, float(1.5));
    
    let solution = model.minimize(x).expect("Should have solution");
    
    let Val::ValF(x_val) = solution[x] else { panic!("Expected float") };
    assert!(x_val > 1.5);
    // Should be 1.5 + step_size = 1.5 + 1e-4 = 1.5001
    assert!((x_val - 1.5001).abs() < 1e-5);
}

#[test]
fn test_type_aware_greater_than_with_minimize() {
    // Test from model.rs - type-aware greater_than with optimization
    let mut model = Model::default();
    let v1_10 = model.new_var_int(1, 10);
    model.greater_than(v1_10, float(2.5));

    let solution = model.minimize(v1_10).expect("Should have solution");
    let val = match solution[v1_10] {
        Val::ValI(x) => x,
        _ => panic!("Expected integer value"),
    };

    // Should find v0 = 3 since v0 > 2.5
    assert_eq!(val, 3);
}
