//! Test FlatZinc parser - Batch 01: Simple arithmetic puzzles
//! Tests files that are likely to have basic constraints

use selen::prelude::*;
use std::path::Path;

#[test]
#[ignore]
fn test_batch_01_simple_arithmetic() {
    let examples_dir = Path::new("zinc/ortools");
    
    if !examples_dir.exists() {
        println!("Skipping test - examples directory not found");
        return;
    }
    
    let test_files = vec![
        "1d_rubiks_cube.fzn",
        "2DPacking.fzn",
        "3_coins.fzn",
        "3_jugs2_all.fzn",
        "3_jugs2.fzn",
        "3_jugs.fzn",
        "50_puzzle.fzn",
        "5x5_puzzle.fzn",
        "99_bottles_of_beer.fzn",
        "abbott.fzn",
        "abc_endview.fzn",
        "abpuzzle.fzn",
        "added_corner.fzn",
        "adjacency_matrix_from_degrees.fzn",
        "ages2.fzn",
        "alien.fzn",
        "alldifferent_consecutive_values.fzn",
        "alldifferent_cst.fzn",
        "alldifferent_except_0.fzn",
        "alldifferent_interval.fzn",
        "all_different_modulo.fzn",
        "alldifferent_modulo.fzn",
        "alldifferent_on_intersection.fzn",
        "alldifferent_same_value.fzn",
        "alldifferent_soft.fzn",
        "all_differ_from_at_least_k_pos.fzn",
        "all_equal_me.fzn",
        "all_interval1.fzn",
        "all_interval2.fzn",
        "all_interval3.fzn",
        "all_interval4.fzn",
        "all_interval5.fzn",
        "all_interval6.fzn",
        "all_interval.fzn",
        "all_min_dist.fzn",
        "allocating_developments.fzn",
        "all_paths_graph.fzn",
        "allperm.fzn",
        "alpha.fzn",
        "among_diff_0.fzn",
        "among_interval.fzn",
        "among_low_up.fzn",
        "among_modulo.fzn",
        "among_seq.fzn",
        "and.fzn",
        "another_kind_of_magic_square.fzn",
        "antisymmetric.fzn",
        "a_puzzle.fzn",
        "arch_friends.fzn",
        "argmax.fzn",
        "arith.fzn",
        "arithmetic_ring.fzn",
        "arith_or.fzn",
        "arith_sliding.fzn",
        "a_round_of_golf.fzn",
        "arrow.fzn",
        "artificial_intelligence.fzn",
        "assign_and_counts.fzn",
        "assign_and_nvalues.fzn",
        "assignment2_2.fzn",
        "assignment2.fzn",
        "assignment3.fzn",
        "assignment4.fzn",
        "assignment5.fzn",
        "assignment6.fzn",
        "assignment.fzn",
        "atom_smasher.fzn",
        "averbach_1.2.fzn",
        "averbach_1.3.fzn",
        "averbach_1.4.fzn",
        "averbach_1.5.fzn",
        "averback_1.4.fzn",
        "babysitting.fzn",
        "balanced_brackets.fzn",
        "balanced_matrix.fzn",
        "balance.fzn",
        "balance_interval.fzn",
        "balance_modulo.fzn",
        "bales_of_hay.fzn",
        "bank_card.fzn",
        "battleships10.fzn",
        "battleships_1.fzn",
        "battleships_2.fzn",
        "battleships_3.fzn",
        "battleships_4.fzn",
        "battleships_5.fzn",
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
