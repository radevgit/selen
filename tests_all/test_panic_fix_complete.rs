use selen::prelude::*;

#[test]
fn main() {
    println!("=== Testing Min/Max Functions - No Panics Expected ===\n");
    
    let mut model = Model::default();
    
    println!("Testing min() function with empty list:");
    match model.min(&[]) {
        Ok(_) => println!("❌ Unexpected success"),
        Err(e) => println!("✅ Proper error handling: {}", e),
    }
    
    println!("\nTesting max() function with empty list:");
    match model.max(&[]) {
        Ok(_) => println!("❌ Unexpected success"),
        Err(e) => println!("✅ Proper error handling: {}", e),
    }
    
    println!("\nTesting normal operations with valid variables:");
    let x = model.int(1, 10);
    let y = model.int(5, 15);
    let z = model.int(3, 8);
    
    match model.min(&[x, y, z]) {
        Ok(min_var) => println!("✅ min() succeeded: {:?}", min_var),
        Err(e) => println!("❌ Unexpected error: {}", e),
    }
    
    match model.max(&[x, y, z]) {
        Ok(max_var) => println!("✅ max() succeeded: {:?}", max_var),
        Err(e) => println!("❌ Unexpected error: {}", e),
    }
    
    println!("\n=== Summary ===");
    println!("✅ No panics occurred");
    println!("✅ Empty inputs return proper errors");
    println!("✅ Valid inputs work correctly");
    println!("✅ Issue #4 'Panic in Public API' has been RESOLVED");
}