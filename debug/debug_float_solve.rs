use selen::prelude::*;
use std::time::Instant;

fn main() {
    println!("Testing simple float constraint solving...");
    
    // Test 1: Very small domain
    println!("\n=== Test 1: Very small domain [-1.0, 1.0] ===");
    let mut model = Model::new();
    let x = model.float(-1.0, 1.0);
    println!("Created float variable with domain: {:?}", x);
    
    // Get domain info
    let domain_info = format!("{:?}", x);
    println!("Domain details: {}", domain_info);
    
    // Add simple constraint
    model.new(x.ge(0.0));
    println!("Added constraint: x >= 0.0");
    
    println!("Starting solve...");
    let start = Instant::now();
    
    // Try to solve with timeout
    let result = model.solve();
    
    let elapsed = start.elapsed();
    println!("Solve finished in {:?}", elapsed);
    
    match result {
        Ok(solution) => {
            println!("SUCCESS! Solution found: {:?}", solution.get_int(x));
            println!("Stats: {:?}", solution.stats());
        }
        Err(e) => {
            println!("FAILED: {:?}", e);
        }
    }
    
    // Test 2: Slightly larger domain
    println!("\n=== Test 2: Larger domain [-10.0, 10.0] ===");
    let mut model2 = Model::new();
    let y = model2.float(-10.0, 10.0);
    println!("Created float variable with domain: {:?}", y);
    
    model2.new(y.ge(-5.0));
    println!("Added constraint: y >= -5.0");
    
    println!("Starting solve...");
    let start = Instant::now();
    
    // Try to solve with explicit timeout of 5 seconds
    let result = model2.solve();
    
    let elapsed = start.elapsed();
    println!("Solve attempt took {:?}", elapsed);
    
    match result {
        Ok(solution) => {
            println!("SUCCESS! Solution found: {:?}", solution.get_int(y));
            println!("Stats: {:?}", solution.stats());
        }
        Err(e) => {
            println!("FAILED: {:?}", e);
        }
    }
}
