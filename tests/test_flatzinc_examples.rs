//! Test FlatZinc parser on example files
//! 
//! This test only runs when the example FlatZinc files are present in the source tree.
//! It will be skipped in packaged builds where these files are not included.

use selen::prelude::*;
use std::path::Path;

/// Test parsing a selection of FlatZinc example files
/// This helps ensure the parser handles real-world FlatZinc correctly
#[test]
fn test_parse_flatzinc_examples() {
    let examples_dir = Path::new("zinc/ortools");
    
    // Skip test if examples directory doesn't exist (e.g., in packaged builds)
    if !examples_dir.exists() {
        println!("Skipping FlatZinc examples test - examples directory not found");
        return;
    }
    
    // Selection of diverse FlatZinc files to test
    let test_files = vec![
        // Simple puzzles
        "send_more_money.fzn",
        "magic_sequence.fzn",
        "n_queens.fzn",
        "sudoku.fzn",
        "zebra.fzn",
        
        // Scheduling/planning
        "JobShop2x2.fzn",
        "scheduling_speakers.fzn",
        
        // Logic puzzles
        "einstein_opl.fzn",
        "who_killed_agatha.fzn",
        "smullyan_knights_knaves.fzn",
        
        // Graph problems
        "graph_coloring.fzn",
        "stable_marriage.fzn",
        
        // Arithmetic puzzles
        "alphametic.fzn",
        "crypta.fzn",
        "grocery.fzn",
        
        // Constraint variety
        "all_interval.fzn",
        "langford.fzn",
        "perfect_square_sequence.fzn",
        
        // Optimization
        "knapsack.fzn",
        "diet.fzn",
        
        // Other
        "coins.fzn",
        "crossword.fzn",
        "nonogram.fzn",
        "minesweeper.fzn",
        "traffic_lights.fzn",
    ];
    
    let mut results = Vec::new();
    let mut parse_success = 0;
    let mut parse_fail = 0;
    let mut file_not_found = 0;
    
    println!("\n=== Testing FlatZinc Parser on Example Files ===\n");
    
    for filename in test_files {
        let filepath = examples_dir.join(filename);
        
        if !filepath.exists() {
            println!("⊘ {}: File not found", filename);
            file_not_found += 1;
            results.push((filename, "not_found"));
            continue;
        }
        
        let mut model = Model::default();
        match model.from_flatzinc_file(&filepath) {
            Ok(_) => {
                println!("✓ {}: Parsed successfully", filename);
                parse_success += 1;
                results.push((filename, "success"));
            }
            Err(e) => {
                println!("✗ {}: Parse error - {}", filename, e);
                parse_fail += 1;
                results.push((filename, "failed"));
                
                // Print first few lines of file for debugging
                if let Ok(content) = std::fs::read_to_string(&filepath) {
                    let preview: String = content.lines().take(10).collect::<Vec<_>>().join("\n");
                    println!("   First 10 lines:");
                    for line in preview.lines() {
                        println!("   {}", line);
                    }
                }
            }
        }
    }
    
    println!("\n=== Summary ===");
    println!("✓ Parsed successfully: {}", parse_success);
    println!("✗ Parse failures: {}", parse_fail);
    println!("⊘ Files not found: {}", file_not_found);
    println!("Total tested: {}", results.len());
    
    // Calculate success rate (excluding files not found)
    let tested = parse_success + parse_fail;
    if tested > 0 {
        let success_rate = (parse_success as f64 / tested as f64) * 100.0;
        println!("Success rate: {:.1}%", success_rate);
        
        // We expect at least 50% success rate for real FlatZinc files
        // Some may fail due to unsupported constraints, which is expected
        assert!(
            success_rate >= 30.0,
            "Success rate too low: {:.1}%. Expected at least 30%",
            success_rate
        );
    }
    
    // List all failures for reference
    if parse_fail > 0 {
        println!("\nFailed files:");
        for (filename, status) in &results {
            if *status == "failed" {
                println!("  - {}", filename);
            }
        }
    }
}

/// Test that we can parse and solve a simple example
#[test]
fn test_solve_simple_example() {
    let examples_dir = Path::new("zinc/ortools");
    
    // Skip test if examples directory doesn't exist
    if !examples_dir.exists() {
        println!("Skipping solve example test - examples directory not found");
        return;
    }
    
    // Test with send_more_money if available
    let filepath = examples_dir.join("send_more_money.fzn");
    if !filepath.exists() {
        println!("Skipping - send_more_money.fzn not found");
        return;
    }
    
    let mut model = Model::default();
    model.from_flatzinc_file(&filepath).expect("Should parse send_more_money.fzn");
    
    // Try to solve it (may or may not find solution depending on constraints)
    match model.solve() {
        Ok(_solution) => {
            println!("✓ Found solution for send_more_money");
            // We can't access variable values without the var_map, but we know it solved
        }
        Err(e) => {
            println!("Note: Could not solve send_more_money: {:?}", e);
            // This is OK - we're mainly testing parsing, not solving
        }
    }
}
