use cspsolver::prelude::*;

#[test]
fn trace_optimization_steps_precision_4() {
    println!("=== TRACING OPTIMIZATION STEPS - PRECISION 4 ===");
    let mut model = Model::with_float_precision(4);
    let step_size = model.float_step_size();
    println!("Step size: {}", step_size);
    
    let x = model.new_var_float(1.0, 10.0);
    model.less_than(x, float(5.5));
    
    println!("Using maximize_with_callback to track progress...");
    let solution = model.maximize_with_callback(x, |stats| {
        println!("Solve completed - Propagations: {}, Nodes: {}", 
                stats.propagation_count, stats.node_count);
    });
    
    if let Some(sol) = solution {
        let Val::ValF(x_val) = sol[x] else { panic!("Expected float") };
        println!("Final result: x = {}", x_val);
    } else {
        println!("No solution found");
    }
}

#[test]
fn trace_optimization_steps_precision_6() {
    println!("\n=== TRACING OPTIMIZATION STEPS - PRECISION 6 ===");
    let mut model = Model::with_float_precision(6);
    let step_size = model.float_step_size();
    println!("Step size: {}", step_size);
    
    let x = model.new_var_float(1.0, 10.0);
    model.less_than(x, float(5.5));
    
    println!("Using maximize_with_callback to track progress...");
    println!("This will likely hang - the callback won't be called until completion");
    
    // Try to use the iterator approach to get early results
    println!("Trying iterator approach to get first few solutions...");
    let mut solutions = model.maximize_and_iterate(x);
    
    println!("Getting first solution (if any)...");
    if let Some(sol) = solutions.next() {
        let Val::ValF(x_val) = sol[x] else { panic!("Expected float") };
        println!("First solution: x = {}", x_val);
    } else {
        println!("No first solution found (this may hang)");
    }
}
