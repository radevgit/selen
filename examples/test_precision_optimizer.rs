use selen::prelude::*;
use std::time::Instant;

fn main() {
    println!("Testing precision optimizer behavior...\n");
    
    // Test 1: Single variable with constraint (should use precision optimizer)
    println!("=== Test 1: Single variable optimization ===");
    let mut model1 = Model::default();
    let x = model1.float(-1e6, 1e6);
    model1.new(x.ge(-5.0));
    
    let start = Instant::now();
    let result1 = model1.maximize(x);
    let elapsed = start.elapsed();
    
    match result1 {
        Ok(solution) => {
            println!("✓ SUCCESS in {:?}", elapsed);
            println!("  x = {}", solution.get_float(x));
            println!("  Stats: {:?}", solution.stats());
        }
        Err(e) => {
            println!("✗ FAILED in {:?}: {:?}", elapsed, e);
        }
    }
    
    // Test 2: Two variables with addition constraint
    println!("\n=== Test 2: Two variables with addition (CURRENT FAILING TEST) ===");
    let mut model2 = Model::default();
    let a = model2.float(-1e6, 1e6);
    let b = model2.float(-1e6, 1e6);
    let sum_var = model2.float(-2e6, 2e6);
    model2.new(a.add(b).eq(sum_var));
    model2.new(sum_var.le(0.0));
    
    println!("Starting maximize...");
    let start = Instant::now();
    let result2 = model2.maximize(a);
    let elapsed = start.elapsed();
    
    match result2 {
        Ok(solution) => {
            println!("✓ SUCCESS in {:?}", elapsed);
            println!("  a = {}", solution.get_float(a));
            println!("  b = {}", solution.get_float(b));
            println!("  sum = {}", solution.get_float(sum_var));
            println!("  Stats: {:?}", solution.stats());
        }
        Err(e) => {
            println!("✗ FAILED in {:?}: {:?}", elapsed, e);
        }
    }
    
    // Test 3: Same as test 2 but with smaller domains
    println!("\n=== Test 3: Two variables with smaller domains ===");
    let mut model3 = Model::default();
    let c = model3.float(-100.0, 100.0);
    let d = model3.float(-100.0, 100.0);
    let sum_var2 = model3.float(-200.0, 200.0);
    model3.new(c.add(d).eq(sum_var2));
    model3.new(sum_var2.le(0.0));
    
    let start = Instant::now();
    let result3 = model3.maximize(c);
    let elapsed = start.elapsed();
    
    match result3 {
        Ok(solution) => {
            println!("✓ SUCCESS in {:?}", elapsed);
            println!("  c = {}", solution.get_float(c));
            println!("  d = {}", solution.get_float(d));
            println!("  sum = {}", solution.get_float(sum_var2));
            println!("  Stats: {:?}", solution.stats());
        }
        Err(e) => {
            println!("✗ FAILED in {:?}: {:?}", elapsed, e);
        }
    }
}
