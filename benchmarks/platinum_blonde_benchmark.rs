/// Benchmark for the Platinum Blonde Sudoku puzzle
/// 
/// This is the notorious "Platinum Blonde" puzzle from examples/sudoku.rs
/// that's known to be computationally intensive and take multiple seconds.

use std::time::Instant;
use selen::solvers::sudoku::SudokuSolver;
use selen::constraints::gac_hybrid::HybridGAC;
use selen::constraints::gac::{Variable};

// The Platinum Blonde puzzle from examples/sudoku.rs
const PLATINUM_BLONDE: [[i32; 9]; 9] = [
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

/// Display the puzzle in a nice format
fn display_puzzle(title: &str, grid: &[[i32; 9]; 9]) {
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
fn count_clues(grid: &[[i32; 9]; 9]) -> usize {
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
    println!("🔢 Platinum Blonde Sudoku Benchmark");
    println!("===================================");
    
    let grid = PLATINUM_BLONDE;
    let clue_count = count_clues(&grid);
    
    display_puzzle("The Notorious Platinum Blonde Puzzle:", &grid);
    println!("Number of clues: {} (very sparse!)", clue_count);
    println!();
    
    println!("⚠️  This puzzle is known to be computationally intensive!");
    println!("   Expected solve time: multiple seconds with specialized solver");
    println!();
    
    let iterations = 3; // Only a few iterations due to expected long solve times
    
    println!("Testing Specialized Sudoku Solver...");
    let mut specialized_times = Vec::new();
    let mut specialized_successes = 0;
    let mut total_propagations = 0;
    let mut total_nodes = 0;
    
    for i in 0..iterations {
        println!("Iteration {}/{}...", i + 1, iterations);
        let (solved, time, propagations, nodes) = solve_with_specialized_solver(grid);
        specialized_times.push(time);
        total_propagations += propagations;
        total_nodes += nodes;
        
        if solved {
            specialized_successes += 1;
            if time > 1_000_000_000 { // > 1 second
                println!("✓ Solved in {:.2} seconds", time as f64 / 1_000_000_000.0);
            } else {
                println!("✓ Solved in {:.2} ms", time as f64 / 1_000_000.0);
            }
        } else {
            println!("✗ Failed in {:.2} ms", time as f64 / 1_000_000.0);
        }
        println!("  {} propagations, {} nodes explored", propagations, nodes);
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
    
    println!("\n=== PLATINUM BLONDE RESULTS ===");
    println!("Specialized Sudoku Solver:");
    println!("  Success rate: {}/{} ({:.1}%)", specialized_successes, iterations, 
             (specialized_successes as f64 / iterations as f64) * 100.0);
    
    if avg_specialized > 0 {
        if avg_specialized > 1_000_000_000 { // > 1 second
            println!("  Average time: {:.2} seconds", avg_specialized as f64 / 1_000_000_000.0);
        } else {
            println!("  Average time: {:.2} ms", avg_specialized as f64 / 1_000_000.0);
        }
    }
    
    if specialized_successes > 0 {
        println!("  Avg propagations: {}", total_propagations / specialized_successes);
        println!("  Avg search nodes: {}", total_nodes / specialized_successes);
    }
    
    println!("\nHybrid GAC Constraint Propagation:");
    println!("  Success rate: {}/{} ({:.1}%)", gac_successes, iterations,
             (gac_successes as f64 / iterations as f64) * 100.0);
    
    if avg_gac > 0 {
        println!("  Average time: {:.2} ms", avg_gac as f64 / 1_000_000.0);
    }
    
    // Analysis
    println!("\n=== PLATINUM BLONDE ANALYSIS ===");
    println!("🔥 Difficulty Assessment:");
    println!("   • Only {} clues (extremely sparse)", clue_count);
    println!("   • Known as one of the hardest computational Sudoku puzzles");
    println!("   • Requires extensive search with advanced techniques");
    
    if specialized_successes > 0 {
        if avg_specialized > 1_000_000_000 {
            println!("\n⏱️ Multi-second solve time confirmed!");
            println!("   • This is the puzzle that takes ~3+ seconds");
            println!("   • Demonstrates the computational complexity limit");
        } else if avg_specialized > 100_000_000 { // > 100ms
            println!("\n⏱️ Significant solve time (hundreds of milliseconds)");
            println!("   • Solver has been optimized since original timing");
        } else {
            println!("\n🚀 Fast solve time indicates recent optimizations!");
            println!("   • Solver performance has dramatically improved");
        }
    }
    
    println!("\n🎯 Implementation Performance:");
    println!("   • BitSet GAC helps with constraint propagation efficiency");
    println!("   • Specialized solver uses optimized search + propagation");
    println!("   • Advanced solving techniques are crucial for this puzzle");
    
    if avg_specialized > 0 && avg_gac > 0 {
        let ratio = avg_specialized as f64 / avg_gac as f64;
        if ratio > 1.0 {
            println!("   • Complete solving is {:.1}x slower than propagation alone", ratio);
        }
    }
    
    // Show GAC stats if available
    println!("\n📊 BitSet vs SparseSet GAC Usage:");
    println!("   • All Sudoku variables have domains 1-9 (≤64 values)");
    println!("   • Should utilize BitSet GAC for maximum efficiency");
    println!("   • Constraint propagation alone insufficient for this puzzle");
}