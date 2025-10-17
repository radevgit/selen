use selen::prelude::*;

fn main() {
    println!("=== Debug Modulo Result Bounds ===\n");

    let mut m = Model::default();
    
    let dividend = m.int(1, 100);
    let divisor = m.int(1, 10);
    
    println!("dividend domain: [1, 100]");
    println!("divisor domain: [1, 10]");
    println!("Calling m.modulo(dividend, divisor)...");
    
    // When we call modulo with divisor domain [1, 10]:
    // The result should be conservative enough to handle [0, 9]
    // But if deferred constraints will later fix divisor to 10,
    // the result should still be [0, 9]
    let mod_result = m.modulo(dividend, divisor);
    
    println!("Created mod_result variable");
    println!("Now constraining divisor to 10 via equality");
    
    let const_10 = m.int(10, 10);
    m.new(divisor.eq(const_10));
    
    println!("Now constraining dividend to 47 via equality");
    let const_47 = m.int(47, 47);
    m.new(dividend.eq(const_47));
    
    println!("\nAttempting to solve:");
    println!("Expected: 47 mod 10 = 7");
    
    match m.solve() {
        Ok(sol) => {
            let div = sol.get_int(dividend);
            let vis = sol.get_int(divisor);
            let res = sol.get_int(mod_result);
            println!("\n✓ SOLVED!");
            println!("  dividend = {}", div);
            println!("  divisor = {}", vis);
            println!("  mod_result = {}", res);
            println!("  Expected mod_result = 7: {}", if res == 7 { "✓" } else { "✗" });
        }
        Err(e) => {
            println!("\n✗ CANNOT SOLVE");
            println!("Error: {:?}", e);
        }
    }
}
