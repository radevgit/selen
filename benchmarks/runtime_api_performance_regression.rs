//! Performance regression tests for Runtime Constraint API
//!
//! This test suite ensures that future changes to the runtime API don't
//! introduce significant performance regressions.

use cspsolver::{
    model::Model,
    prelude::*,
    runtime_api::{ModelExt, VarIdExt},
};
use std::time::{Duration, Instant};

/// Maximum acceptable performance overhead for runtime API vs post! macro
/// Based on actual testing, runtime API has 3-4x overhead for simple constraints
const MAX_ACCEPTABLE_OVERHEAD: f64 = 5.0; // 400% overhead max (realistic threshold)

/// Minimum constraints per second for basic operations
const MIN_CONSTRAINTS_PER_SEC: f64 = 50_000.0; // 50K constraints/sec (realistic minimum)

#[test]
fn test_basic_constraint_performance_regression() {
    let constraint_count = 5000; // Use larger count for more realistic testing
    
    // Test runtime API performance
    let runtime_api_duration = measure_runtime_api_constraints(constraint_count);
    let runtime_api_rate = constraint_count as f64 / runtime_api_duration.as_secs_f64();
    
    // Test post! macro performance
    let post_macro_duration = measure_post_macro_constraints(constraint_count);
    let post_macro_rate = constraint_count as f64 / post_macro_duration.as_secs_f64();
    
    // Calculate overhead ratio
    let overhead_ratio = runtime_api_duration.as_secs_f64() / post_macro_duration.as_secs_f64();
    
    println!("Runtime API: {:.2} constraints/sec", runtime_api_rate);
    println!("post! macro: {:.2} constraints/sec", post_macro_rate);
    println!("Overhead ratio: {:.2}x", overhead_ratio);
    
    // Assert performance requirements
    assert!(runtime_api_rate >= MIN_CONSTRAINTS_PER_SEC, 
        "Runtime API performance too slow: {:.2} < {:.2} constraints/sec", 
        runtime_api_rate, MIN_CONSTRAINTS_PER_SEC);
    
    assert!(overhead_ratio <= MAX_ACCEPTABLE_OVERHEAD, 
        "Runtime API overhead too high: {:.2}x > {:.2}x", 
        overhead_ratio, MAX_ACCEPTABLE_OVERHEAD);
}

#[test]
fn test_batch_posting_performance() {
    let constraint_count = 1000;
    
    // Test individual constraint posting
    let individual_duration = measure_individual_posting(constraint_count);
    let individual_rate = constraint_count as f64 / individual_duration.as_secs_f64();
    
    // Test batch constraint posting
    let batch_duration = measure_batch_posting(constraint_count);
    let batch_rate = constraint_count as f64 / batch_duration.as_secs_f64();
    
    println!("Individual posting: {:.2} constraints/sec", individual_rate);
    println!("Batch posting: {:.2} constraints/sec", batch_rate);
    
    // Batch posting should be at least as fast as individual posting
    assert!(batch_rate >= individual_rate * 0.9, 
        "Batch posting slower than individual: {:.2} < {:.2}", 
        batch_rate, individual_rate * 0.9);
    
    // Both should meet minimum performance requirements
    assert!(batch_rate >= MIN_CONSTRAINTS_PER_SEC * 0.5, 
        "Batch posting too slow: {:.2} < {:.2} constraints/sec", 
        batch_rate, MIN_CONSTRAINTS_PER_SEC * 0.5);
}

#[test]
fn test_expression_building_performance() {
    let expr_count = 500;
    
    let start = Instant::now();
    let mut model = Model::default();
    let vars: Vec<_> = (0..expr_count).map(|_| model.int(1, 100)).collect();
    
    for i in 0..vars.len() {
        if i < vars.len() - 1 {
            // Build complex expression: (vars[i] + vars[i+1]) * 2 <= 200
            model.post(vars[i].add(vars[i + 1]).mul(2).le(200));
        }
    }
    
    let duration = start.elapsed();
    let expr_rate = (expr_count - 1) as f64 / duration.as_secs_f64();
    
    println!("Expression building: {:.2} expr/sec", expr_rate);
    
    // Should build at least 10K expressions per second
    assert!(expr_rate >= 10_000.0, 
        "Expression building too slow: {:.2} < 10000 expr/sec", expr_rate);
}

#[test]
fn test_global_constraints_performance() {
    // Test alldiff performance
    for size in [10, 50, 100] {
        let start = Instant::now();
        let mut model = Model::default();
        let vars: Vec<_> = (0..size).map(|_| model.int(1, size * 2)).collect();
        
        model.alldiff(&vars);
        
        let duration = start.elapsed();
        
        // Should complete within reasonable time
        assert!(duration.as_millis() < 10, 
            "alldiff({}) too slow: {}ms", size, duration.as_millis());
    }
    
    // Test count constraints performance
    let start = Instant::now();
    let mut model = Model::default();
    let vars: Vec<_> = (0..100).map(|_| model.int(1, 10)).collect();
    
    for i in 1..=10 {
        let count_var = model.int(0, 100);
        model.count(&vars, i, count_var);
    }
    
    let duration = start.elapsed();
    
    // Should complete within reasonable time
    assert!(duration.as_millis() < 5, 
        "count constraints too slow: {}ms", duration.as_millis());
}

fn measure_runtime_api_constraints(count: usize) -> Duration {
    let start = Instant::now();
    let mut model = Model::default();
    let vars: Vec<_> = (0..count).map(|_| model.int(1, 100)).collect();
    
    for &var in &vars {
        model.post(var.gt(50));
    }
    
    start.elapsed()
}

fn measure_post_macro_constraints(count: usize) -> Duration {
    let start = Instant::now();
    let mut model = Model::default();
    let vars: Vec<_> = (0..count).map(|_| model.int(1, 100)).collect();
    
    for &var in &vars {
        post!(model, var > 50);
    }
    
    start.elapsed()
}

fn measure_individual_posting(count: usize) -> Duration {
    let start = Instant::now();
    let mut model = Model::default();
    let vars: Vec<_> = (0..count).map(|_| model.int(1, 100)).collect();
    
    for &var in &vars {
        model.post(var.gt(25));
    }
    
    start.elapsed()
}

fn measure_batch_posting(count: usize) -> Duration {
    let start = Instant::now();
    let mut model = Model::default();
    let vars: Vec<_> = (0..count).map(|_| model.int(1, 100)).collect();
    
    let constraints: Vec<_> = vars.iter().map(|&var| var.gt(25)).collect();
    model.postall(constraints);
    
    start.elapsed()
}