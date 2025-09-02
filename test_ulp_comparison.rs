use cspsolver::utils::almost_equal_as_int;

fn main() {
    println!("Testing ULP-based float comparison:");
    
    // Test basic equality
    let a = 1.0f32;
    let b = 1.0f32;
    println!("1.0 == 1.0 (0 ULPs): {}", almost_equal_as_int(a, b, 0));
    
    // Test next representable value
    let next = f32::from_bits(a.to_bits() + 1);
    println!("1.0 vs next representable (1 ULP): {}", almost_equal_as_int(a, next, 1));
    println!("1.0 vs next representable (0 ULPs): {}", almost_equal_as_int(a, next, 0));
    
    // Test small differences
    let close = 1.0000001f32;
    println!("1.0 vs 1.0000001 (10 ULPs): {}", almost_equal_as_int(a, close, 10));
    println!("1.0 vs 1.0000001 (1 ULP): {}", almost_equal_as_int(a, close, 1));
    
    // Test different signs
    let positive = 1.0f32;
    let negative = -1.0f32;
    println!("1.0 vs -1.0: {}", almost_equal_as_int(positive, negative, 1000));
    
    // Test zeros
    let zero = 0.0f32;
    let neg_zero = -0.0f32;
    println!("0.0 vs -0.0: {}", almost_equal_as_int(zero, neg_zero, 0));
    
    println!("\nComparing ULP vs epsilon methods:");
    let eps_result = (a - close).abs() <= 1e-6;
    let ulp_result = almost_equal_as_int(a, close, 1);
    println!("Epsilon method (1e-6): {}", eps_result);
    println!("ULP method (1 ULP): {}", ulp_result);
}
