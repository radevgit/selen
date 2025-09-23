use selen::prelude::*;
use std::time::Instant;

fn main() -> SolverResult<()> {
    println!("=== Step 1: Debug Optimization Failure Point ===\n");
    
    // Test the failing case with debug info
    println!("Case: Two variables, minimize x with x >= 10.0, unused y");
    let mut m = Model::default();
    let x = m.float(0.0, 100.0);
    let y = m.float(0.0, 100.0);
    post!(m, x >= float(10.0));
    
    println!("Model state:");
    println!("- Variables: x={:?}, y={:?}", x, y);
    println!("- Constraints: x >= 10.0");
    println!("- Objective: minimize x");
    
    println!("\nThis should be optimizable because:");
    println!("- Objective is a simple variable (x)");
    println!("- Only bound constraints on x");
    println!("- Variable y is irrelevant to optimization");
    
    println!("\nLet's see what happens in the optimization chain...");
    
    // We need to manually check the optimization steps that m.minimize() does:
    // 1. minimize() calls minimize_and_iterate().last()
    // 2. minimize_and_iterate() calls try_optimization_minimize()
    // 3. If that returns None, falls back to search
    
    println!("\n🔍 Investigation needed:");
    println!("1. Does try_optimization_minimize() fail?");
    println!("2. Why does ProblemClassifier reject this case?");
    println!("3. Why does search hang when optimization fails?");
    
    // For now, let's just show that this hangs
    let start = Instant::now();
    let result = m.minimize(x);
    let duration = start.elapsed();
    
    match result {
        Ok(solution) => {
            println!("✅ Unexpected success: {:?}", duration);
            if let Val::ValF(value) = solution[x] {
                println!("   x = {:.2}", value);
            }
        }
        Err(e) => {
            println!("❌ Error: {:?}", e);
        }
    }
    
    Ok(())
}