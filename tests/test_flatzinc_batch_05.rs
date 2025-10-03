//! Test FlatZinc parser - Batch 05: Magic sequences and numbers
//! Tests magic sequence and magic square problems

use selen::prelude::*;
use std::path::Path;

#[test]
#[ignore]
fn test_batch_05_magic() {
    let examples_dir = Path::new("zinc/ortools");
    
    if !examples_dir.exists() {
        println!("Skipping test - examples directory not found");
        return;
    }
    
                let test_files = vec![
        "hidato_table.fzn",
        "high_iq_problem.fzn",
        "hitchcock_transporation_problem.fzn",
        "hitting_set.fzn",
        "home_improvement.fzn",
        "honey_division.fzn",
        "houses.fzn",
        "how_old_am_i.fzn",
        "huey_dewey_louie.fzn",
        "hundred_doors_optimized_array.fzn",
        "hundred_fowls.fzn",
        "ice_cream.fzn",
        "imply.fzn",
        "increasing_except_0.fzn",
        "indexed_sum.fzn",
        "inflexions.fzn",
        "in_interval.fzn",
        "in_relation.fzn",
        "in_set.fzn",
        "integer_programming1.fzn",
        "inter_distance.fzn",
        "int_value_precede.fzn",
        "inverse_within_range.fzn",
        "investment_problem.fzn",
        "investment_problem_mip.fzn",
        "isbn.fzn",
        "itemset_mining.fzn",
        "ith_pos_different_from_0.fzn",
        "jive_turkeys.fzn",
        "jobshop2x2.fzn",
        "jobs_puzzle.fzn",
        "joshua.fzn",
        "jssp.fzn",
        "just_forgotten.fzn",
        "K4P2GracefulGraph2.fzn",
        "K4P2GracefulGraph.fzn",
        "kakuro2.fzn",
        "kakuro.fzn",
        "k_alldifferent.fzn",
        "kaprekars_constant2.fzn",
        "kaprekars_constant_3.fzn",
        "kaprekars_constant_8.fzn",
        "kaprekars_constant.fzn",
        "kenken2.fzn",
        "killer_sudoku2.fzn",
        "killer_sudoku.fzn",
        "kiselman_semigroup_problem.fzn",
        "knapsack1.fzn",
        "knapsack2.fzn",
        "knapsack_investments.fzn",
        "knapsack_rosetta_code_01.fzn",
        "knapsack_rosetta_code_bounded.fzn",
        "knapsack_rosetta_code_unbounded_int.fzn",
        "knight_path.fzn",
        "kntdom.fzn",
        "kqueens.fzn",
        "k_same.fzn",
        "k_same_modulo.fzn",
        "labeled_dice.fzn",
        "lager.fzn",
        "lams_problem.fzn",
        "langford2.fzn",
        "langford.fzn",
        "latin_square_card_puzzle.fzn",
        "latin_square.fzn",
        "latin_squares_fd.fzn",
        "lccoin.fzn",
        "least_diff.fzn",
        "lecture_series.fzn",
        "lectures.fzn",
        "letter_square.fzn",
        "lex2_me.fzn",
        "lex_alldifferent.fzn",
        "lex_between.fzn",
        "lex_chain_less.fzn",
        "lex_different.fzn",
        "lex_greater_me.fzn",
        "lichtenstein_coloring.fzn",
        "life.fzn",
        "lightmeal2.fzn",
        "lightmeal.fzn",
        "lights.fzn",
        "limerick_primes2.fzn",
        "limerick_primes.fzn",
        "locker.fzn",
        "logical_design.fzn",
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
