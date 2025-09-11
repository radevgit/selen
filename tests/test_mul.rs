//! Comprehensive tests for the multiplication constraint

use cspsolver::prelude::*;

#[test]
fn test_basic_multiplication() {
    let mut model = Model::default();
    
    let x = model.new_var_int(3, 3);
    let y = model.new_var_int(4, 4);
    let z = model.mul(x, y);
    
    let solution = model.solve().expect("Should find solution");
    let z_val = match solution[z] {
        Val::ValI(v) => v,
        Val::ValF(v) => v as i32,
    };
    
    assert_eq!(z_val, 12, "3 * 4 should equal 12");
}

#[test]
fn test_multiplication_with_ranges() {
    let mut model = Model::default();
    
    let x = model.new_var_int(2, 4);  // x ∈ [2, 4]
    let y = model.new_var_int(3, 5);  // y ∈ [3, 5]
    let z = model.mul(x, y);          // z = x * y
    
    let solution = model.solve().expect("Should find solution");
    let z_val = match solution[z] {
        Val::ValI(v) => v,
        Val::ValF(v) => v as i32,
    };
    
    // Product should be in range [2*3, 4*5] = [6, 20]
    assert!(z_val >= 6 && z_val <= 20, "Product {} should be in range [6, 20]", z_val);
}

#[test]
fn test_inverse_propagation_solve_for_x() {
    let mut model = Model::default();
    
    let x = model.new_var_int(1, 10);  // x unknown
    let y = model.new_var_int(3, 3);   // y = 3
    let z = model.new_var_int(15, 15); // z = 15
    
    // x * y = z, so x = z / y = 15 / 3 = 5
    let product = model.mul(x, y);
    model.equals(product, z);
    
    let solution = model.solve().expect("Should find solution");
    let x_val = match solution[x] {
        Val::ValI(v) => v,
        Val::ValF(v) => v as i32,
    };
    
    assert_eq!(x_val, 5, "x should be 5 when x * 3 = 15");
}

#[test]
fn test_inverse_propagation_solve_for_y() {
    let mut model = Model::default();
    
    let x = model.new_var_int(7, 7);   // x = 7
    let y = model.new_var_int(1, 10);  // y unknown
    let z = model.new_var_int(21, 21); // z = 21
    
    // x * y = z, so y = z / x = 21 / 7 = 3
    let product = model.mul(x, y);
    model.equals(product, z);
    
    let solution = model.solve().expect("Should find solution");
    let y_val = match solution[y] {
        Val::ValI(v) => v,
        Val::ValF(v) => v as i32,
    };
    
    assert_eq!(y_val, 3, "y should be 3 when 7 * y = 21");
}

#[test]
fn test_negative_multiplication() {
    let mut model = Model::default();
    
    let x = model.new_var_int(-3, -3);
    let y = model.new_var_int(4, 4);
    let z = model.mul(x, y);
    
    let solution = model.solve().expect("Should find solution");
    let z_val = match solution[z] {
        Val::ValI(v) => v,
        Val::ValF(v) => v as i32,
    };
    
    assert_eq!(z_val, -12, "-3 * 4 should equal -12");
}

#[test]
fn test_both_negative_multiplication() {
    let mut model = Model::default();
    
    let x = model.new_var_int(-5, -5);
    let y = model.new_var_int(-2, -2);
    let z = model.mul(x, y);
    
    let solution = model.solve().expect("Should find solution");
    let z_val = match solution[z] {
        Val::ValI(v) => v,
        Val::ValF(v) => v as i32,
    };
    
    assert_eq!(z_val, 10, "-5 * -2 should equal 10");
}

#[test]
fn test_multiplication_by_zero() {
    let mut model = Model::default();
    
    let x = model.new_var_int(5, 5);
    let y = model.new_var_int(0, 0);
    let z = model.mul(x, y);
    
    let solution = model.solve().expect("Should find solution");
    let z_val = match solution[z] {
        Val::ValI(v) => v,
        Val::ValF(v) => v as i32,
    };
    
    assert_eq!(z_val, 0, "5 * 0 should equal 0");
}

#[test]
fn test_multiplication_range_with_zero() {
    let mut model = Model::default();
    
    let x = model.new_var_int(-2, 3);  // x ∈ [-2, 3] (includes 0)
    let y = model.new_var_int(4, 4);   // y = 4
    let z = model.mul(x, y);           // z = x * 4
    
    let solution = model.solve().expect("Should find solution");
    let z_val = match solution[z] {
        Val::ValI(v) => v,
        Val::ValF(v) => v as i32,
    };
    
    // Product should be in range [-2*4, 3*4] = [-8, 12]
    assert!(z_val >= -8 && z_val <= 12, "Product {} should be in range [-8, 12]", z_val);
}

#[test]
fn test_multiplication_with_floats() {
    let mut model = Model::default();
    
    let x = model.new_var_float(2.5, 2.5);
    let y = model.new_var_float(4.0, 4.0);
    let z = model.mul(x, y);
    
    let solution = model.solve().expect("Should find solution");
    let z_val = match solution[z] {
        Val::ValF(v) => v,
        Val::ValI(v) => v as f64,
    };
    
    assert!((z_val - 10.0).abs() < 1e-10, "2.5 * 4.0 should equal 10.0, got {}", z_val);
}

#[test]
fn test_multiplication_mixed_types() {
    let mut model = Model::default();
    
    let x = model.new_var_int(3, 3);      // integer
    let y = model.new_var_float(2.5, 2.5); // float
    let z = model.mul(x, y);               // mixed multiplication
    
    let solution = model.solve().expect("Should find solution");
    let z_val = match solution[z] {
        Val::ValF(v) => v,
        Val::ValI(v) => v as f64,
    };
    
    assert!((z_val - 7.5).abs() < 1e-10, "3 * 2.5 should equal 7.5, got {}", z_val);
}

#[test]
fn test_complex_multiplication_constraint() {
    let mut model = Model::default();
    
    // Two unknowns with constraints: x * y = 12, x + y = 7
    let x = model.new_var_int(1, 10);
    let y = model.new_var_int(1, 10);
    
    let product = model.mul(x, y);
    model.equals(product, Val::ValI(12));  // x * y = 12
    
    let sum = model.add(x, y);
    model.equals(sum, Val::ValI(7));       // x + y = 7
    
    // Solutions: (3, 4) or (4, 3)
    let solution = model.solve().expect("Should find solution");
    let x_val = match solution[x] { Val::ValI(v) => v, Val::ValF(v) => v as i32 };
    let y_val = match solution[y] { Val::ValI(v) => v, Val::ValF(v) => v as i32 };
    
    assert_eq!(x_val * y_val, 12, "x * y should equal 12");
    assert_eq!(x_val + y_val, 7, "x + y should equal 7");
    assert!((x_val == 3 && y_val == 4) || (x_val == 4 && y_val == 3), 
            "Solution should be (3,4) or (4,3), got ({},{})", x_val, y_val);
}

#[test]
fn test_impossible_multiplication() {
    let mut model = Model::default();
    
    let x = model.new_var_int(2, 3);   // x ∈ [2, 3]
    let y = model.new_var_int(2, 3);   // y ∈ [2, 3]
    let z = model.new_var_int(20, 30); // z ∈ [20, 30]
    
    let product = model.mul(x, y);
    model.equals(product, z);   // x * y ∈ [20, 30], but max(x)*max(y) = 9
    
    // This should be impossible since max product is 3*3=9, but z requires >= 20
    let solution = model.solve();
    assert!(solution.is_none(), "Should not find solution for impossible constraint");
}

#[test]
fn test_multiplication_bounds_propagation() {
    let mut model = Model::default();
    
    let x = model.new_var_int(2, 5);   // x ∈ [2, 5]
    let y = model.new_var_int(3, 4);   // y ∈ [3, 4]
    let z = model.mul(x, y);           // z = x * y
    
    // Force z to be in a smaller range
    model.le(z, Val::ValI(10));  // z <= 10
    
    let solution = model.solve().expect("Should find solution");
    let x_val = match solution[x] { Val::ValI(v) => v, Val::ValF(v) => v as i32 };
    let y_val = match solution[y] { Val::ValI(v) => v, Val::ValF(v) => v as i32 };
    let z_val = match solution[z] { Val::ValI(v) => v, Val::ValF(v) => v as i32 };
    
    assert_eq!(x_val * y_val, z_val, "Product constraint should be satisfied");
    assert!(z_val <= 10, "z should be <= 10");
    
    // With z <= 10 constraint, possible combinations are limited:
    // 2*3=6, 2*4=8, 3*3=9 (but y max is 4), etc.
    assert!(z_val <= 10, "z should respect the upper bound constraint");
}

#[test]
fn test_large_number_multiplication() {
    let mut model = Model::default();
    
    let x = model.new_var_int(100, 100);
    let y = model.new_var_int(200, 200);
    let z = model.mul(x, y);
    
    let solution = model.solve().expect("Should find solution");
    let z_val = match solution[z] {
        Val::ValI(v) => v,
        Val::ValF(v) => v as i32,
    };
    
    assert_eq!(z_val, 20000, "100 * 200 should equal 20000");
}

#[test]
fn test_multiplication_precision_with_floats() {
    let mut model = Model::default();
    
    // Use slightly larger numbers to avoid extreme precision issues
    let x = model.new_var_float(0.5, 0.5);
    let y = model.new_var_float(0.4, 0.4);
    let z = model.mul(x, y);
    
    let solution = model.solve().expect("Should find solution");
    let z_val = match solution[z] {
        Val::ValF(v) => v,
        Val::ValI(v) => v as f64,
    };
    
    // 0.5 * 0.4 = 0.2
    assert!((z_val - 0.2).abs() < 1e-6, "0.5 * 0.4 should equal 0.2, got {}", z_val);
}

#[test]
fn test_zero_in_range_division_safety() {
    let mut model = Model::default();
    
    // Test case where one variable's range includes zero
    let x = model.new_var_int(-1, 2);   // x ∈ [-1, 2] (includes 0)
    let y = model.new_var_int(5, 5);    // y = 5
    let z = model.new_var_int(10, 10);  // z = 10
    
    let product = model.mul(x, y);
    model.equals(product, z);   // x * 5 = 10, so x = 2
    
    let solution = model.solve().expect("Should find solution despite zero in range");
    let x_val = match solution[x] { Val::ValI(v) => v, Val::ValF(v) => v as i32 };
    
    assert_eq!(x_val, 2, "x should be 2 when x * 5 = 10");
}

#[test]
fn test_multiplication_chaining() {
    let mut model = Model::default();
    
    // Test chaining: a * b = c, c * d = e
    let a = model.new_var_int(2, 2);
    let b = model.new_var_int(3, 3);
    let c = model.mul(a, b);  // c = 2 * 3 = 6
    
    let d = model.new_var_int(4, 4);
    let e = model.mul(c, d);  // e = c * 4 = 6 * 4 = 24
    
    let solution = model.solve().expect("Should find solution");
    let c_val = match solution[c] { Val::ValI(v) => v, Val::ValF(v) => v as i32 };
    let e_val = match solution[e] { Val::ValI(v) => v, Val::ValF(v) => v as i32 };
    
    assert_eq!(c_val, 6, "c should be 6");
    assert_eq!(e_val, 24, "e should be 24");
}
