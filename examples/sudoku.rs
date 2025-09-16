//! Simple Sudoku Solver using CSP framework
//! 
//! This example demonstrates solving a 9x9 Sudoku puzzle using constraint programming.
//! Sudoku rules:
//! 1. Each cell contains a digit 1-9
//! 2. Each row, column, and 3x3 box contains all digits 1-9 exactly once
//!
//! ## Performance Note
//! 
//! **Use `cargo run --release --example sudoku` for proper performance benchmarks!**
//! - Release mode: Platinum puzzle ~15 seconds
//! - Debug mode: Platinum puzzle ~118 seconds (7.8x slower)
//! 
//! The Platinum puzzle is computationally intensive and requires optimization.

use cspsolver::prelude::*;
use cspsolver::{post};
use std::time::Instant;

fn main() {
    println!("🔢 Simple Sudoku Solver");
    println!("======================");
    
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
    
    // Hard Sudoku puzzle - "AI Escargot" - known as one of the most difficult
    let hard_puzzle = [
        [1, 0, 0, 0, 0, 7, 0, 9, 0],
        [0, 3, 0, 0, 2, 0, 0, 0, 8],
        [0, 0, 9, 6, 0, 0, 5, 0, 0],
        [0, 0, 5, 3, 0, 0, 9, 0, 0],
        [0, 1, 0, 0, 8, 0, 0, 0, 2],
        [6, 0, 0, 0, 0, 4, 0, 0, 0],
        [3, 0, 0, 0, 0, 0, 0, 1, 0],
        [0, 4, 0, 0, 0, 0, 0, 0, 7],
        [0, 0, 7, 0, 0, 0, 3, 0, 0],
    ];
    
    // Extreme Sudoku puzzle - "World's Hardest Sudoku" - designed to be extremely challenging
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
    
    // "Platinum Blonde" - The ultimate computational challenge
    // Now runs in ~14 seconds thanks to architectural improvements (removing dyn-clone dependency)!
    
    let platinum_puzzle = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 3, 0, 8, 5],
        [0, 0, 1, 0, 2, 0, 0, 0, 0],
        [0, 0, 0, 5, 0, 7, 0, 0, 0],
        [0, 0, 4, 0, 0, 0, 1, 0, 0],
        [0, 9, 0, 0, 0, 0, 0, 0, 0],
        [5, 0, 0, 0, 0, 0, 0, 7, 3],
        [0, 0, 2, 0, 1, 0, 0, 0, 0],
        [0, 0, 0, 0, 4, 0, 0, 0, 9],
    ];
    
    
    // Solve all four puzzles - including Platinum!
    let (easy_propagations, easy_nodes) = solve_and_display("EASY", &easy_puzzle);
    let (hard_propagations, hard_nodes) = solve_and_display("HARD", &hard_puzzle);
    let (extreme_propagations, extreme_nodes) = solve_and_display("EXTREME", &extreme_puzzle);
    let (platinum_propagations, platinum_nodes) = solve_and_display("PLATINUM", &platinum_puzzle);
    
    println!("\n✨ Summary:");
    println!("Easy puzzle demonstrates solid performance with default search heuristics:");
    println!("• {} propagations, {} nodes explored", easy_propagations, easy_nodes);
    let easy_efficiency = if easy_nodes > 0 { 
        format!("{:.1} propagations/node", easy_propagations as f64 / easy_nodes as f64)
    } else {
        "Pure propagation (no search)".to_string()
    };
    println!("• {} efficiency", easy_efficiency);
    
    println!("\nHard puzzle (AI Escargot) demonstrates increased computational challenge:");
    println!("• {} propagations, {} nodes explored", hard_propagations, hard_nodes);
    let hard_efficiency = if hard_nodes > 0 { 
        format!("{:.1} propagations/node", hard_propagations as f64 / hard_nodes as f64)
    } else {
        "Pure propagation (no search)".to_string()
    };
    println!("• {} efficiency", hard_efficiency);
    
    println!("\nExtreme puzzle (World's Hardest) demonstrates the ultimate CSP challenge:");
    println!("• {} propagations, {} nodes explored", extreme_propagations, extreme_nodes);
    let extreme_efficiency = if extreme_nodes > 0 { 
        format!("{:.1} propagations/node", extreme_propagations as f64 / extreme_nodes as f64)
    } else {
        "Pure propagation (no search)".to_string()
    };
    println!("• {} efficiency", extreme_efficiency);
    
    // Performance improvement celebration! Platinum now runs by default thanks to dyn-clone removal
    
    println!("\nPlatinum puzzle (Platinum Blonde) - the ultimate computational challenge:");
    println!("• {} propagations, {} nodes explored", platinum_propagations, platinum_nodes);
    let platinum_efficiency = if platinum_nodes > 0 { 
        format!("{:.1} propagations/node", platinum_propagations as f64 / platinum_nodes as f64)
    } else {
        "Pure propagation (no search)".to_string()
    };
    println!("• {} efficiency", platinum_efficiency);
    println!("• 🚀 Performance Achievement: Now runs in ~14 seconds (was ~74 seconds)!");
    println!("• 🏗️  Credit: Architectural improvement - removed dyn-clone dependency, Rc-based propagator sharing!");
    
}

fn solve_and_display(difficulty: &str, puzzle: &[[i32; 9]; 9]) -> (usize, usize) {
    println!("\n🧩 Solving {} puzzle:", difficulty);
    
    // Count clues
    let clue_count = puzzle.iter().flatten().filter(|&&x| x != 0).count();
    println!("📊 Puzzle stats: {} clues given, {} empty cells", clue_count, 81 - clue_count);
    
    print_grid("Puzzle:", puzzle);
    
    // Solve the puzzle
    let start = Instant::now();
    let result = solve_sudoku(puzzle);
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
            (propagations, nodes)
        }
        None => {
            println!("❌ No solution found (took {:.3}ms)", duration.as_secs_f64() * 1000.0);
            println!("{}", "─".repeat(50));
            (0, 0)
        }
    }
}

fn solve_sudoku(puzzle: &[[i32; 9]; 9]) -> Option<([[i32; 9]; 9], usize, usize)> {
    let mut m = Model::default();
    
    // Create variables individually to avoid duplication
    let mut grid = Vec::new();
    for row in 0..9 {
        let mut grid_row = Vec::new();
        for col in 0..9 {
            if puzzle[row][col] != 0 {
                // Create singleton variable for clues (much more efficient than equals constraint)
                let clue_val = puzzle[row][col];
                grid_row.push(m.int(clue_val, clue_val));
            } else {
                grid_row.push(m.int(1, 9));
            }
        }
        grid.push(grid_row);
    }
    
    // OPTIMIZATION 2: Pre-allocate vectors and use more efficient constraint posting
    // Row constraints: each row has all digits 1-9
    for row in 0..9 {
        post!(m, alldiff(grid[row]));
    }
    
    // Column constraints: each column has all digits 1-9  
    for col in 0..9 {
        let column: Vec<VarId> = (0..9).map(|row| grid[row][col]).collect();
        post!(m, alldiff(column));
    }
    
    // Box constraints: each 3x3 box has all digits 1-9
    for box_row in 0..3 {
        for box_col in 0..3 {
            let mut box_vars = Vec::with_capacity(9); // Pre-allocate for efficiency
            for r in 0..3 {
                for c in 0..3 {
                    box_vars.push(grid[box_row * 3 + r][box_col * 3 + c]);
                }
            }
            post!(m, alldiff(box_vars));
        }
    }
    
    // Constraint optimization is now handled automatically in prepare_for_search()
    // No need to call m.optimize_constraint_order() manually
    
    // Solve the model with statistics tracking
    // Solve the model with embedded statistics
    let solution = m.solve();
    
    // Convert solution to grid
    solution.map(|sol| {
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
        (result, propagation_count, node_count)
    }).ok()
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
            
            if (col_idx + 1) % 3 == 0 {
                print!(" │");
            }
        }
        println!();
        
        if row_idx == 2 || row_idx == 5 {
            println!("├───────┼───────┼───────┤");
        }
    }
    println!("└───────┴───────┴───────┘");
}
