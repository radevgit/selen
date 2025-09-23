//! Performance validation benchmark for vec! optimization
//! 
//! This benchmark measures the performance impact of our Phase 1 optimizations:
//! - vec! macro replacement with Vec::with_capacity()
//! - HashMap capacity hints 
//! - SparseSet preallocation

use selen::prelude::*;
use std::time::Instant;

fn main() {
    println!("🚀 Performance Validation: Phase 1 Optimizations");
    println!("=================================================\n");

    // Benchmark 1: Constraint macro performance 
    benchmark_constraint_macros();
    
    // Benchmark 2: AllDifferent constraint performance
    benchmark_alldiff_constraints();
    
    // Benchmark 3: Model creation performance
    benchmark_model_creation();
    
    // Benchmark 4: Complex solving performance
    benchmark_complex_solving();
    
    println!("\n✅ Performance validation complete!");
}

fn benchmark_constraint_macros() {
    println!("📊 Benchmark 1: Constraint Macro Performance");
    println!("Testing global constraints that use vec! internally...");
    
    let start = Instant::now();
    let mut total_constraints = 0;
    
    // Create many models with global constraints
    for _i in 0..1000 {
        let mut m = Model::default();
        let vars: Vec<_> = (0..10).map(|_| m.int(1, 10)).collect();
        
        // These macros now use optimized allocation patterns
        post!(m, alldiff([vars[0], vars[1], vars[2], vars[3]]));
        post!(m, allequal([vars[4], vars[5], vars[6]]));
        post!(m, element([vars[0], vars[1], vars[2]], vars[7], vars[8]));
        
        total_constraints += 3;
    }
    
    let duration = start.elapsed();
    println!("  Created {} global constraints in {:.2}ms", total_constraints, duration.as_secs_f64() * 1000.0);
    println!("  Average: {:.4}ms per constraint\n", (duration.as_secs_f64() * 1000.0) / total_constraints as f64);
}

fn benchmark_alldiff_constraints() {
    println!("📊 Benchmark 2: AllDifferent Constraint Performance");
    println!("Testing AllDifferent with optimized Vec allocations...");
    
    let start = Instant::now();
    let mut total_propagations = 0;
    
    // Create models with various AllDifferent constraint sizes
    for size in [5, 10, 15, 20] {
        for _trial in 0..100 {
            let mut m = Model::default();
            let vars: Vec<_> = (0..size).map(|_| m.int(1, size as i32)).collect();
            
            // AllDifferent now uses optimized Vec::with_capacity
            post!(m, alldiff(vars.clone()));
            
            // Trigger some propagation
            let _solutions = m.solve_all();
            total_propagations += 1;
        }
    }
    
    let duration = start.elapsed();
    println!("  Completed {} AllDifferent propagations in {:.2}ms", total_propagations, duration.as_secs_f64() * 1000.0);
    println!("  Average: {:.4}ms per propagation\n", (duration.as_secs_f64() * 1000.0) / total_propagations as f64);
}

fn benchmark_model_creation() {
    println!("📊 Benchmark 3: Model Creation Performance");
    println!("Testing variable and constraint creation with optimized allocations...");
    
    let start = Instant::now();
    let mut total_variables = 0;
    
    // Test model creation with many variables
    for _i in 0..100 {
        let mut m = Model::default();
        
        // Create many variables (tests SparseSet optimizations)
        for _j in 0..100 {
            let _var = m.int(1, 100);
            total_variables += 1;
        }
        
        // Create many constraints (tests HashMap capacity optimizations)
        let vars: Vec<_> = (0..10).map(|_| m.int(1, 10)).collect();
        for k in 0..10 {
            post!(m, vars[k % vars.len()] < vars[(k + 1) % vars.len()]);
        }
    }
    
    let duration = start.elapsed();
    println!("  Created {} variables in {:.2}ms", total_variables, duration.as_secs_f64() * 1000.0);
    println!("  Average: {:.4}ms per 100 variables\n", (duration.as_secs_f64() * 1000.0) / (total_variables as f64 / 100.0));
}

fn benchmark_complex_solving() {
    println!("📊 Benchmark 4: Complex Problem Solving");
    println!("Testing end-to-end solving performance...");
    
    let start = Instant::now();
    let mut total_solutions = 0;
    
    // Create and solve several N-Queens problems (uses AllDifferent heavily)
    for n in [6, 7, 8] {
        let mut m = Model::default();
        let queens: Vec<_> = (0..n).map(|_| m.int(1, n as i32)).collect();
        
        // Row constraints (AllDifferent)
        post!(m, alldiff(queens.clone()));
        
        // Diagonal constraints
        let diag1: Vec<_> = queens.iter().enumerate().map(|(i, &q)| q + int(i as i32)).collect();
        let diag2: Vec<_> = queens.iter().enumerate().map(|(i, &q)| q - int(i as i32)).collect();
        
        post!(m, alldiff(diag1));
        post!(m, alldiff(diag2));
        
        let solutions = m.solve_all();
        total_solutions += solutions.len();
    }
    
    let duration = start.elapsed();
    println!("  Found {} solutions in {:.2}ms", total_solutions, duration.as_secs_f64() * 1000.0);
    println!("  Average solving time: {:.2}ms per N-Queens problem", duration.as_secs_f64() * 1000.0 / 3.0);
}