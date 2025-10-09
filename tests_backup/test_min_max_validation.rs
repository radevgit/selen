use selen::prelude::*;
use selen::core::error::SolverError;

/// Tests for the new centralized validation system that catches empty min/max constraints
/// and other validation scenarios.

#[test]
fn test_validation_architecture_works() {
    // Test that the validation system is properly integrated
    let mut m = Model::default();
    
    // Create valid constraints
    let x = m.int(1, 10);
    let y = m.int(5, 15);
    let z = m.int(3, 8);
    
    let _min_xyz = m.min(&[x, y, z]);
    let _max_xyz = m.max(&[x, y, z]);
    
    // Add some additional constraints
    m.new(x.le(y));
    m.new(y.le(z.add(int(5))));
    
    // Validation should pass
    let result = m.validate();
    assert!(result.is_ok(), "Valid model should pass validation: {:?}", result);
    
    // Should be solvable
    let solution = m.solve();
    assert!(solution.is_ok(), "Valid model should be solvable: {:?}", solution);
}

#[test]
fn test_validation_catches_domain_issues() {
    // Test that validation catches various types of issues
    let mut m = Model::default();
    
    let x = m.int(1, 10);
    let y = m.int(15, 20);
    
    // Create conflicting constraints that make the model unsolvable
    m.new(x.eq(y)); // x ∈ [1,10] and y ∈ [15,20] can't be equal
    
    // This should either be caught by validation or fail during solving
    let result = m.solve();
    
    // We expect either validation error or no solution
    match result {
        Ok(_) => panic!("Expected model with conflicting constraints to fail"),
        Err(SolverError::NoSolution { .. }) => {
            // This is acceptable - solver detected unsatisfiability
        },
        Err(SolverError::ConflictingConstraints { .. }) => {
            // This is also acceptable - validation caught the issue
        },
        Err(other) => {
            // Other errors are also acceptable for invalid models
            println!("Model failed with: {:?}", other);
        }
    }
}

#[test]
fn test_min_max_validation_with_edge_cases() {
    // Test edge cases for min/max validation
    let mut m = Model::default();
    
    // Single variable min/max (valid edge case)
    let x = m.int(1, 10);
    let min_x = m.min(&[x]).expect("non-empty variable list");
    let max_x = m.max(&[x]).expect("non-empty variable list");
    
    // Should pass validation
    let result = m.validate();
    assert!(result.is_ok(), "Single-variable min/max should be valid: {:?}", result);
    
    // Should solve correctly
    let solution = m.solve();
    assert!(solution.is_ok(), "Single-variable min/max should solve: {:?}", solution);
    
    if let Ok(sol) = solution {
        let x_val = sol.get_int(x);
        let min_val = sol.get_int(min_x);
        let max_val = sol.get_int(max_x);
        
        // Min and max of single variable should equal the variable
        assert_eq!(x_val, min_val, "Min of single variable should equal the variable");
        assert_eq!(x_val, max_val, "Max of single variable should equal the variable");
    }
}

#[test]
fn test_validation_error_messages() {
    // Test that validation provides helpful error messages
    let mut m = Model::default();
    
    // Create some variables with problematic ranges
    let _x = m.int(10, 5); // Invalid range: min > max
    
    // Validation should catch this now
    let result = m.validate();
    
    // We expect validation to fail for invalid variable range
    match result {
        Ok(_) => {
            panic!("Expected validation to catch invalid variable range min > max");
        },
        Err(error) => {
            // Validation caught it - check error message quality
            let error_string = format!("{}", error);
            assert!(!error_string.is_empty(), "Error message should not be empty");
            // Error message should be informative
            assert!(error_string.len() > 10, "Error message should be descriptive: {}", error_string);
            // Should mention the invalid bounds
            assert!(error_string.contains("invalid") || error_string.contains("bounds") || error_string.contains("domain"), 
                    "Error should mention invalid bounds: {}", error_string);
        }
    }
}

#[test]
fn test_validation_with_complex_constraints() {
    // Test validation with more complex constraint combinations
    let mut m = Model::default();
    
    let a = m.int(1, 10);
    let b = m.int(5, 15);
    let c = m.int(8, 12);
    let d = m.int(3, 7);
    
    // Create nested min/max operations
    let min_ab = m.min(&[a, b]).expect("non-empty variable list");
    let max_cd = m.max(&[c, d]).expect("non-empty variable list");
    let final_min = m.min(&[min_ab, max_cd]).expect("non-empty variable list");
    
    // Add constraints
    m.new(final_min.ge(int(4)));
    m.new(final_min.le(int(9)));
    
    // Should pass validation
    let result = m.validate();
    assert!(result.is_ok(), "Complex min/max model should pass validation: {:?}", result);
    
    // Should be solvable
    let solution = m.solve();
    assert!(solution.is_ok(), "Complex min/max model should be solvable: {:?}", solution);
    
    if let Ok(sol) = solution {
        let final_min_val = sol.get_int(final_min);
        assert!(final_min_val >= 4 && final_min_val <= 9, 
                "Final min should satisfy constraints: got {}", final_min_val);
    }
}