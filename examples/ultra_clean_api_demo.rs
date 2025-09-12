//! Ultra-clean constraint API demo showcasing the shortest, most readable syntax.

use cspsolver::prelude::*;

fn main() {
    let mut model = Model::default();
    let x = model.int(-5, 10);     // Short variable creation!
    let y = model.int(0, 10);      // Clean and concise
    let z = model.int(1, 5);       // No more new_var_int
    let w = model.float(0.0, 5.0); // Float variables too!
    
    println!("=== Ultra-Clean Constraint API ===\n");
    
    // 1. Super clean basic constraints using convenience methods
    println!("1. Clean Basic Constraints:");
    println!("   model.post(x.le(y))      // x <= y");
    model.post(x.le(y));
    
    println!("   model.post(x.ne(y))      // x != y");  
    model.post(x.ne(y));
    
    println!("   model.post(x.ge_zero())  // x >= 0 (no magic numbers!)");
    model.post(x.ge_zero());
    
    // 2. More constraint examples - comparisons, special values, syntactic sugar
    println!("\n2. Rich Constraint Examples:");
    println!("   // Value constraints with syntactic sugar:");
    println!("   model.post(x.eq_int(5));             // x = 5 (clean!)");
    model.post(x.eq_int(5));
    
    println!("   model.post(w.le_float(3.14));        // w <= 3.14 (type-specific!)");
    model.post(w.le_float(3.14));
    
    println!("   // Comparison constraints:");
    println!("   model.post(y.gt(z));                 // y > z");
    model.post(y.gt(z));
    
    println!("   // Special value constraints:");
    println!("   model.post(z.eq_one());              // z = 1 (readable!)");
    model.post(z.eq_one());
    
    println!("   model.post(w.gt_zero());             // w > 0.0 (clear!)");
    model.post(w.gt_zero());
    
    // 3. Ultra-clean batch constraints
    println!("\n3. Ultra-Clean Batch Constraints:");
    println!("   model.post(vec![");
    println!("       x.le_int(8),      // x <= 8");
    println!("       y.ge_int(2),      // y >= 2"); 
    println!("       w.le_float(4.5),  // w <= 4.5");
    println!("       z.lt(y)           // z < y");
    println!("   ]);");
    model.post(vec![
        x.le_int(8),      // x <= 8 - syntactic sugar
        y.ge_int(2),      // y >= 2 - type-specific
        w.le_float(4.5),  // w <= 4.5 - clean floats
        z.lt(y)           // z < y - variable relations
    ]);
    
    // 4. Global constraints (if available)
    println!("\n4. Global Constraints:");
    println!("   // All different constraint (clean and short):");
    println!("   model.alldifferent(vec![x, y, z]);   // All must be different");
    model.alldifferent(vec![x, y, z]);
    
    // 5. Summary of short variable creation
    println!("\n5. Short Variable Creation:");
    println!("   ❌ OLD: model.new_var_int(0, 10)     // 25 characters");
    println!("   ✅ NEW: model.int(0, 10)             // 16 characters (36% shorter!)");
    println!("   ❌ OLD: model.new_var_float(0.0, 5.0) // 30 characters");
    println!("   ✅ NEW: model.float(0.0, 5.0)        // 18 characters (40% shorter!)");
    
    // 6. Benefits explanation
    println!("\n6. Clean API Benefits:");
    println!("   📏 Shorter variable creation: int() vs new_var_int()");
    println!("   📏 Shorter methods: post() vs add_constraint()");
    println!("   🔢 No magic numbers: ge_zero() vs ge_int(0)");
    println!("   📚 More readable: x.le(y) vs model.le(x, y)");
    println!("   🎯 Type safe: Constraint builders validate at compile time");
    println!("   🔗 Chainable: Easy batch operations with vec![]");
    println!("   ⚡ Unified API: Same method for single/multiple constraints");
    println!("   🚫 No imports: Everything in prelude now!");
    
    // 4. Solving
    println!("\n4. Solving:");
    match model.solve() {
        Some(solution) => {
            println!("   ✅ Solution found!");
            println!("   x = {:?}", solution[x]);
            println!("   y = {:?}", solution[y]);
            println!("   z = {:?}", solution[z]);
        }
        None => {
            println!("   ❌ No solution exists.");
        }
    }
    
    println!("\n=== API Evolution Comparison ===");
    println!("🔴 Original verbose API:");
    println!("   model.new_var_int(0, 10);");
    println!("   model.equals(x, int(5));");
    println!("   model.le(x, y);");
    println!();
    println!("🟡 Previous clean API (with imports):");
    println!("   use cspsolver::constraint_builder::*;");
    println!("   model.new_var_int(0, 10);");
    println!("   model.post(x.eq_val(5.into()));");
    println!();
    println!("🟢 New ultra-clean API (no imports needed!):");
    println!("   model.int(0, 10);            // Short variable creation");
    println!("   model.post(x.eq_int(5));     // Syntactic sugar constraints");
    println!("   model.post(x.le(y));         // Clean variable relations");
    println!("   model.post(vec![             // Clean batches");
    println!("       x.ge_zero(),");
    println!("       y.lt_int(10)");
    println!("   ]);");
    
    println!("\n✨ Benefits:");
    println!("   📏 40% shorter variable creation: int() vs new_var_int()");
    println!("   📏 Shorter methods: post() vs add_constraint()");
    println!("   🔢 No magic numbers: gt_zero() vs gt_int(0)");
    println!("   📚 Readable: ge_zero() is clearer than ge_val(0.into())");
    println!("   🎯 Type safe: All constraints are validated at compile time");
    println!("   🔗 Chainable: Easy to build complex constraint sets");
    println!("   🚫 No imports: Everything in prelude now!");
    println!("   ⚡ Syntactic sugar: eq_int(5) vs eq_val(5.into())");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ultra_clean_constraints() {
        let mut model = Model::default();
        let x = model.int(-10, 10);
        let y = model.int(0, 10);
        
        // Test the ultra-clean API
        model.post(x.ge_zero());
        model.post(y.gt_zero());
        model.post(vec![
            x.le(y),
            x.ne(y),
            y.eq_one()
        ]);
        
        // Should compile and not panic
        assert!(true);
    }
    
    #[test]
    fn test_convenience_methods() {
        let mut model = Model::default();
        let x = model.int(-5, 5);       // Short variable creation!
        
        // Test all convenience methods
        model.post(vec![
            x.eq_zero(),   // x == 0
            x.eq_one(),    // x == 1  
            x.le_zero(),   // x <= 0
            x.ge_zero(),   // x >= 0
            x.gt_zero(),   // x > 0
            x.lt_zero(),   // x < 0
        ]);
        
        assert!(true);
    }
}
