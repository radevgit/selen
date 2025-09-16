use cspsolver::prelude::*;
use cspsolver::{post};
use std::time::Instant;

fn solve_sudoku(puzzle: &[[i32; 9]; 9]) -> Option<([[i32; 9]; 9], usize, usize)> {
    let mut m = Model::default();
    
    // OPTIMIZATION 1: Create variables more efficiently
    // For clues, create singleton variables directly; for empty cells, create full domain
    let mut grid = [[m.int(1, 9); 9]; 9];
    for row in 0..9 {
        for col in 0..9 {
            if puzzle[row][col] != 0 {
                // Create singleton variable for clues (much more efficient than equals constraint)
                let clue_val = puzzle[row][col];
                grid[row][col] = m.int(clue_val, clue_val);
            } else {
                grid[row][col] = m.int(1, 9);
            }
        }
    }
    
    // OPTIMIZATION 2: Pre-allocate vectors and use more efficient constraint posting
    // Row constraints: each row has all digits 1-9
    for row in 0..9 {
        post!(m, alldiff(grid[row]));
    }
    
    // Column constraints: each column has all digits 1-9
    for col in 0..9 {
        let column = [
            grid[0][col], grid[1][col], grid[2][col],
            grid[3][col], grid[4][col], grid[5][col],
            grid[6][col], grid[7][col], grid[8][col]
        ];
        post!(m, alldiff(column));
    }
    
    // Box constraints: each 3x3 box has all digits 1-9
    for box_row in 0..3 {
        for box_col in 0..3 {
            let mut box_vars = Vec::with_capacity(9);
            for row in 0..3 {
                for col in 0..3 {
                    box_vars.push(grid[box_row * 3 + row][box_col * 3 + col]);
                }
            }
            post!(m, alldiff(box_vars));
        }
    }
    
    // OPTIMIZATION 3: Optimize constraint order for better propagation
    m.optimize_constraint_order();
    
    // Solve the model with embedded statistics
    let solution = m.solve();
    
    // Convert solution to grid
    match solution {
        Ok(sol) => {
            // Access statistics from the solution
            let propagation_count = sol.stats.propagation_count;
            let node_count = sol.stats.node_count;
            
            let mut result = [[0; 9]; 9];
            for row in 0..9 {
                for col in 0..9 {
                    if let Val::ValI(value) = sol[grid[row][col]] {
                        result[row][col] = value;
                    }
                }
            }
            Some((result, propagation_count, node_count))
        }
        Err(_) => None
    }
}

fn print_grid(title: &str, grid: &[[i32; 9]; 9]) {
    println!("\n{}", title);
    println!("┌───────┬───────┬───────┐");
    
    for (row_idx, row) in grid.iter().enumerate() {
        print!("│");
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell == 0 {
                print!(" ·");
            } else {
                print!(" {}", cell);
            }
            if col_idx % 3 == 2 && col_idx < 8 {
                print!(" │");
            }
        }
        println!(" │");
        
        if row_idx % 3 == 2 && row_idx < 8 {
            println!("├───────┼───────┼───────┤");
        }
    }
    println!("└───────┴───────┴───────┘");
}

fn main() {
    println!("🔢 Testing EASY Sudoku Only");
    println!("============================");
    
    // Easy Sudoku puzzle (26 clues)
    let easy_puzzle = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 0, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 0],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 0, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 0],
    ];
    
    // Count clues
    let clue_count = easy_puzzle.iter().flatten().filter(|&&x| x != 0).count();
    println!("📊 Puzzle stats: {} clues given, {} empty cells", clue_count, 81 - clue_count);
    
    print_grid("Easy Puzzle:", &easy_puzzle);
    
    // Solve the puzzle
    let start = Instant::now();
    let result = solve_sudoku(&easy_puzzle);
    let duration = start.elapsed();
    
    match result {
        Some((grid, propagations, nodes)) => {
            println!("✅ Solution found in {:.3}ms!", duration.as_secs_f64() * 1000.0);
            println!("📊 Statistics: {} propagations, {} nodes explored", propagations, nodes);
            
            // Performance analysis
            let efficiency = if nodes > 0 { 
                format!("{:.1} propagations/node", propagations as f64 / nodes as f64)
            } else {
                "Pure propagation (no search)".to_string()
            };
            println!("🔍 Efficiency: {}", efficiency);
            
            print_grid("Solution:", &grid);
            println!("{}", "─".repeat(50));
        }
        None => {
            println!("❌ No solution found (took {:.3}ms)", duration.as_secs_f64() * 1000.0);
            println!("{}", "─".repeat(50));
        }
    }
}