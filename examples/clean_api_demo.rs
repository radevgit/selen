//! Example demonstrating the clean constraint API with post! macro.
//!
//! This shows the new mathematical syntax using the post! macro,
//! which provides the cleanest constraint specification possible.

use cspsolver::prelude::*;
use cspsolver::vars::Val;

fn main() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let z = m.int(1, 5);
    
    println!("=== Clean Mathematical Constraint API Demo ===\n");
    
    // 1. Basic comparison constraints with mathematical syntax
    println!("1. Mathematical Comparison Syntax:");
    println!("   post!(m, x <= y);  // x <= y");
    post!(m, x <= y);
    
    println!("   post!(m, x != y);  // x != y");  
    post!(m, x != y);
    
    println!("   post!(m, x == 5);  // x == 5");
    post!(m, x == 5);
    
    // 2. Batch constraint addition with postall!
    println!("\n2. Batch Constraints with postall!:");
    println!("   postall!(m, x >= z, z < y);");
    postall!(m, x >= z, z < y);
    
    // 3. Clean batch syntax with more constraints
    println!("\n3. Complex Mathematical Expressions:");
    println!("   postall!(m, x <= y, x != z, y > z);");
    postall!(m, x <= y, x != z, y > z);
    
    // 4. Mathematical operations and functions
    println!("\n4. Mathematical Operations:");
    println!("   post!(m, alldiff([x, y, z]));  // All different");
    post!(m, alldiff([x, y, z]));
    
    // 5. Solving
    println!("\n5. Solving:");
    match m.solve() {
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
