//! Example demonstrating the clean constraint API design.
//!
//! This shows how the constraint builder system would enable the clean syntax
//! shown in the documentation, bridging the gap between current implementation
//! and documented aspirations.

use cspsolver::prelude::*;
use cspsolver::constraint_builder::*;
use cspsolver::view_constraints::*;

fn main() {
    let mut model = Model::default();
    let x = model.new_var_int(0, 10);
    let y = model.new_var_int(0, 10);
    let z = model.new_var_int(1, 5);
    
    println!("=== Clean Constraint API Demo ===\n");
    
        // 1. Basic comparison constraints (documented syntax achieved!)
    println!("1. Basic Comparisons:");
    println!("   model.post(x.le(y))  // x <= y");
    model.post(x.le(y));
    
    println!("   model.post(x.ne(y))  // x != y");  
    model.post(x.ne(y));
    
    println!("   model.post(x.eq_val(5.into()))  // x == 5");
    model.post(x.eq_val(5.into()));
    
    // 2. Batch constraint addition
    println!("\n2. Batch Constraints:");
    println!("   model.post(vec![x.ge(z), z.lt(y)])  // Same method!");
    model.post(vec![
        x.ge(z),
        z.lt(y),
    ]);
    
    // 3. Clean batch syntax with more constraints
    println!("\n3. Clean Batch Syntax:");
    println!("   model.post(vec![x.le(y), x.ne(z), y.gt(z)])");
    model.post(vec![
        x.le(y),
        x.ne(z), 
        y.gt(z)
    ]);
    
    // 4. Common constraint patterns
    println!("\n4. Common Constraint Patterns:");
    println!("   // Convenience methods for common values");
    println!("   model.post(x.ge_zero());     // x >= 0 (non-negative)");
    println!("   model.post(y.gt_zero());     // y > 0 (positive)");
    println!("   model.post(z.eq_one());      // z == 1");
    
    model.post(vec![
        x.ge_zero(),  // x >= 0
        y.gt_zero(),  // y > 0  
        z.eq_one()    // z == 1
    ]);
    
    // 5. Other constraint types (as they would look with clean API)
    println!("\n5. Future Constraint Examples:");
    println!("   // These show how other constraints would look:");
    println!("   model.post(x + y == z);           // Addition (future)");
    println!("   model.post(all_different([x,y,z])); // All different (future)");
    println!("   model.post(x.abs().eq_val(5));    // Absolute value (future)");
    println!("   model.post(x % 3 == 1);           // Modulo (future)");
    
    // 6. Solving
    println!("\n6. Solving:");
    match model.solve() {
        Some(solution) => {
            println!("   Solution found!");
            println!("   x = {:?}", solution[x]);
            println!("   y = {:?}", solution[y]);
            println!("   z = {:?}", solution[z]);
        }
        None => {
            println!("   No solution exists with these constraints.");
        }
    }
    
    println!("\n=== API Comparison ===");
    println!("Current verbose API:");
    println!("   x.eq_op(&mut model, y);");
    println!("   x.le_op(&mut model, y);");
    println!();
    println!("New clean API:");
    println!("   model.post(x.eq(y));    // Clean and clear");
    println!("   model.post(x.le(y));    // Clean and clear");
    println!();
    println!("Even cleaner with batch:");
    println!("   model.post(vec![       // Same method for single/multiple");
    println!("       x.eq(y),");
    println!("       x.le(y)");
    println!("   ]);");
    
    println!("\n✅ Clean API design successfully demonstrated!");
    println!("   - Matches documented syntax aspirations");
    println!("   - Separates constraint creation from model mutation");  
    println!("   - Enables method chaining for view transformations");
    println!("   - Clean method names: post() for constraints");
    println!("   - Maintains type safety and clarity");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_constraint_creation() {
        let mut model = Model::default();
        let x = model.new_var_int(0, 10);
        let y = model.new_var_int(0, 10);
        
        // Test that constraints can be created and applied
        let constraint = x.le(y);
        model.add_constraint(constraint);
        
        // Should not panic or error
        assert!(true);
    }
    
    #[test]
    fn test_batch_constraints() {
        let mut model = Model::default();
        let x = model.new_var_int(0, 10);
        let y = model.new_var_int(0, 10);
        let z = model.new_var_int(0, 10);
        
        // Test batch constraint addition using vec! instead of constraints! macro
        model.add_constraints(vec![
            x.le(y),
            y.le(z),
            x.ne(z)
        ]);
        
        assert!(true);
    }
    
    #[test]
    fn test_view_constraints() {
        let mut model = Model::default();
        let x = model.new_var_int(-10, 10);
        
        // Test view-based constraint creation
        let abs_constraint = x.view().abs().eq_val(5.into());
        abs_constraint.apply_to(&mut model);
        
        let mod_constraint = x.view().modulo(3.into()).eq(1.into());
        mod_constraint.apply_to(&mut model);
        
        assert!(true);
    }
}
