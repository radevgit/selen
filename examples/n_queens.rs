//! N-Queens Problem Solver (New API)
//!
//! This example demonstrates solving the classic N-Queens problem using the new unified API.
//! The N-Queens problem involves placing N queens on an N×N chessboard such that no two queens
//! attack each other (same row, column, or diagonal).
//!
//! This example demonstrates:
//! - m.ints() for creating multiple variables
//! - m.alldiff() for all-different constraints  
//! - add() for expression building
//! - m.solve() for finding solutions

use selen::prelude::*;
use std::time::Instant;

fn main() {
    println!("👑 N-Queens Problem Solver (New API)");
    println!("====================================\n");

    // Test 8-Queens problem
    let n = 8;
    println!("🔍 Solving {}-Queens problem", n);
    
    let start = Instant::now();
    match solve_n_queens(n) {
        Some(solution) => {
            let duration = start.elapsed();
            println!("✅ Solution found in {:.3}ms!", duration.as_secs_f64() * 1000.0);
            print_board(&solution, n);
        }
        None => {
            println!("❌ No solution found");
        }
    }
}

fn solve_n_queens(n: usize) -> Option<Vec<i32>> {
    let mut m = Model::default();
    
    // Variables: queen_rows[i] = row position of queen in column i (1-based)
    let queen_rows = m.ints(n, 1, n as i32);
    
    // Constraint 1: All queens must be in different rows
    m.alldiff(&queen_rows);
    
    // Constraint 2: No two queens on the same ascending diagonal
    // Ascending diagonal: queen_row[i] + i must be different for all i
    let mut ascending_diagonals = Vec::new();
    for (i, &queen_row) in queen_rows.iter().enumerate() {
        // queen_row + i (using explicit int constant)
        let diagonal = add(queen_row, int(i as i32));
        ascending_diagonals.push(diagonal);
    }
    
    // We need to materialize these expressions into variables for alldiff
    let ascending_vars: Vec<_> = ascending_diagonals.iter()
        .map(|expr| {
            let var = m.int(1 - n as i32, 2 * n as i32);
            m.new(expr.clone().eq(var));
            var
        })
        .collect();
    m.alldiff(&ascending_vars);
    
    // Constraint 3: No two queens on the same descending diagonal
    // Descending diagonal: queen_row[i] - i must be different for all i
    let mut descending_diagonals = Vec::new();
    for (i, &queen_row) in queen_rows.iter().enumerate() {
        // queen_row - i (using explicit int constant)
        let diagonal = sub(queen_row, int(i as i32));
        descending_diagonals.push(diagonal);
    }
    
    // Materialize descending diagonal expressions
    let descending_vars: Vec<_> = descending_diagonals.iter()
        .map(|expr| {
            let var = m.int(1 - n as i32, n as i32);
            m.new(expr.clone().eq(var));
            var
        })
        .collect();
    m.alldiff(&descending_vars);
    
    // Solve the model
    match m.solve() {
        Ok(sol) => {
            let positions: Vec<i32> = queen_rows.iter()
                .map(|&var| sol.get_int(var))
                .collect();
            Some(positions)
        }
        Err(_) => None,
    }
}

fn print_board(solution: &[i32], n: usize) {
    println!("\n📋 Solution:");
    
    // Print column numbers
    print!("   ");
    for col in 1..=n {
        print!("{:2} ", col);
    }
    println!();
    
    // Print board with queens
    for row in 1..=(n as i32) {
        print!("{:2} ", row);
        for col in 0..n {
            if solution[col] == row {
                print!(" ♛ ");
            } else {
                print!(" · ");
            }
        }
        println!();
    }
    println!();
}
