//! Debug variable bounds handling

use selen::lpsolver::{LpProblem, solve};

fn main() {
    // Very simple test: maximize x subject to x <= 10, 5 <= x <= 8
    // Answer should be x = 8
    
    let problem = LpProblem::new(
        1,                    // 1 variable
        1,                    // 1 constraint
        vec![1.0],           // maximize x
        vec![vec![1.0]],     // x <= 10
        vec![10.0],
        vec![5.0],           // lower: x >= 5
        vec![8.0],           // upper: x <= 8
    );
    
    println!("Problem:");
    println!("  Maximize: x");
    println!("  Subject to: x <= 10");
    println!("  Bounds: 5 <= x <= 8");
    println!();
    
    match problem.validate() {
        Ok(_) => println!("Problem validation: OK"),
        Err(e) => {
            println!("Problem validation error: {}", e);
            return;
        }
    }
    
    match solve(&problem) {
        Ok(solution) => {
            println!("\nSolution found:");
            println!("  Status: {:?}", solution.status);
            println!("  x = {:?}", solution.x);
            println!("  Objective = {}", solution.objective);
            
            // Check if x is in bounds
            if solution.x[0] >= 5.0 && solution.x[0] <= 8.0 {
                println!("\n✓ Solution respects bounds!");
            } else {
                println!("\n✗ Solution violates bounds!");
            }
        }
        Err(e) => {
            println!("\nError: {}", e);
        }
    }
}
