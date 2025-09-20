//! Advanced Constraint Types Example
//!
//! This example demonstrates advanced constraint types that were added in Step 9.1:
//! - Range constraints (betw) for value bounds
//! - Cardinality constraints (at_least, at_most, exactly) for counting values
//! - Runtime API usage vs traditional post! macros
//!
//! Focus: User-friendly APIs instead of internal implementation details

use cspsolver::prelude::*;
use cspsolver::runtime_api::{ModelExt, VarIdExt};

fn main() {
    println!("🚀 Advanced Constraint Types Example\n");
    
    // Example 1: Range Constraints (Runtime API)
    range_constraints_example();
    
    // Example 2: Cardinality Constraints (post! macros)
    cardinality_constraints_example();
    
    // Example 3: Mixed constraint approaches
    mixed_constraint_example();
    
    println!("\n✅ Advanced constraint types demonstration complete!");
    println!("Key takeaway: Use runtime API (m.betw, m.atleast, etc.) for dynamic constraints,");
    println!("              Use post! macros for static constraint patterns.");
}

/// Demonstrate range constraints using the runtime API
fn range_constraints_example() {
    println!("=== 1. Range Constraints (Runtime API) ===");
    let mut m = Model::default();
    
    let temperature = m.int(-10, 50);  // Celsius temperature
    let humidity = m.int(0, 100);      // Percentage
    
    // Runtime API: Comfortable temperature range
    m.betw(temperature, 18, 25);
    println!("✓ Temperature between 18-25°C (m.betw runtime API)");
    
    // Runtime API: Optimal humidity range
    m.betw(humidity, 40, 60);
    println!("✓ Humidity between 40-60% (m.betw runtime API)");
    
    if let Ok(solution) = m.solve() {
        println!("  → Comfort solution: {}°C, {}% humidity", 
                solution.get_int(temperature), solution.get_int(humidity));
    }
    println!();
}

/// Demonstrate cardinality constraints using post! macros
fn cardinality_constraints_example() {
    println!("=== 2. Cardinality Constraints (post! macros) ===");
    let mut m = Model::default();
    
    // Work schedule: 5 days, 3 shift types (0=morning, 1=afternoon, 2=night)
    let schedule = vec![
        m.int(0, 2), m.int(0, 2), m.int(0, 2), 
        m.int(0, 2), m.int(0, 2)
    ];
    
    // At least 2 morning shifts (value 0)
    post!(m, at_least(schedule.clone(), 0, 2));
    println!("✓ At least 2 morning shifts required");
    
    // At most 1 night shift (value 2) 
    post!(m, at_most(schedule.clone(), 2, 1));
    println!("✓ At most 1 night shift allowed");
    
    // Exactly 2 afternoon shifts (value 1)
    post!(m, exactly(schedule.clone(), 1, 2));
    println!("✓ Exactly 2 afternoon shifts required");
    
    if let Ok(solution) = m.solve() {
        let shifts = ["morning", "afternoon", "night"];
        println!("  → Schedule solution:");
        for (day, &var) in schedule.iter().enumerate() {
            let shift_id = solution.get_int(var);
            println!("    Day {}: {} shift", day+1, shifts[shift_id as usize]);
        }
    }
    println!();
}

/// Demonstrate mixing runtime API with traditional constraints
fn mixed_constraint_example() {
    println!("=== 3. Mixed Constraint Approaches ===");
    let mut m = Model::default();
    
    let budget = m.int(0, 1000);
    let quality = m.int(1, 10);
    let urgency = m.int(1, 5);
    
    // Runtime API: Budget constraints
    m.betw(budget, 100, 500);
    println!("✓ Budget between $100-500 (runtime API)");
    
    // Traditional post!: Quality requirements
    post!(m, quality >= 7);
    println!("✓ Quality ≥ 7/10 (traditional constraint)");
    
    // Runtime API: At least a certain urgency
    m.atleast(urgency, 3);
    println!("✓ Urgency ≥ 3/5 (runtime API)");
    
    // Complex relationship: Higher quality costs more using runtime API
    let cost_per_quality = m.int(50, 50);
    let total_cost = m.mul(quality, cost_per_quality);
    m.post(total_cost.le(budget));
    println!("✓ quality × $50 ≤ budget (cost relationship)");
    
    if let Ok(solution) = m.solve() {
        let b = solution.get_int(budget);
        let q = solution.get_int(quality);
        let u = solution.get_int(urgency);
        
        println!("  → Project solution:");
        println!("    Budget: ${}", b);
        println!("    Quality: {}/10", q);
        println!("    Urgency: {}/5", u);
        println!("    Quality cost: ${} ({}% of budget)", q * 50, (q * 50 * 100) / b);
    }
    println!();
}