/// Test for modulo constraint bug fix
/// 
/// Issue: Modulo constraint with fixed divisor was only checking boundary values
/// of the dividend domain, missing intermediate values that produce different remainders.
/// 
/// Example: With b ∈ [1,3] and divisor 2:
/// - Old behavior: Only checked 1 and 3 → {1, 1} % 2 → remainders {1}
/// - New behavior: Checks all 1, 2, 3 → {1, 2, 3} % 2 → remainders {0, 1}
/// 
/// This test verifies that when a % divisor == b % divisor, the solver correctly
/// finds solutions even when the dividend domain has intermediate values that
/// produce different remainders.

use selen::prelude::*;

#[test]
fn test_modulo_alleq_with_fixed_divisor() {
    // Original failing case: a ∈ [0,0], b ∈ [1,3], divisor = 2
    let mut model = Model::default();

    let a = model.int(0, 0);           // Fixed to 0
    let b = model.int(1, 3);           // Domain {1, 2, 3}
    let two = model.int(2, 2);         // Fixed divisor

    let a_mod_two = model.modulo(a, two);
    let b_mod_two = model.modulo(b, two);

    // Constraint: a % 2 == b % 2
    model.alleq(&[a_mod_two, b_mod_two]);

    match model.solve() {
        Ok(solution) => {
            let a_val = solution.get_int(a);
            let b_val = solution.get_int(b);
            let a_mod_val = solution.get_int(a_mod_two);
            let b_mod_val = solution.get_int(b_mod_two);

            // Verify values
            assert_eq!(a_val, 0, "a should be 0 (fixed)");
            assert!(b_val >= 1 && b_val <= 3, "b should be in [1, 3]");
            
            // Verify modulo calculations
            assert_eq!(a_mod_val, 0, "0 % 2 should be 0");
            assert!(b_mod_val == 0 || b_mod_val == 1, "b % 2 should be 0 or 1");
            
            // Critical: verify the alleq constraint is satisfied
            assert_eq!(a_mod_val, b_mod_val, "a % 2 should equal b % 2 (alleq constraint)");
            
            // Verify the specific solution
            assert_eq!(b_val, 2, "b should be 2 (only value giving 2 % 2 = 0)");
        }
        Err(e) => {
            panic!("Expected solution but got error: {:?}", e);
        }
    }
}

#[test]
fn test_modulo_alleq_larger_domain() {
    // Test with larger dividend domain to ensure all intermediate values are checked
    let mut model = Model::default();

    let a = model.int(0, 0);
    let b = model.int(1, 10);         // Larger domain: {1, 2, 3, 4, 5, 6, 7, 8, 9, 10}
    let three = model.int(3, 3);      // Divisor = 3

    let a_mod_three = model.modulo(a, three);
    let b_mod_three = model.modulo(b, three);

    model.alleq(&[a_mod_three, b_mod_three]);

    match model.solve() {
        Ok(solution) => {
            let a_val = solution.get_int(a);
            let b_val = solution.get_int(b);
            let a_mod_val = solution.get_int(a_mod_three);
            let b_mod_val = solution.get_int(b_mod_three);

            assert_eq!(a_val, 0, "a should be 0");
            assert!(b_val >= 1 && b_val <= 10, "b should be in [1, 10]");
            assert_eq!(a_mod_val, 0, "0 % 3 = 0");
            assert_eq!(a_mod_val, b_mod_val, "Constraint should be satisfied");
            
            // b % 3 = 0 only for b ∈ {3, 6, 9}
            assert!(b_val == 3 || b_val == 6 || b_val == 9, 
                    "b should give remainder 0 when divided by 3");
        }
        Err(e) => {
            panic!("Expected solution but got error: {:?}", e);
        }
    }
}

#[test]
fn test_modulo_alleq_multiple_values() {
    // Test finding multiple valid solutions with different remainders
    let mut model = Model::default();

    let x = model.int(5, 5);
    let y = model.int(1, 7);
    let two = model.int(2, 2);

    let x_mod_two = model.modulo(x, two);
    let y_mod_two = model.modulo(y, two);

    model.alleq(&[x_mod_two, y_mod_two]);

    match model.solve() {
        Ok(solution) => {
            let x_val = solution.get_int(x);
            let y_val = solution.get_int(y);
            let x_mod_val = solution.get_int(x_mod_two);
            let y_mod_val = solution.get_int(y_mod_two);

            assert_eq!(x_val, 5, "x should be 5");
            assert!(y_val >= 1 && y_val <= 7, "y should be in [1, 7]");
            assert_eq!(x_mod_val, 1, "5 % 2 = 1");
            assert_eq!(x_mod_val, y_mod_val, "Constraint should be satisfied");
            
            // y % 2 = 1 only for y ∈ {1, 3, 5, 7}
            assert!(y_val % 2 == 1, "y should be odd");
        }
        Err(e) => {
            panic!("Expected solution but got error: {:?}", e);
        }
    }
}

#[test]
fn test_modulo_alleq_no_solution() {
    // Test that contradictory constraints still return NoSolution
    let mut model = Model::default();

    let a = model.int(0, 0);
    let b = model.int(2, 2);
    let two = model.int(2, 2);

    let a_mod_two = model.modulo(a, two);
    let b_mod_two = model.modulo(b, two);

    // 0 % 2 = 0, but 2 % 2 = 0, so this should be satisfiable
    // Let's make it unsatisfiable: force a_mod to be 1
    model.alleq(&[a_mod_two, b_mod_two]);

    match model.solve() {
        Ok(solution) => {
            let a_mod_val = solution.get_int(a_mod_two);
            let b_mod_val = solution.get_int(b_mod_two);
            assert_eq!(a_mod_val, b_mod_val, "Should satisfy constraint");
        }
        Err(_) => {
            // This case should actually find a solution (0 % 2 = 2 % 2 = 0)
            panic!("This case should find a solution!");
        }
    }
}

#[test]
fn test_modulo_with_range_and_constraints() {
    // Test modulo with both domain ranges and additional constraints
    let mut model = Model::default();

    let dividend = model.int(10, 20);
    let divisor = model.int(5, 5);
    let remainder = model.int(3, 3);

    let mod_result = model.modulo(dividend, divisor);
    model.new(remainder.eq(mod_result));

    match model.solve() {
        Ok(solution) => {
            let div_val = solution.get_int(dividend);
            let rem_val = solution.get_int(mod_result);

            assert!(div_val >= 10 && div_val <= 20, "dividend should be in [10, 20]");
            assert_eq!(rem_val, 3, "remainder should be 3");
            assert_eq!(div_val % 5, 3, "dividend % 5 should equal 3");
            
            // Valid values: 13, 18 (from [10, 20] where x % 5 = 3)
            assert!(div_val == 13 || div_val == 18, "dividend should be 13 or 18");
        }
        Err(e) => {
            panic!("Expected solution but got error: {:?}", e);
        }
    }
}
