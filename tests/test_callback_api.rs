use cspsolver::prelude::*;
use std::time::Instant;

fn main() -> SolverResult<()> {
    println!("Testing embedded statistics API (replacing solve_with_callback)...");
    
    let start = Instant::now();
    let mut m = Model::default();
    let x = m.int(1, 9);
    let y = m.int(1, 9);
    
    post!(m, x != y);
    
    let solution = m.solve()?;
    
    let duration = start.elapsed();
    
    println!("✅ Success in {:?}!", duration);
    println!("Stats: {} propagations, {} nodes", 
             solution.stats.propagation_count, solution.stats.node_count);
    if let Val::ValI(x_val) = solution[x] {
        println!("x = {}", x_val);
    }
    if let Val::ValI(y_val) = solution[y] {
        println!("y = {}", y_val);
    }
    
    Ok(())
}