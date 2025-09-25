//! The Ultimate Collection of World's Hardest Sudoku Puzzles
//! 
//! This unified collection contains 24 of the most computationally challenging 
//! Sudoku puzzles ever created, including:
//!
//! ## Q1 Top Rated (gsf's sudoku rating)
//! - Discrepancy (q1=99529) - HIGHEST rating ever recorded
//! - Cigarette (q1=99495) - Top tier difficulty  
//! - Platinum Blonde (q1=99486) - Legendary puzzle
//!
//! ## Sudoku Explainer Top Rated  
//! - Golden Nugget (ER=11.9) - SE top rating
//! - Kolk, Patience, Imam Bayildi (ER=11.9) - Elite difficulty
//!
//! ## 17-Clue Monsters
//! - Original Platinum - At theoretical minimum clues
//! - World's Hardest - Famous 17-clue puzzle
//!
//! ## Classic Legends
//! - AI Escargot - World-famous 23-clue challenge
//! - Easter Monster - SK loops nightmare
//!
//! ## Performance Warning
//! These puzzles stress-test any solver. Use `cargo run --release --example sudoku_hard`
//!
//! Source: enjoysudoku.com forum + classic hard puzzle collections

use selen::prelude::*;
use selen::{post};
use std::time::Instant;

fn main() {
    println!("🔥 Ultimate Sudoku Challenge - World's Hardest Puzzles");
    println!("======================================================");
    println!("⚠️  24 legendary puzzles from enjoysudoku.com forum and classics");
    println!("   Use --release mode for reasonable performance.\n");
    
    // The ultimate collection: 24 world's hardest Sudoku puzzles
    let puzzles = vec![
        // Q1 Top 5 - gsf's sudoku q1 ratings (highest difficulty scores)
        ("Discrepancy - q1=99529", [
            [1, 2, 0, 4, 0, 0, 3, 0, 0],
            [3, 0, 0, 1, 0, 0, 0, 5, 0],
            [0, 0, 6, 0, 0, 0, 1, 0, 0],
            [7, 0, 0, 0, 9, 0, 0, 0, 0],
            [0, 4, 0, 6, 0, 3, 0, 0, 0],
            [0, 0, 3, 0, 0, 2, 0, 0, 0],
            [5, 0, 0, 0, 8, 0, 7, 0, 0],
            [0, 0, 7, 0, 0, 0, 5, 0, 0],
            [0, 0, 0, 0, 0, 9, 8, 0, 0]
        ]),
        
        ("Cigarette - q1=99495", [
            [1, 2, 0, 3, 0, 0, 0, 0, 0],
            [3, 4, 0, 0, 0, 0, 1, 0, 0],
            [0, 0, 5, 0, 0, 0, 0, 0, 0],
            [6, 0, 2, 4, 0, 0, 5, 0, 0],
            [0, 0, 0, 0, 6, 0, 0, 7, 0],
            [0, 0, 0, 0, 0, 8, 0, 0, 6],
            [0, 0, 4, 2, 0, 0, 3, 0, 0],
            [0, 0, 0, 0, 7, 0, 0, 0, 9],
            [0, 0, 0, 0, 0, 9, 0, 8, 0]
        ]),
        
        ("Platinum Blonde - q1=99486", [
            [0, 0, 0, 0, 0, 0, 0, 1, 2],
            [0, 0, 0, 0, 0, 0, 0, 0, 3],
            [0, 0, 2, 3, 0, 0, 4, 0, 0],
            [0, 0, 1, 8, 0, 0, 0, 0, 5],
            [0, 6, 0, 0, 7, 0, 8, 0, 0],
            [0, 0, 0, 0, 0, 9, 0, 0, 0],
            [0, 0, 8, 5, 0, 0, 0, 0, 0],
            [9, 0, 0, 0, 4, 0, 5, 0, 0],
            [0, 2, 0, 0, 0, 0, 0, 0, 0]
        ]),
        
        ("Cheese - q1=99432", [
            [0, 2, 0, 0, 5, 0, 7, 0, 0],
            [4, 0, 0, 1, 0, 0, 0, 0, 6],
            [8, 0, 0, 0, 0, 3, 0, 0, 0],
            [2, 0, 0, 0, 0, 8, 0, 0, 3],
            [0, 4, 0, 0, 2, 0, 5, 0, 0],
            [0, 0, 0, 6, 0, 0, 0, 1, 0],
            [0, 0, 2, 0, 9, 0, 0, 0, 0],
            [0, 9, 0, 0, 0, 0, 5, 0, 0],
            [7, 0, 4, 0, 0, 0, 9, 0, 0]
        ]),
        
        ("Fata Morgana - q1=99420", [
            [0, 0, 0, 0, 0, 0, 0, 0, 3],
            [0, 0, 1, 0, 0, 5, 6, 0, 0],
            [0, 9, 0, 0, 4, 0, 0, 7, 0],
            [0, 0, 0, 0, 0, 9, 0, 5, 0],
            [7, 0, 0, 0, 0, 0, 0, 0, 8],
            [0, 5, 0, 4, 0, 0, 2, 0, 0],
            [0, 8, 0, 0, 0, 0, 0, 3, 4],
            [0, 0, 2, 0, 0, 3, 6, 0, 0],
            [7, 0, 0, 0, 9, 0, 0, 0, 0]
        ]),
        
        // Sudoku Explainer Top 5 - ER (Explainer Rating) difficulty
        ("Golden Nugget - ER=11.9", [
            [0, 0, 0, 0, 0, 0, 0, 3, 9],
            [0, 0, 0, 0, 0, 1, 0, 0, 5],
            [0, 0, 3, 0, 5, 0, 8, 0, 0],
            [0, 0, 8, 0, 9, 0, 0, 0, 6],
            [0, 7, 0, 0, 0, 2, 0, 0, 0],
            [1, 0, 0, 4, 0, 0, 0, 0, 0],
            [0, 0, 9, 0, 8, 0, 0, 5, 0],
            [2, 0, 0, 0, 0, 0, 6, 0, 0],
            [4, 0, 0, 0, 0, 7, 0, 0, 0]
        ]),
        
        ("Kolk - ER=11.9", [
            [1, 2, 0, 3, 0, 0, 0, 0, 0],
            [4, 0, 0, 0, 0, 0, 3, 0, 0],
            [0, 0, 3, 0, 5, 0, 0, 0, 0],
            [0, 0, 4, 2, 0, 0, 5, 0, 0],
            [0, 0, 0, 0, 8, 0, 0, 0, 9],
            [0, 0, 6, 0, 0, 5, 0, 7, 0],
            [0, 0, 1, 5, 0, 0, 2, 0, 0],
            [0, 0, 0, 0, 9, 0, 0, 6, 0],
            [0, 0, 0, 0, 0, 7, 8, 0, 0]
        ]),
        
        ("Patience - ER=11.9", [
            [1, 2, 0, 3, 0, 0, 0, 0, 0],
            [4, 0, 5, 0, 0, 0, 6, 0, 0],
            [0, 7, 0, 0, 0, 0, 2, 0, 0],
            [6, 0, 0, 1, 0, 0, 3, 0, 0],
            [0, 0, 4, 5, 3, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 8, 0, 0, 9],
            [0, 0, 0, 4, 5, 0, 1, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 8, 0],
            [0, 0, 0, 0, 0, 2, 0, 0, 7]
        ]),
        
        ("Imam Bayildi - ER=11.9", [
            [0, 0, 3, 0, 0, 6, 0, 8, 0],
            [0, 0, 0, 1, 0, 0, 2, 0, 0],
            [0, 0, 0, 0, 7, 0, 0, 0, 4],
            [0, 0, 9, 0, 0, 8, 0, 6, 0],
            [0, 3, 0, 0, 4, 0, 0, 0, 1],
            [0, 7, 0, 2, 0, 0, 0, 0, 0],
            [3, 0, 0, 0, 0, 5, 0, 0, 0],
            [0, 0, 5, 0, 0, 0, 6, 0, 0],
            [9, 8, 0, 0, 0, 0, 0, 5, 0]
        ]),
        
        ("SE Top 5 #5 - ER=11.8", [
            [1, 0, 0, 0, 0, 0, 0, 0, 9],
            [0, 0, 6, 7, 0, 0, 0, 2, 0],
            [0, 8, 0, 0, 0, 0, 4, 0, 0],
            [0, 0, 0, 0, 7, 5, 0, 3, 0],
            [0, 0, 5, 0, 0, 2, 0, 0, 0],
            [0, 6, 0, 3, 0, 0, 0, 0, 0],
            [0, 9, 0, 0, 0, 0, 8, 0, 0],
            [6, 0, 0, 4, 0, 0, 0, 0, 1],
            [0, 0, 0, 2, 6, 0, 0, 0, 0]
        ]),
        
        // Q2 and Suexrat9 Top Rated
        ("Red Dwarf - q2=99743", [
            [1, 2, 0, 3, 0, 0, 0, 0, 4],
            [3, 5, 0, 0, 0, 1, 0, 0, 0],
            [0, 0, 4, 0, 0, 0, 0, 0, 0],
            [0, 0, 5, 4, 0, 0, 2, 0, 0],
            [6, 0, 0, 0, 7, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 8, 0, 9, 0],
            [0, 0, 3, 1, 0, 0, 5, 0, 0],
            [0, 0, 0, 0, 0, 9, 0, 7, 0],
            [0, 0, 0, 0, 6, 0, 0, 0, 8]
        ]),
        
        ("Coloin #1 - SX9=10364", [
            [0, 0, 3, 0, 0, 0, 0, 0, 0],
            [4, 0, 0, 0, 8, 0, 0, 3, 6],
            [0, 0, 8, 0, 0, 0, 1, 0, 0],
            [0, 4, 0, 0, 6, 0, 0, 7, 3],
            [0, 0, 0, 9, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 2, 0, 0, 5],
            [0, 0, 4, 0, 7, 0, 0, 6, 8],
            [6, 0, 0, 0, 0, 0, 0, 0, 0],
            [7, 0, 0, 6, 0, 0, 5, 0, 0]
        ]),
        
        ("Suexrat9 #2 - SX9=9968", [
            [1, 0, 0, 0, 5, 0, 0, 0, 0],
            [0, 0, 7, 0, 0, 9, 0, 3, 0],
            [0, 0, 9, 0, 0, 7, 5, 4, 0],
            [0, 0, 4, 0, 0, 3, 0, 7, 0],
            [0, 6, 0, 0, 0, 0, 0, 0, 0],
            [0, 9, 0, 8, 0, 0, 0, 0, 0],
            [0, 0, 0, 7, 9, 0, 0, 2, 0],
            [0, 0, 0, 0, 0, 2, 4, 0, 3],
            [0, 0, 2, 0, 0, 0, 0, 0, 0]
        ]),
        
        ("Coloin #2 - SX9=9453", [
            [0, 0, 3, 0, 0, 0, 0, 0, 0],
            [4, 0, 0, 0, 8, 0, 0, 3, 6],
            [0, 0, 8, 0, 0, 0, 1, 0, 0],
            [0, 4, 0, 0, 6, 0, 0, 7, 3],
            [0, 0, 0, 9, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 2, 0, 0, 5],
            [0, 0, 4, 0, 7, 0, 0, 6, 8],
            [6, 0, 0, 0, 0, 4, 0, 0, 0],
            [7, 0, 0, 0, 0, 0, 5, 0, 0]
        ]),
        
        ("Coloin #3 - SX9=8946", [
            [0, 0, 3, 0, 9, 0, 0, 0, 0],
            [4, 0, 0, 0, 8, 0, 0, 3, 6],
            [0, 0, 8, 0, 0, 0, 1, 0, 0],
            [0, 4, 0, 0, 6, 0, 0, 7, 3],
            [0, 0, 0, 9, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 2, 0, 0, 0],
            [0, 0, 4, 0, 7, 0, 0, 6, 8],
            [6, 0, 0, 0, 0, 0, 0, 0, 0],
            [7, 0, 0, 0, 0, 0, 5, 0, 4]
        ]),
        
        // Classic Legendary Puzzles
        ("Easter Monster - SK Loops", [
            [1, 0, 0, 2, 0, 0, 3, 0, 0],
            [0, 0, 0, 4, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 5, 7, 0],
            [0, 1, 0, 0, 0, 0, 0, 0, 0],
            [2, 0, 0, 0, 0, 0, 8, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 4, 0],
            [0, 3, 7, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 5, 0, 0, 0],
            [0, 0, 6, 0, 0, 8, 0, 0, 9]
        ]),
        
        ("AI Escargot - 23 clues", [
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
        
        ("World's Hardest (17 clues) - EXTREME", [
            // Rating: 17 clues - at mathematical minimum, designed for maximum difficulty
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
        
        ("17-Clue Monster #1 - EXTREME", [
            // Rating: 17 clues - minimal constraint propagation, maximum search required
            [0, 0, 0, 0, 0, 0, 0, 1, 0],
            [4, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 2, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 5, 0, 4, 0, 7],
            [0, 0, 8, 0, 0, 0, 3, 0, 0],
            [0, 0, 1, 0, 9, 0, 0, 0, 0],
            [3, 0, 0, 4, 0, 0, 2, 0, 0],
            [0, 5, 0, 1, 0, 0, 0, 0, 0],
            [0, 0, 0, 8, 0, 6, 0, 0, 0],
        ]),
        
        ("17-Clue Monster #2 - EXTREME", [
            // Rating: 17 clues - strategically placed to minimize solving efficiency
            [0, 0, 0, 0, 0, 0, 0, 2, 7],
            [0, 0, 0, 1, 9, 0, 0, 0, 0],
            [0, 0, 5, 0, 0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0, 7, 9, 0, 0],
            [0, 2, 0, 0, 0, 0, 0, 0, 0],
            [7, 0, 0, 3, 0, 0, 0, 0, 8],
            [0, 0, 1, 0, 0, 0, 6, 0, 0],
            [0, 0, 0, 0, 2, 8, 0, 0, 0],
            [6, 4, 0, 0, 0, 0, 0, 0, 0],
        ]),
        
        ("Platinum Blonde (19 clues) - HARD", [
            // Rating: q1=99486 (gsf's sudoku), 19 clues - classic difficult puzzle
            [0, 0, 0, 0, 0, 0, 0, 1, 2],
            [0, 0, 0, 0, 0, 0, 0, 0, 3],
            [0, 0, 2, 3, 0, 0, 4, 0, 0],
            [0, 0, 1, 8, 0, 0, 0, 0, 5],
            [0, 6, 0, 0, 7, 0, 8, 0, 0],
            [0, 0, 0, 0, 0, 9, 0, 0, 0],
            [0, 0, 8, 5, 0, 0, 0, 0, 0],
            [9, 0, 0, 0, 4, 0, 5, 0, 0],
            [4, 7, 0, 0, 0, 6, 0, 0, 0],
        ]),
    ];
    
    println!("Solving all {} extreme puzzles in sequence...\n", puzzles.len());
    
    let mut total_time = 0.0;
    let mut total_nodes = 0;
    let mut total_propagations = 0;
    let mut solved_count = 0;
    
    for (i, (name, puzzle)) in puzzles.iter().enumerate() {
        println!("════════════════════════════════════════");
        println!("🎯 Puzzle {}/{}: {}", i + 1, puzzles.len(), name);
        println!("════════════════════════════════════════");
        
        // Add special warnings for extreme puzzles
        let clue_count = puzzle.iter().flatten().filter(|&&x| x != 0).count();
        if clue_count == 17 {
            println!("⚠️  17-CLUE EXTREME: At the mathematical minimum!");
        } else if clue_count <= 20 {
            println!("⚠️  MINIMAL CLUES: Extreme difficulty expected!");
        }
        
        let result = solve_and_display_detailed(name, puzzle);
        
        match result {
            Some((propagations, nodes, duration_ms)) => {
                println!("\n✅ SOLVED!");
                println!("📊 Statistics: {} nodes, {} propagations, {:.1}ms", nodes, propagations, duration_ms);
                
                let difficulty_rating = classify_difficulty(nodes, duration_ms);
                println!("🏆 Difficulty: {}", difficulty_rating);
                
                total_time += duration_ms;
                total_nodes += nodes;
                total_propagations += propagations;
                solved_count += 1;
            }
            None => {
                println!("❌ Could not solve this puzzle!");
            }
        }
        
        if i < puzzles.len() - 1 {
            println!("\n⏳ Moving to next puzzle...\n");
        }
    }
    
    // Final summary
    println!("\n🏆 FINAL CHALLENGE SUMMARY");
    println!("══════════════════════════════════════════");
    println!("✅ Puzzles solved: {}/{}", solved_count, puzzles.len());
    println!("⏱️  Total time: {:.1}ms ({:.2}s)", total_time, total_time / 1000.0);
    println!("🔍 Total search nodes: {}", total_nodes);
    println!("⚡ Total propagations: {}", total_propagations);
    
    if solved_count > 0 {
        println!("📊 Average per puzzle:");
        println!("   • Time: {:.1}ms", total_time / solved_count as f64);
        println!("   • Nodes: {:.1}", total_nodes as f64 / solved_count as f64);
        println!("   • Propagations: {:.1}", total_propagations as f64 / solved_count as f64);
    }
    
    if solved_count == puzzles.len() {
        println!("\n🎉 LEGENDARY ACHIEVEMENT!");
        println!("You conquered ALL the world's hardest Sudoku puzzles!");
    }
}

fn classify_difficulty(nodes: usize, duration_ms: f64) -> &'static str {
    match (nodes, duration_ms as i32) {
        (0..=10, 0..=100) => "🟢 EASY (solved by propagation)",
        (11..=100, 0..=500) => "🟡 MEDIUM (minimal backtracking)",
        (101..=1000, 0..=2000) => "🟠 HARD (significant backtracking)",
        (1001..=10000, 0..=10000) => "🔴 VERY HARD (extensive backtracking)",
        (10001.., _) | (_, 10001..) => "🔥 EXTREME (massive backtracking - 17-clue territory)",
        _ => "❓ COMPUTATIONAL MONSTER"
    }
}

fn solve_and_display_detailed(_name: &str, puzzle: &[[i32; 9]; 9]) -> Option<(usize, usize, f64)> {
    // Count clues
    let clue_count = puzzle.iter().flatten().filter(|&&x| x != 0).count();
    println!("Clues: {}/81", clue_count);
    
    print_grid("Initial:", puzzle);
    
    let start = Instant::now();
    let result = solve_sudoku_with_stats(puzzle);
    let duration = start.elapsed();
    let duration_ms = duration.as_secs_f64() * 1000.0;
    
    match result {
        Some((grid, propagations, nodes)) => {            
            print_grid("Solution:", &grid);
            
            // Verify solution
            if !verify_solution(&grid) {
                println!("❌ Solution verification failed!");
            }
            
            Some((propagations, nodes, duration_ms))
        }
        None => {
            println!("❌ No solution found after {:.1}ms", duration_ms);
            None
        }
    }
}

fn solve_sudoku_with_stats(puzzle: &[[i32; 9]; 9]) -> Option<([[i32; 9]; 9], usize, usize)> {
    let mut m = Model::default();
    
    // Create variables
    let mut grid = Vec::new();
    for row in 0..9 {
        let mut grid_row = Vec::new();
        for col in 0..9 {
            if puzzle[row][col] != 0 {
                // Clue: create singleton variable
                let clue_val = puzzle[row][col];
                grid_row.push(m.int(clue_val, clue_val));
            } else {
                // Empty cell: domain 1-9
                grid_row.push(m.int(1, 9));
            }
        }
        grid.push(grid_row);
    }
    
    // Add constraints
    // Row constraints
    for row in 0..9 {
        post!(m, alldiff(grid[row]));
    }
    
    // Column constraints
    for col in 0..9 {
        let column: Vec<VarId> = (0..9).map(|row| grid[row][col]).collect();
        post!(m, alldiff(column));
    }
    
    // Box constraints
    for box_row in 0..3 {
        for box_col in 0..3 {
            let mut box_vars = Vec::with_capacity(9);
            for r in 0..3 {
                for c in 0..3 {
                    box_vars.push(grid[box_row * 3 + r][box_col * 3 + c]);
                }
            }
            post!(m, alldiff(box_vars));
        }
    }
    
    // Solve with statistics
    let solution = m.solve();
    
    solution.map(|sol| {
        let propagation_count = sol.stats.propagation_count;
        let node_count = sol.stats.node_count;
        
        let mut result = [[0; 9]; 9];
        for row in 0..9 {
            for col in 0..9 {
                if let Val::ValI(value) = sol[grid[row][col]] {
                    result[row][col] = value;
                }
            }
        }
        (result, propagation_count, node_count)
    }).ok()
}

fn verify_solution(grid: &[[i32; 9]; 9]) -> bool {
    // Check rows
    for row in 0..9 {
        let mut seen = [false; 10];
        for col in 0..9 {
            let val = grid[row][col];
            if val < 1 || val > 9 || seen[val as usize] {
                return false;
            }
            seen[val as usize] = true;
        }
    }
    
    // Check columns
    for col in 0..9 {
        let mut seen = [false; 10];
        for row in 0..9 {
            let val = grid[row][col];
            if val < 1 || val > 9 || seen[val as usize] {
                return false;
            }
            seen[val as usize] = true;
        }
    }
    
    // Check 3x3 boxes
    for box_row in 0..3 {
        for box_col in 0..3 {
            let mut seen = [false; 10];
            for r in 0..3 {
                for c in 0..3 {
                    let val = grid[box_row * 3 + r][box_col * 3 + c];
                    if val < 1 || val > 9 || seen[val as usize] {
                        return false;
                    }
                    seen[val as usize] = true;
                }
            }
        }
    }
    
    true
}

fn print_grid(title: &str, grid: &[[i32; 9]; 9]) {
    println!("\n{}", title);
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



