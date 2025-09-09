use cspsolver::prelude::*;

#[test]
fn test_basic_not_equals() {
    let mut model = Model::default();
    
    let x = model.new_var_int(1, 3);
    let y = model.new_var_int(1, 3);
    
    model.not_equals(x, y);
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    let Val::ValI(y_val) = solution[y] else { panic!("Expected integer") };
    
    assert_ne!(x_val, y_val);
}

#[test]
fn test_not_equals_with_constant() {
    let mut model = Model::default();
    
    let x = model.new_var_int(1, 5);
    model.not_equals(x, int(3));
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    assert_ne!(x_val, 3);
    assert!(vec![1, 2, 4, 5].contains(&x_val));
}

#[test]
fn test_not_equals_with_floats() {
    let mut model = Model::default();
    
    let x = model.new_var_float(1.0, 5.0);
    let y = model.new_var_float(1.0, 5.0);
    
    model.not_equals(x, y);
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValF(x_val) = solution[x] else { panic!("Expected float") };
    let Val::ValF(y_val) = solution[y] else { panic!("Expected float") };
    
    assert!((x_val - y_val).abs() > 1e-6);
}

#[test]
fn test_not_equals_mixed_types() {
    let mut model = Model::default();
    
    let x = model.new_var_int(1, 5);
    let y = model.new_var_float(2.5, 4.5);
    
    model.not_equals(x, y);
    
    let solution = model.solve().expect("Should have solution");
    
    let x_val = match solution[x] {
        Val::ValI(i) => i as f64,
        Val::ValF(f) => f,
    };
    let Val::ValF(y_val) = solution[y] else { panic!("Expected float") };
    
    assert!((x_val - y_val).abs() > 1e-6);
}

#[test]
fn test_not_equals_multiple_constraints() {
    let mut model = Model::default();
    
    let x = model.new_var_int(1, 5);
    let y = model.new_var_int(1, 5);
    let z = model.new_var_int(1, 5);
    
    model.not_equals(x, y);
    model.not_equals(y, z);
    model.not_equals(x, z);
    // All three must be different
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    let Val::ValI(y_val) = solution[y] else { panic!("Expected integer") };
    let Val::ValI(z_val) = solution[z] else { panic!("Expected integer") };
    
    assert_ne!(x_val, y_val);
    assert_ne!(y_val, z_val);
    assert_ne!(x_val, z_val);
}

#[test]
fn test_not_equals_impossible_single_value() {
    let mut model = Model::default();
    
    let x = model.new_var_int(5, 5); // Fixed to 5
    model.not_equals(x, int(5));     // But cannot be 5
    
    let solution = model.solve();
    assert!(solution.is_none(), "Should have no solution");
}

#[test]
fn test_not_equals_with_specific_values() {
    let mut model = Model::default();
    
    let x = model.new_var_with_values(vec![2, 4, 6]);
    let y = model.new_var_with_values(vec![4, 6, 8]);
    
    model.not_equals(x, y);
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    let Val::ValI(y_val) = solution[y] else { panic!("Expected integer") };
    
    assert_ne!(x_val, y_val);
    
    // Valid combinations: (2,4), (2,6), (2,8), (4,6), (4,8), (6,4), (6,8)
    // Invalid: (4,4), (6,6)
    if x_val == 4 {
        assert!(vec![6, 8].contains(&y_val));
    } else if x_val == 6 {
        assert!(vec![4, 8].contains(&y_val));
    } else if x_val == 2 {
        assert!(vec![4, 6, 8].contains(&y_val));
    }
}

#[test]
fn test_not_equals_narrow_domain() {
    let mut model = Model::default();
    
    let x = model.new_var_int(1, 2);
    let y = model.new_var_int(1, 2);
    
    model.not_equals(x, y);
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    let Val::ValI(y_val) = solution[y] else { panic!("Expected integer") };
    
    assert_ne!(x_val, y_val);
    // Must be (1,2) or (2,1)
    assert!(
        (x_val == 1 && y_val == 2) || (x_val == 2 && y_val == 1)
    );
}

#[test]
fn test_not_equals_with_expressions() {
    let mut model = Model::default();
    
    let x = model.new_var_int(1, 5);
    let y = model.new_var_int(1, 5);
    let sum = model.add(x, y);
    
    model.not_equals(sum, int(6));
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    let Val::ValI(y_val) = solution[y] else { panic!("Expected integer") };
    
    assert_ne!(x_val + y_val, 6);
}

#[test]
fn test_not_equals_all_different_pattern() {
    let mut model = Model::default();
    
    let vars: Vec<_> = model.new_vars_int(4, 1, 4).collect();
    
    // Manually implement all_different using not_equals
    for i in 0..vars.len() {
        for j in i+1..vars.len() {
            model.not_equals(vars[i], vars[j]);
        }
    }
    
    let solution = model.solve().expect("Should have solution");
    
    let vals: Vec<i32> = vars.iter()
        .map(|&v| match solution[v] {
            Val::ValI(i) => i,
            _ => panic!("Expected integer"),
        })
        .collect();
    
    // Should be a permutation of [1,2,3,4]
    let mut sorted_vals = vals.clone();
    sorted_vals.sort();
    assert_eq!(sorted_vals, vec![1, 2, 3, 4]);
    
    // Check all different
    for i in 0..vals.len() {
        for j in i+1..vals.len() {
            assert_ne!(vals[i], vals[j]);
        }
    }
}

#[test]
fn test_not_equals_precision() {
    let mut model = Model::with_float_precision(4);
    
    let x = model.new_var_float(1.4999, 1.5001);
    model.not_equals(x, float(1.5));
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValF(x_val) = solution[x] else { panic!("Expected float") };
    assert!((x_val - 1.5).abs() > 1e-6);
}

#[test]
fn test_not_equals_impossible_over_constraint() {
    let mut model = Model::default();
    
    // 3 variables, 3 values, all must be different - possible
    let x = model.new_var_int(1, 3);
    let y = model.new_var_int(1, 3);
    let z = model.new_var_int(1, 3);
    
    model.not_equals(x, y);
    model.not_equals(y, z);
    model.not_equals(x, z);
    
    // Now add 4th variable - impossible since only 3 values available
    let w = model.new_var_int(1, 3);
    model.not_equals(w, x);
    model.not_equals(w, y);
    model.not_equals(w, z);
    
    let solution = model.solve();
    assert!(solution.is_none(), "Should have no solution - 4 vars, 3 values");
}
