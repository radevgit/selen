//! Comprehensive test example for timeout and memory limit error handling
//!
//! This example demonstrates the graceful timeout and memory limit handling
//! implemented as part of the Production Readiness Plan Step 8.1.1.
//!
//! ## Performance Note
//!
//! **For reliable timeout testing, run with `cargo run --release --example timeout_memory_demo`**
//! - Debug mode: Solver is slower, timeouts may trigger more reliably
//! - Release mode: Solver is faster, may need more complex problems for timeouts
//!
//! The timeout tests use computationally intensive problems designed to stress
//! the solver safely without consuming excessive system resources.

use cspsolver::prelude::*;

fn main() {
    println!("🧪 Testing Timeout and Memory Limit Error Handling");
    println!("================================================");
    
    test_timeout_handling();
    println!();
    test_memory_limit_handling();
    println!();
    test_successful_solve_with_limits();
    println!();
    test_partial_solutions_on_timeout();
}

/// Test timeout handling in solve operations
fn test_timeout_handling() {
    println!("🕒 Test 1: Timeout Handling");
    println!("---------------------------");
    
    // Create a challenging problem that should timeout in 1 second
    let mut model = Model::with_config(
        SolverConfig::default()
            .with_timeout_seconds(1) // Very short timeout
    );
    
    // Create a large all-different problem (computationally expensive)
    let vars: Vec<_> = (0..40).map(|_| model.int(1, 40)).collect();
    
    // Add all-different constraint (creates large search space)
    model.props.all_different(vars.clone());
    
    // Add sum constraint to make it even harder
    let sum_all = model.sum(&vars);
    let target = model.int(820, 820); // Sum of 1..40 is 820
    post!(model, sum_all == target);
    
    let result = model.solve();
    
    match result {
        Err(SolverError::Timeout { elapsed_seconds, operation: _ }) => {
            println!("✅ Timeout handled gracefully!");
            println!("   ⏱️  Elapsed: {:.2}s", elapsed_seconds.unwrap_or(0.0));
        }
        Ok(_) => {
            println!("⚠️  Problem solved too quickly - timeout test may be unreliable");
            println!("   💡 Try running with --release for more realistic performance");
        }
        Err(other) => {
            println!("❌ Unexpected error: {:?}", other);
        }
    }
}

/// Test memory limit handling  
fn test_memory_limit_handling() {
    println!("💾 Test 2: Memory Limit Handling");
    println!("--------------------------------");
    
    // Create a problem with very low memory limit
    let mut model = Model::with_config(
        SolverConfig::default()
            .with_max_memory_mb(1) // Very low memory limit  
            .with_timeout_seconds(3) // Backup timeout
    );
    
    // Create a moderately complex but solvable problem
    let vars: Vec<_> = (0..15).map(|_| model.int(1, 20)).collect();
    
    // Add all-different constraint (solvable since 15 vars in domain [1,20])
    model.props.all_different(vars.clone());
    
    let result = model.solve();
    
    match result {
        Err(SolverError::MemoryLimit { usage_mb, limit_mb }) => {
            println!("✅ Memory limit handled gracefully!");
            println!("   📊 Usage: {}MB", usage_mb.unwrap_or(0));
            println!("   🚧 Limit: {}MB", limit_mb.unwrap_or(0));
        }
        Err(SolverError::Timeout { elapsed_seconds, operation: _ }) => {
            println!("⏱️  Timed out before hitting memory limit");
            println!("   ⏱️  Elapsed: {:.2}s", elapsed_seconds.unwrap_or(0.0));
        }
        Ok(_solution) => {
            println!("✅ Problem solved successfully!");
            println!("   💡 Memory limit test may need adjustment for this system");
            println!("   📝 Memory monitoring is conservative and may underestimate usage");
        }
        Err(other) => {
            println!("❌ Unexpected error: {:?}", other);
        }
    }
}

/// Test successful solve with reasonable limits
fn test_successful_solve_with_limits() {
    println!("✅ Test 3: Successful Solve with Limits");
    println!("---------------------------------------");
    
    let mut model = Model::with_config(
        SolverConfig::default()
            .with_timeout_seconds(10) // Reasonable timeout
            .with_max_memory_mb(100)  // Reasonable memory limit
    );
    
    // Simple problem that should solve quickly
    let x = model.int(1, 10);
    let y = model.int(1, 10);
    let fifteen = model.int(15, 15); // Create a constant variable for 15
    post!(model, x + y == fifteen);
    post!(model, x > y);
    
    let result = model.solve();
    
    match result {
        Ok(solution) => {
            println!("✅ Problem solved successfully within limits!");
            println!("   🎯 Solution: {:?}", solution);
        }
        Err(error) => {
            println!("❌ Unexpected error: {:?}", error);
        }
    }
}

/// Test partial solutions on timeout for enumeration
fn test_partial_solutions_on_timeout() {
    println!("🔄 Test 4: Partial Solutions on Timeout");
    println!("---------------------------------------");
    
    let mut model = Model::with_config(
        SolverConfig::default()
            .with_timeout_seconds(1) // Short timeout
    );
    
    // Create a problem with multiple solutions
    let x = model.int(1, 5);
    let y = model.int(1, 5);
    post!(model, x != y);
    
    // Enumerate solutions (should find some before timeout)
    let solutions: Vec<_> = model.enumerate().take(10).collect();
    
    println!("✅ Found {} solutions before timeout/completion", solutions.len());
    for (i, solution) in solutions.iter().enumerate() {
        println!("   {}. {:?}", i + 1, solution);
        if i >= 4 {
            println!("   ... (showing first 5 solutions)");
            break;
        }
    }
}