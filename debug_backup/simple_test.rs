use selen::prelude::*;

fn main() -> SolverResult<()> {
    println!("🔍 Simple test to debug statistics...");
    
    let mut m = Model::default();
    let x = m.int(1, 3);
    let y = m.int(1, 3);
    
    // Simple constraint
    post!(m, x != y);
    
    println!("Solving simple constraint: x != y where x,y ∈ [1,3]");
    let solution = m.solve()?;
    
    println!("✅ Solution found!");
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