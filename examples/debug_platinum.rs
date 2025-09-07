use cspsolver::prelude::*;
use std::time::Instant;

fn main() {
    println!("Testing PLATINUM puzzle specifically...");
    
    let platinum_puzzle = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0], // This empty row is the problem
        [0, 0, 0, 0, 0, 3, 0, 8, 5],
        [0, 0, 1, 0, 2, 0, 0, 0, 0],
        [0, 0, 0, 5, 0, 7, 0, 0, 0],
        [0, 0, 4, 0, 0, 0, 1, 0, 0],
        [0, 9, 0, 0, 0, 0, 0, 0, 0],
        [5, 0, 0, 0, 0, 0, 0, 7, 3],
        [0, 0, 2, 0, 1, 0, 0, 0, 0],
        [0, 0, 0, 0, 4, 0, 0, 0, 9],
    ];

    let mut model = Model::default();
    
    // Create variables for each cell (1-9)
    let mut grid = [[model.new_var_int(1, 9); 9]; 9];
    for row in 0..9 {
        for col in 0..9 {
            grid[row][col] = model.new_var_int(1, 9);
        }
    }
    
    // Add clue constraints
    for row in 0..9 {
        for col in 0..9 {
            if platinum_puzzle[row][col] != 0 {
                model.equals(grid[row][col], Val::int(platinum_puzzle[row][col]));
            }
        }
    }
    
    println!("Creating constraints...");
    
    // Row constraints: each row has all digits 1-9
    for row in 0..9 {
        model.all_different(grid[row].to_vec());
    }
    
    // Column constraints: each column has all digits 1-9
    for col in 0..9 {
        let column: Vec<VarId> = (0..9).map(|row| grid[row][col]).collect();
        model.all_different(column);
    }
    
    // Box constraints: each 3x3 box has all digits 1-9
    for box_row in 0..3 {
        for box_col in 0..3 {
            let mut box_vars = Vec::new();
            for r in 0..3 {
                for c in 0..3 {
                    box_vars.push(grid[box_row * 3 + r][box_col * 3 + c]);
                }
            }
            model.all_different(box_vars);
        }
    }
    
    println!("Starting solve with timeout...");
    let start = Instant::now();
    
    // Try to solve with simple time tracking
    let mut propagation_count = 0;
    let mut node_count = 0;
    
    let solution = model.solve_with_callback(|stats| {
        propagation_count = stats.propagation_count;
        node_count = stats.node_count;
        
        // Print progress every 100 propagations to see what's happening
        if propagation_count % 100 == 0 && propagation_count > 0 {
            let elapsed = start.elapsed();
            println!("Progress: {} propagations, {} nodes, {:.1}s elapsed", 
                     propagation_count, node_count, elapsed.as_secs_f64());
            
            // Emergency timeout after 10 seconds
            if elapsed.as_secs() >= 10 {
                println!("EMERGENCY TIMEOUT - stopping after 10 seconds");
                return; // This won't actually stop the solver, but it shows where we are
            }
        }
    });
    
    let duration = start.elapsed();
    
    match solution {
        Some(_) => {
            println!("✅ SOLVED in {:.3}ms", duration.as_secs_f64() * 1000.0);
            println!("📊 {} propagations, {} nodes", propagation_count, node_count);
        }
        None => {
            println!("❌ No solution found in {:.3}ms", duration.as_secs_f64() * 1000.0);
            println!("📊 {} propagations, {} nodes attempted", propagation_count, node_count);
        }
    }
}
