//! Simple Magic Square test to debug constraints

use selen::prelude::*;

fn main() {
    println!("🔍 Testing basic 3x3 magic square constraints...");
    
    let mut model = Model::default();
    
    // Create 9 variables for 3x3 grid, values 1-9
    let vars: Vec<VarId> = (0..9).map(|_| model.new_var(Val::int(1), Val::int(9))).collect();
    
    // Test alldiff constraint
    post!(model, alldiff(vars.clone()));
    
    println!("Added alldiff constraint for {} variables", vars.len());
    
    // Test basic solve
    match model.solve() {
        Ok(solution) => {
            println!("✅ Found a solution!");
            for (i, &var) in vars.iter().enumerate() {
                if let Val::ValI(value) = solution[var] {
                    print!("{} ", value);
                    if (i + 1) % 3 == 0 {
                        println!();
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ No solution: {}", e);
        }
    }
}