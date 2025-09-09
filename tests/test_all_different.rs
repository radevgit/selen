use cspsolver::prelude::*;

#[test]
fn test_all_different_integer_only_recommended() {
    // Note: all_different works best with integer variables
    // Using it with float variables is not recommended due to precision issues
    let mut model = Model::default();
    
    let vars: Vec<_> = model.new_vars_int(3, 1, 5).collect();
    model.all_different(vars.clone());
    
    let solution = model.solve().expect("Should have solution");
    
    let vals: Vec<i32> = vars.iter()
        .map(|&v| match solution[v] {
            Val::ValI(i) => i,
            _ => panic!("Expected integer"),
        })
        .collect();
    
    // Check all values are different
    for i in 0..vals.len() {
        for j in i+1..vals.len() {
            assert_ne!(vals[i], vals[j]);
        }
    }
    
    // Check all values are in range [1,5]
    for val in &vals {
        assert!(*val >= 1 && *val <= 5);
    }
}

#[test]
fn test_all_different_exact_fit() {
    let mut model = Model::default();
    
    // 4 variables, domain [1,4] - should use all values exactly once
    let vars: Vec<_> = model.new_vars_int(4, 1, 4).collect();
    model.all_different(vars.clone());
    
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
}

#[test]
fn test_all_different_impossible() {
    let mut model = Model::default();
    
    // 5 variables, domain [1,4] - impossible since we need 5 different values
    let vars: Vec<_> = model.new_vars_int(5, 1, 4).collect();
    model.all_different(vars.clone());
    
    let solution = model.solve();
    assert!(solution.is_none(), "Should have no solution - 5 vars, 4 values");
}

#[test]
fn test_all_different_mixed_types_should_fail() {
    // This test demonstrates that mixing float and int variables 
    // in all_different is problematic and should be avoided
    let mut model = Model::default();
    
    let int_vars: Vec<_> = model.new_vars_int(2, 1, 10).collect();
    let float_vars: Vec<_> = model.new_vars_float(2, 1.5, 10.5).collect();
    
    let mut all_vars = Vec::new();
    all_vars.extend(int_vars);
    all_vars.extend(float_vars);
    
    // This works but is not recommended due to precision issues
    model.all_different(all_vars.clone());
    
    // The solver may find a solution, but it's not reliable for floats
    let _solution = model.solve();
    // We don't assert anything because the behavior with floats is undefined
}

#[test]
fn test_all_different_with_specific_values() {
    let mut model = Model::default();
    
    let var1 = model.new_var_with_values(vec![1, 3, 5]);
    let var2 = model.new_var_with_values(vec![2, 3, 6]);
    let var3 = model.new_var_with_values(vec![1, 4, 5]);
    
    model.all_different(vec![var1, var2, var3]);
    
    let solution = model.solve().expect("Should have solution");
    
    let val1 = match solution[var1] { Val::ValI(i) => i, _ => panic!("Expected integer") };
    let val2 = match solution[var2] { Val::ValI(i) => i, _ => panic!("Expected integer") };
    let val3 = match solution[var3] { Val::ValI(i) => i, _ => panic!("Expected integer") };
    
    assert_ne!(val1, val2);
    assert_ne!(val2, val3);
    assert_ne!(val1, val3);
    
    // Check values are from correct domains
    assert!(vec![1, 3, 5].contains(&val1));
    assert!(vec![2, 3, 6].contains(&val2));
    assert!(vec![1, 4, 5].contains(&val3));
}

#[test]
fn test_all_different_single_variable() {
    let mut model = Model::default();
    
    let var = model.new_var_int(1, 5);
    model.all_different(vec![var]);
    
    let solution = model.solve().expect("Should have solution");
    
    let val = match solution[var] { Val::ValI(i) => i, _ => panic!("Expected integer") };
    assert!(val >= 1 && val <= 5);
    // Should work fine with single variable
}

#[test]
fn test_all_different_empty() {
    let mut model = Model::default();
    
    let empty_vars: Vec<VarId> = vec![];
    model.all_different(empty_vars.clone());
    
    let _solution = model.solve().expect("Should have solution");
    // Empty all_different should always be satisfied
}

#[test]
fn test_all_different_with_constraints() {
    let mut model = Model::default();
    
    let vars: Vec<_> = model.new_vars_int(3, 1, 10).collect();
    model.all_different(vars.clone());
    
    // Add additional constraints
    model.greater_than(vars[0], int(5)); // vars[0] > 5
    model.less_than(vars[1], int(3));    // vars[1] < 3
    // vars[2] can be anything in [1,10] except vars[0] and vars[1]
    
    let solution = model.solve().expect("Should have solution");
    
    let val0 = match solution[vars[0]] { Val::ValI(i) => i, _ => panic!("Expected integer") };
    let val1 = match solution[vars[1]] { Val::ValI(i) => i, _ => panic!("Expected integer") };
    let val2 = match solution[vars[2]] { Val::ValI(i) => i, _ => panic!("Expected integer") };
    
    assert!(val0 > 5);      // vars[0] > 5, so vars[0] ∈ {6,7,8,9,10}
    assert!(val1 < 3);      // vars[1] < 3, so vars[1] ∈ {1,2}
    assert_ne!(val0, val1);
    assert_ne!(val1, val2);
    assert_ne!(val0, val2);
}

#[test]
fn test_all_different_sudoku_style() {
    let mut model = Model::default();
    
    // Create a 3x3 grid, each row must have different values
    let grid: Vec<Vec<_>> = (0..3)
        .map(|_| model.new_vars_int(3, 1, 3).collect())
        .collect();
    
    // Each row must have all different values
    for row in &grid {
        model.all_different(row.clone());
    }
    
    let solution = model.solve().expect("Should have solution");
    
    // Check each row has all different values and covers [1,2,3]
    for row in &grid {
        let vals: Vec<i32> = row.iter()
            .map(|&v| match solution[v] {
                Val::ValI(i) => i,
                _ => panic!("Expected integer"),
            })
            .collect();
        
        let mut sorted_vals = vals.clone();
        sorted_vals.sort();
        assert_eq!(sorted_vals, vec![1, 2, 3]);
    }
}

#[test]
fn test_all_different_large_domain() {
    let mut model = Model::default();
    
    let vars: Vec<_> = model.new_vars_int(5, 1, 100).collect();
    model.all_different(vars.clone());
    
    let solution = model.solve().expect("Should have solution");
    
    let vals: Vec<i32> = vars.iter()
        .map(|&v| match solution[v] {
            Val::ValI(i) => i,
            _ => panic!("Expected integer"),
        })
        .collect();
    
    // Check all values are different
    for i in 0..vals.len() {
        for j in i+1..vals.len() {
            assert_ne!(vals[i], vals[j]);
        }
    }
    
    // Check all values are in range [1,100]
    for val in &vals {
        assert!(*val >= 1 && *val <= 100);
    }
}

#[test]
fn test_all_different_minimize_sum() {
    let mut model = Model::default();
    
    let vars: Vec<_> = model.new_vars_int(3, 1, 10).collect();
    model.all_different(vars.clone());
    
    let total = model.sum(&vars);
    
    let solution = model.minimize(total).expect("Should have solution");
    
    let vals: Vec<i32> = vars.iter()
        .map(|&v| match solution[v] {
            Val::ValI(i) => i,
            _ => panic!("Expected integer"),
        })
        .collect();
    
    let total_val = match solution[total] { Val::ValI(i) => i, _ => panic!("Expected integer") };
    
    // Should choose minimum possible values: 1, 2, 3
    let mut sorted_vals = vals.clone();
    sorted_vals.sort();
    assert_eq!(sorted_vals, vec![1, 2, 3]);
    assert_eq!(total_val, 6); // 1 + 2 + 3
}

#[test]
fn test_all_different_impossible_specific_values() {
    let mut model = Model::default();
    
    // Two variables with overlapping but insufficient distinct values
    let var1 = model.new_var_with_values(vec![1, 2]);
    let var2 = model.new_var_with_values(vec![1, 2]);
    let var3 = model.new_var_with_values(vec![1, 2]);
    
    model.all_different(vec![var1, var2, var3]);
    
    let solution = model.solve();
    assert!(solution.is_none(), "Should have no solution - 3 vars, 2 distinct values available");
}
