//! Performance investigation comparing EXTREME vs PLATINUM puzzles
//! This test uses basic timing to identify where the 1000x slowdown occurs

use cspsolver::prelude::*;
use std::time::Instant;

fn main() {
    println!("🔍 Performance Investigation: EXTREME vs PLATINUM");
    println!("=================================================");
    
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
    
    println!("\n1️⃣  Testing EXTREME puzzle...");
    let (extreme_time, extreme_stats) = solve_with_timing("EXTREME", &extreme_puzzle);
    
    println!("\n2️⃣  Testing PLATINUM puzzle...");
    let (platinum_time, platinum_stats) = solve_with_timing("PLATINUM", &platinum_puzzle);
    
    println!("\n📊 COMPARISON RESULTS:");
    println!("==========================================");
    compare_performance(extreme_time, extreme_stats, platinum_time, platinum_stats);
}

fn solve_with_timing(name: &str, puzzle: &[[i32; 9]; 9]) -> (std::time::Duration, cspsolver::solution::SolveStats) {
    let total_start = Instant::now();
    
    // Build the model with timing
    let model_start = Instant::now();
    let mut model = Model::default();
    let mut grid = [[model.new_var_int(1, 9); 9]; 9];
    
    for row in 0..9 {
        for col in 0..9 {
            grid[row][col] = model.new_var_int(1, 9);
        }
    }
    
    // Add clue constraints
    for row in 0..9 {
        for col in 0..9 {
            if puzzle[row][col] != 0 {
                model.equals(grid[row][col], Val::int(puzzle[row][col]));
            }
        }
    }
    
    // Add sudoku constraints
    for row in 0..9 {
        model.all_different(grid[row].to_vec());
    }
    
    for col in 0..9 {
        let column: Vec<VarId> = (0..9).map(|row| grid[row][col]).collect();
        model.all_different(column);
    }
    
    for box_row in 0..3 {
        for box_col in 0..3 {
            let mut box_vars = Vec::new();
            for r in 0..3 {
                for c in 0..3 {
                    box_vars.push(grid[box_row * 3 + r][box_col * 3 + c]);
                }
            }
            model.all_different(box_vars);
        }
    }
    
    let model_time = model_start.elapsed();
    
    // Solve with detailed timing
    let solve_start = Instant::now();
    let mut stats = cspsolver::solution::SolveStats::default();
    
    let solution = model.solve_with_callback(|solve_stats| {
        stats = solve_stats.clone();
        // Print progress every few seconds to see if it's stuck
        if solve_stats.node_count % 5 == 0 && solve_stats.node_count > 0 {
            let elapsed = solve_start.elapsed();
            println!("   Progress: {} nodes, {} props, {:.1}s elapsed", 
                     solve_stats.node_count, solve_stats.propagation_count, 
                     elapsed.as_secs_f64());
        }
    });
    
    let solve_time = solve_start.elapsed();
    let total_time = total_start.elapsed();
    
    println!("📋 {} Results:", name);
    println!("   Model setup: {:.3}ms", model_time.as_secs_f64() * 1000.0);
    println!("   Solve time: {:.3}ms", solve_time.as_secs_f64() * 1000.0);
    println!("   Total time: {:.3}ms", total_time.as_secs_f64() * 1000.0);
    println!("   Propagations: {}, Nodes: {}", stats.propagation_count, stats.node_count);
    
    if solution.is_some() {
        println!("   ✅ Solution found!");
    } else {
        println!("   ❌ No solution found!");
    }
    
    (total_time, stats)
}

fn compare_performance(
    extreme_time: std::time::Duration, extreme_stats: cspsolver::solution::SolveStats,
    platinum_time: std::time::Duration, platinum_stats: cspsolver::solution::SolveStats
) {
    let time_ratio = platinum_time.as_nanos() as f64 / extreme_time.as_nanos() as f64;
    let prop_ratio = platinum_stats.propagation_count as f64 / extreme_stats.propagation_count as f64;
    let node_ratio = platinum_stats.node_count as f64 / extreme_stats.node_count as f64;
    
    println!("⏱️  Time comparison:");
    println!("   EXTREME: {:.3}ms", extreme_time.as_secs_f64() * 1000.0);
    println!("   PLATINUM: {:.3}ms", platinum_time.as_secs_f64() * 1000.0);
    println!("   RATIO: {:.1}x slower", time_ratio);
    
    println!("\n🔄 Work comparison:");
    println!("   Propagations: {:.2}x ratio ({} vs {})", prop_ratio, extreme_stats.propagation_count, platinum_stats.propagation_count);
    println!("   Nodes: {:.2}x ratio ({} vs {})", node_ratio, extreme_stats.node_count, platinum_stats.node_count);
    
    if extreme_stats.propagation_count > 0 {
        let extreme_time_per_prop = extreme_time.as_nanos() as f64 / extreme_stats.propagation_count as f64;
        let platinum_time_per_prop = platinum_time.as_nanos() as f64 / platinum_stats.propagation_count as f64;
        let time_per_prop_ratio = platinum_time_per_prop / extreme_time_per_prop;
        
        println!("   Time per propagation: {:.1}x ratio ({:.1}ns vs {:.1}ns)", 
                 time_per_prop_ratio, extreme_time_per_prop, platinum_time_per_prop);
    }
    
    println!("\n🚨 ANALYSIS:");
    if time_ratio > 10.0 && prop_ratio < 2.0 && node_ratio < 2.0 {
        println!("   ⚠️  PERFORMANCE BUG DETECTED!");
        println!("   Similar work ({:.1}x propagations, {:.1}x nodes)", prop_ratio, node_ratio);
        println!("   But {:.1}x longer time - suggests implementation issue", time_ratio);
        println!("   Likely causes:");
        println!("   - Memory allocation inefficiency");
        println!("   - Algorithmic complexity issue"); 
        println!("   - Constraint evaluation bottleneck");
        println!("   - Empty row handling inefficiency");
        println!("\n   🔍 Key insight: PLATINUM has completely empty first row!");
        println!("      This might trigger pathological behavior in:");
        println!("      - Variable ordering heuristics");
        println!("      - Constraint propagation patterns");
        println!("      - Memory allocation patterns");
    } else {
        println!("   ✅ Performance difference explained by work complexity");
    }
}
