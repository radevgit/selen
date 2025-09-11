use cspsolver::prelude::*;

#[test]
fn test_abs_positive() {
    let mut model = Model::default();
    
    // Test: |5| = 5
    let x = model.new_var_int(5, 5);
    let result = model.abs(x);
    
    let solution = model.solve().unwrap();
    assert_eq!(solution[result], Val::ValI(5));
}

#[test]
fn test_abs_negative() {
    let mut model = Model::default();
    
    // Test: |-7| = 7
    let x = model.new_var_int(-7, -7);
    let result = model.abs(x);
    
    let solution = model.solve().unwrap();
    assert_eq!(solution[result], Val::ValI(7));
}

#[test]
fn test_abs_range() {
    let mut model = Model::default();
    
    // Test: |x| where x in [-5, 3] should give |x| in [0, 5]
    let x = model.new_var_int(-5, 3);
    let result = model.abs(x);
    
    // Constrain the result to be exactly 4
    model.eq(result, int(4));
    
    let solution = model.solve().unwrap();
    let x_val = match solution[x] {
        Val::ValI(i) => i,
        _ => panic!("Expected integer result"),
    };
    
    // x should be either 4 or -4, but -4 is in range and 4 is in range
    assert!(x_val == 4 || x_val == -4);
    assert_eq!(solution[result], Val::ValI(4));
}

#[test]
fn test_abs_float() {
    let mut model = Model::default();
    
    // Test: |-3.5| = 3.5
    let x = model.new_var_float(-3.5, -3.5);
    let result = model.abs(x);
    
    let solution = model.solve().unwrap();
    match solution[result] {
        Val::ValF(f) => assert!((f - 3.5).abs() < 1e-6),
        _ => panic!("Expected float result"),
    }
}

#[test]
fn test_div_basic() {
    let mut model = Model::default();
    
    // Test: 12 / 3 = 4
    let x = model.new_var_int(12, 12);
    let y = model.new_var_int(3, 3);
    let result = model.div(x, y);
    
    let solution = model.solve().unwrap();
    match solution[result] {
        Val::ValF(f) => assert!((f - 4.0).abs() < 1e-6),
        Val::ValI(i) => assert_eq!(i, 4),
    }
}

#[test]
fn test_div_float() {
    let mut model = Model::default();
    
    // Test: 7.5 / 2.5 = 3.0
    let x = model.new_var_float(7.5, 7.5);
    let y = model.new_var_float(2.5, 2.5);
    let result = model.div(x, y);
    
    let solution = model.solve().unwrap();
    match solution[result] {
        Val::ValF(f) => assert!((f - 3.0).abs() < 1e-6),
        _ => panic!("Expected float result"),
    }
}

#[test]
fn test_div_with_constraint() {
    let mut model = Model::default();
    
    // Find x such that x / 4 = 3, so x should be 12
    let x = model.new_var_int(1, 20);
    let four = model.new_var_int(4, 4);
    let quotient = model.div(x, four);
    
    model.eq(quotient, float(3.0));
    
    let solution = model.solve().unwrap();
    let x_val = match solution[x] {
        Val::ValI(i) => i,
        _ => panic!("Expected integer result"),
    };
    
    assert_eq!(x_val, 12);
}

#[test]
fn test_combined_abs_div() {
    let mut model = Model::default();
    
    // Test: |x| / 2 = 3, so |x| = 6, so x = 6 or x = -6
    let x = model.new_var_int(-10, 10);
    let abs_x = model.abs(x);
    let two = model.new_var_int(2, 2);
    let result = model.div(abs_x, two);
    
    model.eq(result, float(3.0));
    
    let solution = model.solve().unwrap();
    let x_val = match solution[x] {
        Val::ValI(i) => i,
        _ => panic!("Expected integer result"),
    };
    
    assert!(x_val == 6 || x_val == -6);
    assert_eq!(solution[abs_x], Val::ValI(6));
}
