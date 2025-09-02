use cspsolver::utils::*;

fn main() {
    println!("=== Current function behavior ===");
    
    // Test float_prev and float_next with different values
    let test_values = [0.0f32, 1.0f32, -1.0f32];
    
    for &a in &test_values {
        let prev = float_prev(a);
        let next = float_next(a);
        
        println!("a = {}", a);
        println!("  float_prev(a) = {}", prev);
        println!("  float_next(a) = {}", next);
        println!("  prev < a: {}", prev < a);
        println!("  next > a: {}", next > a);
        println!("  float_equal(a, prev): {}", float_equal(a, prev));
        println!("  float_equal(a, next): {}", float_equal(a, next));
        println!();
    }
    
    // Test problematic float_perturbed_as_int cases
    println!("=== Testing float_perturbed_as_int ===");
    let a = 1e6f32;
    for c in [10, -10, 20, -20] {
        let b = float_perturbed_as_int(a, c);
        println!("float_perturbed_as_int({}, {}) = {}, is_finite: {}", a, c, b, b.is_finite());
    }
}
