//! API Evolution Demo
//!
//! This example demonstrates the evolution of the CSP solver's constraint API
//! from verbose manual constraint creation to clean mathematical syntax.

use cspsolver::prelude::*;

fn main() {
    println!("🚀 CSP Solver API Evolution Demo");
    println!("==================================");
    
    // EVOLUTION: From verbose methods to mathematical syntax
    println!("\n❌ PAST: Verbose Method Calls");
    show_verbose_api();
    
    println!("\n✅ PRESENT: Clean Mathematical Syntax");
    show_mathematical_api();
    
    println!("\n📊 API Evolution Summary:");
    api_comparison();
}

fn show_verbose_api() {
    println!("   // The old way - verbose method names");
    println!("   model.equals(x, int(5));                    // x = 5");
    println!("   model.le(y, float(3.14));                  // y <= 3.14");
    println!("   model.ge(z, int(-2));                      // z >= -2");
    println!("   model.all_different(vec![x, y, z]);        // All different");
    println!("   
   // These methods are now REMOVED to avoid confusion!");
    println!("   // Users forced to use clean mathematical syntax");
}

fn show_mathematical_api() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.float(0.0, 10.0);
    let z = m.int(-5, 5);
    
    println!("   // The modern way - mathematical syntax");
    println!("   post!(m, x == 5);                          // x = 5");
    println!("   post!(m, y <= 3.14);                       // y <= 3.14"); 
    println!("   post!(m, z >= -2);                         // z >= -2");
    println!("   post!(m, alldiff([x, y, z]));              // All different");
    println!("   
   // Batch constraints with postall!");
    println!("   postall!(m, x >= 0, z != 0);              // Multiple at once");
    
    // Actually create the constraints for solving
    post!(m, x == 5);
    post!(m, y <= 3.14);
    post!(m, z >= -2);
    post!(m, y > 1);
    post!(m, z != 0);
    
    match m.solve() {
        Ok(solution) => {
            println!("   ✅ Mathematical solution: x={:?}, y={:?}, z={:?}", 
                     solution[x], solution[y], solution[z]);
        }
        Err(err) => {
            println!("   ❌ Failed to find solution: {}", err);
            return;
        }
    }
}

fn api_comparison() {
    println!("   Evolution: Method Calls → Mathematical Expressions");
    println!("   ┌─────────────────────────────────────────┬────────────┬────────────┬──────────┐");
    println!("   │ Constraint                              │ OLD API    │ NEW API    │ Savings  │");
    println!("   ├─────────────────────────────────────────┼────────────┼────────────┼──────────┤");
    println!("   │ x equals 5                              │ 21 chars   │ 12 chars   │ 43%      │");
    println!("   │ model.equals(x, int(5))                 │            │            │          │");
    println!("   │ post!(m, x == 5)                        │            │            │          │");
    println!("   ├─────────────────────────────────────────┼────────────┼────────────┼──────────┤");
    println!("   │ y less than or equal 3.14               │ 25 chars   │ 16 chars   │ 36%      │");
    println!("   │ model.le(y, float(3.14))                │            │            │          │");
    println!("   │ post!(m, y <= 3.14)                     │            │            │          │");
    println!("   ├─────────────────────────────────────────┼────────────┼────────────┼──────────┤");
    println!("   │ All different constraint                 │ 31 chars   │ 25 chars   │ 19%      │");
    println!("   │ model.all_different(vec![x, y, z])      │            │            │          │");
    println!("   │ post!(m, alldiff([x, y, z]))            │            │            │          │");
    println!("   └─────────────────────────────────────────┴────────────┴────────────┴──────────┘");
    
    println!("   
   🎯 Key Evolutionary Benefits:");
    println!("   • Mathematical syntax matches problem description");
    println!("   • 20-45% fewer characters to type");
    println!("   • No more method name confusion (equals vs eq vs equal)"); 
    println!("   • Familiar operators: ==, <=, >=, !=, <, >");
    println!("   • Batch constraints: postall!(m, x >= 1, y <= 10, z != 0)");
    println!("   • Function syntax: abs(x), min(x, y), max(x, y)");
    println!("   • Global constraints: alldiff([x, y, z])");
    
    println!("   
   🔮 Mathematical Expression Support:");
    println!("   • post!(m, x + y == z)               // Arithmetic");
    println!("   • post!(m, abs(x - y) <= 5)          // Functions");
    println!("   • post!(m, min(x, y) >= 1)           // Min/Max");
    println!("   • post!(m, x * 2 <= y + 3)           // Complex expressions");
    
    println!("   
   💡 Philosophy:");
    println!("   Mathematical problems deserve mathematical syntax!");
}
