//! Simple Sudoku Solver (New API)
//!
//! This example demonstrates solving a simple 4x4 Sudoku puzzle using the new unified API.
//!
//! Demonstrates:
//! - m.ints() for creating multiple variables
//! - m.alldiff() for all-different constraints
//! - m.new(var.eq(int(value))) for setting known values

use selen::prelude::*;

fn main() {
    println!("🔢 Simple 4x4 Sudoku Solver (New API)");
    println!("====================================\n");

    // Create a 4x4 Sudoku puzzle
    // Puzzle:
    //  1  _  _  _
    //  _  _  2  _
    //  _  3  _  _
    //  _  _  _  4
    
    let mut m = Model::default();
    
    // Create 16 variables for the 4x4 grid (values 1-4)
    let cells = m.ints(16, 1, 4);
    
    // Helper to get cell index
    let cell = |row: usize, col: usize| cells[row * 4 + col];
    
    // Set known values using runtime API with explicit int() constants
    m.new(cell(0, 0).eq(int(1)));
    m.new(cell(1, 2).eq(int(2)));
    m.new(cell(2, 1).eq(int(3)));
    m.new(cell(3, 3).eq(int(4)));
    
    // Row constraints - each row must have all different values
    for row in 0..4 {
        let row_cells: Vec<_> = (0..4).map(|col| cell(row, col)).collect();
        m.alldiff(&row_cells);
    }
    
    // Column constraints - each column must have all different values
    for col in 0..4 {
        let col_cells: Vec<_> = (0..4).map(|row| cell(row, col)).collect();
        m.alldiff(&col_cells);
    }
    
    // 2x2 box constraints - each box must have all different values
    for box_row in 0..2 {
        for box_col in 0..2 {
            let mut box_cells = Vec::new();
            for r in 0..2 {
                for c in 0..2 {
                    box_cells.push(cell(box_row * 2 + r, box_col * 2 + c));
                }
            }
            m.alldiff(&box_cells);
        }
    }
    
    println!("🔍 Solving...");
    match m.solve() {
        Ok(solution) => {
            println!("✅ Solution found!\n");
            
            // Print the solution
            for row in 0..4 {
                for col in 0..4 {
                    let val = solution.get_int(cell(row, col));
                    print!(" {} ", val);
                }
                println!();
            }
        }
        Err(_) => {
            println!("❌ No solution found!");
        }
    }
}
