//! Example demonstrating the clean constraint API with post! macro.
//!
//! This shows the new mathematical syntax using the post! macro,
//! which provides the cleanest constraint specification possible.

use cspsolver::prelude::*;
use cspsolver::vars::Val;

fn main() {
    let mut model = Model::default();
    let x = model.int(0, 10);
    let y = model.int(0, 10);
    let z = model.int(1, 5);
    
    println!("=== Clean Mathematical Constraint API Demo ===\n");
    
    // 1. Basic comparison constraints with mathematical syntax
    println!("1. Mathematical Comparison Syntax:");
    println!("   post!(model, x <= y);  // x <= y");
    post!(model, x <= y);
    
    println!("   post!(model, x != y);  // x != y");  
    post!(model, x != y);
    
    println!("   post!(model, x == 5);  // x == 5");
    post!(model, x == 5);
    
    // 2. Batch constraint addition with postall!
    println!("\n2. Batch Constraints with postall!:");
    println!("   postall!(model, x >= z, z < y);");
    postall!(model, x >= z, z < y);
    
    // 3. Clean batch syntax with more constraints
    println!("\n3. Complex Mathematical Expressions:");
    println!("   postall!(model, x <= y, x != z, y > z);");
    postall!(model, x <= y, x != z, y > z);
    
    // 4. Mathematical operations and functions
    println!("\n4. Mathematical Operations:");
    println!("   post!(model, alldiff([x, y, z]));  // All different");
    post!(model, alldiff([x, y, z]));
    
    // 5. Solving
    println!("\n5. Solving:");
    match model.solve() {
        Some(solution) => {
            println!("   Solution found!");
            println!("   x = {:?}", solution[x]);
            println!("   y = {:?}", solution[y]);
            println!("   z = {:?}", solution[z]);
            
            // Verify some constraints by pattern matching the values
            if let (Val::ValI(x_val), Val::ValI(y_val), Val::ValI(z_val)) = (solution[x], solution[y], solution[z]) {
                println!("   Verification:");
                println!("     x <= y: {} <= {} = {}", x_val, y_val, x_val <= y_val);
                println!("     x != y: {} != {} = {}", x_val, y_val, x_val != y_val);
                println!("     All different: x={}, y={}, z={}", x_val, y_val, z_val);
            }
        }
        None => {
            println!("   No solution exists with these constraints.");
        }
    }
    
    println!("\n✅ Mathematical syntax demonstration complete!");
    println!("   The post! macro provides the cleanest constraint API possible.");
}
