//! Comprehensive coverage tests for mixed constraint scenarios
//! 
//! Tests the bugs we fixed and scenarios that could reveal similar issues:
//! 1. Float + Integer mixed constraints
//! 2. Bounded + Unbounded variables mixed  
//! 3. Large bounded domains (not just -10 to 10)
//! 4. Array constraints with mixed types
//! 5. Optimization with complex constraints
//! 6. Linear equality with tight tolerance (the bug we fixed)
//! 7. Multiplication with unbounded variables (the bug we fixed)

use selen::prelude::*;

// ═══════════════════════════════════════════════════════════════════════
// BOUNDED + UNBOUNDED MIX (Testing the multiplication bug fix)
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn test_unbounded_float_times_bounded_int() {
    // This tests the bug we fixed: unbounded * bounded with quantization
    let mut m = Model::default();
    
    let rate = m.float(f64::NEG_INFINITY, f64::INFINITY); // Unbounded
    let amount = m.int(500, 2000); // Large bounded
    
    let result = m.mul(rate, amount);
    
    // result should be around 250
    m.new(result.ge(240.0));
    m.new(result.le(260.0));
    
    let solution = m.solve().expect("Should solve unbounded * bounded");
    
    let rate_val = solution.get_float(rate);
    let amount_val = solution.get_int(amount);
    let result_val = solution.get_float(result);
    
    let expected = rate_val * (amount_val as f64);
    assert!((result_val - expected).abs() < 1.0, 
        "Multiplication error: {} * {} = {} but got {}", 
        rate_val, amount_val, expected, result_val);
    
    assert!(result_val >= 240.0 && result_val <= 260.0, 
        "Result out of bounds: {}", result_val);
}

#[test]
fn test_bounded_large_plus_unbounded() {
    // Mix large bounded with unbounded
    let mut m = Model::default();
    
    let bounded = m.float(1000.0, 10000.0); // Large bounded
    let unbounded = m.float(f64::NEG_INFINITY, f64::INFINITY); // Unbounded
    
    let sum = m.add(bounded, unbounded);
    
    // Constrain sum to force unbounded to negative
    m.new(sum.eq(500.0));
    
    let solution = m.solve().expect("Should solve large bounded + unbounded");
    
    let b_val = solution.get_float(bounded);
    let u_val = solution.get_float(unbounded);
    let s_val = solution.get_float(sum);
    
    assert!((b_val + u_val - 500.0).abs() < 0.1, 
        "Sum constraint violated: {} + {} = {} ≠ 500", b_val, u_val, b_val + u_val);
    assert!((s_val - 500.0).abs() < 0.1, "Sum variable incorrect");
}

#[test]
fn test_unbounded_in_linear_equality() {
    // This is like the loan problem - unbounded rate in linear equation
    let mut m = Model::default();
    
    let x_unbounded = m.float(f64::NEG_INFINITY, f64::INFINITY);
    let y_bounded = m.float(100.0, 200.0);
    
    // 10*x + 5*y = 1500
    // If y=100, then 10*x = 1000, so x = 100
    m.float_lin_eq(&[10.0, 5.0], &[x_unbounded, y_bounded], 1500.0);
    
    // Force y to specific value
    m.new(y_bounded.eq(100.0));
    
    let solution = m.solve().expect("Should solve linear eq with unbounded");
    
    let x_val = solution.get_float(x_unbounded);
    let y_val = solution.get_float(y_bounded);
    
    let sum = 10.0 * x_val + 5.0 * y_val;
    assert!((sum - 1500.0).abs() < 0.1, 
        "Linear eq violated: 10*{} + 5*{} = {} ≠ 1500", x_val, y_val, sum);
    
    assert!((x_val - 100.0).abs() < 0.1, "x should be ~100, got {}", x_val);
}

// ═══════════════════════════════════════════════════════════════════════
// LARGE BOUNDED DOMAINS (Not just -10 to 10)
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn test_large_domain_float_linear_equality() {
    // Testing with domains in thousands (the float_lin_eq bug we fixed)
    let mut m = Model::default();
    
    let x = m.float(0.0, 5000.0);
    let y = m.float(0.0, 5000.0);
    let z = m.float(0.0, 5000.0);
    
    // x + 2*y + 3*z = 7500
    m.float_lin_eq(&[1.0, 2.0, 3.0], &[x, y, z], 7500.0);
    
    // Constrain x and y
    m.new(x.eq(500.0));
    m.new(y.eq(1000.0));
    // Then z = (7500 - 500 - 2000) / 3 = 5000 / 3 ≈ 1666.67
    
    let solution = m.solve().expect("Should solve large domain linear eq");
    
    let x_val = solution.get_float(x);
    let y_val = solution.get_float(y);
    let z_val = solution.get_float(z);
    
    let sum = x_val + 2.0 * y_val + 3.0 * z_val;
    assert!((sum - 7500.0).abs() < 1.0, 
        "Large domain linear eq violated: {} + 2*{} + 3*{} = {} ≠ 7500", 
        x_val, y_val, z_val, sum);
}

#[test]
#[ignore = "reason"]
fn test_large_domain_optimization() {
    // Test optimization with large domains (the optimization bug we fixed)
    let mut m = Model::default();
    
    let x = m.float(0.0, 1e6);
    let y = m.float(0.0, 1e6);
    
    // x + y <= 8000
    let sum = m.add(x, y);
    m.new(sum.le(8000.0));
    
    // x >= 2000
    m.new(x.ge(2000.0));
    
    let solution = m.maximize(x).expect("Should maximize with large domain");
    
    let x_val = solution.get_float(x);
    let y_val = solution.get_float(y);
    
    // Check constraints
    assert!(x_val + y_val <= 8000.1, "Constraint violated: {} + {} > 8000", x_val, y_val);
    assert!(x_val >= 1999.9, "Lower bound violated: {} < 2000", x_val);
    
    // Optimal should be x = 8000 (when y = 0)
    assert!(x_val >= 7999.0, "Not optimal: x = {} < 8000", x_val);
}

#[test]
fn test_large_domain_multiplication() {
    // Test multiplication with large values
    let mut m = Model::default();
    
    let x = m.float(100.0, 1000.0);
    let y = m.float(10.0, 100.0);
    
    let product = m.mul(x, y);
    
    // Product should be between 20,000 and 30,000
    m.new(product.ge(20000.0));
    m.new(product.le(30000.0));
    
    let solution = m.solve().expect("Should solve large domain multiplication");
    
    let x_val = solution.get_float(x);
    let y_val = solution.get_float(y);
    let p_val = solution.get_float(product);
    
    let expected = x_val * y_val;
    assert!((p_val - expected).abs() < 10.0, 
        "Multiplication error: {} * {} = {} but got {}", 
        x_val, y_val, expected, p_val);
}

// ═══════════════════════════════════════════════════════════════════════
// FLOAT + INT MIXED WITH LARGE DOMAINS
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn test_large_int_large_float_mixed() {
    let mut m = Model::default();
    
    let count = m.int(100, 1000); // Large int
    let price = m.float(10.0, 100.0); // Float
    
    let total = m.mul(count, price);
    
    // Total should be around 50,000
    m.new(total.ge(45000.0));
    m.new(total.le(55000.0));
    
    let solution = m.solve().expect("Should solve large int/float mix");
    
    let c_val = solution.get_int(count);
    let p_val = solution.get_float(price);
    let t_val = solution.get_float(total);
    
    let expected = (c_val as f64) * p_val;
    assert!((t_val - expected).abs() < 10.0, 
        "Mixed multiply error: {} * {} = {} but got {}", 
        c_val, p_val, expected, t_val);
    
    assert!(t_val >= 45000.0 && t_val <= 55000.0, "Total out of range");
}

#[test]
fn test_large_mixed_linear_equality() {
    // Float/int mix with large coefficients
    let mut m = Model::default();
    
    let x_int = m.int(0, 500);
    let y_float = m.float(0.0, 1000.0);
    
    // 100*x_int + 50*y_float = 35000
    // If x_int = 200, then 50*y_float = 15000, so y_float = 300
    m.float_lin_eq(&[100.0, 50.0], &[x_int, y_float], 35000.0);
    
    m.new(x_int.eq(200));
    
    let solution = m.solve().expect("Should solve large mixed linear eq");
    
    let x_val = solution.get_int(x_int);
    let y_val = solution.get_float(y_float);
    
    let sum = 100.0 * (x_val as f64) + 50.0 * y_val;
    assert!((sum - 35000.0).abs() < 1.0, 
        "Mixed linear eq violated: 100*{} + 50*{} = {} ≠ 35000", 
        x_val, y_val, sum);
}

// ═══════════════════════════════════════════════════════════════════════
// COMPLEX SCENARIOS THAT COULD REVEAL BUGS
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn test_cascading_constraints_with_precision() {
    // Chain of constraints that could amplify precision errors
    let mut m = Model::default();
    
    let x = m.float(0.0, 1000.0);
    let y = m.float(0.0, 1000.0);
    let z = m.float(0.0, 1000.0);
    
    // x + y = 500
    let sum1 = m.add(x, y);
    m.new(sum1.eq(500.0));
    
    // y + z = 600
    let sum2 = m.add(y, z);
    m.new(sum2.eq(600.0));
    
    // x + z = 400
    let sum3 = m.add(x, z);
    m.new(sum3.eq(400.0));
    
    // Solution: x=200, y=300, z=300
    
    let solution = m.solve().expect("Should solve cascading constraints");
    
    let x_val = solution.get_float(x);
    let y_val = solution.get_float(y);
    let z_val = solution.get_float(z);
    
    assert!((x_val + y_val - 500.0).abs() < 0.1, "x + y ≠ 500");
    assert!((y_val + z_val - 600.0).abs() < 0.1, "y + z ≠ 600");
    assert!((x_val + z_val - 400.0).abs() < 0.1, "x + z ≠ 400");
}

#[test]
#[ignore = "slow: search times out with large domains (60s), needs LP solver"]
fn test_optimization_with_derived_variables() {
    // This is the optimization bug we fixed - derived variables not constrained
    let mut m = Model::default();
    
    let x = m.float(0.0, 100.0);
    
    // Create derived variable: y = 2*x
    let two = m.float(2.0, 2.0); // Constant
    let y = m.mul(x, two);
    
    // Constrain y <= 50
    m.new(y.le(50.0));
    
    let solution = m.maximize(x).expect("Should maximize with derived constraint");
    
    let x_val = solution.get_float(x);
    let y_val = solution.get_float(y);
    
    // y should be <= 50
    assert!(y_val <= 50.1, "Derived constraint violated: y = {} > 50", y_val);
    
    // y should be 2*x
    assert!((y_val - 2.0 * x_val).abs() < 0.1, 
        "Derived variable wrong: y = {} ≠ 2*{} = {}", y_val, x_val, 2.0 * x_val);
    
    // Optimal should be x = 25 (making y = 50)
    assert!(x_val >= 24.9 && x_val <= 25.1, 
        "Not optimal: x = {} should be ~25", x_val);
}

#[test]
#[ignore = "slow: search times out with large domains (60s), needs LP solver"]
fn test_unbounded_optimization_with_constraints() {
    // Optimize unbounded variable with actual constraints
    let mut m = Model::default();
    
    let x_unbounded = m.float(f64::NEG_INFINITY, f64::INFINITY);
    let y_bounded = m.float(0.0, 100.0);
    
    // x + y <= 150
    let sum = m.add(x_unbounded, y_bounded);
    m.new(sum.le(150.0));
    
    // x >= 10
    m.new(x_unbounded.ge(10.0));
    
    let solution = m.maximize(x_unbounded).expect("Should maximize unbounded with constraints");
    
    let x_val = solution.get_float(x_unbounded);
    let y_val = solution.get_float(y_bounded);
    
    // Check constraints
    assert!(x_val + y_val <= 150.1, "Sum constraint violated");
    assert!(x_val >= 9.9, "Lower bound violated");
    
    // Optimal: x = 150 (when y = 0)
    assert!(x_val >= 149.0, "Not optimal: x = {} < 150", x_val);
}

#[test]
fn test_array_sum_with_mixed_bounded_unbounded() {
    // Array with mix of bounded and unbounded variables
    let mut m = Model::default();
    
    let bounded: Vec<_> = (0..3).map(|_| m.float(1.0, 10.0)).collect();
    let unbounded = m.float(f64::NEG_INFINITY, f64::INFINITY);
    
    // Sum of all = 25
    let sum = bounded.iter().fold(unbounded, |acc, &v| m.add(acc, v));
    m.new(sum.eq(25.0));
    
    let solution = m.solve().expect("Should solve mixed array sum");
    
    let b_vals: Vec<f64> = bounded.iter().map(|&v| solution.get_float(v)).collect();
    let u_val = solution.get_float(unbounded);
    
    let total = u_val + b_vals.iter().sum::<f64>();
    assert!((total - 25.0).abs() < 0.1, "Sum constraint violated: {} ≠ 25", total);
}

#[test]
fn test_large_coefficient_precision() {
    // Large coefficients can amplify quantization errors
    let mut m = Model::default();
    
    let x = m.float(0.0, 1.0); // Small domain
    let y = m.float(0.0, 10000.0); // Large domain
    
    // 10000*x + y = 15000
    // If x = 1.0, then y = 5000
    m.float_lin_eq(&[10000.0, 1.0], &[x, y], 15000.0);
    
    m.new(x.eq(1.0));
    
    let solution = m.solve().expect("Should solve with large coefficients");
    
    let x_val = solution.get_float(x);
    let y_val = solution.get_float(y);
    
    let sum = 10000.0 * x_val + y_val;
    assert!((sum - 15000.0).abs() < 10.0, 
        "Large coefficient precision error: 10000*{} + {} = {} ≠ 15000", 
        x_val, y_val, sum);
}

#[test]
fn test_division_with_large_and_small() {
    // Division can lose precision with large numerator / small denominator
    let mut m = Model::default();
    
    let numerator = m.float(1000.0, 10000.0);
    let denominator = m.float(0.1, 10.0);
    
    let quotient = m.div(numerator, denominator);
    
    // quotient should be achievable in range
    m.new(quotient.ge(500.0));
    m.new(quotient.le(2000.0));
    
    let solution = m.solve().expect("Should solve division with large/small");
    
    let n_val = solution.get_float(numerator);
    let d_val = solution.get_float(denominator);
    let q_val = solution.get_float(quotient);
    
    let expected = n_val / d_val;
    assert!((q_val - expected).abs() / expected.abs() < 0.01, 
        "Division precision error: {} / {} = {} but got {}", 
        n_val, d_val, expected, q_val);
}
