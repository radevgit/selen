//! Performance Best Practices for Runtime Constraint API
//!
//! This example demonstrates optimal usage patterns to achieve
//! the best performance with the runtime constraint API.

use selen::{
    model::Model,
    prelude::*,
    runtime_api::{ModelExt, VarIdExt},
};

fn main() {
    println!("🚀 Runtime API Performance Best Practices");
    println!("==========================================\n");

    // Best Practice 1: Use batch posting for multiple constraints
    demonstrate_batch_posting();
    
    // Best Practice 2: Prefer runtime API for complex expressions
    demonstrate_complex_expressions();
    
    // Best Practice 3: Use global constraints when possible
    demonstrate_global_constraints();
    
    // Best Practice 4: When to use post! macro vs runtime API
    demonstrate_api_selection();
    
    println!("\n✅ Performance best practices complete!");
}

/// Best Practice 1: Batch posting is more efficient than individual posting
fn demonstrate_batch_posting() {
    println!("📊 Best Practice 1: Batch Posting");
    
    let mut model = Model::default();
    let vars: Vec<_> = (0..100).map(|_| model.int(1, 100)).collect();
    
    // ✅ GOOD: Use batch posting for multiple constraints
    let constraints: Vec<_> = vars.iter().map(|&var| var.gt(25)).collect();
    model.postall(constraints);
    
    // ❌ AVOID: Individual posting for many constraints
    // for &var in &vars {
    //     model.post(var.gt(25));
    // }
    
    println!("  ✅ Use postall() for multiple constraints");
    println!("  ❌ Avoid individual post() calls in loops");
}

/// Best Practice 2: Runtime API is best for complex, data-driven expressions
fn demonstrate_complex_expressions() {
    println!("\n📊 Best Practice 2: Complex Expressions");
    
    let mut model = Model::default();
    let x = model.int(1, 100);
    let y = model.int(1, 100);
    let z = model.int(1, 100);
    
    // ✅ GOOD: Runtime API excels at complex, calculated expressions
    let coefficient = 2; // From data/config
    let threshold = 150; // From business rules
    
    model.post(x.add(y).mul(coefficient).sub(z).le(threshold));
    
    println!("  ✅ Use runtime API for data-driven constraint building");
    println!("  ✅ Excellent for complex mathematical expressions");
}

/// Best Practice 3: Global constraints are highly optimized
fn demonstrate_global_constraints() {
    println!("\n📊 Best Practice 3: Global Constraints");
    
    let mut model = Model::default();
    let vars: Vec<_> = (0..10).map(|i| model.int(1, i + 1)).collect();
    
    // ✅ GOOD: Use global constraints for common patterns
    model.alldiff(&vars);
    
    // ❌ AVOID: Manual implementation of global constraints
    // for i in 0..vars.len() {
    //     for j in i+1..vars.len() {
    //         model.post(vars[i].ne(vars[j]));
    //     }
    // }
    
    println!("  ✅ Use alldiff() instead of manual != constraints");
    println!("  ✅ Global constraints are highly optimized");
}

/// Best Practice 4: Choose the right API for the task
fn demonstrate_api_selection() {
    println!("\n📊 Best Practice 4: API Selection");
    
    let mut model = Model::default();
    let x = model.int(1, 100);
    let y = model.int(1, 100);
    
    println!("  Runtime API is best for:");
    println!("    • Data-driven constraint building");
    println!("    • Complex mathematical expressions");
    println!("    • Dynamic constraint generation");
    println!("    • Global constraint patterns");
    
    // ✅ Runtime API: Data-driven constraints
    let configs = [(x, 50), (y, 75)];
    for (var, threshold) in configs {
        model.post(var.gt(threshold));
    }
    
    println!("\n  post! macro is best for:");
    println!("    • Simple, static constraints");
    println!("    • Maximum performance for basic operations");
    println!("    • Direct translation from mathematical notation");
    
    // ✅ post! macro: Simple, static constraints
    let sum = model.int(1, 200);
    post!(model, x + y == sum);
    post!(model, sum <= 100);
    post!(model, x > 10);
    
    println!("\n  Performance characteristics:");
    println!("    • Runtime API: ~3-4x overhead for simple constraints");
    println!("    • Runtime API: ~1.4x overhead for complex expressions");
    println!("    • Batch operations reduce per-constraint overhead");
}

/// Performance measurement utility
#[allow(dead_code)]
fn measure_performance() {
    use std::time::Instant;
    
    let constraint_count = 1000;
    
    // Runtime API measurement
    let start = Instant::now();
    let mut model1 = Model::default();
    let vars1: Vec<_> = (0..constraint_count).map(|_| model1.int(1, 100)).collect();
    
    let constraints: Vec<_> = vars1.iter().map(|&var| var.gt(50)).collect();
    model1.postall(constraints);
    
    let runtime_duration = start.elapsed();
    
    // post! macro measurement
    let start = Instant::now();
    let mut model2 = Model::default();
    let vars2: Vec<_> = (0..constraint_count).map(|_| model2.int(1, 100)).collect();
    
    for &var in &vars2 {
        post!(model2, var > 50);
    }
    
    let post_duration = start.elapsed();
    
    println!("Performance comparison:");
    println!("  Runtime API: {:?} ({:.0} constraints/sec)", 
        runtime_duration, 
        constraint_count as f64 / runtime_duration.as_secs_f64());
    println!("  post! macro: {:?} ({:.0} constraints/sec)", 
        post_duration, 
        constraint_count as f64 / post_duration.as_secs_f64());
    println!("  Overhead: {:.1}x", 
        runtime_duration.as_secs_f64() / post_duration.as_secs_f64());
}

/// Memory usage optimization tips
#[allow(dead_code)]
fn memory_optimization_tips() {
    println!("\n🧠 Memory Optimization Tips:");
    
    let mut model = Model::default();
    let vars: Vec<_> = (0..100).map(|_| model.int(1, 100)).collect();
    
    // ✅ GOOD: Pre-allocate vectors for large constraint sets
    let mut constraints = Vec::with_capacity(vars.len());
    for &var in &vars {
        constraints.push(var.gt(25));
    }
    model.postall(constraints);
    
    // ✅ GOOD: Use references to avoid cloning (when working with borrowed data)
    // let var_refs: Vec<&VarId> = vars.iter().collect();
    
    // ✅ GOOD: Reuse constraint patterns
    let threshold = 50;
    let reusable_constraint = |var: VarId| var.le(threshold);
    
    for &var in &vars[0..10] {
        model.post(reusable_constraint(var));
    }
    
    println!("  ✅ Pre-allocate vectors with with_capacity()");
    println!("  ✅ Use references to avoid unnecessary cloning");
    println!("  ✅ Reuse constraint patterns with closures");
}