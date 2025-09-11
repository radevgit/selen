use cspsolver::prelude::*;

#[test]
fn trace_optimization_precision_4() {
    println!("=== TRACING PRECISION 4 (should work) ===");
    let mut model = Model::with_float_precision(4);
    let step_size = model.float_step_size();
    println!("Step size: {}", step_size);
    
    let x = model.new_var_float(1.0, 10.0);
    println!("Created variable x with initial bounds [1.0, 10.0]");
    
    model.lt(x, float(5.5));
    println!("Added constraint x < 5.5 (implemented as x.next() <= 5.5)");
    println!("This means x + {} <= 5.5, so x <= {}", step_size, 5.5 - step_size);
    
    println!("Starting maximization...");
    let solution = model.maximize(x).expect("Should have solution");
    
    let Val::ValF(x_val) = solution[x] else { panic!("Expected float") };
    println!("Final result: x = {}", x_val);
    println!("Expected: close to {}", 5.5 - step_size);
    println!("Difference: {}", (x_val - (5.5 - step_size)).abs());
    
    assert!(x_val < 5.5);
}

#[test]
fn trace_optimization_precision_6_limited() {
    println!("\n=== TRACING PRECISION 6 (hangs - will timeout) ===");
    let mut model = Model::with_float_precision(6);
    let step_size = model.float_step_size();
    println!("Step size: {}", step_size);
    
    let x = model.new_var_float(1.0, 10.0);
    println!("Created variable x with initial bounds [1.0, 10.0]");
    
    model.lt(x, float(5.5));
    println!("Added constraint x < 5.5 (implemented as x.next() <= 5.5)");
    println!("This means x + {} <= 5.5, so x <= {}", step_size, 5.5 - step_size);
    
    println!("Starting maximization (will likely hang)...");
    
    // This will likely hang, but we can see the setup
    // let solution = model.maximize(x).expect("Should have solution");
}
