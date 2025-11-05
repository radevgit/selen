/// Tests for combinations of two or more constraints
/// 
/// This test suite explores interactions between different constraint types,
/// as bugs can emerge from unexpected constraint interactions.
/// 
/// Focus areas:
/// - alldiff + modulo
/// - alldiff + linear equality
/// - modulo + linear inequality
/// - Multiple modulo constraints
/// - alldiff + neq patterns

use selen::prelude::*;

// ============================================================================
// ALLDIFF + MODULO COMBINATIONS
// ============================================================================

#[test]
fn test_alldiff_modulo_simple() {
    // Variables must all be different AND all have same remainder when divided by 3
    let mut model = Model::default();

    let x1 = model.int(1, 10);
    let x2 = model.int(1, 10);
    let x3 = model.int(1, 10);
    let three = model.int(3, 3);

    // All different
    model.alldiff(&[x1, x2, x3]);

    // All have remainder 1 when divided by 3
    let x1_mod = model.modulo(x1, three);
    let x2_mod = model.modulo(x2, three);
    let x3_mod = model.modulo(x3, three);
    let one = model.int(1, 1);
    
    model.alleq(&[x1_mod, x2_mod, x3_mod, one]);

    match model.solve() {
        Ok(solution) => {
            let v1 = solution.get_int(x1);
            let v2 = solution.get_int(x2);
            let v3 = solution.get_int(x3);

            // Check alldiff
            assert_ne!(v1, v2, "x1 and x2 should be different");
            assert_ne!(v2, v3, "x2 and x3 should be different");
            assert_ne!(v1, v3, "x1 and x3 should be different");

            // Check modulo - all should have remainder 1
            assert_eq!(v1 % 3, 1, "x1 % 3 should be 1");
            assert_eq!(v2 % 3, 1, "x2 % 3 should be 1");
            assert_eq!(v3 % 3, 1, "x3 % 3 should be 1");

            // Valid values: 1, 4, 7, 10
            assert!(vec![1, 4, 7, 10].contains(&v1));
            assert!(vec![1, 4, 7, 10].contains(&v2));
            assert!(vec![1, 4, 7, 10].contains(&v3));
        }
        Err(e) => panic!("Expected solution but got error: {:?}", e),
    }
}

#[test]
fn test_alldiff_modulo_different_divisors() {
    // alldiff with modulo constraints using different divisors
    let mut model = Model::default();

    let x1 = model.int(1, 20);
    let x2 = model.int(1, 20);
    let x3 = model.int(1, 20);

    // All different
    model.alldiff(&[x1, x2, x3]);

    // x1 % 5 = 2
    let five = model.int(5, 5);
    let two = model.int(2, 2);
    let x1_mod = model.modulo(x1, five);
    model.new(x1_mod.eq(two));

    // x2 % 5 = 3
    let three = model.int(3, 3);
    let x2_mod = model.modulo(x2, five);
    model.new(x2_mod.eq(three));

    // x3 % 5 = 4
    let four = model.int(4, 4);
    let x3_mod = model.modulo(x3, five);
    model.new(x3_mod.eq(four));

    match model.solve() {
        Ok(solution) => {
            let v1 = solution.get_int(x1);
            let v2 = solution.get_int(x2);
            let v3 = solution.get_int(x3);

            assert_ne!(v1, v2);
            assert_ne!(v2, v3);
            assert_ne!(v1, v3);
            assert_eq!(v1 % 5, 2);
            assert_eq!(v2 % 5, 3);
            assert_eq!(v3 % 5, 4);
        }
        Err(e) => panic!("Expected solution but got error: {:?}", e),
    }
}

#[test]
fn test_alldiff_modulo_complex_domain() {
    // alldiff + modulo with complex domain ranges
    let mut model = Model::default();

    let vars: Vec<_> = (0..4).map(|_| model.int(1, 30)).collect();

    // All different
    model.alldiff(&vars);

    // All have remainder 3 when divided by 7
    let seven = model.int(7, 7);
    let three = model.int(3, 3);

    for &var in &vars {
        let var_mod = model.modulo(var, seven);
        model.new(var_mod.eq(three));
    }

    match model.solve() {
        Ok(solution) => {
            let values: Vec<_> = vars.iter().map(|&v| solution.get_int(v)).collect();

            // Check all different
            for i in 0..values.len() {
                for j in (i + 1)..values.len() {
                    assert_ne!(values[i], values[j]);
                }
            }

            // Check modulo: valid values are 3, 10, 17, 24
            for val in &values {
                assert_eq!(val % 7, 3);
                assert!(vec![3, 10, 17, 24].contains(val), "Value {} should be in [3, 10, 17, 24]", val);
            }
        }
        Err(e) => panic!("Expected solution but got error: {:?}", e),
    }
}

// ============================================================================
// ALLDIFF + LINEAR CONSTRAINTS
// ============================================================================

#[test]
fn test_alldiff_with_linear_sum() {
    // alldiff + sum constraint
    let mut model = Model::default();

    let x1 = model.int(1, 5);
    let x2 = model.int(1, 5);
    let x3 = model.int(1, 5);

    model.alldiff(&[x1, x2, x3]);

    // Sum should be exactly 9 (1+2+3 or similar)
    let sum = x1.add(x2).add(x3);
    let nine = model.int(9, 9);
    model.new(sum.eq(nine));

    match model.solve() {
        Ok(solution) => {
            let v1 = solution.get_int(x1);
            let v2 = solution.get_int(x2);
            let v3 = solution.get_int(x3);

            // Check alldiff
            assert_ne!(v1, v2);
            assert_ne!(v2, v3);
            assert_ne!(v1, v3);

            // Check sum
            assert_eq!(v1 + v2 + v3, 9);
        }
        Err(e) => panic!("Expected solution but got error: {:?}", e),
    }
}

#[test]
fn test_alldiff_with_linear_inequality() {
    // alldiff + inequality constraints
    let mut model = Model::default();

    let x1 = model.int(1, 10);
    let x2 = model.int(1, 10);
    let x3 = model.int(1, 10);

    model.alldiff(&[x1, x2, x3]);

    // Sum should be at least 15
    let sum = x1.add(x2).add(x3);
    let fifteen = model.int(15, 15);
    model.new(sum.ge(fifteen));

    match model.solve() {
        Ok(solution) => {
            let v1 = solution.get_int(x1);
            let v2 = solution.get_int(x2);
            let v3 = solution.get_int(x3);

            // Check alldiff
            assert_ne!(v1, v2);
            assert_ne!(v2, v3);
            assert_ne!(v1, v3);

            // Check sum constraint
            assert!(v1 + v2 + v3 >= 15);
        }
        Err(e) => panic!("Expected solution but got error: {:?}", e),
    }
}

// ============================================================================
// MODULO + LINEAR COMBINATIONS
// ============================================================================

#[test]
fn test_modulo_linear_basic() {
    // Modulo result used in linear constraint
    let mut model = Model::default();

    let x = model.int(1, 20);
    let five = model.int(5, 5);

    let x_mod = model.modulo(x, five);

    // Create linear constraint: x_mod + 2*x <= 40
    model.new(x_mod.add(x.mul(2)).le(40));

    match model.solve() {
        Ok(solution) => {
            let x_val = solution.get_int(x);
            let x_mod_val = solution.get_int(x_mod);

            assert!(x_val >= 1 && x_val <= 20);
            assert_eq!(x_mod_val, x_val % 5);
            assert!(x_mod_val + 2 * x_val <= 40);
        }
        Err(e) => panic!("Expected solution but got error: {:?}", e),
    }
}

#[test]
fn test_modulo_linear_sum() {
    // Sum of modulo results in linear constraint
    let mut model = Model::default();

    let x1 = model.int(1, 15);
    let x2 = model.int(1, 15);
    let three = model.int(3, 3);

    let x1_mod = model.modulo(x1, three);
    let x2_mod = model.modulo(x2, three);

    // Sum of modulos should equal 2
    let x1_mod_val = x1_mod.add(x2_mod);
    let two = model.int(2, 2);
    model.new(x1_mod_val.eq(two));

    match model.solve() {
        Ok(solution) => {
            let _x1_val = solution.get_int(x1);
            let _x2_val = solution.get_int(x2);
            let x1_mod_result = solution.get_int(x1_mod);
            let x2_mod_result = solution.get_int(x2_mod);

            let sum = x1_mod_result + x2_mod_result;
            assert_eq!(sum, 2);
        }
        Err(e) => panic!("Expected solution but got error: {:?}", e),
    }
}

// ============================================================================
// MULTIPLE MODULO CONSTRAINTS
// ============================================================================

#[test]
fn test_multiple_modulo_cascading() {
    // Multiple modulo operations on same variable
    let mut model = Model::default();

    let x = model.int(1, 30);
    let seven = model.int(7, 7);
    let three = model.int(3, 3);

    let x_mod_7 = model.modulo(x, seven);
    let x_mod_3 = model.modulo(x, three);

    // x % 7 = 2
    let two = model.int(2, 2);
    model.new(x_mod_7.eq(two));

    // x % 3 = 1
    let one = model.int(1, 1);
    model.new(x_mod_3.eq(one));

    match model.solve() {
        Ok(solution) => {
            let x_val = solution.get_int(x);

            assert_eq!(x_val % 7, 2);
            assert_eq!(x_val % 3, 1);

            // Valid values: 16 (16 % 7 = 2, 16 % 3 = 1)
            assert!(x_val == 16 || x_val == 23 || x_val == 30);  // Solutions within domain
        }
        Err(e) => panic!("Expected solution but got error: {:?}", e),
    }
}

#[test]
fn test_modulo_chain_with_alldiff() {
    // Chain: modulo results used in alldiff
    let mut model = Model::default();

    let x1 = model.int(2, 20);
    let x2 = model.int(2, 20);
    let x3 = model.int(2, 20);
    let four = model.int(4, 4);

    model.alldiff(&[x1, x2, x3]);

    let x1_mod = model.modulo(x1, four);
    let x2_mod = model.modulo(x2, four);
    let x3_mod = model.modulo(x3, four);

    // The modulo results must also be different
    model.alldiff(&[x1_mod, x2_mod, x3_mod]);

    match model.solve() {
        Ok(solution) => {
            let v1 = solution.get_int(x1);
            let v2 = solution.get_int(x2);
            let v3 = solution.get_int(x3);

            let m1 = solution.get_int(x1_mod);
            let m2 = solution.get_int(x2_mod);
            let m3 = solution.get_int(x3_mod);

            // Original values all different
            assert_ne!(v1, v2);
            assert_ne!(v2, v3);
            assert_ne!(v1, v3);

            // Modulo results all different
            assert_ne!(m1, m2);
            assert_ne!(m2, m3);
            assert_ne!(m1, m3);

            // Verify modulo calculations
            assert_eq!(m1, v1 % 4);
            assert_eq!(m2, v2 % 4);
            assert_eq!(m3, v3 % 4);
        }
        Err(e) => panic!("Expected solution but got error: {:?}", e),
    }
}

// ============================================================================
// COMPLEX MULTI-CONSTRAINT SCENARIOS
// ============================================================================

#[test]
fn test_alldiff_modulo_linear_combined() {
    // Three constraints: alldiff + modulo + linear
    let mut model = Model::default();

    let x1 = model.int(1, 15);
    let x2 = model.int(1, 15);
    let x3 = model.int(1, 15);

    // Constraint 1: All different
    model.alldiff(&[x1, x2, x3]);

    // Constraint 2: All even (remainder 0)
    let two = model.int(2, 2);
    let zero = model.int(0, 0);
    let x1_mod = model.modulo(x1, two);
    model.new(x1_mod.eq(zero));
    let x2_mod = model.modulo(x2, two);
    model.new(x2_mod.eq(zero));
    let x3_mod = model.modulo(x3, two);
    model.new(x3_mod.eq(zero));

    // Constraint 3: Sum is at least 20
    let x1_x2 = x1.add(x2);
    let x1_x2_x3 = x1_x2.add(x3);
    let twenty = model.int(20, 20);
    model.new(x1_x2_x3.ge(twenty));

    match model.solve() {
        Ok(solution) => {
            let v1 = solution.get_int(x1);
            let v2 = solution.get_int(x2);
            let v3 = solution.get_int(x3);

            // All different
            assert_ne!(v1, v2);
            assert_ne!(v2, v3);
            assert_ne!(v1, v3);

            // All even
            assert_eq!(v1 % 2, 0);
            assert_eq!(v2 % 2, 0);
            assert_eq!(v3 % 2, 0);

            // Sum >= 20
            assert!(v1 + v2 + v3 >= 20);
        }
        Err(e) => panic!("Expected solution but got error: {:?}", e),
    }
}

#[test]
fn test_neq_with_modulo() {
    // NEQ constraints with modulo
    let mut model = Model::default();

    let x1 = model.int(1, 10);
    let x2 = model.int(1, 10);
    let three = model.int(3, 3);

    // x1 != x2
    model.new(x1.ne(x2));

    // x1 % 3 != 0 (not divisible by 3)
    let zero = model.int(0, 0);
    let x1_mod = model.modulo(x1, three);
    model.new(x1_mod.ne(zero));

    // x2 % 3 != 1
    let one = model.int(1, 1);
    let x2_mod = model.modulo(x2, three);
    model.new(x2_mod.ne(one));

    match model.solve() {
        Ok(solution) => {
            let v1 = solution.get_int(x1);
            let v2 = solution.get_int(x2);

            assert_ne!(v1, v2);
            assert_ne!(v1 % 3, 0);
            assert_ne!(v2 % 3, 1);
        }
        Err(e) => panic!("Expected solution but got error: {:?}", e),
    }
}

#[test]
fn test_modulo_ordering_constraints() {
    // Modulo combined with ordering
    let mut model = Model::default();

    let x = model.int(5, 30);
    let y = model.int(5, 30);
    let five = model.int(5, 5);

    let x_mod = model.modulo(x, five);
    let y_mod = model.modulo(y, five);

    // x > y (original values)
    model.new(x.gt(y));

    // x % 5 < y % 5 (modulo values inverted)
    model.new(x_mod.lt(y_mod));

    match model.solve() {
        Ok(solution) => {
            let x_val = solution.get_int(x);
            let y_val = solution.get_int(y);
            let x_mod_val = solution.get_int(x_mod);
            let y_mod_val = solution.get_int(y_mod);

            assert!(x_val > y_val);
            assert!(x_mod_val < y_mod_val);
        }
        Err(e) => panic!("Expected solution but got error: {:?}", e),
    }
}
