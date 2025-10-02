//! Test FlatZinc parser - Batch 09: Knapsack and optimization
//! Tests knapsack variants and optimization problems

use selen::prelude::*;
use std::path::Path;

#[test]
fn test_batch_09_knapsack() {
    let examples_dir = Path::new("zinc/ortools");
    
    if !examples_dir.exists() {
        println!("Skipping test - examples directory not found");
        return;
    }
    
                let test_files = vec![
        "seating_plan.fzn",
        "seating_row1.fzn",
        "seating_row.fzn",
        "seating_table.fzn",
        "secret_santa2.fzn",
        "secret_santa.fzn",
        "seg_fault.fzn",
        "self_referential_quiz.fzn",
        "send_more_money2.fzn",
        "send_more_money_any_base.fzn",
        "send_more_money.fzn",
        "send_more_money_ip.fzn",
        "send_most_money.fzn",
        "sequence_2_3.fzn",
        "seseman2.fzn",
        "seseman.fzn",
        "set_covering2.fzn",
        "set_covering3.fzn",
        "set_covering4b.fzn",
        "set_covering4.fzn",
        "set_covering5.fzn",
        "set_covering6.fzn",
        "set_covering_deployment.fzn",
        "set_covering.fzn",
        "set_covering_skiena.fzn",
        "set_packing.fzn",
        "seven11.fzn",
        "shift.fzn",
        "shopping_basket2.fzn",
        "shopping_basket5.fzn",
        "shopping_basket6.fzn",
        "shopping_basket.fzn",
        "shopping.fzn",
        "shortest_path1.fzn",
        "shortest_path2.fzn",
        "sicherman_dice.fzn",
        "simple_sat.fzn",
        "singHoist2.fzn",
        "ski_assignment_problem.fzn",
        "skyscraper.fzn",
        "sliding_sum_me.fzn",
        "sliding_time_window_from_start.fzn",
        "sliding_time_window.fzn",
        "smooth.fzn",
        "smuggler_knapsack.fzn",
        "smullyan_knights_knaves.fzn",
        "smullyan_knights_knaves_normals_bahava.fzn",
        "smullyan_knights_knaves_normals.fzn",
        "smullyan_lion_and_unicorn.fzn",
        "smullyan_portia.fzn",
        "soccer_puzzle.fzn",
        "social_golfers1.fzn",
        "soft_all_equal_ctr.fzn",
        "soft_same_var.fzn",
        "solitaire_battleship.fzn",
        "sonet_problem.fzn",
        "sort_permutation.fzn",
        "spinning_disks.fzn",
        "sportsScheduling.fzn",
        "spp.fzn",
        "spy_girls.fzn",
        "square_root_of_wonderful.fzn",
        "squeens.fzn",
        "stable_marriage3_random10.fzn",
        "stable_marriage3_random200.fzn",
        "stable_marriage3_random50.fzn",
        "stable_marriage.fzn",
        "stamp_licking.fzn",
        "state_name_puzzle.fzn",
        "stretch_circuit.fzn",
        "stretch_path.fzn",
        "strictly_decreasing.fzn",
        "strimko2.fzn",
        "stuckey_assignment.fzn",
        "stuckey_seesaw.fzn",
        "subsequence.fzn",
        "subsequence_sum.fzn",
        "subset_sum.fzn",
        "successive_number_problem.fzn",
        "sudoku_25x25_250.fzn",
        "sudoku_alldifferent.fzn",
        "sudoku.fzn",
        "sudoku_gcc.fzn",
        "sudoku_ip.fzn",
        "sudoku_pi_2008.fzn",
        "sudoku_pi_2010.fzn",
    ];
    
    let mut success = 0;
    let mut failed = 0;
    let mut not_found = 0;
    
    println!("\n=== Batch 09: Knapsack and Optimization ===\n");
    
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
