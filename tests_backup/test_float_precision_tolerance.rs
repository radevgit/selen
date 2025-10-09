// Tests for float precision tolerance in bound propagation and constraints
// These tests verify that tolerance-based comparisons work correctly with small float values

use selen::prelude::*;

#[cfg(test)]
mod tolerance_tests {
    use super::*;

    // ========== Tests for try_set_min/max with tolerance ==========

    #[test]
    fn test_try_set_min_with_small_difference() {
        // Test that setting min very close to max doesn't incorrectly fail
        let mut model = Model::default();
        let x = model.float(0.0, 1.0);
        
        // Set bounds that are within tolerance
        model.new(x.ge(0.9999995)); // Within default tolerance of 5e-7
        
        let result = model.solve();
        assert!(result.is_ok(), "Should succeed with values within tolerance");
    }

    #[test]
    fn test_try_set_max_with_small_difference() {
        // Test that setting max very close to min doesn't incorrectly fail
        let mut model = Model::default();
        let x = model.float(0.0, 1.0);
        
        model.new(x.le(0.0000005)); // Within default tolerance
        
        let result = model.solve();
        assert!(result.is_ok(), "Should succeed with values within tolerance");
    }

    #[test]
    fn test_bounds_just_outside_tolerance() {
        // Test that bounds outside tolerance correctly fail
        let mut model = Model::default();
        let x = model.float(0.0, 0.1);
        
        model.new(x.ge(0.15)); // Clearly outside domain
        
        let result = model.solve();
        assert!(result.is_err(), "Should fail with bounds outside tolerance");
    }

    #[test]
    fn test_small_float_coefficients_004() {
        // Original failing case: I=0.04
        let mut model = Model::default();
        let i = model.float(0.0, 10.0);
        let x1 = model.float(1.0, 11.0);
        
        model.new(i.eq(0.04));
        model.float_lin_eq(&[1.0, -1.0], &[i, x1], -1.0); // X1 = I + 1
        
        match model.solve() {
            Ok(sol) => {
                let i_val: f64 = sol.get(i);
                let x1_val: f64 = sol.get(x1);
                
                assert!((i_val - 0.04).abs() < 1e-6, "I should be 0.04");
                assert!((x1_val - 1.04).abs() < 1e-6, "X1 should be 1.04");
            }
            Err(e) => panic!("Should find solution with I=0.04, got error: {:?}", e),
        }
    }

    #[test]
    fn test_small_float_coefficients_001() {
        // Even smaller coefficient: I=0.001
        let mut model = Model::default();
        let i = model.float(0.0, 1.0);
        let x1 = model.float(1.0, 2.0);
        
        model.new(i.eq(0.001));
        model.float_lin_eq(&[1.0, -1.0], &[i, x1], -1.0);
        
        match model.solve() {
            Ok(sol) => {
                let x1_val: f64 = sol.get(x1);
                // Use larger tolerance to account for cascading precision errors in tight constraints
                assert!((x1_val - 1.001).abs() < 1e-5, "X1 should be ~1.001");
            }
            Err(e) => panic!("Should find solution with I=0.001, got error: {:?}", e),
        }
    }

    #[test]
    fn test_accumulated_rounding_errors() {
        // Test multiple operations that could accumulate errors
        let mut model = Model::default();
        let x = model.float(0.0, 1.0);
        let y = model.float(0.0, 1.0);
        let z = model.float(0.0, 1.0);
        
        model.new(x.eq(0.01));
        model.new(y.eq(0.02));
        
        // z = 2*x + 3*y = 0.02 + 0.06 = 0.08
        model.float_lin_eq(&[2.0, 3.0, -1.0], &[x, y, z], 0.0);
        
        match model.solve() {
            Ok(sol) => {
                let z_val: f64 = sol.get(z);
                assert!((z_val - 0.08).abs() < 1e-5, "Z should be 0.08, got {}", z_val);
            }
            Err(e) => panic!("Should handle accumulated errors, got: {:?}", e),
        }
    }

    // ========== Tests for FloatInterval contains/remove methods ==========

    #[test]
    fn test_contains_with_tolerance() {
        let mut model = Model::default();
        let x = model.float(0.0, 1.0);
        
        // Value very close to boundary should be contained
        model.new(x.eq(0.9999999)); // Within tolerance of 1.0
        
        let result = model.solve();
        assert!(result.is_ok(), "Should contain values within tolerance of boundary");
    }

    #[test]
    fn test_remove_below_near_boundary() {
        let mut model = Model::default();
        let x = model.float(0.0, 1.0);
        
        // Remove below a value very close to 0
        model.new(x.ge(0.0000001)); // Within tolerance of 0
        
        let result = model.solve();
        assert!(result.is_ok(), "Should handle remove_below near boundary");
    }

    #[test]
    fn test_remove_above_near_boundary() {
        let mut model = Model::default();
        let x = model.float(0.0, 1.0);
        
        // Remove above a value very close to 1
        model.new(x.le(0.9999999)); // Within tolerance of 1
        
        let result = model.solve();
        assert!(result.is_ok(), "Should handle remove_above near boundary");
    }

    // ========== Tests for float_lin_eq with various coefficients ==========

    #[test]
    fn test_float_lin_eq_very_small_coefficients() {
        let mut model = Model::default();
        let x = model.float(0.0, 10.0);
        let y = model.float(0.0, 10.0);
        
        // 0.001*x + 0.002*y = 0.005
        // With x=1, y=2: 0.001 + 0.004 = 0.005 ✓
        model.new(x.eq(1.0));
        model.new(y.eq(2.0));
        model.float_lin_eq(&[0.001, 0.002], &[x, y], 0.005);
        
        let result = model.solve();
        assert!(result.is_ok(), "Should handle very small coefficients");
    }

    #[test]
    fn test_float_lin_eq_mixed_scale_coefficients() {
        let mut model = Model::default();
        let x = model.float(0.0, 1000.0);
        let y = model.float(0.0, 1.0);
        
        // Large value with small coefficient + small value with large coefficient
        // 0.001*x + 100.0*y = 1.5
        // With x=500, y=0.01: 0.5 + 1.0 = 1.5 ✓
        model.new(x.eq(500.0));
        model.new(y.eq(0.01));
        model.float_lin_eq(&[0.001, 100.0], &[x, y], 1.5);
        
        let result = model.solve();
        assert!(result.is_ok(), "Should handle mixed scale coefficients");
    }

    #[test]
    fn test_float_lin_eq_negative_small_coefficients() {
        let mut model = Model::default();
        let x = model.float(-10.0, 10.0);
        let y = model.float(-10.0, 10.0);
        
        // -0.03*x + 0.05*y = 0.01
        // With x=1: -0.03 + 0.05*y = 0.01 => 0.05*y = 0.04 => y = 0.8
        model.new(x.eq(1.0));
        model.float_lin_eq(&[-0.03, 0.05], &[x, y], 0.01);
        
        match model.solve() {
            Ok(sol) => {
                let y_val: f64 = sol.get(y);
                assert!((y_val - 0.8).abs() < 1e-4, "Y should be 0.8, got {}", y_val);
            }
            Err(e) => panic!("Should handle negative small coefficients, got: {:?}", e),
        }
    }

    // ========== Tests for float_lin_le ==========

    #[test]
    fn test_float_lin_le_small_coefficients() {
        let mut model = Model::default();
        let x = model.float(0.0, 10.0);
        let y = model.float(0.0, 10.0);
        
        // 0.04*x + 0.06*y ≤ 1.0
        model.new(x.eq(5.0));
        model.new(y.eq(10.0));
        // 0.2 + 0.6 = 0.8 ≤ 1.0 ✓
        model.float_lin_le(&[0.04, 0.06], &[x, y], 1.0);
        
        let result = model.solve();
        assert!(result.is_ok(), "Should handle float_lin_le with small coefficients");
    }

    #[test]
    fn test_float_lin_le_at_boundary() {
        let mut model = Model::default();
        let x = model.float(0.0, 10.0);
        
        // 0.04*x ≤ 0.4 (x ≤ 10)
        model.new(x.eq(10.0));
        model.float_lin_le(&[0.04], &[x], 0.4);
        
        let result = model.solve();
        assert!(result.is_ok(), "Should handle boundary case in float_lin_le");
    }

    #[test]
    fn test_float_lin_le_violation() {
        let mut model = Model::default();
        let x = model.float(0.0, 10.0);
        
        // 0.04*x ≤ 0.3 (x ≤ 7.5)
        model.new(x.eq(10.0)); // Forces violation
        model.float_lin_le(&[0.04], &[x], 0.3);
        
        let result = model.solve();
        assert!(result.is_err(), "Should detect violation in float_lin_le");
    }

    // ========== Tests for float_lin_ne ==========

    #[test]
    fn test_float_lin_ne_small_coefficients() {
        let mut model = Model::default();
        let x = model.float(0.0, 10.0);
        let y = model.float(0.0, 10.0);
        
        // 0.03*x + 0.05*y ≠ 0.5
        model.new(x.eq(5.0));
        model.new(y.eq(7.0));
        // 0.15 + 0.35 = 0.5, so this should fail
        model.float_lin_ne(&[0.03, 0.05], &[x, y], 0.5);
        
        let result = model.solve();
        assert!(result.is_err(), "Should detect equality violation in float_lin_ne");
    }

    #[test]
    fn test_float_lin_ne_satisfied() {
        let mut model = Model::default();
        let x = model.float(0.0, 10.0);
        let y = model.float(0.0, 10.0);
        
        // 0.03*x + 0.05*y ≠ 0.5
        model.new(x.eq(5.0));
        // 0.15 + 0.05*y ≠ 0.5 => 0.05*y ≠ 0.35 => y ≠ 7.0
        // So any y except 7.0 should work
        model.float_lin_ne(&[0.03, 0.05], &[x, y], 0.5);
        
        match model.solve() {
            Ok(sol) => {
                let y_val: f64 = sol.get(y);
                // Verify the constraint is satisfied
                let sum = 0.03 * 5.0 + 0.05 * y_val;
                assert!((sum - 0.5).abs() > 1e-5, "Should not equal 0.5");
            }
            Err(e) => panic!("Should satisfy float_lin_ne, got: {:?}", e),
        }
    }

    // ========== Tests with different precision settings ==========

    #[test]
    fn test_with_higher_precision() {
        // Test with 8 decimal places (step = 1e-8)
        let config = SolverConfig::default().with_float_precision(8);
        let mut model = Model::with_config(config);
        
        let x = model.float(0.0, 2.0);
        let y = model.float(0.0, 2.0);
        
        model.new(x.eq(0.0001)); // Small value within range
        model.float_lin_eq(&[1.0, -1.0], &[x, y], -1.0); // y = x + 1
        
        match model.solve() {
            Ok(sol) => {
                let y_val: f64 = sol.get(y);
                assert!((y_val - 1.0001).abs() < 1e-7, "Should handle high precision, got {}", y_val);
            }
            Err(e) => panic!("High precision should work, got: {:?}", e),
        }
    }

    #[test]
    fn test_with_lower_precision() {
        // Test with 4 decimal places (step = 1e-4, tolerance = 5e-5)
        let config = SolverConfig::default().with_float_precision(4);
        let mut model = Model::with_config(config);
        
        let x = model.float(0.0, 10.0);
        let y = model.float(0.0, 10.0);
        
        model.new(x.eq(0.01)); // 0.01 with 4 decimal places
        model.float_lin_eq(&[1.0, -1.0], &[x, y], -1.0);
        
        let result = model.solve();
        assert!(result.is_ok(), "Should work with lower precision");
    }

    // ========== Regression tests for original bug ==========

    #[test]
    fn test_loan_problem_minimal() {
        // Minimal version of the loan problem that was failing
        let mut model = Model::default();
        
        let i = model.float(0.0, 10.0);
        let x1 = model.float(1.0, 11.0);
        
        model.new(i.eq(0.04));
        model.float_lin_eq(&[1.0, -1.0], &[i, x1], -1.0);
        
        match model.solve() {
            Ok(sol) => {
                let i_val: f64 = sol.get(i);
                let x1_val: f64 = sol.get(x1);
                
                assert!((i_val - 0.04).abs() < 1e-6);
                assert!((x1_val - 1.04).abs() < 1e-6);
            }
            Err(e) => panic!("Loan problem regression: {:?}", e),
        }
    }

    #[test]
    fn test_loan_problem_two_steps() {
        // Two-step calculation like in loan problem
        let mut model = Model::default();
        
        let p = model.float(0.0, 10000.0);
        let i = model.float(0.0, 1.0);
        let x1 = model.float(1.0, 2.0);
        let x2 = model.float(0.0, 20000.0);
        
        model.new(p.eq(1000.0));
        model.new(i.eq(0.04));
        
        // X1 = I + 1
        model.float_lin_eq(&[1.0, -1.0], &[i, x1], -1.0);
        
        // X2 = P * X1
        let x2_calc = model.mul(p, x1);
        model.new(x2.eq(x2_calc));
        
        match model.solve() {
            Ok(sol) => {
                let x1_val: f64 = sol.get(x1);
                let x2_val: f64 = sol.get(x2);
                
                assert!((x1_val - 1.04).abs() < 1e-5, "X1 should be 1.04");
                assert!((x2_val - 1040.0).abs() < 1e-3, "X2 should be 1040");
            }
            Err(e) => panic!("Two-step calculation failed: {:?}", e),
        }
    }

    #[test]
    fn test_multiple_small_coefficients_chain() {
        // Chain of operations with small coefficients
        let mut model = Model::default();
        
        let x1 = model.float(0.0, 100.0);
        let x2 = model.float(0.0, 100.0);
        let x3 = model.float(0.0, 10.0);
        
        model.new(x1.eq(10.0));
        
        // Each step multiplies by small coefficient
        model.float_lin_eq(&[0.1, -1.0], &[x1, x2], 0.0); // x2 = 0.1 * x1 = 1.0
        model.float_lin_eq(&[0.1, -1.0], &[x2, x3], 0.0); // x3 = 0.1 * x2 = 0.1
        
        match model.solve() {
            Ok(sol) => {
                let x2_val: f64 = sol.get(x2);
                let x3_val: f64 = sol.get(x3);
                assert!((x2_val - 1.0).abs() < 1e-5, "X2 should be 1.0");
                assert!((x3_val - 0.1).abs() < 1e-5, "X3 should be 0.1");
            }
            Err(e) => panic!("Chain of small coefficients failed: {:?}", e),
        }
    }

    // ========== Edge cases ==========

    #[test]
    fn test_zero_coefficient() {
        let mut model = Model::default();
        let x = model.float(0.0, 10.0);
        let y = model.float(0.0, 10.0);
        
        // 0.0*x + 1.0*y = 5.0 => y = 5.0
        model.float_lin_eq(&[0.0, 1.0], &[x, y], 5.0);
        
        match model.solve() {
            Ok(sol) => {
                let y_val: f64 = sol.get(y);
                assert!((y_val - 5.0).abs() < 1e-6);
            }
            Err(e) => panic!("Zero coefficient should work: {:?}", e),
        }
    }

    #[test]
    fn test_near_zero_constant() {
        let mut model = Model::default();
        let x = model.float(-1.0, 1.0);
        let y = model.float(-1.0, 1.0);
        
        // x + y = 0.000001 (near zero)
        model.new(x.eq(0.5));
        model.new(y.eq(-0.499999));
        model.float_lin_eq(&[1.0, 1.0], &[x, y], 0.000001);
        
        let result = model.solve();
        assert!(result.is_ok(), "Should handle near-zero constant");
    }

    #[test]
    fn test_tolerance_at_exact_boundary() {
        // Test that tolerance doesn't make problems infeasible
        let mut model = Model::default();
        let x = model.float(0.0, 1.0);
        
        // Set to exact boundary
        model.new(x.eq(1.0));
        model.new(x.le(1.0));
        
        let result = model.solve();
        assert!(result.is_ok(), "Exact boundary should work");
    }
}
