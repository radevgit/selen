// Benchmark modules focused on solver limits and engineering scales
pub mod precision_validation;
pub mod solver_limits;
pub mod medium_scale_proposals;

use precision_validation::run_precision_validation_suite;
use solver_limits::run_solver_limit_investigation;
use medium_scale_proposals::run_medium_scale_optimization_proposals;

pub fn run_all_benchmarks() {
    println!("CSP Solver Performance & Limits Investigation");
    println!("============================================");
    println!("Focus: Solver limits with engineering-scale numerical values");
    println!();
    
    // Basic precision validation
    run_precision_validation_suite();
    
    let separator = format!("\n{}\n", "=".repeat(60));
    println!("{}", separator);
    
    // Solver limits investigation
    run_solver_limit_investigation();
    println!("{}", separator);
    
    // Medium-scale optimization proposals
    run_medium_scale_optimization_proposals();
    println!("{}", separator);
    
    println!("🎯 SOLVER LIMITS INVESTIGATION COMPLETE");
    println!("Ready to understand solver boundaries and optimize for engineering applications!");
}
