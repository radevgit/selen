//! Test FlatZinc parser - Batch 01: Simple arithmetic puzzles
//! Tests files that are likely to have basic constraints

use selen::prelude::*;
use std::path::Path;

#[test]
fn test_batch_01_simple_arithmetic() {
    let examples_dir = Path::new("src/zinc/flatzinc");
    
    if !examples_dir.exists() {
        println!("Skipping test - examples directory not found");
        return;
    }
    
    let test_files = vec![
        "send_more_money.fzn",
        "send_more_money2.fzn",
        "send_more_money_any_base.fzn",
        "send_more_money_ip.fzn",
        "send_most_money.fzn",
        "donald.fzn",
        "crypta.fzn",
        "crypto.fzn",
        "crypto_ip.fzn",
        "alpha.fzn",
    ];
    
    let mut success = 0;
    let mut failed = 0;
    let mut not_found = 0;
    
    println!("\n=== Batch 01: Simple Arithmetic Puzzles ===\n");
    
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
    println!("Success rate: {}/{} ({:.1}%)", 
             success, test_files.len() - not_found,
             100.0 * success as f64 / (test_files.len() - not_found) as f64);
}
