//! Benchmark the enhanced Sudoku solver with basic techniques against hard puzzles.

use selen::solvers::{SudokuSolver, SudokuResult};
use std::time::Instant;

fn main() {
    println!("🧩 Sudoku Solver Benchmark with Basic Techniques");
    println!("{}", "=".repeat(60));

    // Test the real "Platinum Blonde" puzzle from sudoku.rs example
    let platinum_blonde = [
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
    
    println!("\n🔥 Testing 'Platinum Blonde' - one of the hardest Sudoku puzzles");
    println!("The real Platinum Blonde from sudoku.rs example");
    
    let grid = platinum_blonde;
    println!("\n📊 Original puzzle:");
    println!("{}", SudokuSolver::format_grid("Platinum Blonde", &grid));
    
    let start = Instant::now();
    let solver = SudokuSolver::new(grid);
    
    // Show initial candidate state
    println!("\n🔍 Analyzing candidates after initialization...");
    let candidates = solver.get_candidates();
    let mut empty_cells = 0;
    let mut total_candidates = 0;
    
    for row in 0..9 {
        for col in 0..9 {
            if grid[row][col] == 0 {
                empty_cells += 1;
                total_candidates += candidates[row][col].len();
            }
        }
    }
    
    println!("Empty cells: {}", empty_cells);
    println!("Average candidates per empty cell: {:.1}", total_candidates as f64 / empty_cells as f64);
    
    let result = solver.solve();
    let total_time = start.elapsed();
    
    match result.solution {
        Some(solution) => {
            println!("\n✅ Solution found!");
            println!("{}", SudokuSolver::format_grid("Solution", &solution));
            
            println!("\n📈 Performance Statistics:");
            println!("Total time: {:.2}ms", total_time.as_secs_f64() * 1000.0);
            println!("Solver time: {:.2}ms", result.duration_ms);
            println!("Constraint propagations: {}", result.propagations);
            println!("Search nodes: {}", result.nodes);
            println!("Pure propagation: {}", if result.pure_propagation { "Yes" } else { "No" });
            
            if SudokuSolver::verify_solution(&solution) {
                println!("✅ Solution verified as correct!");
            } else {
                println!("❌ Solution verification failed!");
            }
        }
        None => {
            println!("❌ No solution found");
            println!("Time: {:.2}ms", total_time.as_secs_f64() * 1000.0);
        }
    }
    
    // Test a few more hard puzzles for comparison
    println!("\n{}", "=".repeat(60));
    println!("🎯 Testing additional hard puzzles:");
    
    let hard_puzzles = vec![
        ("AI Escargot", "100007090030020008009600500005300900010080002600004000300000010040000007007000300"),
        ("Easter Monster", "100000000020003000003005006004200700050000016000078000000145000000000820000006004"),
        ("Tarek Hardest", "200000000000050040004600000050000200000000000003000090000036000020070000000000005"),
    ];
    
    for (name, puzzle_str) in hard_puzzles {
        println!("\n🧩 {}", name);
        if let Ok(grid) = SudokuSolver::parse_string(puzzle_str) {
            let start = Instant::now();
            let solver = SudokuSolver::new(grid);
            let result = solver.solve();
            let total_time = start.elapsed();
            
            match result.solution {
                Some(_) => {
                    println!("✅ Solved in {:.2}ms (propagations: {}, nodes: {})", 
                             total_time.as_secs_f64() * 1000.0, result.propagations, result.nodes);
                }
                None => {
                    println!("❌ Failed to solve in {:.2}ms", total_time.as_secs_f64() * 1000.0);
                }
            }
        }
    }
    
    println!("\n🏁 Benchmark complete!");
}