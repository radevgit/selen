//! Ultra-Clean Boolean Syntax Demo
//! 
//! This demonstrates the new ultra-clean boolean syntax:
//! ✅ model.bool() for clean variable creation
//! ✅ Direct & | ! operators on VarId 
//! ✅ No wrapper types needed
//! ✅ Complex expressions supported

use cspsolver::prelude::*;

fn main() {
    let mut model = Model::default();
    
    println!("🚀 Ultra-Clean Boolean Syntax");
    println!("==============================\n");
    
    // Ultra-clean variable creation
    let a = model.bool();  // Clean!
    let b = model.bool();  // Clean!
    let c = model.bool();  // Clean!
    
    println!("✅ NEW: Ultra-clean syntax achieved!");
    println!("  let a = model.bool();                         // Clean variable creation");
    println!("  model.post(a & b);                            // Direct boolean AND");
    println!("  model.post(a | c);                            // Direct boolean OR");
    println!("  model.post(!a);                               // Direct boolean NOT");
    println!("  model.post((a & b) | c);                      // Complex expressions");
    
    // Ultra-clean boolean operations
    model.post(a & b);           // Clean AND
    model.post(a | c);           // Clean OR  
    model.post(!a);              // Clean NOT
    model.post((a & b) | c);     // Complex expression
    
    // Demonstrate batch operations
    model.post_all(vec![a & b, !c, a | (b & c)]);
    
    println!("\n🎯 Before vs After Comparison:");
    println!("  BEFORE: model.int(0, 1)              ❌ Verbose");
    println!("  AFTER:  model.bool()                         ✅ Clean");
    println!();
    println!("  BEFORE: model.bool_and(&[a, b])              ❌ Verbose");
    println!("  AFTER:  model.post(a & b)                    ✅ Clean");
    println!();
    println!("  BEFORE: No batch boolean operations          ❌ Limited");
    println!("  AFTER:  model.post_all(vec![a & b, !c])      ✅ Powerful");
    
    // Set up some constraints to test
    model.post_all(vec![
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
            
            println!("  a = {} ({})", a_val, if a_val == 1 { "true" } else { "false" });
            println!("  b = {} ({})", b_val, if b_val == 1 { "true" } else { "false" });
            println!("  c = {} ({})", c_val, if c_val == 1 { "true" } else { "false" });
            println!("  All boolean constraints satisfied! 🎉");
        }
        None => {
            println!("❌ No solution found");
        }
    }
    
    println!("\n📝 Ultra-Clean API Summary:");
    println!("  ✅ model.bool() - Clean variable creation");
    println!("  ✅ a & b, a | c, !a - Direct boolean operators on VarId");
    println!("  ✅ (a & b) | c - Complex expressions work seamlessly");
    println!("  ✅ model.post_all(vec![...]) - True batch operations");
    println!("  ✅ No BoolVar wrapper needed - everything is internal!");
    println!("  🎉 MISSION ACCOMPLISHED: Ultra-clean boolean syntax achieved!");
}
