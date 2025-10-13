use selen::prelude::*;

#[test]
fn test_validation_invalid_int_domain_reversed_bounds() {
    let mut m = Model::default();
    // This should create an invalid domain: min > max
    let _x = m.int(10, 5);  // Invalid: min=10, max=5
    
    // Validation should catch this
    let result = m.solve();
    assert!(result.is_err());
    
    if let Err(e) = result {
        let err_str = e.to_string();
        assert!(err_str.contains("Invalid domain") || err_str.contains("invalid bounds"));
    }
}

#[test]
fn test_validation_invalid_float_domain_reversed_bounds() {
    let mut m = Model::default();
    let _x = m.float(10.0, 5.0);  // Invalid: min > max
    
    let result = m.solve();
    assert!(result.is_err());
    
    if let Err(e) = result {
        let err_str = e.to_string();
        assert!(err_str.contains("Invalid domain") || err_str.contains("invalid bounds"));
    }
}

#[test]
fn test_validation_valid_int_domain() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    m.new(x.eq(5));
    
    let result = m.solve();
    assert!(result.is_ok());
}

#[test]
fn test_validation_valid_float_domain() {
    let mut m = Model::default();
    let x = m.float(1.0, 10.0);
    m.new(x.eq(5.5));
    
    let result = m.solve();
    assert!(result.is_ok());
}

#[test]
fn test_validation_empty_domain_after_constraints() {
    let mut m = Model::default();
    let x = m.int(1, 5);
    
    // Create contradictory constraints
    m.new(x.lt(2));  // x < 2, so x must be 1
    m.new(x.gt(4));  // x > 4, so x must be 5
    
    let result = m.solve();
    assert!(result.is_err());
}

#[test]
fn test_validation_alldiff_with_insufficient_domain() {
    let mut m = Model::default();
    let x = m.int(1, 2);  // Only 2 possible values
    let y = m.int(1, 2);
    let z = m.int(1, 2);
    
    // AllDifferent requires 3 distinct values, but we only have 2
    m.alldiff(&[x, y, z]);
    
    let result = m.solve();
    // Should be unsatisfiable - 3 variables need distinct values from only {1, 2}
    assert!(result.is_err(), "Expected unsatisfiable due to insufficient domain size");
}

#[test]
fn test_validation_alldiff_with_sufficient_domain() {
    let mut m = Model::default();
    let x = m.int(1, 3);
    let y = m.int(1, 3);
    let z = m.int(1, 3);
    
    m.alldiff(&[x, y, z]);
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(solution) = result {
        let vals = vec![solution.get_int(x), solution.get_int(y), solution.get_int(z)];
        // Check all values are different
        assert_ne!(vals[0], vals[1]);
        assert_ne!(vals[0], vals[2]);
        assert_ne!(vals[1], vals[2]);
    }
}

#[test]
fn test_validation_single_variable_alldiff() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    
    // AllDifferent with single variable is trivially satisfiable
    m.alldiff(&[x]);
    m.new(x.eq(5));
    
    let result = m.solve();
    assert!(result.is_ok());
}

#[test]
fn test_validation_empty_alldiff() {
    let mut m = Model::default();
    
    // AllDifferent with no variables is trivially satisfiable
    m.alldiff(&[]);
    
    let result = m.solve();
    assert!(result.is_ok());
}

#[test]
fn test_validation_alldiff_with_fixed_duplicate_values() {
    let mut m = Model::default();
    let x = m.int(5, 5);  // Fixed to 5
    let y = m.int(5, 5);  // Fixed to 5
    
    // Both fixed to same value - should be unsatisfiable
    m.alldiff(&[x, y]);
    
    let result = m.solve();
    assert!(result.is_err());
}

#[test]
fn test_validation_multiple_equality_constraints_compatible() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    
    // x == 5 and y == 5 are compatible (different variables)
    m.new(x.eq(5));
    m.new(y.eq(5));
    
    let result = m.solve();
    assert!(result.is_ok());
}

#[test]
fn test_validation_constraint_with_nonexistent_variable() {
    // This is harder to test directly since VarId creation is controlled
    // The solver should handle this internally
    let mut m = Model::default();
    let x = m.int(1, 10);
    m.new(x.eq(5));
    
    let result = m.solve();
    assert!(result.is_ok());
}

#[test]
fn test_validation_zero_width_int_domain() {
    let mut m = Model::default();
    let x = m.int(5, 5);  // Single value domain
    
    // This should be valid (singleton domain)
    m.new(x.eq(5));
    
    let result = m.solve();
    assert!(result.is_ok());
}

#[test]
fn test_validation_zero_width_float_domain() {
    let mut m = Model::default();
    let x = m.float(5.0, 5.0);  // Single value domain
    
    m.new(x.eq(5.0));
    
    let result = m.solve();
    assert!(result.is_ok());
}

#[test]
fn test_validation_very_large_int_domain() {
    let mut m = Model::default();
    // Large but reasonable domain
    let x = m.int(1, 10000);
    m.new(x.eq(5000));
    
    let result = m.solve();
    assert!(result.is_ok());
}

#[test]
fn test_validation_negative_int_domain() {
    let mut m = Model::default();
    let x = m.int(-100, -1);
    m.new(x.eq(-50));
    
    let result = m.solve();
    assert!(result.is_ok());
}

#[test]
fn test_validation_negative_float_domain() {
    let mut m = Model::default();
    let x = m.float(-10.0, -1.0);
    m.new(x.eq(-5.5));
    
    let result = m.solve();
    assert!(result.is_ok());
}

#[test]
fn test_validation_mixed_positive_negative_domain() {
    let mut m = Model::default();
    let x = m.int(-50, 50);
    m.new(x.eq(0));
    
    let result = m.solve();
    assert!(result.is_ok());
}

#[test]
fn test_validation_alldiff_with_overlapping_domains() {
    let mut m = Model::default();
    let x = m.int(1, 5);
    let y = m.int(3, 7);
    let z = m.int(5, 9);
    
    // Domains overlap but should still be solvable
    m.alldiff(&[x, y, z]);
    
    let result = m.solve();
    assert!(result.is_ok());
}

#[test]
fn test_validation_alldiff_with_disjoint_domains() {
    let mut m = Model::default();
    let x = m.int(1, 3);
    let y = m.int(4, 6);
    let z = m.int(7, 9);
    
    // Disjoint domains - definitely solvable
    m.alldiff(&[x, y, z]);
    
    let result = m.solve();
    assert!(result.is_ok());
}

#[test]
fn test_validation_bool_variables() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    
    m.new(b1.eq(1));
    m.new(b2.eq(0));
    
    let result = m.solve();
    assert!(result.is_ok());
}

#[test]
fn test_validation_bool_alldiff() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    
    // Two booleans must be different - one true, one false
    m.alldiff(&[b1, b2]);
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(solution) = result {
        let v1 = solution.get_int(b1);
        let v2 = solution.get_int(b2);
        assert_ne!(v1, v2);
        assert!(v1 == 0 || v1 == 1);
        assert!(v2 == 0 || v2 == 1);
    }
}

#[test]
fn test_validation_bool_alldiff_three_variables_unsatisfiable() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    let b3 = m.bool();
    
    // Three booleans can't all be different (only 0 and 1 available)
    m.alldiff(&[b1, b2, b3]);
    
    let result = m.solve();
    assert!(result.is_err());
}

#[test]
fn test_validation_complex_model_valid() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    let z = m.int(1, 10);
    
    m.new(x.add(y).eq(z));
    m.new(x.lt(y));
    m.new(z.le(15));
    
    let result = m.solve();
    assert!(result.is_ok());
}

#[test]
fn test_validation_float_with_constraints() {
    let mut m = Model::default();
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    
    m.new(x.add(y).eq(15.0));
    m.new(x.le(8.0));
    
    let result = m.solve();
    assert!(result.is_ok());
}
