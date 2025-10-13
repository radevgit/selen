//! Unit tests for array constraint API
//! This file improves coverage for src/constraints/api/array.rs (50% -> higher)

use selen::prelude::*;

#[test]
fn test_array_int_minimum() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    let y = m.int(5, 15);
    let z = m.int(3, 8);
    
    let min_var = m.array_int_minimum(&[x, y, z]).expect("Should succeed");
    
    let solution = m.solve().expect("Should find solution");
    let min_val = solution[min_var].as_int().unwrap();
    let x_val = solution[x].as_int().unwrap();
    let y_val = solution[y].as_int().unwrap();
    let z_val = solution[z].as_int().unwrap();
    
    assert!(min_val <= x_val);
    assert!(min_val <= y_val);
    assert!(min_val <= z_val);
}

#[test]
fn test_array_int_minimum_empty() {
    let mut m = Model::default();
    
    let result = m.array_int_minimum(&[]);
    assert!(result.is_err());
}

#[test]
fn test_array_int_minimum_single() {
    let mut m = Model::default();
    let x = m.int(5, 5);
    
    let min_var = m.array_int_minimum(&[x]).expect("Should succeed");
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[min_var].as_int().unwrap(), 5);
}

#[test]
fn test_array_int_maximum() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    let y = m.int(5, 15);
    let z = m.int(3, 8);
    
    let max_var = m.array_int_maximum(&[x, y, z]).expect("Should succeed");
    
    let solution = m.solve().expect("Should find solution");
    let max_val = solution[max_var].as_int().unwrap();
    let x_val = solution[x].as_int().unwrap();
    let y_val = solution[y].as_int().unwrap();
    let z_val = solution[z].as_int().unwrap();
    
    assert!(max_val >= x_val);
    assert!(max_val >= y_val);
    assert!(max_val >= z_val);
}

#[test]
fn test_array_int_maximum_empty() {
    let mut m = Model::default();
    
    let result = m.array_int_maximum(&[]);
    assert!(result.is_err());
}

#[test]
fn test_array_int_maximum_single() {
    let mut m = Model::default();
    let x = m.int(7, 7);
    
    let max_var = m.array_int_maximum(&[x]).expect("Should succeed");
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[max_var].as_int().unwrap(), 7);
}

#[test]
fn test_array_int_element_basic() {
    let mut m = Model::default();
    
    let arr = vec![
        m.int(10, 10),
        m.int(20, 20),
        m.int(30, 30),
    ];
    
    let index = m.int(0, 2);
    let result = m.int(0, 50);
    
    m.array_int_element(index, &arr, result);
    
    let solution = m.solve().expect("Should find solution");
    let idx = solution[index].as_int().unwrap() as usize;
    let res = solution[result].as_int().unwrap();
    let expected = solution[arr[idx]].as_int().unwrap();
    
    assert_eq!(res, expected);
}

#[test]
fn test_array_int_element_constrained_index() {
    let mut m = Model::default();
    
    let arr = vec![
        m.int(10, 10),
        m.int(20, 20),
        m.int(30, 30),
    ];
    
    let index = m.int(1, 1);  // Force index = 1
    let result = m.int(0, 50);
    
    m.array_int_element(index, &arr, result);
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[index].as_int().unwrap(), 1);
    assert_eq!(solution[result].as_int().unwrap(), 20);
}

#[test]
fn test_array_int_element_constrained_result() {
    let mut m = Model::default();
    
    let arr = vec![
        m.int(10, 10),
        m.int(20, 20),
        m.int(30, 30),
    ];
    
    let index = m.int(0, 2);
    let result = m.int(30, 30);  // Force result = 30
    
    m.array_int_element(index, &arr, result);
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[index].as_int().unwrap(), 2);  // Should select index 2
    assert_eq!(solution[result].as_int().unwrap(), 30);
}

#[test]
fn test_array_float_minimum() {
    let mut m = Model::default();
    let x = m.float(1.0, 10.0);
    let y = m.float(5.0, 15.0);
    let z = m.float(3.0, 8.0);
    
    let min_var = m.array_float_minimum(&[x, y, z]).expect("Should succeed");
    
    let solution = m.solve().expect("Should find solution");
    let min_val = solution[min_var].as_float().unwrap();
    let x_val = solution[x].as_float().unwrap();
    let y_val = solution[y].as_float().unwrap();
    let z_val = solution[z].as_float().unwrap();
    
    assert!(min_val <= x_val + 1e-5);
    assert!(min_val <= y_val + 1e-5);
    assert!(min_val <= z_val + 1e-5);
}

#[test]
fn test_array_float_minimum_empty() {
    let mut m = Model::default();
    
    let result = m.array_float_minimum(&[]);
    assert!(result.is_err());
}

#[test]
fn test_array_float_maximum() {
    let mut m = Model::default();
    let x = m.float(1.0, 10.0);
    let y = m.float(5.0, 15.0);
    let z = m.float(3.0, 8.0);
    
    let max_var = m.array_float_maximum(&[x, y, z]).expect("Should succeed");
    
    let solution = m.solve().expect("Should find solution");
    let max_val = solution[max_var].as_float().unwrap();
    let x_val = solution[x].as_float().unwrap();
    let y_val = solution[y].as_float().unwrap();
    let z_val = solution[z].as_float().unwrap();
    
    assert!(max_val >= x_val - 1e-5);
    assert!(max_val >= y_val - 1e-5);
    assert!(max_val >= z_val - 1e-5);
}

#[test]
fn test_array_float_maximum_empty() {
    let mut m = Model::default();
    
    let result = m.array_float_maximum(&[]);
    assert!(result.is_err());
}

#[test]
fn test_array_float_element() {
    let mut m = Model::default();
    
    let arr = vec![
        m.float(1.5, 1.5),
        m.float(2.5, 2.5),
        m.float(3.5, 3.5),
    ];
    
    let index = m.int(0, 2);
    let result = m.float(0.0, 10.0);
    
    m.array_float_element(index, &arr, result);
    
    let solution = m.solve().expect("Should find solution");
    let idx = solution[index].as_int().unwrap() as usize;
    let res = solution[result].as_float().unwrap();
    let expected = solution[arr[idx]].as_float().unwrap();
    
    assert!((res - expected).abs() < 1e-5);
}
