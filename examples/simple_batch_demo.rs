//! Simple Batch Operations Demo
//! 
//! This demonstrates what you probably want for batch operations:
//! Using a.abs(), a.add(b), a.sub(b) etc. in constraints

use cspsolver::prelude::*;

fn main() {
    let mut model = Model::default();
    
    println!("🚀 Simple Batch Operations");
    println!("==========================\n");
    
    let a = model.int(-10, 10);
    let b = model.int(-5, 15);
    
    println!("Current way (direct model calls):");
    println!("  let abs_a = model.abs(a);");
    println!("  let sum_ab = model.add(a, b);");
    println!("  let diff_ab = model.sub(a, b);");
    
    // Current way
    let abs_a = model.abs(a);
    let sum_ab = model.add(a, b);
    let diff_ab = model.sub(a, b);
    
    println!("\nWhat you probably want:");
    println!("  let abs_a = a.abs();          // Simpler!");
    println!("  let sum_ab = a.add(b);        // Cleaner!");
    println!("  let diff_ab = a.sub(b);       // More intuitive!");
    
    println!("\nUsing in batch constraints:");
    println!("  model.post(vec![");
    println!("      a.ge_zero(),");
    println!("      abs_a.le_int(8),     // |a| <= 8");
    println!("      sum_ab.ge_zero(),    // a + b >= 0");
    println!("      diff_ab.lt_int(5)    // a - b < 5");
    println!("  ]);");
    
    // Set up constraints
    model.post(vec![
        a.ge_zero(),            // a >= 0
        abs_a.le_int(8),        // |a| <= 8
        sum_ab.ge_zero(),       // a + b >= 0
        diff_ab.lt_int(5)       // a - b < 5
    ]);
    
    match model.solve() {
        Some(solution) => {
            println!("\n✅ Solution found!");
            println!("  a = {:?}", solution[a]);
            println!("  b = {:?}", solution[b]);
            println!("  |a| = {:?}", solution[abs_a]);
            println!("  a + b = {:?}", solution[sum_ab]);
            println!("  a - b = {:?}", solution[diff_ab]);
        }
        None => {
            println!("❌ No solution found");
        }
    }
    
    println!("\n📝 Summary:");
    println!("  • a.abs() instead of model.abs(a)");
    println!("  • a.add(b) instead of model.add(a, b)");
    println!("  • a.sub(b) instead of model.sub(a, b)");
    println!("  • Much cleaner for batch operations!");
}
