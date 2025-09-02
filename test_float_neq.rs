use cspsolver::prelude::*;

fn main() {
    println!("=== Testing not_equals with Float Variables ===");
    
    // Test 1: Basic float not_equals
    println!("\n1. Testing basic float not_equals:");
    {
        let mut m = Model::default();
        let x = m.new_var(Val::ValF(1.0), Val::ValF(5.0));
        let y = m.new_var(Val::ValF(1.0), Val::ValF(5.0));
        
        println!("  Created variables: x ∈ [1.0, 5.0], y ∈ [1.0, 5.0]");
        
        // Add not_equals constraint
        m.not_equals(x, y);
        
        let solution = m.solve();
        match solution {
            Some(sol) => {
                let x_val = sol[x];
                let y_val = sol[y];
                println!("  Solution found: x = {:?}, y = {:?}", x_val, y_val);
                if x_val == y_val {
                    println!("  ERROR: x and y have same value despite not_equals constraint!");
                } else {
                    println!("  SUCCESS: x and y have different values");
                }
            }
            None => println!("  No solution found"),
        }
    }
    
    // Test 2: Float with very narrow domains
    println!("\n2. Testing float not_equals with narrow domains:");
    {
        let mut m = Model::default();
        let x = m.new_var(Val::ValF(1.0), Val::ValF(1.1));
        let y = m.new_var(Val::ValF(1.0), Val::ValF(1.1));
        
        println!("  Created variables: x ∈ [1.0, 1.1], y ∈ [1.0, 1.1]");
        
        // Add not_equals constraint
        m.not_equals(x, y);
        
        let solution = m.solve();
        match solution {
            Some(sol) => {
                let x_val = sol[x];
                let y_val = sol[y];
                println!("  Solution found: x = {:?}, y = {:?}", x_val, y_val);
                
                // Check if values are actually different (within floating point precision)
                if let (Val::ValF(x_f), Val::ValF(y_f)) = (x_val, y_val) {
                    let diff = (x_f - y_f).abs();
                    println!("  Difference: |{} - {}| = {}", x_f, y_f, diff);
                    if diff < 1e-10 {
                        println!("  WARNING: Values are very close (potential precision issue)");
                    }
                }
            }
            None => println!("  No solution found"),
        }
    }
    
    // Test 3: Count solutions with float domains (this could be problematic)
    println!("\n3. Testing solution enumeration (limited to first 5):");
    {
        let mut m = Model::default();
        let x = m.new_var(Val::ValF(1.0), Val::ValF(2.0));
        let y = m.new_var(Val::ValF(1.0), Val::ValF(2.0));
        
        println!("  Created variables: x ∈ [1.0, 2.0], y ∈ [1.0, 2.0]");
        
        // Add not_equals constraint
        m.not_equals(x, y);
        
        let mut solution_count = 0;
        for sol in m.enumerate() {
            let x_val = sol[x];
            let y_val = sol[y];
            solution_count += 1;
            println!("  Solution {}: x = {:?}, y = {:?}", solution_count, x_val, y_val);
            
            if solution_count >= 5 {
                println!("  Stopping after 5 solutions to avoid infinite enumeration...");
                break;
            }
        }
        
        if solution_count == 0 {
            println!("  No solutions found");
        } else if solution_count >= 5 {
            println!("  Found at least {} solutions", solution_count);
        } else {
            println!("  Total solutions: {}", solution_count);
        }
    }
    
    println!("\n=== Float Test Complete ===");
}
