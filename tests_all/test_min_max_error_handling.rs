use selen::prelude::*;

#[test]
fn main() {
    println!("Testing min/max functions with empty input...");
    
    let mut model = Model::default();
    
    // Test min with empty variable list
    println!("Testing min with empty list:");
    match model.min(&[]) {
        Ok(_) => println!("❌ min() succeeded (unexpected)"),
        Err(e) => println!("✅ min() failed as expected: {}", e),
    }
    
    // Test max with empty variable list
    println!("Testing max with empty list:");
    match model.max(&[]) {
        Ok(_) => println!("❌ max() succeeded (unexpected)"),
        Err(e) => println!("✅ max() failed as expected: {}", e),
    }
    
    // Test normal operation with variables
    println!("\nTesting normal operation:");
    let x = model.int(1, 10);
    let y = model.int(5, 15);
    let z = model.int(3, 8);
    
    match model.min(&[x, y, z]) {
        Ok(min_var) => println!("✅ min() with variables succeeded: {:?}", min_var),
        Err(e) => println!("❌ min() with variables failed: {}", e),
    }
    
    match model.max(&[x, y, z]) {
        Ok(max_var) => println!("✅ max() with variables succeeded: {:?}", max_var),
        Err(e) => println!("❌ max() with variables failed: {}", e),
    }
}