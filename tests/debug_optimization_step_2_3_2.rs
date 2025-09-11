//! Debug Step 2.3.2 - Single Variable Optimization Integration
//! 
//! This test helps debug the optimization to understand why the test is failing

use cspsolver::prelude::*;

#[test]
fn debug_step_2_3_2_optimization() {
    println!("=== Debug Step 2.3.2 Optimization ===");
    
    // First test: constraint-free case (should work)
    println!("\n--- Test 1: Constraint-free maximization ---");
    let mut model1 = Model::default();
    let x1 = model1.new_var_float(1.0, 10.0);
    // NO constraints
    
    println!("Created model with:");
    println!("  x ∈ [1.0, 10.0]");
    println!("  NO constraints");
    println!();
    
    println!("Attempting to maximize x...");
    match model1.maximize(x1) {
        Some(solution) => {
            let Val::ValF(x_value) = solution[x1] else { 
                panic!("Expected float value"); 
            };
            println!("✅ Constraint-free optimization succeeded!");
            println!("  x = {}", x_value);
            println!("  Expected: x = 10.0 (max of domain)");
            
            if (x_value - 10.0).abs() < 0.001 {
                println!("✅ Result is correct!");
            } else {
                println!("❌ Result is incorrect - should be 10.0");
            }
        },
        None => {
            println!("❌ Constraint-free optimization failed");
        }
    }
    
    // Second test: with constraints (may fall back, but shouldn't hang)
    println!("\n--- Test 2: With constraints (may fall back) ---");
    let mut model2 = Model::default();
    let x2 = model2.new_var_float(1.0, 10.0);
    model2.lt(x2, float(5.5));
    
    println!("Created model with:");
    println!("  x ∈ [1.0, 10.0]");
    println!("  constraint: x < 5.5");
    println!();
    
    println!("Attempting to maximize x...");
    match model2.maximize(x2) {
        Some(solution) => {
            let Val::ValF(x_value) = solution[x2] else { 
                panic!("Expected float value"); 
            };
            println!("✅ Constrained optimization succeeded!");
            println!("  x = {}", x_value);
            println!("  Expected: x > 5.4 (very close to 5.5)");
            
            if x_value > 5.4 && x_value < 5.5 {
                println!("✅ Result is correct!");
            } else {
                println!("❌ Result is incorrect - expected > 5.4 and < 5.5");
                println!("   (This may be expected if Step 2.3.2 falls back to search)");
            }
        },
        None => {
            println!("❌ Constrained optimization failed - no solution found");
        }
    }
}
