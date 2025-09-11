use cspsolver::prelude::*;

#[test]
fn debug_maximization_result() {
    let mut model = Model::default();
    let step_size = model.float_step_size();
    
    let x = model.new_var_float(1.0, 10.0);
    model.lt(x, float(5.5));
    
    let solution = model.maximize(x).expect("Should have solution");
    let Val::ValF(x_val) = solution[x] else { panic!("Expected float") };
    
    println!("Step size: {}", step_size);
    println!("Maximized x: {}", x_val);
    println!("Expected upper bound (prev(5.5)): {}", 5.5 - step_size);
    println!("Difference from expected: {}", x_val - (5.5 - step_size));
    
    // The result should be very close to prev(5.5)
    let expected = 5.5 - step_size;
    let tolerance = step_size * 1000.0; // Allow some tolerance for optimization
    
    assert!(x_val < 5.5, "x should be strictly less than 5.5");
    assert!(x_val >= expected - tolerance, "x should be close to the optimal value");
    
    println!("Test passed: x = {} is properly maximized for x < 5.5", x_val);
}
