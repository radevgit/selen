use cspsolver::prelude::*;
use std::time::Instant;

fn main() -> SolverResult<()> {
    println!("=== Debug Optimization Path ===\n");
    
    // Test case that hangs: two variables, minimize one
    println!("Creating model with two variables...");
    let mut m = Model::default();
    let x = m.float(0.0, 100.0);
    let y = m.float(0.0, 100.0);
    
    println!("Adding constraint: x >= float(10.0)");
    post!(m, x >= float(10.0));
    
    println!("About to call minimize(x)...");
    println!("This will:");
    println!("1. Call minimize_and_iterate(x).last()");
    println!("2. Which calls try_optimization_minimize first");
    println!("3. If that fails, falls back to search");
    
    let start = Instant::now();
    
    // This is where it hangs
    let result = m.minimize(x);
    let duration = start.elapsed();
    
    match result {
        Ok(solution) => {
            println!("✅ Success after {:?}", duration);
            if let Val::ValF(value) = solution[x] {
                println!("   x = {:.2}", value);
            }
        }
        Err(e) => {
            println!("❌ Error after {:?}: {:?}", duration, e);
        }
    }
    
    Ok(())
}