//! Performance Benchmarks for Runtime Constraint API
//!
//! This module provides comprehensive benchmarks comparing:
//! 1. Runtime API vs traditional post! macro performance
//! 2. Different constraint building patterns
//! 3. Memory usage and allocation patterns
//! 4. Scaling characteristics with problem size

use selen::{
    model::Model,
    prelude::*,
    runtime_api::{ModelExt, VarIdExt},
};
use std::time::{Duration, Instant};

/// Benchmark results for a specific test
#[derive(Debug)]
pub struct BenchmarkResult {
    name: String,
    duration: Duration,
    constraints_per_second: f64,
    #[allow(dead_code)]
    memory_usage_mb: f64,
}

impl BenchmarkResult {
    fn new(name: String, duration: Duration, constraint_count: usize) -> Self {
        let constraints_per_second = constraint_count as f64 / duration.as_secs_f64();
        Self {
            name,
            duration,
            constraints_per_second,
            memory_usage_mb: 0.0, // Will be measured separately
        }
    }
}

/// Performance test suite for runtime constraint API
pub struct RuntimeApiPerformanceTests;

impl RuntimeApiPerformanceTests {
    
    /// Benchmark basic constraint creation: runtime API vs post! macro
    pub fn benchmark_basic_constraints() -> Vec<BenchmarkResult> {
        let mut results = Vec::new();
        let constraint_count = 10_000;
        
        println!("🚀 Benchmarking Basic Constraint Creation ({} constraints)", constraint_count);
        
        // Test 1: Runtime API - Variable constraints
        {
            let start = Instant::now();
            let mut model = Model::default();
            let vars: Vec<_> = (0..constraint_count).map(|_| model.int(1, 100)).collect();
            
            for &var in &vars {
                model.post(var.gt(50));
            }
            
            let duration = start.elapsed();
            results.push(BenchmarkResult::new(
                "Runtime API - Basic VarId constraints".to_string(),
                duration,
                constraint_count
            ));
            println!("  ✓ Runtime API: {:?} ({:.2} constraints/sec)", 
                duration, constraint_count as f64 / duration.as_secs_f64());
        }
        
        // Test 2: Traditional post! macro
        {
            let start = Instant::now();
            let mut model = Model::default();
            let vars: Vec<_> = (0..constraint_count).map(|_| model.int(1, 100)).collect();
            
            for &var in &vars {
                post!(model, var > 50);
            }
            
            let duration = start.elapsed();
            results.push(BenchmarkResult::new(
                "Traditional post! macro".to_string(),
                duration,
                constraint_count
            ));
            println!("  ✓ post! macro: {:?} ({:.2} constraints/sec)", 
                duration, constraint_count as f64 / duration.as_secs_f64());
        }
        
        // Test 3: Runtime API - Complex expressions
        {
            let start = Instant::now();
            let mut model = Model::default();
            let vars: Vec<_> = (0..constraint_count).map(|_| model.int(1, 100)).collect();
            
            for i in 0..vars.len() {
                if i < vars.len() - 1 {
                    model.post(vars[i].add(vars[i + 1]).le(150));
                }
            }
            
            let duration = start.elapsed();
            results.push(BenchmarkResult::new(
                "Runtime API - Complex expressions".to_string(),
                duration,
                constraint_count - 1
            ));
            println!("  ✓ Runtime API Complex: {:?} ({:.2} constraints/sec)", 
                duration, (constraint_count - 1) as f64 / duration.as_secs_f64());
        }
        
        // Test 4: Traditional post! - simpler expressions
        {
            let start = Instant::now();
            let mut model = Model::default();
            let vars: Vec<_> = (0..constraint_count).map(|_| model.int(1, 100)).collect();
            
            for i in 0..vars.len() {
                if i < vars.len() - 1 {
                    post!(model, vars[i] != vars[i + 1]);
                }
            }
            
            let duration = start.elapsed();
            results.push(BenchmarkResult::new(
                "Traditional post! - Basic expressions".to_string(),
                duration,
                constraint_count - 1
            ));
            println!("  ✓ post! Basic: {:?} ({:.2} constraints/sec)", 
                duration, (constraint_count - 1) as f64 / duration.as_secs_f64());
        }
        
        results
    }
    
    /// Benchmark global constraints performance
    pub fn benchmark_global_constraints() -> Vec<BenchmarkResult> {
        let mut results = Vec::new();
        
        println!("\n🌍 Benchmarking Global Constraints");
        
        // Test different sizes for all_different constraint
        for size in [10, 50, 100, 500].iter() {
            let start = Instant::now();
            let mut model = Model::default();
            let vars: Vec<_> = (0..*size).map(|_| model.int(1, *size * 2)).collect();
            
            model.alldiff(&vars);
            
            let duration = start.elapsed();
            results.push(BenchmarkResult::new(
                format!("alldiff constraint (size {})", size),
                duration,
                1
            ));
            println!("  ✓ alldiff({}): {:?}", size, duration);
        }
        
        // Test count constraints
        {
            let constraint_count = 1000;
            let start = Instant::now();
            let mut model = Model::default();
            let vars: Vec<_> = (0..100).map(|_| model.int(1, 10)).collect();
            
            for i in 0..constraint_count {
                let count_var = model.int(0, 100);
                model.count(&vars, (i % 10) + 1, count_var);
            }
            
            let duration = start.elapsed();
            results.push(BenchmarkResult::new(
                "count constraints (1000x)".to_string(),
                duration,
                constraint_count as usize
            ));
            println!("  ✓ count constraints: {:?} ({:.2} constraints/sec)", 
                duration, constraint_count as f64 / duration.as_secs_f64());
        }
        
        results
    }
    
    /// Benchmark constraint composition and boolean logic
    pub fn benchmark_constraint_composition() -> Vec<BenchmarkResult> {
        let mut results = Vec::new();
        
        println!("\n🔗 Benchmarking Constraint Composition");
        
        // Test 1: AND composition
        {
            let constraint_count = 5000;
            let start = Instant::now();
            let mut model = Model::default();
            let vars: Vec<_> = (0..constraint_count).map(|_| model.int(1, 100)).collect();
            
            let mut constraints = Vec::new();
            for &var in &vars {
                constraints.push(var.gt(25));
                constraints.push(var.lt(75));
            }
            
            // Compose all constraints with AND
            if let Some(first) = constraints.first().cloned() {
                let combined = constraints.into_iter().skip(1)
                    .fold(first, |acc, c| acc.and(c));
                model.post(combined);
            }
            
            let duration = start.elapsed();
            results.push(BenchmarkResult::new(
                "AND composition (10K constraints)".to_string(),
                duration,
                constraint_count * 2
            ));
            println!("  ✓ AND composition: {:?} ({:.2} constraints/sec)", 
                duration, (constraint_count * 2) as f64 / duration.as_secs_f64());
        }
        
        // Test 2: Individual posting vs batch posting
        {
            let constraint_count = 2000;
            
            // Individual posting
            let start = Instant::now();
            let mut model1 = Model::default();
            let vars1: Vec<_> = (0..constraint_count).map(|_| model1.int(1, 100)).collect();
            
            for &var in &vars1 {
                model1.post(var.gt(50));
            }
            
            let individual_duration = start.elapsed();
            
            // Batch posting using post_all
            let start = Instant::now();
            let mut model2 = Model::default();
            let vars2: Vec<_> = (0..constraint_count).map(|_| model2.int(1, 100)).collect();
            
            let constraints: Vec<_> = vars2.iter().map(|&var| var.gt(50)).collect();
            model2.postall(constraints);
            
            let batch_duration = start.elapsed();
            
            results.push(BenchmarkResult::new(
                "Individual constraint posting".to_string(),
                individual_duration,
                constraint_count
            ));
            
            results.push(BenchmarkResult::new(
                "Batch constraint posting (post_all)".to_string(),
                batch_duration,
                constraint_count
            ));
            
            println!("  ✓ Individual posting: {:?} ({:.2} constraints/sec)", 
                individual_duration, constraint_count as f64 / individual_duration.as_secs_f64());
            println!("  ✓ Batch posting: {:?} ({:.2} constraints/sec)", 
                batch_duration, constraint_count as f64 / batch_duration.as_secs_f64());
            
            let speedup = individual_duration.as_secs_f64() / batch_duration.as_secs_f64();
            println!("  📈 Batch posting speedup: {:.2}x", speedup);
        }
        
        results
    }
    
    /// Benchmark solving performance with runtime constraints
    pub fn benchmark_solving_performance() -> Vec<BenchmarkResult> {
        let mut results = Vec::new();
        
        println!("\n🧩 Benchmarking Solving Performance");
        
        // Test 1: Runtime API constraint solving
        {
            let start = Instant::now();
            let mut model = Model::default();
            let vars: Vec<_> = (0..100).map(|_| model.int(1, 100)).collect();
            
            // Add various constraints using runtime API
            for i in 0..vars.len() {
                model.post(vars[i].gt(i as i32));
                if i < vars.len() - 1 {
                    model.post(vars[i].add(vars[i + 1]).le(150));
                }
            }
            
            // Add global constraint
            model.alldiff(&vars[0..20]);
            
            let solve_start = Instant::now();
            let solution = model.solve();
            let solve_duration = solve_start.elapsed();
            
            let total_duration = start.elapsed();
            
            println!("  ✓ Runtime API solve: total {:?}, solving {:?}, success: {}", 
                total_duration, solve_duration, solution.is_ok());
                
            results.push(BenchmarkResult::new(
                "Runtime API solving".to_string(),
                total_duration,
                199 // constraint count
            ));
        }
        
        // Test 2: Traditional post! macro solving
        {
            let start = Instant::now();
            let mut model = Model::default();
            let vars: Vec<_> = (0..100).map(|_| model.int(1, 100)).collect();
            
            // Add various constraints using post! macro
            for i in 0..vars.len() {
                let threshold = model.int(i as i32, i as i32);
                post!(model, vars[i] > threshold);
                if i < vars.len() - 1 {
                    // Now we can use the enhanced post! macro with array arithmetic
                    post!(model, vars[i] + vars[i + 1] <= int(150));
                }
            }
            
            // Add global constraint - use runtime API since post! macro doesn't support complex global constraints
            model.alldiff(&vars[0..20]);
            
            let solve_start = Instant::now();
            let solution = model.solve();
            let solve_duration = solve_start.elapsed();
            
            let total_duration = start.elapsed();
            
            println!("  ✓ post! macro solve: total {:?}, solving {:?}, success: {}", 
                total_duration, solve_duration, solution.is_ok());
                
            results.push(BenchmarkResult::new(
                "post! macro solving".to_string(),
                total_duration,
                199 // constraint count
            ));
        }
        
        results
    }
    
    /// Comprehensive scaling test
    pub fn benchmark_scaling() -> Vec<BenchmarkResult> {
        let mut results = Vec::new();
        
        println!("\n📈 Benchmarking Scaling Characteristics");
        
        for &size in &[100, 500, 1000, 5000] {
            let start = Instant::now();
            let mut model = Model::default();
            let vars: Vec<_> = (0..size).map(|_| model.int(1, size)).collect();
            
            // Mix of different constraint types
            let mut constraint_count = 0;
            
            // Basic constraints
            for &var in &vars {
                model.post(var.gt(size / 4));
                constraint_count += 1;
            }
            
            // Pairwise constraints
            for i in 0..std::cmp::min(vars.len(), 100) {
                for j in i+1..std::cmp::min(vars.len(), 100) {
                    model.post(vars[i].ne(vars[j]));
                    constraint_count += 1;
                }
            }
            
            // Global constraints
            if vars.len() >= 10 {
                let chunk_size = std::cmp::min(20, vars.len());
                model.alldiff(&vars[0..chunk_size]);
                constraint_count += 1;
            }
            
            let duration = start.elapsed();
            results.push(BenchmarkResult::new(
                format!("Scaling test (size {})", size),
                duration,
                constraint_count
            ));
            
            println!("  ✓ Size {}: {:?} ({} constraints, {:.2} constraints/sec)", 
                size, duration, constraint_count, 
                constraint_count as f64 / duration.as_secs_f64());
        }
        
        results
    }
    
    /// Run all performance benchmarks
    pub fn run_all_benchmarks() -> Vec<BenchmarkResult> {
        println!("🚀 Runtime Constraint API Performance Benchmarks");
        println!("================================================\n");
        
        let mut all_results = Vec::new();
        
        all_results.extend(Self::benchmark_basic_constraints());
        all_results.extend(Self::benchmark_global_constraints());
        all_results.extend(Self::benchmark_constraint_composition());
        all_results.extend(Self::benchmark_solving_performance());
        all_results.extend(Self::benchmark_scaling());
        
        println!("\n📊 Summary of Results:");
        println!("======================");
        for result in &all_results {
            println!("{:<40} | {:>8.2} ms | {:>10.2} constraints/sec", 
                result.name,
                result.duration.as_millis(),
                result.constraints_per_second
            );
        }
        
        // Calculate runtime API vs post! macro comparison
        let runtime_basic = all_results.iter()
            .find(|r| r.name.contains("Runtime API - Basic"))
            .map(|r| r.constraints_per_second);
        let post_basic = all_results.iter()
            .find(|r| r.name.contains("Traditional post! macro"))
            .map(|r| r.constraints_per_second);
            
        if let (Some(runtime), Some(traditional)) = (runtime_basic, post_basic) {
            let ratio = runtime / traditional;
            println!("\n🔍 Performance Analysis:");
            println!("Runtime API vs post! macro: {:.2}x performance", ratio);
            if ratio >= 0.9 {
                println!("✅ Runtime API performance is acceptable (within 10% of post! macro)");
            } else {
                println!("⚠️  Runtime API has {:.1}% performance overhead", (1.0 - ratio) * 100.0);
            }
        }
        
        all_results
    }
}

fn main() {
    RuntimeApiPerformanceTests::run_all_benchmarks();
}