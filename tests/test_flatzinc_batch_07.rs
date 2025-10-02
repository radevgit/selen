//! Test FlatZinc parser - Batch 07: Graph problems
//! Tests graph coloring, paths, and graph-based puzzles

use selen::prelude::*;
use std::path::Path;

#[test]
fn test_batch_07_graph() {
    let examples_dir = Path::new("zinc/ortools");
    
    if !examples_dir.exists() {
        println!("Skipping test - examples directory not found");
        return;
    }
    
                let test_files = vec![
        "nadel.fzn",
        "narcissistic_numbers.fzn",
        "n_change.fzn",
        "nchange.fzn",
        "newspaper0.fzn",
        "newspaper.fzn",
        "next_element.fzn",
        "next_greater_element.fzn",
        "nim.fzn",
        "nine_digit_arrangement.fzn",
        "nine_to_one_equals_100.fzn",
        "non_dominating_queens.fzn",
        "nonogram_create_automaton2.fzn",
        "nonogram.fzn",
        "nontransitive_dice.fzn",
        "no_solve_item.fzn",
        "not_all_equal.fzn",
        "no_three_in_line.fzn",
        "not_in.fzn",
        "npair.fzn",
        "n_puzzle.fzn",
        "n_puzzle_table.fzn",
        "number_generation.fzn",
        "number_of_days.fzn",
        "number_of_regions.fzn",
        "number_puzzle.fzn",
        "number_square.fzn",
        "numeric_keypad.fzn",
        "OandX.fzn",
        "olympic.fzn",
        "onroad.fzn",
        "open_alldifferent.fzn",
        "open_among.fzn",
        "open_atleast.fzn",
        "open_atmost.fzn",
        "open_global_cardinality.fzn",
        "open_global_cardinality_low_up.fzn",
        "optimal_picking_elements_from_each_list.fzn",
        "organize_day.fzn",
        "or_matching2.fzn",
        "or_matching.fzn",
        "or_matching_orig.fzn",
        "or_matching_xxx.fzn",
        "ormat_game.fzn",
        "ormat_game_generate.fzn",
        "ormat_game_mip_problem1.fzn",
        "ormat_game_mip_problem2.fzn",
        "ormat_game_mip_problem3.fzn",
        "ormat_game_mip_problem4.fzn",
        "ormat_game_mip_problem5.fzn",
        "ormat_game_mip_problem6.fzn",
        "ormat_game_problem1.fzn",
        "ormat_game_problem2.fzn",
        "ormat_game_problem3.fzn",
        "ormat_game_problem4.fzn",
        "ormat_game_problem5.fzn",
        "ormat_game_problem6.fzn",
        "or_seating.fzn",
        "orth_link_ori_siz_end.fzn",
        "orth_on_the_ground.fzn",
        "oss.fzn",
        "packing.fzn",
        "pair_divides_the_sum.fzn",
        "pairwise_sum_of_n_numbers.fzn",
        "pandigital_numbers.fzn",
        "parallel_resistors.fzn",
        "partial_latin_square.fzn",
        "partition.fzn",
        "partition_into_subset_of_equal_values2.fzn",
        "partition_into_subset_of_equal_values3.fzn",
        "partition_into_subset_of_equal_values.fzn",
        "partitions.fzn",
        "path_from_to.fzn",
        "patient_no_21.fzn",
        "pchange.fzn",
        "peacableArmyOfQueens.fzn",
        "penguin.fzn",
        "perfect_shuffle.fzn",
        "perfect_square_sequence.fzn",
        "perfsq2.fzn",
        "perfsq.fzn",
        "period.fzn",
        "permutation_number.fzn",
        "pert.fzn",
        "photo.fzn",
        "photo_hkj2_data1.fzn",
    ];
    
    let mut success = 0;
    let mut failed = 0;
    let mut not_found = 0;
    
    println!("\n=== Batch 07: Graph Problems ===\n");
    
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
