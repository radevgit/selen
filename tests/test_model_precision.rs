use cspsolver::prelude::*;

#[test]
fn test_precision_configuration() {
    // Test default precision
    let default_model = Model::default();
    assert_eq!(default_model.float_precision_digits(), 6);
    assert_eq!(default_model.float_step_size(), 1e-6);

    // Test custom precision
    let high_precision_model = Model::with_float_precision(10);
    assert_eq!(high_precision_model.float_precision_digits(), 10);
    assert_eq!(high_precision_model.float_step_size(), 1e-10);

    let low_precision_model = Model::with_float_precision(2);
    assert_eq!(low_precision_model.float_precision_digits(), 2);
    assert_eq!(low_precision_model.float_step_size(), 1e-2);

    // Test that variables can be created with different precisions
    let mut model1 = Model::with_float_precision(4);
    let mut model2 = Model::with_float_precision(8);
    
    let _var1 = model1.new_var_float(0.0, 1.0);
    let _var2 = model2.new_var_float(0.0, 1.0);
    
    // Both should succeed without errors
    assert_eq!(model1.float_step_size(), 1e-4);
    assert_eq!(model2.float_step_size(), 1e-8);
}

#[test]
fn test_precision_backward_compatibility() {
    // Verify that existing code using Model::default() continues to work
    let mut model = Model::default();
    
    // These should all work as before
    let _int_var = model.new_var_int(0, 10);
    let _float_var = model.new_var_float(0.0, 1.0);
    let _val_var = model.new_var(Val::int(0), Val::int(5));
    let _values_var = model.new_var_with_values(vec![1, 3, 5, 7]);
    
    // Default precision should be maintained
    assert_eq!(model.float_precision_digits(), 6);
}

#[test]
fn test_precision_with_constraints() {
    let mut model = Model::with_float_precision(3); // 1e-3 precision
    
    let x = model.new_var_float(0.0, 1.0);
    let y = model.new_var_float(0.0, 1.0);
    let sum = model.add(x, y);
    
    model.equals(sum, float(1.5));
    
    let solution = model.solve().expect("Should have solution");
    
    let Val::ValF(x_val) = solution[x] else { panic!("Expected float") };
    let Val::ValF(y_val) = solution[y] else { panic!("Expected float") };
    let Val::ValF(sum_val) = solution[sum] else { panic!("Expected float") };
    
    assert!((x_val + y_val - 1.5).abs() < 1e-2);
    assert!((sum_val - 1.5).abs() < 1e-2);
}

#[test]
fn test_precision_affects_domain_operations() {
    // Test that precision affects floating-point domain operations
    let mut low_precision = Model::with_float_precision(1); // 1e-1 precision
    let mut high_precision = Model::with_float_precision(8); // 1e-8 precision
    
    let x_low = low_precision.new_var_float(0.0, 1.0);
    let x_high = high_precision.new_var_float(0.0, 1.0);
    
    low_precision.gt(x_low, float(0.5));
    high_precision.gt(x_high, float(0.5));
    
    let sol_low = low_precision.minimize(x_low).expect("Should have solution");
    let sol_high = high_precision.minimize(x_high).expect("Should have solution");
    
    let Val::ValF(x_low_val) = sol_low[x_low] else { panic!("Expected float") };
    let Val::ValF(x_high_val) = sol_high[x_high] else { panic!("Expected float") };
    
    // Low precision should have larger step from 0.5
    assert!((x_low_val - 0.6).abs() < 1e-1);
    
    // High precision should have much smaller step from 0.5
    assert!((x_high_val - 0.50000001).abs() < 1e-7);
}

#[test]
fn test_precision_configuration_detailed() {
    // Test from model.rs - comprehensive precision configuration testing
    // Test default precision
    let default_model = Model::default();
    assert_eq!(default_model.float_precision_digits(), 6);
    assert_eq!(default_model.float_step_size(), 1e-6);

    // Test custom precision
    let high_precision_model = Model::with_float_precision(10);
    assert_eq!(high_precision_model.float_precision_digits(), 10);
    assert_eq!(high_precision_model.float_step_size(), 1e-10);

    let low_precision_model = Model::with_float_precision(2);
    assert_eq!(low_precision_model.float_precision_digits(), 2);
    assert_eq!(low_precision_model.float_step_size(), 1e-2);

    // Test that variables can be created with different precisions
    let mut model1 = Model::with_float_precision(4);
    let mut model2 = Model::with_float_precision(8);
    
    let _var1 = model1.new_var_float(0.0, 1.0);
    let _var2 = model2.new_var_float(0.0, 1.0);
    
    // Both should succeed without errors
    assert_eq!(model1.float_step_size(), 1e-4);
    assert_eq!(model2.float_step_size(), 1e-8);
}

#[test]
fn test_precision_backward_compatibility_detailed() {
    // Test from model.rs - verify backward compatibility
    // Verify that existing code using Model::default() continues to work
    let mut model = Model::default();
    
    // These should all work as before
    let _int_var = model.new_var_int(0, 10);
    let _float_var = model.new_var_float(0.0, 1.0);
    let _val_var = model.new_var(Val::int(0), Val::int(5));
    let _values_var = model.new_var_with_values(vec![1, 3, 5, 7]);
    
    // Default precision should be maintained
    assert_eq!(model.float_precision_digits(), 6);
}
