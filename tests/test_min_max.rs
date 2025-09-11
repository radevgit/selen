use cspsolver::prelude::*;

#[test]
fn test_min_basic() {
    let mut model = Model::default();
    
    // Test: min(5, 8, 3) = 3
    let x = model.new_var_int(5, 5);
    let y = model.new_var_int(8, 8);
    let z = model.new_var_int(3, 3);
    let min_result = model.min(&[x, y, z]);
    
    let solution = model.solve().unwrap();
    assert_eq!(solution[min_result], Val::ValI(3));
}

#[test]
fn test_max_basic() {
    let mut model = Model::default();
    
    // Test: max(5, 8, 3) = 8
    let x = model.new_var_int(5, 5);
    let y = model.new_var_int(8, 8);
    let z = model.new_var_int(3, 3);
    let max_result = model.max(&[x, y, z]);
    
    let solution = model.solve().unwrap();
    assert_eq!(solution[max_result], Val::ValI(8));
}

#[test]
fn test_min_with_ranges() {
    let mut model = Model::default();
    
    // Test: min(x, y) where x ∈ [1, 5], y ∈ [3, 7]
    // min should be in [1, 5] (can't be larger than 5 since x can be that small)
    let x = model.new_var_int(1, 5);
    let y = model.new_var_int(3, 7);
    let min_result = model.min(&[x, y]);
    
    // Constrain minimum to be exactly 2
    model.eq(min_result, int(2));
    
    let solution = model.solve().unwrap();
    assert_eq!(solution[min_result], Val::ValI(2));
    
    // x must be 2 (since it's the only one that can achieve the minimum)
    assert_eq!(solution[x], Val::ValI(2));
    // y can be anything >= 2, so it should be at least 3 due to its domain
    let y_val = match solution[y] {
        Val::ValI(i) => i,
        _ => panic!("Expected integer"),
    };
    assert!(y_val >= 3);
}

#[test]
fn test_max_with_ranges() {
    let mut model = Model::default();
    
    // Test: max(x, y) where x ∈ [1, 5], y ∈ [3, 7]
    // max should be in [3, 7] (can't be smaller than 3 since y is at least that)
    let x = model.new_var_int(1, 5);
    let y = model.new_var_int(3, 7);
    let max_result = model.max(&[x, y]);
    
    // Constrain maximum to be exactly 6
    model.eq(max_result, int(6));
    
    let solution = model.solve().unwrap();
    assert_eq!(solution[max_result], Val::ValI(6));
    
    // y must be 6 (since it's the only one that can achieve the maximum)
    assert_eq!(solution[y], Val::ValI(6));
    // x can be anything <= 6, so it should be at most 5 due to its domain
    let x_val = match solution[x] {
        Val::ValI(i) => i,
        _ => panic!("Expected integer"),
    };
    assert!(x_val <= 5);
}

#[test]
fn test_min_float() {
    let mut model = Model::default();
    
    // Test: min(2.5, 1.7, 3.2) = 1.7
    let x = model.new_var_float(2.5, 2.5);
    let y = model.new_var_float(1.7, 1.7);
    let z = model.new_var_float(3.2, 3.2);
    let min_result = model.min(&[x, y, z]);
    
    let solution = model.solve().unwrap();
    match solution[min_result] {
        Val::ValF(f) => assert!((f - 1.7).abs() < 1e-6),
        _ => panic!("Expected float result"),
    }
}

#[test]
fn test_max_float() {
    let mut model = Model::default();
    
    // Test: max(2.5, 1.7, 3.2) = 3.2
    let x = model.new_var_float(2.5, 2.5);
    let y = model.new_var_float(1.7, 1.7);
    let z = model.new_var_float(3.2, 3.2);
    let max_result = model.max(&[x, y, z]);
    
    let solution = model.solve().unwrap();
    match solution[max_result] {
        Val::ValF(f) => assert!((f - 3.2).abs() < 1e-6),
        _ => panic!("Expected float result"),
    }
}

#[test]
fn test_min_propagation() {
    let mut model = Model::default();
    
    // Test propagation: if min([x, y, z]) = 5 and x ∈ [1, 4], then x cannot be the minimum
    let x = model.new_var_int(1, 4);
    let y = model.new_var_int(3, 10);
    let z = model.new_var_int(4, 8);
    let min_result = model.min(&[x, y, z]);
    
    // This should be unsatisfiable since no variable can be 5
    model.eq(min_result, int(5));
    
    let solution = model.solve();
    assert!(solution.is_none(), "This constraint should be unsatisfiable");
}

#[test]
fn test_max_propagation() {
    let mut model = Model::default();
    
    // Test propagation: if max([x, y, z]) = 2 and z ∈ [3, 8], then z cannot be <= 2
    let x = model.new_var_int(1, 4);
    let y = model.new_var_int(1, 3);
    let z = model.new_var_int(3, 8);
    let max_result = model.max(&[x, y, z]);
    
    // This should be unsatisfiable since z must be <= 2 but its domain is [3, 8]
    model.eq(max_result, int(2));
    
    let solution = model.solve();
    assert!(solution.is_none(), "This constraint should be unsatisfiable");
}

#[test]
fn test_single_variable_min_max() {
    let mut model = Model::default();
    
    // Test: min/max of a single variable should equal that variable
    let x = model.new_var_int(5, 10);
    let min_result = model.min(&[x]);
    let max_result = model.max(&[x]);
    
    let solution = model.solve().unwrap();
    
    // Both min and max should equal x
    assert_eq!(solution[x], solution[min_result]);
    assert_eq!(solution[x], solution[max_result]);
}

#[test]
fn test_mixed_int_float_min() {
    let mut model = Model::default();
    
    // Test: min with mixed integer and float variables
    let x = model.new_var_int(2, 6);
    let y = model.new_var_float(1.5, 4.5);
    let min_result = model.min(&[x, y]);
    
    // Set minimum to 3.0
    model.eq(min_result, float(3.0));
    
    let solution = model.solve().unwrap();
    match solution[min_result] {
        Val::ValF(f) => assert!((f - 3.0).abs() < 1e-6),
        _ => panic!("Expected float result"),
    }
    
    // Either x or y should be 3.0
    let x_val = match solution[x] {
        Val::ValI(i) => i as f64,
        Val::ValF(f) => f,
    };
    let y_val = match solution[y] {
        Val::ValI(i) => i as f64,
        Val::ValF(f) => f,
    };
    
    assert!(x_val >= 3.0 && y_val >= 3.0);
    assert!((x_val - 3.0).abs() < 1e-6 || (y_val - 3.0).abs() < 1e-6);
}

#[test]
fn test_large_vector_min_max() {
    let mut model = Model::default();
    
    // Test with larger number of variables
    let vars: Vec<VarId> = (0..10).map(|i| model.new_var_int(i, i + 10)).collect();
    let min_result = model.min(&vars);
    let max_result = model.max(&vars);
    
    let solution = model.solve().unwrap();
    
    // Minimum should be 0 (from first variable)
    assert_eq!(solution[min_result], Val::ValI(0));
    
    // Maximum should be 19 (from last variable: 9 + 10)
    assert_eq!(solution[max_result], Val::ValI(19));
}
