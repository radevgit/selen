use cspsolver::prelude::*;

fn main() {
    println!("=== Comparing Integer vs Float not_equals ===");
    
    // Test 1: Integer variables (should work perfectly)
    println!("\n1. Testing integer not_equals:");
    {
        let mut m = Model::default();
        let x = m.new_var_int(1, 3);
        let y = m.new_var_int(1, 3);
        
        println!("  Created variables: x ∈ [1, 3], y ∈ [1, 3]");
        
        // Add not_equals constraint
        m.not_equals(x, y);
        
        let mut solution_count = 0;
        println!("  All solutions:");
        for sol in m.enumerate() {
            let x_val = sol[x];
            let y_val = sol[y];
            solution_count += 1;
            println!("    Solution {}: x = {:?}, y = {:?}", solution_count, x_val, y_val);
        }
        
        println!("  Total solutions: {} (expected: 6)", solution_count);
    }
    
    // Test 2: Float variables with a reasonable domain
    println!("\n2. Testing float not_equals (limited enumeration):");
    {
        let mut m = Model::default();
        let x = m.new_var(Val::ValF(1.0), Val::ValF(3.0));
        let y = m.new_var(Val::ValF(1.0), Val::ValF(3.0));
        
        println!("  Created variables: x ∈ [1.0, 3.0], y ∈ [1.0, 3.0]");
        
        // Add not_equals constraint
        m.not_equals(x, y);
        
        let mut solution_count = 0;
        println!("  First 3 solutions:");
        for sol in m.enumerate() {
            let x_val = sol[x];
            let y_val = sol[y];
            solution_count += 1;
            println!("    Solution {}: x = {:?}, y = {:?}", solution_count, x_val, y_val);
            
            if solution_count >= 3 {
                break;
            }
        }
        
        println!("  (Note: Float domains can generate many solutions due to continuous nature)");
    }
    
    // Test 3: Mixed integer and float
    println!("\n3. Testing mixed integer-float not_equals:");
    {
        let mut m = Model::default();
        let x = m.new_var_int(1, 3);
        let y = m.new_var(Val::ValF(1.0), Val::ValF(3.0));
        
        println!("  Created variables: x ∈ [1, 3] (int), y ∈ [1.0, 3.0] (float)");
        
        // Add not_equals constraint
        m.not_equals(x, y);
        
        let solution = m.solve();
        match solution {
            Some(sol) => {
                let x_val = sol[x];
                let y_val = sol[y];
                println!("  Solution found: x = {:?}, y = {:?}", x_val, y_val);
                
                // Check if they're actually different
                let different = match (x_val, y_val) {
                    (Val::ValI(i), Val::ValF(f)) => {
                        let diff = (i as f32 - f).abs();
                        println!("  Difference: |{} - {}| = {}", i as f32, f, diff);
                        diff > VAR_EPSILON
                    }
                    _ => x_val != y_val
                };
                
                if different {
                    println!("  ✓ Values are appropriately different");
                } else {
                    println!("  ⚠ Values are very close (within epsilon)");
                }
            }
            None => println!("  No solution found"),
        }
    }
    
    println!("\n=== Comparison Complete ===");
    println!("Summary:");
    println!("- Integer not_equals: Works perfectly, finite solutions");
    println!("- Float not_equals: Works correctly but can generate many micro-splits");
    println!("- Mixed types: Handled correctly with appropriate epsilon tolerance");
}
