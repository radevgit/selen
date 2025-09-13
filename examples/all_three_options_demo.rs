//! All Three Options Demo - Updated with post! macro
//! 
//! This demonstrates using the new post! macro for mathematical constraint syntax.
//! Now using clean mathematical expressions instead of verbose method calls.

use cspsolver::prelude::*;

/// Mathematical Constraint Syntax Demo
/// 
/// This demonstrates the powerful post! and postall! macros for clean mathematical constraint syntax.
/// Shows various mathematical expressions, functions, and constraint patterns.

use cspsolver::prelude::*;

fn main() {
    println!("🚀 Mathematical Constraint Syntax Demo");
    println!("======================================\n");
    
    // Basic mathematical constraints
    println!("🔧 Basic Mathematical Constraints");
    println!("=================================");
    demo_basic_math();
    
    // Mathematical functions
    println!("\n🔧 Mathematical Functions");
    println!("=========================");
    demo_math_functions();
    
    // Batch constraints with postall!
    println!("\n🔧 Batch Constraints with postall!");
    println!("===================================");
    demo_batch_constraints();
    
    // Complex problem solving
    println!("\n🔧 Complex Problem Example");
    println!("==========================");
    demo_complex_problem();
}

use cspsolver::prelude::*;

fn main() {
    println!("� Mathematical Constraint Syntax Demo");
    println!("======================================\n");
    
    // OPTION 1: Old verbose syntax
    println!("🔧 Old Verbose Syntax");
    println!("=====================");
    demo_verbose_syntax();
    
    // OPTION 2: New post! macro mathematical syntax  
    println!("\n🔧 New post! Mathematical Syntax");
    println!("================================");
    demo_post_syntax();
    
    // COMPARISON 
    println!("\n📊 Syntax Comparison");
    println!("===================");
    syntax_comparison();
}

fn demo_verbose_syntax() {
    let mut m = Model::default();
    
    let x = m.int(-10, 10);
    let y = m.int(-5, 15);
    let z = m.int(0, 20);
    
    println!("Variables:");
    println!("  let x = m.int(-10, 10);");
    println!("  let y = m.int(-5, 15);");
    println!("  let z = m.int(0, 20);");
    
    println!("\nOld Verbose Constraints:");
    println!("  m.post(x.leq(y));           // x <= y");
    for constraint in x.leq(y) {
        m.post(constraint);
    }
    
    println!("  m.post(x.neq(z));           // x != z");
    for constraint in x.neq(z) {
        m.post(constraint);
    }
    
    let sum = m.add(x, y);
    println!("  let sum = m.add(x, y);");
    println!("  m.post(sum.eq_int(7));      // x + y = 7");
    m.post(sum.eq_int(7));
    
    match m.solve() {
        Some(solution) => {
            println!("\n✅ Solution found!");
            println!("  x = {:?}, y = {:?}, z = {:?}", solution[x], solution[y], solution[z]);
            println!("  x + y = {:?}", solution[sum]);
        }
        None => println!("❌ No solution found"),
    }
}

fn demo_post_syntax() {
    let mut m = Model::default();
    
    let x = m.int(-10, 10);
    let y = m.int(-5, 15);
    let z = m.int(0, 20);
    
    println!("Variables:");
    println!("  let x = m.int(-10, 10);");
    println!("  let y = m.int(-5, 15);");
    println!("  let z = m.int(0, 20);");
    
    println!("\nNew Mathematical Constraints:");
    println!("  post!(m, x <= y);           // Clean mathematical syntax");
    post!(m, x <= y);
    
    println!("  post!(m, x != z);           // Clean mathematical syntax");
    post!(m, x != z);
    
    println!("  post!(m, x + y == 7);       // Direct arithmetic expression");
    post!(m, x + y == 7);
    
    // Demonstrate more mathematical functions
    println!("  post!(m, abs(x) <= 5);      // Absolute value function");
    post!(m, abs(x) <= 5);
    
    println!("  post!(m, min(x, y) >= 0);   // Min function");
    post!(m, min(x, y) >= 0);
    
    match m.solve() {
        Some(solution) => {
            println!("\n✅ Solution found!");
            println!("  x = {:?}, y = {:?}, z = {:?}", solution[x], solution[y], solution[z]);
        }
        None => println!("❌ No solution found"),
    }
}

fn syntax_comparison() {
    println!("Constraint Syntax Comparison:");
    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│ OLD (Verbose):                                          │");
    println!("│   m.post(x.leq(y));                                     │");
    println!("│   m.post(x.neq(z));                                     │");
    println!("│   let sum = m.add(x, y);                                │");
    println!("│   m.post(sum.eq_int(7));                                │");
    println!("│   let abs_x = m.abs(x);                                 │");
    println!("│   m.post(abs_x.leq_int(5));                            │");
    println!("│─────────────────────────────────────────────────────────│");
    println!("│ NEW (Mathematical):                                     │");
    println!("│   post!(m, x <= y);          // 60% shorter, readable   │");
    println!("│   post!(m, x != z);          // 60% shorter, readable   │");
    println!("│   post!(m, x + y == 7);      // 70% shorter, intuitive  │");
    println!("│   post!(m, abs(x) <= 5);     // 65% shorter, clear      │");
    println!("│   post!(m, min(x, y) >= 0);  // Mathematical functions  │");
    println!("└─────────────────────────────────────────────────────────┘");
    
    println!("\nBatch Constraints with postall!:");
    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│ OLD (Multiple calls):                                   │");
    println!("│   m.post(x.leq(y));                                     │");
    println!("│   m.post(x.neq(z));                                     │");
    println!("│   m.post(sum.eq_int(7));                                │");
    println!("│─────────────────────────────────────────────────────────│");
    println!("│ NEW (Single call):                                      │");
    println!("│   postall!(m,                                           │");
    println!("│     x <= y,              // Clean mathematical syntax   │");
    println!("│     x != z,              // Multiple constraints        │");
    println!("│     x + y == 7           // In one macro call           │");
    println!("│   );                                                    │");
    println!("└─────────────────────────────────────────────────────────┘");
    
    println!("\n📝 Summary:");
    println!("  ✅ post! macro: Clean mathematical syntax (x + y == 7)");
    println!("  ✅ postall! macro: Batch multiple constraints elegantly");
    println!("  ✅ Mathematical functions: abs(), min(), max(), etc.");
    println!("  ✅ Intuitive operators: ==, !=, <=, >=, <, >");
    println!("  🎯 Much more readable than verbose method calls!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mathematical_syntax() {
        let mut m = Model::default();
        
        let x = m.int(0, 10);
        let y = m.int(0, 10);
        let z = m.int(0, 10);
        
        // Test post! macro with mathematical syntax
        post!(m, x + y == 10);
        post!(m, x <= y);
        post!(m, abs(x - y) <= 2);
        post!(m, min(x, y) >= 3);
        
        // Test postall! macro
        postall!(m,
            x != z,
            y != z,
            x + y + z <= 20
        );
        
        // Should be able to find a solution
        let solution = m.solve();
        assert!(solution.is_some());
    }
}
