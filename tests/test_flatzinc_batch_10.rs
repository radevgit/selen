//! Test FlatZinc parser - Batch 10: Miscellaneous puzzles
//! Tests various other puzzles and problems

use selen::prelude::*;
use std::path::Path;

#[test]
fn test_batch_10_misc() {
    let examples_dir = Path::new("src/zinc/flatzinc");
    
    if !examples_dir.exists() {
        println!("Skipping test - examples directory not found");
        return;
    }
    
                let test_files = vec![
        "sudoku_pi_2011.fzn",
        "sudoku_pi.fzn",
        "sum_ctr.fzn",
        "sum_free.fzn",
        "sum_of_weights_of_distinct_values.fzn",
        "sum_to_100.fzn",
        "survivor.fzn",
        "survo_puzzle.fzn",
        "symmetric_alldifferent.fzn",
        "symmetry_breaking.fzn",
        "table_of_numbers.fzn",
        "talent.fzn",
        "talisman_square.fzn",
        "tank.fzn",
        "tea_mixing.fzn",
        "template_design.fzn",
        "temporal_reasoning.fzn",
        "tenpenki_1.fzn",
        "tenpenki_2.fzn",
        "tenpenki_3.fzn",
        "tenpenki_4.fzn",
        "tenpenki_5.fzn",
        "tenpenki_6.fzn",
        "test.fzn",
        "the_bomb.fzn",
        "the_family_puzzle.fzn",
        "three_digit.fzn",
        "tickTackToe.fzn",
        "timeslots_for_songs.fzn",
        "timetabling.fzn",
        "timpkin.fzn",
        "tobacco.fzn",
        "tomography.fzn",
        "tomography_n_colors.fzn",
        "torn_number.fzn",
        "touching_numbers.fzn",
        "traffic_lights.fzn",
        "traffic_lights_table.fzn",
        "transportation2.fzn",
        "transportation.fzn",
        "transpose.fzn",
        "transshipment.fzn",
        "trial12.fzn",
        "trial1.fzn",
        "trial2.fzn",
        "trial3.fzn",
        "trial4.fzn",
        "trial5.fzn",
        "trial6.fzn",
        "tripuzzle1.fzn",
        "tripuzzle2.fzn",
        "trucking.fzn",
        "tsp_circuit.fzn",
        "tsp.fzn",
        "tunapalooza.fzn",
        "twelve.fzn",
        "twin_letters.fzn",
        "two_cube_calendar.fzn",
        "two_dimensional_channels.fzn",
        "uzbekian_puzzle.fzn",
        "vingt_cinq_cinq_trente.fzn",
        "warehouses.fzn",
        "war_or_peace.fzn",
        "water_buckets1.fzn",
        "wedding_optimal_chart.fzn",
        "weighted_sum.fzn",
        "were2.fzn",
        "were4.fzn",
        "who_killed_agatha.fzn",
        "wolf_goat_cabbage.fzn",
        "wolf_goat_cabbage_lp.fzn",
        "word_golf.fzn",
        "word_square.fzn",
        "work_shift_problem.fzn",
        "wwr.fzn",
        "xkcd_among_diff_0.fzn",
        "xkcd.fzn",
        "young_tableaux.fzn",
        "zebra.fzn",
        "zebra_inverse.fzn",
        "zebra_ip.fzn",
    ];
    
    let mut success = 0;
    let mut failed = 0;
    let mut not_found = 0;
    
    println!("\n=== Batch 10: Miscellaneous Puzzles ===\n");
    
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
