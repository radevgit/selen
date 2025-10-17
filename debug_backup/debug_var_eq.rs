use selen::prelude::*;

fn main() {
    println!("=== Debug Variable-to-Variable Equality ===\n");

    let mut m = Model::default();
    
    let dividend = m.int(1, 100);
    let divisor = m.int(1, 10);
    
    // Create constant VarIds for constraints
    let const_47 = m.int(47, 47);
    let const_10 = m.int(10, 10);
    
    println!("Before any constraints:");
    println!("  dividend domain: [1, 100]");
    println!("  divisor domain: [1, 10]");
    println!("  const_47 domain: [47, 47]");
    println!("  const_10 domain: [10, 10]\n");
    
    // Create modulo FIRST with unconstrained variables
    let mod_result = m.modulo(dividend, divisor);
    println!("After modulo() call:");
    println!("  mod_result domain should be based on unconstrained dividend/divisor\n");
    
    // THEN post equality constraints to constants
    println!("Posting: dividend.eq(const_47)");
    m.new(dividend.eq(const_47));
    
    println!("Posting: divisor.eq(const_10)");
    m.new(divisor.eq(const_10));
    
    println!("Posting: mod_result constraints\n");
    
    // Try to solve
    match m.solve() {
        Ok(sol) => {
            println!("✓ SOLVED!");
            println!("  dividend = {}", sol.get_int(dividend));
            println!("  divisor = {}", sol.get_int(divisor));
            println!("  mod_result = {}", sol.get_int(mod_result));
        }
        Err(e) => {
            println!("✗ CANNOT SOLVE");
            println!("Error: {:?}\n", e);
        }
    }
}
