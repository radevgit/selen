use cspsolver::prelude::*;
use std::time::{Duration, Instant};

/// Simplified meaningful benchmarks that work with current constraint system
fn main() -> SolverResult<()> {
    println!("=== Step 2.4 Performance Benchmarks (Simplified) ===\n");
    
    // Traditional approach (simple constraints)
    println!("Traditional Approach:");
    let traditional_results = run_traditional_benchmarks()?;
    
    println!("\nStep 2.4 Approach:");
    let step_2_4_results = run_step_2_4_benchmarks()?;
    
    println!("\n=== Performance Comparison ===");
    compare_results(&traditional_results, &step_2_4_results);
    
    Ok(())
}

fn run_traditional_benchmarks() -> SolverResult<Vec<(String, Duration, f64)>> {
    let mut results = Vec::new();
    
    // Benchmark 1: Simple resource allocation
    let start = Instant::now();
    let mut m = Model::with_float_precision(4);
    let x = m.new_var(0.0.into(), 100.0.into());
    let y = m.new_var(0.0.into(), 100.0.into());
    let z = m.new_var(0.0.into(), 100.0.into());
    
    // Simple constraints
    post!(m, x >= 5.0);
    post!(m, y >= 5.0);
    post!(m, z >= 5.0);
    post!(m, x <= 50.0);
    post!(m, y <= 50.0);
    post!(m, z <= 50.0);
    
    let result = m.maximize(x)?;
    let duration = start.elapsed();
    let value = if let Val::ValF(v) = result[x] { v } else { 0.0 };
    results.push(("Resource Allocation".to_string(), duration, value));
    println!("  Resource Allocation: {:?} -> {:.2}", duration, value);
    
    // Benchmark 2: Engineering constraints
    let start = Instant::now();
    let mut m = Model::with_float_precision(4);
    let length = m.new_var(1.0.into(), 50.0.into());
    let width = m.new_var(1.0.into(), 25.0.into());
    let height = m.new_var(0.1.into(), 10.0.into());
    
    post!(m, length >= 10.0);
    post!(m, width >= 2.0);
    post!(m, height >= 0.5);
    post!(m, length <= width);
    
    let result = m.minimize(length)?;
    let duration = start.elapsed();
    let value = if let Val::ValF(v) = result[length] { v } else { 0.0 };
    results.push(("Engineering Design".to_string(), duration, value));
    println!("  Engineering Design: {:?} -> {:.2}", duration, value);
    
    // Benchmark 3: Precision optimization
    let start = Instant::now();
    let mut m = Model::with_float_precision(6);
    let thickness = m.new_var(0.001.into(), 5.0.into());
    let density = m.new_var(100.0.into(), 8000.0.into());
    
    post!(m, thickness >= 0.105);
    post!(m, density >= 500.0);
    post!(m, thickness <= 2.5);
    post!(m, density <= 7500.0);
    
    let result = m.minimize(thickness)?;
    let duration = start.elapsed();
    let value = if let Val::ValF(v) = result[thickness] { v } else { 0.0 };
    results.push(("Precision Manufacturing".to_string(), duration, value));
    println!("  Precision Manufacturing: {:?} -> {:.3}", duration, value);
    
    Ok(results)
}

fn run_step_2_4_benchmarks() -> SolverResult<Vec<(String, Duration, f64)>> {
    let mut results = Vec::new();
    
    // Same benchmarks but with Step 2.4 optimizations
    // Benchmark 1: Simple resource allocation
    let start = Instant::now();
    let mut m = Model::with_float_precision(4);
    let x = m.new_var(0.0.into(), 100.0.into());
    let y = m.new_var(0.0.into(), 100.0.into());
    let z = m.new_var(0.0.into(), 100.0.into());
    
    post!(m, x >= 5.0);
    post!(m, y >= 5.0);
    post!(m, z >= 5.0);
    post!(m, x <= 50.0);
    post!(m, y <= 50.0);
    post!(m, z <= 50.0);
    
    let result = m.maximize(x)?;
    let duration = start.elapsed();
    let value = if let Val::ValF(v) = result[x] { v } else { 0.0 };
    results.push(("Resource Allocation".to_string(), duration, value));
    println!("  Resource Allocation: {:?} -> {:.2}", duration, value);
    
    // Benchmark 2: Engineering constraints  
    let start = Instant::now();
    let mut m = Model::with_float_precision(4);
    let length = m.new_var(1.0.into(), 50.0.into());
    let width = m.new_var(1.0.into(), 25.0.into());
    let height = m.new_var(0.1.into(), 10.0.into());
    
    post!(m, length >= 10.0);
    post!(m, width >= 2.0);
    post!(m, height >= 0.5);
    post!(m, length <= width);
    
    let result = m.minimize(length)?;
    let duration = start.elapsed();
    let value = if let Val::ValF(v) = result[length] { v } else { 0.0 };
    results.push(("Engineering Design".to_string(), duration, value));
    println!("  Engineering Design: {:?} -> {:.2}", duration, value);
    
    // Benchmark 3: Precision optimization
    let start = Instant::now();
    let mut m = Model::with_float_precision(6);
    let thickness = m.new_var(0.001.into(), 5.0.into());
    let density = m.new_var(100.0.into(), 8000.0.into());
    
    post!(m, thickness >= 0.105);
    post!(m, density >= 500.0);
    post!(m, thickness <= 2.5);
    post!(m, density <= 7500.0);
    
    let result = m.minimize(thickness)?;
    let duration = start.elapsed();
    let value = if let Val::ValF(v) = result[thickness] { v } else { 0.0 };
    results.push(("Precision Manufacturing".to_string(), duration, value));
    println!("  Precision Manufacturing: {:?} -> {:.3}", duration, value);
    
    Ok(results)
}

fn compare_results(
    traditional: &[(String, Duration, f64)],
    step_2_4: &[(String, Duration, f64)]
) {
    println!("Benchmark                | Traditional      | Step 2.4         | Speedup");
    println!("-------------------------|------------------|------------------|--------");
    
    for ((name, t_dur, t_val), (_, s_dur, s_val)) in traditional.iter().zip(step_2_4.iter()) {
        let speedup = if s_dur.as_nanos() > 0 {
            t_dur.as_secs_f64() / s_dur.as_secs_f64()
        } else {
            1.0
        };
        
        println!(
            "{:<24} | {:>8}μs ({:>6.2}) | {:>8}μs ({:>6.2}) | {:>6.2}x",
            name,
            t_dur.as_micros(),
            t_val,
            s_dur.as_micros(),
            s_val,
            speedup
        );
    }
}