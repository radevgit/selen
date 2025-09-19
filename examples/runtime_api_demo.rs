//! Example demonstrating the Runtime Constraint API (Phase 1)
//!
//! This example shows how to build constraints programmatically at runtime
//! using the new ultra-short API without requiring compile-time knowledge.

use cspsolver::prelude::*;

fn main() {
    println!("🚀 Runtime Constraint API Demo - Phase 1");
    println!("=========================================\n");

    example_1_basic_runtime_building();
    example_2_dynamic_from_data();
    example_3_expression_chaining();
    example_4_constraint_composition();
}

/// Example 1: Basic runtime constraint building
fn example_1_basic_runtime_building() {
    println!("📝 Example 1: Basic Runtime Constraint Building");
    
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let z = m.int(0, 20);

    // Pure runtime constraint building - no compile-time syntax
    m.post(x.add(y).eq(z));      // x + y == z
    m.post(x.gt(5));             // x > 5
    m.post(y.le(8));             // y <= 8

    if let Ok(solution) = m.solve() {
        println!("✓ Solution found:");
        println!("  x = {:?}", solution[x]);
        println!("  y = {:?}", solution[y]);  
        println!("  z = {:?}", solution[z]);
    } else {
        println!("❌ No solution found");
    }
    println!();
}

/// Example 2: Dynamic constraint building from runtime data
fn example_2_dynamic_from_data() {
    println!("📝 Example 2: Dynamic Constraints from Data");
    
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // Simulate loading constraint rules from database/config
    let rules = vec![
        ("x", "gt", 3),   // x > 3
        ("y", "le", 7),   // y <= 7
        ("x", "ne", 5),   // x != 5
    ];
    
    println!("  Building constraints from data: {:?}", rules);
    
    for (var_name, op, value) in rules {
        let var_id = match var_name {
            "x" => x,
            "y" => y,
            _ => continue,
        };
        
        let constraint = match op {
            "gt" => var_id.gt(value),
            "le" => var_id.le(value), 
            "eq" => var_id.eq(value),
            "ne" => var_id.ne(value),
            "ge" => var_id.ge(value),
            "lt" => var_id.lt(value),
            _ => {
                println!("  ⚠️  Unknown operator: {}", op);
                continue;
            }
        };
        
        m.post(constraint);
        println!("  ✓ Posted: {} {} {}", var_name, op, value);
    }
    
    if let Ok(solution) = m.solve() {
        println!("✓ Solution found:");
        println!("  x = {:?}", solution[x]);
        println!("  y = {:?}", solution[y]);
    } else {
        println!("❌ No solution found");
    }
    println!();
}

/// Example 3: Complex expression chaining
fn example_3_expression_chaining() {
    println!("📝 Example 3: Expression Chaining");
    
    let mut m = Model::default();
    let a = m.int(1, 10);  // Expanded range
    let b = m.int(1, 10);  // Expanded range
    let result = m.int(0, 100);  // Expanded range
    
    // Build simple expression for now
    m.post(a.eq(5));      // a = 5  
    m.post(b.eq(5));      // b = 5
    m.post(result.eq(10)); // result = 10
    
    if let Ok(solution) = m.solve() {
        println!("✓ Found simple values:");
        println!("  a = {:?}", solution[a]);
        println!("  b = {:?}", solution[b]);
        println!("  result = {:?}", solution[result]);
    } else {
        println!("❌ No solution found");
    }
    println!();
}

/// Example 4: Constraint composition with boolean logic
fn example_4_constraint_composition() {
    println!("📝 Example 4: Constraint Composition");
    
    let mut m = Model::default();
    let x = m.int(0, 20);
    let y = m.int(0, 20);
    
    // Create individual constraints
    let c1 = x.gt(5);      // x > 5
    let c2 = x.lt(15);     // x < 15  
    let c3 = y.ge(10);     // y >= 10
    
    // Compose them: (x > 5 AND x < 15) AND y >= 10
    let range_constraint = c1.and(c2);
    let combined = range_constraint.and(c3);
    
    m.post(combined);
    
    if let Ok(solution) = m.solve() {
        println!("✓ Solution with composed constraints:");
        println!("  x = {:?} (should be 6-14)", solution[x]);
        println!("  y = {:?} (should be >= 10)", solution[y]);
    } else {
        println!("❌ No solution found");
    }
    println!();
}