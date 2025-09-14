// Simple demonstration of the simplified SolverConfig system

use cspsolver::prelude::*;

fn main() {
    println!("=== SolverConfig Simple Demo ===\n");

    // Example 1: Default configuration
    let default_config = SolverConfig::default();
    println!("Default config:");
    println!("- Float precision: {} digits", default_config.float_precision_digits);
    println!("- Timeout: {:?} seconds", default_config.timeout_seconds);
    println!("- Memory limit: {:?} MB", default_config.max_memory_mb);

    // Example 2: High precision configuration
    let high_precision_config = SolverConfig::default()
        .with_float_precision(15)
        .with_timeout_seconds(60)
        .with_max_memory_mb(512);
    
    println!("\nHigh precision config:");
    println!("- Float precision: {} digits", high_precision_config.float_precision_digits);
    println!("- Timeout: {:?} seconds", high_precision_config.timeout_seconds);
    println!("- Memory limit: {:?} MB", high_precision_config.max_memory_mb);

    // Example 3: Using the config with a model
    let mut model = Model::with_config(high_precision_config);
    let x = model.int(0, 10);
    let y = model.int(0, 10);
    
    // Add a simple constraint using mathematical syntax
    post!(model, x + y >= int(5));
    
    println!("\nCreated model with high precision config");
    println!("Variables: x ∈ [0, 10], y ∈ [0, 10]");
    println!("Constraint: x + y >= 5");

    match model.solve() {
        Ok(solution) => {
            use cspsolver::vars::Val;
            println!("Solution found:");
            if let Val::ValI(x_val) = solution.get_values(&[x])[0] {
                println!("x = {}", x_val);
            }
            if let Val::ValI(y_val) = solution.get_values(&[y])[0] {
                println!("y = {}", y_val);
            }
        }
        Err(err) => {
            println!("No solution found: {}", err);
        }
    }
}