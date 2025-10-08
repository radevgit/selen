use selen::prelude::*;
use std::time::Instant;

fn main() {
    println!("Testing large domain optimization...\n");
    
    let mut model = Model::default();
    let x = model.float(-1e6, 1e6);
    let y = model.float(-1e6, 1e6);
    
    let sum_var = model.float(-2e6, 2e6);
    model.new(x.add(y).eq(sum_var));
    model.new(sum_var.le(0.0));
    
    println!("Starting maximize...");
    let start = Instant::now();
    
    let result = model.maximize(x);
    
    let elapsed = start.elapsed();
    println!("Maximize finished in {:?}", elapsed);
    
    match result {
        Ok(solution) => {
            let x_val = solution.get_float(x);
            let y_val = solution.get_float(y);
            let sum_val = solution.get_float(sum_var);
            
            println!("SUCCESS!");
            println!("  x = {}", x_val);
            println!("  y = {}", y_val);
            println!("  sum = {}", sum_val);
            println!("Stats: {:?}", solution.stats());
        }
        Err(e) => {
            println!("FAILED: {:?}", e);
        }
    }
}
