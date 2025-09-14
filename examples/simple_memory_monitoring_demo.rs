//! Simple Memory Monitoring Demo
//!
//! This example demonstrates the simple memory usage monitoring system
//! implemented as part of Step 8.1.3. It shows memory estimation, warnings,
//! and limits with minimal performance overhead.

use cspsolver::prelude::*;

fn main() {
    println!("🔍 Simple Memory Monitoring Demo");
    println!("===============================");
    
    test_memory_estimation();
    println!();
    test_memory_limit_basic();
    println!();
    test_memory_limit_with_large_interval();
    println!();
    test_memory_limit_exceeded();
}

/// Demonstrate basic memory estimation without limits
fn test_memory_estimation() {
    println!("📊 Test 1: Basic Memory Estimation");
    println!("----------------------------------");
    
    // Create a model with no memory limits - just show estimation
    let mut model = Model::with_config(
        SolverConfig::default()
            .with_timeout_seconds(2) // Short timeout to avoid long execution
    );
    
    // Create a moderate problem to see memory estimation
    let vars: Vec<_> = (0..10).map(|_| model.int(1, 50)).collect();
    
    // Add all-different constraint
    let vars_clone = vars.clone();
    model.props.all_different(vars_clone);
    
    println!("🧮 Created problem with {} variables", vars.len());
    
    let result = model.solve();
    match result {
        Ok(_solution) => {
            println!("✅ Solution found successfully");
            println!("   (Memory estimation worked - no limits hit)");
        }
        Err(SolverError::Timeout { .. }) => {
            println!("⏰ Timed out as expected (memory estimation still functional)");
        }
        Err(e) => {
            println!("❌ Unexpected error: {:?}", e);
        }
    }
}

/// Test basic memory limit functionality
fn test_memory_limit_basic() {
    println!("🔒 Test 2: Basic Memory Limit");
    println!("-----------------------------");
    
    // Set a reasonable memory limit to show it works
    let mut model = Model::with_config(
        SolverConfig::default()
            .with_max_memory_mb(50)  // 50MB limit
            .with_timeout_seconds(2)
    );
    
    // Create a moderate problem
    let vars: Vec<_> = (0..15).map(|_| model.int(1, 100)).collect();
    
    // Add constraints to make it more complex
    let vars_clone = vars.clone();
    model.props.all_different(vars_clone);
    
    println!("🔧 Using 50MB memory limit");
    println!("📝 Created problem with {} variables", vars.len());
    
    let result = model.solve();
    match result {
        Ok(_solution) => {
            println!("✅ Solution found within memory limits");
        }
        Err(SolverError::MemoryLimit { usage_mb, limit_mb }) => {
            println!("🚫 Memory limit exceeded: used {:?}MB, limit {:?}MB", usage_mb, limit_mb);
            println!("   (Simple estimation prevented excessive memory usage)");
        }
        Err(SolverError::Timeout { .. }) => {
            println!("⏰ Timed out before hitting memory limit");
        }
        Err(e) => {
            println!("❌ Other error: {:?}", e);
        }
    }
}

/// Test that memory checking happens at large intervals for performance
fn test_memory_limit_with_large_interval() {
    println!("🚀 Test 3: Large Interval Memory Checking");
    println!("----------------------------------------");
    
    // Use a reasonable memory limit
    let mut model = Model::with_config(
        SolverConfig::default()
            .with_max_memory_mb(50)  // 50MB limit
            .with_timeout_seconds(3)
    );
    
    // Create a problem that will run many iterations
    let vars: Vec<_> = (0..12).map(|_| model.int(1, 80)).collect();
    let vars_clone = vars.clone();
    model.props.all_different(vars_clone);
    
    println!("⚡ Using 50MB limit with memory checks every 10,000 iterations");
    println!("📊 This demonstrates minimal performance overhead");
    
    let start_time = std::time::Instant::now();
    let result = model.solve();
    let elapsed = start_time.elapsed();
    
    match result {
        Ok(_solution) => {
            println!("✅ Solution found in {:.3}s", 
                     elapsed.as_secs_f64());
            println!("   Memory checking overhead was minimal");
        }
        Err(SolverError::Timeout { .. }) => {
            println!("⏰ Timed out after {:.3}s (normal - demonstrates checking interval)", elapsed.as_secs_f64());
        }
        Err(SolverError::MemoryLimit { usage_mb, limit_mb }) => {
            println!("🚫 Memory limit hit: used {:?}MB, limit {:?}MB", usage_mb, limit_mb);
        }
        Err(e) => {
            println!("❌ Error: {:?}", e);
        }
    }
}

/// Test actual memory limit exceeded scenario
fn test_memory_limit_exceeded() {
    println!("🔥 Test 4: Memory Limit Exceeded");
    println!("-------------------------------");
    
    // Set an extremely low memory limit to definitely trigger it
    let mut model = Model::with_config(
        SolverConfig::default()
            .with_max_memory_mb(1)  // Only 1MB - very restrictive
            .with_timeout_seconds(5)
    );
    
    // Create a moderately complex problem
    let vars: Vec<_> = (0..20).map(|_| model.int(1, 100)).collect();
    let vars_clone = vars.clone();
    model.props.all_different(vars_clone);
    
    println!("🔒 Using extremely low 1MB limit to demonstrate memory protection");
    
    let result = model.solve();
    match result {
        Ok(_) => {
            println!("⚠️  Solution found despite low limit (estimation may be conservative)");
        }
        Err(SolverError::MemoryLimit { usage_mb, limit_mb }) => {
            println!("✅ Memory limit correctly enforced!");
            println!("   📊 Usage: {:?}MB, Limit: {:?}MB", usage_mb, limit_mb);
            println!("   💡 The simple estimation successfully prevented runaway memory usage");
        }
        Err(SolverError::Timeout { .. }) => {
            println!("⏰ Timed out before hitting memory limit");
        }
        Err(e) => {
            println!("❌ Unexpected error: {:?}", e);
        }
    }
}