//! Test FlatZinc parser - Batch 06: Scheduling and planning
//! Tests scheduling and job shop problems

use selen::prelude::*;
use std::path::Path;

#[test]
fn test_batch_06_scheduling() {
    let examples_dir = Path::new("src/zinc/flatzinc");
    
    if !examples_dir.exists() {
        println!("Skipping test - examples directory not found");
        return;
    }
    
                let test_files = vec![
        "logic_puzzle_aop.fzn",
        "longest_change.fzn",
        "lucky_number.fzn",
        "M12.fzn",
        "magic3.fzn",
        "magic4.fzn",
        "magic.fzn",
        "magic_modulo_number.fzn",
        "magic_sequence2.fzn",
        "magic_sequence3.fzn",
        "magic_sequence4.fzn",
        "magic_sequence.fzn",
        "magicsq_3.fzn",
        "magicsq_4.fzn",
        "magicsq_5.fzn",
        "magic_square_frenicle_form.fzn",
        "magic_square.fzn",
        "magic_squares_and_cards.fzn",
        "mamas_age.fzn",
        "mango_puzzle.fzn",
        "map2.fzn",
        "map_coloring_with_costs.fzn",
        "map.fzn",
        "map_stuckey.fzn",
        "marathon2.fzn",
        "marathon.fzn",
        "matchmaker.fzn",
        "matrix2num.fzn",
        "max_cut.fzn",
        "maxflow.fzn",
        "max_flow_taha.fzn",
        "max_flow_winston1.fzn",
        "maximal_independent_sets.fzn",
        "maximum_density_still_life.fzn",
        "maximum_modulo.fzn",
        "maximum_subarray.fzn",
        "max_index.fzn",
        "max_m_in_row.fzn",
        "max_n.fzn",
        "max_nvalue.fzn",
        "max_size_set_of_consecutive_var.fzn",
        "mceverywhere.fzn",
        "message_sending.fzn",
        "mfasp.fzn",
        "mfvsp.fzn",
        "minesweeper_0.fzn",
        "minesweeper_1.fzn",
        "minesweeper_2.fzn",
        "minesweeper_3.fzn",
        "minesweeper_4.fzn",
        "minesweeper_5.fzn",
        "minesweeper_6.fzn",
        "minesweeper_7.fzn",
        "minesweeper_8.fzn",
        "minesweeper_9.fzn",
        "minesweeper_basic3.fzn",
        "minesweeper_basic4.fzn",
        "minesweeper_basic4x4.fzn",
        "minesweeper_config_page2.fzn",
        "minesweeper_config_page3.fzn",
        "minesweeper.fzn",
        "minesweeper_german_Lakshtanov.fzn",
        "minesweeper_inverse.fzn",
        "minesweeper_splitter.fzn",
        "minesweeper_wire.fzn",
        "minimum_except_0.fzn",
        "minimum_greater_than.fzn",
        "minimum_modulo.fzn",
        "minimum_weight_alldifferent.fzn",
        "min_index.fzn",
        "min_n.fzn",
        "min_nvalue.fzn",
        "misp.fzn",
        "missing_digit.fzn",
        "mixing_party.fzn",
        "money_change.fzn",
        "monkey_coconuts.fzn",
        "monks_and_doors.fzn",
        "movie_stars.fzn",
        "mr_smith.fzn",
        "multidimknapsack_simple.fzn",
        "multipl.fzn",
        "murder.fzn",
        "music_men.fzn",
        "mvcp.fzn",
        "my_precedence.fzn",
    ];
    
    let mut success = 0;
    let mut failed = 0;
    let mut not_found = 0;
    
    println!("\n=== Batch 06: Scheduling and Planning ===\n");
    
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
