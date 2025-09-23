//! N-Queens Problem Solver
//!
//! This example demonstrates solving the classic N-Queens problem using AllDifferent constraints.
//! The N-Queens problem involves placing N queens on an N×N chessboard such that no two queens
//! attack each other (same row, column, or diagonal).
//!
//! This is an excellent test case for AllDifferent constraint optimization because:
//! - Multiple AllDifferent constraints with different variable connectivity patterns
//! - Diagonal constraints create asymmetric constraint graphs
//! - Scales well to test optimization effectiveness across problem sizes
//!
//! Problem formulation:
//! - Variables: queen_row[i] = row position of queen in column i (1-based)
//! - Constraints:
//!   1. AllDifferent(queen_row) - no two queens in same row
//!   2. AllDifferent(queen_row[i] + i) - no two queens on same ascending diagonal
//!   3. AllDifferent(queen_row[i] - i) - no two queens on same descending diagonal

use selen::prelude::*;
use std::time::Instant;

fn main() {
    println!("👑 N-Queens Problem Solver");
    println!("=========================\n");

    // Test different board sizes to validate optimization across problem scales
    let test_sizes = [4, 6, 8];
    
    for &n in &test_sizes {
        println!("🔍 Solving {}-Queens problem:", n);
        println!("📋 Problem stats: {} queens, {} variables, {} AllDifferent constraints", n, n, 3);
        
        let start = Instant::now();
        match solve_n_queens(n) {
            Some((solution, stats)) => {
                let duration = start.elapsed();
                println!("✅ Solution found in {:.3}ms!", duration.as_secs_f64() * 1000.0);
                println!("📊 Statistics: {} propagations, {} nodes explored", stats.propagations, stats.nodes);
                println!("🔍 Efficiency: {:.1} propagations/node", stats.propagations as f64 / stats.nodes.max(1) as f64);
                
                print_board(&solution, n);
                println!("");
            }
            None => {
                println!("❌ No solution found for {}-Queens", n);
            }
        }
        println!("──────────────────────────────────────────────────");
    }
    
    // Performance test with larger sizes
    println!("\n🚀 Performance Testing:");
    let performance_sizes = [10, 12, 20];
    
    for &n in &performance_sizes {
        println!("\n🔍 Performance test: {}-Queens", n);
        let start = Instant::now();
        match solve_n_queens(n) {
            Some((_, stats)) => {
                let duration = start.elapsed();
                println!("✅ Solved in {:.3}ms", duration.as_secs_f64() * 1000.0);
                println!("📊 {} propagations, {} nodes, {:.1} prop/node", 
                    stats.propagations, stats.nodes, 
                    stats.propagations as f64 / stats.nodes.max(1) as f64);
            }
            None => {
                println!("❌ No solution found");
            }
        }
    }
    
    println!("\n✨ Summary:");
    println!("N-Queens demonstrates AllDifferent optimization with:");
    println!("• Multiple AllDifferent constraints with different connectivity patterns");
    println!("• Row constraint: simple 1-to-1 variable mapping");
    println!("• Diagonal constraints: transformed variables with different connectivity");
    println!("• Universal optimization applies across all constraint types");
    println!("• 🎯 Perfect test case for validating general-purpose heuristics!");
}

struct SolverStats {
    propagations: usize,
    nodes: usize,
}

fn solve_n_queens(n: usize) -> Option<(Vec<i32>, SolverStats)> {
    let mut model = Model::default();
    
    // Variables: queen_row[i] = row position of queen in column i
    let queen_rows: Vec<_> = (0..n)
        .map(|_| model.int(1, n as i32))
        .collect();
    
    // Constraint 1: All queens must be in different rows
    // This is the most direct AllDifferent constraint
    post!(model, alldiff(queen_rows.clone()));
    
    // Constraint 2: No two queens on the same ascending diagonal
    // Ascending diagonal: queen_row[i] + i must be different for all i
    let ascending_diagonals: Vec<_> = queen_rows.iter().enumerate()
        .map(|(i, &queen_row)| {
            let col_offset = model.int(i as i32, i as i32);
            model.add(queen_row, col_offset)
        })
        .collect();
    post!(model, alldiff(ascending_diagonals));
    
    // Constraint 3: No two queens on the same descending diagonal  
    // Descending diagonal: queen_row[i] - i must be different for all i
    let descending_diagonals: Vec<_> = queen_rows.iter().enumerate()
        .map(|(i, &queen_row)| {
            let col_offset = model.int(-(i as i32), -(i as i32));
            model.add(queen_row, col_offset)
        })
        .collect();
    post!(model, alldiff(descending_diagonals));
    
    // Solve the model with statistics tracking
    // Solve the model with embedded statistics
    let solution = model.solve();
    
    match solution {
        Ok(sol) => {
            // Access statistics from the solution
            let propagation_count = sol.stats.propagation_count;
            let node_count = sol.stats.node_count;
            
            // Extract queen positions
            let positions: Vec<i32> = queen_rows.iter()
                .map(|&var| match sol[var] {
                    Val::ValI(row) => row,
                    _ => 0,
                })
                .collect();
            
            // Get solver statistics
            let stats = SolverStats {
                propagations: propagation_count,
                nodes: node_count,
            };
            
            Some((positions, stats))
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
    
    // Print the board
    for row in 1..=n {
        print!("{:2} ", row);
        for col in 0..n {
            if solution[col] == row as i32 {
                print!(" ♛ ");
            } else {
                print!(" · ");
            }
        }
        println!();
    }
    
    // Print solution in compact form
    print!("🎯 Queen positions: ");
    for (col, &row) in solution.iter().enumerate() {
        print!("C{}R{}", col + 1, row);
        if col < solution.len() - 1 {
            print!(", ");
        }
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4_queens() {
        let solution = solve_n_queens(4);
        assert!(solution.is_some(), "4-Queens should have a solution");
        
        if let Some((positions, _)) = solution {
            // Verify solution validity
            assert_eq!(positions.len(), 4);
            
            // Check no two queens in same row
            let mut rows = positions.clone();
            rows.sort();
            for i in 1..rows.len() {
                assert_ne!(rows[i], rows[i-1], "Two queens in same row");
            }
            
            // Check diagonals
            for i in 0..4 {
                for j in i+1..4 {
                    // Ascending diagonal check
                    assert_ne!(positions[i] + i as i32, positions[j] + j as i32, 
                              "Two queens on same ascending diagonal");
                    // Descending diagonal check  
                    assert_ne!(positions[i] - i as i32, positions[j] - j as i32,
                              "Two queens on same descending diagonal");
                }
            }
        }
    }

    #[test]
    fn test_8_queens() {
        let solution = solve_n_queens(8);
        assert!(solution.is_some(), "8-Queens should have a solution");
        
        if let Some((_, stats)) = solution {
            // Performance sanity check - 8-Queens should solve reasonably quickly
            assert!(stats.propagations < 10000, "Too many propagations for 8-Queens");
            assert!(stats.nodes < 1000, "Too many nodes for 8-Queens");
        }
    }

    #[test]
    fn test_optimization_effectiveness() {
        // This test validates that our AllDifferent optimization is working
        // by ensuring reasonable performance on larger instances
        let solution = solve_n_queens(10);
        assert!(solution.is_some(), "10-Queens should have a solution");
        
        if let Some((_, stats)) = solution {
            // With good optimization, 10-Queens should be solvable efficiently
            println!("10-Queens stats: {} propagations, {} nodes", stats.propagations, stats.nodes);
            // These bounds are generous but validate that optimization is preventing exponential blowup
            assert!(stats.propagations < 50000, "Optimization may not be working - too many propagations");
            assert!(stats.nodes < 5000, "Optimization may not be working - too many nodes");
        }
    }
}