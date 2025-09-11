use cspsolver::prelude::*;

#[test]
fn debug_precision_constraint_analysis() {
    let mut model = Model::default();
    let x = model.new_var_float(1.0, 10.0);
    model.less_than(x, float(5.5));

    // Get access to the constraint registry
    let registry = model.props.get_constraint_registry();
    let constraint_count = registry.constraint_count();
    println!("Total constraints registered: {}", constraint_count);

    // Analyze the variable constraints
    let analysis = registry.analyze_variable_constraints(x);
    println!("Constraint analysis for variable {:?}:", x);
    println!("  Upper bounds: {:?}", analysis.upper_bounds);
    println!("  Strict upper bounds: {:?}", analysis.strict_upper_bounds);
    println!("  Lower bounds: {:?}", analysis.lower_bounds);
    println!("  Strict lower bounds: {:?}", analysis.strict_lower_bounds);
    println!("  Equality values: {:?}", analysis.equality_values);
    
    // Test the effective bound calculation
    let step_size = model.float_step_size();
    let effective_upper = analysis.get_effective_upper_bound(step_size);
    println!("Step size: {}", step_size);
    println!("Effective upper bound: {:?}", effective_upper);
    
    if let Some(upper) = effective_upper {
        // Test the ULP calculation
        let ulp_upper = crate::optimization::ulp_utils::UlpUtils::strict_upper_bound(5.5);
        println!("ULP-based upper bound for x < 5.5: {}", ulp_upper);
        println!("Difference between effective and ULP: {}", (upper - ulp_upper).abs());
        
        assert!(upper < 5.5, "Effective upper bound should be less than 5.5");
        assert!((upper - ulp_upper).abs() < 1e-10, "Should use ULP-based calculation");
    } else {
        panic!("No effective upper bound found for x < 5.5 constraint");
    }
}
