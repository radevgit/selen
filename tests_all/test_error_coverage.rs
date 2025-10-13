use selen::prelude::*;
use selen::core::error::SolverError;

#[test]
fn test_no_solution_basic() {
    let err = SolverError::no_solution();
    assert_eq!(err.to_string(), "No solution found");
}

#[test]
fn test_no_solution_with_context() {
    let err = SolverError::no_solution_with_context("domain too restrictive", 5, 10);
    let s = err.to_string();
    assert!(s.contains("No solution found"));
    assert!(s.contains("domain too restrictive"));
    assert!(s.contains("5 variables"));
    assert!(s.contains("10 constraints"));
}

#[test]
fn test_timeout_basic() {
    let err = SolverError::timeout();
    assert_eq!(err.to_string(), "Solving timed out");
}

#[test]
fn test_timeout_with_context() {
    let err = SolverError::timeout_with_context(15.5, "search");
    let s = err.to_string();
    assert!(s.contains("timed out"));
    assert!(s.contains("15.50s"));
    assert!(s.contains("search"));
}

#[test]
fn test_memory_limit_basic() {
    let err = SolverError::memory_limit();
    assert_eq!(err.to_string(), "Memory limit exceeded");
}

#[test]
fn test_memory_limit_with_context() {
    let err = SolverError::memory_limit_with_context(512, 256);
    let s = err.to_string();
    assert!(s.contains("Memory limit exceeded"));
    assert!(s.contains("512MB"));
    assert!(s.contains("256MB"));
}

#[test]
fn test_invalid_constraint_basic() {
    let err = SolverError::invalid_constraint("empty array");
    let s = err.to_string();
    assert!(s.contains("Invalid constraint"));
    assert!(s.contains("empty array"));
}

#[test]
fn test_invalid_constraint_with_context() {
    let err = SolverError::invalid_constraint_with_context(
        "array length mismatch",
        "lin_eq",
        vec!["x".to_string(), "y".to_string()]
    );
    let s = err.to_string();
    assert!(s.contains("Invalid constraint"));
    assert!(s.contains("array length mismatch"));
    assert!(s.contains("lin_eq"));
    assert!(s.contains("x"));
    assert!(s.contains("y"));
}

#[test]
fn test_conflicting_constraints_with_names() {
    let err = SolverError::conflicting_constraints_with_names(vec![
        "x > 10".to_string(),
        "x < 5".to_string()
    ]);
    let s = err.to_string();
    assert!(s.contains("Conflicting constraints"));
    assert!(s.contains("x > 10"));
    assert!(s.contains("x < 5"));
}

#[test]
fn test_conflicting_constraints_with_context() {
    let err = SolverError::conflicting_constraints_with_context(
        vec!["c1".to_string(), "c2".to_string()],
        vec!["x".to_string()],
        "incompatible bounds"
    );
    let s = err.to_string();
    assert!(s.contains("Conflicting constraints"));
    assert!(s.contains("c1"));
    assert!(s.contains("c2"));
    assert!(s.contains("x"));
    assert!(s.contains("incompatible bounds"));
}

#[test]
fn test_invalid_domain_basic() {
    let err = SolverError::invalid_domain("min > max");
    let s = err.to_string();
    assert!(s.contains("Invalid domain"));
    assert!(s.contains("min > max"));
}

#[test]
fn test_invalid_domain_with_context() {
    let err = SolverError::invalid_domain_with_context(
        "empty domain",
        "x",
        "[10..5]"
    );
    let s = err.to_string();
    assert!(s.contains("Invalid domain"));
    assert!(s.contains("empty domain"));
    assert!(s.contains("variable: x"));
    assert!(s.contains("[10..5]"));
}

#[test]
fn test_invalid_variable_basic() {
    let err = SolverError::invalid_variable("not found");
    let s = err.to_string();
    assert!(s.contains("Invalid variable"));
    assert!(s.contains("not found"));
}

#[test]
fn test_invalid_variable_with_context() {
    let err = SolverError::invalid_variable_with_context(
        "id out of range",
        "var_42",
        "0..40"
    );
    let s = err.to_string();
    assert!(s.contains("Invalid variable"));
    assert!(s.contains("id out of range"));
    assert!(s.contains("var_42"));
    assert!(s.contains("0..40"));
}

#[test]
fn test_internal_error_basic() {
    let err = SolverError::internal_error("unexpected state");
    let s = err.to_string();
    assert!(s.contains("Internal error"));
    assert!(s.contains("unexpected state"));
}

#[test]
fn test_internal_error_with_context() {
    let err = SolverError::internal_error_with_context(
        "stack overflow",
        "solver.rs",
        123,
        "depth=1000"
    );
    let s = err.to_string();
    assert!(s.contains("Internal error"));
    assert!(s.contains("stack overflow"));
    assert!(s.contains("solver.rs:123"));
    assert!(s.contains("depth=1000"));
}

#[test]
fn test_error_clone() {
    let err1 = SolverError::no_solution();
    let err2 = err1.clone();
    assert_eq!(err1, err2);
}

#[test]
fn test_error_debug() {
    let err = SolverError::timeout();
    let debug_str = format!("{:?}", err);
    assert!(debug_str.contains("Timeout"));
}

#[test]
fn test_invalid_input_error() {
    // Test InvalidInput variant
    let err = SolverError::InvalidInput {
        message: "negative value not allowed".to_string(),
        function_name: Some("set_limit".to_string()),
        expected: Some("positive integer".to_string()),
    };
    let s = err.to_string();
    assert!(s.contains("Invalid input"));
    assert!(s.contains("negative value not allowed"));
    assert!(s.contains("set_limit"));
    assert!(s.contains("positive integer"));
}

#[test]
fn test_invalid_input_minimal() {
    let err = SolverError::InvalidInput {
        message: "bad format".to_string(),
        function_name: None,
        expected: None,
    };
    let s = err.to_string();
    assert!(s.contains("Invalid input"));
    assert!(s.contains("bad format"));
}

#[test]
fn test_conflicting_constraints_empty_lists() {
    let err = SolverError::ConflictingConstraints {
        constraint_names: Some(vec![]),
        variables: Some(vec![]),
        context: None,
    };
    let s = err.to_string();
    assert!(s.contains("Conflicting constraints detected"));
}

#[test]
fn test_invalid_constraint_empty_variables() {
    let err = SolverError::InvalidConstraint {
        message: "test".to_string(),
        constraint_name: None,
        variables: Some(vec![]),
    };
    let s = err.to_string();
    assert!(s.contains("Invalid constraint"));
}

#[test]
fn test_no_solution_partial_context() {
    // Only context, no counts
    let err = SolverError::NoSolution {
        context: Some("test".to_string()),
        variable_count: None,
        constraint_count: None,
    };
    let s = err.to_string();
    assert!(s.contains("No solution found"));
    assert!(s.contains("(test)"));
}

#[test]
fn test_timeout_only_elapsed() {
    let err = SolverError::Timeout {
        elapsed_seconds: Some(5.0),
        operation: None,
    };
    let s = err.to_string();
    assert!(s.contains("timed out"));
    assert!(s.contains("5.00s"));
}

#[test]
fn test_timeout_only_operation() {
    let err = SolverError::Timeout {
        elapsed_seconds: None,
        operation: Some("propagation".to_string()),
    };
    let s = err.to_string();
    assert!(s.contains("timed out"));
    assert!(s.contains("propagation"));
}

#[test]
fn test_memory_limit_partial() {
    let err1 = SolverError::MemoryLimit {
        usage_mb: Some(100),
        limit_mb: None,
    };
    assert!(err1.to_string().contains("Memory limit exceeded"));
    
    let err2 = SolverError::MemoryLimit {
        usage_mb: None,
        limit_mb: Some(50),
    };
    assert!(err2.to_string().contains("Memory limit exceeded"));
}

#[test]
fn test_conflicting_constraints_only_names() {
    let err = SolverError::ConflictingConstraints {
        constraint_names: Some(vec!["c1".to_string()]),
        variables: None,
        context: None,
    };
    let s = err.to_string();
    assert!(s.contains("Conflicting constraints"));
}

#[test]
fn test_conflicting_constraints_only_variables() {
    let err = SolverError::ConflictingConstraints {
        constraint_names: None,
        variables: Some(vec!["x".to_string(), "y".to_string()]),
        context: None,
    };
    let s = err.to_string();
    assert!(s.contains("Conflicting constraints"));
    assert!(s.contains("x"));
    assert!(s.contains("y"));
}

#[test]
fn test_conflicting_constraints_only_context() {
    let err = SolverError::ConflictingConstraints {
        constraint_names: None,
        variables: None,
        context: Some("bounds issue".to_string()),
    };
    let s = err.to_string();
    assert!(s.contains("Conflicting constraints"));
    assert!(s.contains("bounds issue"));
}
