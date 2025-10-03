//! Test FlatZinc parser - Batch 02: Sudoku and grid puzzles
//! Tests sudoku variants and similar grid-based puzzles

use selen::prelude::*;
use std::path::Path;

#[test]
#[ignore]
fn test_batch_02_sudoku() {
    let examples_dir = Path::new("zinc/ortools");
    
    if !examples_dir.exists() {
        println!("Skipping test - examples directory not found");
        return;
    }
    
    let test_files = vec![
        "battleships_6.fzn",
        "battleships_7.fzn",
        "battleships_8.fzn",
        "battleships_9.fzn",
        "best_shuffle.fzn",
        "between_min_max.fzn",
        "binary_matrix2array.fzn",
        "binary_tree.fzn",
        "binero.fzn",
        "bin_packing2.fzn",
        "bin_packing_me.fzn",
        "birthdays_2010.fzn",
        "birthdays_coins.fzn",
        "bit_vector1.fzn",
        "blending_problem.fzn",
        "blocksworld_instance_1.fzn",
        "blocksworld_instance_2.fzn",
        "blueberry_muffins.fzn",
        "bobs_sale.fzn",
        "bokus_competition.fzn",
        "book_buy.fzn",
        "bpp.fzn",
        "breaking_news.fzn",
        "bridges_to_somewhere.fzn",
        "broken_weights.fzn",
        "buckets.fzn",
        "bug_unsat.fzn",
        "building_a_house2.fzn",
        "building_a_house.fzn",
        "building_a_house_model.fzn",
        "building_blocks.fzn",
        "bus.fzn",
        "bus_scheduling_csplib.fzn",
        "bus_scheduling.fzn",
        "calculs_d_enfer.fzn",
        "calvin_puzzle.fzn",
        "candles.fzn",
        "capital_budget2.fzn",
        "cardinality_atleast.fzn",
        "cardinality_atmost.fzn",
        "car.fzn",
        "car_painting.fzn",
        "catalan_numbers.fzn",
        "change.fzn",
        "change_pair.fzn",
        "checker_puzzle.fzn",
        "chessset.fzn",
        "choose_your_crew.fzn",
        "circling_squares.fzn",
        "circuit_path.fzn",
        "circuit_test.fzn",
        "circular_change.fzn",
        "clock_triplets.fzn",
        "coins3.fzn",
        "coins_41_58.fzn",
        "coins.fzn",
        "coins_grid.fzn",
        "coins_problem.fzn",
        "collatz2.fzn",
        "collatz.fzn",
        "coloring_ip.fzn",
        "color_simple.fzn",
        "col_sum_puzzle.fzn",
        "combinatorial_auction.fzn",
        "common.fzn",
        "common_interval.fzn",
        "cond_lex_cost.fzn",
        "cond_lex_less.fzn",
        "config.fzn",
        "congress.fzn",
        "connected.fzn",
        "consecutive_digits.fzn",
        "consecutive_values.fzn",
        "constraint.fzn",
        "contains_array.fzn",
        "contiguity_regular.fzn",
        "contractor_costs.fzn",
        "correspondence.fzn",
        "costas_array.fzn",
        "count_ctr.fzn",
        "counts.fzn",
        "crew.fzn",
        "critical_path1.fzn",
        "crossbar.fzn",
        "crossfigure.fzn",
        "crossword2.fzn",
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
