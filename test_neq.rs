//! Test the not_equals constraint

use cspsolver::prelude::*;

fn main() {
    println!("🧪 Testing not_equals constraint");
    println!("================================");
    
    // Test 1: Basic not_equals with integer variables
    {
        println!("\n📋 Test 1: Basic not_equals with integers");
        let mut m = Model::default();
        
        let x = m.new_var_int(1, 5);
        let y = m.new_var_int(1, 5);
        
        // Add constraint: x != y
        m.not_equals(x, y);
        
        println!("Variables: x ∈ [1,5], y ∈ [1,5]");
        println!("Constraint: x ≠ y");
        
        // Find all solutions
        let solutions = m.enumerate();
        println!("Found {} solutions:", solutions.len());
        
        for (i, solution) in solutions.iter().enumerate() {
            let x_val = match solution[x] { Val::ValI(v) => v, _ => panic!() };
            let y_val = match solution[y] { Val::ValI(v) => v, _ => panic!() };
            println!("  Solution {}: x={}, y={}", i+1, x_val, y_val);
            
            // Verify constraint
            assert_ne!(x_val, y_val, "Constraint x ≠ y violated!");
        }
        
        // Should have 5*5 - 5 = 20 solutions (all pairs except (1,1), (2,2), ..., (5,5))
        assert_eq!(solutions.len(), 20, "Expected 20 solutions");
        println!("✅ Test 1 passed!");
    }
    
    // Test 2: not_equals with pre-assigned value
    {
        println!("\n📋 Test 2: not_equals with pre-assigned value");
        let mut m = Model::default();
        
        let x = m.new_var_int(1, 5);
        let y = m.new_var_int(3, 3); // y is fixed to 3
        
        // Add constraint: x != y (so x != 3)
        m.not_equals(x, y);
        
        println!("Variables: x ∈ [1,5], y = 3");
        println!("Constraint: x ≠ y (so x ≠ 3)");
        
        let solutions = m.enumerate();
        println!("Found {} solutions:", solutions.len());
        
        for (i, solution) in solutions.iter().enumerate() {
            let x_val = match solution[x] { Val::ValI(v) => v, _ => panic!() };
            let y_val = match solution[y] { Val::ValI(v) => v, _ => panic!() };
            println!("  Solution {}: x={}, y={}", i+1, x_val, y_val);
            
            // Verify constraint
            assert_ne!(x_val, y_val, "Constraint x ≠ y violated!");
            assert_ne!(x_val, 3, "x should not equal 3!");
        }
        
        // Should have 4 solutions: x ∈ {1, 2, 4, 5}
        assert_eq!(solutions.len(), 4, "Expected 4 solutions");
        println!("✅ Test 2 passed!");
    }
    
    // Test 3: Infeasible case
    {
        println!("\n📋 Test 3: Infeasible case");
        let mut m = Model::default();
        
        let x = m.new_var_int(5, 5); // x is fixed to 5
        let y = m.new_var_int(5, 5); // y is fixed to 5
        
        // Add constraint: x != y (but both are fixed to 5, so this should be infeasible)
        m.not_equals(x, y);
        
        println!("Variables: x = 5, y = 5");
        println!("Constraint: x ≠ y (should be infeasible)");
        
        let solution = m.solve();
        
        if solution.is_none() {
            println!("✅ Correctly detected infeasibility!");
        } else {
            panic!("❌ Should have been infeasible!");
        }
        println!("✅ Test 3 passed!");
    }
    
    // Test 4: With callback to see solver statistics
    {
        println!("\n📋 Test 4: Solver statistics");
        let mut m = Model::default();
        
        let x = m.new_var_int(1, 10);
        let y = m.new_var_int(1, 10);
        let z = m.new_var_int(1, 10);
        
        // Add constraints: x != y and y != z
        m.not_equals(x, y);
        m.not_equals(y, z);
        
        println!("Variables: x,y,z ∈ [1,10]");
        println!("Constraints: x ≠ y and y ≠ z");
        
        let mut stats = SolveStats::default();
        let solution = m.solve_with_callback(|solve_stats| {
            stats = *solve_stats;
            println!("Propagation steps: {}", solve_stats.propagation_count);
            println!("Search nodes: {}", solve_stats.node_count);
        });
        
        if let Some(sol) = solution {
            let x_val = match sol[x] { Val::ValI(v) => v, _ => panic!() };
            let y_val = match sol[y] { Val::ValI(v) => v, _ => panic!() };
            let z_val = match sol[z] { Val::ValI(v) => v, _ => panic!() };
            
            println!("Found solution: x={}, y={}, z={}", x_val, y_val, z_val);
            
            // Verify constraints
            assert_ne!(x_val, y_val, "Constraint x ≠ y violated!");
            assert_ne!(y_val, z_val, "Constraint y ≠ z violated!");
            
            println!("✅ Test 4 passed!");
        } else {
            panic!("❌ Should have found a solution!");
        }
    }
    
    println!("\n🎉 All tests passed! The not_equals constraint is working correctly.");
    println!("The constraint properly:");
    println!("  • Enforces x ≠ y for different values");
    println!("  • Detects infeasibility when both variables must be equal");
    println!("  • Works with propagation and search");
    println!("  • Handles integer domains correctly");
}
