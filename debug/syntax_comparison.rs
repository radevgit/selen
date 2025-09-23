use selen::prelude::*;

fn main() {
    let mut model = Model::new();
    let a = m.int(0, 1);
    let b = m.int(0, 1);
    
    println!("Comparing different NOT syntax options:");
    
    // Current implementations:
    println!("1. Function style: not(a) - WORKS");
    post!(m, not(a) == b);
    
    // The 'not()' function works in both post! and postall! macros
    println!("2. Traditional: not(a) - WORKS in both post! and postall!");
    
    println!("3. Rust-style: !a - NOT IMPLEMENTED (would require complex parsing)");
    // This would be ideal: post!(m, !a == b);
    
    println!("\nCurrent clean syntax:");
    println!("We use clean traditional functions: and(), or(), not()");
    println!("The bool_* functions have been removed for cleaner syntax.");
}
}

// Example of what would be ideal:
fn ideal_syntax_example() {
    let mut model = Model::new();
    let a = m.int(0, 1);
    let b = m.int(0, 1);
    let c = m.int(0, 1);
    
    // These would be the most natural Rust syntax:
    // post!(m, !a == b);               // NOT
    // post!(m, a && b == c);           // AND  
    // post!(m, a || b == c);           // OR
    // post!(m, !(a && b) == (!a || !b)); // De Morgan's law
}