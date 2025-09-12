//! Simple Boolean Operators Demo
//! 
//! This demonstrates clean boolean syntax options:
//! 1. Using bitwise operators & and | (if we implement them)
//! 2. Using short method names .and(), .or(), .not()

use cspsolver::prelude::*;

fn main() {
    let mut model = Model::default();
    
    println!("🚀 Simple Boolean Syntax Options");
    println!("================================\n");
    
    let a = model.int(0, 1);  // Boolean variable
    let b = model.int(0, 1);  // Boolean variable
    
    println!("Current verbose syntax:");
    println!("  let and_result = model.bool_and(&[a, b]);     // Verbose!");
    println!("  let or_result = model.bool_or(&[a, b]);       // Verbose!");
    println!("  let not_result = model.bool_not(a);           // Verbose!");
    
    // Current verbose way
    let and_result = model.bool_and(&[a, b]);
    let or_result = model.bool_or(&[a, b]); 
    let not_result = model.bool_not(a);
    
    println!("\nWhat we WANT (but can't have with || &&):");
    println!("  let and_result = a && b;                      // Can't do this!");
    println!("  let or_result = a || b;                       // Can't do this!");
    
    println!("\nOption 1 - Bitwise operators (we could implement):");
    println!("  let and_result = a & b;                       // Could work!");
    println!("  let or_result = a | b;                        // Could work!");
    println!("  let not_result = !a;                          // Could work!");
    
    println!("\nOption 2 - Short method names:");
    println!("  let and_result = a.and(b);                    // Much cleaner!");
    println!("  let or_result = a.or(b);                      // Much cleaner!");
    println!("  let not_result = a.not();                     // Much cleaner!");
    
    // Set up constraints
    model.post(vec![
        a.eq_int(1),
        b.eq_int(0)
    ]);
    
    match model.solve() {
        Some(solution) => {
            println!("\n✅ Solution found!");
            let a_val = if let Val::ValI(v) = solution[a] { v } else { 0 };
            let b_val = if let Val::ValI(v) = solution[b] { v } else { 0 };
            let and_val = if let Val::ValI(v) = solution[and_result] { v } else { 0 };
            let or_val = if let Val::ValI(v) = solution[or_result] { v } else { 0 };
            let not_val = if let Val::ValI(v) = solution[not_result] { v } else { 0 };
            
            println!("  a = {} ({})", a_val, if a_val == 1 { "true" } else { "false" });
            println!("  b = {} ({})", b_val, if b_val == 1 { "true" } else { "false" });
            println!("  a AND b = {}", and_val);
            println!("  a OR b = {}", or_val);
            println!("  NOT a = {}", not_val);
        }
        None => {
            println!("❌ No solution found");
        }
    }
    
    println!("\n📝 Summary:");
    println!("  • Can't use && || (they're for immediate evaluation)");
    println!("  • CAN use & | (bitwise operators can be overloaded)");
    println!("  • CAN use .and() .or() .not() (method syntax)");
    println!("  • Both options are MUCH cleaner than model.bool_and(&[a, b])!");
}
