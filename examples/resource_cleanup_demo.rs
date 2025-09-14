use cspsolver::prelude::*;
use std::{sync::Arc, sync::atomic::{AtomicBool, Ordering}, time::Duration, thread};

fn main() {
    println!("Resource Cleanup Demo");
    println!("=====================\n");

    // Demo 1: Timeout with cleanup
    demo_timeout_cleanup();
    
    // Demo 2: Memory limit with cleanup  
    demo_memory_limit_cleanup();
    
    // Demo 3: Manual interruption with cleanup
    demo_manual_interruption_cleanup();
}

fn demo_timeout_cleanup() {
    println!("Demo 1: Timeout with Resource Cleanup");
    println!("--------------------------------------");
    
    let cleanup_called = Arc::new(AtomicBool::new(false));
    let cleanup_flag = cleanup_called.clone();
    
    // Create a model that will take a long time to solve
    let mut model = Model::default();
    let x = model.int(1, 1000);
    let y = model.int(1, 1000);
    let z = model.int(1, 1000);
    let sum_xyz = model.sum(&[x, y, z]);
    post!(model, sum_xyz == 1500);
    post!(model, x * y == z);
    
    // Configure with short timeout (convert to seconds)
    let config = SolverConfig::default()
        .with_timeout_seconds(1); // 1 second timeout
    
    // Create engine and register cleanup callback
    let mut engine = model.engine().with_config(config);
    engine.register_cleanup(Box::new(move || {
        cleanup_flag.store(true, Ordering::SeqCst);
        println!("  ✓ Cleanup callback executed for timeout!");
    }));
    
    println!("  Starting search with 1 second timeout...");
    let start = std::time::Instant::now();
    let result = engine.solve_any();
    let elapsed = start.elapsed();
    
    match result {
        Some(_) => println!("  Unexpectedly found solution in {:?}", elapsed),
        None => {
            println!("  Search timed out after {:?}", elapsed);
            if cleanup_called.load(Ordering::SeqCst) {
                println!("  ✓ Resource cleanup was properly triggered!");
            } else {
                println!("  ✗ Resource cleanup was NOT triggered!");
            }
        }
    }
    println!();
}

fn demo_memory_limit_cleanup() {
    println!("Demo 2: Memory Limit with Resource Cleanup");
    println!("-------------------------------------------");
    
    let cleanup_called = Arc::new(AtomicBool::new(false));
    let cleanup_flag = cleanup_called.clone();
    
    // Create a model that should trigger memory limits
    let mut model = Model::default();
    let x1 = model.int(1, 100);
    let x2 = model.int(1, 100);
    let x3 = model.int(1, 100);
    let x4 = model.int(1, 100);
    let sum_vars = model.sum(&[x1, x2, x3, x4]);
    post!(model, sum_vars == 200);
    
    // Configure with very low memory limit
    let config = SolverConfig::default()
        .with_max_memory_mb(1); // Very low limit to trigger quickly
    
    // Create engine and register cleanup callback
    let mut engine = model.engine().with_config(config);
    engine.register_cleanup(Box::new(move || {
        cleanup_flag.store(true, Ordering::SeqCst);
        println!("  ✓ Cleanup callback executed for memory limit!");
    }));
    
    println!("  Starting search with 1MB memory limit...");
    let start = std::time::Instant::now();
    let result = engine.solve_any();
    let elapsed = start.elapsed();
    
    match result {
        Some(_) => println!("  Found solution in {:?}", elapsed),
        None => {
            println!("  Search stopped after {:?}", elapsed);
            if cleanup_called.load(Ordering::SeqCst) {
                println!("  ✓ Resource cleanup was properly triggered!");
            } else {
                println!("  ✗ Resource cleanup was NOT triggered!");
            }
        }
    }
    println!();
}

fn demo_manual_interruption_cleanup() {
    println!("Demo 3: Manual Interruption with Resource Cleanup");
    println!("--------------------------------------------------");
    
    let cleanup_called = Arc::new(AtomicBool::new(false));
    let cleanup_flag = cleanup_called.clone();
    
    // Create a simple model
    let mut model = Model::default();
    let x = model.int(1, 10);
    let y = model.int(1, 10);
    let sum_xy = model.sum(&[x, y]);
    post!(model, sum_xy == 15);
    
    // No time/memory limits - we'll drop the engine manually
    let config = SolverConfig::default();
    
    println!("  Creating engine and registering cleanup...");
    {
        let mut engine = model.engine().with_config(config);
        engine.register_cleanup(Box::new(move || {
            cleanup_flag.store(true, Ordering::SeqCst);
            println!("  ✓ Cleanup callback executed when engine was dropped!");
        }));
        
        println!("  Engine created with cleanup registered");
        println!("  Dropping engine (simulating manual interruption)...");
        // Engine will be dropped here, triggering cleanup via Drop trait
    }
    
    // Give cleanup a moment to execute
    thread::sleep(Duration::from_millis(10));
    
    if cleanup_called.load(Ordering::SeqCst) {
        println!("  ✓ Resource cleanup was properly triggered on drop!");
    } else {
        println!("  ✗ Resource cleanup was NOT triggered on drop!");
    }
    println!();
}