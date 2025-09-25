/// Benchmark using the extremely hard 17-clue Sudoku puzzle
/// 
/// This puzzle is one of the hardest known Sudoku puzzles with only 17 clues
/// (the theoretical minimum). It should take much longer to solve than 
/// typical hard puzzles like Platinum Blonde.

use std::time::Instant;
use selen::solvers::sudoku::SudokuSolver;
use selen::constraints::gac_hybrid::HybridGAC;
use selen::constraints::gac::{Variable};

// The extremely hard 17-clue puzzle from the user
const EXTREME_17_CLUE: &str = "000000000000385001020000000507000004000100090000000500000073002010000000400009000";

/// Convert Sudoku string to 9x9 grid
fn parse_sudoku(puzzle_str: &str) -> [[i32; 9]; 9] {
    let mut grid = [[0; 9]; 9];
    let chars: Vec<char> = puzzle_str.chars().collect();
    
    for (i, &ch) in chars.iter().enumerate() {
        if i < 81 {
            let row = i / 9;
            let col = i % 9;
            grid[row][col] = ch.to_digit(10).unwrap_or(0) as i32;
        }
    }
    grid
}

/// Display the puzzle in a nice format
fn display_puzzle(title: &str, grid: [[i32; 9]; 9]) {
    println!("{}", title);
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

/// Count the number of clues in the puzzle
fn count_clues(grid: [[i32; 9]; 9]) -> usize {
    grid.iter()
        .flat_map(|row| row.iter())
        .filter(|&&cell| cell != 0)
        .count()
}

/// Solve using specialized Sudoku solver
fn solve_with_specialized_solver(grid: [[i32; 9]; 9]) -> (bool, u128, usize, usize) {
    let start = Instant::now();
    let mut solver = SudokuSolver::new(grid);
    let result = solver.solve();
    let duration = start.elapsed().as_nanos();
    
    let solved = result.solution.is_some();
    let propagations = result.propagations;
    let nodes = result.nodes;
    
    (solved, duration, propagations, nodes)
}

/// Simple GAC-based solver (constraint propagation only)
fn solve_with_gac_propagation(grid: [[i32; 9]; 9]) -> Result<(bool, u128), String> {
    let start = Instant::now();
    
    let mut gac = HybridGAC::new();
    
    // Setup variables
    for row in 0..9 {
        for col in 0..9 {
            let var_id = row * 9 + col;
            if grid[row][col] != 0 {
                gac.add_variable_with_values(Variable(var_id), vec![grid[row][col]])?;
            } else {
                gac.add_variable(Variable(var_id), 1, 9)?;
            }
        }
    }
    
    // Apply constraints
    for row in 0..9 {
        let row_vars: Vec<Variable> = (0..9).map(|col| Variable(row * 9 + col)).collect();
        gac.propagate_alldiff(&row_vars)?;
    }
    
    for col in 0..9 {
        let col_vars: Vec<Variable> = (0..9).map(|row| Variable(row * 9 + col)).collect();
        gac.propagate_alldiff(&col_vars)?;
    }
    
    for box_row in 0..3 {
        for box_col in 0..3 {
            let mut box_vars = Vec::new();
            for r in 0..3 {
                for c in 0..3 {
                    box_vars.push(Variable((box_row * 3 + r) * 9 + (box_col * 3 + c)));
                }
            }
            gac.propagate_alldiff(&box_vars)?;
        }
    }
    
    // Check if solved by propagation alone
    let mut solved = true;
    for var_id in 0..81 {
        if !gac.is_assigned(Variable(var_id)) {
            solved = false;
            break;
        }
    }
    
    let duration = start.elapsed().as_nanos();
    Ok((solved, duration))
}

fn main() {
    println!("Extremely Hard 17-Clue Sudoku Benchmark");
    println!("======================================");
    
    let grid = parse_sudoku(EXTREME_17_CLUE);
    let clue_count = count_clues(grid);
    
    display_puzzle("The Extremely Hard 17-Clue Puzzle:", grid);
    println!("Number of clues: {} (theoretical minimum)", clue_count);
    println!();
    
    let iterations = 10; // Fewer iterations due to expected difficulty
    
    println!("Testing Specialized Sudoku Solver...");
    let mut specialized_times = Vec::new();
    let mut specialized_successes = 0;
    let mut total_propagations = 0;
    let mut total_nodes = 0;
    
    for i in 0..iterations {
        print!("Iteration {}/{}... ", i + 1, iterations);
        let (solved, time, propagations, nodes) = solve_with_specialized_solver(grid);
        specialized_times.push(time);
        total_propagations += propagations;
        total_nodes += nodes;
        
        if solved {
            specialized_successes += 1;
            println!("✓ Solved in {:.2} ms", time as f64 / 1_000_000.0);
        } else {
            println!("✗ Failed");
        }
    }
    
    println!("\nTesting GAC Constraint Propagation...");
    let mut gac_times = Vec::new();
    let mut gac_successes = 0;
    
    for i in 0..iterations {
        print!("Iteration {}/{}... ", i + 1, iterations);
        match solve_with_gac_propagation(grid) {
            Ok((solved, time)) => {
                gac_times.push(time);
                if solved {
                    gac_successes += 1;
                    println!("✓ Solved by propagation in {:.2} ms", time as f64 / 1_000_000.0);
                } else {
                    println!("⚠ Partial propagation in {:.2} ms", time as f64 / 1_000_000.0);
                }
            }
            Err(e) => {
                println!("✗ Error: {}", e);
                gac_times.push(0);
            }
        }
    }
    
    // Calculate results
    let avg_specialized = if !specialized_times.is_empty() {
        specialized_times.iter().sum::<u128>() / specialized_times.len() as u128
    } else {
        0
    };
    
    let avg_gac = if !gac_times.is_empty() {
        gac_times.iter().filter(|&&t| t > 0).sum::<u128>() / gac_times.iter().filter(|&&t| t > 0).count() as u128
    } else {
        0
    };
    
    println!("\n=== RESULTS ===");
    println!("Specialized Sudoku Solver:");
    println!("  Success rate: {}/{} ({:.1}%)", specialized_successes, iterations, 
             (specialized_successes as f64 / iterations as f64) * 100.0);
    
    if avg_specialized > 0 {
        println!("  Average time: {:.2} ms", avg_specialized as f64 / 1_000_000.0);
        if avg_specialized > 1_000_000_000 { // > 1 second
            println!("  Average time: {:.2} seconds", avg_specialized as f64 / 1_000_000_000.0);
        }
    }
    
    if specialized_successes > 0 {
        println!("  Avg propagations: {}", total_propagations / specialized_successes);
        println!("  Avg search nodes: {}", total_nodes / specialized_successes);
    }
    
    println!("\nGAC Constraint Propagation:");
    println!("  Success rate: {}/{} ({:.1}%)", gac_successes, iterations,
             (gac_successes as f64 / iterations as f64) * 100.0);
    
    if avg_gac > 0 {
        println!("  Average time: {:.2} ms", avg_gac as f64 / 1_000_000.0);
    }
    
    // Analysis
    println!("\n=== ANALYSIS ===");
    println!("🔥 Difficulty Assessment:");
    println!("   • Only {} clues (theoretical minimum)", clue_count);
    println!("   • Requires extensive search beyond constraint propagation");
    println!("   • Tests the limits of both approaches");
    
    if specialized_successes > 0 && avg_specialized > 1_000_000_000 {
        println!("\n⏱️ This puzzle indeed takes multiple seconds!");
        println!("   • Much harder than typical 'hard' puzzles");
        println!("   • Demonstrates the value of advanced techniques");
        println!("   • Shows where pure constraint propagation falls short");
    }
    
    println!("\n🎯 Implementation Performance:");
    println!("   • Specialized solver uses optimized search + constraint propagation");
    println!("   • GAC approach shows constraint propagation limits");
    println!("   • BitSet optimization helps but search is still required");
    println!("   • Advanced techniques (naked pairs, etc.) become crucial");
    
    if avg_specialized > 0 && avg_gac > 0 {
        let ratio = avg_specialized as f64 / avg_gac as f64;
        if ratio > 1.0 {
            println!("   • Specialized solver is {:.1}x slower (due to complete solving)", ratio);
        } else {
            println!("   • GAC propagation is {:.1}x faster (but incomplete)", 1.0 / ratio);
        }
    }
}