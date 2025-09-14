use cspsolver::prelude::*;
use std::time::Instant;

fn main() -> SolverResult<()> {
    println!("=== CSP Solver Performance Benchmarks ===");
    
    // Test 1: Simple single-variable optimization (baseline)
    let start = Instant::now();
    let mut iterations = 0;
    while start.elapsed().as_millis() < 100 {
        let mut m = Model::default();
        let x = m.float(0.0, 100.0);
        post!(m, x >= float(10.0));
        let _result = m.minimize(x)?;
        iterations += 1;
    }
    let duration = start.elapsed();
    let per_solve = duration.as_nanos() as f64 / iterations as f64;
    println!("Single Variable Optimization:");
    println!("  {} iterations in {:?}", iterations, duration);
    println!("  Average: {:.2}µs per solve", per_solve / 1000.0);

    // Test 2: Multi-variable model (previously hanging)
    let start = Instant::now();
    let mut iterations = 0;
    while start.elapsed().as_millis() < 100 {
        let mut m = Model::default();
        let x = m.float(0.0, 100.0);
        let _y = m.float(0.0, 100.0); // Extra variable, unused in objective
        post!(m, x >= float(10.0));
        let _result = m.minimize(x)?;
        iterations += 1;
    }
    let duration = start.elapsed();
    let per_solve = duration.as_nanos() as f64 / iterations as f64;
    println!("\nMulti-Variable Model (fix validation):");
    println!("  {} iterations in {:?}", iterations, duration);
    println!("  Average: {:.2}µs per solve", per_solve / 1000.0);

    // Test 3: Slightly more complex constraints
    let start = Instant::now();
    let mut iterations = 0;
    while start.elapsed().as_millis() < 100 {
        let mut m = Model::default();
        let x = m.float(0.0, 100.0);
        let y = m.float(0.0, 100.0);
        post!(m, x >= float(10.0));
        post!(m, y >= float(5.0));
        post!(m, x <= y);
        let _result = m.minimize(x)?;
        iterations += 1;
    }
    let duration = start.elapsed();
    let per_solve = duration.as_nanos() as f64 / iterations as f64;
    println!("\nConstrained Multi-Variable:");
    println!("  {} iterations in {:?}", iterations, duration);
    println!("  Average: {:.2}µs per solve", per_solve / 1000.0);

    // Test 4: Multiple constraints on the optimized variable
    let start = Instant::now();
    let mut iterations = 0;
    while start.elapsed().as_millis() < 100 {
        let mut m = Model::default();
        let x = m.float(0.0, 100.0);
        let y = m.float(0.0, 100.0);
        post!(m, x >= float(10.0));
        post!(m, x <= float(80.0));
        post!(m, y >= float(5.0));
        let _result = m.minimize(x)?;
        iterations += 1;
    }
    let duration = start.elapsed();
    let per_solve = duration.as_nanos() as f64 / iterations as f64;
    println!("\nMultiple Constraints on Objective:");
    println!("  {} iterations in {:?}", iterations, duration);
    println!("  Average: {:.2}µs per solve", per_solve / 1000.0);

    Ok(())
}