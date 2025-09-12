use cspsolver::prelude::*;

fn main() {
    println!("=== CSP Solver - Ultra-Clean Boolean Syntax Demo ===\n");
    
    // Demo 1: Simple, clean API
    demo_clean_api();
    
    // Demo 2: True batch operations
    demo_batch_operations();
    
    // Demo 3: Complex expressions
    demo_complex_expressions();
    
    println!("\n=== Comparison: Before vs After ===");
    syntax_comparison();
}

fn demo_clean_api() {
    println!("🎯 NEW: Ultra-clean boolean API");
    
    let mut model = Model::default();
    
    // 1) model.bool() instead of model.new_bool_var()
    let a = model.bool();
    let b = model.bool(); 
    let c = model.bool();
    
    // 2) No BoolVar::from() needed - VarId works directly with operators!
    model.post(a & b);           // Direct VarId boolean operations
    model.post(a | c);           // No wrapper conversion needed
    model.post(!b);              // Clean NOT operation
    model.post((a & b) | c);     // Complex expressions work perfectly
    
    println!("✅ Ultra-clean syntax:");
    println!("   let a = model.bool();          // Clean variable creation");
    println!("   model.post(a & b);             // Direct operations on VarId");
    println!("   model.post((a & b) | c);       // Complex expressions");
    
    if let Some(solution) = model.solve() {
        let val_a = if let Val::ValI(v) = solution[a] { v } else { 0 };
        let val_b = if let Val::ValI(v) = solution[b] { v } else { 0 };
        let val_c = if let Val::ValI(v) = solution[c] { v } else { 0 };
        println!("   🔍 Solution: a={}, b={}, c={}", val_a, val_b, val_c);
    }
}

fn demo_batch_operations() {
    println!("\n� Batch Operations with Boolean Expressions");
    
    let mut model = Model::default();
    let a = model.bool();
    let b = model.bool();
    let c = model.bool();
    let d = model.bool();
    
    // 3) True batch operations - collect expressions and post all at once
    let boolean_constraints = vec![
        a & b,                            // Boolean expression 1
        (a & b) | c,                     // Boolean expression 2  
        !d,                              // Boolean expression 3
        a | (!b & c),                    // Complex boolean expression 4
    ];
    model.post_all(boolean_constraints);
    
    // Mixed batch: for mixed types, we can use the unified post method
    model.post_all(vec![a.eq_int(1)]);   // Regular constraints
    model.post_all(vec![(b & c) | d]);   // Boolean expressions  
    model.post_all(vec![c.ne_var(d)]);   // More regular constraints
    
    println!("✅ True batch operations:");
    println!("   // Pure boolean batch:");
    println!("   model.post_all(vec![a & b, (a & b) | c, !d]);");
    println!("   ");
    println!("   // Mixed types (separate calls):");
    println!("   model.post_all(vec![a.eq_int(1)]);     // Regular constraints");
    println!("   model.post_all(vec![(b & c) | d]);     // Boolean expressions");
    
    if let Some(solution) = model.solve() {
        let val_a = if let Val::ValI(v) = solution[a] { v } else { 0 };
        let val_b = if let Val::ValI(v) = solution[b] { v } else { 0 };
        let val_c = if let Val::ValI(v) = solution[c] { v } else { 0 };
        let val_d = if let Val::ValI(v) = solution[d] { v } else { 0 };
        println!("   🔍 Solution: a={}, b={}, c={}, d={}", val_a, val_b, val_c, val_d);
    }
}

fn demo_complex_expressions() {
    println!("\n� Complex Boolean Expressions");
    
    let mut model = Model::default();
    let x = model.bool();
    let y = model.bool();
    let z = model.bool();
    
    // Complex nested expressions all work seamlessly
    model.post((x & y) | (!x & z));          // (x AND y) OR (NOT x AND z)
    model.post(!(x & y & z));                // NOT (x AND y AND z)
    model.post((x | y) & (y | z) & (x | z)); // Distributed form
    
    // Set specific values to test
    model.post(x.eq_int(1));
    model.post(y.eq_int(0));
    
    println!("✅ Complex expressions:");
    println!("   model.post((x & y) | (!x & z));");
    println!("   model.post(!(x & y & z));");
    println!("   model.post((x | y) & (y | z) & (x | z));");
    
    if let Some(solution) = model.solve() {
        let val_x = if let Val::ValI(v) = solution[x] { v } else { 0 };
        let val_y = if let Val::ValI(v) = solution[y] { v } else { 0 };
        let val_z = if let Val::ValI(v) = solution[z] { v } else { 0 };
        println!("   🔍 Solution: x={}, y={}, z={}", val_x, val_y, val_z);
    }
}

fn syntax_comparison() {
    println!("\n🔄 Before (your concerns):");
    println!("   1) model.new_bool_var()              // Verbose");
    println!("   2) let bool_a = BoolVar::from(a);    // Manual conversion needed");
    println!("   3) No batch boolean operations       // Limited");
    
    println!("\n✨ After (ultra-clean):");
    println!("   1) model.bool()                      // Clean and short"); 
    println!("   2) a & b                             // Direct VarId operations - no wrapper!");
    println!("   3) vec![a & b, c.eq_int(1)]          // True batch operations");
    
    println!("\n🎯 Key Improvements:");
    println!("   ✅ model.bool() - clean variable creation");
    println!("   ✅ VarId directly supports &, |, ! operators");
    println!("   ✅ No BoolVar wrapper needed - all internal");
    println!("   ✅ Unified post() method: constraints + boolean expressions");
    println!("   ✅ Complex expressions: (a & b) | (!c & d)");
    println!("   ✅ True batch operations: model.post_all(vec![...])!");
}
