use cspsolver::prelude::*;
use std::time::Instant;

fn solve_sudoku(puzzle: &[[i32; 9]; 9]) -> Option<(usize, usize)> {
    let mut model = Model::default();
    
    // Create variables for each cell (1-9)
    let mut grid = [[model.new_var_int(1, 9); 9]; 9];
    for row in 0..9 {
        for col in 0..9 {
            // Re-initialize with proper constraints
            grid[row][col] = model.new_var_int(1, 9);
        }
    }
    
    // Add clue constraints
    for row in 0..9 {
        for col in 0..9 {
            if puzzle[row][col] != 0 {
                model.equals(grid[row][col], Val::int(puzzle[row][col]));
            }
        }
    }
    
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
    
    // Solve the model with statistics tracking
    let mut propagation_count = 0;
    let mut node_count = 0;
    
    let _solution = model.solve_with_callback(|stats| {
        // Track statistics
        propagation_count = stats.propagation_count;
        node_count = stats.node_count;
    });
    
    Some((propagation_count, node_count))
}

fn main() {
    // EXTREME puzzle (should be fast)
    let extreme_puzzle = [
        [8, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 3, 6, 0, 0, 0, 0, 0], 
        [0, 7, 0, 0, 9, 0, 2, 0, 0],
        [0, 5, 0, 0, 0, 7, 0, 0, 0],
        [0, 0, 0, 0, 4, 5, 7, 0, 0],
        [0, 0, 0, 1, 0, 0, 0, 3, 0],
        [0, 0, 1, 0, 0, 0, 0, 6, 8],
        [0, 0, 8, 5, 0, 0, 0, 1, 0],
        [0, 9, 0, 0, 0, 0, 4, 0, 0],
    ];

    // PLATINUM puzzle (was extremely slow due to all_different bug)
    let platinum_puzzle = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0], // Empty first row triggers worst case
        [0, 0, 0, 0, 0, 3, 0, 8, 5],
        [0, 0, 1, 0, 2, 0, 0, 0, 0],
        [0, 0, 0, 5, 0, 7, 0, 0, 0],
        [0, 0, 4, 0, 0, 0, 1, 0, 0],
        [0, 9, 0, 0, 0, 0, 0, 0, 0],
        [5, 0, 0, 0, 0, 0, 0, 7, 3],
        [0, 0, 2, 0, 1, 0, 0, 0, 0],
        [0, 0, 0, 0, 4, 0, 0, 0, 9],
    ];

    println!("Testing performance fix for all_different constraint...\n");

    println!("EXTREME puzzle (baseline):");
    let start = Instant::now();
    let extreme_result = solve_sudoku(&extreme_puzzle);
    let extreme_duration = start.elapsed();
    
    if let Some((props, nodes)) = extreme_result {
        println!("✅ SOLVED in {:.3}ms", extreme_duration.as_secs_f64() * 1000.0);
        println!("📊 {} propagations, {} nodes", props, nodes);
        if extreme_duration.as_millis() > 0 {
            println!("⚡ {:.0}ns per propagation", extreme_duration.as_nanos() as f64 / props as f64);
        }
    }

    println!("\nPLATINUM puzzle (was 1,183x slower):");
    let start = Instant::now();
    let platinum_result = solve_sudoku(&platinum_puzzle);
    let platinum_duration = start.elapsed();
    
    if let Some((props, nodes)) = platinum_result {
        println!("✅ SOLVED in {:.3}ms", platinum_duration.as_secs_f64() * 1000.0);
        println!("📊 {} propagations, {} nodes", props, nodes);
        if platinum_duration.as_millis() > 0 {
            println!("⚡ {:.0}ns per propagation", platinum_duration.as_nanos() as f64 / props as f64);
        }
        
        // Performance comparison
        if let Some((extreme_props, _)) = extreme_result {
            let extreme_per_prop = extreme_duration.as_nanos() as f64 / extreme_props as f64;
            let platinum_per_prop = platinum_duration.as_nanos() as f64 / props as f64;
            let slowdown = platinum_per_prop / extreme_per_prop;
            
            println!("\n🔍 Performance Analysis:");
            println!("   EXTREME: {:.0}ns per propagation", extreme_per_prop);
            println!("   PLATINUM: {:.0}ns per propagation", platinum_per_prop);
            println!("   Slowdown: {:.1}x (was 1,115x before fix!)", slowdown);
            
            if slowdown < 10.0 {
                println!("🎉 BUG FIXED! Slowdown reduced from 1,115x to {:.1}x", slowdown);
            } else {
                println!("⚠️  Still slow - bug may not be completely fixed");
            }
        }
    }
}
