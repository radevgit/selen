use selen::prelude::*;
use std::time::Instant;

fn main() -> SolverResult<()> {
    println!("=== Solver Hanging Investigation ===\n");
    
    // Test 1: Single variable (this works)
    println!("Test 1: Single variable constraint");
    let start = Instant::now();
    let mut m = Model::default();
    let x = m.new_var(0.0.into(), 100.0.into());
    post!(m, x >= 10.0);
    let result = m.minimize(x)?;
    println!("  ✅ Single variable: {:?}", start.elapsed());
    if let Val::ValF(value) = result[x] {
        println!("     Value: {:.2}", value);
    }
    
    // Test 2: Two variables, no relation (should work)
    println!("\nTest 2: Two independent variables");
    let start = Instant::now();
    let mut m = Model::default();
    let x = m.new_var(0.0.into(), 100.0.into());
    let y = m.new_var(0.0.into(), 100.0.into());
    post!(m, x >= 10.0);
    post!(m, y >= 20.0);
    let result = m.minimize(x)?;
    println!("  ✅ Two independent: {:?}", start.elapsed());
    if let Val::ValF(value) = result[x] {
        println!("     Value: {:.2}", value);
    }
    
    // Test 3: Two variables with relation (this hangs)
    println!("\nTest 3: Two variables with x <= y relation");
    let start = Instant::now();
    let mut m = Model::default();
    let x = m.new_var(0.0.into(), 100.0.into());
    let y = m.new_var(0.0.into(), 100.0.into());
    post!(m, x >= 10.0);
    post!(m, y >= 20.0);
    
    println!("  About to add: x <= y");
    post!(m, x <= y);  // This is where it likely hangs
    println!("  ✅ Constraint added successfully");
    
    println!("  About to solve...");
    let result = m.minimize(x);
    let duration = start.elapsed();
    
    match result {
        Ok(solution) => {
            println!("  ✅ Solved in {:?}", duration);
            if let Val::ValF(value) = solution[x] {
                println!("     x = {:.2}", value);
            }
            if let Val::ValF(value) = solution[y] {
                println!("     y = {:.2}", value);
            }
        }
        Err(e) => {
            println!("  ❌ Error after {:?}: {:?}", duration, e);
        }
    }
    
    Ok(())
}