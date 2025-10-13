// Test LP solver integration with CSP search
use selen::prelude::*;

#[test]
fn test_lp_integration_simple_linear() {
    // Simple linear system:
    // x + y <= 10
    // x >= 2
    // y >= 3
    // Should tighten bounds
    
    let mut model = Model::default();
    let x = model.float(2.0, 20.0);
    let y = model.float(3.0, 20.0);
    
    // x + y <= 10
    model.lin_le(&[1.0, 1.0], &[x, y], 10.0);
    
    // Any feasible solution will work - LP should tighten bounds
    let result = model.solve();
    assert!(result.is_ok(), "Should find a solution");
    
    let sol = result.unwrap();
    let x_val = sol.get_float(x);
    let y_val = sol.get_float(y);
    
    // Check solution satisfies constraints
    assert!(x_val >= 2.0);
    assert!(y_val >= 3.0);
    assert!(x_val + y_val <= 10.0 + 1e-6);
}

#[test]
fn test_lp_integration_infeasible() {
    // Infeasible linear system:
    // x + y <= 5
    // x >= 10
    // y >= 10
    // LP should detect infeasibility
    
    let mut model = Model::default();
    let x = model.float(10.0, 20.0);
    let y = model.float(10.0, 20.0);
    
    // x + y <= 5
    model.lin_le(&[1.0, 1.0], &[x, y], 5.0);
    
    // Should find no solution
    let result = model.solve();
    assert!(result.is_err(), "Should find no solution for infeasible problem");
}

#[test]
fn test_lp_integration_int_linear() {
    // Integer linear constraint
    // 2x + 3y <= 20
    // x >= 1, y >= 1
    
    let mut model = Model::default();
    let x = model.int(1, 10);
    let y = model.int(1, 10);
    
    // 2x + 3y <= 20
    model.lin_le(&[2, 3], &[x, y], 20);
    
    // Should find a solution
    let result = model.solve();
    assert!(result.is_ok(), "Should find a solution");
    
    let sol = result.unwrap();
    let x_val = sol.get_int(x);
    let y_val = sol.get_int(y);
    
    // Verify constraint
    assert!(2 * x_val + 3 * y_val <= 20);
}
