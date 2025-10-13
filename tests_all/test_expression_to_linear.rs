//! Test that expression-based linear constraints are converted to linear AST
//!
//! This tests the Phase 2 enhancement where constraints like:
//!   add(mul(x, int(5)), mul(y, int(4))).eq(int(3))
//! are automatically detected as linear and converted to LinearInt AST nodes.

use selen::prelude::*;

#[test]
fn test_expression_to_linear_simple_add() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // Post: x + y == 10 (using expressions)
    m.new(add(x, y).eq(int(10)));
    
    // Should be converted to LinearInt AST internally
    // Solve to verify it works
    match m.solve() {
        Ok(sol) => {
            let x_val = sol.get_int(x);
            let y_val = sol.get_int(y);
            assert_eq!(x_val + y_val, 10, "x + y should equal 10");
        }
        Err(e) => panic!("Should have solution: {:?}", e),
    }
}

#[test]
fn test_expression_to_linear_with_coefficients() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // Post: 2*x + 3*y == 12 (using expressions)
    m.new(add(mul(x, int(2)), mul(y, int(3))).eq(int(12)));
    
    match m.solve() {
        Ok(sol) => {
            let x_val = sol.get_int(x);
            let y_val = sol.get_int(y);
            assert_eq!(2 * x_val + 3 * y_val, 12, "2*x + 3*y should equal 12");
        }
        Err(e) => panic!("Should have solution: {:?}", e),
    }
}

#[test]
fn test_expression_to_linear_inequality() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // Post: x + y <= 8 (using expressions)
    m.new(add(x, y).le(int(8)));
    
    match m.solve() {
        Ok(sol) => {
            let x_val = sol.get_int(x);
            let y_val = sol.get_int(y);
            assert!(x_val + y_val <= 8, "x + y should be <= 8");
        }
        Err(e) => panic!("Should have solution: {:?}", e),
    }
}

#[test]
fn test_expression_to_linear_subtraction() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // Post: x - y == 3 (using expressions)
    m.new(sub(x, y).eq(int(3)));
    
    match m.solve() {
        Ok(sol) => {
            let x_val = sol.get_int(x);
            let y_val = sol.get_int(y);
            assert_eq!(x_val - y_val, 3, "x - y should equal 3");
        }
        Err(e) => panic!("Should have solution: {:?}", e),
    }
}

#[test]
fn test_expression_to_linear_complex() {
    let mut m = Model::default();
    let x = m.int(0, 20);
    let y = m.int(0, 20);
    let z = m.int(0, 20);
    
    // Post: 2*x + 3*y - z == 10 (using expressions)
    // Equivalent to: (2*x + 3*y) - z == 10
    m.new(sub(add(mul(x, int(2)), mul(y, int(3))), z).eq(int(10)));
    
    match m.solve() {
        Ok(sol) => {
            let x_val = sol.get_int(x);
            let y_val = sol.get_int(y);
            let z_val = sol.get_int(z);
            assert_eq!(2 * x_val + 3 * y_val - z_val, 10, "2*x + 3*y - z should equal 10");
        }
        Err(e) => panic!("Should have solution: {:?}", e),
    }
}

#[test]
fn test_expression_to_linear_float() {
    let mut m = Model::default();
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    
    // Post: 1.5*x + 2.5*y == 10.0 (using expressions)
    m.new(add(mul(x, float(1.5)), mul(y, float(2.5))).eq(float(10.0)));
    
    match m.solve() {
        Ok(sol) => {
            let x_val = sol.get_float(x);
            let y_val = sol.get_float(y);
            let result = 1.5 * x_val + 2.5 * y_val;
            assert!((result - 10.0).abs() < 0.01, "1.5*x + 2.5*y should equal 10.0, got {}", result);
        }
        Err(e) => panic!("Should have solution: {:?}", e),
    }
}
