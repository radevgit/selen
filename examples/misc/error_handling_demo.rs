use cspsolver::prelude::*;

fn main() {
    println!("� CSP Solver Error Handling Demo");
    println!("==================================");
    
    demo_basic_errors();
    demo_error_handling();
    demo_rich_error_context();
}

fn demo_basic_errors() {
    println!("\n📋 Basic Error Types:");
    
    // Create different error types
    let errors = vec![
        SolverError::no_solution(),
        SolverError::timeout(),
        SolverError::memory_limit(),
        SolverError::invalid_constraint("x > x"),
        SolverError::ConflictingConstraints {
            constraint_names: None,
            variables: None,
            context: None,
        },
        SolverError::invalid_domain("min > max"),
        SolverError::invalid_variable("var_999"),
        SolverError::internal_error("unexpected state"),
    ];
    
    for error in errors {
        println!("  - {}", error);
    }
}

fn demo_rich_error_context() {
    println!("\n🎯 Rich Error Context Examples:");
    
    // Demonstrate errors with detailed context
    let rich_errors = vec![
        SolverError::no_solution_with_context(
            "constraints too restrictive", 
            15, 
            8
        ),
        SolverError::timeout_with_context(
            30.5, 
            "optimization phase"
        ),
        SolverError::memory_limit_with_context(
            2048, 
            1024
        ),
        SolverError::invalid_constraint_with_context(
            "reflexive comparison not allowed",
            "reflexive_constraint",
            vec!["x".to_string()]
        ),
        SolverError::conflicting_constraints_with_context(
            vec!["x > 10".to_string(), "x < 5".to_string()],
            vec!["x".to_string()],
            "bounds are incompatible"
        ),
        SolverError::invalid_domain_with_context(
            "empty domain after constraint propagation",
            "variable_x",
            "domain: {} (empty)"
        ),
        SolverError::invalid_variable_with_context(
            "variable index out of bounds",
            "var_999",
            "valid range: 0..50"
        ),
        SolverError::internal_error_with_context(
            "unexpected state in propagator",
            file!(),
            line!(),
            "propagator_id=5, state=FAILED"
        ),
    ];
    
    for (i, error) in rich_errors.iter().enumerate() {
        println!("  {}. {}", i + 1, error);
    }
}fn demo_error_handling() {
    println!("\n🔧 Error Handling Example:");
    
    // Simulate different failure scenarios
    let results: Vec<SolverResult<i32>> = vec![
        Ok(42),
        Err(SolverError::no_solution()),
        Err(SolverError::timeout()),
        Err(SolverError::invalid_constraint("malformed")),
    ];
    
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(value) => println!("  Result {}: Success with value {}", i + 1, value),
            Err(SolverError::NoSolution { .. }) => {
                println!("  Result {}: No solution - try relaxing constraints", i + 1);
            }
            Err(SolverError::Timeout { .. }) => {
                println!("  Result {}: Timeout - try increasing time limit", i + 1);
            }
            Err(SolverError::InvalidConstraint { message, .. }) => {
                println!("  Result {}: Invalid constraint '{}' - check syntax", i + 1, message);
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
        Err(SolverError::no_solution())
    }
}