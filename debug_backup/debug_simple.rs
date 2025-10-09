use selen::prelude::*;
use std::time::Instant;

fn main() -> SolverResult<()> {
    println!("=== Detailed Solver Investigation ===\n");
    
    // Test 1: Single variable works
    println!("Test 1: Single variable - minimize x where x >= 10");
    let start = Instant::now();
    let mut m = Model::default();
    let x = m.float(0.0, 100.0);
    post!(m, x >= float(10.0));
    let result = m.minimize(x)?;
    println!("  ✅ Completed in {:?}", start.elapsed());
    
    // Test 2: Two variables, minimize first one 
    println!("\nTest 2: Two variables, minimize first (no constraints between them)");
    let start = Instant::now();
    let mut m = Model::default();
    let x = m.float(0.0, 100.0);
    let y = m.float(0.0, 100.0);
    
    println!("  Variables created, adding constraint x >= float(10.0)");
    post!(m, x >= float(10.0));
    println!("  Constraint added, about to solve...");
    
    // Try to solve - this hangs
    let result = m.minimize(x);
    let duration = start.elapsed();
    
    match result {
        Ok(solution) => {
            println!("  ✅ Solved in {:?}", duration);
            if let Val::ValF(value) = solution[x] {
                println!("     x = {:.2}", value);
            }
        }
        Err(e) => {
            println!("  ❌ Error: {:?}", e);
        }
    }
    
    Ok(())
}