//! Test FlatZinc parser - Batch 02: Sudoku and grid puzzles
//! Tests sudoku variants and similar grid-based puzzles

use selen::prelude::*;
use std::path::Path;

#[test]
fn test_batch_02_sudoku() {
    let examples_dir = Path::new("src/zinc/flatzinc");
    
    if !examples_dir.exists() {
        println!("Skipping test - examples directory not found");
        return;
    }
    
    let test_files = vec![
        "sudoku.fzn",
        "sudoku_alldifferent.fzn",
        "sudoku_gcc.fzn",
        "sudoku_ip.fzn",
        "sudoku_pi.fzn",
        "sudoku_pi_2008.fzn",
        "sudoku_pi_2010.fzn",
        "sudoku_pi_2011.fzn",
        "sudoku_25x25_250.fzn",
        "killer_sudoku.fzn",
        "killer_sudoku2.fzn",
    ];
    
    let mut success = 0;
    let mut failed = 0;
    let mut not_found = 0;
    
    println!("\n=== Batch 02: Sudoku Puzzles ===\n");
    
    for filename in &test_files {
        let filepath = examples_dir.join(filename);
        
        if !filepath.exists() {
            println!("⊘ {}", filename);
            not_found += 1;
            continue;
        }
        
        let mut model = Model::default();
        match model.from_flatzinc_file(&filepath) {
            Ok(_) => {
                println!("✓ {}", filename);
                success += 1;
            }
            Err(e) => {
                println!("✗ {} - {}", filename, e);
                failed += 1;
            }
        }
    }
    
    println!("\nResults: {} success, {} failed, {} not found", success, failed, not_found);
    if success + failed > 0 {
        println!("Success rate: {}/{} ({:.1}%)", 
                 success, success + failed,
                 100.0 * success as f64 / (success + failed) as f64);
    }
}
