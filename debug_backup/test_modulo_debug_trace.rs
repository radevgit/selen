// Debug: trace what happens with deferred modulo
use selen::prelude::*;

fn main() {
    let mut m = Model::default();
    
    let number = m.int(10, 100);
    let divisor = m.int(10, 10);
    let remainder = m.int(0, 9);
    
    println!("After creating variables:");
    println!("  number domain: [10..100]");
    println!("  divisor domain: [10..10]");
    println!("  remainder domain: [0..9]");
    
    println!("\nCalling m.new(number.eq(47))...");
    m.new(number.eq(47));
    println!("  (constraint deferred)");
    
    println!("\nCalling m.modulo(number, divisor)...");
    let mod_result = m.modulo(number, divisor);
    println!("  mod_result variable created: {:?}", mod_result);
    
    println!("\nCalling m.new(remainder.eq(mod_result))...");
    m.new(remainder.eq(mod_result));
    println!("  (constraint deferred)");
    
    println!("\nCalling m.solve()...");
    match m.solve() {
        Ok(sol) => {
            println!("✓ SOLUTION FOUND!");
            println!("  number = {}", sol.get_int(number));
            println!("  divisor = {}", sol.get_int(divisor));
            println!("  remainder = {}", sol.get_int(remainder));
            println!("  mod_result = {}", sol.get_int(mod_result));
        }
        Err(e) => {
            println!("✗ NO SOLUTION: {:?}", e);
        }
    }
}
