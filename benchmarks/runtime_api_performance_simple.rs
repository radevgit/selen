//! Simple Performance Test for Runtime Constraint API
//!
//! This demonstrates the performance characteristics of the runtime API
//! and compares it to traditional constraint building approaches.

use selen::{
    model::Model,
    prelude::*,
    runtime_api::{ModelExt, VarIdExt},
};
use std::time::Instant;

fn main() {
    println!("🚀 Runtime API Performance Test");
    println!("================================\n");

    // Test 1: Basic constraint creation performance
    test_basic_constraint_performance();
    
    // Test 2: Global constraints performance
    test_global_constraints_performance();
    
    // Test 3: Complex expression building performance
    test_expression_building_performance();
    
    println!("\n✅ Performance testing complete!");
}

fn test_basic_constraint_performance() {
    println!("📊 Test 1: Basic Constraint Creation");
    
    let constraint_count = 5000;
    
    // Runtime API approach
    let start = Instant::now();
    let mut model1 = Model::default();
    let vars1: Vec<_> = (0..constraint_count).map(|_| model1.int(1, 100)).collect();
    
    for &var in &vars1 {
        model1.post(var.gt(50));
    }
    
    let runtime_api_duration = start.elapsed();
    
    // Traditional post! macro approach
    let start = Instant::now();
    let mut model2 = Model::default();
    let vars2: Vec<_> = (0..constraint_count).map(|_| model2.int(1, 100)).collect();
    
    for &var in &vars2 {
        post!(model2, var > 50);
    }
    
    let post_macro_duration = start.elapsed();
    
    println!("  Runtime API:   {:?} ({:.2} constraints/sec)", 
        runtime_api_duration, constraint_count as f64 / runtime_api_duration.as_secs_f64());
    println!("  post! macro:   {:?} ({:.2} constraints/sec)", 
        post_macro_duration, constraint_count as f64 / post_macro_duration.as_secs_f64());
    
    let ratio = runtime_api_duration.as_secs_f64() / post_macro_duration.as_secs_f64();
    println!("  Performance ratio: {:.2}x (runtime API vs post! macro)", ratio);
    
    if ratio <= 1.5 {
        println!("  ✅ Performance overhead is acceptable");
    } else {
        println!("  ⚠️  Significant performance overhead detected");
    }
}

fn test_global_constraints_performance() {
    println!("\n📊 Test 2: Global Constraints Performance");
    
    // Test alldiff with different sizes
    for size in [10, 50, 100] {
        let start = Instant::now();
        let mut model = Model::default();
        let vars: Vec<_> = (0..size).map(|_| model.int(1, size * 2)).collect();
        
        model.alldiff(&vars);
        
        let duration = start.elapsed();
        println!("  alldiff({}): {:?}", size, duration);
    }
    
    // Test count constraints
    let start = Instant::now();
    let mut model = Model::default();
    let vars: Vec<_> = (0..100).map(|_| model.int(1, 10)).collect();
    
    for i in 1..=10 {
        let count_var = model.int(0, 100);
        model.count(&vars, i, count_var);
    }
    
    let duration = start.elapsed();
    println!("  count constraints (10x): {:?}", duration);
}

fn test_expression_building_performance() {
    println!("\n📊 Test 3: Expression Building Performance");
    
    let expr_count = 1000;
    
    // Runtime API - complex expressions
    let start = Instant::now();
    let mut model = Model::default();
    let vars: Vec<_> = (0..expr_count).map(|_| model.int(1, 100)).collect();
    
    for i in 0..vars.len() {
        if i < vars.len() - 1 {
            model.post(vars[i].add(vars[i + 1]).mul(2).le(200));
        }
    }
    
    let runtime_duration = start.elapsed();
    
    println!("  Runtime API complex expressions: {:?} ({:.2} expr/sec)", 
        runtime_duration, (expr_count - 1) as f64 / runtime_duration.as_secs_f64());
    
    // Batch constraint posting
    let start = Instant::now();
    let mut model = Model::default();
    let vars: Vec<_> = (0..expr_count).map(|_| model.int(1, 100)).collect();
    
    let constraints: Vec<_> = vars.iter().map(|&var| var.gt(25)).collect();
    model.postall(constraints);
    
    let batch_duration = start.elapsed();
    
    println!("  Batch posting ({} constraints): {:?} ({:.2} constraints/sec)", 
        expr_count, batch_duration, expr_count as f64 / batch_duration.as_secs_f64());
}