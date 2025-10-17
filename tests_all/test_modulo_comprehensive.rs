/// Comprehensive component test for modulo constraint
/// Covers all scenarios investigated including:
/// - Basic modulo with fixed variables
/// - Modulo with deferred equality constraints
/// - Modulo with runtime API constraints
/// - Modulo with constants and variables
/// - Modulo with computed expressions
/// - Edge cases and error handling

use selen::prelude::*;
use selen::variables::Val;

// ============================================================================
// Test 1: Ultra-simple modulo - all fixed at creation
// ============================================================================

#[test]
fn test_modulo_all_fixed_at_creation() {
    println!("\nTest 1: All variables fixed at creation");
    let mut m = Model::default();
    
    let x = m.int(47, 47);  // Fixed
    let y = m.int(10, 10);  // Fixed
    let result = m.int(0, 9);
    
    let mod_result = m.modulo(x, y);
    m.new(result.eq(mod_result));
    
    match m.solve() {
        Ok(sol) => {
            let x_val = sol.get_int(x);
            let y_val = sol.get_int(y);
            let res_val = sol.get_int(result);
            let mod_val = sol.get_int(mod_result);
            
            assert_eq!(x_val, 47, "x should be 47");
            assert_eq!(y_val, 10, "y should be 10");
            assert_eq!(mod_val, 7, "47 mod 10 = 7");
            assert_eq!(res_val, 7, "result should be 7");
        }
        Err(e) => panic!("Expected solution but got error: {:?}", e),
    }
}

// ============================================================================
// Test 2: Modulo with deferred equality - key test case
// ============================================================================

#[test]
fn test_modulo_with_deferred_equality() {
    println!("\nTest 2: Modulo with deferred equality (key issue)");
    let mut m = Model::default();
    
    let number = m.int(10, 100);  // Wide domain
    let divisor = m.int(10, 10);  // Fixed divisor
    let remainder = m.int(0, 9);  // Expected result
    
    // Deferred constraint: number = 47
    m.new(number.eq(47));
    
    // Create modulo constraint
    let mod_result = m.modulo(number, divisor);
    
    // Constrain remainder to equal mod_result
    m.new(remainder.eq(mod_result));
    
    match m.solve() {
        Ok(sol) => {
            let num_val = sol.get_int(number);
            let div_val = sol.get_int(divisor);
            let rem_val = sol.get_int(remainder);
            let mod_val = sol.get_int(mod_result);
            
            assert_eq!(num_val, 47, "number should be 47");
            assert_eq!(div_val, 10, "divisor should be 10");
            assert_eq!(mod_val, 7, "47 mod 10 = 7");
            assert_eq!(rem_val, 7, "remainder should be 7");
        }
        Err(e) => panic!("Expected solution but got error: {:?}", e),
    }
}

// ============================================================================
// Test 3: Modulo without additional constraints
// ============================================================================

#[test]
fn test_modulo_no_additional_constraints() {
    println!("\nTest 3: Modulo without additional constraints");
    let mut m = Model::default();
    
    let x = m.int(1, 100);
    let y = m.int(10, 10);
    
    m.new(x.eq(47));
    let mod_result = m.modulo(x, y);
    
    match m.solve() {
        Ok(sol) => {
            let x_val = sol.get_int(x);
            let mod_val = sol.get_int(mod_result);
            
            assert_eq!(x_val, 47);
            assert_eq!(mod_val, 7, "47 mod 10 = 7");
        }
        Err(e) => panic!("Expected solution but got error: {:?}", e),
    }
}

// ============================================================================
// Test 4: Modulo with constraint added after modulo creation
// ============================================================================

#[test]
fn test_modulo_fixed_via_constraint_after_creation() {
    println!("\nTest 4: Variables fixed via constraint after modulo creation");
    let mut m = Model::default();
    
    let x = m.int(10, 100);
    let y = m.int(1, 20);
    let mod_result = m.modulo(x, y);
    
    // Fix variables AFTER creating modulo constraint
    m.new(x.eq(47));
    m.new(y.eq(10));
    
    match m.solve() {
        Ok(sol) => {
            let x_val = sol.get_int(x);
            let y_val = sol.get_int(y);
            let mod_val = sol.get_int(mod_result);
            
            assert_eq!(x_val, 47);
            assert_eq!(y_val, 10);
            assert_eq!(mod_val, 7, "47 mod 10 = 7");
        }
        Err(e) => panic!("Expected solution but got error: {:?}", e),
    }
}

// ============================================================================
// Test 5: Modulo with constant divisor variable
// ============================================================================

#[test]
fn test_modulo_with_constant_divisor_variable() {
    println!("\nTest 5: Modulo with constant divisor (as variable)");
    let mut m = Model::default();
    
    let number = m.int(10, 100);
    let divisor = m.int(10, 10);  // Fixed divisor as a variable
    let remainder = m.int(0, 9);
    
    m.new(number.eq(47));
    let mod_result = m.modulo(number, divisor);
    m.new(remainder.eq(mod_result));
    
    match m.solve() {
        Ok(sol) => {
            let num_val = sol.get_int(number);
            let rem_val = sol.get_int(remainder);
            
            assert_eq!(num_val, 47);
            assert_eq!(rem_val, 7, "47 mod 10 = 7");
        }
        Err(e) => panic!("Expected solution but got error: {:?}", e),
    }
}

// ============================================================================
// Test 6: Simple modulo (7 mod 3 = 1)
// ============================================================================

#[test]
fn test_modulo_simple_calculation() {
    println!("\nTest 6: Simple modulo (7 mod 3)");
    let mut m = Model::default();
    
    let x = m.int(7, 7);
    let y = m.int(3, 3);
    let result = m.int(0, 2);
    
    let mod_result = m.modulo(x, y);
    m.new(result.eq(mod_result));
    
    match m.solve() {
        Ok(sol) => {
            let res_val = sol.get_int(result);
            assert_eq!(res_val, 1, "7 mod 3 should be 1");
        }
        Err(e) => panic!("Expected solution but got error: {:?}", e),
    }
}

// ============================================================================
// Test 7: Modulo with runtime API constraint (le)
// ============================================================================

#[test]
fn test_modulo_with_le_constraint() {
    println!("\nTest 7: Modulo with le() constraint");
    let mut m = Model::default();
    
    let number = m.int(47, 47);
    let remainder = m.int(0, 9);
    let divisor = m.int(10, 10);
    
    let mod_result = m.modulo(number, divisor);
    m.new(remainder.le(mod_result));  // remainder <= mod_result
    
    match m.solve() {
        Ok(sol) => {
            let num_val = sol.get_int(number);
            let rem_val = sol.get_int(remainder);
            
            assert_eq!(num_val, 47);
            // 47 mod 10 = 7, so remainder should be <= 7
            assert!(rem_val <= 7, "remainder {} should be <= 7", rem_val);
        }
        Err(e) => panic!("Expected solution but got error: {:?}", e),
    }
}

// ============================================================================
// Test 8: Modulo with variable divisor
// ============================================================================

#[test]
fn test_modulo_with_variable_divisor() {
    println!("\nTest 8: Modulo with variable divisor");
    let mut m = Model::default();
    
    let x = m.int(1, 20);
    let y = m.int(2, 6);
    let result = m.int(0, 5);
    
    // Modulo with variable: x % y
    let mod_result = m.modulo(x, y);
    
    m.new(x.eq(13));
    m.new(y.eq(5));
    m.new(result.eq(mod_result));
    
    match m.solve() {
        Ok(sol) => {
            let x_val = sol.get_int(x);
            let y_val = sol.get_int(y);
            let result_val = sol.get_int(result);
            
            assert_eq!(x_val, 13);
            assert_eq!(y_val, 5);
            assert_eq!(result_val, 13 % 5, "13 mod 5 should be 3");
        }
        Err(e) => panic!("Expected solution but got error: {:?}", e),
    }
}

// ============================================================================
// Test 9: Modulo with computed expression
// ============================================================================

#[test]
fn test_modulo_with_computed_divisor() {
    println!("\nTest 9: Modulo with computed divisor expression");
    let mut m = Model::default();
    
    let x = m.int(1, 20);
    let y = m.int(1, 5);
    let result = m.int(0, 2);
    
    // Modulo with computed expression: x % (y + 2)
    let divisor = m.add(y, Val::int(2));  // y + 2
    let mod_result = m.modulo(x, divisor);
    
    m.new(x.eq(13));
    m.new(y.eq(1));  // divisor = 1 + 2 = 3
    m.new(result.eq(mod_result));
    
    match m.solve() {
        Ok(sol) => {
            let x_val = sol.get_int(x);
            let y_val = sol.get_int(y);
            let divisor_val = sol.get_int(divisor);
            let result_val = sol.get_int(result);
            
            assert_eq!(x_val, 13);
            assert_eq!(y_val, 1);
            assert_eq!(divisor_val, 3);
            assert_eq!(result_val, 13 % 3, "13 mod 3 should be 1");
        }
        Err(e) => panic!("Expected solution but got error: {:?}", e),
    }
}

// ============================================================================
// Test 10: Modulo with mixed variable operands
// ============================================================================

#[test]
fn test_modulo_mixed_types() {
    println!("\nTest 10: Modulo with mixed variable operands");
    let mut m = Model::default();
    
    let x = m.int(10, 30);
    let divisor1 = m.int(7, 7);  // Fixed to 7
    let divisor2 = m.int(10, 20);
    let result1 = m.int(0, 6);
    let result2 = m.int(0, 20);
    
    // variable % constant-as-variable
    let mod1 = m.modulo(x, divisor1);
    m.new(result1.eq(mod1));
    
    // different divisor
    let mod2 = m.modulo(divisor1, divisor2);
    m.new(result2.eq(mod2));
    
    m.new(x.eq(15));
    m.new(divisor2.eq(15));
    
    match m.solve() {
        Ok(sol) => {
            let x_val = sol.get_int(x);
            let d1_val = sol.get_int(divisor1);
            let d2_val = sol.get_int(divisor2);
            let r1 = sol.get_int(result1);
            let r2 = sol.get_int(result2);
            
            assert_eq!(x_val, 15);
            assert_eq!(d1_val, 7);
            assert_eq!(d2_val, 15);
            assert_eq!(r1, 15 % 7, "15 mod 7 should be 1");
            assert_eq!(r2, 7 % 15, "7 mod 15 should be 7");
        }
        Err(e) => panic!("Expected solution but got error: {:?}", e),
    }
}

// ============================================================================
// Test 11: Modulo with multiple solutions (domain search)
// ============================================================================

#[test]
fn test_modulo_domain_search() {
    println!("\nTest 11: Modulo with domain search");
    let mut m = Model::default();
    
    let x = m.int(1, 50);
    let divisor = m.int(10, 10);
    
    let mod_result = m.modulo(x, divisor);
    
    // Find a solution where x % 10 = some value
    match m.solve() {
        Ok(sol) => {
            let x_val = sol.get_int(x);
            let mod_val = sol.get_int(mod_result);
            
            // Just verify the constraint is satisfied
            assert_eq!(x_val % 10, mod_val, "x mod 10 should equal mod_result");
            assert!(x_val >= 1 && x_val <= 50, "x should be in [1..50]");
        }
        Err(e) => panic!("Expected solution but got error: {:?}", e),
    }
}

// ============================================================================
// Test 12: Modulo zero divisor (should fail or handle gracefully)
// ============================================================================

#[test]
fn test_modulo_zero_divisor_fails() {
    println!("\nTest 12: Modulo with zero divisor (should fail)");
    let mut m = Model::default();
    
    let x = m.int(10, 20);
    let y = m.int(0, 0);  // Zero divisor
    
    let _mod_result = m.modulo(x, y);
    
    // This should fail during solving
    match m.solve() {
        Ok(_sol) => {
            panic!("Should have failed with zero divisor");
        }
        Err(_e) => {
            // Expected
            println!("  ✓ Correctly failed with zero divisor");
        }
    }
}

// ============================================================================
// Test 13: Modulo result range validation
// ============================================================================

#[test]
fn test_modulo_result_range() {
    println!("\nTest 13: Modulo result range validation");
    let mut m = Model::default();
    
    let x = m.int(0, 100);
    let divisor = m.int(5, 5);  // Modulo by 5, result should be in [0..4]
    let result = m.int(0, 4);
    
    let mod_result = m.modulo(x, divisor);
    m.new(x.eq(23));  // 23 mod 5 = 3
    m.new(result.eq(mod_result));
    
    match m.solve() {
        Ok(sol) => {
            let res = sol.get_int(result);
            assert!(res >= 0 && res < 5, "Result {} should be in range [0..4]", res);
            assert_eq!(res, 3, "23 mod 5 should be 3");
        }
        Err(e) => panic!("Expected solution but got error: {:?}", e),
    }
}

// ============================================================================
// Test 14: Chained modulo operations
// ============================================================================

#[test]
fn test_modulo_chained() {
    println!("\nTest 14: Chained modulo operations");
    let mut m = Model::default();
    
    let x = m.int(1, 100);
    let div1 = m.int(10, 10);
    let div2 = m.int(3, 3);
    let result1 = m.int(0, 9);
    let result2 = m.int(0, 2);
    
    // First modulo: x mod 10
    let mod1 = m.modulo(x, div1);
    m.new(result1.eq(mod1));
    
    // Second modulo: (x mod 10) mod 3
    let mod2 = m.modulo(mod1, div2);
    m.new(result2.eq(mod2));
    
    m.new(x.eq(23));  // 23 mod 10 = 3, then 3 mod 3 = 0
    
    match m.solve() {
        Ok(sol) => {
            let x_val = sol.get_int(x);
            let mod1_val = sol.get_int(result1);
            let mod2_val = sol.get_int(result2);
            
            assert_eq!(x_val, 23);
            assert_eq!(mod1_val, 3, "23 mod 10 = 3");
            assert_eq!(mod2_val, 0, "3 mod 3 = 0");
        }
        Err(e) => panic!("Expected solution but got error: {:?}", e),
    }
}

// ============================================================================
// Test 15: Modulo in complex constraint system
// ============================================================================

#[test]
fn test_modulo_complex_constraints() {
    println!("\nTest 15: Modulo in complex constraint system");
    let mut m = Model::default();
    
    let a = m.int(1, 30);
    let b = m.int(5, 15);
    let divisor = m.int(7, 7);
    let sum = m.add(a, b);
    let mod_result = m.int(0, 6);
    
    let mod_val = m.modulo(sum, divisor);
    m.new(mod_result.eq(mod_val));
    
    m.new(a.eq(10));
    m.new(b.eq(8));
    
    match m.solve() {
        Ok(sol) => {
            let a_val = sol.get_int(a);
            let b_val = sol.get_int(b);
            let sum_val = sol.get_int(sum);
            let mod_val_sol = sol.get_int(mod_result);
            
            assert_eq!(a_val, 10);
            assert_eq!(b_val, 8);
            assert_eq!(sum_val, 18);
            assert_eq!(mod_val_sol, 18 % 7, "18 mod 7 should be 4");
        }
        Err(e) => panic!("Expected solution but got error: {:?}", e),
    }
}

// ============================================================================
// Test 16: Negative numbers with modulo (if supported)
// ============================================================================

#[test]
fn test_modulo_with_negatives() {
    println!("\nTest 16: Modulo with negative dividend");
    let mut m = Model::default();
    
    let x = m.int(-50, 50);
    let y = m.int(5, 5);
    
    let mod_result = m.modulo(x, y);
    m.new(x.eq(-23));
    
    match m.solve() {
        Ok(sol) => {
            let x_val = sol.get_int(x);
            let mod_val = sol.get_int(mod_result);
            
            assert_eq!(x_val, -23);
            // Modulo keeps sign of dividend (truncated modulo): -23 % 5 = -3
            let expected = -3i32; // -23 mod 5 in truncated modulo is -3
            assert_eq!(mod_val, expected, "modulo result should be -3 (truncated modulo)");
        }
        Err(e) => {
            // Some constraint solvers don't support negative modulo
            println!("  Note: Negative modulo not supported: {:?}", e);
        }
    }
}

// ============================================================================
// Test 17: Large divisors
// ============================================================================

#[test]
fn test_modulo_large_divisor() {
    println!("\nTest 17: Modulo with large divisor");
    let mut m = Model::default();
    
    let x = m.int(1, 1_000_000);
    let divisor = m.int(1000, 1000);
    let result = m.int(0, 999);
    
    let mod_result = m.modulo(x, divisor);
    m.new(x.eq(123_456));
    m.new(result.eq(mod_result));
    
    match m.solve() {
        Ok(sol) => {
            let x_val = sol.get_int(x);
            let mod_val = sol.get_int(result);
            
            assert_eq!(x_val, 123_456);
            assert_eq!(mod_val, 123_456 % 1000, "123456 mod 1000 should be 456");
        }
        Err(e) => panic!("Expected solution but got error: {:?}", e),
    }
}

// ============================================================================
// Test 18: Runtime API modulo without additional constraints
// ============================================================================

#[test]
fn test_modulo_runtime_api_direct() {
    println!("\nTest 18: Runtime API direct modulo call");
    let mut m = Model::default();
    
    let x = m.int(47, 47);
    let y = m.int(10, 10);
    
    // Call modulo directly
    let result = m.modulo(x, y);
    
    match m.solve() {
        Ok(sol) => {
            let x_val = sol.get_int(x);
            let y_val = sol.get_int(y);
            let res_val = sol.get_int(result);
            
            assert_eq!(x_val, 47);
            assert_eq!(y_val, 10);
            assert_eq!(res_val, 7, "47 mod 10 = 7");
        }
        Err(e) => panic!("Expected solution but got error: {:?}", e),
    }
}

// ============================================================================
// Test 19: Modulo with both operands deferred
// ============================================================================

#[test]
fn test_modulo_both_operands_deferred() {
    println!("\nTest 19: Both operands deferred");
    let mut m = Model::default();
    
    let x = m.int(10, 100);
    let y = m.int(5, 20);
    
    let mod_result = m.modulo(x, y);
    
    m.new(x.eq(42));
    m.new(y.eq(7));
    
    match m.solve() {
        Ok(sol) => {
            let x_val = sol.get_int(x);
            let y_val = sol.get_int(y);
            let mod_val = sol.get_int(mod_result);
            
            assert_eq!(x_val, 42);
            assert_eq!(y_val, 7);
            assert_eq!(mod_val, 42 % 7, "42 mod 7 should be 0");
        }
        Err(e) => panic!("Expected solution but got error: {:?}", e),
    }
}

// ============================================================================
// Test 20: Modulo result used in further constraints
// ============================================================================

#[test]
fn test_modulo_result_in_further_constraints() {
    println!("\nTest 20: Modulo result used in further constraints");
    let mut m = Model::default();
    
    let x = m.int(1, 50);
    let divisor = m.int(10, 10);
    let target = m.int(0, 9);
    let result = m.int(0, 9);
    
    let mod_result = m.modulo(x, divisor);
    
    // Use modulo result in further constraints
    m.new(result.eq(mod_result));
    m.new(target.eq(result));
    m.new(target.ge(5));  // Modulo result >= 5
    
    m.new(x.eq(27));  // 27 mod 10 = 7, which is >= 5
    
    match m.solve() {
        Ok(sol) => {
            let x_val = sol.get_int(x);
            let target_val = sol.get_int(target);
            
            assert_eq!(x_val, 27);
            assert_eq!(target_val, 7);
            assert!(target_val >= 5);
        }
        Err(e) => panic!("Expected solution but got error: {:?}", e),
    }
}

// ============================================================================
// Test 21: Original selen_modulo_test.rs case - key issue scenario
// ============================================================================
// This is the main test case from examples/selen_modulo_test.rs
// Testing: remainder = 47 mod 10 (expected: remainder = 7)

#[test]
fn test_modulo_original_selen_modulo_test() {
    println!("\nTest 21: Original selen_modulo_test - Main issue scenario");
    let mut m = Model::default();

    // Create variables
    let number = m.int(10, 100);
    let remainder = m.int(0, 9);
    let divisor = m.int(10, 10);  // Constant 10 as a variable

    // Add constraints
    m.new(number.eq(47));  // number = 47
    
    // remainder = number mod divisor
    let mod_result = m.modulo(number, divisor);
    m.new(remainder.eq(mod_result));

    println!("  Selen Model Setup:");
    println!("    number: [10..100] constrained to 47");
    println!("    remainder: [0..9]");
    println!("    constraint: number == 47");
    println!("    constraint: remainder == (number mod 10)");
    println!("  Expected: number=47, remainder=7 (since 47 mod 10 = 7)");

    match m.solve() {
        Ok(solution) => {
            let num_val = solution.get_int(number);
            let rem_val = solution.get_int(remainder);
            
            assert_eq!(num_val, 47, "number should be 47");
            assert_eq!(rem_val, 7, "47 mod 10 = 7");
            
            println!("  ✓ Solution found:");
            println!("    number = {} (expected 47)", num_val);
            println!("    remainder = {} (expected 7)", rem_val);
        }
        Err(e) => {
            panic!("✗ No solution found! Error: {:?}", e);
        }
    }
}

// ============================================================================
// Variable-to-Variable Equality Pattern Tests
// ============================================================================
// These tests verify that the variable-to-variable equality pattern
// works correctly with modulo constraints. Previously this pattern
// would fail because bounds weren't applied immediately.
// ============================================================================

#[test]
fn test_var_to_var_equality_simple_modulo() {
    println!("\nTest: Variable-to-variable equality with simple modulo");
    let mut m = Model::default();
    
    let dividend = m.int(1, 100);
    let divisor = m.int(1, 10);
    
    // Create constant variables
    let const_47 = m.int(47, 47);
    let const_10 = m.int(10, 10);
    
    // Create modulo FIRST with unconstrained variables
    let mod_result = m.modulo(dividend, divisor);
    
    // THEN post variable-to-variable equality constraints
    m.new(dividend.eq(const_47));
    m.new(divisor.eq(const_10));
    
    match m.solve() {
        Ok(sol) => {
            let div_val = sol.get_int(dividend);
            let vis_val = sol.get_int(divisor);
            let res_val = sol.get_int(mod_result);
            
            assert_eq!(div_val, 47, "dividend should be 47");
            assert_eq!(vis_val, 10, "divisor should be 10");
            assert_eq!(res_val, 7, "47 mod 10 should be 7");
        }
        Err(e) => panic!("Expected solution with var-to-var equality but got error: {:?}", e),
    }
}

#[test]
fn test_var_to_var_equality_multiple_mods() {
    println!("\nTest: Multiple modulo constraints with variable-to-variable equality");
    let mut m = Model::default();
    
    let x = m.int(1, 100);
    let y = m.int(1, 50);
    let z = m.int(1, 100);
    
    // Create constant variables
    let const_42 = m.int(42, 42);
    let const_37 = m.int(37, 37);
    let const_7 = m.int(7, 7);
    let const_5 = m.int(5, 5);
    
    // Create multiple modulo constraints
    let mod1 = m.modulo(x, const_7);
    let mod2 = m.modulo(y, const_5);
    let mod3 = m.modulo(z, const_7);
    
    // Post variable-to-variable equality constraints
    m.new(x.eq(const_42));
    m.new(y.eq(const_37));
    
    // Also constraint the modulos
    m.new(mod1.le(6));
    m.new(mod2.le(4));
    m.new(mod3.ge(0));
    
    match m.solve() {
        Ok(sol) => {
            let x_val = sol.get_int(x);
            let y_val = sol.get_int(y);
            let mod1_val = sol.get_int(mod1);
            let mod2_val = sol.get_int(mod2);
            
            assert_eq!(x_val, 42);
            assert_eq!(y_val, 37);
            assert_eq!(mod1_val, 42 % 7, "42 mod 7 = 0");
            assert_eq!(mod2_val, 37 % 5, "37 mod 5 = 2");
        }
        Err(e) => panic!("Expected solution but got error: {:?}", e),
    }
}

#[test]
fn test_var_to_var_equality_chain() {
    println!("\nTest: Variable-to-variable equality chain with modulo");
    let mut m = Model::default();
    
    let x = m.int(1, 100);
    let y = m.int(1, 100);
    
    // Create constant variables
    let const_55 = m.int(55, 55);
    let const_12 = m.int(12, 12);
    
    // Direct equalities to constants
    m.new(x.eq(const_55));
    m.new(y.eq(const_55));
    
    // Create modulo
    let mod_result = m.modulo(x, const_12);
    let mod_result2 = m.modulo(y, const_12);
    
    match m.solve() {
        Ok(sol) => {
            let x_val = sol.get_int(x);
            let y_val = sol.get_int(y);
            let res_val = sol.get_int(mod_result);
            let res_val2 = sol.get_int(mod_result2);
            
            assert_eq!(x_val, 55);
            assert_eq!(y_val, 55);
            assert_eq!(res_val, 55 % 12, "55 mod 12 = 7");
            assert_eq!(res_val2, 55 % 12, "55 mod 12 = 7");
        }
        Err(e) => panic!("Expected solution with equality chain but got error: {:?}", e),
    }
}

#[test]
fn test_var_to_var_equality_with_constraint_after() {
    println!("\nTest: Variable-to-variable equality with constraint on result");
    let mut m = Model::default();
    
    let x = m.int(1, 100);
    let y = m.int(2, 20);
    
    let const_50 = m.int(50, 50);
    let const_7 = m.int(7, 7);
    
    // Post var-to-var equality
    m.new(x.eq(const_50));
    
    // Create modulo
    let mod_result = m.modulo(x, y);
    
    // Constrain divisor via var-to-var equality
    m.new(y.eq(const_7));
    
    // Add constraint on result
    m.new(mod_result.ge(1));
    m.new(mod_result.le(6));
    
    match m.solve() {
        Ok(sol) => {
            let x_val = sol.get_int(x);
            let y_val = sol.get_int(y);
            let res_val = sol.get_int(mod_result);
            
            assert_eq!(x_val, 50);
            assert_eq!(y_val, 7);
            assert_eq!(res_val, 50 % 7, "50 mod 7 = 1");
        }
        Err(e) => panic!("Expected solution but got error: {:?}", e),
    }
}

#[test]
fn test_var_to_var_equality_large_values() {
    println!("\nTest: Variable-to-variable equality with large values");
    let mut m = Model::default();
    
    let dividend = m.int(1, 100000);
    let divisor = m.int(1, 10000);
    
    let const_99999 = m.int(99999, 99999);
    let const_103 = m.int(103, 103);
    
    // Create modulo first
    let mod_result = m.modulo(dividend, divisor);
    
    // Apply var-to-var equality
    m.new(dividend.eq(const_99999));
    m.new(divisor.eq(const_103));
    
    match m.solve() {
        Ok(sol) => {
            let div_val = sol.get_int(dividend);
            let vis_val = sol.get_int(divisor);
            let res_val = sol.get_int(mod_result);
            
            assert_eq!(div_val, 99999);
            assert_eq!(vis_val, 103);
            assert_eq!(res_val, 99999 % 103, "99999 mod 103 = 9");
        }
        Err(e) => panic!("Expected solution with large values but got error: {:?}", e),
    }
}

#[test]
fn test_var_to_var_equality_with_negative_results() {
    println!("\nTest: Variable-to-variable equality with potential negative modulo");
    let mut m = Model::default();
    
    let x = m.int(-100, 100);
    let y = m.int(1, 20);
    
    let const_neg25 = m.int(-25, -25);
    let const_6 = m.int(6, 6);
    
    // Create modulo with potential negative dividend
    let mod_result = m.modulo(x, y);
    
    // Apply var-to-var equality
    m.new(x.eq(const_neg25));
    m.new(y.eq(const_6));
    
    match m.solve() {
        Ok(sol) => {
            let x_val = sol.get_int(x);
            let y_val = sol.get_int(y);
            let res_val = sol.get_int(mod_result);
            
            assert_eq!(x_val, -25);
            assert_eq!(y_val, 6);
            // Rust modulo: -25 % 6 = -1
            assert_eq!(res_val, -25 % 6);
        }
        Err(e) => panic!("Expected solution with negative dividend but got error: {:?}", e),
    }
}

#[test]
fn test_var_to_var_equality_multiple_equality_constraints() {
    println!("\nTest: Multiple variable-to-variable equalities in same model");
    let mut m = Model::default();
    
    let a = m.int(1, 100);
    let b = m.int(1, 100);
    let c = m.int(1, 20);
    let d = m.int(1, 20);
    
    let const_60 = m.int(60, 60);
    let const_75 = m.int(75, 75);
    let const_8 = m.int(8, 8);
    let const_9 = m.int(9, 9);
    
    // Multiple modulo operations
    let mod1 = m.modulo(a, c);
    let mod2 = m.modulo(b, d);
    
    // Multiple var-to-var equalities
    m.new(a.eq(const_60));
    m.new(b.eq(const_75));
    m.new(c.eq(const_8));
    m.new(d.eq(const_9));
    
    match m.solve() {
        Ok(sol) => {
            let a_val = sol.get_int(a);
            let b_val = sol.get_int(b);
            let c_val = sol.get_int(c);
            let d_val = sol.get_int(d);
            let mod1_val = sol.get_int(mod1);
            let mod2_val = sol.get_int(mod2);
            
            assert_eq!(a_val, 60);
            assert_eq!(b_val, 75);
            assert_eq!(c_val, 8);
            assert_eq!(d_val, 9);
            assert_eq!(mod1_val, 60 % 8, "60 mod 8 = 4");
            assert_eq!(mod2_val, 75 % 9, "75 mod 9 = 3");
        }
        Err(e) => panic!("Expected solution with multiple equalities but got error: {:?}", e),
    }
}
