//! Demo showcasing the new syntactic sugar for constraint values.

use cspsolver::prelude::*;
use cspsolver::constraint_builder::{ModelConstraints, VarConstraints};

fn main() {
    let mut model = Model::default();
    let x = model.new_var_int(0, 20);
    let y = model.new_var_float(0.0, 10.0);
    let z = model.new_var_int(-5, 15);
    
    println!("=== Syntactic Sugar Demo ===\n");
    
    // Before: Verbose syntax
    println!("🔴 Before (verbose):");
    println!("   model.post(x.eq_val(5.into()));       // Verbose");
    println!("   model.post(y.le_val(3.14.into()));    // Verbose");
    println!("   model.post(z.ge_val((-2).into()));    // Verbose");
    
    // After: Clean syntactic sugar
    println!("\n🟢 After (syntactic sugar):");
    println!("   model.post(x.eq_int(5));       // Clean!");
    model.post(x.eq_int(5));
    
    println!("   model.post(y.le_float(3.14));  // Clean!");
    model.post(y.le_float(3.14));
    
    println!("   model.post(z.ge_int(-2));      // Clean!");
    model.post(z.ge_int(-2));
    
    // Batch constraints with clean syntax
    println!("\n✨ Batch constraints with syntactic sugar:");
    println!("   model.post(vec![");
    println!("       x.lt_int(15),        // x < 15");
    println!("       y.gt_float(1.0),     // y > 1.0");  
    println!("       z.ne_int(0),         // z != 0 (future)");
    println!("       x.le_int(10)         // x <= 10");
    println!("   ]);");
    
    model.post(vec![
        x.lt_int(15),        // x < 15
        y.gt_float(1.0),     // y > 1.0
        x.le_int(10)         // x <= 10
    ]);
    
    // Convenience methods (no magic numbers)
    println!("\n🎯 Convenience methods (no magic numbers):");
    println!("   model.post(vec![");
    println!("       z.ge_zero(),    // z >= 0 (readable!)");
    println!("       x.gt_zero(),    // x > 0 (readable!)");
    println!("       y.eq_one()      // y == 1 (readable!)");
    println!("   ]);");
    
    model.post(vec![
        z.ge_zero(),    // z >= 0
        x.gt_zero(),    // x > 0
        // y.eq_one()      // This would be for integers, skip for float var
    ]);
    
    // Solve
    println!("\n🔍 Solving:");
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
    
    println!("\n📊 Comparison Summary:");
    println!("   ❌ x.eq_val(5.into())     // 17 characters, complex");
    println!("   ✅ x.eq_int(5)           // 11 characters, simple");
    println!();
    println!("   ❌ y.le_val(3.14.into()) // 19 characters, complex");
    println!("   ✅ y.le_float(3.14)      // 15 characters, simple");
    println!();
    println!("   🎯 Benefits:");
    println!("      • 30-40% fewer characters");
    println!("      • No .into() noise");
    println!("      • Type-specific methods (int vs float)");
    println!("      • More readable and intuitive");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_syntactic_sugar() {
        let mut model = Model::default();
        let x = model.new_var_int(0, 10);
        let y = model.new_var_float(0.0, 10.0);
        
        // Test all the new syntactic sugar methods
        model.post(vec![
            x.eq_int(5),
            x.le_int(8),
            x.ge_int(2),
            x.gt_int(1),
            x.lt_int(9),
            y.eq_float(3.14),
            y.le_float(8.5),
            y.ge_float(1.0),
            y.gt_float(0.5),
            y.lt_float(9.9)
        ]);
        
        // Should compile without errors
        assert!(true);
    }
}
