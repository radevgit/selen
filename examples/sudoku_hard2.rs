//! The Hardest Sudoku Puzzles from enjoysudoku.com Forum
//! 
//! This collection contains the most computationally challenging Sudoku puzzles
//! from the famous "The hardest sudokus (new thread)" forum thread at
//! http://forum.enjoysudoku.com/the-hardest-sudokus-new-thread-t6539.html
//!
//! These puzzles are rated by multiple difficulty rating programs:
//! - **q1/q2**: gsf's sudoku rating (higher is harder, 99000+ is extreme)
//! - **ER**: Sudoku Explainer rating (11.6+ is extremely hard)
//! - **SX9/SXT**: dukuso's suexrat ratings
//!
//! ## Performance Warning
//! 
//! **These are the ABSOLUTE HARDEST puzzles known to the Sudoku community!**
//! Some may take MINUTES even in release mode. Use `cargo run --release --example sudoku_hard2`

use selen::prelude::*;
use std::time::Instant;

fn main() {
    println!("🌟 The Hardest Sudoku Puzzles from enjoysudoku.com Forum");
    println!("=======================================================");
    println!("🔥 These are the ABSOLUTE HARDEST puzzles known to humanity!");
    println!("⚠️  WARNING: Extreme computational intensity - use --release mode!\n");
    
    // The hardest Sudoku puzzles from the enjoysudoku.com forum
    // Each puzzle includes name, rating info, and source details
    let puzzles = vec![
        // Q1 Top 5 - gsf's sudoku q1 ratings
        ("Discrepancy", 
         // Rating: q1=99529 by eleven (HardestSudokusThread-02085)
         [
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
        
        ("Cigarette", 
         // Rating: q1=99495 by eleven (HardestSudokusThread-02023)
         [
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
        
        ("Platinum Blonde", 
         // Rating: q1=99486 by coloin (HardestSudokusThread-00078)
         [
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
        
        ("Cheese", 
         // Rating: q1=99432 by eleven (HardestSudokusThread-00209)
         [
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
        
        ("Fata Morgana", 
         // Rating: q1=99420 by tarek (HardestSudokusThread-00041)
         [
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
        
        // SE Top 5 - Sudoku Explainer ratings
        ("Golden Nugget", 
         // Rating: ER=11.9 by tarek (pearly6000-1812)
         [
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
        
        ("Kolk", 
         // Rating: ER=11.9 by eleven (HardestSudokusThread-00208)
         [
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
        
        ("Patience", 
         // Rating: ER=11.9 by eleven (HardestSudokusThread-02095)
         [
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
        
        ("Imam Bayildi", 
         // Rating: ER=11.9 by eleven (HardestSudokusThread-00211)
         [
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
        
        ("SE Top 5 #5", 
         // Rating: ER=11.8 by eleven (HardestSudokusThread-00212)
         [
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
        
        // Q2 Top 5 - gsf's sudoku q2 ratings  
        ("Red Dwarf", 
         // Rating: q2=99743 by eleven (HardestSudokusThread-00245)
         [
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
        
        // Suexrat9 Top 5 - dukuso's suexrat9 ratings
        ("Coloin #1", 
         // Rating: SX9=10364 by eleven (HardestSudokusThread-01418)
         [
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
        
        ("Suexrat9 #2", 
         // Rating: SX9=9968 by eleven (HardestSudokusThread-02087)
         [
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
        
        ("Coloin #2", 
         // Rating: SX9=9453 by eleven (HardestSudokusThread-01419)
         [
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
        
        ("Coloin #3", 
         // Rating: SX9=8946 by coloin (HardestSudokusThread-02061)
         [
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
        
        // Additional extremely hard puzzles mentioned in the thread
        ("Easter Monster", 
         // Classic extremely difficult puzzle with SK loops
         [
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
        
        ("AI Escargot", 
         // Rating: Often cited as one of the hardest, 23 clues
         [
             [1, 0, 0, 0, 0, 7, 0, 9, 0],
             [0, 3, 0, 0, 2, 0, 0, 0, 8],
             [0, 0, 9, 6, 0, 0, 5, 0, 0],
             [0, 0, 5, 3, 0, 0, 9, 0, 0],
             [0, 1, 0, 0, 8, 0, 0, 0, 2],
             [6, 0, 0, 0, 0, 4, 0, 0, 0],
             [3, 0, 0, 0, 0, 0, 0, 1, 0],
             [0, 4, 0, 0, 0, 0, 0, 0, 7],
             [0, 0, 7, 0, 0, 0, 3, 0, 0]
         ]),
        
        ("World's Hardest", 
         // 17-clue monster
         [
             [8, 0, 0, 0, 0, 0, 0, 0, 0],
             [0, 0, 3, 6, 0, 0, 0, 0, 0],
             [0, 7, 0, 0, 9, 0, 2, 0, 0],
             [0, 5, 0, 0, 0, 7, 0, 0, 0],
             [0, 0, 0, 0, 4, 5, 7, 0, 0],
             [0, 0, 0, 1, 0, 0, 0, 3, 0],
             [0, 0, 1, 0, 0, 0, 0, 6, 8],
             [0, 0, 8, 5, 0, 0, 0, 1, 0],
             [0, 9, 0, 0, 0, 0, 4, 0, 0]
         ]),
    ];
    
    println!("� Solving ALL {} legendary forum puzzles in sequence!", puzzles.len());
    println!("   This is the ultimate Sudoku endurance challenge!\n");
    
    let mut total_time = 0.0;
    let mut total_nodes = 0;
    let mut total_propagations = 0;
    let mut solved_count = 0;
    let mut extreme_count = 0; // puzzles with >50k nodes
    let mut forum_legends = 0; // puzzles with >10k nodes
    
    for (i, (name, puzzle)) in puzzles.iter().enumerate() {
        println!("════════════════════════════════════════════════════");
        println!("🎯 Forum Legend {}/{}: {}", i + 1, puzzles.len(), name);
        println!("════════════════════════════════════════════════════");
        
        // Add specific rating info based on puzzle name
        match name {
            &"Discrepancy" => println!("🏆 q1=99529 - HIGHEST rating ever recorded!"),
            &"Cigarette" => println!("🏆 q1=99495 - Top q1 rating!"),
            &"Platinum Blonde" => println!("� q1=99486 - Legendary difficulty!"),
            &"Golden Nugget" => println!("🏆 ER=11.9 - Sudoku Explainer top rating!"),
            &"AI Escargot" => println!("🏆 World-famous 23-clue monster!"),
            &"World's Hardest" => println!("🏆 17-clue extreme - theoretical minimum!"),
            _ => {}
        }
        
        let result = solve_and_display_detailed(name, puzzle);
        
        match result {
            Some((propagations, nodes, duration_ms)) => {
                println!("\n✅ FORUM LEGEND CONQUERED!");
                println!("📊 Battle Statistics:");
                println!("   • Search Nodes: {} (backtracking steps)", nodes);
                println!("   • Propagations: {} (constraint checks)", propagations);
                println!("   • Battle Time: {:.1}ms", duration_ms);
                
                let difficulty_rating = classify_forum_difficulty(nodes, duration_ms);
                println!("   • Forum Rating: {}", difficulty_rating);
                
                // Track difficulty categories
                if nodes > 50000 {
                    extreme_count += 1;
                    println!("   🔥 EXTREME FORUM MONSTER - Massive search required!");
                } else if nodes > 10000 {
                    forum_legends += 1;
                    println!("   🔴 TRUE FORUM LEGEND - Significant challenge!");
                } else {
                    println!("   🟡 Your solver conquered this efficiently!");
                }
                
                total_time += duration_ms;
                total_nodes += nodes;
                total_propagations += propagations;
                solved_count += 1;
            }
            None => {
                println!("❌ This forum legend proved too challenging!");
            }
        }
        
        if i < puzzles.len() - 1 {
            println!("\n⏳ Preparing for next forum legend...\n");
        }
    }
    
    // Epic final summary
    println!("\n🏆 ULTIMATE FORUM CHALLENGE COMPLETE!");
    println!("════════════════════════════════════════════════════");
    println!("✅ Forum Legends Conquered: {}/{}", solved_count, puzzles.len());
    println!("⏱️  Total Battle Time: {:.1}ms ({:.2}s)", total_time, total_time / 1000.0);
    println!("🔍 Total Search Nodes: {}", total_nodes);
    println!("⚡ Total Propagations: {}", total_propagations);
    println!("🔥 Extreme Monsters (>50k nodes): {}", extreme_count);
    println!("🔴 Forum Legends (>10k nodes): {}", forum_legends);
    
    if solved_count > 0 {
        println!("\n� Forum Challenge Statistics:");
        println!("   • Average Time: {:.1}ms per puzzle", total_time / solved_count as f64);
        println!("   • Average Nodes: {:.1} per puzzle", total_nodes as f64 / solved_count as f64);
        println!("   • Average Propagations: {:.1} per puzzle", total_propagations as f64 / solved_count as f64);
        
        if total_time > 60000.0 {
            println!("   • Total Duration: {:.1} minutes of pure computation!", total_time / 60000.0);
        }
    }
    
    if solved_count == puzzles.len() {
        println!("\n🎉 LEGENDARY FORUM MASTER!");
        println!("You have conquered EVERY puzzle from the enjoysudoku.com hall of fame!");
        println!("This achievement puts you among the elite Sudoku computational masters!");
    }
    println!("\n🌐 Source: http://forum.enjoysudoku.com/the-hardest-sudokus-new-thread-t6539.html");
}

fn classify_forum_difficulty(nodes: usize, duration_ms: f64) -> &'static str {
    match (nodes, duration_ms as i32) {
        (0..=50, 0..=100) => "🟢 EASY (forum puzzles should not be this easy!)",
        (51..=500, 0..=1000) => "🟡 MEDIUM (below forum standards)",
        (501..=5000, 0..=5000) => "🟠 HARD (approaching forum difficulty)",
        (5001..=50000, 0..=30000) => "🔴 VERY HARD (forum-level difficulty!)",
        (50001.., _) | (_, 30001..) => "🔥 EXTREME (legendary forum monster!)",
        _ => "🌟 COMPUTATIONAL LEGEND (off the charts!)"
    }
}



fn solve_and_display_detailed(name: &str, puzzle: &[[i32; 9]; 9]) -> Option<(usize, usize, f64)> {
    // Count clues
    let clue_count = puzzle.iter().flatten().filter(|&&x| x != 0).count();
    println!("📊 Puzzle Analysis - {}:", name);
    println!("   • Given clues: {}/81", clue_count);
    println!("   • Empty cells: {}/81", 81 - clue_count);
    
    // Clue analysis for forum puzzles
    match clue_count {
        17 => println!("   🔥 17-CLUE MONSTER: At the theoretical minimum!"),
        18..=20 => println!("   🔴 MINIMAL CLUES: Extremely difficult territory!"),
        21..=25 => println!("   🟠 LOW CLUE COUNT: Forum-level difficulty!"),
        _ => println!("   🟡 Moderate clues: Should still be challenging!"),
    }
    
    print_grid("Forum Puzzle:", puzzle);
    
    println!("🔍 Solving forum legend... (this is the real deal!)");
    let start = Instant::now();
    let result = solve_sudoku_with_stats(puzzle);
    let duration = start.elapsed();
    let duration_ms = duration.as_secs_f64() * 1000.0;
    
    match result {
        Some((grid, propagations, nodes)) => {
            println!("✅ SOLVED in {:.1}ms!", duration_ms);
            
            // Performance analysis with forum context
            if nodes == 0 {
                println!("🎯 Pure propagation solved it - this puzzle may not be forum-caliber!");
            } else if nodes > 100000 {
                println!("🔥 LEGENDARY: {} nodes - this is forum monster territory!", nodes);
            } else if nodes > 10000 {
                println!("🔴 FORUM-WORTHY: {} nodes explored - true difficulty!", nodes);
            } else {
                println!("🟡 MODERATE: {} nodes - your solver is efficient!", nodes);
            }
            
            let efficiency = if nodes > 0 { 
                propagations as f64 / nodes as f64 
            } else { 
                propagations as f64 
            };
            println!("📊 Efficiency: {:.1} propagations per search node", efficiency);
            
            print_grid("Forum Solution:", &grid);
            
            // Verify solution
            if verify_solution(&grid) {
                println!("✅ Solution verified - you conquered a forum legend!");
            } else {
                println!("❌ Warning: Solution verification failed!");
            }
            
            Some((propagations, nodes, duration_ms))
        }
        None => {
            println!("❌ No solution found after {:.1}ms", duration_ms);
            println!("   This puzzle may be at the limits of computational feasibility!");
            None
        }
    }
}

fn solve_sudoku_with_stats(puzzle: &[[i32; 9]; 9]) -> Option<([[i32; 9]; 9], usize, usize)> {
    let mut m = Model::default();
    
    // Create variables with efficient singleton handling for clues
    let mut grid = Vec::new();
    for row in 0..9 {
        let mut grid_row = Vec::new();
        for col in 0..9 {
            if puzzle[row][col] != 0 {
                // Clue: create singleton variable for maximum efficiency
                let clue_val = puzzle[row][col];
                grid_row.push(m.int(clue_val, clue_val));
            } else {
                // Empty cell: full domain 1-9
                grid_row.push(m.int(1, 9));
            }
        }
        grid.push(grid_row);
    }
    
    // Add Sudoku constraints
    // Row constraints: each row contains 1-9 exactly once
    for row in 0..9 {
        m.alldiff(&grid[row]);
    }
    
    // Column constraints: each column contains 1-9 exactly once
    for col in 0..9 {
        let column: Vec<VarId> = (0..9).map(|row| grid[row][col]).collect();
        m.alldiff(&column);
    }
    
    // Box constraints: each 3x3 box contains 1-9 exactly once
    for box_row in 0..3 {
        for box_col in 0..3 {
            let mut box_vars = Vec::with_capacity(9);
            for r in 0..3 {
                for c in 0..3 {
                    box_vars.push(grid[box_row * 3 + r][box_col * 3 + c]);
                }
            }
            m.alldiff(&box_vars);
        }
    }
    
    // Solve with comprehensive statistics
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
    // Verify all Sudoku constraints are satisfied
    
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