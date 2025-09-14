//! Comprehensive test example for timeout and memory limit error handling
//!
//! This example demonstrates the graceful timeout and memory limit handling
//! implemented as part of the Production Readiness Plan Step 8.1.1.

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
    
    // Create a complex problem that should timeout
    let mut model = Model::with_config(
        SolverConfig::default()
            .with_timeout_seconds(1) // Very short timeout
    );
    
    // Create many variables with large domains to make solving slow
    let vars: Vec<_> = (0..20).map(|_| model.int(1, 100)).collect();
    
    // Add all-different constraint to make the problem complex
    let vars_clone = vars.clone();
    model.props.all_different(vars_clone);
    
    // This should timeout
    let result = model.solve();
    
    match result {
        Err(SolverError::Timeout { elapsed_seconds, operation }) => {
            println!("✅ Timeout handled gracefully!");
            println!("   ⏱️  Elapsed: {:.2}s", elapsed_seconds.unwrap_or(0.0));
            println!("   🔧 Operation: {}", operation.as_deref().unwrap_or("unknown"));
        }
        Ok(_) => {
            println!("⚠️  Problem solved too quickly - timeout test may be unreliable");
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
            .with_max_memory_mb(2) // Very low memory limit
            .with_timeout_seconds(5) // Reasonable timeout as backup
    );
    
    // Create a problem that might consume memory
    let vars: Vec<_> = (0..15).map(|_| model.int(1, 50)).collect();
    
    // Add all-different constraint instead of manual != constraints
    let vars_clone = vars.clone();
    model.props.all_different(vars_clone);
    
    let result = model.solve();
    
    match result {
        Err(SolverError::MemoryLimit { usage_mb, limit_mb }) => {
            println!("✅ Memory limit handled gracefully!");
            println!("   📊 Usage: {}MB", usage_mb.unwrap_or(0));
            println!("   🚧 Limit: {}MB", limit_mb.unwrap_or(0));
        }
        Err(SolverError::Timeout { elapsed_seconds, operation }) => {
            println!("⏱️  Timed out before hitting memory limit");
            println!("   ⏱️  Elapsed: {:.2}s", elapsed_seconds.unwrap_or(0.0));
        }
        Ok(solution) => {
            println!("✅ Problem solved successfully: {:?}", solution);
            println!("   (Memory limit test may be unreliable - problem too simple)");
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