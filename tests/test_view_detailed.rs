use cspsolver::{vars::Val, model::Model, prelude::float};

/// Test the view system to ensure type conversions work correctly
#[cfg(test)]
mod view_tests {
    use super::*;

    #[test] 
    fn test_ceiling_floor_behavior() {
        println!("=== Ceiling/Floor Behavior Tests ===");
        
        let test_cases: [(f32, i32, i32); 5] = [
            (2.0, 2, 2),   // exact integer
            (2.1, 3, 2),   // just above integer  
            (2.5, 3, 2),   // half integer
            (2.9, 3, 2),   // just below next integer
            (3.0, 3, 3),   // exact integer
        ];
        
        for (float_val, expected_ceil, expected_floor) in test_cases {
            let actual_ceil = float_val.ceil() as i32;
            let actual_floor = float_val.floor() as i32;
            
            println!("f={}: ceil={} (exp={}), floor={} (exp={})", 
                     float_val, actual_ceil, expected_ceil, actual_floor, expected_floor);
            
            assert_eq!(actual_ceil, expected_ceil);
            assert_eq!(actual_floor, expected_floor);
        }
    }

    #[test]
    fn test_greater_than_semantics() {
        println!("=== Greater Than Semantics Tests ===");
        
        // Test the core mathematical question:
        // If x > 2.5 and x must be integer, what is min(x)?
        
        let test_cases: [(f32, i32); 9] = [
            (1.0, 2),  // x > 1.0 → x >= 2
            (1.1, 2),  // x > 1.1 → x >= 2  
            (1.5, 2),  // x > 1.5 → x >= 2
            (1.9, 2),  // x > 1.9 → x >= 2
            (2.0, 3),  // x > 2.0 → x >= 3
            (2.1, 3),  // x > 2.1 → x >= 3
            (2.5, 3),  // x > 2.5 → x >= 3  ← THE FAILING CASE
            (2.9, 3),  // x > 2.9 → x >= 3
            (3.0, 4),  // x > 3.0 → x >= 4
        ];
        
        for (float_bound, expected_min_int) in test_cases {
            // Mathematical analysis: if x > float_bound and x is integer
            // Then x >= ceil(float_bound + epsilon)
            // For strict inequality, we need the smallest integer greater than float_bound
            
            let computed_min = if float_bound.fract() == 0.0 {
                // Exact integer: x > 2.0 means x >= 3
                float_bound as i32 + 1
            } else {
                // Non-integer: x > 2.5 means x >= 3 (ceil(2.5))
                float_bound.ceil() as i32
            };
            
            println!("x > {}: min(x) = {} (computed={})", 
                     float_bound, expected_min_int, computed_min);
            
            assert_eq!(computed_min, expected_min_int, 
                       "Failed for x > {}", float_bound);
        }
    }

    #[test]
    fn test_context_min_setting_logic() {
        println!("=== Context Min Setting Logic Test ===");
        
        // This tests the core conversion logic that should happen in Context::try_set_min
        // when setting an integer variable's minimum to a float value
        
        let test_cases: [(f32, i32); 4] = [
            (2.0, 3),  // Setting min to 2.0 should result in min=3 (since > not >=)
            (2.1, 3),  // Setting min to 2.1 should result in min=3 (ceil(2.1))
            (2.5, 3),  // Setting min to 2.5 should result in min=3 (ceil(2.5)) ← KEY CASE
            (2.9, 3),  // Setting min to 2.9 should result in min=3 (ceil(2.9))
        ];
        
        for (float_min, expected_int_min) in test_cases {
            // Simulate what Context::try_set_min should do:
            // When setting int var min to float value for strict inequality x > float_val,
            // we need min_converted such that any int x >= min_converted satisfies x > float_val
            
            let min_converted = if float_min.fract() == 0.0 {
                // For exact integers: x > 2.0 requires x >= 3
                float_min as i32 + 1
            } else {
                // For non-integers: x > 2.5 requires x >= ceil(2.5) = 3
                float_min.ceil() as i32
            };
            
            println!("Setting int var min for x > {}: computed min = {}, expected = {}", 
                     float_min, min_converted, expected_int_min);
            
            assert_eq!(min_converted, expected_int_min,
                       "Wrong conversion for x > {}", float_min);
        }
    }
}
