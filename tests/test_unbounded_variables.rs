/// Test suite for unbounded variable bound inference
/// 
/// Tests the automatic inference of reasonable bounds for variables declared with
/// unbounded or infinite bounds (i32::MIN/MAX, f64::INFINITY).

use selen::prelude::*;

// ============================================================================
// INTEGER UNBOUNDED VARIABLE TESTS
// ============================================================================

#[test]
fn test_unbounded_integer_fallback() {
    // Test: First variable is unbounded, should use fallback [-10000, 10000]
    let mut m = Model::default();
    let x = m.int(i32::MIN, i32::MAX);
    
    match &m.vars[x] {
        selen::variables::Var::VarI(sparse_set) => {
            let min = sparse_set.min();
            let max = sparse_set.max();
            assert_eq!(min, -10000, "Unbounded integer should use fallback min");
            assert_eq!(max, 10000, "Unbounded integer should use fallback max");
            
            // Domain size: 20,001 elements
            let domain_size = (max - min + 1) as u64;
            assert!(domain_size < 100_000, "Fallback should have small domain");
        }
        _ => panic!("Expected integer variable"),
    }
}

#[test]
fn test_unbounded_integer_with_small_context() {
    // Test: Inference from small bounded variables
    let mut m = Model::default();
    
    let _a = m.int(100, 200);  // span = 100
    let _b = m.int(150, 250);  // context: [100, 250], span = 150
    
    let x = m.int(i32::MIN, i32::MAX); // Unbounded
    
    match &m.vars[x] {
        selen::variables::Var::VarI(sparse_set) => {
            let min = sparse_set.min();
            let max = sparse_set.max();
            
            // Context: [100, 250], span = 150
            // Inference: 100 - 1000*150 = -149900, 250 + 1000*150 = 150250
            assert_eq!(min, -149900, "Should expand context by 1000x");
            assert_eq!(max, 150250, "Should expand context by 1000x");
            
            let domain_size = (max as i64 - min as i64 + 1) as u64;
            assert!(domain_size < 1_000_000, "Domain should be under 1M limit");
        }
        _ => panic!("Expected integer variable"),
    }
}

#[test]
fn test_unbounded_integer_with_large_context() {
    // Test: When 1000x expansion would exceed 1M domain, clamp to ±500K around center
    let mut m = Model::default();
    
    // Large context: span = 10,000
    let _a = m.int(0, 10000);
    
    let x = m.int(i32::MIN, i32::MAX);
    
    match &m.vars[x] {
        selen::variables::Var::VarI(sparse_set) => {
            let min = sparse_set.min();
            let max = sparse_set.max();
            
            let domain_size = (max as i64 - min as i64 + 1) as u64;
            println!("Large context test: min={}, max={}, domain_size={}", min, max, domain_size);
            
            // Naive would be: 0 - 10M, 10000 + 10M (20M+ elements)
            // Should clamp to 1M or less
            assert!(domain_size <= 1_000_000, 
                "Domain must respect 1M limit, got {} elements", domain_size);
            
            // Should be centered around the context
            let center = (min as i64 + max as i64) / 2;
            assert!(center >= 0 && center <= 10000, 
                "Should be centered around original context [0, 10000], got center={}", center);
        }
        _ => panic!("Expected integer variable"),
    }
}

#[test]
fn test_unbounded_integer_near_i32_max() {
    // Test: Context near i32::MAX boundary
    let mut m = Model::default();
    
    let _a = m.int(i32::MAX - 1000, i32::MAX - 500);
    let x = m.int(i32::MIN, i32::MAX);
    
    match &m.vars[x] {
        selen::variables::Var::VarI(sparse_set) => {
            let max = sparse_set.max();
            
            // Should be clamped to i32::MAX - 1 (not i32::MAX itself)
            assert!(max < i32::MAX, "Max should be clamped");
            assert_eq!(max, i32::MAX - 1, "Should clamp to i32::MAX - 1");
        }
        _ => panic!("Expected integer variable"),
    }
}

#[test]
fn test_unbounded_integer_near_i32_min() {
    // Test: Context near i32::MIN boundary
    let mut m = Model::default();
    
    let _a = m.int(i32::MIN + 500, i32::MIN + 1000);
    let x = m.int(i32::MIN, i32::MAX);
    
    match &m.vars[x] {
        selen::variables::Var::VarI(sparse_set) => {
            let min = sparse_set.min();
            
            // Should be clamped to i32::MIN + 1 (not i32::MIN itself)
            assert!(min > i32::MIN, "Min should be clamped");
            assert_eq!(min, i32::MIN + 1, "Should clamp to i32::MIN + 1");
        }
        _ => panic!("Expected integer variable"),
    }
}

#[test]
fn test_partially_unbounded_integer_only_min() {
    // Test: Only min is unbounded
    let mut m = Model::default();
    
    let _context = m.int(0, 100);
    let x = m.int(i32::MIN, 500); // Only min is unbounded
    
    match &m.vars[x] {
        selen::variables::Var::VarI(sparse_set) => {
            let min = sparse_set.min();
            let max = sparse_set.max();
            
            // Should infer both bounds from context since min is unbounded
            assert!(min < max, "Should have valid bounds");
            assert!(max < i32::MAX, "Max should be inferred");
        }
        _ => panic!("Expected integer variable"),
    }
}

#[test]
fn test_partially_unbounded_integer_only_max() {
    // Test: Only max is unbounded
    let mut m = Model::default();
    
    let _context = m.int(0, 100);
    let x = m.int(-500, i32::MAX); // Only max is unbounded
    
    match &m.vars[x] {
        selen::variables::Var::VarI(sparse_set) => {
            let min = sparse_set.min();
            let max = sparse_set.max();
            
            // Should infer both bounds from context since max is unbounded
            assert!(min < max, "Should have valid bounds");
            assert!(max < i32::MAX, "Max should be inferred");
        }
        _ => panic!("Expected integer variable"),
    }
}

// ============================================================================
// FLOAT UNBOUNDED VARIABLE TESTS
// ============================================================================

#[test]
fn test_unbounded_float_fallback() {
    // Test: First variable is unbounded, should use fallback [-10000.0, 10000.0]
    let mut m = Model::default();
    let x = m.float(f64::NEG_INFINITY, f64::INFINITY);
    
    match &m.vars[x] {
        selen::variables::Var::VarF(interval) => {
            assert_eq!(interval.min, -10000.0, "Unbounded float should use fallback min");
            assert_eq!(interval.max, 10000.0, "Unbounded float should use fallback max");
        }
        _ => panic!("Expected float variable"),
    }
}

#[test]
fn test_unbounded_float_with_context() {
    // Test: Inference from bounded float variables
    let mut m = Model::default();
    
    let _a = m.float(10.0, 20.0);  // span = 10.0
    let _b = m.float(15.0, 25.0);  // context: [10.0, 25.0], span = 15.0
    
    let x = m.float(f64::NEG_INFINITY, f64::INFINITY);
    
    match &m.vars[x] {
        selen::variables::Var::VarF(interval) => {
            // Context: [10.0, 25.0], span = 15.0
            // Inference: 10.0 - 1000*15.0 = -14990.0, 25.0 + 1000*15.0 = 15025.0
            assert_eq!(interval.min, -14990.0, "Should expand context by 1000x");
            assert_eq!(interval.max, 15025.0, "Should expand context by 1000x");
        }
        _ => panic!("Expected float variable"),
    }
}

#[test]
fn test_unbounded_float_with_nan_min() {
    // Test: NaN is detected as unbounded
    let mut m = Model::default();
    
    let _context = m.float(0.0, 100.0);
    let x = m.float(f64::NAN, 50.0);
    
    match &m.vars[x] {
        selen::variables::Var::VarF(interval) => {
            assert!(interval.min.is_finite(), "NaN should be inferred to finite value");
            assert!(interval.max.is_finite(), "Should have finite bounds");
            assert!(interval.min < interval.max, "Should have valid bounds");
        }
        _ => panic!("Expected float variable"),
    }
}

#[test]
fn test_unbounded_float_with_nan_max() {
    // Test: NaN in max bound is detected as unbounded
    let mut m = Model::default();
    
    let _context = m.float(0.0, 100.0);
    let x = m.float(-50.0, f64::NAN);
    
    match &m.vars[x] {
        selen::variables::Var::VarF(interval) => {
            assert!(interval.min.is_finite(), "Should have finite bounds");
            assert!(interval.max.is_finite(), "NaN should be inferred to finite value");
            assert!(interval.min < interval.max, "Should have valid bounds");
        }
        _ => panic!("Expected float variable"),
    }
}

#[test]
fn test_unbounded_float_only_min_infinity() {
    // Test: Only min is infinite
    let mut m = Model::default();
    
    let _context = m.float(0.0, 100.0);
    let x = m.float(f64::NEG_INFINITY, 500.0);
    
    match &m.vars[x] {
        selen::variables::Var::VarF(interval) => {
            assert!(interval.min.is_finite(), "Should infer finite min");
            assert!(interval.max.is_finite(), "Should have finite max");
            assert!(interval.min < interval.max, "Should have valid bounds");
        }
        _ => panic!("Expected float variable"),
    }
}

#[test]
fn test_unbounded_float_only_max_infinity() {
    // Test: Only max is infinite
    let mut m = Model::default();
    
    let _context = m.float(0.0, 100.0);
    let x = m.float(-500.0, f64::INFINITY);
    
    match &m.vars[x] {
        selen::variables::Var::VarF(interval) => {
            assert!(interval.min.is_finite(), "Should have finite min");
            assert!(interval.max.is_finite(), "Should infer finite max");
            assert!(interval.min < interval.max, "Should have valid bounds");
        }
        _ => panic!("Expected float variable"),
    }
}

#[test]
fn test_unbounded_float_extreme_values() {
    // Test: Very large (but not infinite) bounds are left as-is
    let mut m = Model::default();
    
    let x = m.float(-1e100, 1e100); // Large but finite
    
    match &m.vars[x] {
        selen::variables::Var::VarF(interval) => {
            // These are finite, so no inference needed
            assert_eq!(interval.min, -1e100, "Large finite values should be preserved");
            assert_eq!(interval.max, 1e100, "Large finite values should be preserved");
        }
        _ => panic!("Expected float variable"),
    }
}

// ============================================================================
// TYPE ISOLATION TESTS
// ============================================================================

#[test]
fn test_integer_inference_ignores_float_context() {
    // Test: Integer inference only uses integer variables
    let mut m = Model::default();
    
    // Create float context (should be ignored)
    let _f1 = m.float(1000.0, 2000.0);
    let _f2 = m.float(1500.0, 2500.0);
    
    // Create unbounded integer
    let x = m.int(i32::MIN, i32::MAX);
    
    match &m.vars[x] {
        selen::variables::Var::VarI(sparse_set) => {
            let min = sparse_set.min();
            let max = sparse_set.max();
            
            // Should use fallback, not float context
            assert_eq!(min, -10000, "Should ignore float context, use fallback");
            assert_eq!(max, 10000, "Should ignore float context, use fallback");
        }
        _ => panic!("Expected integer variable"),
    }
}

#[test]
fn test_float_inference_ignores_integer_context() {
    // Test: Float inference only uses float variables
    let mut m = Model::default();
    
    // Create integer context (should be ignored)
    let _i1 = m.int(1000, 2000);
    let _i2 = m.int(1500, 2500);
    
    // Create unbounded float
    let x = m.float(f64::NEG_INFINITY, f64::INFINITY);
    
    match &m.vars[x] {
        selen::variables::Var::VarF(interval) => {
            // Should use fallback, not integer context
            assert_eq!(interval.min, -10000.0, "Should ignore integer context, use fallback");
            assert_eq!(interval.max, 10000.0, "Should ignore integer context, use fallback");
        }
        _ => panic!("Expected float variable"),
    }
}

#[test]
fn test_mixed_types_separate_inference() {
    // Test: Integers and floats infer independently
    let mut m = Model::default();
    
    let _int_context = m.int(100, 200);
    let _float_context = m.float(500.0, 600.0);
    
    let int_var = m.int(i32::MIN, i32::MAX);
    let float_var = m.float(f64::NEG_INFINITY, f64::INFINITY);
    
    match (&m.vars[int_var], &m.vars[float_var]) {
        (selen::variables::Var::VarI(int_ss), selen::variables::Var::VarF(float_iv)) => {
            // Integer should infer from [100, 200]
            let int_min = int_ss.min();
            let int_max = int_ss.max();
            assert!(int_min < 100, "Integer should expand from its context");
            assert!(int_max > 200, "Integer should expand from its context");
            
            // Float should infer from [500.0, 600.0]
            assert!(float_iv.min < 500.0, "Float should expand from its context");
            assert!(float_iv.max > 600.0, "Float should expand from its context");
        }
        _ => panic!("Expected correct variable types"),
    }
}

// ============================================================================
// INTEGRATION TESTS WITH SOLVING
// ============================================================================

#[test]
fn test_unbounded_integer_solves_correctly() {
    // Test: Unbounded variable works in actual solving
    let mut m = Model::default();
    
    let _context = m.int(0, 10);
    let x = m.int(i32::MIN, i32::MAX); // Unbounded
    let y = m.int(0, 100);
    
    // Constraint: x + y == 50
    post!(m, x + y == int(50));
    
    let result = m.solve();
    assert!(result.is_ok(), "Should solve with inferred bounds");
    
    if let Ok(solution) = result {
        if let (selen::variables::Val::ValI(x_val), selen::variables::Val::ValI(y_val)) = (solution[x], solution[y]) {
            assert_eq!(x_val + y_val, 50, "Solution should satisfy constraint");
        }
    }
}

#[test]
fn test_unbounded_float_solves_correctly() {
    // Test: Unbounded float works in actual solving
    let mut m = Model::default();
    
    let _context = m.float(0.0, 10.0);
    let x = m.float(f64::NEG_INFINITY, f64::INFINITY);
    let y = m.float(0.0, 100.0);
    
    // Constraint: x + y == 50.0
    let sum = m.add(x, y);
    post!(m, sum == 50.0);
    
    let result = m.solve();
    assert!(result.is_ok(), "Should solve with inferred bounds");
    
    if let Ok(solution) = result {
        if let (selen::variables::Val::ValF(x_val), selen::variables::Val::ValF(y_val)) = (solution[x], solution[y]) {
            assert!((x_val + y_val - 50.0).abs() < 0.01, "Solution should satisfy constraint");
        }
    }
}

#[test]
fn test_loan_problem_with_unbounded_rate() {
    // Real-world example: loan interest rate calculation
    let mut m = Model::default();
    
    let r = m.float(f64::NEG_INFINITY, f64::INFINITY); // Interest rate (unknown)
    let b = m.float(1000.0, 1000.0); // Loan amount: $1000
    
    let rb = m.mul(r, b);
    
    // Monthly payment = rate * balance
    // $100 = r * $1000
    post!(m, rb == 100.0);
    
    let result = m.solve();
    assert!(result.is_ok(), "Loan problem should solve");
    
    if let Ok(solution) = result {
        if let selen::variables::Val::ValF(r_val) = solution[r] {
            // r * 1000 = 100 => r = 0.1 (10%)
            assert!((r_val - 0.1).abs() < 0.01, 
                "Interest rate should be ~0.1 (10%), got {}", r_val);
        }
    }
}

#[test]
fn test_zero_span_context() {
    // Edge case: All context variables have same value
    let mut m = Model::default();
    
    let _a = m.int(100, 100); // Fixed at 100
    let _b = m.int(100, 100); // Also fixed at 100
    
    let x = m.int(i32::MIN, i32::MAX);
    
    match &m.vars[x] {
        selen::variables::Var::VarI(sparse_set) => {
            let min = sparse_set.min();
            let max = sparse_set.max();
            
            // Context span = 0, so expansion = 0
            // Result should be [100, 100]
            assert!(min <= 100, "Min should include context value");
            assert!(max >= 100, "Max should include context value");
        }
        _ => panic!("Expected integer variable"),
    }
}

#[test]
fn test_negative_context() {
    // Test: Context entirely in negative range
    let mut m = Model::default();
    
    let _a = m.int(-500, -400);
    let x = m.int(i32::MIN, i32::MAX);
    
    match &m.vars[x] {
        selen::variables::Var::VarI(sparse_set) => {
            let min = sparse_set.min();
            let max = sparse_set.max();
            
            // Context: [-500, -400], span = 100
            // Inference: -500 - 100K = -100500, -400 + 100K = 99600
            assert_eq!(min, -100500, "Should handle negative context");
            assert_eq!(max, 99600, "Should expand from negative context");
        }
        _ => panic!("Expected integer variable"),
    }
}

#[test]
fn test_large_negative_float_context() {
    // Test: Float context in large negative range
    let mut m = Model::default();
    
    let _a = m.float(-1000.0, -900.0);
    let x = m.float(f64::NEG_INFINITY, f64::INFINITY);
    
    match &m.vars[x] {
        selen::variables::Var::VarF(interval) => {
            // Context: [-1000.0, -900.0], span = 100.0
            // Inference: -1000.0 - 100K = -101000.0, -900.0 + 100K = 99100.0
            assert_eq!(interval.min, -101000.0, "Should handle negative float context");
            assert_eq!(interval.max, 99100.0, "Should expand from negative context");
        }
        _ => panic!("Expected float variable"),
    }
}

// ============================================================================
// CONFIGURATION TESTS
// ============================================================================

#[test]
fn test_custom_inference_factor() {
    // Test: Custom expansion factor (300x instead of default 1000x)
    let config = SolverConfig::default()
        .with_unbounded_inference_factor(300);
    let mut m = Model::with_config(config);
    
    let _context = m.int(0, 100);  // span = 100
    let x = m.int(i32::MIN, i32::MAX);
    
    match &m.vars[x] {
        selen::variables::Var::VarI(sparse_set) => {
            let min = sparse_set.min();
            let max = sparse_set.max();
            
            // With 300x: 0 - 300*100 = -30000, 100 + 300*100 = 30100
            assert_eq!(min, -30000, "Should use custom 300x factor");
            assert_eq!(max, 30100, "Should use custom 300x factor");
        }
        _ => panic!("Expected integer variable"),
    }
}

#[test]
fn test_custom_inference_factor_float() {
    // Test: Custom expansion factor for floats
    let config = SolverConfig::default()
        .with_unbounded_inference_factor(500);
    let mut m = Model::with_config(config);
    
    let _context = m.float(10.0, 20.0);  // span = 10.0
    let x = m.float(f64::NEG_INFINITY, f64::INFINITY);
    
    match &m.vars[x] {
        selen::variables::Var::VarF(interval) => {
            // With 500x: 10.0 - 500*10.0 = -4990.0, 20.0 + 500*10.0 = 5020.0
            assert_eq!(interval.min, -4990.0, "Should use custom 500x factor");
            assert_eq!(interval.max, 5020.0, "Should use custom 500x factor");
        }
        _ => panic!("Expected float variable"),
    }
}
