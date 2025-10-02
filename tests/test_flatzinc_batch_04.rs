//! Test FlatZinc parser - Batch 04: N-Queens variants
//! Tests various N-Queens problems

use selen::prelude::*;
use std::path::Path;

#[test]
fn test_batch_04_queens() {
    let examples_dir = Path::new("zinc/ortools");
    
    if !examples_dir.exists() {
        println!("Skipping test - examples directory not found");
        return;
    }
    
                let test_files = vec![
        "enigma_counting_pennies.fzn",
        "enigma_eighteen.fzn",
        "enigma_eight_times2.fzn",
        "enigma_eight_times.fzn",
        "enigma_five_fives.fzn",
        "enigma.fzn",
        "enigma_planets.fzn",
        "enigma_portuguese_squares.fzn",
        "eq10.fzn",
        "eq20.fzn",
        "equal_sized_groups.fzn",
        "equivalent.fzn",
        "ett_ett_ett_ett_ett__fem.fzn",
        "euler_18.fzn",
        "euler_1.fzn",
        "euler_2.fzn",
        "euler_30.fzn",
        "euler_39.fzn",
        "euler_52.fzn",
        "euler_6.fzn",
        "euler_9.fzn",
        "evens2.fzn",
        "evens.fzn",
        "evision.fzn",
        "exact_cover_dlx.fzn",
        "exact_cover_dlx_matrix.fzn",
        "exodus.fzn",
        "facility_location_problem.fzn",
        "factorial.fzn",
        "factory_planning_instance.fzn",
        "fairies.fzn",
        "fair_split_into_3_groups.fzn",
        "family.fzn",
        "family_riddle.fzn",
        "fancy.fzn",
        "farm_puzzle0.fzn",
        "farm_puzzle.fzn",
        "fib_test2.fzn",
        "fill_a_pix.fzn",
        "filling_table_with_ticks.fzn",
        "fill_in_the_squares.fzn",
        "five_brigades.fzn",
        "five_floors.fzn",
        "five.fzn",
        "fixed_charge.fzn",
        "fix_points.fzn",
        "fizz_buzz.fzn",
        "football.fzn",
        "four_islands.fzn",
        "four_power.fzn",
        "fractions.fzn",
        "franklin_8x8_magic_square.fzn",
        "freight_transfer.fzn",
        "full_adder.fzn",
        "furniture_moving.fzn",
        "futoshiki.fzn",
        "gap.fzn",
        "gardner_prime_puzzle.fzn",
        "gardner_sum_square.fzn",
        "generalized_knapsack_problem.fzn",
        "general_store.fzn",
        "giapetto.fzn",
        "global_cardinality_no_loop.fzn",
        "global_cardinality_table.fzn",
        "global_cardinality_with_costs.fzn",
        "global_contiguity.fzn",
        "golomb.fzn",
        "graceful_labeling.fzn",
        "graph_degree_sequence.fzn",
        "gray_code.fzn",
        "greatest_combination.fzn",
        "grid_puzzle.fzn",
        "grime_puzzle.fzn",
        "grocery2.fzn",
        "grocery.fzn",
        "guards_and_apples2.fzn",
        "guards_and_apples.fzn",
        "gunport_problem1.fzn",
        "gunport_problem2.fzn",
        "hamming_distance.fzn",
        "hanging_weights.fzn",
        "hardy_1729.fzn",
        "heterosquare.fzn",
        "hidato_exists.fzn",
        "hidato.fzn",
        "hidato_table2.fzn",
    ];
    
    let mut success = 0;
    let mut failed = 0;
    let mut not_found = 0;
    
    println!("\n=== Batch 04: N-Queens Variants ===\n");
    
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
