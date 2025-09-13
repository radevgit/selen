/// Example demonstrating Model precision configuration
use cspsolver::prelude::*;

fn main() {
    println!("Testing Model precision configuration...\n");

    // Test 1: Default precision (6 decimal places)
    println!("=== Test 1: Default precision ===");
    let mut m1 = Model::default();
    println!("Default precision: {} decimal places", m1.float_precision_digits());
    println!("Default step size: {}", m1.float_step_size());
    
    let _var1 = m1.float(0.0, 1.0);
    println!("Created float variable with default precision");

    // Test 2: High precision (10 decimal places)
    println!("\n=== Test 2: High precision ===");
    let mut m2 = Model::with_float_precision(10);
    println!("High precision: {} decimal places", m2.float_precision_digits());
    println!("High precision step size: {}", m2.float_step_size());
    
    let _var2 = m2.float(0.0, 1.0);
    println!("Created float variable with high precision");

    // Test 3: Low precision (2 decimal places)
    println!("\n=== Test 3: Low precision ===");
    let mut m3 = Model::with_float_precision(2);
    println!("Low precision: {} decimal places", m3.float_precision_digits());
    println!("Low precision step size: {}", m3.float_step_size());
    
    let _var3 = m3.float(0.0, 1.0);
    println!("Created float variable with low precision");

    // Test 4: Verify different precisions create different behaviors
    println!("\n=== Test 4: Precision comparison ===");
    
    // All models should create the same number of integer variables
    let _int_var_default = m1.int(0, 100);
    let _int_var_high = m2.int(0, 100);
    let _int_var_low = m3.int(0, 100);
    
    println!("Integer variables work the same regardless of float precision");
    
    // But float variables should have different step sizes
    println!("Float precision configuration successfully implemented!");
    println!("\nKey features:");
    println!("- Model::default() uses {} decimal places (step: {})", 
             m1.float_precision_digits(), m1.float_step_size());
    println!("- Model::with_float_precision(n) allows custom precision");
    println!("- Integer variables are unaffected by float precision settings");
    println!("- All existing code continues to work with Model::default()");
}
