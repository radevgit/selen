//! Test the enhanced not_equals constraint with custom branching

use cspsolver::prelude::*;

fn main() {
    println!("🧪 Testing Enhanced not_equals Constraint");
    println!("=========================================");
    
    // Test 1: Basic not_equals constraint
    {
        println!("\n📋 Test 1: Basic not_equals with integers");
        let mut m = Model::default();
        
        let x = m.new_var_int(1, 3);
        let y = m.new_var_int(1, 3);
        
        // Add constraint: x != y
        m.not_equals(x, y);
        
        println!("Variables: x ∈ [1,3], y ∈ [1,3]");
        println!("Constraint: x ≠ y");
        
        // Find all solutions
        let solutions: Vec<_> = m.enumerate().collect();
        println!("Found {} solutions:", solutions.len());
        
        for (i, solution) in solutions.iter().enumerate() {
            let x_val = match solution[x] { Val::ValI(v) => v, _ => panic!() };
            let y_val = match solution[y] { Val::ValI(v) => v, _ => panic!() };
            println!("  Solution {}: x={}, y={}", i+1, x_val, y_val);
            
            // Verify constraint
            assert_ne!(x_val, y_val, "Constraint x ≠ y violated!");
        }
        
        // Should have 3*3 - 3 = 6 solutions (all pairs except (1,1), (2,2), (3,3))
        assert_eq!(solutions.len(), 6, "Expected 6 solutions");
        println!("✅ Test 1 passed!");
    }
    
    // Test 2: not_equals with propagation
    {
        println!("\n📋 Test 2: not_equals with propagation");
        let mut m = Model::default();
        
        let x = m.new_var_int(1, 5);
        let y = m.new_var_int(3, 3); // y is fixed to 3
        
        // Add constraint: x != y (so x != 3)
        m.not_equals(x, y);
        
        println!("Variables: x ∈ [1,5], y = 3");
        println!("Constraint: x ≠ y (so x ≠ 3)");
        
        let mut stats = SolveStats::default();
        let solution = m.solve_with_callback(|solve_stats| {
            stats = *solve_stats;
            println!("Propagation steps: {}", solve_stats.propagation_count);
            println!("Search nodes: {}", solve_stats.node_count);
        });
        
        if let Some(sol) = solution {
            let x_val = match sol[x] { Val::ValI(v) => v, _ => panic!() };
            let y_val = match sol[y] { Val::ValI(v) => v, _ => panic!() };
            println!("Found solution: x={}, y={}", x_val, y_val);
            
            // Verify constraint
            assert_ne!(x_val, y_val, "Constraint x ≠ y violated!");
            assert_ne!(x_val, 3, "x should not equal 3!");
            
            println!("✅ Test 2 passed!");
        } else {
            panic!("❌ Should have found a solution!");
        }
    }
    
    // Test 3: Multiple not_equals constraints (simulating all_different)
    {
        println!("\n📋 Test 3: Multiple not_equals constraints");
        let mut m = Model::default();
        
        let x = m.new_var_int(1, 4);
        let y = m.new_var_int(1, 4);
        let z = m.new_var_int(1, 4);
        
        // Add constraints: x != y, y != z, x != z (all different)
        m.not_equals(x, y);
        m.not_equals(y, z);
        m.not_equals(x, z);
        
        println!("Variables: x,y,z ∈ [1,4]");
        println!("Constraints: x ≠ y, y ≠ z, x ≠ z (all different)");
        
        let solutions = m.enumerate_with_callback(|solve_stats| {
            println!("Final propagation steps: {}", solve_stats.propagation_count);
            println!("Final search nodes: {}", solve_stats.node_count);
        });
        
        println!("Found {} solutions:", solutions.len());
        
        for (i, solution) in solutions.iter().take(5).enumerate() {
            let x_val = match solution[x] { Val::ValI(v) => v, _ => panic!() };
            let y_val = match solution[y] { Val::ValI(v) => v, _ => panic!() };
            let z_val = match solution[z] { Val::ValI(v) => v, _ => panic!() };
            println!("  Solution {}: x={}, y={}, z={}", i+1, x_val, y_val, z_val);
            
            // Verify all constraints
            assert_ne!(x_val, y_val, "Constraint x ≠ y violated!");
            assert_ne!(y_val, z_val, "Constraint y ≠ z violated!");
            assert_ne!(x_val, z_val, "Constraint x ≠ z violated!");
        }
        
        if solutions.len() > 5 {
            println!("  ... and {} more solutions", solutions.len() - 5);
        }
        
        // Should have 4*3*2 = 24 solutions (4 choices for x, 3 for y, 2 for z)
        assert_eq!(solutions.len(), 24, "Expected 24 solutions for all_different");
        println!("✅ Test 3 passed!");
    }
    
    println!("\n🎉 All tests passed! The enhanced not_equals constraint is working.");
    println!("💡 This demonstrates the foundation for:");
    println!("  • Efficient Sudoku solving (all_different constraints)");
    println!("  • Custom branching around forbidden values");
    println!("  • Creating domain 'holes' through search branching");
}
