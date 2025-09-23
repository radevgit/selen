//! Global Constraints Demo - Phase 4
//!
//! This example demonstrates all global constraints in the runtime API:
//! - alldiff: All variables must have different values
//! - alleq: All variables must have the same value
//! - elem: Element constraint (array[index] == value)
//! - count: Count occurrences of a value
//! - betw: Between constraint (min <= var <= max)
//! - atmost/atleast: Upper/lower bound constraints
//! - gcc: Global cardinality constraint

use selen::{
    model::Model,
    runtime_api::{ModelExt, VarIdExt},
};

fn main() {
    println!("🚀 Global Constraints Demo - Phase 4");
    println!("====================================\n");

    // Example 1: All Different (Classic N-Queens-like)
    println!("📝 Example 1: All Different Constraint");
    {
        let mut model = Model::default();
        let vars: Vec<_> = (0..4).map(|_| model.int(1, 4)).collect();
        
        // All variables must have different values
        model.alldiff(&vars);
        
        if let Ok(solution) = model.solve() {
            println!("✓ All different solution:");
            for (i, &var) in vars.iter().enumerate() {
                let value: i32 = solution.get(var);
                println!("  var[{}] = {}", i, value);
            }
        } else {
            println!("❌ No solution found");
        }
    }
    
    println!();

    // Example 2: All Equal
    println!("📝 Example 2: All Equal Constraint");
    {
        let mut model = Model::default();
        let vars: Vec<_> = (0..3).map(|_| model.int(1, 10)).collect();
        
        // All variables must have the same value
        model.alleq(&vars);
        
        // Add a constraint to make it more interesting
        model.new(vars[0].ge(5));
        
        if let Ok(solution) = model.solve() {
            println!("✓ All equal solution:");
            for (i, &var) in vars.iter().enumerate() {
                let value: i32 = solution.get(var);
                println!("  var[{}] = {}", i, value);
            }
        } else {
            println!("❌ No solution found");
        }
    }
    
    println!();

    // Example 3: Element Constraint
    println!("📝 Example 3: Element Constraint (array[index] == value)");
    {
        let mut model = Model::default();
        
        // Create an array of variables
        let array: Vec<_> = (0..5).map(|i| model.int(i * 10, i * 10 + 9)).collect();
        let index = model.int(0, 4);
        let value = model.int(0, 50);
        
        // Element constraint: array[index] == value
        model.elem(&array, index, value);
        
        if let Ok(solution) = model.solve() {
            println!("✓ Element constraint solution:");
            println!("  Array values:");
            for (i, &var) in array.iter().enumerate() {
                let val: i32 = solution.get(var);
                println!("    array[{}] = {}", i, val);
            }
            let idx: i32 = solution.get(index);
            let val: i32 = solution.get(value);
            println!("  index = {}, value = {}", idx, val);
            println!("  Verification: array[{}] = {} == {}", idx, solution.get::<i32>(array[idx as usize]), val);
        } else {
            println!("❌ No solution found");
        }
    }
    
    println!();

    // Example 4: Count Constraint
    println!("📝 Example 4: Count Constraint");
    {
        let mut model = Model::default();
        let vars: Vec<_> = (0..6).map(|_| model.int(1, 3)).collect();
        let count_result = model.int(0, 6);
        
        // Count how many variables have value 2
        model.count(&vars, 2, count_result);
        
        // Force exactly 3 variables to have value 2
        model.new(count_result.eq(3));
        
        if let Ok(solution) = model.solve() {
            println!("✓ Count constraint solution:");
            let mut count_2s = 0;
            for (i, &var) in vars.iter().enumerate() {
                let value: i32 = solution.get(var);
                if value == 2 { count_2s += 1; }
                println!("  var[{}] = {}", i, value);
            }
            let count: i32 = solution.get(count_result);
            println!("  Count of 2s: {} (should be 3)", count);
            println!("  Verification: actual count = {}", count_2s);
        } else {
            println!("❌ No solution found");
        }
    }
    
    println!();

    // Example 5: Between Constraint (Cardinality)
    println!("📝 Example 5: Between Constraint (Cardinality)");
    {
        let mut model = Model::default();
        let x = model.int(0, 100);
        let y = model.int(0, 100);
        let z = model.int(0, 100);
        
        // x must be between 10 and 20
        model.betw(x, 10, 20);
        
        // y must be at most 50
        model.atmost(y, 50);
        
        // z must be at least 75
        model.atleast(z, 75);
        
        // Add some relationships
        model.new(x.add(y).eq(z));
        
        if let Ok(solution) = model.solve() {
            println!("✓ Cardinality constraints solution:");
            let x_val: i32 = solution.get(x);
            let y_val: i32 = solution.get(y);
            let z_val: i32 = solution.get(z);
            println!("  x = {} (should be 10-20)", x_val);
            println!("  y = {} (should be ≤ 50)", y_val);
            println!("  z = {} (should be ≥ 75)", z_val);
            println!("  Verification: {} + {} = {}", x_val, y_val, z_val);
        } else {
            println!("❌ No solution found");
        }
    }
    
    println!();

    // Example 6: Global Cardinality Constraint (GCC)
    println!("📝 Example 6: Global Cardinality Constraint");
    {
        let mut model = Model::default();
        let vars: Vec<_> = (0..8).map(|_| model.int(1, 4)).collect();
        
        // We want to count 1s, 2s, 3s, and 4s
        let values = [1, 2, 3, 4];
        let counts: Vec<_> = (0..4).map(|_| model.int(0, 8)).collect();
        
        // Global cardinality constraint
        model.gcc(&vars, &values, &counts);
        
        // Add some constraints on the counts
        model.new(counts[0].eq(2)); // Exactly 2 ones
        model.new(counts[1].eq(3)); // Exactly 3 twos  
        model.new(counts[2].eq(2)); // Exactly 2 threes
        model.new(counts[3].eq(1)); // Exactly 1 four
        
        if let Ok(solution) = model.solve() {
            println!("✓ Global cardinality constraint solution:");
            
            // Show the variables
            print!("  vars: [");
            for (i, &var) in vars.iter().enumerate() {
                let value: i32 = solution.get(var);
                print!("{}", value);
                if i < vars.len() - 1 { print!(", "); }
            }
            println!("]");
            
            // Show the counts
            println!("  Value counts:");
            for (i, &count_var) in counts.iter().enumerate() {
                let count: i32 = solution.get(count_var);
                println!("    Value {}: {} times", values[i], count);
            }
            
            // Verify the counts manually
            println!("  Manual verification:");
            for value in 1..=4 {
                let actual_count = vars.iter()
                    .map(|&var| solution.get::<i32>(var))
                    .filter(|&v| v == value)
                    .count();
                println!("    Value {}: {} times", value, actual_count);
            }
        } else {
            println!("❌ No solution found");
        }
    }
    
    println!();

    // Example 7: Complex Global Constraint Composition
    println!("📝 Example 7: Complex Global Constraint Composition");
    {
        let mut model = Model::default();
        
        // Create a small scheduling problem with global constraints
        let tasks: Vec<_> = (0..4).map(|_| model.int(1, 10)).collect(); // Start times
        let resources: Vec<_> = (0..4).map(|_| model.int(1, 3)).collect(); // Resource assignments
        
        // All tasks must start at different times (no overlap)
        model.alldiff(&tasks);
        
        // Count resource usage - we have 3 resources, want balanced usage
        let resource_counts: Vec<_> = (0..3).map(|_| model.int(0, 4)).collect();
        model.gcc(&resources, &[1, 2, 3], &resource_counts);
        
        // Each resource should be used at least once
        for &count_var in &resource_counts {
            model.atleast(count_var, 1);
        }
        
        // Tasks should be reasonably spaced (between 1 and 8)
        for &task in &tasks {
            model.betw(task, 1, 8);
        }
        
        if let Ok(solution) = model.solve() {
            println!("✓ Complex scheduling solution:");
            
            println!("  Task schedule:");
            for (i, &task) in tasks.iter().enumerate() {
                let start_time: i32 = solution.get(task);
                let resource: i32 = solution.get(resources[i]);
                println!("    Task {} starts at time {} using resource {}", i, start_time, resource);
            }
            
            println!("  Resource usage:");
            for (i, &count_var) in resource_counts.iter().enumerate() {
                let count: i32 = solution.get(count_var);
                println!("    Resource {} used {} times", i + 1, count);
            }
        } else {
            println!("❌ No solution found");
        }
    }

    println!("\n🎉 Phase 4 Global Constraints Demo Complete!");
    println!("All constraint types working: alldiff, alleq, elem, count, betw, atmost, atleast, gcc");
}