/// Test cases for the boolean XOR constraint
/// 
/// XOR (exclusive-or) returns true when exactly one input is true

use selen::prelude::*;

#[test]
fn test_bool_xor_both_false() {
    // 0 XOR 0 = 0
    let mut m = Model::default();
    let a = m.bool();
    let b = m.bool();
    let result = m.bool_xor(a, b);
    
    m.new(a.eq(0));
    m.new(b.eq(0));
    
    if let Ok(solution) = m.solve() {
        let a_val = solution[a].as_int().unwrap();
        let b_val = solution[b].as_int().unwrap();
        let result_val = solution[result].as_int().unwrap();
        
        assert_eq!(a_val, 0);
        assert_eq!(b_val, 0);
        assert_eq!(result_val, 0, "0 XOR 0 should equal 0");
    }
}

#[test]
fn test_bool_xor_left_true_right_false() {
    // 1 XOR 0 = 1
    let mut m = Model::default();
    let a = m.bool();
    let b = m.bool();
    let result = m.bool_xor(a, b);
    
    m.new(a.eq(1));
    m.new(b.eq(0));
    
    if let Ok(solution) = m.solve() {
        let a_val = solution[a].as_int().unwrap();
        let b_val = solution[b].as_int().unwrap();
        let result_val = solution[result].as_int().unwrap();
        
        assert_eq!(a_val, 1);
        assert_eq!(b_val, 0);
        assert_eq!(result_val, 1, "1 XOR 0 should equal 1");
    }
}

#[test]
fn test_bool_xor_left_false_right_true() {
    // 0 XOR 1 = 1
    let mut m = Model::default();
    let a = m.bool();
    let b = m.bool();
    let result = m.bool_xor(a, b);
    
    m.new(a.eq(0));
    m.new(b.eq(1));
    
    if let Ok(solution) = m.solve() {
        let a_val = solution[a].as_int().unwrap();
        let b_val = solution[b].as_int().unwrap();
        let result_val = solution[result].as_int().unwrap();
        
        assert_eq!(a_val, 0);
        assert_eq!(b_val, 1);
        assert_eq!(result_val, 1, "0 XOR 1 should equal 1");
    }
}

#[test]
fn test_bool_xor_both_true() {
    // 1 XOR 1 = 0
    let mut m = Model::default();
    let a = m.bool();
    let b = m.bool();
    let result = m.bool_xor(a, b);
    
    m.new(a.eq(1));
    m.new(b.eq(1));
    
    if let Ok(solution) = m.solve() {
        let a_val = solution[a].as_int().unwrap();
        let b_val = solution[b].as_int().unwrap();
        let result_val = solution[result].as_int().unwrap();
        
        assert_eq!(a_val, 1);
        assert_eq!(b_val, 1);
        assert_eq!(result_val, 0, "1 XOR 1 should equal 0");
    }
}

#[test]
fn test_bool_xor_result_true_forces_difference() {
    // If XOR result must be 1, then inputs must differ
    let mut m = Model::default();
    let a = m.bool();
    let b = m.bool();
    let result = m.bool_xor(a, b);
    
    m.new(result.eq(1));  // Force XOR to be true
    
    if let Ok(solution) = m.solve() {
        let a_val = solution[a].as_int().unwrap();
        let b_val = solution[b].as_int().unwrap();
        let result_val = solution[result].as_int().unwrap();
        
        assert_eq!(result_val, 1);
        // Exactly one should be true (they should differ)
        assert_ne!(a_val, b_val, "When XOR is 1, inputs must differ");
        assert_eq!((a_val + b_val) % 2, 1, "Exactly one input must be true");
    }
}

#[test]
fn test_bool_xor_result_false_forces_same() {
    // If XOR result must be 0, then inputs must be the same
    let mut m = Model::default();
    let a = m.bool();
    let b = m.bool();
    let result = m.bool_xor(a, b);
    
    m.new(result.eq(0));  // Force XOR to be false
    
    if let Ok(solution) = m.solve() {
        let a_val = solution[a].as_int().unwrap();
        let b_val = solution[b].as_int().unwrap();
        let result_val = solution[result].as_int().unwrap();
        
        assert_eq!(result_val, 0);
        // Both should be the same (both true or both false)
        assert_eq!(a_val, b_val, "When XOR is 0, inputs must be the same");
    }
}

#[test]
fn test_bool_xor_propagation_from_result_true_left_fixed() {
    // If result is 1 and left is fixed to 1, then right must be 0
    let mut m = Model::default();
    let a = m.bool();
    let b = m.bool();
    let result = m.bool_xor(a, b);
    
    m.new(a.eq(1));        // Left is true
    m.new(result.eq(1));   // Result is true
    
    if let Ok(solution) = m.solve() {
        let a_val = solution[a].as_int().unwrap();
        let b_val = solution[b].as_int().unwrap();
        let result_val = solution[result].as_int().unwrap();
        
        assert_eq!(a_val, 1);
        assert_eq!(result_val, 1);
        assert_eq!(b_val, 0, "If a=1 and result=1, then b must be 0");
    }
}

#[test]
fn test_bool_xor_propagation_from_result_true_right_fixed() {
    // If result is 1 and right is fixed to 0, then left must be 1
    let mut m = Model::default();
    let a = m.bool();
    let b = m.bool();
    let result = m.bool_xor(a, b);
    
    m.new(b.eq(0));        // Right is false
    m.new(result.eq(1));   // Result is true
    
    if let Ok(solution) = m.solve() {
        let a_val = solution[a].as_int().unwrap();
        let b_val = solution[b].as_int().unwrap();
        let result_val = solution[result].as_int().unwrap();
        
        assert_eq!(b_val, 0);
        assert_eq!(result_val, 1);
        assert_eq!(a_val, 1, "If b=0 and result=1, then a must be 1");
    }
}

#[test]
fn test_bool_xor_propagation_from_result_false_left_fixed() {
    // If result is 0 and left is fixed to 1, then right must be 1
    let mut m = Model::default();
    let a = m.bool();
    let b = m.bool();
    let result = m.bool_xor(a, b);
    
    m.new(a.eq(1));        // Left is true
    m.new(result.eq(0));   // Result is false
    
    if let Ok(solution) = m.solve() {
        let a_val = solution[a].as_int().unwrap();
        let b_val = solution[b].as_int().unwrap();
        let result_val = solution[result].as_int().unwrap();
        
        assert_eq!(a_val, 1);
        assert_eq!(result_val, 0);
        assert_eq!(b_val, 1, "If a=1 and result=0, then b must be 1");
    }
}

#[test]
fn test_bool_xor_multiple_clauses() {
    // Test XOR in a more complex constraint scenario
    let mut m = Model::default();
    let a = m.bool();
    let b = m.bool();
    let c = m.bool();
    let xor_ab = m.bool_xor(a, b);
    let and_result = m.bool_and(&[xor_ab, c]);
    
    // Constraint: (a XOR b) AND c must be true
    m.new(and_result.eq(1));
    
    if let Ok(solution) = m.solve() {
        let a_val = solution[a].as_int().unwrap();
        let b_val = solution[b].as_int().unwrap();
        let c_val = solution[c].as_int().unwrap();
        let xor_val = solution[xor_ab].as_int().unwrap();
        let and_val = solution[and_result].as_int().unwrap();
        
        // c must be true
        assert_eq!(c_val, 1);
        // XOR must be true (so a and b differ)
        assert_eq!(xor_val, 1);
        assert_ne!(a_val, b_val);
        // AND must be true
        assert_eq!(and_val, 1);
    }
}

#[test]
fn test_bool_xor_unsatisfiable_conflict() {
    // Create an unsatisfiable constraint with XOR
    let mut m = Model::default();
    let a = m.bool();
    let b = m.bool();
    let result = m.bool_xor(a, b);
    
    // Try to force both: (a and b are the same) AND (a XOR b is true)
    // This is impossible!
    m.new(a.eq(b));        // a and b must be the same
    m.new(result.eq(1));   // a XOR b must be 1 (they must differ)
    
    let solution = m.solve();
    assert!(solution.is_err(), "Should be unsatisfiable: a=b AND a XOR b=1");
}
