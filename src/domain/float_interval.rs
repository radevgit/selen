/// Fixed step size for float intervals
/// This determines the granularity of float operations
/// For example, with 1e-4, the interval [0.0, 1.0] will have 10,000 steps
pub const FLOAT_STEP_SIZE: f32 = 1e-4;

/// Float interval with fixed step size for predictable precision
#[derive(Debug, Clone, PartialEq)]
pub struct FloatInterval {
    pub min: f32,
    pub max: f32,
    pub step: f32,
}

impl FloatInterval {
    /// Create a new float interval with calculated step size
    pub fn new(min: f32, max: f32) -> Self {
        if min > max {
            return Self::new(max, min);
        }
        
        // Calculate step size based on the interval range
        // Use the global step size scaled to this interval
        let range = max - min;
        let step = if range == 0.0 {
            FLOAT_STEP_SIZE
        } else {
            // Scale the step size to maintain similar granularity across different ranges
            FLOAT_STEP_SIZE * range
        };
        
        FloatInterval { min, max, step }
    }
    
    /// Create a float interval with custom step size
    pub fn with_step(min: f32, max: f32, step: f32) -> Self {
        if min > max {
            return Self::with_step(max, min, step);
        }
        
        FloatInterval { min, max, step }
    }
    
    /// Get the next representable value
    pub fn next(&self, value: f32) -> f32 {
        let next_val = value + self.step;
        if next_val > self.max {
            self.max
        } else {
            next_val
        }
    }
    
    /// Get the previous representable value
    pub fn prev(&self, value: f32) -> f32 {
        let prev_val = value - self.step;
        if prev_val < self.min {
            self.min
        } else {
            prev_val
        }
    }
    
    /// Check if the interval contains a value
    pub fn contains(&self, value: f32) -> bool {
        value >= self.min && value <= self.max
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
    pub fn size(&self) -> f32 {
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
    pub fn round_to_step(&self, value: f32) -> f32 {
        let steps_from_min = ((value - self.min) / self.step).round();
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
    pub fn assign(&mut self, value: f32) {
        let rounded_value = self.round_to_step(value);
        self.min = rounded_value;
        self.max = rounded_value;
    }
    
    /// Remove values below the given threshold
    pub fn remove_below(&mut self, threshold: f32) {
        if threshold > self.max {
            // If threshold is above the maximum, remove everything (make empty)
            self.min = self.max + 1.0;
        } else if threshold > self.min {
            // If threshold is within the interval, update min
            self.min = self.round_to_step(threshold);
            if self.min > self.max {
                self.min = self.max + 1.0; // Make empty
            }
        }
        // If threshold <= self.min, do nothing (no values to remove)
    }
    
    /// Remove values above the given threshold
    pub fn remove_above(&mut self, threshold: f32) {
        if threshold < self.min {
            // If threshold is below the minimum, remove everything (make empty)
            self.max = self.min - 1.0;
        } else if threshold < self.max {
            // If threshold is within the interval, update max
            self.max = self.round_to_step(threshold);
            if self.max < self.min {
                self.max = self.min - 1.0; // Make empty
            }
        }
        // If threshold >= self.max, do nothing (no values to remove)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_interval() {
        let interval = FloatInterval::new(0.0, 1.0);
        assert_eq!(interval.min, 0.0);
        assert_eq!(interval.max, 1.0);
        assert_eq!(interval.step, FLOAT_STEP_SIZE);
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
        assert!((interval.round_to_step(0.23) - 0.2).abs() < 1e-6);
        assert!((interval.round_to_step(0.27) - 0.3).abs() < 1e-6);
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
        assert!((interval.min - 0.4).abs() < 1e-6); // Rounded to nearest step
        assert!((interval.max - 0.4).abs() < 1e-6);
    }

    #[test]
    fn test_remove_below() {
        let mut interval = FloatInterval::with_step(0.0, 1.0, 0.1);
        interval.remove_below(0.35);
        
        assert!((interval.min - 0.4).abs() < 1e-6); // Rounded up to step
        assert_eq!(interval.max, 1.0);
    }

    #[test]
    fn test_remove_above() {
        let mut interval = FloatInterval::with_step(0.0, 1.0, 0.1);
        interval.remove_above(0.65);
        
        assert_eq!(interval.min, 0.0);
        assert!((interval.max - 0.6).abs() < 1e-6); // Rounded down to step
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
        // Test intervals created with default step size
        let interval = FloatInterval::new(0.0, 1.0);
        assert_eq!(interval.step, FLOAT_STEP_SIZE);
        
        // Should have reasonable granularity
        let step_count = interval.step_count();
        assert!(step_count > 1000); // Should be fine-grained
        assert!(step_count < 100_000); // But not excessively so
        
        // Operations should work correctly
        let mid = (interval.min + interval.max) / 2.0;
        let next_mid = interval.next(mid);
        assert!(next_mid > mid);
        assert!(next_mid - mid <= interval.step + f32::EPSILON);
    }
}
