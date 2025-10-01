//! Test FlatZinc parser - Batch 05: Magic sequences and numbers
//! Tests magic sequence and magic square problems

use selen::prelude::*;
use std::path::Path;

#[test]
fn test_batch_05_magic() {
    let examples_dir = Path::new("src/zinc/flatzinc");
    
    if !examples_dir.exists() {
        println!("Skipping test - examples directory not found");
        return;
    }
    
    let test_files = vec![
        "magic_sequence.fzn",
        "magic_sequence2.fzn",
        "magic_sequence3.fzn",
        "magic_sequence4.fzn",
        "magic_square.fzn",
        "magic_square_frenicle_form.fzn",
        "magic_squares_and_cards.fzn",
        "magic.fzn",
        "magic3.fzn",
        "magic4.fzn",
        "magicsq_3.fzn",
        "magicsq_4.fzn",
        "magicsq_5.fzn",
        "another_kind_of_magic_square.fzn",
        "franklin_8x8_magic_square.fzn",
    ];
    
    let mut success = 0;
    let mut failed = 0;
    let mut not_found = 0;
    
    println!("\n=== Batch 05: Magic Sequences and Squares ===\n");
    
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
