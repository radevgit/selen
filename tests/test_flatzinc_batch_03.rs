//! Test FlatZinc parser - Batch 03: Logic puzzles
//! Tests zebra, einstein, and other logic puzzles

use selen::prelude::*;
use std::path::Path;

#[test]
#[ignore]
fn test_batch_03_logic_puzzles() {
    let examples_dir = Path::new("zinc/ortools");
    
    if !examples_dir.exists() {
        println!("Skipping test - examples directory not found");
        return;
    }
    
    let test_files = vec![
        "crossword_bratko.fzn",
        "crossword.fzn",
        "crowd.fzn",
        "crypta.fzn",
        "crypto.fzn",
        "crypto_ip.fzn",
        "cube_sum.fzn",
        "cumulative_test.fzn",
        "cumulative_test_mats_carlsson.fzn",
        "curious_set_of_integers.fzn",
        "cur_num.fzn",
        "cutstock.fzn",
        "cutting_stock_winston.fzn",
        "cycle_test2.fzn",
        "czech_logical_labyrinth.fzn",
        "debruijn2d_2.fzn",
        "debruijn2d_3.fzn",
        "debruijn2d.fzn",
        "debruijn2.fzn",
        "debruijn_binary.fzn",
        "debruijn_mike_winter2.fzn",
        "debruijn_mike_winter3.fzn",
        "debruijn_no_repetition.fzn",
        "decision_tree_binary.fzn",
        "decreasing_me.fzn",
        "defending_castle.fzn",
        "dennys_menu.fzn",
        "derangement.fzn",
        "devils_word.fzn",
        "diet1.fzn",
        "differs_from_at_least_k_pos.fzn",
        "diffn_me.fzn",
        "digital_roots.fzn",
        "digits_of_the_square.fzn",
        "dimes.fzn",
        "dinner.fzn",
        "disjunctive.fzn",
        "distance_between.fzn",
        "distance_change.fzn",
        "dividing_the_spoils.fzn",
        "divisible_by_7.fzn",
        "divisible_by_9_trough_1.fzn",
        "domain_constraint.fzn",
        "domain.fzn",
        "donald.fzn",
        "dqueens.fzn",
        "drinking_game.fzn",
        "dudeney_bishop_placement1.fzn",
        "dudeney_bishop_placement2.fzn",
        "dudeney_numbers.fzn",
        "earthlin.fzn",
        "egg_basket.fzn",
        "einav_puzzle.fzn",
        "ein_ein_ein_ein_vier.fzn",
        "einstein_hurlimann.fzn",
        "einstein_opl.fzn",
        "element_greatereq.fzn",
        "element_lesseq.fzn",
        "element_matrix.fzn",
        "elementn.fzn",
        "element_product.fzn",
        "elements_alldifferent.fzn",
        "elements.fzn",
        "element_sparse.fzn",
        "elevator_6_3.fzn",
        "elevator_8_4.fzn",
        "eliza_pseudonym7.fzn",
        "enclosed_tiles.fzn",
        "enigma_1000.fzn",
        "enigma_1001.fzn",
        "enigma_1293.fzn",
        "enigma_1530.fzn",
        "enigma_1535.fzn",
        "enigma_1553.fzn",
        "enigma_1555.fzn",
        "enigma_1557.fzn",
        "enigma_1568.fzn",
        "enigma_1570.fzn",
        "enigma_1573.fzn",
        "enigma_1574.fzn",
        "enigma_1575.fzn",
        "enigma_1576.fzn",
        "enigma_1577.fzn",
        "enigma_843.fzn",
        "enigma_birthday_magic.fzn",
        "enigma_circular_chain.fzn",
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
