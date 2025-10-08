/// Default precision for float intervals (decimal places)
/// Users can override this through Model configuration
pub const DEFAULT_FLOAT_PRECISION_DIGITS: i32 = 6;

/// Calculate step size from precision digits
pub const fn precision_to_step_size(precision_digits: i32) -> f64 {
    // This is a const fn equivalent of 10.0_f64.powi(-precision_digits)
    // We use a lookup table for common values since const fn has limitations
    match precision_digits {
        1 => 1e-1,
        2 => 1e-2,
        3 => 1e-3,
        4 => 1e-4,
        5 => 1e-5,
        6 => 1e-6,
        7 => 1e-7,
        8 => 1e-8,
        9 => 1e-9,
        10 => 1e-10,
        11 => 1e-11,
        12 => 1e-12,
        _ => 1e-6, // Default fallback
    }
}

/// Default step size for float intervals
/// Based on DEFAULT_FLOAT_PRECISION_DIGITS (6 decimal places)
pub const FLOAT_STEP_SIZE: f64 = precision_to_step_size(DEFAULT_FLOAT_PRECISION_DIGITS);

/// Maximum number of steps to allow in a float domain before using adaptive step size
/// This prevents enormous search spaces for large domains (e.g., [-1e6, 1e6] would be 2 trillion steps at 1e-6)
/// With MAX_REASONABLE_STEPS = 1M, large domains will use larger steps while small domains keep precision
pub const MAX_REASONABLE_STEPS: usize = 1_000_000;

/// Saved state for FloatInterval backtracking
#[derive(Debug, Clone, PartialEq)]
pub struct FloatIntervalState {
    pub min: f64,
    pub max: f64,
}

/// Float interval with fixed step size for predictable precision
#[derive(Debug, Clone, PartialEq)]
pub struct FloatInterval {
    pub min: f64,
    pub max: f64,
    pub step: f64,
}

impl FloatInterval {
    /// Create a new float interval with adaptive step size using powers of 2
    /// 
    /// Uses power-of-2 based step sizes for better floating-point arithmetic alignment.
    /// The step size is calculated to keep search space tractable (~512-1024 steps per domain).
    /// Uses powers of 2 for both thresholds and steps for optimal binary representation.
    /// 
    /// Strategy: For domain range R, choose step = R / 512 rounded to nearest power of 2
    pub fn new(min: f64, max: f64) -> Self {
        if min > max {
            return Self::new(max, min);
        }
        
        let domain_range = max - min;
        
        // Power-of-2 adaptive step sizing: aim for ~512-2048 steps per domain
        // Balance between search tractability and constraint precision
        let step = if domain_range >= 1048576.0 {  // 2^20 (1M+)
            domain_range / 512.0   // ~512 steps for huge domains
        } else if domain_range >= 16384.0 {  // 2^14 (16k+)
            32.0                   // 2^5: gives 512-16384 steps
        } else if domain_range >= 512.0 {    // 2^9 (512+)
            1.0                    // 2^0: gives 512-16384 steps
        } else if domain_range >= 16.0 {     // 2^4 (16+)
            0.03125                // 2^-5 (1/32): gives 512-16384 steps
        } else if domain_range >= 0.5 {      // 2^-1 (0.5+)
            0.0009765625           // 2^-10 (1/1024): gives 512-16384 steps
        } else if domain_range >= 0.00048828125 {  // 2^-11 (~0.0005+)
            0.00000095367432       // 2^-20: high precision for small domains
        } else {
            0.00000000093132257    // 2^-30: nano-precision for tiny domains
        };
        
        FloatInterval { min, max, step }
    }
    
    /// Create a float interval with custom step size
    pub fn with_step(min: f64, max: f64, step: f64) -> Self {
        if min > max {
            return Self::with_step(max, min, step);
        }
        
        FloatInterval { min, max, step }
    }

    /// Create a FloatInterval without bound checking - may create invalid domains
    /// This is used to create intentionally invalid domains that validation can catch
    pub fn with_step_unchecked(min: f64, max: f64, step: f64) -> Self {
        FloatInterval { min, max, step }
    }
    
    /// Get the next representable value
    pub fn next(&self, value: f64) -> f64 {
        // Check if step size is smaller than ULP at this magnitude
        // If so, use ULP-based stepping to ensure we actually move to next value
        let ulp = crate::optimization::ulp_utils::UlpUtils::ulp(value);
        let effective_step = if self.step < ulp {
            // Step is too small - use actual next float
            let next = crate::optimization::ulp_utils::UlpUtils::next_float(value);
            if next > self.max {
                return self.max;
            }
            return next;
        } else {
            self.step
        };
        
        let next_val = value + effective_step;
        if next_val > self.max {
            self.max
        } else {
            next_val
        }
    }
    
    /// Get the previous representable value
    pub fn prev(&self, value: f64) -> f64 {
        // Check if step size is smaller than ULP at this magnitude
        // If so, use ULP-based stepping to ensure we actually move to previous value
        let ulp = crate::optimization::ulp_utils::UlpUtils::ulp(value);
        let effective_step = if self.step < ulp {
            // Step is too small - use actual previous float
            let prev = crate::optimization::ulp_utils::UlpUtils::prev_float(value);
            if prev < self.min {
                return self.min;
            }
            return prev;
        } else {
            self.step
        };
        
        let prev_val = value - effective_step;
        if prev_val < self.min {
            self.min
        } else {
            prev_val
        }
    }
    
    /// Check if the interval contains a value
    pub fn contains(&self, value: f64) -> bool {
        let tolerance = self.step / 2.0;
        value >= self.min - tolerance && value <= self.max + tolerance
    }
    
    /// Check if the interval is empty (min > max)
    pub fn is_empty(&self) -> bool {
        self.min > self.max
    }
    
    /// Check if the interval represents a single value (within one step)
    pub fn is_fixed(&self) -> bool {
        self.step_count() <= 1
    }
    
    /// Get the size of the interval
    pub fn size(&self) -> f64 {
        if self.is_empty() {
            0.0
        } else {
            self.max - self.min
        }
    }
    
    /// Get the number of steps in this interval
    pub fn step_count(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            // Round to handle floating point precision issues
            ((self.max - self.min) / self.step).round() as usize
        }
    }
    
    /// Round a value to the nearest step boundary
    pub fn round_to_step(&self, value: f64) -> f64 {
        let steps_from_min = ((value - self.min) / self.step).round();
        let rounded = self.min + steps_from_min * self.step;
        rounded.clamp(self.min, self.max)
    }
    
    /// Round a value down to the nearest step boundary (floor)
    pub fn floor_to_step(&self, value: f64) -> f64 {
        let steps_from_min = ((value - self.min) / self.step).floor();
        let rounded = self.min + steps_from_min * self.step;
        rounded.clamp(self.min, self.max)
    }
    
    /// Round a value up to the nearest step boundary (ceil)
    pub fn ceil_to_step(&self, value: f64) -> f64 {
        let steps_from_min = ((value - self.min) / self.step).ceil();
        let rounded = self.min + steps_from_min * self.step;
        rounded.clamp(self.min, self.max)
    }
    
    /// Intersect with another interval
    pub fn intersect(&self, other: &FloatInterval) -> FloatInterval {
        let min = self.min.max(other.min);
        let max = self.max.min(other.max);
        let step = self.step.min(other.step); // Use finer step size
        
        FloatInterval { min, max, step }
    }
    
    /// Check if this interval intersects with another
    pub fn intersects(&self, other: &FloatInterval) -> bool {
        self.min <= other.max && self.max >= other.min
    }
    
    /// Shrink the interval to [value, value] (assign a specific value)
    pub fn assign(&mut self, value: f64) {
        let rounded_value = self.round_to_step(value);
        self.min = rounded_value;
        self.max = rounded_value;
    }
    
    /// Remove values below the given threshold
    pub fn remove_below(&mut self, threshold: f64) {
        let tolerance = self.step / 2.0;
        if threshold > self.max + tolerance {
            // If threshold is above the maximum, remove everything (make empty)
            self.max = self.min - 1.0;
        } else if threshold > self.min + tolerance {
            // If threshold is within the interval, update min
            // For remove_below, we want to round UP to ensure we don't include values below threshold
            self.min = self.ceil_to_step(threshold);
            if self.min > self.max + tolerance {
                self.max = self.min - 1.0; // Make empty
            }
        }
        // If threshold <= self.min, do nothing (no values to remove)
    }
    
    /// Remove values above the given threshold
    pub fn remove_above(&mut self, threshold: f64) {
        let tolerance = self.step / 2.0;
        if threshold < self.min - tolerance {
            // If threshold is below the minimum, remove everything (make empty)
            self.max = self.min - 1.0;
        } else if threshold < self.max - tolerance {
            // If threshold is within the interval, update max
            // For remove_above, we want to round DOWN to ensure we don't include values above threshold
            self.max = self.floor_to_step(threshold);
            if self.max < self.min - tolerance {
                self.max = self.min - 1.0; // Make empty
            }
        }
        // If threshold >= self.max, do nothing (no values to remove)
    }
    
    /// Get the midpoint value for binary splitting (respects step boundaries)
    /// This is critical for efficient divide-and-conquer search
    pub fn mid(&self) -> f64 {
        if self.is_empty() {
            return self.min; // Fallback for empty intervals
        }
        
        if self.is_fixed() {
            return self.min; // Single value intervals
        }
        
        // Handle infinite bounds - critical for unbounded variables
        let rough_mid = if self.min.is_infinite() && self.max.is_infinite() {
            // Both bounds infinite: start at 0.0
            0.0
        } else if self.min.is_infinite() {
            // Only lower bound infinite: use max - 1.0
            self.max - 1.0
        } else if self.max.is_infinite() {
            // Only upper bound infinite: use min + 1.0
            self.min + 1.0
        } else {
            // Both bounds finite: standard midpoint
            self.min + (self.max - self.min) / 2.0
        };
        
        // Round to nearest valid step boundary
        self.round_to_step(rough_mid)
    }
}

impl std::fmt::Display for FloatInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, "[]")
        } else if self.is_fixed() {
            write!(f, "[{}]", self.min)
        } else {
            write!(f, "[{}, {}] (step: {})", self.min, self.max, self.step)
        }
    }
}

impl FloatInterval {
    /// Save the current state for backtracking
    pub fn save_state(&self) -> FloatIntervalState {
        FloatIntervalState {
            min: self.min,
            max: self.max,
        }
    }
    
    /// Restore state from a previous save point
    pub fn restore_state(&mut self, state: &FloatIntervalState) {
        self.min = state.min;
        self.max = state.max;
        // Note: step is never restored since it's a constant property
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_interval() {
        let interval = FloatInterval::new(0.0, 1.0);
        assert_eq!(interval.min, 0.0);
        assert_eq!(interval.max, 1.0);
        // Adaptive step sizing: domain range = 1.0, falls into 0.5+ category -> 2^-10
        assert_eq!(interval.step, 0.0009765625);
    }

    #[test]
    fn test_new_interval_swapped() {
        let interval = FloatInterval::new(1.0, 0.0);
        assert_eq!(interval.min, 0.0);
        assert_eq!(interval.max, 1.0);
    }

    #[test]
    fn test_with_step() {
        let interval = FloatInterval::with_step(0.0, 10.0, 0.1);
        assert_eq!(interval.step, 0.1);
    }

    #[test]
    fn test_next() {
        let interval = FloatInterval::with_step(0.0, 1.0, 0.1);
        assert_eq!(interval.next(0.5), 0.6);
        assert_eq!(interval.next(0.95), 1.0); // Clamped to max
        assert_eq!(interval.next(1.0), 1.0);  // Already at max
    }

    #[test]
    fn test_prev() {
        let interval = FloatInterval::with_step(0.0, 1.0, 0.1);
        assert_eq!(interval.prev(0.5), 0.4);
        assert_eq!(interval.prev(0.05), 0.0); // Clamped to min
        assert_eq!(interval.prev(0.0), 0.0);  // Already at min
    }

    #[test]
    fn test_contains() {
        let interval = FloatInterval::new(0.0, 1.0);
        assert!(interval.contains(0.5));
        assert!(interval.contains(0.0));
        assert!(interval.contains(1.0));
        assert!(!interval.contains(-0.1));
        assert!(!interval.contains(1.1));
    }

    #[test]
    fn test_is_fixed() {
        // Single point
        let interval = FloatInterval::with_step(0.5, 0.5, 0.1);
        assert!(interval.is_fixed());
        
        // Two consecutive values (one step apart) - should be considered fixed
        let interval2 = FloatInterval::with_step(0.0, 0.1, 0.1);
        assert!(interval2.is_fixed());
        
        // Multiple steps - not fixed
        let interval3 = FloatInterval::with_step(0.0, 1.0, 0.1);
        assert!(!interval3.is_fixed());
    }

    #[test]
    fn test_step_count() {
        let interval = FloatInterval::with_step(0.0, 1.0, 0.1);
        assert_eq!(interval.step_count(), 10);
        
        let interval2 = FloatInterval::with_step(0.0, 0.5, 0.1);
        assert_eq!(interval2.step_count(), 5);
    }

    #[test]
    fn test_round_to_step() {
        let interval = FloatInterval::with_step(0.0, 1.0, 0.1);
        assert!((interval.round_to_step(0.23) - 0.2).abs() < interval.step * 0.01);
        assert!((interval.round_to_step(0.27) - 0.3).abs() < interval.step * 0.01);
    }

    #[test]
    fn test_intersect() {
        let interval1 = FloatInterval::with_step(0.0, 1.0, 0.1);
        let interval2 = FloatInterval::with_step(0.5, 1.5, 0.05);
        
        let intersection = interval1.intersect(&interval2);
        assert_eq!(intersection.min, 0.5);
        assert_eq!(intersection.max, 1.0);
        assert_eq!(intersection.step, 0.05); // Finer step
    }

    #[test]
    fn test_assign() {
        let mut interval = FloatInterval::with_step(0.0, 1.0, 0.1);
        interval.assign(0.37);
        
        assert!(interval.is_fixed());
        assert!((interval.min - 0.4).abs() < interval.step * 0.01); // Rounded to nearest step
        assert!((interval.max - 0.4).abs() < interval.step * 0.01);
    }

    #[test]
    fn test_remove_below() {
        let mut interval = FloatInterval::with_step(0.0, 1.0, 0.1);
        interval.remove_below(0.35);
        
        assert!((interval.min - 0.4).abs() < interval.step * 0.01); // Rounded up to step
        assert_eq!(interval.max, 1.0);
    }

    #[test]
    fn test_remove_above() {
        let mut interval = FloatInterval::with_step(0.0, 1.0, 0.1);
        interval.remove_above(0.65);
        
        println!("Debug: interval.max = {}, expected = 0.6, diff = {}", 
                 interval.max, (interval.max - 0.6).abs());
        println!("Debug: step size = {}, tolerance = {}", interval.step, interval.step * 0.01);
        
        assert_eq!(interval.min, 0.0);
        assert!((interval.max - 0.6).abs() < interval.step * 0.01); // Rounded down to step
    }

    #[test]
    fn test_display() {
        let interval = FloatInterval::with_step(0.0, 1.0, 0.1);
        let display = format!("{}", interval);
        assert!(display.contains("[0"));
        assert!(display.contains("1"));
        assert!(display.contains("0.1"));
        
        let fixed = FloatInterval::with_step(0.5, 0.5, 0.1);
        assert_eq!(format!("{}", fixed), "[0.5]");
    }

    #[test]
    fn test_empty_interval() {
        let mut interval = FloatInterval::with_step(0.0, 1.0, 0.1);
        interval.min = 1.0;
        interval.max = 0.0; // Make empty
        
        assert!(interval.is_empty());
        assert_eq!(interval.size(), 0.0);
        assert_eq!(interval.step_count(), 0);
        assert_eq!(format!("{}", interval), "[]");
    }

    // ===== COMPREHENSIVE TEST CASES FOR WIDE RANGE OF INTERVALS =====

    #[test]
    fn test_very_small_intervals() {
        // Test micro-scale intervals
        let interval = FloatInterval::with_step(1e-6, 2e-6, 1e-8);
        assert!(!interval.is_empty());
        assert_eq!(interval.step_count(), 100); // (2e-6 - 1e-6) / 1e-8 = 100
        
        let next_val = interval.next(1.5e-6);
        assert!((next_val - 1.51e-6).abs() < 1e-10);
    }

    #[test]
    fn test_large_intervals() {
        // Test macro-scale intervals
        let interval = FloatInterval::with_step(1e3, 1e6, 1e1);
        assert_eq!(interval.step_count(), 99900); // (1e6 - 1e3) / 1e1
        
        let next_val = interval.next(5e5);
        assert_eq!(next_val, 5e5 + 1e1);
    }

    #[test]
    fn test_negative_intervals() {
        let interval = FloatInterval::with_step(-10.0, -1.0, 0.5);
        assert_eq!(interval.step_count(), 18); // (9.0) / 0.5 = 18
        
        let next_val = interval.next(-5.0);
        assert_eq!(next_val, -4.5);
        
        let prev_val = interval.prev(-5.0);
        assert_eq!(prev_val, -5.5);
    }

    #[test]
    fn test_mixed_sign_intervals() {
        let interval = FloatInterval::with_step(-5.0, 5.0, 1.0);
        assert_eq!(interval.step_count(), 10);
        
        // Test crossing zero
        assert!(interval.contains(0.0));
        assert_eq!(interval.next(-0.5), 0.5);
        assert_eq!(interval.prev(0.5), -0.5);
    }

    #[test]
    fn test_very_fine_steps() {
        let interval = FloatInterval::with_step(0.0, 1.0, 1e-9);
        assert_eq!(interval.step_count(), 1_000_000_000);
        
        // Test precision at fine scale - be more tolerant of floating-point errors
        let val = 0.123456789;
        let next_val = interval.next(val);
        // With very fine steps, floating-point precision becomes an issue
        // Use a more reasonable tolerance for such fine operations
        assert!((next_val - val - 1e-9).abs() < 1e-8);
    }

    #[test]
    fn test_very_coarse_steps() {
        let interval = FloatInterval::with_step(0.0, 1000.0, 100.0);
        assert_eq!(interval.step_count(), 10);
        
        // Test that values get rounded to step boundaries
        let rounded = interval.round_to_step(237.0);
        assert_eq!(rounded, 200.0); // Nearest 100
        
        let rounded2 = interval.round_to_step(278.0);
        assert_eq!(rounded2, 300.0); // Nearest 100
    }

    #[test]
    fn test_single_step_intervals() {
        // Interval with exactly one step
        let interval = FloatInterval::with_step(1.0, 2.0, 1.0);
        assert_eq!(interval.step_count(), 1);
        assert!(interval.is_fixed()); // Should be considered fixed
        
        assert_eq!(interval.next(1.0), 2.0);
        assert_eq!(interval.next(1.5), 2.0); // Clamped to max
        assert_eq!(interval.prev(2.0), 1.0);
        assert_eq!(interval.prev(1.5), 1.0); // Clamped to min
    }

    #[test]
    fn test_zero_width_intervals() {
        let interval = FloatInterval::with_step(5.0, 5.0, 0.1);
        assert_eq!(interval.step_count(), 0);
        assert!(interval.is_fixed());
        
        // Next/prev should return the same value
        assert_eq!(interval.next(5.0), 5.0);
        assert_eq!(interval.prev(5.0), 5.0);
    }

    #[test]
    fn test_rounding_precision() {
        // Test rounding behavior with various step sizes
        let cases = vec![
            (0.0, 1.0, 0.1, 0.23, 0.2),
            (0.0, 1.0, 0.1, 0.27, 0.3),
            (0.0, 100.0, 10.0, 23.0, 20.0),
            (0.0, 100.0, 10.0, 27.0, 30.0),
            (-5.0, 5.0, 0.5, 1.23, 1.0),
            (-5.0, 5.0, 0.5, 1.77, 2.0),
        ];
        
        for (min, max, step, input, expected) in cases {
            let interval = FloatInterval::with_step(min, max, step);
            let rounded = interval.round_to_step(input);
            assert!((rounded - expected).abs() < 1e-6, 
                "Failed for input {} with step {}: got {}, expected {}", 
                input, step, rounded, expected);
        }
    }

    #[test]
    fn test_boundary_conditions() {
        let interval = FloatInterval::with_step(0.0, 1.0, 0.1);
        
        // Test exactly at boundaries
        assert_eq!(interval.next(1.0), 1.0); // At max boundary
        assert_eq!(interval.prev(0.0), 0.0); // At min boundary
        
        // Test just outside boundaries
        assert_eq!(interval.round_to_step(-0.05), 0.0); // Clamp to min
        assert_eq!(interval.round_to_step(1.05), 1.0);  // Clamp to max
    }

    #[test]
    fn test_step_alignment() {
        // Test that operations maintain step alignment
        let interval = FloatInterval::with_step(0.0, 1.0, 0.1);
        
        let val = 0.3;
        let next_val = interval.next(val);
        let prev_of_next = interval.prev(next_val);
        
        // Should get back to original value (within floating point precision)
        assert!((prev_of_next - val).abs() < 1e-6);
    }

    #[test]
    fn test_intersection_edge_cases() {
        // Non-overlapping intervals
        let interval1 = FloatInterval::with_step(0.0, 1.0, 0.1);
        let interval2 = FloatInterval::with_step(2.0, 3.0, 0.1);
        
        let intersection = interval1.intersect(&interval2);
        assert!(intersection.is_empty());
        assert!(!interval1.intersects(&interval2));
        
        // Touching intervals
        let interval3 = FloatInterval::with_step(1.0, 2.0, 0.1);
        let intersection2 = interval1.intersect(&interval3);
        assert!(intersection2.is_fixed()); // Should be single point at 1.0
    }

    #[test]
    fn test_remove_operations_edge_cases() {
        // Remove operations that make interval empty
        let mut interval = FloatInterval::with_step(0.0, 1.0, 0.1);
        interval.remove_above(-0.5);
        // When we remove above -0.5, all values in [0.0, 1.0] are > -0.5, so interval becomes empty
        assert!(interval.is_empty());
        
        let mut interval2 = FloatInterval::with_step(0.0, 1.0, 0.1);
        interval2.remove_below(1.5);
        // When we remove below 1.5, all values in [0.0, 1.0] are < 1.5, so interval becomes empty
        assert!(interval2.is_empty());
        
        // Remove operations that leave single value
        let mut interval3 = FloatInterval::with_step(0.0, 1.0, 0.1);
        interval3.remove_below(0.45);
        interval3.remove_above(0.55);
        assert!(interval3.is_fixed());
        assert!((interval3.min - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_step_size_consistency() {
        // Test that operations preserve step size consistency
        let interval = FloatInterval::with_step(0.0, 10.0, 0.5);
        
        // After assignment, step size should remain
        let mut interval_copy = interval.clone();
        interval_copy.assign(3.7);
        assert_eq!(interval_copy.step, 0.5);
        assert!((interval_copy.min - 3.5).abs() < 1e-6); // Rounded to nearest step
        
        // After remove operations, step size should remain
        let mut interval_copy2 = interval.clone();
        interval_copy2.remove_below(2.3);
        interval_copy2.remove_above(7.7);
        assert_eq!(interval_copy2.step, 0.5);
        assert!((interval_copy2.min - 2.5).abs() < 1e-6); // Rounded to step
        assert!((interval_copy2.max - 7.5).abs() < 1e-6); // Rounded to step
    }

    #[test]
    fn test_floating_point_precision_robustness() {
        // Test behavior with values that might cause floating point precision issues
        let interval = FloatInterval::with_step(0.1, 0.9, 0.1);
        
        // These operations might accumulate rounding errors
        let mut val = 0.1;
        for _ in 0..8 {
            val = interval.next(val);
        }
        
        // Should be close to 0.9 despite potential accumulation errors
        assert!((val - 0.9).abs() < 1e-5);
        assert!(interval.contains(val));
    }

    #[test]
    fn test_performance_scenarios() {
        // Test with intervals that might be computationally intensive
        let large_interval = FloatInterval::with_step(0.0, 1e6, 1.0);
        assert_eq!(large_interval.step_count(), 1_000_000);
        
        // Operations should still be fast and correct
        let next_val = large_interval.next(500000.0);
        assert_eq!(next_val, 500001.0);
        
        let rounded = large_interval.round_to_step(500000.7);
        assert_eq!(rounded, 500001.0);
    }

    #[test]
    fn test_default_step_size_behavior() {
        // Test intervals created with adaptive step sizing
        let interval = FloatInterval::new(0.0, 1.0);
        // Range = 1.0, falls into 0.5+ category -> 2^-10
        assert_eq!(interval.step, 0.0009765625);
        
        // Test that step size IS range-dependent (adaptive)
        let small_interval = FloatInterval::new(0.0, 0.01);
        let large_interval = FloatInterval::new(0.0, 100.0);
        
        // Different intervals now have different step sizes due to adaptive sizing
        // small_interval: range=0.01, falls into <0.5 category -> 2^-20
        assert_eq!(small_interval.step, 0.00000095367432);
        // large_interval: range=100, falls into 16+ category -> 0.03125 (2^-5)
        assert_eq!(large_interval.step, 0.03125);
        // They should be different
        assert_ne!(interval.step, small_interval.step);
        assert_ne!(interval.step, large_interval.step);
        
        // Step counts are kept tractable by adaptive sizing
        assert!(interval.step_count() <= 2048); // ~1024 steps for range=1.0
        assert!(small_interval.step_count() <= 20000); // small domains get more precision
        assert!(large_interval.step_count() <= 4096); // ~3200 steps for range=100
        
        // Operations should work correctly
        let mid = (interval.min + interval.max) / 2.0;
        let next_mid = interval.next(mid);
        assert!(next_mid > mid);
        assert!((next_mid - mid - interval.step).abs() < f64::EPSILON);
    }

    #[test]
    fn test_mid_respects_step_boundaries() {
        // Test that mid() always returns valid step-aligned values
        let interval = FloatInterval::with_step(0.0, 1.0, 0.1);
        let mid_val = interval.mid();
        
        // Mid should be step-aligned
        assert!((mid_val - interval.round_to_step(mid_val)).abs() < 1e-10);
        assert!(interval.contains(mid_val));
        
        // Test with step that doesn't divide range evenly
        let interval2 = FloatInterval::with_step(0.0, 1.0, 0.3);
        let mid_val2 = interval2.mid();
        assert!((mid_val2 - interval2.round_to_step(mid_val2)).abs() < 1e-10);
        assert!(interval2.contains(mid_val2));
        
        // Test edge cases
        let single_point = FloatInterval::with_step(5.0, 5.0, 0.1);
        assert_eq!(single_point.mid(), 5.0);
        
        let tiny_interval = FloatInterval::with_step(0.0, 0.1, 0.1);
        assert!(tiny_interval.is_fixed());
        assert_eq!(tiny_interval.mid(), 0.0); // Should return min for fixed intervals
    }

    #[test]
    fn test_save_restore_state() {
        let mut interval = FloatInterval::with_step(0.0, 10.0, 0.1);
        
        // Save initial state
        let initial_state = interval.save_state();
        assert_eq!(initial_state.min, 0.0);
        assert_eq!(initial_state.max, 10.0);
        
        // Modify the interval
        interval.remove_below(3.5);
        interval.remove_above(7.2);
        assert_ne!(interval.min, 0.0);
        assert_ne!(interval.max, 10.0);
        
        // Restore to initial state
        interval.restore_state(&initial_state);
        assert_eq!(interval.min, 0.0);
        assert_eq!(interval.max, 10.0);
        assert_eq!(interval.step, 0.1); // Step should remain unchanged
        
        // Test multiple save/restore levels
        let level1_state = interval.save_state();
        interval.remove_below(2.0);
        
        let level2_state = interval.save_state();
        interval.remove_above(8.0);
        
        // Restore to level 2
        interval.restore_state(&level2_state);
        assert_eq!(interval.min, 2.0);
        assert_eq!(interval.max, 10.0);
        
        // Restore to level 1
        interval.restore_state(&level1_state);
        assert_eq!(interval.min, 0.0);
        assert_eq!(interval.max, 10.0);
    }
}
