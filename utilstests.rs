use cspsolver::utils::{almost_equal_as_int, float_equal, float_perturbed, close_enough};

#[test]
fn test_close_enough() {
    // Test basic epsilon comparison
    assert!(close_enough(1.0, 1.0, 0.0));
    assert!(close_enough(1.0, 1.000001, 0.00001));
    assert!(!close_enough(1.0, 1.1, 0.01));
    
    // Test with different signs
    assert!(!close_enough(1.0, -1.0, 1.0));
    assert!(close_enough(1.0, -1.0, 2.1));
    
    // Test with zeros
    assert!(close_enough(0.0, 0.0, 0.0));
    assert!(close_enough(0.0, -0.0, 0.0));
}

#[test]
fn test_almost_equal_as_int_same_values() {
    assert!(almost_equal_as_int(1.0, 1.0, 0));
    assert!(almost_equal_as_int(0.0, 0.0, 0));
    assert!(almost_equal_as_int(-1.0, -1.0, 0));
}

#[test]
fn test_almost_equal_as_int_different_signs() {
    // Different signs should only be equal if both are exactly zero
    assert!(!almost_equal_as_int(1.0, -1.0, 1000));
    assert!(!almost_equal_as_int(0.1, -0.1, 1000));
    assert!(almost_equal_as_int(0.0, -0.0, 0));
}

#[test]
fn test_almost_equal_as_int_ulp_tolerance() {
    let a = 1.0f32;
    
    // Next representable value (1 ULP away)
    let next = f32::from_bits(a.to_bits() + 1);
    assert!(almost_equal_as_int(a, next, 1));
    assert!(!almost_equal_as_int(a, next, 0));
    
    // 2 ULPs away
    let next2 = f32::from_bits(a.to_bits() + 2);
    assert!(almost_equal_as_int(a, next2, 2));
    assert!(!almost_equal_as_int(a, next2, 1));
}

#[test]
fn test_almost_equal_as_int_negative_values() {
    let a = -1.0f32;
    let next = f32::from_bits(a.to_bits() + 1);
    
    assert!(almost_equal_as_int(a, next, 1));
    assert!(!almost_equal_as_int(a, next, 0));
}

#[test]
fn test_float_equal() {
    // Test basic equality
    assert!(float_equal(1.0, 1.0));
    assert!(float_equal(0.0, 0.0));
    assert!(float_equal(-1.0, -1.0));
    
    // Test very close values (should be equal with 10 ULP tolerance)
    let a = 1.0f32;
    let close = f32::from_bits(a.to_bits() + 5); // 5 ULPs away
    assert!(float_equal(a, close));
    
    // Test values that are too far apart
    let far = a + 0.001;
    assert!(!float_equal(a, far));
    
    // Test different signs
    assert!(!float_equal(1.0, -1.0));
    assert!(float_equal(0.0, -0.0));
}

#[test]
fn test_float_perturbed_basic() {
    let f = 1.0f64;
    
    // Test incrementing
    let next = float_perturbed(f, 1);
    assert!(next > f);
    
    // Test decrementing
    let prev = float_perturbed(f, -1);
    assert!(prev < f);
    
    // Test no change
    let same = float_perturbed(f, 0);
    assert_eq!(same, f);
}

#[test]
fn test_float_perturbed_zero_special_cases() {
    // Special case: 0.0 with c = -1 should return 0.0 (not -0.0)
    let result = float_perturbed(0.0, -1);
    assert_eq!(result, 0.0);
    
    // Special case: -0.0 with c = 1 should return 0.0
    let result = float_perturbed(-0.0, 1);
    assert_eq!(result, 0.0);
}

#[test]
fn test_float_perturbed_large_increments() {
    let f = 1.0f64;
    
    // Test large positive increment
    let far_next = float_perturbed(f, 1000);
    assert!(far_next > f);
    
    // Test large negative increment
    let far_prev = float_perturbed(f, -1000);
    assert!(far_prev < f);
}

#[test]
fn test_float_perturbed_negative_values() {
    let f = -1.0f64;
    
    let next = float_perturbed(f, 1);
    let prev = float_perturbed(f, -1);
    
    // For negative values, incrementing should move towards zero
    assert!(next > f);
    assert!(prev < f);
}

fn test_float_perturbed() {
    println!("Testing float_perturbed:");
    
    // Test incrementing by 1 ULP
    let a = 1.0f64;
    let next = float_perturbed(a, 1);
    assert!(next > a);
    assert_eq!(next.to_bits(), a.to_bits() + 1);
    println!("✓ Increment 1 ULP: 1.0 -> next representable");
    
    // Test decrementing by 1 ULP
    let prev = float_perturbed(a, -1);
    assert!(prev < a);
    assert_eq!(prev.to_bits(), a.to_bits() - 1);
    println!("✓ Decrement 1 ULP: 1.0 -> previous representable");
    
    // Test multiple ULPs
    let far_next = float_perturbed(a, 10);
    assert_eq!(far_next.to_bits(), a.to_bits() + 10);
    println!("✓ Increment 10 ULPs: 1.0 -> +10 ULPs");
    
    let far_prev = float_perturbed(a, -10);
    assert_eq!(far_prev.to_bits(), a.to_bits() - 10);
    println!("✓ Decrement 10 ULPs: 1.0 -> -10 ULPs");
    
    // Test zero perturbation
    let same = float_perturbed(a, 0);
    assert_eq!(same, a);
    println!("✓ Zero perturbation: 1.0 -> 1.0");
    
    // Test special case: 0.0 and -1
    let zero_minus_one = float_perturbed(0.0, -1);
    assert_eq!(zero_minus_one, 0.0); // Should return 0.0, not -0.0
    println!("✓ Special case: 0.0 + (-1 ULP) -> 0.0");
    
    // Test special case: -0.0 and +1
    let neg_zero_plus_one = float_perturbed(-0.0, 1);
    assert_eq!(neg_zero_plus_one, 0.0);
    println!("✓ Special case: -0.0 + (1 ULP) -> 0.0");
    
    // Test negative numbers
    let neg = -1.0f64;
    let neg_next = float_perturbed(neg, 1);
    assert!(neg_next > neg); // Moving towards zero
    assert_eq!(neg_next.to_bits(), neg.to_bits() + 1);
    println!("✓ Negative increment: -1.0 -> next towards zero");
    
    // Test very small positive numbers
    let tiny = 1e-200f64;
    let tiny_next = float_perturbed(tiny, 1);
    assert!(tiny_next > tiny);
    assert_eq!(tiny_next.to_bits(), tiny.to_bits() + 1);
    println!("✓ Tiny number increment: 1e-200 -> next");
    
    // Test large numbers
    let large = 1e100f64;
    let large_next = float_perturbed(large, 1);
    assert!(large_next > large);
    assert_eq!(large_next.to_bits(), large.to_bits() + 1);
    println!("✓ Large number increment: 1e100 -> next");
    
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_all_utils_functions() {
        test_almost_equal_as_int();
        test_float_equal();
        test_float_perturbed();
    }
    
    #[test]
    fn test_consistency_between_functions() {
        let a = 1.0f32;
        let b = f32::from_bits(a.to_bits() + 1);
        
        // float_equal should use 1 ULP tolerance
        assert_eq!(float_equal(a, b), almost_equal_as_int(a, b, 1));
        
        // 2 ULPs should not be equal with float_equal
        let c = f32::from_bits(a.to_bits() + 2);
        assert_eq!(float_equal(a, c), almost_equal_as_int(a, c, 1));
        assert!(!float_equal(a, c)); // Should be false
    }
    
    #[test]
    fn test_edge_cases() {
        // Test NaN behavior is caught by debug_assert
        // (Can't test directly as it would panic in debug mode)
        
        // Test infinity edge cases would also be caught
        
        // Test the actual working edge cases
        assert!(float_equal(f32::MIN, f32::MIN));
        assert!(float_equal(f32::MAX, f32::MAX));
        
        // Test subnormal numbers
        let subnormal = f32::MIN_POSITIVE / 2.0;
        assert!(float_equal(subnormal, subnormal));
    }
}
