//! Benchmark to compare baseline vs enhanced Sudoku solver performance

use selen::solvers::SudokuSolver;
use selen::model::*;
use selen::prelude::*;
use selen::runtime_api::*;
use std::time::Instant;

/// Basic Sudoku solver without any advanced techniques - just alldiff constraints
fn solve_basic_sudoku(puzzle: [[i32; 9]; 9]) -> (Option<[[i32; 9]; 9]>, f64, usize, usize) {
    let start = Instant::now();
    let mut model = Model::default();
    let mut grid = Vec::new();
    
    // Create variables for each cell
    for row in 0..9 {
        let mut grid_row = Vec::new();
        for col in 0..9 {
            let var = if puzzle[row][col] != 0 {
                // Clue: create singleton variable
                let clue_val = puzzle[row][col];
                model.int(clue_val, clue_val)
            } else {
                // Empty cell: domain 1-9
                model.int(1, 9)
            };
            grid_row.push(var);
        }
        grid.push(grid_row);
    }
    
    // Add ONLY basic Sudoku constraints (no advanced techniques)
    
    // Row constraints - each row must contain all digits 1-9
    for row in 0..9 {
        model.alldiff(&grid[row]);
    }
    
    // Column constraints - each column must contain all digits 1-9
    for col in 0..9 {
        let column_vars: Vec<VarId> = (0..9).map(|row| grid[row][col]).collect();
        model.alldiff(&column_vars);
    }
    
    // Box constraints - each 3x3 box must contain all digits 1-9
    for box_row in 0..3 {
        for box_col in 0..3 {
            let mut box_vars = Vec::with_capacity(9);
            for r in 0..3 {
                for c in 0..3 {
                    box_vars.push(grid[box_row * 3 + r][box_col * 3 + c]);
                }
            }
            model.alldiff(&box_vars);
        }
    }
    
    // Solve without any advanced techniques
    let solution = model.solve();
    let duration = start.elapsed();
    let duration_ms = duration.as_secs_f64() * 1000.0;
    
    match solution {
        Ok(sol) => {
            let propagations = sol.stats.propagation_count;
            let nodes = sol.stats.node_count;
            
            // Extract solution grid
            let mut result_grid = [[0; 9]; 9];
            for row in 0..9 {
                for col in 0..9 {
                    if let Val::ValI(value) = sol[grid[row][col]] {
                        result_grid[row][col] = value;
                    }
                }
            }
            
            (Some(result_grid), duration_ms, propagations, nodes)
        }
        Err(_) => {
            (None, duration_ms, 0, 0)
        }
    }
}

fn main() {
    println!("🔬 Sudoku Solver Baseline vs Enhanced Comparison");
    println!("{}", "=".repeat(60));
    println!("Testing multiple hard puzzles to measure technique effectiveness\n");

    // Collection of famous hard puzzles
    let test_puzzles = vec![
        ("Platinum Blonde", [
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 3, 0, 8, 5],
            [0, 0, 1, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 5, 0, 7, 0, 0, 0],
            [0, 0, 4, 0, 0, 0, 1, 0, 0],
            [0, 9, 0, 0, 0, 0, 0, 0, 0],
            [5, 0, 0, 0, 0, 0, 0, 7, 3],
            [0, 0, 2, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 4, 0, 0, 0, 9],
        ]),
        ("AI Escargot", [
            [1, 0, 0, 0, 0, 7, 0, 9, 0],
            [0, 3, 0, 0, 2, 0, 0, 0, 8],
            [0, 0, 9, 6, 0, 0, 5, 0, 0],
            [0, 0, 5, 3, 0, 0, 9, 0, 0],
            [0, 1, 0, 0, 8, 0, 0, 0, 2],
            [6, 0, 0, 0, 0, 4, 0, 0, 0],
            [3, 0, 0, 0, 0, 0, 0, 1, 0],
            [0, 4, 0, 0, 0, 0, 0, 0, 7],
            [0, 0, 7, 0, 0, 0, 3, 0, 0],
        ]),
        ("World's Hardest", [
            [8, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 3, 6, 0, 0, 0, 0, 0],
            [0, 7, 0, 0, 9, 0, 2, 0, 0],
            [0, 5, 0, 0, 0, 7, 0, 0, 0],
            [0, 0, 0, 0, 4, 5, 7, 0, 0],
            [0, 0, 0, 1, 0, 0, 0, 3, 0],
            [0, 0, 1, 0, 0, 0, 0, 6, 8],
            [0, 0, 8, 5, 0, 0, 0, 1, 0],
            [0, 9, 0, 0, 0, 0, 4, 0, 0],
        ]),
        ("Easter Monster", [
            [1, 0, 0, 0, 0, 7, 0, 9, 0],
            [0, 3, 0, 0, 2, 0, 0, 0, 8],
            [0, 0, 9, 6, 0, 0, 5, 0, 0],
            [0, 0, 5, 3, 0, 0, 9, 0, 0],
            [0, 1, 0, 0, 8, 0, 0, 0, 2],
            [6, 0, 0, 0, 0, 4, 0, 0, 0],
            [3, 0, 0, 0, 0, 0, 0, 1, 0],
            [0, 4, 0, 0, 0, 0, 0, 0, 7],
            [0, 0, 7, 0, 0, 0, 3, 0, 0],
        ]),
        ("Tarek Hardest", [
            [2, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 5, 0, 0, 4, 0],
            [0, 0, 4, 6, 0, 0, 0, 0, 0],
            [0, 5, 0, 0, 0, 0, 2, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 3, 0, 0, 0, 0, 9, 0],
            [0, 0, 0, 0, 0, 3, 6, 0, 0],
            [0, 2, 0, 0, 7, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 5],
        ]),
    ];

    let mut total_baseline_time = 0.0;
    let mut total_enhanced_time = 0.0;
    let mut baseline_results = Vec::new();
    let mut enhanced_results = Vec::new();

    for (name, puzzle) in &test_puzzles {
        println!("\n{}", "=".repeat(60));
        println!("🔥 Testing: {}", name);
        println!("{}", SudokuSolver::format_grid(name, puzzle));

        // Test 1: Basic solver (TRUE BASELINE - only alldiff constraints)
        println!("\n🟡 BASELINE: Basic solver with only alldiff constraints");
        let (basic_solution, basic_time, basic_propagations, basic_nodes) = solve_basic_sudoku(*puzzle);
        
        let baseline_result = match basic_solution {
            Some(solution) => {
                println!("✅ Baseline solved in {:.2}ms", basic_time);
                println!("📊 Stats: {} propagations, {} nodes", basic_propagations, basic_nodes);
                if SudokuSolver::verify_solution(&solution) {
                    println!("✅ Solution verified");
                } else {
                    println!("❌ Solution verification failed");
                }
                total_baseline_time += basic_time;
                Some((basic_time, basic_propagations, basic_nodes))
            }
            None => {
                println!("❌ Baseline failed to solve in {:.2}ms", basic_time);
                None
            }
        };

        // Test 2: Enhanced solver (with advanced techniques)
        println!("\n🟢 ENHANCED: Solver with candidate tracking + basic techniques");
        let start = Instant::now();
        let solver = SudokuSolver::new(*puzzle);
        let result = solver.solve();
        let enhanced_time = start.elapsed().as_secs_f64() * 1000.0;
        
        let enhanced_result = match result.solution {
            Some(solution) => {
                println!("✅ Enhanced solved in {:.2}ms", enhanced_time);
                println!("📊 Stats: {} propagations, {} nodes", result.propagations, result.nodes);
                if SudokuSolver::verify_solution(&solution) {
                    println!("✅ Solution verified");
                } else {
                    println!("❌ Solution verification failed");
                }
                total_enhanced_time += enhanced_time;
                Some((enhanced_time, result.propagations, result.nodes))
            }
            None => {
                println!("❌ Enhanced failed to solve in {:.2}ms", enhanced_time);
                None
            }
        }; 

        // Individual puzzle comparison
        if let (Some((basic_time, basic_prop, basic_nodes)), Some((enh_time, enh_prop, enh_nodes))) = 
            (&baseline_result, &enhanced_result) {
            let speedup = basic_time / enh_time;
            let improvement = ((basic_time - enh_time) / basic_time) * 100.0;
            
            println!("\n📈 {} COMPARISON:", name.to_uppercase());
            println!("Baseline: {:.2}ms | Enhanced: {:.2}ms | Speedup: {:.2}x | Improvement: {:.1}%", 
                     basic_time, enh_time, speedup, improvement);
        }

        baseline_results.push((name.to_string(), baseline_result));
        enhanced_results.push((name.to_string(), enhanced_result));
    }

    // Overall summary
    println!("\n{}", "=".repeat(60));
    println!("📊 OVERALL PERFORMANCE SUMMARY");
    println!("{}", "=".repeat(60));

    let mut successful_comparisons = 0;
    let mut total_speedup = 0.0;
    let mut best_speedup = 0.0;
    let mut best_puzzle = "";

    for (i, ((name, baseline), (_, enhanced))) in baseline_results.iter().zip(enhanced_results.iter()).enumerate() {
        if let (Some((b_time, b_prop, b_nodes)), Some((e_time, e_prop, e_nodes))) = (baseline, enhanced) {
            let speedup = b_time / e_time;
            let improvement = ((b_time - e_time) / b_time) * 100.0;
            total_speedup += speedup;
            successful_comparisons += 1;

            if speedup > best_speedup {
                best_speedup = speedup;
                best_puzzle = name;
            }

            println!("🧩 {:<15} | Base: {:>8.1}ms | Enh: {:>8.1}ms | {:>5.2}x | {:>5.1}%", 
                     name, b_time, e_time, speedup, improvement);
        } else {
            println!("🧩 {:<15} | ❌ Failed to solve with one or both methods", name);
        }
    }

    if successful_comparisons > 0 {
        let avg_speedup = total_speedup / successful_comparisons as f64;
        let total_improvement = ((total_baseline_time - total_enhanced_time) / total_baseline_time) * 100.0;

        println!("\n🏆 AGGREGATE RESULTS:");
        println!("Total baseline time:  {:>8.1}ms", total_baseline_time);
        println!("Total enhanced time:  {:>8.1}ms", total_enhanced_time);
        println!("Total time saved:     {:>8.1}ms", total_baseline_time - total_enhanced_time);
        println!("Average speedup:      {:>8.2}x", avg_speedup);
        println!("Overall improvement:  {:>8.1}%", total_improvement);
        println!("Best speedup:         {:>8.2}x ({})", best_speedup, best_puzzle);
        println!("Successful solves:    {:>8} / {}", successful_comparisons, test_puzzles.len());
    }

    println!("\n🏁 Comprehensive comparison complete!");
}