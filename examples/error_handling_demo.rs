use cspsolver::prelude::*;

fn main() {
    println!("🚫 Error Handling Demo");
    println!("======================");

    // Demonstrate basic error usage
    demo_basic_errors();
    
    // Show how errors can be matched and handled
    demo_error_handling();
}

fn demo_basic_errors() {
    println!("\n📋 Basic Error Types:");
    
    // Create different error types
    let errors = vec![
        SolverError::NoSolution,
        SolverError::Timeout,
        SolverError::MemoryLimit,
        SolverError::InvalidConstraint("x > x".to_string()),
        SolverError::ConflictingConstraints,
        SolverError::InvalidDomain("min > max".to_string()),
        SolverError::InvalidVariable("var_999".to_string()),
        SolverError::InternalError("unexpected state".to_string()),
    ];
    
    for error in errors {
        println!("  - {}", error);
    }
}

fn demo_error_handling() {
    println!("\n🔧 Error Handling Example:");
    
    // Simulate different failure scenarios
    let results: Vec<SolverResult<i32>> = vec![
        Ok(42),
        Err(SolverError::NoSolution),
        Err(SolverError::Timeout),
        Err(SolverError::InvalidConstraint("malformed".to_string())),
    ];
    
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(value) => println!("  Result {}: Success with value {}", i + 1, value),
            Err(SolverError::NoSolution) => {
                println!("  Result {}: No solution - try relaxing constraints", i + 1);
            }
            Err(SolverError::Timeout) => {
                println!("  Result {}: Timeout - try increasing time limit", i + 1);
            }
            Err(SolverError::InvalidConstraint(msg)) => {
                println!("  Result {}: Invalid constraint '{}' - check syntax", i + 1, msg);
            }
            Err(other) => {
                println!("  Result {}: Other error: {}", i + 1, other);
            }
        }
    }
}

// Example helper function that returns SolverResult
fn _example_solver_function(valid: bool) -> SolverResult<String> {
    if valid {
        Ok("Solution found!".to_string())
    } else {
        Err(SolverError::NoSolution)
    }
}