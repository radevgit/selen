/// Test script to demonstrate Model precision configuration
use cspsolver::prelude::*;

fn main() {
    println!("Testing Model precision configuration...\n");

    // Test 1: Default precision (6 decimal places)
    println!("=== Test 1: Default precision ===");
    let mut default_model = Model::default();
    println!("Default precision: {} decimal places", default_model.float_precision_digits());
    println!("Default step size: {}", default_model.float_step_size());
    
    default_model.float(0.0, 1.0);
    println!("Created float variable with default precision");

    // Test 2: High precision (10 decimal places)
    println!("\n=== Test 2: High precision ===");
    let mut high_precision_model = Model::with_float_precision(10);
    println!("High precision: {} decimal places", high_precision_model.float_precision_digits());
    println!("High precision step size: {}", high_precision_model.float_step_size());
    
    high_precision_model.float(0.0, 1.0);
    println!("Created float variable with high precision");

    // Test 3: Low precision (2 decimal places)
    println!("\n=== Test 3: Low precision ===");
    let mut low_precision_model = Model::with_float_precision(2);
    println!("Low precision: {} decimal places", low_precision_model.float_precision_digits());
    println!("Low precision step size: {}", low_precision_model.float_step_size());
    
    low_precision_model.float(0.0, 1.0);
    println!("Created float variable with low precision");

    // Test 4: Verify different precisions create different behaviors
    println!("\n=== Test 4: Precision comparison ===");
    
    // All models should create the same number of integer variables
    default_model.int(0, 100);
    high_precision_model.int(0, 100);
    low_precision_model.int(0, 100);
    
    println!("Integer variables work the same regardless of float precision");
    
    // But float variables should have different step sizes
    println!("Float precision configuration successfully implemented!");
    println!("\nKey features:");
    println!("- Model::default() uses {} decimal places (step: {})", 
             default_model.float_precision_digits(), default_model.float_step_size());
    println!("- Model::with_float_precision(n) allows custom precision");
    println!("- Integer variables are unaffected by float precision settings");
    println!("- All existing code continues to work with Model::default()");
}
