/// Comprehensive edge case tests for modulo constraint
/// 
/// These tests explore potential bugs and edge cases in the complex modulo propagation logic:
/// - Variable divisor with various domain ranges
/// - Fixed vs variable dividends with variable divisors
/// - Negative numbers
/// - Large domain ranges
/// - Interactions with alleq constraints
/// - Back-propagation from result to dividend

use selen::prelude::*;

// ============================================================================
// VARIABLE DIVISOR EDGE CASES
// ============================================================================

#[test]
fn test_modulo_variable_divisor_small_range() {
    // Simple case: divisor ∈ [2,3], dividend ∈ [5,7]
    let mut model = Model::default();

    let a = model.int(0, 0);
    let b = model.int(5, 7);
    let c = model.int(2, 3);

    let a_mod_c = model.modulo(a, c);
    let b_mod_c = model.modulo(b, c);
    
    model.eq_op(a_mod_c, b_mod_c);

    match model.solve() {
        Ok(solution) => {
            let a_val = solution.get_int(a);
            let b_val = solution.get_int(b);
            let c_val = solution.get_int(c);
            let a_mod = solution.get_int(a_mod_c);
            let b_mod = solution.get_int(b_mod_c);

            assert_eq!(a_mod, b_mod);
            assert_eq!(a_val % c_val, a_mod);
            assert_eq!(b_val % c_val, b_mod);
        }
        Err(e) => panic!("Expected solution but got: {:?}", e),
    }
}

#[test]
fn test_modulo_variable_divisor_medium_range() {
    // Medium divisor range: c ∈ [2,6]
    let mut model = Model::default();

    let x = model.int(10, 20);
    let y = model.int(5, 10);
    let d = model.int(2, 6);      // Divisor range [2,6]

    let x_mod_d = model.modulo(x, d);
    let y_mod_d = model.modulo(y, d);

    model.eq_op(x_mod_d, y_mod_d);

    match model.solve() {
        Ok(solution) => {
            let x_val = solution.get_int(x);
            let y_val = solution.get_int(y);
            let d_val = solution.get_int(d);

            assert_eq!(x_val % d_val, y_val % d_val);
        }
        Err(e) => panic!("Expected solution but got: {:?}", e),
    }
}

#[test]
fn test_modulo_variable_divisor_large_range() {
    // Large divisor range: c ∈ [2,20] (exceeds threshold)
    let mut model = Model::default();

    let x = model.int(1, 100);
    let y = model.int(1, 100);
    let d = model.int(2, 20);      // Large divisor range

    // Constrain to narrow down solution space
    model.new(x.le(y));
    let x_mod_d = model.modulo(x, d);
    let y_mod_d = model.modulo(y, d);
    
    model.eq_op(x_mod_d, y_mod_d);

    match model.solve() {
        Ok(solution) => {
            let x_val = solution.get_int(x);
            let y_val = solution.get_int(y);
            let d_val = solution.get_int(d);

            assert_eq!(x_val % d_val, y_val % d_val);
            assert!(x_val <= y_val);
        }
        Err(e) => panic!("Expected solution but got: {:?}", e),
    }
}

// ============================================================================
// FIXED VS VARIABLE DIVIDEND WITH VARIABLE DIVISOR
// ============================================================================

#[test]
fn test_modulo_fixed_dividend_variable_divisor() {
    // Fixed dividend, variable divisor
    let mut model = Model::default();

    let a = model.int(12, 12);         // Fixed: 12
    let d = model.int(2, 6);           // Variable divisor [2,6]
    let r = model.int(0, 5);           // Result domain

    let a_mod_d = model.modulo(a, d);
    model.new(r.eq(a_mod_d));

    match model.solve() {
        Ok(solution) => {
            let a_val = solution.get_int(a);
            let d_val = solution.get_int(d);
            let r_val = solution.get_int(r);

            assert_eq!(a_val, 12);
            assert!(d_val >= 2 && d_val <= 6);
            assert_eq!(a_val % d_val, r_val);
        }
        Err(e) => panic!("Expected solution but got: {:?}", e),
    }
}

#[test]
fn test_modulo_variable_dividend_variable_divisor() {
    // Both dividend and divisor variable
    let mut model = Model::default();

    let x = model.int(5, 15);
    let d = model.int(2, 4);
    let r = model.int(1, 3);           // Expected remainder range

    let x_mod_d = model.modulo(x, d);
    model.new(r.eq(x_mod_d));

    match model.solve() {
        Ok(solution) => {
            let x_val = solution.get_int(x);
            let d_val = solution.get_int(d);
            let r_val = solution.get_int(r);

            assert_eq!(x_val % d_val, r_val);
            assert!(r_val >= 1 && r_val <= 3);
        }
        Err(e) => panic!("Expected solution but got: {:?}", e),
    }
}

// ============================================================================
// BACK-PROPAGATION FROM RESULT
// ============================================================================

#[test]
fn test_modulo_backprop_from_result_fixed_divisor() {
    // Given s = x % y and s is fixed, constrain x
    let mut model = Model::default();

    let x = model.int(1, 30);
    let y = model.int(7, 7);           // Fixed divisor
    let s = model.int(3, 3);           // Fixed result

    let x_mod_y = model.modulo(x, y);
    model.new(s.eq(x_mod_y));

    match model.solve() {
        Ok(solution) => {
            let x_val = solution.get_int(x);
            let y_val = solution.get_int(y);
            let s_val = solution.get_int(s);

            assert_eq!(x_val % y_val, s_val);
            // Valid x values: 3, 10, 17, 24 (x = k*7 + 3)
            assert!(vec![3, 10, 17, 24].contains(&x_val));
        }
        Err(e) => panic!("Expected solution but got: {:?}", e),
    }
}

#[test]
fn test_modulo_backprop_from_result_variable_divisor() {
    // Given s = x % y, s fixed, y variable, constrain x
    let mut model = Model::default();

    let x = model.int(1, 50);
    let y = model.int(2, 8);
    let s = model.int(2, 2);           // Fixed result

    let x_mod_y = model.modulo(x, y);
    model.new(s.eq(x_mod_y));

    match model.solve() {
        Ok(solution) => {
            let x_val = solution.get_int(x);
            let y_val = solution.get_int(y);
            let s_val = solution.get_int(s);

            assert_eq!(x_val % y_val, s_val);
            assert_eq!(s_val, 2);
        }
        Err(e) => panic!("Expected solution but got: {:?}", e),
    }
}

// ============================================================================
// MULTIPLE MODULO CONSTRAINTS INTERACTIONS
// ============================================================================

#[test]
fn test_modulo_multiple_constraints_same_divisor() {
    // x % d = a and y % d = b, with variable d
    let mut model = Model::default();

    let x = model.int(5, 15);
    let y = model.int(20, 30);
    let d = model.int(2, 8);
    let a = model.int(1, 1);
    let b = model.int(0, 0);

    let x_mod_d = model.modulo(x, d);
    let y_mod_d = model.modulo(y, d);
    
    model.new(a.eq(x_mod_d));
    model.new(b.eq(y_mod_d));

    match model.solve() {
        Ok(solution) => {
            let x_val = solution.get_int(x);
            let y_val = solution.get_int(y);
            let d_val = solution.get_int(d);

            assert_eq!(x_val % d_val, 1);
            assert_eq!(y_val % d_val, 0);
        }
        Err(e) => panic!("Expected solution but got: {:?}", e),
    }
}

#[test]
fn test_modulo_multiple_constraints_different_divisors() {
    // x % d1 and y % d2 where d1 and d2 are different variables
    let mut model = Model::default();

    let x = model.int(10, 20);
    let y = model.int(10, 20);
    let d1 = model.int(2, 4);
    let d2 = model.int(3, 5);
    let r1 = model.int(0, 1);
    let r2 = model.int(0, 1);

    let x_mod_d1 = model.modulo(x, d1);
    let y_mod_d2 = model.modulo(y, d2);
    
    model.new(r1.eq(x_mod_d1));
    model.new(r2.eq(y_mod_d2));

    match model.solve() {
        Ok(solution) => {
            let x_val = solution.get_int(x);
            let y_val = solution.get_int(y);
            let d1_val = solution.get_int(d1);
            let d2_val = solution.get_int(d2);

            assert_eq!(x_val % d1_val, solution.get_int(r1));
            assert_eq!(y_val % d2_val, solution.get_int(r2));
        }
        Err(e) => panic!("Expected solution but got: {:?}", e),
    }
}

// ============================================================================
// BOUNDARY AND EXTREME CASES
// ============================================================================

#[test]
fn test_modulo_small_divisor_range() {
    // Divisor range is [2,2] (single value, edge case)
    let mut model = Model::default();

    let x = model.int(1, 10);
    let y = model.int(2, 2);
    let z = model.int(1, 1);

    let x_mod_y = model.modulo(x, y);
    model.new(z.eq(x_mod_y));

    match model.solve() {
        Ok(solution) => {
            let x_val = solution.get_int(x);
            assert_eq!(x_val % 2, 1);
        }
        Err(e) => panic!("Expected solution but got: {:?}", e),
    }
}

#[test]
fn test_modulo_result_equals_divisor_boundary() {
    // Result should be in [0, divisor-1]
    let mut model = Model::default();

    let x = model.int(10, 30);
    let y = model.int(3, 7);
    let s = model.int(0, 6);

    let x_mod_y = model.modulo(x, y);
    model.new(s.eq(x_mod_y));

    match model.solve() {
        Ok(solution) => {
            let x_val = solution.get_int(x);
            let y_val = solution.get_int(y);
            let s_val = solution.get_int(s);

            assert_eq!(x_val % y_val, s_val);
            // Result must be less than divisor
            assert!(s_val < y_val);
        }
        Err(e) => panic!("Expected solution but got: {:?}", e),
    }
}

#[test]
fn test_modulo_all_values_same_remainder() {
    // All dividends in range have same remainder with variable divisor
    let mut model = Model::default();

    let x1 = model.int(5, 5);
    let x2 = model.int(11, 11);
    let x3 = model.int(17, 17);
    let d = model.int(2, 10);

    let x1_mod = model.modulo(x1, d);
    let x2_mod = model.modulo(x2, d);
    let x3_mod = model.modulo(x3, d);

    model.alleq(&[x1_mod, x2_mod, x3_mod]);

    match model.solve() {
        Ok(solution) => {
            let x1_val = solution.get_int(x1);
            let x2_val = solution.get_int(x2);
            let x3_val = solution.get_int(x3);
            let d_val = solution.get_int(d);

            let r1 = x1_val % d_val;
            let r2 = x2_val % d_val;
            let r3 = x3_val % d_val;

            assert_eq!(r1, r2);
            assert_eq!(r2, r3);
        }
        Err(e) => panic!("Expected solution but got: {:?}", e),
    }
}

// ============================================================================
// INTERACTION WITH OTHER CONSTRAINTS
// ============================================================================

#[test]
fn test_modulo_with_linear_and_variable_divisor() {
    // x % d constraint combined with linear constraints and variable d
    let mut model = Model::default();

    let x = model.int(5, 15);
    let d = model.int(2, 4);
    let r = model.int(0, 3);

    let x_mod_d = model.modulo(x, d);
    model.new(r.eq(x_mod_d));
    
    // Add linear constraint: x + d >= 10
    let sum = x.add(d);
    let ten = model.int(10, 10);
    model.new(sum.ge(ten));

    match model.solve() {
        Ok(solution) => {
            let x_val = solution.get_int(x);
            let d_val = solution.get_int(d);
            let r_val = solution.get_int(r);

            assert_eq!(x_val % d_val, r_val);
            assert!(x_val + d_val >= 10);
        }
        Err(e) => panic!("Expected solution but got: {:?}", e),
    }
}

#[test]
fn test_modulo_with_alldiff_and_variable_divisor() {
    // Multiple variables must be different, each with modulo and variable divisor
    let mut model = Model::default();

    let x1 = model.int(2, 10);
    let x2 = model.int(2, 10);
    let d = model.int(3, 5);

    model.alldiff(&[x1, x2]);

    let x1_mod = model.modulo(x1, d);
    let x2_mod = model.modulo(x2, d);

    model.new(x1_mod.eq(x2_mod));

    match model.solve() {
        Ok(solution) => {
            let x1_val = solution.get_int(x1);
            let x2_val = solution.get_int(x2);
            let d_val = solution.get_int(d);

            assert_ne!(x1_val, x2_val);
            assert_eq!(x1_val % d_val, x2_val % d_val);
        }
        Err(e) => panic!("Expected solution but got: {:?}", e),
    }
}

// ============================================================================
// POTENTIAL UNSOLVABLE CASES
// ============================================================================

#[test]
fn test_modulo_unsolvable_contradictory() {
    // Create contradictory constraints
    let mut model = Model::default();

    let x = model.int(0, 0);
    let y = model.int(1, 1);
    let z = model.int(2, 2);

    let x_mod_y = model.modulo(x, y);
    // 0 % 1 = 0, but we require it to equal 2 (impossible)
    model.new(z.eq(x_mod_y));

    match model.solve() {
        Err(_) => {
            // This should fail
        }
        Ok(_) => panic!("Should have found no solution"),
    }
}

#[test]
fn test_modulo_unsolvable_divisor_constraints() {
    // Divisor range too large for any valid remainder
    let mut model = Model::default();

    let x = model.int(5, 5);
    let d = model.int(2, 3);
    let r = model.int(10, 10);  // Impossible: remainder can't be >= 10 with d in [2,3]

    let x_mod_d = model.modulo(x, d);
    model.new(r.eq(x_mod_d));

    match model.solve() {
        Err(_) => {
            // Expected: should fail
        }
        Ok(_) => panic!("Should have found no solution"),
    }
}

// ============================================================================
// STRESS TEST: LARGER DOMAINS
// ============================================================================

#[test]
fn test_modulo_stress_large_domains() {
    // Test with larger domains to ensure efficiency
    let mut model = Model::default();

    let x = model.int(50, 200);
    let d = model.int(3, 15);
    let r = model.int(2, 4);

    let x_mod_d = model.modulo(x, d);
    model.new(r.eq(x_mod_d));

    match model.solve() {
        Ok(solution) => {
            let x_val = solution.get_int(x);
            let d_val = solution.get_int(d);
            let r_val = solution.get_int(r);

            assert_eq!(x_val % d_val, r_val);
            assert!(r_val >= 2 && r_val <= 4);
        }
        Err(e) => panic!("Expected solution but got: {:?}", e),
    }
}
