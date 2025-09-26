//! Example demonstrating the Runtime Constraint API (Phase 1 & 2)
//!
//! This example shows how to build constraints programmatically at runtime
//! using the new ultra-short API without requiring compile-time knowledge.
//!
//! # Phase 1: Core Expression System
//! - ExprBuilder for mathematical expressions (x.add(y).eq(z))
//! - VarIdExt trait for direct variable methods
//! - ModelExt trait for posting constraints
//!
//! # Phase 2: Constraint Builder  
//! - Builder struct for fluent constraint building
//! - Model::c() ultra-short syntax (m.c(x).eq(5))
//! - Global constraint shortcuts (alldiff, alleq, elem, count)

use selen::prelude::*;

fn main() {
    println!("🚀 Runtime Constraint API Demo - Phase 1 & 2");
    println!("===============================================\n");

    // Phase 1 Examples
    example_1_basic_runtime_building();
    example_2_dynamic_from_data();
    example_3_expression_chaining();
    example_4_constraint_composition();
    
    // Phase 2 Examples  
    example_5_model_c_syntax();
    example_6_builder_fluent_interface();
    example_7_global_constraints();
    example_8_mixed_phase_usage();
}

/// Example 1: Basic runtime constraint building
fn example_1_basic_runtime_building() {
    println!("📝 Example 1: Basic Runtime Constraint Building");
    
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let z = m.int(0, 20);

    // Pure runtime constraint building - no compile-time syntax
    m.new(x.add(y).eq(z));      // x + y == z
    m.new(x.gt(int(5)));        // x > int(5)
    m.new(y.le(int(8)));        // y <= int(8)

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
            "gt" => var_id.gt(int(value)),
            "le" => var_id.le(int(value)), 
            "eq" => var_id.eq(int(value)),
            "ne" => var_id.ne(int(value)),
            "ge" => var_id.ge(int(value)),
            "lt" => var_id.lt(int(value)),
            _ => {
                println!("  ⚠️  Unknown operator: {}", op);
                continue;
            }
        };
        
        m.new(constraint);
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
    m.new(a.eq(int(5)));      // a = int(5)  
    m.new(b.eq(int(5)));      // b = int(5)
    m.new(result.eq(int(10))); // result = int(10)
    
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
    let c1 = x.gt(int(5));      // x > int(5)
    let c2 = x.lt(int(15));     // x < int(15)  
    let c3 = y.ge(int(10));     // y >= int(10)
    
    // Compose them: (x > 5 AND x < 15) AND y >= 10
    let range_constraint = c1.and(c2);
    let combined = range_constraint.and(c3);
    
    m.new(combined);
    
    if let Ok(solution) = m.solve() {
        println!("✓ Solution with composed constraints:");
        println!("  x = {:?} (should be 6-14)", solution[x]);
        println!("  y = {:?} (should be >= 10)", solution[y]);
    } else {
        println!("❌ No solution found");
    }
    println!();
}

// =================== PHASE 2 EXAMPLES ===================

/// Example 5: Model::c() ultra-short syntax
fn example_5_model_c_syntax() {
    println!("📝 Example 5: Model::c() Ultra-Short Syntax (Phase 2)");
    
    let mut m = Model::default();
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    let sum = m.int(2, 20);
    
    // Ultra-short Model::c() syntax - auto-posts constraints
    m.c(x).gt(int(3));                    // x > int(3)
    m.c(y).le(int(8));                    // y <= int(8)
    m.c(x).add(y).eq(sum);               // x + y == sum
    m.c(sum).eq(int(12));                // sum == int(12)
    
    if let Ok(solution) = m.solve() {
        println!("✓ Solution found:");
        println!("  x = {:?}", solution[x]);
        println!("  y = {:?}", solution[y]);
        println!("  sum = {:?}", solution[sum]);
    } else {
        println!("❌ No solution found");
    }
    println!();
}

/// Example 6: Builder fluent interface
fn example_6_builder_fluent_interface() {
    println!("📝 Example 6: Builder Fluent Interface (Phase 2)");
    
    let mut m = Model::default();
    let a = m.int(0, 20);
    let b = m.int(0, 20);
    let result = m.int(0, 100);
    
    // Complex expression building with fluent interface
    m.c(a).mul(int(3)).add(int(5)).le(int(50));         // a * int(3) + int(5) <= int(50)
    m.c(b).div(int(2)).sub(int(1)).ge(int(2));          // b / int(2) - int(1) >= int(2)
    m.c(a).add(b).mul(int(2)).eq(result);              // (a + b) * int(2) == result
    m.c(result).ne(int(20));                           // result != int(20)
    
    if let Ok(solution) = m.solve() {
        println!("✓ Complex fluent constraints satisfied:");
        println!("  a = {:?}", solution[a]);
        println!("  b = {:?}", solution[b]);
        println!("  result = {:?}", solution[result]);
    } else {
        println!("❌ No solution found");
    }
    println!();
}

/// Example 7: Global constraint shortcuts
fn example_7_global_constraints() {
    println!("📝 Example 7: Global Constraint Shortcuts (Phase 2)");
    
    let mut m = Model::default();
    let digits = m.ints(4, 1, 4);
    
    // Ultra-short global constraints
    m.alldiff(&digits);              // All digits must be different
    m.c(digits[0]).gt(digits[1]);    // First > Second
    m.c(digits[2]).add(digits[3]).eq(int(5)); // Third + Fourth == int(5)
    
    if let Ok(solution) = m.solve() {
        println!("✓ Global constraint solution:");
        for (i, &digit) in digits.iter().enumerate() {
            println!("  digit[{}] = {:?}", i, solution[digit]);
        }
    } else {
        println!("❌ No solution found");
    }
    println!();
}

/// Example 8: Mixed Phase 1 & 2 usage
fn example_8_mixed_phase_usage() {
    println!("📝 Example 8: Mixed Phase 1 & 2 Usage");
    
    let mut m = Model::default();
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    let z = m.int(1, 10);
    
    // Phase 1: Manual constraint creation and posting
    let constraint1 = x.add(y).gt(int(5));
    m.new(constraint1);
    
    // Phase 2: Auto-posting builder syntax
    m.c(y).mul(int(2)).le(z.add(int(3)));
    
    // Global constraints (Phase 2)
    m.alldiff(&[x, y, z]);
    
    // Phase 1: Complex constraint composition  
    let c1 = x.lt(int(8));
    let c2 = y.ge(int(2));
    let combined = c1.and(c2);
    m.new(combined);
    
    if let Ok(solution) = m.solve() {
        println!("✓ Mixed API solution:");
        println!("  x = {:?}", solution[x]);
        println!("  y = {:?}", solution[y]);
        println!("  z = {:?}", solution[z]);
    } else {
        println!("❌ No solution found");
    }
    println!();
}