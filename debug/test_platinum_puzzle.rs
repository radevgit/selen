//! Test the Platinum Blonde puzzle that was causing timeout

use selen::prelude::*;
use std::time::Instant;

fn main() {
    println!("Testing Platinum Blonde puzzle...");
    
    // The puzzle that was taking over 20 seconds
    let puzzle = [
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
    
    let start = Instant::now();
    let result = solve_sudoku(&puzzle);
    let duration = start.elapsed();
    
    match result {
        Some((grid, propagations, nodes)) => {
            println!("✅ Solution found in {:.3}ms!", duration.as_secs_f64() * 1000.0);
            println!("📊 Statistics: {} propagations, {} nodes explored", propagations, nodes);
            print_solution(&grid);
        }
        None => {
            println!("❌ No solution found (took {:.3}ms)", duration.as_secs_f64() * 1000.0);
        }
    }
}

fn solve_sudoku(puzzle: &[[i32; 9]; 9]) -> Option<([[i32; 9]; 9], usize, usize)> {
    let mut m = Model::default();
    
    // Create variables for each cell (1-9)
    let mut grid = [[m.int(1, 9); 9]; 9];
    for row in 0..9 {
        for col in 0..9 {
            grid[row][col] = m.int(1, 9);
        }
    }
    
    // Add clue constraints
    for row in 0..9 {
        for col in 0..9 {
            if puzzle[row][col] != 0 {
                m.equals(grid[row][col], Val::int(puzzle[row][col]));
            }
        }
    }
    
    // Row constraints
    for row in 0..9 {
        m.all_different(grid[row].to_vec());
    }
    
    // Column constraints
    for col in 0..9 {
        let column: Vec<VarId> = (0..9).map(|row| grid[row][col]).collect();
        m.all_different(column);
    }
    
    // Box constraints
    for box_row in 0..3 {
        for box_col in 0..3 {
            let mut box_vars = Vec::new();
            for r in 0..3 {
                for c in 0..3 {
                    box_vars.push(grid[box_row * 3 + r][box_col * 3 + c]);
                }
            }
            m.all_different(box_vars);
        }
    }
    
    
    let solution = m.solve();
    
    solution.map(|sol| {
        let mut result = [[0; 9]; 9];
        for row in 0..9 {
            for col in 0..9 {
                if let Val::ValI(value) = sol[grid[row][col]] {
                    result[row][col] = value;
                }
            }
        }
        (result, sol.stats.propagation_count, sol.stats.node_count)
    })
}

fn print_solution(grid: &[[i32; 9]; 9]) {
    println!("\nSolution:");
    for row in grid {
        println!("{:?}", row);
    }
}
