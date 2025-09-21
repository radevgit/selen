use cspsolver::prelude::*;

fn main() {
    println!("Creating invalid float variable with float(3.5, 1.2)...");
    
    let mut model = Model::default();
    let x = model.float(3.5, 1.2);
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