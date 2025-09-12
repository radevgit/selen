use cspsolver::prelude::*;

#[test]
fn test_basic_equals() {
    let mut model = Model::default();
    
    let x = model.int(1, 10);
    let y = model.int(1, 10);
    
    model.equals(x, y);
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    let Val::ValI(y_val) = solution[y] else { panic!("Expected integer") };
    
    assert_eq!(x_val, y_val);
}

#[test]
fn test_equals_with_constant() {
    let mut model = Model::default();
    
    let x = model.int(1, 10);
    model.equals(x, int(5));
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    assert_eq!(x_val, 5);
}

#[test]
fn test_equals_with_floats() {
    let mut model = Model::default();
    
    let x = model.float(1.0, 10.0);
    let y = model.float(1.0, 10.0);
    
    model.equals(x, y);
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValF(x_val) = solution[x] else { panic!("Expected float") };
    let Val::ValF(y_val) = solution[y] else { panic!("Expected float") };
    
    assert!((x_val - y_val).abs() < 1e-6);
}

#[test]
fn test_equals_mixed_types() {
    let mut model = Model::default();
    
    let x = model.int(1, 10);
    let y = model.float(1.0, 10.0);
    
    model.equals(x, y);
    
    let solution = model.solve().expect("Should have solution");
    
    let x_val = match solution[x] {
        Val::ValI(i) => i as f64,
        Val::ValF(f) => f,
    };
    let Val::ValF(y_val) = solution[y] else { panic!("Expected float") };
    
    assert!((x_val - y_val).abs() < 1e-6);
}

#[test]
fn test_equals_chaining() {
    let mut model = Model::default();
    
    let x = model.int(1, 10);
    let y = model.int(1, 10);
    let z = model.int(1, 10);
    
    model.equals(x, y);
    model.equals(y, z);
    // This should make x = y = z
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    let Val::ValI(y_val) = solution[y] else { panic!("Expected integer") };
    let Val::ValI(z_val) = solution[z] else { panic!("Expected integer") };
    
    assert_eq!(x_val, y_val);
    assert_eq!(y_val, z_val);
}

#[test]
fn test_equals_impossible() {
    let mut model = Model::default();
    
    let x = model.int(1, 5);
    model.equals(x, int(10)); // Impossible: 10 not in [1,5]
    
    let solution = model.solve();
    assert!(solution.is_none(), "Should have no solution");
}

#[test]
fn test_equals_with_expressions() {
    let mut model = Model::default();
    
    let x = model.int(1, 5);
    let y = model.int(1, 5);
    let sum = model.add(x, y);
    
    model.equals(sum, int(8));
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    let Val::ValI(y_val) = solution[y] else { panic!("Expected integer") };
    
    assert_eq!(x_val + y_val, 8);
}

#[test]
fn test_equals_with_specific_values() {
    let mut model = Model::default();
    
    let x = model.new_var_with_values(vec![2, 4, 6, 8]);
    let y = model.new_var_with_values(vec![1, 4, 7, 8]);
    
    model.equals(x, y);
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    let Val::ValI(y_val) = solution[y] else { panic!("Expected integer") };
    
    assert_eq!(x_val, y_val);
    // Must be in intersection: {4, 8}
    assert!(vec![4, 8].contains(&x_val));
}

#[test]
fn test_equals_no_common_values() {
    let mut model = Model::default();
    
    let x = model.new_var_with_values(vec![1, 3, 5]);
    let y = model.new_var_with_values(vec![2, 4, 6]);
    
    model.equals(x, y); // No common values
    
    let solution = model.solve();
    assert!(solution.is_none(), "Should have no solution");
}

#[test]
fn test_equals_precision() {
    let mut model = Model::with_float_precision(4);
    
    let x = model.float(1.0, 2.0);
    model.equals(x, float(1.5));
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValF(x_val) = solution[x] else { panic!("Expected float") };
    assert!((x_val - 1.5).abs() < 1e-6);
}

#[test]
fn test_multiple_equals_constraints() {
    let mut model = Model::default();
    
    let vars: Vec<_> = model.int_vars(5, 1, 10).collect();
    
    // Make all variables equal
    for i in 1..vars.len() {
        model.equals(vars[0], vars[i]);
    }
    
    // Fix one to propagate to all
    model.equals(vars[0], int(7));
    
    let solution = model.solve().expect("Should have solution");
    
    for &var in &vars {
        let Val::ValI(val) = solution[var] else { panic!("Expected integer") };
        assert_eq!(val, 7);
    }
}

#[test]
fn test_equals_transitivity() {
    let mut model = Model::default();
    
    let a = model.int(1, 10);
    let b = model.int(1, 10);
    let c = model.int(1, 10);
    let d = model.int(1, 10);
    
    // Create chain: a = b, b = c, c = d
    model.equals(a, b);
    model.equals(b, c);
    model.equals(c, d);
    
    // Fix one end
    model.equals(a, int(5));
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValI(a_val) = solution[a] else { panic!("Expected integer") };
    let Val::ValI(b_val) = solution[b] else { panic!("Expected integer") };
    let Val::ValI(c_val) = solution[c] else { panic!("Expected integer") };
    let Val::ValI(d_val) = solution[d] else { panic!("Expected integer") };
    
    assert_eq!(a_val, 5);
    assert_eq!(b_val, 5);
    assert_eq!(c_val, 5);
    assert_eq!(d_val, 5);
}
