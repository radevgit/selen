//! ULP (Unit in the Last Place) Utilities for Floating-Point Precision
//! 
//! This module provides utilities for precise floating-point operations,
//! particularly for handling constraint boundaries with proper floating-point
//! precision awareness.

/// ULP (Unit in the Last Place) utilities for floating-point precision
pub struct UlpUtils;

impl UlpUtils {
    /// Get the ULP (Unit in the Last Place) for a floating-point number
    pub fn ulp(value: f64) -> f64 {
        if value == 0.0 {
            f64::EPSILON
        } else if value.is_infinite() || value.is_nan() {
            f64::NAN
        } else {
            let bits = value.to_bits();
            let next_bits = if value > 0.0 { bits + 1 } else { bits - 1 };
            let next_value = f64::from_bits(next_bits);
            (next_value - value).abs()
        }
    }

    /// Get the previous representable floating-point number
    pub fn prev_float(value: f64) -> f64 {
        if value.is_infinite() && value > 0.0 {
            f64::MAX
        } else if value.is_nan() {
            f64::NAN
        } else {
            let bits = value.to_bits();
            let prev_bits = if value > 0.0 && value != 0.0 {
                bits - 1
            } else if value == 0.0 {
                0x8000_0000_0000_0001u64 // -0.0 but smaller
            } else {
                bits + 1
            };
            f64::from_bits(prev_bits)
        }
    }

    /// Get the next representable floating-point number
    pub fn next_float(value: f64) -> f64 {
        if value.is_infinite() && value < 0.0 {
            f64::MIN
        } else if value.is_nan() {
            f64::NAN
        } else {
            let bits = value.to_bits();
            let next_bits = if value >= 0.0 {
                bits + 1
            } else {
                bits - 1
            };
            f64::from_bits(next_bits)
        }
    }

    /// Calculate precision-aware boundary for strict inequalities
    /// For x < bound, returns the largest representable value less than bound
    pub fn strict_upper_bound(bound: f64) -> f64 {
        Self::prev_float(bound)
    }

    /// Calculate precision-aware boundary for strict inequalities
    /// For x > bound, returns the smallest representable value greater than bound
    pub fn strict_lower_bound(bound: f64) -> f64 {
        Self::next_float(bound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ulp_calculation() {
        // Test ULP calculation
        let value = 5.5;
        let ulp = UlpUtils::ulp(value);
        assert!(ulp > 0.0);
        assert!(ulp < 1e-10); // Should be very small for this value range

        // Test zero case
        let zero_ulp = UlpUtils::ulp(0.0);
        assert_eq!(zero_ulp, f64::EPSILON);
    }

    #[test]
    fn test_next_prev_float() {
        let value = 5.5;
        
        let next = UlpUtils::next_float(value);
        let prev = UlpUtils::prev_float(value);
        
        assert!(next > value);
        assert!(prev < value);
        
        // The difference should be minimal
        assert!((next - value) < 1e-10);
        assert!((value - prev) < 1e-10);
    }

    #[test]
    fn test_strict_bounds() {
        // Test strict bounds
        let strict_upper = UlpUtils::strict_upper_bound(5.5);
        assert!(strict_upper < 5.5);
        assert!(strict_upper > 5.4); // Should be very close to 5.5

        let strict_lower = UlpUtils::strict_lower_bound(5.5);
        assert!(strict_lower > 5.5);
        assert!(strict_lower < 5.6); // Should be very close to 5.5
    }

    #[test]
    fn test_special_values() {
        // Test with infinity
        let pos_inf_prev = UlpUtils::prev_float(f64::INFINITY);
        assert_eq!(pos_inf_prev, f64::MAX);
        
        let neg_inf_next = UlpUtils::next_float(f64::NEG_INFINITY);
        assert_eq!(neg_inf_next, f64::MIN);
        
        // Test with NaN
        assert!(UlpUtils::ulp(f64::NAN).is_nan());
        assert!(UlpUtils::next_float(f64::NAN).is_nan());
        assert!(UlpUtils::prev_float(f64::NAN).is_nan());
    }
}
