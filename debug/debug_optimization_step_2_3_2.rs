//! Debug Step 2.3.2 optimization to understand why the test is failing
//! 
//! This reproduces the exact scenario from test_less_than_with_floats to debug
//! what our optimization system is producing vs what the test expects.

use cspsolver::prelude::*;

fn main() {
    println!("=== Debug Step 2.3.2 Optimization ===");
    
    // Reproduce the exact test scenario
    let mut model = Model::default();
    let x = model.float(1.0, 10.0);
    model.less_than(x, 5.5);
    
    println!("Created model with:");
    println!("  x ∈ [1.0, 10.0]");
    println!("  constraint: x < 5.5");
    println!();
    
    // Try to maximize x
    println!("Attempting to maximize x...");
    
    match model.maximize(x).last() {
        Some(solution) => {
            let x_value = solution[x].as_float();
            println!("✅ Optimization succeeded!");
            println!("  x = {}", x_value);
            println!("  Expected: x > 5.4 (very close to 5.5)");
            
            if x_value > 5.4 {
                println!("✅ Result is correct!");
            } else {
                println!("❌ Result is incorrect - should be > 5.4");
                println!("   This suggests our optimization is not handling constraints properly");
            }
        },
        None => {
            println!("❌ Optimization failed - no solution found");
            println!("   This suggests our optimization router is falling back to search");
            println!("   which means there might be an issue with our float variable detection");
        }
    }
}
