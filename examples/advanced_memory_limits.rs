use cspsolver::prelude::*;
use std::time::Instant;

fn get_memory_usage() -> usize {
    // Simple memory usage estimation (not perfect but gives an idea)
    // In a real implementation, this would use system calls
    std::process::Command::new("ps")
        .args(&["-o", "rss=", "-p", &std::process::id().to_string()])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .and_then(|s| s.trim().parse::<usize>().ok())
        .unwrap_or(0) / 1024 // Convert KB to MB
}

fn main() {
    println!("🧪 Memory Usage During Variable Creation Test");
    println!("=============================================");
    
    let initial_memory = get_memory_usage();
    println!("Initial memory usage: {} MB", initial_memory);
    
    // Test: Monitor memory during variable creation
    println!("\nTest: Creating variables and monitoring memory...");
    let config = SolverConfig::default()
        .with_timeout_seconds(10)
        .with_max_memory_mb(512);  // 512MB limit
    
    let mut m = Model::with_config(config);
    
    let mut variables = Vec::new();
    
    for i in 0..100 {  // Start with smaller number
        let current_memory = get_memory_usage();
        let memory_increase = current_memory.saturating_sub(initial_memory);
        
        if i % 10 == 0 {
            println!("  Step {}: Created {} variables, Memory: +{} MB (total: {} MB)", 
                     i, variables.len(), memory_increase, current_memory);
            
            // Early termination if memory is growing too fast
            if memory_increase > 200 {
                println!("  ⚠️  Memory usage growing too fast, stopping variable creation");
                break;
            }
        }
        
        // Create variables and constraints
        let x = m.float(0.0, 100.0);
        let y = m.float(0.0, 100.0);
        let z = m.add(x, y);
        
        // Add constraint
        m.new(z.gt(float(50.0)));
        
        // Store variables to prevent them from being optimized away
        variables.push((x, y, z));
        
        // Check if we've hit practical memory limits before solver even starts
        if memory_increase > 100 {
            println!("  ⚠️  Reached practical memory limit ({} MB increase), testing solve...", memory_increase);
            break;
        }
    }
    
    let pre_solve_memory = get_memory_usage();
    println!("\nPre-solve memory: {} MB (+{} MB from start)", 
             pre_solve_memory, pre_solve_memory.saturating_sub(initial_memory));
    
    // Now try to solve
    println!("Attempting to solve with {} variables...", variables.len() * 3);
    let start = Instant::now();
    let result = m.solve();
    let duration = start.elapsed();
    
    let post_solve_memory = get_memory_usage();
    
    match result {
        Ok(_) => {
            println!("  ✅ Solved successfully in {:.3}s", duration.as_secs_f64());
            println!("  Memory after solve: {} MB", post_solve_memory);
        },
        Err(e) => {
            println!("  ❌ Failed: {} in {:.3}s", e, duration.as_secs_f64());
            println!("  Memory when failed: {} MB", post_solve_memory);
            
            // Check if it's a memory limit error
            if format!("{}", e).contains("Memory") || format!("{}", e).contains("memory") {
                println!("  ✅ Memory limit working correctly!");
            } else {
                println!("  ❓ Failed for different reason - memory limits may not be working");
            }
        }
    }
    
    println!("\n🔍 Analysis:");
    println!("- Variable creation used: {} MB", pre_solve_memory.saturating_sub(initial_memory));
    println!("- Solving used additional: {} MB", post_solve_memory.saturating_sub(pre_solve_memory));
    println!("- Total memory increase: {} MB", post_solve_memory.saturating_sub(initial_memory));
    
    if pre_solve_memory.saturating_sub(initial_memory) > 50 {
        println!("⚠️  WARNING: Variable creation alone used significant memory!");
        println!("   This suggests we need memory limits during model building, not just solving.");
    }
}