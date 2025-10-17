// Debug test for modulo constraint
use selen::prelude::*;

fn main() {
    println!("=== Debug Modulo Constraint ===\n");

    // Simplest possible test
    let mut m = Model::default();
    
    let x = m.int(47, 47);      // Fixed x = 47
    let y = m.int(10, 10);       // Fixed y = 10
    let result = m.int(0, 9);    // Result in [0..9]

    println!("Variables created:");
    println!("  x: {:?}", x);
    println!("  y: {:?}", y);
    println!("  result: {:?}", result);
    
    // Add modulo constraint
    println!("\nAdding modulo constraint: result = x mod y");
    let mod_id = m.modulo(x, y);
    println!("  Modulo created variable: {:?}", mod_id);
    
    // Constrain result to equal mod_id
    m.new(result.eq(mod_id));
    
    println!("\nSolving...");
    match m.solve() {
        Ok(solution) => {
            let x_val = solution.get_int(x);
            let y_val = solution.get_int(y);
            let result_val = solution.get_int(result);
            let mod_val = solution.get_int(mod_id);
            
            println!("✓ Solution found!");
            println!("  x = {}", x_val);
            println!("  y = {}", y_val);
            println!("  result = {} (should be 7)", result_val);
            println!("  mod_id = {} (the direct modulo result)", mod_val);
        }
        Err(e) => {
            println!("✗ Failed: {:?}", e);
        }
    }
}
