// Demo of timeout functionality in CSP solver

use cspsolver::prelude::*;
use std::time::Instant;

fn main() {
    println!("=== CSP Solver Timeout Demo ===\n");

    // Example 1: Simple problem that should solve quickly (no timeout needed)
    println!("1. Simple problem (should solve instantly):");
    let config = SolverConfig::default().with_timeout_seconds(5);
    let mut model = Model::with_config(config);
    let x = model.int(1, 3);
    let y = model.int(1, 3);
    post!(model, x != y);

    let start = Instant::now();
    match model.solve() {
        Some(solution) => {
            let duration = start.elapsed();
            println!("   ✓ Solution found in {:?}", duration);
            if let Val::ValI(x_val) = solution.get_values(&[x])[0] {
                println!("   x = {}", x_val);
            }
            if let Val::ValI(y_val) = solution.get_values(&[y])[0] {
                println!("   y = {}", y_val);
            }
        },
        None => {
            let duration = start.elapsed();
            println!("   ✗ No solution found in {:?}", duration);
        }
    }

    println!();

    // Example 2: Problem with very short timeout (should timeout)
    println!("2. Simple problem with very short timeout (should timeout):");
    let config = SolverConfig::default().with_timeout_seconds(0); // Immediate timeout
    let mut model = Model::with_config(config);
    let a = model.int(1, 1000);
    let b = model.int(1, 1000);
    post!(model, a != b);

    let start = Instant::now();
    match model.solve() {
        Some(solution) => {
            let duration = start.elapsed();
            println!("   ✓ Solution found in {:?} (timeout not reached)", duration);
            if let Val::ValI(a_val) = solution.get_values(&[a])[0] {
                println!("   a = {}", a_val);
            }
        },
        None => {
            let duration = start.elapsed();
            println!("   ✗ Timeout reached in {:?}", duration);
        }
    }

    println!();

    // Example 3: Configuration without timeout
    println!("3. Problem without timeout (unlimited time):");
    let config = SolverConfig::default(); // No timeout
    let mut model = Model::with_config(config);
    let p = model.int(1, 10);
    let q = model.int(1, 10);
    post!(model, p + q == int(15));

    let start = Instant::now();
    match model.solve() {
        Some(solution) => {
            let duration = start.elapsed();
            println!("   ✓ Solution found in {:?}", duration);
            if let Val::ValI(p_val) = solution.get_values(&[p])[0] {
                println!("   p = {}", p_val);
            }
            if let Val::ValI(q_val) = solution.get_values(&[q])[0] {
                println!("   q = {}", q_val);
            }
        },
        None => {
            let duration = start.elapsed();
            println!("   ✗ No solution found in {:?}", duration);
        }
    }

    println!("\n=== Timeout Feature Summary ===");
    println!("✓ Hard timeout implemented for constraint satisfaction");
    println!("✓ Timeout configuration through SolverConfig");
    println!("✓ Graceful handling when timeout is exceeded");
    println!("📝 Future: Soft timeout for optimization (return best solution so far)");
}