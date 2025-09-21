use cspsolver::prelude::*;
use cspsolver::error::SolverError;

fn main() {
    println!("=== Testing empty domain behavior ===");
    
    // Test 1: Empty domain
    let mut model = Model::default();
    let _var = model.ints(vec![]); // Empty domain
    
    match model.solve() {
        Ok(sol) => {
            println!("Empty domain: Solved successfully - {:#?}", sol);
        }
        Err(err) => {
            println!("Empty domain: Error - {:#?}", err);
            println!("Error message: {}", err);
        }
    }
    
    println!("\n=== Testing invalid float bounds ===");
    
    // Test 2: Invalid float bounds (min > max)
    let mut model2 = Model::default();
    let _x = model2.float(10.0, 5.0); // min > max
    
    match model2.solve() {
        Ok(sol) => {
            println!("Invalid float bounds: Solved successfully - {:#?}", sol);
        }
        Err(err) => {
            println!("Invalid float bounds: Error - {:#?}", err);
            println!("Error message: {}", err);
        }
    }
    
    println!("\n=== Testing what happens with model.ints() ===");
    
    // Let's see what ints() actually does with empty vec
    let mut model3 = Model::default();
    let var_result = model3.ints(vec![]);
    println!("ints(vec![]) returned: {:#?}", var_result);
}