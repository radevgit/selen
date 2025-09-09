use cspsolver::prelude::*;

#[test]
fn test_basic_sum() {
    let mut model = Model::default();
    
    let vars: Vec<_> = model.new_vars_int(3, 1, 5).collect();
    let total = model.sum(&vars);
    
    let solution = model.solve().expect("Should have solution");
    
    let vals: Vec<i32> = vars.iter()
        .map(|&v| match solution[v] {
            Val::ValI(i) => i,
            _ => panic!("Expected integer"),
        })
        .collect();
    
    let Val::ValI(total_val) = solution[total] else { panic!("Expected integer") };
    
    assert_eq!(vals.iter().sum::<i32>(), total_val);
    assert!(total_val >= 3); // min: 1+1+1
    assert!(total_val <= 15); // max: 5+5+5
}

#[test]
fn test_sum_with_constraint() {
    let mut model = Model::default();
    
    let vars: Vec<_> = model.new_vars_int(4, 1, 10).collect();
    let total = model.sum(&vars);
    
    // Constrain sum to be exactly 20
    model.equals(total, int(20));
    
    let solution = model.solve().expect("Should have solution");
    
    let vals: Vec<i32> = vars.iter()
        .map(|&v| match solution[v] {
            Val::ValI(i) => i,
            _ => panic!("Expected integer"),
        })
        .collect();
    
    let sum: i32 = vals.iter().sum();
    assert_eq!(sum, 20);
    
    // Check that all values are in range
    for val in vals {
        assert!(val >= 1 && val <= 10);
    }
}

#[test]
fn test_sum_minimize() {
    let mut model = Model::default();
    
    let vars: Vec<_> = model.new_vars_int(3, 2, 8).collect();
    let total = model.sum(&vars);
    
    let solution = model.minimize(total).expect("Should have solution");
    
    let vals: Vec<i32> = vars.iter()
        .map(|&v| match solution[v] {
            Val::ValI(i) => i,
            _ => panic!("Expected integer"),
        })
        .collect();
    
    let Val::ValI(total_val) = solution[total] else { panic!("Expected integer") };
    
    assert_eq!(vals.iter().sum::<i32>(), total_val);
    assert_eq!(total_val, 6); // minimum: 2+2+2
    assert!(vals.iter().all(|&x| x == 2));
}

#[test]
fn test_sum_maximize() {
    let mut model = Model::default();
    
    let vars: Vec<_> = model.new_vars_int(3, 2, 8).collect();
    let total = model.sum(&vars);
    
    let solution = model.maximize(total).expect("Should have solution");
    
    let vals: Vec<i32> = vars.iter()
        .map(|&v| match solution[v] {
            Val::ValI(i) => i,
            _ => panic!("Expected integer"),
        })
        .collect();
    
    let Val::ValI(total_val) = solution[total] else { panic!("Expected integer") };
    
    assert_eq!(vals.iter().sum::<i32>(), total_val);
    assert_eq!(total_val, 24); // maximum: 8+8+8
    assert!(vals.iter().all(|&x| x == 8));
}

#[test]
fn test_sum_with_floats() {
    let mut model = Model::default();
    
    let vars: Vec<_> = model.new_vars_float(3, 1.0, 5.0).collect();
    let total = model.sum(&vars);
    
    let solution = model.solve().expect("Should have solution");
    
    let vals: Vec<f64> = vars.iter()
        .map(|&v| match solution[v] {
            Val::ValF(f) => f,
            _ => panic!("Expected float"),
        })
        .collect();
    
    let Val::ValF(total_val) = solution[total] else { panic!("Expected float") };
    
    let computed_sum: f64 = vals.iter().sum();
    assert!((computed_sum - total_val).abs() < 1e-6);
    assert!(total_val >= 3.0); // min: 1.0+1.0+1.0
    assert!(total_val <= 15.0); // max: 5.0+5.0+5.0
}

#[test]
fn test_sum_mixed_types() {
    let mut model = Model::default();
    
    let int_vars: Vec<_> = model.new_vars_int(2, 1, 5).collect();
    let float_vars: Vec<_> = model.new_vars_float(2, 1.5, 3.5).collect();
    
    let mut all_vars = Vec::new();
    all_vars.extend(int_vars);
    all_vars.extend(float_vars);
    
    let total = model.sum(&all_vars);
    
    let solution = model.solve().expect("Should have solution");
    
    let mut computed_sum = 0.0;
    for &var in &all_vars {
        match solution[var] {
            Val::ValI(i) => computed_sum += i as f64,
            Val::ValF(f) => computed_sum += f,
        }
    }
    
    let Val::ValF(total_val) = solution[total] else { panic!("Expected float") };
    
    assert!((computed_sum - total_val).abs() < 1e-6);
    assert!(total_val >= 5.0); // min: 1+1+1.5+1.5
    assert!(total_val <= 17.0); // max: 5+5+3.5+3.5
}

#[test]
fn test_sum_with_negative_numbers() {
    let mut model = Model::default();
    
    let vars: Vec<_> = model.new_vars_int(3, -5, 5).collect();
    let total = model.sum(&vars);
    
    let solution = model.solve().expect("Should have solution");
    
    let vals: Vec<i32> = vars.iter()
        .map(|&v| match solution[v] {
            Val::ValI(i) => i,
            _ => panic!("Expected integer"),
        })
        .collect();
    
    let Val::ValI(total_val) = solution[total] else { panic!("Expected integer") };
    
    assert_eq!(vals.iter().sum::<i32>(), total_val);
    assert!(total_val >= -15); // min: -5+(-5)+(-5)
    assert!(total_val <= 15);  // max: 5+5+5
}

#[test]
fn test_sum_single_variable() {
    let mut model = Model::default();
    
    let var = model.new_var_int(3, 7);
    let total = model.sum(&[var]);
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValI(var_val) = solution[var] else { panic!("Expected integer") };
    let Val::ValI(total_val) = solution[total] else { panic!("Expected integer") };
    
    assert_eq!(var_val, total_val);
}

#[test]
fn test_sum_empty_vector() {
    let mut model = Model::default();
    
    let empty_vars: Vec<VarId> = vec![];
    let total = model.sum(&empty_vars);
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValI(total_val) = solution[total] else { panic!("Expected integer") };
    
    assert_eq!(total_val, 0); // sum of empty set is 0
}

#[test]
fn test_sum_large_number_of_variables() {
    let mut model = Model::default();
    
    let vars: Vec<_> = model.new_vars_int(10, 1, 3).collect();
    let total = model.sum(&vars);
    
    // Constrain sum to be exactly 25
    model.equals(total, int(25));
    
    let solution = model.solve().expect("Should have solution");
    
    let vals: Vec<i32> = vars.iter()
        .map(|&v| match solution[v] {
            Val::ValI(i) => i,
            _ => panic!("Expected integer"),
        })
        .collect();
    
    assert_eq!(vals.iter().sum::<i32>(), 25);
    
    // With 10 variables each in [1,3], sum of 25 requires specific distribution
    // Min sum: 10, Max sum: 30, so 25 is achievable
    let ones = vals.iter().filter(|&&x| x == 1).count();
    let twos = vals.iter().filter(|&&x| x == 2).count();
    let threes = vals.iter().filter(|&&x| x == 3).count();
    
    assert_eq!(ones + twos + threes, 10);
    assert_eq!(ones + 2*twos + 3*threes, 25);
}

#[test]
fn test_sum_with_specific_values() {
    let mut model = Model::default();
    
    let var1 = model.new_var_with_values(vec![2, 4, 6]);
    let var2 = model.new_var_with_values(vec![1, 3, 5]);
    let var3 = model.new_var_with_values(vec![2, 8]);
    
    let total = model.sum(&[var1, var2, var3]);
    
    // Constrain sum to be exactly 11 (achievable: 2 + 1 + 8 = 11)
    model.equals(total, int(11));
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValI(val1) = solution[var1] else { panic!("Expected integer") };
    let Val::ValI(val2) = solution[var2] else { panic!("Expected integer") };
    let Val::ValI(val3) = solution[var3] else { panic!("Expected integer") };
    
    assert_eq!(val1 + val2 + val3, 11);
    assert!(vec![2, 4, 6].contains(&val1));
    assert!(vec![1, 3, 5].contains(&val2));
    assert!(vec![2, 8].contains(&val3));
}

#[test]
fn test_sum_impossible_constraint() {
    let mut model = Model::default();
    
    let vars: Vec<_> = model.new_vars_int(3, 5, 10).collect();
    let total = model.sum(&vars);
    
    // Impossible: sum must be 5 but minimum is 15 (5+5+5)
    model.equals(total, int(5));
    
    let solution = model.solve();
    assert!(solution.is_none(), "Should have no solution");
}

#[test]
fn test_sum_iter_method() {
    let mut model = Model::default();
    
    let vars: Vec<_> = model.new_vars_int(4, 1, 6).collect();
    
    // Test sum_iter with iterator
    let total = model.sum_iter(vars.iter().copied());
    
    let solution = model.solve().expect("Should have solution");
    
    let vals: Vec<i32> = vars.iter()
        .map(|&v| match solution[v] {
            Val::ValI(i) => i,
            _ => panic!("Expected integer"),
        })
        .collect();
    
    let Val::ValI(total_val) = solution[total] else { panic!("Expected integer") };
    
    assert_eq!(vals.iter().sum::<i32>(), total_val);
    assert!(total_val >= 4);  // min: 1+1+1+1
    assert!(total_val <= 24); // max: 6+6+6+6
}
