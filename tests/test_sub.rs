//! Comprehensive tests for the subtraction constraint

use cspsolver::prelude::*;

#[test]
fn test_basic_subtraction() {
    let mut model = Model::default();
    
    let x = model.int(10, 10);
    let y = model.int(3, 3);
    let z = model.sub(x, y);
    
    let solution = model.solve().expect("Should find solution");
    let z_val = match solution[z] {
        Val::ValI(v) => v,
        _ => panic!("Expected integer"),
    };
    
    assert_eq!(z_val, 7, "10 - 3 should equal 7");
}

#[test]
fn test_subtraction_solve_for_minuend() {
    // Test: x - y = z where z=5, y=2, solve for x
    let mut model = Model::default();
    
    let x = model.int(1, 20);
    let y = model.int(2, 2);
    let z = model.int(5, 5);
    
    let sub_result = model.sub(x, y);
    model.equals(sub_result, z);
    
    let solution = model.solve().expect("Should find solution");
    let x_val = match solution[x] {
        Val::ValI(v) => v,
        _ => panic!("Expected integer"),
    };
    
    assert_eq!(x_val, 7, "x - 2 = 5 means x should be 7");
}

#[test]
fn test_subtraction_solve_for_subtrahend() {
    // Test: x - y = z where x=15, z=8, solve for y
    let mut model = Model::default();
    
    let x = model.int(15, 15);
    let y = model.int(1, 20);
    let z = model.int(8, 8);
    
    let sub_result = model.sub(x, y);
    model.equals(sub_result, z);
    
    let solution = model.solve().expect("Should find solution");
    let y_val = match solution[y] {
        Val::ValI(v) => v,
        _ => panic!("Expected integer"),
    };
    
    assert_eq!(y_val, 7, "15 - y = 8 means y should be 7");
}

#[test]
fn test_subtraction_with_ranges() {
    let mut model = Model::default();
    
    let x = model.int(5, 10);
    let y = model.int(2, 4);
    let z = model.sub(x, y);
    
    let solution = model.solve().expect("Should find solution");
    let x_val = match solution[x] {
        Val::ValI(v) => v,
        _ => panic!("Expected integer"),
    };
    let y_val = match solution[y] {
        Val::ValI(v) => v,
        _ => panic!("Expected integer"),
    };
    let z_val = match solution[z] {
        Val::ValI(v) => v,
        _ => panic!("Expected integer"),
    };
    
    assert_eq!(z_val, x_val - y_val, "z should equal x - y");
    assert!(z_val >= 1 && z_val <= 8, "z should be in valid range [1, 8]");
}

#[test]
fn test_subtraction_with_negative_result() {
    let mut model = Model::default();
    
    let x = model.int(3, 3);
    let y = model.int(7, 7);
    let z = model.sub(x, y);
    
    let solution = model.solve().expect("Should find solution");
    let z_val = match solution[z] {
        Val::ValI(v) => v,
        _ => panic!("Expected integer"),
    };
    
    assert_eq!(z_val, -4, "3 - 7 should equal -4");
}

#[test]
fn test_subtraction_with_floats() {
    let mut model = Model::default();
    
    let x = model.float(10.5, 10.5);
    let y = model.float(3.2, 3.2);
    let z = model.sub(x, y);
    
    let solution = model.solve().expect("Should find solution");
    let z_val = match solution[z] {
        Val::ValF(v) => v,
        _ => panic!("Expected float"),
    };
    
    assert!((z_val - 7.3).abs() < 1e-10, "10.5 - 3.2 should equal 7.3");
}

#[test]
fn test_subtraction_mixed_types() {
    let mut model = Model::default();
    
    let x = model.int(10, 10);
    let y = model.float(3.5, 3.5);
    let z = model.sub(x, y);
    
    let solution = model.solve().expect("Should find solution");
    let z_val = match solution[z] {
        Val::ValF(v) => v,
        _ => panic!("Expected float when mixing types"),
    };
    
    assert!((z_val - 6.5).abs() < 1e-10, "10 - 3.5 should equal 6.5");
}

#[test]
fn test_subtraction_chaining() {
    let mut model = Model::default();
    
    let x = model.int(20, 20);
    let y = model.int(5, 5);
    let z = model.int(3, 3);
    
    let temp = model.sub(x, y);  // temp = 20 - 5 = 15
    let result = model.sub(temp, z);  // result = 15 - 3 = 12
    
    let solution = model.solve().expect("Should find solution");
    let result_val = match solution[result] {
        Val::ValI(v) => v,
        _ => panic!("Expected integer"),
    };
    
    assert_eq!(result_val, 12, "(20 - 5) - 3 should equal 12");
}

#[test]
fn test_subtraction_bounds_propagation() {
    let mut model = Model::default();
    
    let x = model.int(10, 15);
    let y = model.int(3, 5);
    let z = model.int(6, 8);
    
    let sub_result = model.sub(x, y);
    model.equals(sub_result, z);
    
    let solution = model.solve().expect("Should find solution");
    let x_val = match solution[x] {
        Val::ValI(v) => v,
        _ => panic!("Expected integer"),
    };
    let y_val = match solution[y] {
        Val::ValI(v) => v,
        _ => panic!("Expected integer"),
    };
    let z_val = match solution[z] {
        Val::ValI(v) => v,
        _ => panic!("Expected integer"),
    };
    
    assert_eq!(z_val, x_val - y_val, "Constraint should be satisfied");
    assert!(z_val >= 6 && z_val <= 8, "z should be within bounds");
}

#[test]
fn test_impossible_subtraction() {
    let mut model = Model::default();
    
    let x = model.int(1, 5);
    let y = model.int(10, 15);
    let z = model.int(20, 25);  // Impossible: x - y cannot equal 20-25 when x is 1-5 and y is 10-15
    
    let sub_result = model.sub(x, y);
    model.equals(sub_result, z);
    
    let solution = model.solve();
    assert!(solution.is_none(), "Should have no solution for impossible constraint");
}

#[test]
fn test_subtraction_zero() {
    let mut model = Model::default();
    
    let x = model.int(7, 7);
    let y = model.int(7, 7);
    let z = model.sub(x, y);
    
    let solution = model.solve().expect("Should find solution");
    let z_val = match solution[z] {
        Val::ValI(v) => v,
        _ => panic!("Expected integer"),
    };
    
    assert_eq!(z_val, 0, "7 - 7 should equal 0");
}

#[test]
fn test_subtraction_large_numbers() {
    let mut model = Model::default();
    
    let x = model.int(1000000, 1000000);
    let y = model.int(999999, 999999);
    let z = model.sub(x, y);
    
    let solution = model.solve().expect("Should find solution");
    let z_val = match solution[z] {
        Val::ValI(v) => v,
        _ => panic!("Expected integer"),
    };
    
    assert_eq!(z_val, 1, "1000000 - 999999 should equal 1");
}
