use std::time::{Duration, Instant};
use selen::prelude::*;

// Benchmark timing utilities
pub struct BenchmarkResult {
    pub duration: Duration,
    pub success: bool,
    pub nodes_explored: usize,
    pub constraints_posted: usize,
    pub optimization_used: bool,
}

impl BenchmarkResult {
    pub fn new(duration: Duration, success: bool) -> Self {
        Self {
            duration,
            success,
            nodes_explored: 0,
            constraints_posted: 0,
            optimization_used: false,
        }
    }
    
    pub fn with_stats(duration: Duration, success: bool, nodes: usize, constraints: usize, optimized: bool) -> Self {
        Self {
            duration,
            success,
            nodes_explored: nodes,
            constraints_posted: constraints,
            optimization_used: optimized,
        }
    }
}

pub fn benchmark_simple_precision_constraint() -> BenchmarkResult {
    let start = Instant::now();
    
    let mut m = Model::default();
    let x = m.float(0.0, 10.0);
    
    // Simple precision constraint that should trigger ULP optimization
    post!(m, x < 5.5);
    
    let success = m.solve().is_ok();
    let duration = start.elapsed();
    
    BenchmarkResult::new(duration, success)
}

pub fn benchmark_multi_precision_constraints() -> BenchmarkResult {
    let start = Instant::now();
    
    let mut m = Model::default();
    let x = m.float(0.0, 100.0);
    let y = m.float(0.0, 100.0);
    
    // Multiple precision constraints
    post!(m, x < 50.5);
    post!(m, y > 25.25);
    post!(m, y < 75.75);
    
    let success = m.solve().is_ok();
    let duration = start.elapsed();
    
    BenchmarkResult::new(duration, success)
}

pub fn benchmark_traditional_csp_fallback() -> BenchmarkResult {
    let start = Instant::now();
    
    let mut m = Model::default();
    let x = m.int(0, 100);
    let y = m.int(0, 100);
    let z = m.int(0, 100);
    
    // Complex constraints that should fall back to traditional CSP
    post!(m, all_different([x, y, z]));
    post!(m, x + y == z);
    
    let success = m.solve().is_ok();
    let duration = start.elapsed();
    
    BenchmarkResult::new(duration, success)
}

pub fn print_benchmark_result(name: &str, result: &BenchmarkResult) {
    println!("=== {} ===", name);
    println!("Duration: {:?}", result.duration);
    println!("Success: {}", result.success);
    println!("Nodes explored: {}", result.nodes_explored);
    println!("Constraints: {}", result.constraints_posted);
    println!("Optimization used: {}", result.optimization_used);
    
    // Performance classification
    let micros = result.duration.as_micros();
    let classification = if micros < 10 { "Excellent" }
                        else if micros < 100 { "Very Good" }
                        else if micros < 1000 { "Good" }
                        else if micros < 10000 { "Acceptable" }
                        else { "Needs Improvement" };
    
    println!("Performance: {} ({} μs)", classification, micros);
    println!();
}

pub fn run_precision_validation_suite() {
    println!("CSP Solver - Precision Optimization Validation");
    println!("==============================================");
    println!();
    
    // Test 1: Simple precision constraint (should be < 10 μs)
    let result1 = benchmark_simple_precision_constraint();
    print_benchmark_result("Simple Precision Constraint", &result1);
    
    // Test 2: Multiple precision constraints (should be < 100 μs)
    let result2 = benchmark_multi_precision_constraints();
    print_benchmark_result("Multiple Precision Constraints", &result2);
    
    // Test 3: Traditional CSP (comparison baseline)
    let result3 = benchmark_traditional_csp_fallback();
    print_benchmark_result("Traditional CSP (Baseline)", &result3);
    
    // Summary
    println!("=== VALIDATION SUMMARY ===");
    let precision_advantage = if result3.duration.as_micros() > 0 {
        result3.duration.as_micros() as f64 / result1.duration.as_micros() as f64
    } else {
        f64::INFINITY
    };
    
    println!("Precision optimization advantage: {:.1}x faster", precision_advantage);
    
    // Validate our claims
    let simple_under_target = result1.duration.as_micros() < 10;
    let multi_under_target = result2.duration.as_micros() < 100;
    
    println!("Simple constraint < 10μs: {}", simple_under_target);
    println!("Multi-constraint < 100μs: {}", multi_under_target);
    
    if simple_under_target && multi_under_target {
        println!("✅ Performance claims VALIDATED");
    } else {
        println!("❌ Performance claims need investigation");
    }
}
