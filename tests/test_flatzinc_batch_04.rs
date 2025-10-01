//! Test FlatZinc parser - Batch 04: N-Queens variants
//! Tests various N-Queens problems

use selen::prelude::*;
use std::path::Path;

#[test]
fn test_batch_04_queens() {
    let examples_dir = Path::new("src/zinc/flatzinc");
    
    if !examples_dir.exists() {
        println!("Skipping test - examples directory not found");
        return;
    }
    
    let test_files = vec![
        "queens3.fzn",
        "queens4.fzn",
        "queens_viz.fzn",
        "queens_ip.fzn",
        "queen_ip.fzn",
        "queen_cp2.fzn",
        "kqueens.fzn",
        "squeens.fzn",
        "dqueens.fzn",
        "non_dominating_queens.fzn",
        "peacableArmyOfQueens.fzn",
    ];
    
    let mut success = 0;
    let mut failed = 0;
    let mut not_found = 0;
    
    println!("\n=== Batch 04: N-Queens Variants ===\n");
    
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
