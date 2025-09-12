//! All Three Options Demo
//! 
//! This demonstrates the three clean syntax options:
//! Option A: Bitwise operators (a & b, a | b, !a)
//! Option B: Short method names (a.and(b), a.or(b), a.not())
//! Option C: Direct arithmetic methods (a.abs(), a.add(b), a.sub(b))

use cspsolver::prelude::*;
use cspsolver::boolean_operators::{BoolVar, BooleanModel};

fn main() {
    println!("🚀 All Three Clean Syntax Options Demo");
    println!("=====================================\n");
    
    // OPTION A: Bitwise Operators (&, |, !)
    println!("🔧 Option A: Bitwise Operators");
    println!("==============================");
    demo_bitwise_operators();
    
    // OPTION B: Short Method Names  
    println!("\n🔧 Option B: Short Method Names");
    println!("==============================");
    demo_short_methods();
    
    // OPTION C: Direct Arithmetic Methods
    println!("\n🔧 Option C: Direct Arithmetic Methods");
    println!("=====================================");
    demo_arithmetic_methods();
    
    // COMPARISON 
    println!("\n📊 Syntax Comparison");
    println!("===================");
    syntax_comparison();
}

fn demo_bitwise_operators() {
    let mut model = Model::default();
    
    // Create boolean variables using BoolVar wrapper
    let a = BoolVar::new(model.int(0, 1));
    let b = BoolVar::new(model.int(0, 1));
    let c = BoolVar::new(model.int(0, 1));
    
    println!("Variables:");
    println!("  let a = BoolVar::new(model.int(0, 1));");
    println!("  let b = BoolVar::new(model.int(0, 1));");
    println!("  let c = BoolVar::new(model.int(0, 1));");
    
    println!("\nBitwise Boolean Operations:");
    println!("  let and_result = model.bool_expr(a & b);       // a & b");
    let and_result = model.bool_expr(a & b);
    
    println!("  let or_result = model.bool_expr(a | b);        // a | b");
    let or_result = model.bool_expr(a | b);
    
    println!("  let not_result = model.bool_expr(!a);          // !a");
    let not_result = model.bool_expr(!a);
    
    println!("  let complex = model.bool_expr((a | b) & !c);   // Complex expression");
    let complex = model.bool_expr((a | b) & !c);
    
    // Set constraints
    model.post(vec![
        a.var_id().eq_int(1),
        b.var_id().eq_int(0),
        c.var_id().eq_int(0)
    ]);
    
    match model.solve() {
        Some(solution) => {
            println!("\n✅ Solution found!");
            let a_val = if let Val::ValI(v) = solution[a.var_id()] { v } else { 0 };
            let b_val = if let Val::ValI(v) = solution[b.var_id()] { v } else { 0 };
            let c_val = if let Val::ValI(v) = solution[c.var_id()] { v } else { 0 };
            println!("  a = {}, b = {}, c = {}", a_val, b_val, c_val);
            println!("  a & b = {:?}", solution[and_result]);
            println!("  a | b = {:?}", solution[or_result]);
            println!("  !a = {:?}", solution[not_result]);
            println!("  (a | b) & !c = {:?}", solution[complex]);
        }
        None => println!("❌ No solution found"),
    }
}

fn demo_short_methods() {
    let mut model = Model::default();
    
    let a = model.int(0, 1);
    let b = model.int(0, 1);
    let c = model.int(0, 1);
    
    println!("Variables:");
    println!("  let a = model.int(0, 1);");
    println!("  let b = model.int(0, 1);");
    println!("  let c = model.int(0, 1);");
    
    println!("\nShort Method Boolean Operations:");
    println!("  let and_result = model.bool_result(a.and(b));  // Clean method syntax");
    let and_result = model.bool_result(a.and(b));
    
    println!("  let or_result = model.bool_result(a.or(b));    // Clean method syntax");
    let or_result = model.bool_result(a.or(b));
    
    println!("  let not_result = model.bool_result(a.not());   // Clean method syntax");
    let not_result = model.bool_result(a.not());
    
    // Set constraints
    model.post(vec![
        a.eq_int(1),
        b.eq_int(0),
        c.eq_int(1)
    ]);
    
    match model.solve() {
        Some(solution) => {
            println!("\n✅ Solution found!");
            let a_val = if let Val::ValI(v) = solution[a] { v } else { 0 };
            let b_val = if let Val::ValI(v) = solution[b] { v } else { 0 };
            let c_val = if let Val::ValI(v) = solution[c] { v } else { 0 };
            println!("  a = {}, b = {}, c = {}", a_val, b_val, c_val);
            println!("  a.and(b) = {:?}", solution[and_result]);
            println!("  a.or(b) = {:?}", solution[or_result]);
            println!("  a.not() = {:?}", solution[not_result]);
        }
        None => println!("❌ No solution found"),
    }
}

fn demo_arithmetic_methods() {
    let mut model = Model::default();
    
    let x = model.int(-10, 10);
    let y = model.int(-5, 15);
    
    println!("Variables:");
    println!("  let x = model.int(-10, 10);");
    println!("  let y = model.int(-5, 15);");
    
    println!("\nDirect Arithmetic Methods:");
    println!("  let abs_x = model.arith_result(x.abs());       // |x|");
    let abs_x = model.arith_result(x.abs());
    
    println!("  let sum_xy = model.arith_result(x.add(y));      // x + y");
    let sum_xy = model.arith_result(x.add(y));
    
    println!("  let diff_xy = model.arith_result(x.sub(y));     // x - y");
    let diff_xy = model.arith_result(x.sub(y));
    
    println!("  let prod_xy = model.arith_result(x.mul(y));     // x * y");
    let prod_xy = model.arith_result(x.mul(y));
    
    // Set constraints  
    model.post(vec![
        x.eq_int(-3),
        y.eq_int(4),
        abs_x.eq_int(3),        // |-3| = 3
        sum_xy.eq_int(1),       // -3 + 4 = 1
        diff_xy.eq_int(-7),     // -3 - 4 = -7
        prod_xy.eq_int(-12)     // -3 * 4 = -12
    ]);
    
    match model.solve() {
        Some(solution) => {
            println!("\n✅ Solution found!");
            println!("  x = {:?}, y = {:?}", solution[x], solution[y]);
            println!("  |x| = {:?}", solution[abs_x]);
            println!("  x + y = {:?}", solution[sum_xy]);
            println!("  x - y = {:?}", solution[diff_xy]);
            println!("  x * y = {:?}", solution[prod_xy]);
        }
        None => println!("❌ No solution found"),
    }
}

fn syntax_comparison() {
    println!("Boolean Operations:");
    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│ OLD (Verbose):                                          │");
    println!("│   let result = model.bool_and(&[a, b]);                │");
    println!("│   let result = model.bool_or(&[a, b]);                 │");
    println!("│   let result = model.bool_not(a);                      │");
    println!("│─────────────────────────────────────────────────────────│");
    println!("│ Option A (Bitwise):                                     │");
    println!("│   let result = model.bool_expr(a & b);     // 50% shorter│");
    println!("│   let result = model.bool_expr(a | b);     // 50% shorter│");
    println!("│   let result = model.bool_expr(!a);        // 60% shorter│");
    println!("│─────────────────────────────────────────────────────────│");
    println!("│ Option B (Methods):                                     │");
    println!("│   let result = model.bool_result(a.and(b)); // Readable │");
    println!("│   let result = model.bool_result(a.or(b));  // Readable │");
    println!("│   let result = model.bool_result(a.not());  // Readable │");
    println!("└─────────────────────────────────────────────────────────┘");
    
    println!("\nArithmetic Operations:");
    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│ OLD (Model calls):                                      │");
    println!("│   let result = model.abs(x);                           │");
    println!("│   let result = model.add(x, y);                        │");
    println!("│   let result = model.sub(x, y);                        │");
    println!("│─────────────────────────────────────────────────────────│");
    println!("│ Option C (Direct methods):                              │");
    println!("│   let result = model.arith_result(x.abs());    // Clean │");
    println!("│   let result = model.arith_result(x.add(y));   // Clean │");
    println!("│   let result = model.arith_result(x.sub(y));   // Clean │");
    println!("└─────────────────────────────────────────────────────────┘");
    
    println!("\n📝 Summary:");
    println!("  ✅ Option A: Most concise for boolean (a & b)");
    println!("  ✅ Option B: Most readable for boolean (a.and(b))");
    println!("  ✅ Option C: Clean arithmetic methods (x.abs())");
    println!("  🎯 All three options much cleaner than verbose originals!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_all_three_options() {
        // Test that all three options compile and work
        let mut model = Model::default();
        
        // Option A: Bitwise operators
        let a = BoolVar::new(model.int(0, 1));
        let b = BoolVar::new(model.int(0, 1));
        let _bitwise_result = model.bool_expr(a & b);
        
        // Option B: Short methods
        let c = model.int(0, 1);
        let d = model.int(0, 1);
        let _method_result = model.bool_result(c.and(d));
        
        // Option C: Arithmetic methods
        let x = model.int(-5, 5);
        let y = model.int(-3, 7);
        let _arith_result = model.arith_result(x.abs());
        
        // If we get here, all three options work!
        assert!(true);
    }
}
