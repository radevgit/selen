use selen::prelude::*;

fn main() {
    println!("=== Testing Var==Var Bounds Application ===\n");

    let mut m = Model::default();
    
    let x = m.int(1, 100);
    let const_50 = m.int(50, 50);
    
    // Before any constraints, x should have domain [1, 100]
    println!("Before eq constraint:");
    println!("  x created with domain [1, 100]");
    println!("  const_50 created with domain [50, 50]\n");
    
    // Post equality constraint
    m.new(x.eq(const_50));
    println!("After posting x.eq(const_50), before solve():");
    println!("  The bounds should be applied during materialization\n");
    
    // Try to solve
    match m.solve() {
        Ok(sol) => {
            let x_val = sol.get_int(x);
            println!("✓ SOLVED!");
            println!("  x = {}", x_val);
            println!("  x should equal 50: {}", if x_val == 50 { "✓" } else { "✗" });
        }
        Err(e) => {
            println!("✗ CANNOT SOLVE");
            println!("Error: {:?}", e);
        }
    }
}
