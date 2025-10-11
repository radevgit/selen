//! Simple Sudoku Solver using the specialized SudokuSolver API
//! 
//! This example demonstrates solving a 9x9 Sudoku puzzle using the production-ready
//! SudokuSolver that provides a clean API without manual constraint setup.
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

use selen::solvers::SudokuSolver;

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
    let (_easy_propagations, _easy_nodes) = solve_and_display("EASY", &easy_puzzle);
    let (_hard_propagations, _hard_nodes) = solve_and_display("HARD", &hard_puzzle);
    let (_extreme_propagations, _extreme_nodes) = solve_and_display("EXTREME", &extreme_puzzle);
    let (_platinum_propagations, _platinum_nodes) = solve_and_display("PLATINUM", &platinum_puzzle);
    
}

fn solve_and_display(difficulty: &str, puzzle: &[[i32; 9]; 9]) -> (usize, usize) {
    println!("\n🧩 Solving {} puzzle:", difficulty);
    
    // Create specialized solver
    let solver = SudokuSolver::new(*puzzle);
    println!("📊 Puzzle stats: {} clues given, {} empty cells", solver.clue_count(), 81 - solver.clue_count());
    
    println!("{}", SudokuSolver::format_grid("Puzzle:", puzzle));
    
    // Solve the puzzle using the specialized solver
    let result = solver.solve();
    
    match result.solution {
        Some(grid) => {
            println!("✅ Solution found in {:.3}ms!", result.duration_ms);
            println!("📊 Statistics: {} propagations, {} nodes explored", result.propagations, result.nodes);
            
            // Performance analysis
            let efficiency = if result.nodes > 0 { 
                format!("{:.1} propagations/node", result.propagations as f64 / result.nodes as f64)
            } else {
                "Pure propagation (no search)".to_string()
            };
            println!("🔍 Efficiency: {}", efficiency);
            
            if result.pure_propagation {
                println!("🎯 Solved by pure constraint propagation!");
            }
            
            println!("{}", SudokuSolver::format_grid("Solution:", &grid));
            println!("{}", "─".repeat(50));
            (result.propagations, result.nodes)
        }
        None => {
            println!("❌ No solution found (took {:.3}ms)", result.duration_ms);
            println!("{}", "─".repeat(50));
            (0, 0)
        }
    }
}


