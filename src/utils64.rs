const TWO_COMPLEMENT_64: u64 = 0x8000_0000_0000_0000_u64;
const TWO_COMPLEMENT_CI_64: i64 = TWO_COMPLEMENT_64 as i64;

#[doc(hidden)]
// Compares two f64 values for approximate equality
// Use ULP (Units in the Last Place) comparison.
#[inline]
#[must_use]
pub fn almost_equal_as_int64(a: f64, b: f64, ulps: u64) -> bool {
    debug_assert!(a.is_finite());
    debug_assert!(b.is_finite());

    let mut a_i: i64 = a.to_bits() as i64;
    let mut b_i: i64 = b.to_bits() as i64;

    // Make a_i, b_i lexicographically ordered as a twos-complement int
    if a_i < 0i64 {
        a_i = TWO_COMPLEMENT_CI_64 - a_i;
    }
    if b_i < 0i64 {
        b_i = TWO_COMPLEMENT_CI_64 - b_i;
    }

    // Use saturating arithmetic to avoid overflow when values are very far apart
    let diff = (a_i as i128) - (b_i as i128);
    diff.abs() <= ulps as i128
}

pub const FLOAT_INT_EPS_64: u64 = 10;

// /// Compares two f64 values for equality, accounting for the fact that they may come 
// /// from different FloatInterval instances with different discretizations.
// /// Uses ULP-based comparison to handle floating-point precision issues.
// #[doc(hidden)]
// #[inline]
// #[must_use]
// pub fn float_equal_64(a: f64, b: f64) -> bool {
//     almost_equal_as_int64(a, b, FLOAT_INT_EPS_64)
// }

#[doc(hidden)]
#[must_use]
pub fn float_perturbed_as_int64(f: f64, c: i64) -> f64 {
    debug_assert!(f.is_finite());

    if c == 0 {
        return f;
    }

    // Special cases for zero crossings in ULP ordering:
    // +0.0 with -1 perturbation should give -0.0
    // -0.0 with +1 perturbation should give +0.0
    if f == 0.0 && c == -1 {
        return -0.0;
    }
    if f == -0.0 && c == 1 {
        return 0.0;
    }

    // Convert to the same lexicographically ordered space as almost_equal_as_int64
    let f_bits = f.to_bits();
    let f_i = f_bits as i64;

    // Convert to lexicographically ordered space (same as almost_equal_as_int64)
    let lex_value = if f_i < 0 {
        TWO_COMPLEMENT_CI_64 - f_i
    } else {
        f_i
    };

    // Apply perturbation in lexicographic space
    let result_lex = lex_value + c;

    // Convert back from lexicographically ordered space to IEEE float bits
    let result_bits = if result_lex < 0 {
        // Result is negative in lex space, convert back to IEEE negative representation
        (TWO_COMPLEMENT_CI_64 - result_lex) as u64
    } else {
        // Result is positive in lex space, it's already in IEEE positive representation
        result_lex as u64
    };

    f64::from_bits(result_bits)
}

#[must_use]
pub fn float_prev_64(f: f64) -> f64 {
    let eps = -(FLOAT_INT_EPS_64 as i64) - 1;
    float_perturbed_as_int64(f, eps)
}

#[must_use]
pub fn float_next_64(f: f64) -> f64 {
    let eps = FLOAT_INT_EPS_64 as i64 + 1;
    float_perturbed_as_int64(f, eps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_almost_equal_as_int64_basic() {
        // Test exact equality
        assert!(almost_equal_as_int64(1.0, 1.0, 0));
        
        // Test within tolerance
        let a = 1.0f64;
        let b = 1.0000000000000002f64; // Very close to 1.0
        assert!(almost_equal_as_int64(a, b, 10));
        
        // Test outside tolerance
        let c = 1.1f64;
        assert!(!almost_equal_as_int64(a, c, 10));
    }

    // #[test]
    // fn test_float_equal_64() {
    //     // Test the convenience function
    //     assert!(float_equal_64(2.5, 2.5));
    //     
    //     let a = 2.5f64;
    //     let b = 2.5000000000000004f64; // Very close to 2.5
    //     assert!(float_equal_64(a, b));
    // }

    #[test]
    fn test_float_next_prev_64() {
        let f = 2.5f64;
        let next = float_next_64(f);
        let prev = float_prev_64(f);
        
        // Next should be greater than original
        assert!(next > f);
        
        // Prev should be less than original
        assert!(prev < f);
        
        // They should be different from the original
        assert!(f != next);
        assert!(f != prev);
    }

    #[test]
    fn test_float_perturbed_as_int64() {
        let f = 3.14159f64;
        
        // Zero perturbation should return original
        assert_eq!(float_perturbed_as_int64(f, 0), f);
        
        // Positive perturbation should increase
        let perturbed_pos = float_perturbed_as_int64(f, 5);
        assert!(perturbed_pos > f);
        
        // Negative perturbation should decrease
        let perturbed_neg = float_perturbed_as_int64(f, -5);
        assert!(perturbed_neg < f);
    }

    #[test]
    fn test_zero_crossing_64() {
        // Test zero crossing behavior
        assert_eq!(float_perturbed_as_int64(0.0, -1), -0.0);
        assert_eq!(float_perturbed_as_int64(-0.0, 1), 0.0);
    }
}
