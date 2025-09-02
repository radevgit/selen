#[cfg(test)]
mod test_almost_equal_as_int {
    use cspsolver::utils::{almost_equal_as_int, float_perturbed};

    #[test]
    fn test_almost_equal_as_int_negative_zero() {
        // Make sure that zero and negativeZero compare as equal.
        assert!(almost_equal_as_int(0.0, -0.0, 0));
    }

    #[test]
    fn test_almost_equal_as_int_nearby_numbers() {
        // Make sure that nearby numbers compare as equal.
        let result: bool = almost_equal_as_int(2.0, 1.9999999, 10);
        assert_eq!(result, true);
        let result: bool = almost_equal_as_int(-2.0, -1.9999999, 10);
        assert_eq!(result, true);
    }

    #[test]
    fn test_almost_equal_as_int_slightly_more_distant() {
        // Make sure that slightly more distant numbers compare as equal.
        let result: bool = almost_equal_as_int(2.0, 1.9999998, 10);
        assert_eq!(result, true);
        let result: bool = almost_equal_as_int(-2.0, -1.9999998, 10);
        assert_eq!(result, true);
    }

    #[test]
    fn test_almost_equal_as_int_slightly_more_distant_reversed() {
        // Make sure the results are the same with parameters reversed.
        let result: bool = almost_equal_as_int(1.9999998, 2.0, 10);
        assert_eq!(result, true);
        let result: bool = almost_equal_as_int(-1.9999998, -2.0, 10);
        assert_eq!(result, true);
    }

    #[test]
    fn test_almost_equal_as_int_distant() {
        // Make sure that even more distant numbers don't compare as equal.
        let result: bool = almost_equal_as_int(2.0, 1.9999997, 10);
        assert_eq!(result, false);
        let result: bool = almost_equal_as_int(-2.0, -1.9999997, 10);
        assert_eq!(result, false);
    }

    #[test]
    fn test_almost_equal_as_int_distant_reversed() {
        // Make sure the results are the same with parameters reversed
        let result: bool = almost_equal_as_int(1.9999997, 2.0, 10);
        assert_eq!(result, false);
        let result: bool = almost_equal_as_int(-1.9999997, -2.0, 10);
        assert_eq!(result, false);
    }

    #[test]
    fn test_almost_equal_as_int_upper_limit_small() {
        // Upper limit of f32 small distance
        let mut f_u: u32 = f32::MAX.to_bits();
        f_u -= 2;
        let f_f = f32::from_bits(f_u);
        let result: bool = almost_equal_as_int(f32::MAX, f_f, 3);
        assert_eq!(result, true);
    }

    #[test]
    fn test_almost_equal_as_int_upper_limit_large() {
        // Upper limit of f32 large distance
        let mut f_u: u32 = f32::MAX.to_bits();
        f_u -= 4;
        let f_f = f32::from_bits(f_u);
        let result: bool = almost_equal_as_int(f32::MAX, f_f, 3);
        assert_eq!(result, false);
    }

    #[test]
    fn test_almost_equal_as_int_lower_limit_small() {
        // Lower limit of f32 small distance
        let mut f_u: u32 = f32::MIN.to_bits();
        f_u -= 2;
        let f_f = f32::from_bits(f_u);
        let result: bool = almost_equal_as_int(f32::MIN, f_f, 3);
        assert_eq!(result, true);
    }

    #[test]
    fn test_almost_equal_as_int_lower_limit_large() {
        // Lower limit of f32 large distance
        let mut f_u: u32 = f32::MIN.to_bits();
        f_u -= 4;
        let f_f = f32::from_bits(f_u);
        let result: bool = almost_equal_as_int(f32::MIN, f_f, 3);
        assert_eq!(result, false);
    }

    #[test]
    fn test_almost_equal_as_int_some_numbers() {
        let result: bool = almost_equal_as_int(100.0, -300.0, 10);
        assert_eq!(result, false);
    }

    #[test]
    fn test_positive_negative_zero() {
        // Check that positive and negative zeros are equal
        assert!(almost_equal_as_int(-0f32, 0f32, 0));
    }

    #[test]
    fn test_float_equal() {
        // Test exact equality
        assert!(float_equal(1.0, 1.0));
        
        // Test next representable value (should be equal with 1 ULP tolerance)
        let a = 1.0f32;
        let next = f32::from_bits(a.to_bits() + 1);
        assert!(float_equal(a, next));
        
        // Test 2 ULPs away (should not be equal with 1 ULP tolerance)
        let far = f32::from_bits(a.to_bits() + 2);
        assert!(!float_equal(a, far));
        
        // Test different signs
        assert!(!float_equal(1.0, -1.0));
        
        // Test zeros
        assert!(float_equal(0.0, -0.0));
        
        // Test very small numbers
        let tiny1 = 1e-30f32;
        let tiny2 = f32::from_bits(tiny1.to_bits() + 1);
        assert!(float_equal(tiny1, tiny2));
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
        // Test the actual working edge cases
        assert!(float_equal(f32::MIN, f32::MIN));
        assert!(float_equal(f32::MAX, f32::MAX));
        
        // Test subnormal numbers
        let subnormal = f32::MIN_POSITIVE / 2.0;
        assert!(float_equal(subnormal, subnormal));
    }
}

#[cfg(test)]
mod test_float_perturbed {
    use cspsolver::utils::float_perturbed;

    #[test]
    fn test_perturbed_ulps_as_int_0_minus_1() {
        // f = 0.0, c = -1 should return 0.0 (special case for f64)
        let result = float_perturbed(0.0, -1);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_perturbed_ulps_as_int_basic() {
        let t = 1.0f64;
        let tt = float_perturbed(t, -1);
        // Should be able to get back within 1 ULP
        assert!(tt < t);
        assert_eq!(tt.to_bits(), t.to_bits() - 1);

        let t = 1.0f64;
        let tt = float_perturbed(t, -1000);
        assert!(tt < t);
        assert_eq!(tt.to_bits(), t.to_bits() - 1000);

        let t = f64::MAX;
        let tt = float_perturbed(t, -1000);
        assert!(tt < t);
        assert_eq!(tt.to_bits(), t.to_bits() - 1000);
    }

    #[test]
    fn test_float_perturbed_increments() {
        // Test incrementing by 1 ULP
        let a = 1.0f64;
        let next = float_perturbed(a, 1);
        assert!(next > a);
        assert_eq!(next.to_bits(), a.to_bits() + 1);
        
        // Test decrementing by 1 ULP
        let prev = float_perturbed(a, -1);
        assert!(prev < a);
        assert_eq!(prev.to_bits(), a.to_bits() - 1);
        
        // Test multiple ULPs
        let far_next = float_perturbed(a, 10);
        assert_eq!(far_next.to_bits(), a.to_bits() + 10);
        
        let far_prev = float_perturbed(a, -10);
        assert_eq!(far_prev.to_bits(), a.to_bits() - 10);
        
        // Test zero perturbation
        let same = float_perturbed(a, 0);
        assert_eq!(same, a);
    }

    #[test]
    fn test_float_perturbed_special_cases() {
        // Test special case: 0.0 and -1
        let zero_minus_one = float_perturbed(0.0, -1);
        assert_eq!(zero_minus_one, 0.0); // Should return 0.0
        
        // Test special case: -0.0 and +1
        let neg_zero_plus_one = float_perturbed(-0.0, 1);
        assert_eq!(neg_zero_plus_one, 0.0);
        
        // Test negative numbers
        let neg = -1.0f64;
        let neg_next = float_perturbed(neg, 1);
        assert!(neg_next > neg); // Moving towards zero
        assert_eq!(neg_next.to_bits(), neg.to_bits() + 1);
        
        // Test very small positive numbers
        let tiny = 1e-200f64;
        let tiny_next = float_perturbed(tiny, 1);
        assert!(tiny_next > tiny);
        assert_eq!(tiny_next.to_bits(), tiny.to_bits() + 1);
        
        // Test large numbers
        let large = 1e100f64;
        let large_next = float_perturbed(large, 1);
        assert!(large_next > large);
        assert_eq!(large_next.to_bits(), large.to_bits() + 1);
    }

    #[test]
    fn test_float_perturbed_extreme_cases() {
        // Test with maximum perturbations that are still safe
        let t = f64::MAX;
        let tt = float_perturbed(t, -1000000);
        assert!(tt < t);
        assert_eq!(tt.to_bits(), t.to_bits() - 1000000);
    }
}
