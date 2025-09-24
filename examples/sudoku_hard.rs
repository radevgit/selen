//! Collection of the World's Most Difficult Sudoku Puzzles
//! 
//! This example contains some of the most computationally challenging Sudoku puzzles
//! ever created. These puzzles are designed to stress-test constraint solvers and
//! require significant backtracking to solve.
//!
//! ## Famous Hard Sudokus Included:
//! 
//! 1. **"AI Escargot"** by Arto Inkala - Often cited as one of the hardest
//! 2. **"Platinum Blonde"** by Arto Inkala - Extremely difficult, minimal clues
//! 3. **"17-clue puzzles"** - Minimal clue puzzles at the mathematical limit
//! 4. **"Easter Monster"** - Another notorious difficult puzzle
//!
//! ## Performance Warning
//! 
//! **These puzzles can take MINUTES to solve even in release mode!**
//! Use `cargo run --release --example hardest_sudokus` for best performance.

use selen::prelude::*;
use selen::{post};
use std::time::Instant;

fn main() {
    println!("🔥 World's Most Difficult Sudoku Puzzles");
    println!("========================================");
    println!("⚠️  WARNING: These puzzles are computationally intensive!");
    println!("   Use --release mode for reasonable performance.\n");
    
    // Collection of the world's hardest Sudoku puzzles
    let puzzles = vec![
        ("Original Platinum (17 clues) - EXTREME", 
         "000000000000003085001020000000507000004000100090000000500000073002010000000040009"),
        
        ("AI Escargot (Arto Inkala) - 23 clues", 
         "100007090030020008009600500005300900010080002600004000300000010040000007007000300"),
        
        ("World's Hardest (17 clues) - EXTREME", 
         "800000000003600000070090200050007000000045700000100030001000068008500010090000400"),
        
        ("17-Clue Monster #1 - EXTREME", 
         "000000010400000000020000000000050407008000300001090000300400200050100000000806000"),
        
        ("17-Clue Monster #2 - EXTREME", 
         "000000027000190000005000100000007900020000000700300008001000600000028000640000000"),
        
        ("Platinum Blonde (19 clues) - HARD", 
         "000000012000000003002300400001800005060070800000009000008500000900040500020000000"),
    ];
    
    println!("Available puzzles:");
    for (i, (name, _)) in puzzles.iter().enumerate() {
        println!("{}. {}", i + 1, name);
    }
    println!();
    
    // Solve the first puzzle (AI Escargot) as an example
    let (name, puzzle_str) = &puzzles[1];
    println!("🎯 Solving: {}", name);
    println!("⚠️  This is a 17-CLUE puzzle - at the mathematical minimum!");
    println!("   These require massive backtracking and can take SECONDS to solve.\n");
    
    let puzzle = parse_sudoku_string(puzzle_str);
    let result = solve_and_display_detailed(name, &puzzle);
    
    match result {
        Some((propagations, nodes, duration_ms)) => {
            println!("\n🏆 CHALLENGE COMPLETED!");
            println!("Difficulty Analysis:");
            println!("• Search Nodes: {} (higher = more backtracking required)", nodes);
            println!("• Propagations: {} (constraint propagation steps)", propagations);
            println!("• Time: {:.1}ms", duration_ms);
            
            let difficulty_rating = classify_difficulty(nodes, duration_ms);
            println!("• Difficulty: {}", difficulty_rating);
        }
        None => {
            println!("❌ Puzzle could not be solved (may be invalid)");
        }
    }
    
    println!("\n💡 To try other puzzles, modify the code to use different indices!");
    println!("   Example: Change puzzles[0] to puzzles[1] for Platinum Blonde");
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

/// Parse a sudoku string (81 characters) into a 9x9 grid
fn parse_sudoku_string(puzzle_str: &str) -> [[i32; 9]; 9] {
    assert_eq!(puzzle_str.len(), 81, "Sudoku string must be exactly 81 characters");
    
    let mut grid = [[0; 9]; 9];
    
    for (i, ch) in puzzle_str.chars().enumerate() {
        let row = i / 9;
        let col = i % 9;
        grid[row][col] = ch.to_digit(10).expect("Invalid character in sudoku string") as i32;
    }
    
    grid
}

fn solve_and_display_detailed(name: &str, puzzle: &[[i32; 9]; 9]) -> Option<(usize, usize, f64)> {
    // Count clues
    let clue_count = puzzle.iter().flatten().filter(|&&x| x != 0).count();
    println!("📊 Puzzle Analysis:");
    println!("   • Clues given: {}/81", clue_count);
    println!("   • Empty cells: {}/81", 81 - clue_count);
    println!("   • Difficulty factor: {} clues (17 is theoretical minimum)", clue_count);
    
    print_grid("Initial Puzzle:", puzzle);
    
    println!("🔍 Solving... (this may take a while for hard puzzles)");
    let start = Instant::now();
    let result = solve_sudoku_with_stats(puzzle);
    let duration = start.elapsed();
    let duration_ms = duration.as_secs_f64() * 1000.0;
    
    match result {
        Some((grid, propagations, nodes)) => {
            println!("✅ SOLVED in {:.1}ms!", duration_ms);
            
            // Performance analysis
            if nodes == 0 {
                println!("🎯 Pure constraint propagation - no search required!");
            } else {
                let efficiency = propagations as f64 / nodes as f64;
                println!("📊 Search Statistics:");
                println!("   • Propagations: {}", propagations);
                println!("   • Search nodes: {}", nodes);
                println!("   • Efficiency: {:.1} propagations/node", efficiency);
            }
            
            print_grid("Solution:", &grid);
            
            // Verify solution
            if verify_solution(&grid) {
                println!("✅ Solution verified correct!");
            } else {
                println!("❌ Warning: Solution verification failed!");
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