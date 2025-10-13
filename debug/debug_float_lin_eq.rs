use selen::prelude::*;

fn main() {
    // Test simple float linear equality
    let mut m = Model::default();
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    
    println!("Before constraint:");
    println!("x domain: [{:?}, {:?}]", x.min(&m.vars), x.max(&m.vars));
    println!("y domain: [{:?}, {:?}]", y.min(&m.vars), y.max(&m.vars));
    
    // Add equality: x + y = 5.0
    m.lin_eq(&[1.0, 1.0], &[x, y], 5.0);
    
    println!("\nAfter posting constraint (before solve):");
    println!("x domain: [{:?}, {:?}]", x.min(&m.vars), x.max(&m.vars));
    println!("y domain: [{:?}, {:?}]", y.min(&m.vars), y.max(&m.vars));
    
    let solution = m.solve().expect("Should find solution");
    
    println!("\nAfter solve:");
    if let (Val::ValF(x_val), Val::ValF(y_val)) = (solution[x], solution[y]) {
        println!("x = {}", x_val);
        println!("y = {}", y_val);
        println!("x + y = {}", x_val + y_val);
        println!("Expected: 5.0");
        println!("Difference: {}", (x_val + y_val - 5.0).abs());
        
        if (x_val + y_val - 5.0).abs() > 1e-6 {
            println!("\n❌ FAILED: Constraint not satisfied!");
        } else {
            println!("\n✅ PASSED: Constraint satisfied");
        }
    } else {
        println!("❌ Not float values!");
    }
}
