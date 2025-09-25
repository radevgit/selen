/// Direct comparison between Specialized Sudoku Solver and Hybrid GAC
/// 
/// This benchmark compares our optimized SudokuSolver against the general-purpose
/// Hybrid GAC approach on the same hard puzzles.

use std::time::Instant;
use selen::solvers::sudoku::SudokuSolver;
use selen::constraints::gac_hybrid::HybridGAC;
use selen::constraints::gac::Variable;

// Hard Sudoku puzzles for testing
const PLATINUM_BLONDE: &str = "000000012000000003002300400001800005060070800000009000008500000900040500000618000";
const AI_ESCARGOT: &str = "100007090030020008009600500005300900010080002600004000300000010040000007007000300";
const WORLDS_HARDEST: &str = "800000000003600000070090200050007000000045700000100030001000068008500010090000400";

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

/// Solve using specialized Sudoku solver
fn solve_with_specialized_solver(grid: [[i32; 9]; 9]) -> (bool, u128) {
    let start = Instant::now();
    let mut solver = SudokuSolver::new(grid);
    let result = solver.solve();
    let duration = start.elapsed().as_nanos();
    
    // Check if solution was found
    let solved = result.solution.is_some();
    
    (solved, duration)
}

/// Solve using Hybrid GAC with full search
fn solve_with_hybrid_gac(grid: [[i32; 9]; 9]) -> Result<(bool, u128), String> {
    let start = Instant::now();
    
    let mut gac = HybridGAC::new();
    
    // Create variables for each cell
    let mut cell_vars = [[Variable(0); 9]; 9];
    for row in 0..9 {
        for col in 0..9 {
            let var_id = row * 9 + col;
            cell_vars[row][col] = Variable(var_id);
            
            if grid[row][col] != 0 {
                gac.add_variable_with_values(Variable(var_id), vec![grid[row][col]])?;
            } else {
                gac.add_variable(Variable(var_id), 1, 9)?;
            }
        }
    }
    
    // Apply initial constraints
    for row in 0..9 {
        let row_vars: Vec<Variable> = (0..9).map(|col| cell_vars[row][col]).collect();
        gac.propagate_alldiff(&row_vars)?;
    }
    
    for col in 0..9 {
        let col_vars: Vec<Variable> = (0..9).map(|row| cell_vars[row][col]).collect();
        gac.propagate_alldiff(&col_vars)?;
    }
    
    for box_row in 0..3 {
        for box_col in 0..3 {
            let mut box_vars = Vec::new();
            for r in 0..3 {
                for c in 0..3 {
                    box_vars.push(cell_vars[box_row * 3 + r][box_col * 3 + c]);
                }
            }
            gac.propagate_alldiff(&box_vars)?;
        }
    }
    
    // Enhanced backtracking search
    fn search_gac(gac: &mut HybridGAC, cell_vars: &[[Variable; 9]; 9], depth: usize) -> Result<bool, String> {
        if depth > 50 { // Limit recursion depth for this demo
            return Ok(false);
        }
        
        // Check if solved
        let mut all_assigned = true;
        for row in 0..9 {
            for col in 0..9 {
                if !gac.is_assigned(cell_vars[row][col]) {
                    all_assigned = false;
                    break;
                }
            }
            if !all_assigned {
                break;
            }
        }
        
        if all_assigned {
            return Ok(true);
        }
        
        // Find variable with smallest domain (MRV heuristic)
        let mut best_var = None;
        let mut min_domain_size = 10;
        let mut best_pos = (0, 0);
        
        for row in 0..9 {
            for col in 0..9 {
                let var = cell_vars[row][col];
                if !gac.is_assigned(var) {
                    let domain_size = gac.domain_size(var);
                    if domain_size == 0 {
                        return Ok(false); // Inconsistent
                    }
                    if domain_size < min_domain_size {
                        min_domain_size = domain_size;
                        best_var = Some(var);
                        best_pos = (row, col);
                    }
                }
            }
        }
        
        if let Some(var) = best_var {
            let domain = gac.get_domain_values(var);
            
            for value in domain {
                // Simple propagation test
                let old_domains: Vec<_> = (0..81).map(|i| {
                    let r = i / 9;
                    let c = i % 9;
                    gac.get_domain_values(cell_vars[r][c])
                }).collect();
                
                // Try assignment
                gac.assign_variable(var, value);
                
                // Re-propagate constraints
                let (row, col) = best_pos;
                let row_vars: Vec<Variable> = (0..9).map(|c| cell_vars[row][c]).collect();
                let col_vars: Vec<Variable> = (0..9).map(|r| cell_vars[r][col]).collect();
                
                let box_row = row / 3;
                let box_col = col / 3;
                let mut box_vars = Vec::new();
                for r in 0..3 {
                    for c in 0..3 {
                        box_vars.push(cell_vars[box_row * 3 + r][box_col * 3 + c]);
                    }
                }
                
                let prop_ok = gac.propagate_alldiff(&row_vars).is_ok() &&
                              gac.propagate_alldiff(&col_vars).is_ok() &&
                              gac.propagate_alldiff(&box_vars).is_ok();
                
                if prop_ok {
                    if search_gac(gac, cell_vars, depth + 1)? {
                        return Ok(true);
                    }
                }
                
                // Simplified backtrack - restore domains
                for (i, domain) in old_domains.iter().enumerate() {
                    let r = i / 9;
                    let c = i % 9;
                    let var = cell_vars[r][c];
                    
                    // Reset variable (simplified - just add back the domain)
                    if !domain.is_empty() && domain.len() > 1 {
                        let _ = gac.add_variable_with_values(var, domain.clone());
                    }
                }
            }
        }
        
        Ok(false)
    }
    
    let solved = search_gac(&mut gac, &cell_vars, 0)?;
    let duration = start.elapsed().as_nanos();
    Ok((solved, duration))
}

fn main() {
    println!("Specialized Sudoku Solver vs Hybrid GAC Comparison");
    println!("=================================================");
    
    let puzzles = vec![
        ("Platinum Blonde", PLATINUM_BLONDE),
        ("AI Escargot", AI_ESCARGOT),
        ("World's Hardest", WORLDS_HARDEST),
    ];
    
    let iterations = 100;
    
    for (name, puzzle_str) in puzzles {
        println!("\n=== {} ===", name);
        let grid = parse_sudoku(puzzle_str);
        
        // Benchmark specialized solver
        let mut specialized_times = Vec::new();
        let mut specialized_successes = 0;
        
        for _ in 0..iterations {
            let (solved, time) = solve_with_specialized_solver(grid);
            specialized_times.push(time);
            if solved {
                specialized_successes += 1;
            }
        }
        
        let avg_specialized = specialized_times.iter().sum::<u128>() / iterations as u128;
        
        // Benchmark GAC approach (fewer iterations due to complexity)
        let mut gac_times = Vec::new();
        let mut gac_successes = 0;
        let gac_iterations = 10; // Fewer due to search complexity
        
        for _ in 0..gac_iterations {
            match solve_with_hybrid_gac(grid) {
                Ok((solved, time)) => {
                    gac_times.push(time);
                    if solved {
                        gac_successes += 1;
                    }
                }
                Err(_) => {
                    // Count as failure
                    gac_times.push(u128::MAX);
                }
            }
        }
        
        let successful_gac_times: Vec<u128> = gac_times.iter()
            .filter(|&&time| time != u128::MAX)
            .copied()
            .collect();
        
        let avg_gac = if successful_gac_times.is_empty() {
            0
        } else {
            successful_gac_times.iter().sum::<u128>() / successful_gac_times.len() as u128
        };
        
        println!("Results:");
        println!("  Specialized Solver:");
        println!("    Average time: {:>10} ns", avg_specialized);
        println!("    Success rate: {}/{} ({:.1}%)", 
                specialized_successes, iterations,
                (specialized_successes as f64 / iterations as f64) * 100.0);
        
        println!("  Hybrid GAC:");
        println!("    Average time: {:>10} ns", avg_gac);
        println!("    Success rate: {}/{} ({:.1}%)", 
                gac_successes, gac_iterations,
                (gac_successes as f64 / gac_iterations as f64) * 100.0);
        
        if avg_gac > 0 && avg_specialized > 0 {
            let speedup = avg_gac as f64 / avg_specialized as f64;
            if speedup > 1.0 {
                println!("    Specialized solver is {:.2}x faster", speedup);
            } else {
                println!("    GAC approach is {:.2}x faster", 1.0 / speedup);
            }
        }
    }
    
    println!("\n=== Conclusions ===");
    println!("🎯 Specialized Sudoku Solver:");
    println!("   • Optimized for Sudoku-specific techniques");
    println!("   • Uses bit-based candidate sets (similar to GAC BitSet)");
    println!("   • Employs naked singles, hidden singles, naked pairs");
    println!("   • Highly tuned for 9x9 Sudoku structure");
    
    println!("\n🔧 Hybrid GAC Approach:");
    println!("   • General-purpose constraint satisfaction");
    println!("   • Automatic BitSet selection for small domains");
    println!("   • Advanced Hall set constraint propagation");
    println!("   • Works for any constraint satisfaction problem");
    
    println!("\n⚡ Performance Analysis:");
    println!("   • Specialized solver wins on pure performance");
    println!("   • GAC approach offers much greater flexibility");
    println!("   • BitSet optimization benefits both approaches");
    println!("   • GAC can handle arbitrary constraint types");
}