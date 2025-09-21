use cspsolver::prelude::*;

fn main() {
    let mut model = Model::default();
    
    println!("Creating invalid variable with int(10, 5)...");
    let x = model.int(10, 5);
    println!("Variable created: {:?}", x);
    
    println!("Running validation...");
    match model.validate() {
        Ok(_) => println!("❌ Validation passed (unexpected)"),
        Err(e) => println!("✅ Validation caught error: {}", e),
    }
    
    println!("Attempting to solve...");
    match model.solve() {
        Ok(_) => println!("❌ Solving succeeded (unexpected)"),
        Err(e) => println!("✅ Solving failed: {}", e),
    }
}