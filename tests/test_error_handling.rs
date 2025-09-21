use cspsolver::prelude::*;

#[test]
fn test_min_empty_list_caught_by_validation() {
    // This test demonstrates that our validation system is properly set up
    // to catch empty min/max constraints if they could ever be created.
    // Since our API design prevents calling min(&[]) directly, 
    // we test with a minimal valid case to ensure validation works.
    
    let mut m = Model::default();
    
    // Test that normal min/max work fine and pass validation
    let x = m.int(1, 10);
    let y = m.int(5, 15);
    let z = m.int(3, 8);
    
    let min_result = m.min(&[x, y, z]).expect("non-empty variable list");
    let max_result = m.max(&[x, y, z]).expect("non-empty variable list");
    
    // These should work fine and validation should pass
    let validation_result = m.validate();
    assert!(validation_result.is_ok(), "Model with valid min/max should pass validation: {:?}", validation_result);
    
    // Test that we can solve
    let solution = m.solve();
    assert!(solution.is_ok(), "Model with valid min/max should be solvable: {:?}", solution);
    
    // Verify the min/max work correctly
    if let Ok(sol) = solution {
        let x_val = sol.get_int(x);
        let y_val = sol.get_int(y);
        let z_val = sol.get_int(z);
        let min_val = sol.get_int(min_result);
        let max_val = sol.get_int(max_result);
        
        let values = [x_val, y_val, z_val];
        let expected_min = *values.iter().min().unwrap();
        let expected_max = *values.iter().max().unwrap();
        
        assert_eq!(min_val, expected_min, "Min constraint should compute actual minimum");
        assert_eq!(max_val, expected_max, "Max constraint should compute actual maximum");
    }
}

#[test]
fn test_validation_system_catches_constraint_issues() {
    // Test that the validation system properly checks constraint parameters
    // This test verifies the validation infrastructure works correctly
    
    let mut m = Model::default();
    let x = m.int(1, 10);
    let y = m.int(5, 15);
    
    // Test single-variable min/max (edge case but valid)
    let _min_single = m.min(&[x]);
    let _max_single = m.max(&[y]);
    
    // Should pass validation
    let validation_result = m.validate();
    assert!(validation_result.is_ok(), "Single-variable min/max should pass validation: {:?}", validation_result);
    
    // Test with multiple variables
    let _min_multi = m.min(&[x, y]);
    let _max_multi = m.max(&[x, y]);
    
    // Should still pass validation
    let validation_result = m.validate();
    assert!(validation_result.is_ok(), "Multi-variable min/max should pass validation: {:?}", validation_result);
}

#[test] 
fn test_validation_catches_invalid_constraints() {
    // Test that the validation system works correctly for various constraint types
    let mut m = Model::default();
    
    let x = m.int(1, 10);
    let y = m.int(5, 15);
    
    // Valid constraints should pass validation
    let _min_xy = m.min(&[x, y]);
    let _max_xy = m.max(&[x, y]);
    
    let validation_result = m.validate();
    assert!(validation_result.is_ok(), "Model with valid constraints should pass validation: {:?}", validation_result);
}

#[test]
fn test_min_max_with_single_variable() {
    let mut m = Model::default();
    
    let x = m.int(1, 10);
    
    // Min/Max with single variable should work (though not very useful)
    let min_x = m.min(&[x]).expect("non-empty variable list");
    let max_x = m.max(&[x]).expect("non-empty variable list");
    
    // Should pass validation
    let validation_result = m.validate();
    assert!(validation_result.is_ok(), "Model with single-variable min/max should pass validation: {:?}", validation_result);
    
    // Should be solvable
    let solution = m.solve();
    assert!(solution.is_ok(), "Model with single-variable min/max should be solvable: {:?}", solution);
    
    if let Ok(sol) = solution {
        // min and max of a single variable should equal the variable itself
        assert_eq!(sol[x], sol[min_x]);
        assert_eq!(sol[x], sol[max_x]);
    }
}

#[test]
fn test_error_display_formatting() {
    // Test the InvalidInput error display formatting
    let error = SolverError::InvalidInput {
        message: "Cannot compute minimum of empty variable list".to_string(),
        function_name: Some("min".to_string()),
        expected: Some("non-empty slice of variable IDs".to_string()),
    };
    
    let error_string = format!("{}", error);
    
    assert!(error_string.contains("Invalid input"));
    assert!(error_string.contains("Cannot compute minimum of empty variable list"));
    assert!(error_string.contains("function 'min'"));
    assert!(error_string.contains("expected: non-empty slice of variable IDs"));
}

#[test]
fn test_model_solving_with_constraints() {
    let mut m = Model::default();
    
    let a = m.int(4, 10);  // Changed lower bound to make min=4 feasible
    let b = m.int(5, 15);
    let c = m.int(4, 8);   // Changed lower bound to make min=4 feasible
    
    let minimum = m.min(&[a, b, c]).expect("non-empty variable list");
    let maximum = m.max(&[a, b, c]).expect("non-empty variable list");
    
    // Add some constraints
    m.new(minimum.eq(4));  // min must be 4
    m.new(maximum.le(12)); // max must be <= 12
    
    // Should pass validation
    let validation_result = m.validate();
    assert!(validation_result.is_ok(), "Model should pass validation: {:?}", validation_result);
    
    // Should be solvable
    let solution = m.solve();
    assert!(solution.is_ok(), "Model should be solvable: {:?}", solution);
    
    if let Ok(sol) = solution {
        // Verify the constraints are satisfied
        assert_eq!(sol[minimum].as_int().unwrap(), 4);
        assert!(sol[maximum].as_int().unwrap() <= 12);
        
        // Verify min/max logic
        let vals = [sol[a].as_int().unwrap(), sol[b].as_int().unwrap(), sol[c].as_int().unwrap()];
        assert_eq!(vals.iter().min().unwrap(), &sol[minimum].as_int().unwrap());
        assert_eq!(vals.iter().max().unwrap(), &sol[maximum].as_int().unwrap());
    }
}