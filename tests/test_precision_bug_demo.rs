/// This test demonstrates the precision bug that was missing in FloatSubproblemSolver
/// and shows how proper tests would have caught it.

use cspsolver::prelude::*;
use cspsolver::optimization::subproblem_solving::FloatSubproblemSolver;
use cspsolver::optimization::variable_partitioning::VariablePartition;

#[test]
fn test_precision_bug_would_be_caught() {
    println!("=== Demonstrating Precision Bug Detection ===");
    
    // Create a model with very coarse precision (1 decimal place)
    let mut model = Model::with_float_precision(1);
    let var_id = model.float(0.0, 1.0).into();
    
    println!("Model precision: {} decimal places", model.float_precision_digits());
    println!("Expected step size: {}", model.float_step_size());
    
    let partition = VariablePartition {
        float_variables: vec![var_id],
        integer_variables: vec![],
        constraint_count: 0,
    };
    
    // Create solver with the same precision
    let solver = FloatSubproblemSolver::new(1); // 1 decimal place = 0.1 step
    
    let result = solver.solve_float_subproblem(&model, &partition)
        .expect("Should solve");
    
    if let Some(cspsolver::optimization::subproblem_solving::SubproblemValue::Float(value)) = 
        result.variable_assignments.get(&var_id) {
        
        println!("Solution value: {}", value);
        
        // With 1 decimal precision, solution should be 0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, or 1.0
        let step_size = 0.1;
        let remainder = value % step_size;
        
        println!("Remainder when divided by step size {}: {}", step_size, remainder);
        
        // This assertion would FAIL if precision_digits was missing from the solver
        // because the solver would use arbitrary precision instead of respecting the step boundaries
        assert!(remainder.abs() < 1e-10 || (step_size - remainder).abs() < 1e-10,
            "Value {} should be aligned to 0.1 step boundaries for 1-decimal precision, but remainder is {}", 
            value, remainder);
        
        // Additional check: value should be one of the valid 1-decimal values
        let rounded_to_one_decimal = (value * 10.0).round() / 10.0;
        let diff = (value - rounded_to_one_decimal).abs();
        assert!(diff < 1e-10, 
            "Value {} should equal its 1-decimal rounded version {}, diff: {}", 
            value, rounded_to_one_decimal, diff);
            
        println!("✓ Precision test passed - solver correctly uses 1-decimal precision");
    } else {
        panic!("Expected float solution");
    }
}

#[test]
fn test_extreme_precision_differences() {
    println!("\n=== Testing Extreme Precision Differences ===");
    
    // Test very different precisions to make the bug obvious
    let test_cases = vec![
        (0, 1.0),       // 0 decimals = integer steps
        (1, 0.1),       // 1 decimal  
        (3, 0.001),     // 3 decimals
    ];
    
    for (precision, expected_step) in test_cases {
        println!("\nTesting precision: {} decimals (step size: {})", precision, expected_step);
        
        let mut model = Model::with_float_precision(precision);
        let var_id = model.float(0.0, 2.0).into(); // Wider range to see differences
        
        let partition = VariablePartition {
            float_variables: vec![var_id],
            integer_variables: vec![],
            constraint_count: 0,
        };
        
        let solver = FloatSubproblemSolver::new(precision);
        let result = solver.solve_float_subproblem(&model, &partition)
            .expect("Should solve");
        
        if let Some(cspsolver::optimization::subproblem_solving::SubproblemValue::Float(value)) = 
            result.variable_assignments.get(&var_id) {
            
            println!("  Solution: {}", value);
            
            // Check alignment to step boundaries
            let remainder = value % expected_step;
            println!("  Remainder: {}", remainder);
            
            assert!(remainder.abs() < 1e-12 || (expected_step - remainder).abs() < 1e-12,
                "Precision {} should produce step-aligned values, but {} % {} = {}", 
                precision, value, expected_step, remainder);
                
            println!("  ✓ Correct step alignment");
        }
    }
}

#[test] 
fn test_precision_bug_would_show_non_alignment() {
    println!("\n=== Test That Would Expose Missing Precision Implementation ===");
    
    // This test would fail if FloatSubproblemSolver ignored precision_digits
    // and just used arbitrary floating point arithmetic
    
    let mut model = Model::with_float_precision(2); // 2 decimal places = 0.01 steps
    let var_id = model.float(0.33, 0.67).into(); // Range that doesn't align to nice decimals
    
    let partition = VariablePartition {
        float_variables: vec![var_id],
        integer_variables: vec![],
        constraint_count: 0,
    };
    
    let solver = FloatSubproblemSolver::new(2);
    let result = solver.solve_float_subproblem(&model, &partition).expect("Should solve");
    
    if let Some(cspsolver::optimization::subproblem_solving::SubproblemValue::Float(value)) = 
        result.variable_assignments.get(&var_id) {
        
        println!("Bounds: 0.33 to 0.67");
        println!("Solution: {}", value);
        
        // With 2-decimal precision, valid values in this range are:
        // 0.33, 0.34, 0.35, ..., 0.66, 0.67
        // The midpoint (0.5) should be the solution, which aligns perfectly
        
        let step_size = 0.01;
        let remainder = value % step_size;
        
        println!("Step size: {}", step_size);
        println!("Remainder: {}", remainder);
        
        // If precision was ignored, we might get something like 0.499999999 
        // instead of exactly 0.50, which would fail this test
        assert!(remainder.abs() < 1e-10 || (step_size - remainder).abs() < 1e-10,
            "Solution {} should align to 2-decimal boundaries (0.01 steps)", value);
        
        // More specific: check it's actually a valid 2-decimal number
        let as_hundredths = (value * 100.0).round();
        let reconstructed = as_hundredths / 100.0;
        let diff = (value - reconstructed).abs();
        
        assert!(diff < 1e-12,
            "Solution {} should be exactly representable with 2 decimals. Reconstructed: {}, diff: {}",
            value, reconstructed, diff);
            
        println!("✓ Solution correctly aligned to 2-decimal precision");
    }
}