//! Test FlatZinc parser - Batch 03: Logic puzzles
//! Tests zebra, einstein, and other logic puzzles

use selen::prelude::*;
use std::path::Path;

#[test]
fn test_batch_03_logic_puzzles() {
    let examples_dir = Path::new("src/zinc/flatzinc");
    
    if !examples_dir.exists() {
        println!("Skipping test - examples directory not found");
        return;
    }
    
    let test_files = vec![
        "zebra.fzn",
        "zebra_inverse.fzn",
        "zebra_ip.fzn",
        "einstein_opl.fzn",
        "einstein_hurlimann.fzn",
        "who_killed_agatha.fzn",
        "smullyan_knights_knaves.fzn",
        "smullyan_knights_knaves_normals.fzn",
        "smullyan_knights_knaves_normals_bahava.fzn",
        "smullyan_lion_and_unicorn.fzn",
        "smullyan_portia.fzn",
    ];
    
    let mut success = 0;
    let mut failed = 0;
    let mut not_found = 0;
    
    println!("\n=== Batch 03: Logic Puzzles ===\n");
    
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
