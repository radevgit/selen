use cspsolver::prelude::*;
use std::time::Instant;

fn main() -> SolverResult<()> {
    println!("Testing solve_with_callback API...");
    
    let start = Instant::now();
    let mut m = Model::default();
    let x = m.int(1, 9);
    let y = m.int(1, 9);
    
    post!(m, x != y);
    
    let mut prop_count = 0;
    let mut node_count = 0;
    
    let solution = m.solve_with_callback(|stats| {
        prop_count = stats.propagation_count;
        node_count = stats.node_count;
        
        // Print progress to see if we're making progress
        if node_count % 100 == 0 || prop_count % 1000 == 0 {
            println!("Progress: {} propagations, {} nodes", prop_count, node_count);
        }
    });
    
    let duration = start.elapsed();
    
    match solution {
        Ok(sol) => {
            println!("✅ Success in {:?}!", duration);
            println!("Stats: {} propagations, {} nodes", prop_count, node_count);
            if let Val::ValI(x_val) = sol[x] {
                println!("x = {}", x_val);
            }
            if let Val::ValI(y_val) = sol[y] {
                println!("y = {}", y_val);
            }
        }
        Err(e) => {
            println!("❌ Error after {:?}: {:?}", duration, e);
        }
    }
    
    Ok(())
}