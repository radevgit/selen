//! PC Building Optimizer
//!
//! A practical example of constraint optimization - finding the best PC build 
//! within budget constraints. This example demonstrates:
//! - Binary decision variables for discrete choices
//! - Resource constraints (budget)
//! - Optimization objectives (maximize performance score)
//! - Real-world constraint satisfaction problem

use cspsolver::prelude::*;

fn main() {
    // Create a model for our PC building problem
    let mut m = Model::default();
    
    // How many monitors: at least 1, at most 3
    let n_monitors = m.new_var_int(1, 3);
    
    // Monitor specifications
    let monitor_price = int(100);
    let monitor_score = int(250);
    
    // GPU options: [budget, mid-range, high-end]
    let gpu_prices = [int(150), int(250), int(500)];
    let gpu_scores = [int(100), int(400), int(800)];
    
    // Binary variables: do we pick each GPU?
    let gpus: Vec<_> = m.new_vars_binary(gpu_prices.len()).collect();
    
    // Calculate total GPU price and score based on selection
    let gpu_price = m.sum_iter(
        gpus.iter()
            .zip(gpu_prices)
            .map(|(gpu, price)| gpu.times(price))
    );
    let gpu_score = m.sum_iter(
        gpus.iter()
            .zip(gpu_scores)
            .map(|(gpu, score)| gpu.times(score))
    );
    
    // Total build price and score
    let total_price = m.add(gpu_price, n_monitors.times(monitor_price));
    let total_score = m.add(gpu_score, n_monitors.times(monitor_score));
    
    // Constraints
    let n_gpus = m.sum(&gpus);
    m.equals(n_gpus, int(1)); // Exactly one GPU
    m.less_than_or_equals(total_price, int(600)); // Budget constraint
    
    // Find optimal solution
    let solution = m.maximize(total_score).unwrap();
    
    println!("🖥️  PC Building Optimizer Results");
    println!("================================");
    
    println!("Monitors: {}", match solution[n_monitors] { 
        Val::ValI(n) => n,
        _ => 0
    });
    
    let gpu_selection = solution.get_values_binary(&gpus);
    let gpu_names = ["Budget GPU", "Mid-range GPU", "High-end GPU"];
    for (i, &selected) in gpu_selection.iter().enumerate() {
        if selected {
            println!("GPU: {} (${}, {} points)", 
                gpu_names[i], 
                match gpu_prices[i] { Val::ValI(p) => p, _ => 0 },
                match gpu_scores[i] { Val::ValI(s) => s, _ => 0 }
            );
        }
    }
    
    println!("Total performance score: {}", match solution[total_score] { 
        Val::ValI(s) => s,
        _ => 0
    });
    println!("Total price: ${}", match solution[total_price] { 
        Val::ValI(p) => p,
        _ => 0
    });
    
    let budget = 600;
    let remaining = budget - match solution[total_price] { Val::ValI(p) => p, _ => 0 };
    println!("Remaining budget: ${}", remaining);
    
    println!("\n✅ Optimal build found within budget constraints!");
}
