use cspsolver::prelude::*;

#[test]
fn test_basic_greater_than_or_equals() {
    let mut model = Model::default();
    
    let x = model.new_var_int(1, 10);
    model.ge(x, int(6));
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    assert!(x_val >= 6);
}

#[test]
fn test_greater_than_or_equals_minimize() {
    let mut model = Model::default();
    
    let x = model.new_var_int(1, 10);
    model.ge(x, int(6));
    
    let solution = model.minimize(x).expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    assert_eq!(x_val, 6); // minimum value >= 6
}

#[test]
fn test_greater_than_or_equals_maximize() {
    let mut model = Model::default();
    
    let x = model.new_var_int(1, 10);
    model.ge(x, int(6));
    
    let solution = model.maximize(x).expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    assert_eq!(x_val, 10); // maximum value in range
}

#[test]
fn test_greater_than_or_equals_with_floats() {
    let mut model = Model::default();
    
    let x = model.new_var_float(1.0, 10.0);
    model.ge(x, float(5.5));
    
    let solution = model.minimize(x).expect("Should have solution");
    
    let Val::ValF(x_val) = solution[x] else { panic!("Expected float") };
    assert!(x_val >= 5.5);
    assert!((x_val - 5.5).abs() < 1e-6); // Should be exactly 5.5
}

#[test]
fn test_greater_than_or_equals_mixed_types() {
    let mut model = Model::default();
    
    let x = model.new_var_int(1, 10);
    model.ge(x, float(5.5));
    
    let solution = model.minimize(x).expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    assert_eq!(x_val, 6); // smallest integer >= 5.5
}

#[test]
fn test_greater_than_or_equals_float_vs_int() {
    let mut model = Model::default();
    
    let x = model.new_var_float(2.0, 6.0);
    model.ge(x, int(5));
    
    let solution = model.minimize(x).expect("Should have solution");
    
    let Val::ValF(x_val) = solution[x] else { panic!("Expected float") };
    assert!(x_val >= 5.0);
    assert!((x_val - 5.0).abs() < 1e-6); // Should be exactly 5.0
}

#[test]
fn test_greater_than_or_equals_negative_numbers() {
    let mut model = Model::default();
    
    let x = model.new_var_int(-10, 5);
    model.ge(x, int(-3));
    
    let solution = model.minimize(x).expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    assert_eq!(x_val, -3); // minimum value >= -3
}

#[test]
fn test_greater_than_or_equals_impossible() {
    let mut model = Model::default();
    
    let x = model.new_var_int(1, 5);
    model.ge(x, int(10)); // Impossible: no value in [1,5] >= 10
    
    let solution = model.solve();
    assert!(solution.is_none(), "Should have no solution");
}

#[test]
fn test_greater_than_or_equals_boundary_exact() {
    let mut model = Model::default();
    
    let x = model.new_var_int(1, 5);
    model.ge(x, int(5)); // Only x = 5 is valid
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    assert_eq!(x_val, 5);
}

#[test]
fn test_greater_than_or_equals_chaining() {
    let mut model = Model::default();
    
    let x = model.new_var_int(1, 20);
    let y = model.new_var_int(1, 20);
    let z = model.new_var_int(1, 20);
    
    model.ge(y, x); // y >= x
    model.ge(z, y); // z >= y, so z >= y >= x
    
    // Fix x to test propagation
    model.equals(x, int(10));
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    let Val::ValI(y_val) = solution[y] else { panic!("Expected integer") };
    let Val::ValI(z_val) = solution[z] else { panic!("Expected integer") };
    
    assert_eq!(x_val, 10);
    assert!(y_val >= x_val); // y >= 10
    assert!(z_val >= y_val); // z >= y
}

#[test]
fn test_greater_than_or_equals_with_specific_values() {
    let mut model = Model::default();
    
    let x = model.new_var_with_values(vec![1, 5, 10, 15, 20]);
    model.ge(x, int(10));
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    
    // Only 10, 15, 20 are >= 10
    assert!(vec![10, 15, 20].contains(&x_val));
}

#[test]
fn test_greater_than_or_equals_with_expressions() {
    let mut model = Model::default();
    
    let x = model.new_var_int(1, 5);
    let y = model.new_var_int(1, 5);
    let sum = model.add(x, y);
    
    model.ge(sum, int(7));
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    let Val::ValI(y_val) = solution[y] else { panic!("Expected integer") };
    
    assert!(x_val + y_val >= 7);
}

#[test]
fn test_greater_than_or_equals_vs_greater_than() {
    // Compare behavior of >= vs >
    let mut model1 = Model::default();
    let mut model2 = Model::default();
    
    let x1 = model1.new_var_int(1, 10);
    let x2 = model2.new_var_int(1, 10);
    
    model1.ge(x1, int(5));
    model2.gt(x2, int(5));
    
    let sol1 = model1.minimize(x1).expect("Should have solution");
    let sol2 = model2.minimize(x2).expect("Should have solution");
    
    let Val::ValI(x1_val) = sol1[x1] else { panic!("Expected integer") };
    let Val::ValI(x2_val) = sol2[x2] else { panic!("Expected integer") };
    
    assert_eq!(x1_val, 5); // x >= 5, min is 5
    assert_eq!(x2_val, 6); // x > 5, min is 6
}

#[test]
fn test_greater_than_or_equals_precision() {
    let mut model = Model::with_float_precision(4); // 1e-4 precision
    
    let x = model.new_var_float(1.0, 2.0);
    model.ge(x, float(1.5));
    
    let solution = model.minimize(x).expect("Should have solution");
    
    let Val::ValF(x_val) = solution[x] else { panic!("Expected float") };
    assert!(x_val >= 1.5);
    assert!((x_val - 1.5).abs() < 1e-6); // Should be exactly 1.5
}

#[test]
fn test_greater_than_or_equals_ordering_non_strict() {
    let mut model = Model::default();
    
    let vars: Vec<_> = model.new_vars_int(3, 1, 3).collect();
    
    // Create non-strict ordering: vars[0] >= vars[1] >= vars[2]
    model.ge(vars[0], vars[1]);
    model.ge(vars[1], vars[2]);
    
    let solution = model.solve().expect("Should have solution");
    
    let vals: Vec<i32> = vars.iter()
        .map(|&v| match solution[v] {
            Val::ValI(i) => i,
            _ => panic!("Expected integer"),
        })
        .collect();
    
    // Should be non-increasing (can have equal values)
    for i in 0..vals.len()-1 {
        assert!(vals[i] >= vals[i+1]);
    }
}

#[test]
fn test_greater_than_or_equals_all_equal() {
    let mut model = Model::default();
    
    let x = model.new_var_int(1, 10);
    let y = model.new_var_int(1, 10);
    
    model.ge(x, y);
    model.ge(y, x);
    // This should force x = y
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    let Val::ValI(y_val) = solution[y] else { panic!("Expected integer") };
    
    assert_eq!(x_val, y_val);
}

#[test]
fn test_greater_than_or_equals_range_reduction() {
    let mut model = Model::default();
    
    let x = model.new_var_int(1, 20);
    let y = model.new_var_int(1, 20);
    
    model.ge(x, int(8));
    model.ge(y, x);
    model.le(y, int(15));
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    let Val::ValI(y_val) = solution[y] else { panic!("Expected integer") };
    
    assert!(x_val >= 8);
    assert!(y_val >= x_val);
    assert!(y_val <= 15);
    // So x ∈ [8,20], y ∈ [x,15], which means x ∈ [8,15] and y ∈ [x,15]
    assert!(x_val <= 15);
}
