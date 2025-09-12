use cspsolver::prelude::*;

#[test]
fn test_constraint_metadata_collection() {
    let mut model = Model::default();
    
    // Create variables
    let x = model.float(0.0, 10.0);
    let y = model.float(0.0, 10.0);
    let z = model.float(0.0, 20.0);
    
    // Create various constraints to test metadata collection
    model.lt(x, y);           // x < y
    model.le(y, z); // y <= z
    let sum_result = model.add(x, y); // x + y = sum_result
    model.equals(sum_result, z);     // sum_result == z
    model.ne(x, y);          // x != y
    model.gt(z, x);        // z > x
    model.all_different(vec![x, y, z]); // all different
    
    // Test that constraints were registered
    let constraint_count = model.get_props().constraint_count();
    let registry = model.get_props().get_constraint_registry();
    let metadata_count = registry.constraint_count();
    
    // Print for debugging
    println!("Total constraints registered: {}", constraint_count);
    println!("Metadata entries: {}", metadata_count);
    
    // Test constraint analysis for variable x
    let analysis = registry.analyze_variable_constraints(x);
    println!("Analysis for variable x:");
    println!("  Upper bounds: {:?}", analysis.upper_bounds);
    println!("  Lower bounds: {:?}", analysis.lower_bounds);
    println!("  Strict upper bounds: {:?}", analysis.strict_upper_bounds);
    println!("  Has complex constraints: {}", analysis.has_complex_constraints);
    
    assert!(constraint_count > 0, "Should have registered constraints");
    assert_eq!(constraint_count, metadata_count, "All constraints should have metadata");
}

#[test]
fn test_specific_constraint_types() {
    let mut model = Model::default();
    
    // Create variables
    let x = model.float(0.0, 10.0);
    let y = model.float(5.0, 15.0);
    
    // Test specific constraint types
    model.lt(x, y);  // x < y
    model.ge(y, x); // y >= x
    
    let registry = model.get_props().get_constraint_registry();
    
    // Verify we have the right number of constraints
    assert_eq!(registry.constraint_count(), 2);
    
    // Analyze constraints for variable x
    let analysis = registry.analyze_variable_constraints(x);
    
    // Should detect this as a simple pattern (only comparison constraints)
    println!("Variable x analysis:");
    println!("  Is simple pattern: {}", analysis.is_simple_pattern());
    println!("  Has complex constraints: {}", analysis.has_complex_constraints);
    
    // The pattern should be relatively simple for these basic comparisons
    assert!(!analysis.has_complex_constraints || analysis.is_simple_pattern());
}
