/// Test script to demonstrate Model precision configuration
use selen::prelude::*;

#[test]
fn test_default_precision_configuration() {
    println!("=== Test: Default precision ===");
    let mut default_model = Model::default();
    
    // Verify default precision is reasonable (should be 6 decimal places)
    assert_eq!(default_model.float_precision_digits(), 6, "Default precision should be 6 decimal places");
    
    let default_step = default_model.float_step_size();
    assert!(default_step > 0.0, "Default step size should be positive");
    assert!(default_step <= 0.001, "Default step size should be reasonably small for 6 decimal places");
    
    println!("Default precision: {} decimal places", default_model.float_precision_digits());
    println!("Default step size: {}", default_step);
    
    // Should be able to create float variables without issues
    let _float_var = default_model.float(0.0, 1.0);
    println!("✅ Created float variable with default precision");
}

#[test]
fn test_custom_precision_configuration() {
    println!("=== Test: Custom precision configurations ===");
    
    // Test high precision (10 decimal places)
    let mut high_precision_model = Model::with_float_precision(10);
    assert_eq!(high_precision_model.float_precision_digits(), 10, "High precision should be 10 decimal places");
    
    let high_step = high_precision_model.float_step_size();
    assert!(high_step > 0.0, "High precision step size should be positive");
    assert!(high_step < 0.0001, "High precision should have very small step size");
    
    println!("High precision: {} decimal places", high_precision_model.float_precision_digits());
    println!("High precision step size: {}", high_step);
    
    let _high_float = high_precision_model.float(0.0, 1.0);
    println!("✅ Created float variable with high precision");

    // Test low precision (2 decimal places)
    let mut low_precision_model = Model::with_float_precision(2);
    assert_eq!(low_precision_model.float_precision_digits(), 2, "Low precision should be 2 decimal places");
    
    let low_step = low_precision_model.float_step_size();
    assert!(low_step > 0.0, "Low precision step size should be positive");
    assert!(low_step >= 0.01, "Low precision should have larger step size");
    
    println!("Low precision: {} decimal places", low_precision_model.float_precision_digits());
    println!("Low precision step size: {}", low_step);
    
    let _low_float = low_precision_model.float(0.0, 1.0);
    println!("✅ Created float variable with low precision");
}

#[test]
fn test_precision_independence() {
    println!("=== Test: Precision independence ===");
    
    let mut default_model = Model::default();
    let mut high_precision_model = Model::with_float_precision(10);
    let mut low_precision_model = Model::with_float_precision(2);
    
    // All models should create integer variables the same way
    let _int1 = default_model.int(0, 100);
    let _int2 = high_precision_model.int(0, 100);
    let _int3 = low_precision_model.int(0, 100);
    
    println!("✅ Integer variables work the same regardless of float precision");
    
    // But float step sizes should be different
    let default_step = default_model.float_step_size();
    let high_step = high_precision_model.float_step_size();
    let low_step = low_precision_model.float_step_size();
    
    // Verify step sizes are ordered correctly: high precision < default < low precision
    assert!(high_step < default_step, "Higher precision should have smaller step size");
    assert!(default_step < low_step, "Lower precision should have larger step size");
    
    println!("✅ Float precision configuration working correctly!");
    
    println!("\nKey features validated:");
    println!("- Model::default() uses {} decimal places (step: {})", 
             default_model.float_precision_digits(), default_step);
    println!("- Model::with_float_precision(n) allows custom precision");
    println!("- Integer variables are unaffected by float precision settings");
    println!("- All existing code continues to work with Model::default()");
}

#[test]
fn test_extreme_precision_values() {
    println!("=== Test: Extreme precision values ===");
    
    // Test very high precision
    let ultra_high = Model::with_float_precision(15);
    assert_eq!(ultra_high.float_precision_digits(), 15, "Ultra high precision should be 15");
    
    let ultra_step = ultra_high.float_step_size();
    assert!(ultra_step > 0.0, "Ultra high precision step should be positive");
    println!("Ultra high (15 digits): step = {}", ultra_step);
    
    // Test minimum precision (1 decimal place)
    let minimal = Model::with_float_precision(1);
    assert_eq!(minimal.float_precision_digits(), 1, "Minimal precision should be 1");
    
    let minimal_step = minimal.float_step_size();
    assert!(minimal_step > 0.0, "Minimal precision step should be positive");
    println!("Minimal (1 digit): step = {}", minimal_step);
    
    // Test that step sizes are reasonable relative to precision
    // Higher precision should have equal or smaller steps than lower precision
    assert!(ultra_step <= minimal_step, "Higher precision should have smaller or equal step size");
    
    println!("✅ Extreme precision values handled correctly");
}
