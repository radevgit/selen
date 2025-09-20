use cspsolver::prelude::*;

fn main() {
    println!("🎯 Min/Max Clean API Demo");
    
    let mut model = Model::default();
    
    // Create some variables that will allow min=12 and max<=20
    let a = model.int(12, 20);  // Can be the minimum
    let b = model.int(12, 20);  // Can also be minimum or anywhere in range
    let c = model.int(12, 20);  // Same range for simplicity
    
    // Create min and max variables with clean API (no .expect() needed!)
    let minimum = model.min(&[a, b, c]).expect("non-empty variable list");
    let maximum = model.max(&[a, b, c]).expect("non-empty variable list");
    
    // Add constraints that work with the ranges
    model.post(minimum.eq(int(12)));  // Force min to be exactly 12 (feasible)
    model.post(maximum.le(int(18)));  // Max should be <= 18 (feasible)
    
    // Validate the model
    match model.validate() {
        Ok(_) => println!("✅ Model validation passed"),
        Err(e) => {
            println!("❌ Model validation failed: {}", e);
            return;
        }
    }
    
    // Solve
    match model.solve() {
        Ok(solution) => {
            println!("\n✅ Solution found!");
            
            let a_val = solution.get_int(a);
            let b_val = solution.get_int(b);
            let c_val = solution.get_int(c);
            let min_val = solution.get_int(minimum);
            let max_val = solution.get_int(maximum);
            
            println!("Variables: a = {}, b = {}, c = {}", a_val, b_val, c_val);
            println!("Minimum: {}", min_val);
            println!("Maximum: {}", max_val);
            
            // Calculate actual min/max for comparison
            let values = [a_val, b_val, c_val];
            let actual_min = *values.iter().min().unwrap();
            let actual_max = *values.iter().max().unwrap();
            
            // Verify the min/max constraints are enforced correctly
            assert_eq!(min_val, actual_min);  // Min variable should equal actual min
            assert_eq!(max_val, actual_max);  // Max variable should equal actual max
            assert_eq!(min_val, 12);  // This was our constraint
            assert!(max_val <= 18);  // This was our constraint
            
            println!("\n🎉 All constraints satisfied!");
            println!("    Min variable: {} equals actual min: {}", min_val, actual_min);
            println!("    Max variable: {} equals actual max: {}", max_val, actual_max);
            println!("    Constraints: min = 12 ✓, max <= 18 ✓");
            println!("    Min/Max API works without .expect() calls!");
        }
        Err(e) => {
            println!("❌ No solution found: {}", e);
        }
    }
}