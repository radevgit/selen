use cspsolver::prelude::*;

fn main() {
    let mut model = Model::new();
    
    println!("🧹 Cleaned up constraint macros - using only clean syntax!");
    
    // Test variables
    let a = model.int(0, 1);
    let b = model.int(0, 1);
    let c = model.int(0, 1);
    let x = model.int(1, 10);
    let y = model.int(1, 10);
    let fx = model.float(1.0, 10.0);
    
    println!("\n✅ CLEAN LOGICAL OPERATORS:");
    println!("   and(a, b) instead of bool_and(a, b)");
    post!(model, and(a, b) == c);
    
    println!("   or(a, b) instead of bool_or(a, b)");  
    post!(model, or(a, b) == c);
    
    println!("   not(a) instead of bool_not(a)");
    post!(model, not(a) == b);
    
    println!("\n✅ OTHER CLEAN FEATURES WE KEPT:");
    println!("   sum() function");
    post!(model, sum([x, y]) <= int(15));
    
    println!("   Float constants with math functions");
    post!(model, abs(fx) <= float(5.5));
    post!(model, min(fx, float(3.0)) >= float(1.0));
    
    println!("   Enhanced modulo operations");
    post!(model, x % y <= int(5));
    
    println!("   Complex nested expressions");
    post!(model, sum([x, y]) + abs(fx) >= float(10.0));
    post!(model, and(a, or(b, c)) == not(c));
    
    println!("\n🎉 RESULT: Clean, consistent constraint macro syntax!");
    println!("   - Removed redundant bool_* functions");
    println!("   - Using traditional and(), or(), not() operators");
    println!("   - Kept all powerful functionality");
    println!("   - Much cleaner and more readable!");
}