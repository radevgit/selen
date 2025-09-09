use cspsolver::prelude::*;

#[test]
fn test_basic_addition() {
    let mut model = Model::default();
    
    let x = model.new_var_int(1, 5);
    let y = model.new_var_int(2, 8);
    let z = model.add(x, y); // z = x + y
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    let Val::ValI(y_val) = solution[y] else { panic!("Expected integer") };
    let Val::ValI(z_val) = solution[z] else { panic!("Expected integer") };
    
    assert_eq!(x_val + y_val, z_val);
}

#[test]
fn test_addition_with_constraint() {
    let mut model = Model::default();
    
    let x = model.new_var_int(1, 10);
    let y = model.new_var_int(1, 10);
    let z = model.add(x, y);
    
    // Constrain the sum to be exactly 15
    model.equals(z, int(15));
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    let Val::ValI(y_val) = solution[y] else { panic!("Expected integer") };
    let Val::ValI(z_val) = solution[z] else { panic!("Expected integer") };
    
    assert_eq!(x_val + y_val, 15);
    assert_eq!(z_val, 15);
    assert!(x_val >= 5); // Since y_val <= 10, x_val must be >= 5
    assert!(y_val >= 5); // Since x_val <= 10, y_val must be >= 5
}

#[test]
fn test_addition_with_floats() {
    let mut model = Model::default();
    
    let x = model.new_var_float(1.0, 5.0);
    let y = model.new_var_float(2.0, 8.0);
    let z = model.add(x, y);
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValF(x_val) = solution[x] else { panic!("Expected float") };
    let Val::ValF(y_val) = solution[y] else { panic!("Expected float") };
    let Val::ValF(z_val) = solution[z] else { panic!("Expected float") };
    
    assert!((x_val + y_val - z_val).abs() < 1e-6);
    assert!(z_val >= 3.0); // min possible sum
    assert!(z_val <= 13.0); // max possible sum
}

#[test]
fn test_addition_mixed_types() {
    let mut model = Model::default();
    
    let x = model.new_var_int(1, 5);
    let y = model.new_var_float(2.5, 3.5);
    let z = model.add(x, y);
    
    let solution = model.solve().expect("Should have solution");
    
    let x_val = match solution[x] {
        Val::ValI(i) => i as f64,
        Val::ValF(f) => f,
    };
    let Val::ValF(y_val) = solution[y] else { panic!("Expected float") };
    let Val::ValF(z_val) = solution[z] else { panic!("Expected float") };
    
    assert!((x_val + y_val - z_val).abs() < 1e-6);
    assert!(z_val >= 3.5); // min: 1 + 2.5
    assert!(z_val <= 8.5); // max: 5 + 3.5
}

#[test]
fn test_addition_negative_numbers() {
    let mut model = Model::default();
    
    let x = model.new_var_int(-5, -1);
    let y = model.new_var_int(-3, 2);
    let z = model.add(x, y);
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    let Val::ValI(y_val) = solution[y] else { panic!("Expected integer") };
    let Val::ValI(z_val) = solution[z] else { panic!("Expected integer") };
    
    assert_eq!(x_val + y_val, z_val);
    assert!(z_val >= -8); // min: -5 + (-3)
    assert!(z_val <= 1);  // max: -1 + 2
}

#[test]
fn test_addition_with_zero() {
    let mut model = Model::default();
    
    let x = model.new_var_int(-5, 5);
    let y = model.new_var_int(0, 0); // Fixed to 0
    let z = model.add(x, y);
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    let Val::ValI(z_val) = solution[z] else { panic!("Expected integer") };
    
    assert_eq!(x_val, z_val); // Since y = 0, x = z
}

#[test]
fn test_multiple_addition_constraints() {
    let mut model = Model::default();
    
    let a = model.new_var_int(1, 3);
    let b = model.new_var_int(1, 3);
    let c = model.new_var_int(1, 3);
    let sum_ab = model.add(a, b);
    let total = model.add(sum_ab, c);
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValI(a_val) = solution[a] else { panic!("Expected integer") };
    let Val::ValI(b_val) = solution[b] else { panic!("Expected integer") };
    let Val::ValI(c_val) = solution[c] else { panic!("Expected integer") };
    let Val::ValI(sum_ab_val) = solution[sum_ab] else { panic!("Expected integer") };
    let Val::ValI(total_val) = solution[total] else { panic!("Expected integer") };
    
    assert_eq!(a_val + b_val, sum_ab_val);
    assert_eq!(sum_ab_val + c_val, total_val);
    assert_eq!(a_val + b_val + c_val, total_val);
    assert!(total_val >= 3); // min: 1 + 1 + 1
    assert!(total_val <= 9); // max: 3 + 3 + 3
}

#[test]
fn test_addition_chaining() {
    let mut model = Model::default();
    
    let x1 = model.new_var_int(1, 2);
    let x2 = model.new_var_int(1, 2);
    let x3 = model.new_var_int(1, 2);
    let sum12 = model.add(x1, x2);
    let sum123 = model.add(sum12, x3);
    
    // Force a specific total
    model.equals(sum123, int(5));
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValI(x1_val) = solution[x1] else { panic!("Expected integer") };
    let Val::ValI(x2_val) = solution[x2] else { panic!("Expected integer") };
    let Val::ValI(x3_val) = solution[x3] else { panic!("Expected integer") };
    let Val::ValI(sum123_val) = solution[sum123] else { panic!("Expected integer") };
    
    assert_eq!(x1_val + x2_val + x3_val, 5);
    assert_eq!(sum123_val, 5);
    
    // Since each variable is in [1,2] and sum is 5, we need at least one 2
    // Possible combinations: (1,2,2), (2,1,2), (2,2,1)
    assert!(x1_val == 1 || x1_val == 2);
    assert!(x2_val == 1 || x2_val == 2);
    assert!(x3_val == 1 || x3_val == 2);
}

#[test]
fn test_large_number_addition() {
    let mut model = Model::default();
    
    let x = model.new_var_int(1000000, 2000000);
    let y = model.new_var_int(500000, 1500000);
    let z = model.add(x, y);
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    let Val::ValI(y_val) = solution[y] else { panic!("Expected integer") };
    let Val::ValI(z_val) = solution[z] else { panic!("Expected integer") };
    
    assert_eq!(x_val + y_val, z_val);
    assert!(z_val >= 1500000); // min possible sum
    assert!(z_val <= 3500000); // max possible sum
}

#[test]
fn test_addition_minimize_result() {
    let mut model = Model::default();
    
    let x = model.new_var_int(5, 10);
    let y = model.new_var_int(3, 8);
    let z = model.add(x, y);
    
    let solution = model.minimize(z).expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    let Val::ValI(y_val) = solution[y] else { panic!("Expected integer") };
    let Val::ValI(z_val) = solution[z] else { panic!("Expected integer") };
    
    assert_eq!(x_val + y_val, z_val);
    assert_eq!(z_val, 8); // minimum: 5 + 3
    assert_eq!(x_val, 5);
    assert_eq!(y_val, 3);
}

#[test]
fn test_addition_maximize_result() {
    let mut model = Model::default();
    
    let x = model.new_var_int(5, 10);
    let y = model.new_var_int(3, 8);
    let z = model.add(x, y);
    
    let solution = model.maximize(z).expect("Should have solution");
    
    let Val::ValI(x_val) = solution[x] else { panic!("Expected integer") };
    let Val::ValI(y_val) = solution[y] else { panic!("Expected integer") };
    let Val::ValI(z_val) = solution[z] else { panic!("Expected integer") };
    
    assert_eq!(x_val + y_val, z_val);
    assert_eq!(z_val, 18); // maximum: 10 + 8
    assert_eq!(x_val, 10);
    assert_eq!(y_val, 8);
}
