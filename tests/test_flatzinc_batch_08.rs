//! Test FlatZinc parser - Batch 08: Assignment and matching
//! Tests assignment, stable marriage, and matching problems

use selen::prelude::*;
use std::path::Path;

#[test]
#[ignore]
fn test_batch_08_assignment() {
    let examples_dir = Path::new("zinc/ortools");
    
    if !examples_dir.exists() {
        println!("Skipping test - examples directory not found");
        return;
    }
    
                let test_files = vec![
        "photo_hkj2_data2.fzn",
        "photo_hkj.fzn",
        "picking_teams.fzn",
        "pigeon_hole2.fzn",
        "pigeon_hole.fzn",
        "pilgrim.fzn",
        "place_number.fzn",
        "pool_ball_triangles.fzn",
        "popsicle_stand.fzn",
        "post_office_problem2.fzn",
        "post_office_problem.fzn",
        "power.fzn",
        "prime.fzn",
        "prime_looking.fzn",
        "product_configuration.fzn",
        "product_ctr.fzn",
        "product_fd.fzn",
        "product_lp.fzn",
        "product_test.fzn",
        "public_school_problem.fzn",
        "puzzle1.fzn",
        "pyramid_of_numbers.fzn",
        "pythagoras.fzn",
        "quasiGroup3Idempotent.fzn",
        "quasiGroup3NonIdempotent.fzn",
        "quasiGroup4Idempotent.fzn",
        "quasiGroup4NonIdempotent.fzn",
        "quasiGroup5Idempotent.fzn",
        "quasiGroup5NonIdempotent.fzn",
        "quasiGroup6.fzn",
        "quasiGroup7.fzn",
        "quasigroup_completion.fzn",
        "quasigroup_completion_gcc.fzn",
        "quasigroup_completion_gomes_demo1.fzn",
        "quasigroup_completion_gomes_demo2.fzn",
        "quasigroup_completion_gomes_demo3.fzn",
        "quasigroup_completion_gomes_demo4.fzn",
        "quasigroup_completion_gomes_demo5.fzn",
        "quasigroup_completion_gomes_shmoys_p3.fzn",
        "quasigroup_completion_gomes_shmoys_p7.fzn",
        "quasigroup_completion_martin_lynce.fzn",
        "quasigroup_qg5.fzn",
        "queen_cp2.fzn",
        "queen_ip.fzn",
        "queens3.fzn",
        "queens4.fzn",
        "queens_ip.fzn",
        "queens_viz.fzn",
        "radiation.fzn",
        "range_ctr.fzn",
        "raven_puzzle.fzn",
        "rectangle_from_line_segments.fzn",
        "regular_test.fzn",
        "rehearsal.fzn",
        "relative_sizes.fzn",
        "relief_mission.fzn",
        "remainder_puzzle2.fzn",
        "remainder_puzzle.fzn",
        "remarkable_sequence.fzn",
        "reveal_the_mapping.fzn",
        "rock_star_dressing_problem.fzn",
        "rogo3.fzn",
        "rogo.fzn",
        "rook_path.fzn",
        "rookwise_chain.fzn",
        "roots_test.fzn",
        "rostering.fzn",
        "rot13.fzn",
        "rotation.fzn",
        "runs.fzn",
        "safe_cracking.fzn",
        "same_and_global_cardinality.fzn",
        "same_and_global_cardinality_low_up.fzn",
        "same.fzn",
        "same_interval.fzn",
        "same_modulo.fzn",
        "sangraal.fzn",
        "sat.fzn",
        "satisfy.fzn",
        "scene_allocation.fzn",
        "schedule1.fzn",
        "schedule2.fzn",
        "scheduling_bratko2.fzn",
        "scheduling_bratko.fzn",
        "scheduling_chip.fzn",
        "scheduling_speakers.fzn",
    ];
    
    let mut success = 0;
    let mut failed = 0;
    let mut not_found = 0;
    
    println!("\n=== Batch 08: Assignment and Matching ===\n");
    
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
