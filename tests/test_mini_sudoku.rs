use cspsolver::prelude::*;
use std::time::Instant;

fn main() -> SolverResult<()> {
    println!("Testing minimal sudoku...");
    
    let start = Instant::now();
    let mut m = Model::default();
    
    // Create a 2x2 mini-sudoku for testing
    let mut grid = [[m.int(1, 2); 2]; 2];
    
    // Row constraints: each row has all digits 1-2
    for row in 0..2 {
        post!(m, alldiff(grid[row]));
    }
    
    // Column constraints: each column has all digits 1-2
    for col in 0..2 {
        let column = [grid[0][col], grid[1][col]];
        post!(m, alldiff(column));
    }
    
    println!("Constraints added, solving...");
    
    let mut prop_count = 0;
    let mut node_count = 0;
    
    let solution = m.solve_with_callback(|stats| {
        prop_count = stats.propagation_count;
        node_count = stats.node_count;
        
        // Print progress 
        if node_count % 10 == 0 && node_count > 0 {
            println!("Progress: {} propagations, {} nodes", prop_count, node_count);
        }
    });
    
    let duration = start.elapsed();
    
    match solution {
        Ok(sol) => {
            println!("✅ Success in {:?}!", duration);
            println!("Stats: {} propagations, {} nodes", prop_count, node_count);
            
            // Print solution
            for row in 0..2 {
                for col in 0..2 {
                    if let Val::ValI(val) = sol[grid[row][col]] {
                        print!("{} ", val);
                    }
                }
                println!();
            }
        }
        Err(e) => {
            println!("❌ Error after {:?}: {:?}", duration, e);
        }
    }
    
    Ok(())
}