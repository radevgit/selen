//! Tests that previously timed out with large domains (60s+) but should now work with LP solver
//! These tests motivated the LP solver implementation.

use selen::prelude::*;

#[test]
fn test_optimization_with_large_domains() {
    // Test optimization with large domain handling
    // Previously: timed out (60s+)
    // With LP: should solve quickly
    let mut model = Model::default();
    let x = model.float(-1e6, 1e6);
    let y = model.float(-1e6, 1e6);
    
    let sum_var = model.float(-2e6, 2e6);
    model.new(x.add(y).eq(sum_var));
    model.new(sum_var.le(0.0));
    
    let result = model.maximize(x);
    assert!(result.is_ok(), "Large domain optimization should work");
    
    if let Ok(solution) = result {
        let x_val = solution.get_float(x);
        let y_val = solution.get_float(y);
        let sum_val = solution.get_float(sum_var);
        
        println!("Solution: x={}, y={}, sum={}", x_val, y_val, sum_val);
        
        assert!(sum_val <= 1e-10, "Constraint satisfied in large domain");
        
        // x should be maximized (large positive), y should be negative to keep sum <= 0
        assert!(x_val > 0.0, "x should be positive to maximize");
    }
}

#[test]
fn test_large_domain_optimization_linear() {
    // Test optimization with large domains (the optimization bug we fixed)
    // Previously: timed out (60s+)
    // With LP: should solve instantly
    let mut m = Model::default();
    
    let x = m.float(0.0, 1e6);
    let y = m.float(0.0, 1e6);
    
    // x + y <= 8000
    let sum = m.add(x, y);
    m.new(sum.le(8000.0));
    
    // x >= 2000
    m.new(x.ge(2000.0));
    
    let solution = m.maximize(x).expect("Should maximize with large domain");
    
    let x_val = solution.get_float(x);
    let y_val = solution.get_float(y);
    
    println!("Solution: x={}, y={}", x_val, y_val);
    
    // Check constraints
    assert!(x_val + y_val <= 8000.1, "Constraint violated: {} + {} > 8000", x_val, y_val);
    assert!(x_val >= 1999.9, "Lower bound violated: {} < 2000", x_val);
    
    // Optimal should be x = 8000 (when y = 0)
    assert!(x_val >= 7999.0, "Not optimal: x = {} < 8000", x_val);
}

#[test]
#[ignore] // TODO: LP doesn't handle multiplication (non-linear)
fn test_optimization_with_derived_variables() {
    // This test uses multiplication (y = 2*x) which is non-linear
    // LP solver can only handle linear constraints
    // CSP solver handles this but times out on large domains without LP help
    let mut m = Model::default();
    
    let x = m.float(0.0, 100.0);
    
    // Create derived variable: y = 2*x
    let two = m.float(2.0, 2.0); // Constant
    let y = m.mul(x, two);
    
    // Constrain y <= 50
    m.new(y.le(50.0));
    
    let solution = m.maximize(x).expect("Should maximize with derived constraint");
    
    let x_val = solution.get_float(x);
    let y_val = solution.get_float(y);
    
    println!("Solution: x={}, y={} (should be y=2*x)", x_val, y_val);
    
    // y should be <= 50
    assert!(y_val <= 50.1, "Derived constraint violated: y = {} > 50", y_val);
    
    // y should be 2*x
    assert!((y_val - 2.0 * x_val).abs() < 0.1, 
        "Derived variable wrong: y = {} ≠ 2*{} = {}", y_val, x_val, 2.0 * x_val);
    
    // Optimal should be x = 25 (making y = 50)
    assert!(x_val >= 24.9 && x_val <= 25.1, 
        "Not optimal: x = {} should be ~25", x_val);
}

#[test]
fn test_unbounded_optimization_with_constraints() {
    // Optimize unbounded variable with actual constraints
    // Previously: timed out (60s+)
    // With LP: should solve quickly
    let mut m = Model::default();
    
    let x_unbounded = m.float(f64::NEG_INFINITY, f64::INFINITY);
    let y_bounded = m.float(0.0, 100.0);
    
    // x + y <= 150
    let sum = m.add(x_unbounded, y_bounded);
    m.new(sum.le(150.0));
    
    // x >= 10
    m.new(x_unbounded.ge(10.0));
    
    let solution = m.maximize(x_unbounded).expect("Should maximize unbounded with constraints");
    
    let x_val = solution.get_float(x_unbounded);
    let y_val = solution.get_float(y_bounded);
    
    println!("Solution: x={}, y={}", x_val, y_val);
    
    // Check constraints
    assert!(x_val + y_val <= 150.1, "Constraint violated: {} + {} > 150", x_val, y_val);
    assert!(x_val >= 9.9, "Lower bound violated: {} < 10", x_val);
    
    // Optimal should be x = 150 (when y = 0)
    assert!(x_val >= 149.0, "Not optimal: x = {} < 150", x_val);
}

#[test]
fn test_large_domain_float_linear_equality() {
    // Test with large domains and linear equality
    // Previously: worked but slow
    // With LP: should be instant
    let mut m = Model::default();
    
    let x = m.float(0.0, 10000.0);
    let y = m.float(0.0, 10000.0);
    let z = m.float(0.0, 10000.0);
    
    // x + 2y + 3z = 7500
    m.lin_eq(&[1.0, 2.0, 3.0], &[x, y, z], 7500.0);
    
    // x >= 500
    m.new(x.ge(500.0));
    
    // y >= 1000
    m.new(y.ge(1000.0));
    
    let solution = m.solve().expect("Should solve large domain linear eq");
    
    let x_val = solution.get_float(x);
    let y_val = solution.get_float(y);
    let z_val = solution.get_float(z);
    
    println!("Solution: x={}, y={}, z={}", x_val, y_val, z_val);
    
    let sum = x_val + 2.0 * y_val + 3.0 * z_val;
    assert!(
        (sum - 7500.0).abs() < 0.1, 
        "Large domain linear eq violated: {} + 2*{} + 3*{} = {} ≠ 7500", 
        x_val, y_val, z_val, sum
    );
}
