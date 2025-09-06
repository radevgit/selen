use cspsolver::{model::Model, prelude::float, vars::Val};

/// Test the greater_than propagator with different scenarios to isolate the bug
#[cfg(test)]
mod propagator_tests {
    use super::*;

    #[test]
    fn test_propagator_int_vs_float_basic() {
        println!("=== Basic Integer vs Float Propagation ===");
        
        // Test 1: Simple case that should work
        let mut m1 = Model::default();
        let v1 = m1.new_var_int(1, 5);
        m1.greater_than(v1, float(2.0)); // v1 > 2.0 should give v1 >= 3
        
        let solution = m1.minimize(v1).unwrap();
        let result = match solution[v1] {
            Val::ValI(x) => x,
            _ => panic!("Expected integer"),
        };
        println!("v1 > 2.0: result = {}, expected = 3", result);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_propagator_fractional_boundary() {
        println!("=== Fractional Boundary Cases ===");
        
        // Test 2: The exact failing case
        let mut m2 = Model::default();
        let v2 = m2.new_var_int(1, 10);
        m2.greater_than(v2, float(2.5)); // v2 > 2.5 should give v2 >= 3
        
        let solution = m2.minimize(v2).unwrap();
        let result = match solution[v2] {
            Val::ValI(x) => x,
            _ => panic!("Expected integer"),
        };
        println!("v2 > 2.5: result = {}, expected = 3", result);
        // This is currently failing, returning 4 instead of 3
        
        // Test 3: Edge cases around the boundary
        let mut m3 = Model::default();
        let v3 = m3.new_var_int(1, 10);
        m3.greater_than(v3, float(2.1)); // v3 > 2.1 should give v3 >= 3
        
        let solution = m3.minimize(v3).unwrap();
        let result = match solution[v3] {
            Val::ValI(x) => x,
            _ => panic!("Expected integer"),
        };
        println!("v3 > 2.1: result = {}, expected = 3", result);
        
        // Test 4: Different fractional values
        let mut m4 = Model::default();
        let v4 = m4.new_var_int(1, 10);
        m4.greater_than(v4, float(2.9)); // v4 > 2.9 should give v4 >= 3
        
        let solution = m4.minimize(v4).unwrap();
        let result = match solution[v4] {
            Val::ValI(x) => x,
            _ => panic!("Expected integer"),
        };
        println!("v4 > 2.9: result = {}, expected = 3", result);
    }

    #[test]
    fn test_propagator_domain_sizes() {
        println!("=== Domain Size Impact ===");
        
        // Test 5: Small domain (singleton)
        let mut m5 = Model::default();
        let v5 = m5.new_var_int(3, 3); // Singleton domain
        m5.greater_than(v5, float(2.5)); // v5 > 2.5, v5 = 3 should be valid
        
        let solution = m5.solve();
        match solution {
            Some(sol) => {
                let result = match sol[v5] {
                    Val::ValI(x) => x,
                    _ => panic!("Expected integer"),
                };
                println!("Singleton v5=3 > 2.5: result = {}, valid = true", result);
                assert_eq!(result, 3);
            }
            None => {
                println!("Singleton v5=3 > 2.5: FAILED - should be valid!");
                panic!("v5=3 should satisfy v5 > 2.5");
            }
        }
        
        // Test 6: Small domain starting from failing value
        let mut m6 = Model::default();
        let v6 = m6.new_var_int(2, 3); // Small domain crossing boundary
        m6.greater_than(v6, float(2.5)); // v6 > 2.5, only v6=3 should be valid
        
        let solution = m6.minimize(v6).unwrap();
        let result = match solution[v6] {
            Val::ValI(x) => x,
            _ => panic!("Expected integer"),
        };
        println!("Small domain v6∈[2,3] > 2.5: result = {}, expected = 3", result);
        assert_eq!(result, 3);
        
        // Test 7: Large domain
        let mut m7 = Model::default();
        let v7 = m7.new_var_int(1, 100); // Large domain
        m7.greater_than(v7, float(2.5)); // v7 > 2.5 should give v7 >= 3
        
        let solution = m7.minimize(v7).unwrap();
        let result = match solution[v7] {
            Val::ValI(x) => x,
            _ => panic!("Expected integer"),
        };
        println!("Large domain v7∈[1,100] > 2.5: result = {}, expected = 3", result);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_manual_domain_check() {
        println!("=== Manual Domain Verification ===");
        
        // Test individual values manually
        for test_val in [1, 2, 3, 4, 5] {
            let mut m = Model::default();
            let v = m.new_var_int(test_val, test_val); // Fix to single value
            m.greater_than(v, float(2.5));
            
            let is_valid = m.solve().is_some();
            let should_be_valid = test_val > 2.5 as i32;
            
            println!("v={} > 2.5: valid={}, expected={}", test_val, is_valid, should_be_valid);
            
            if should_be_valid != is_valid {
                println!("  ❌ MISMATCH! {} > 2.5 should be {}", test_val, should_be_valid);
            } else {
                println!("  ✅ Correct");
            }
        }
    }
}
