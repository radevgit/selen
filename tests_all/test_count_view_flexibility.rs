/// Test cases demonstrating the new impl View flexibility in count() constraint
/// 
/// Before: m.count(&vars, m.int(3, 3), count)  // Had to manually create fixed variable
/// After:  m.count(&vars, Val::int(3), count)  // Direct Val constant support via View
///
/// This showcases the API improvement for consistency with arithmetic operations.

use selen::prelude::*;
use selen::variables::Val;

#[test]
fn test_count_with_constant_target_via_val() {
    let mut m = Model::default();
    let vars = vec![m.int(1, 5), m.int(1, 5), m.int(1, 5)];
    let count = m.int(0, 3);
    
    // Now we can pass Val constants directly instead of creating a fixed variable!
    m.count(&vars, Val::int(3), count);  // Count how many vars equal 3
    
    if let Ok(solution) = m.solve() {
        // Verify the constraint is properly enforced
        let c = solution[count].as_int().unwrap();
        let matches = vars.iter()
            .filter(|&&v| solution[v].as_int().unwrap() == 3)
            .count();
        assert_eq!(c as usize, matches, "Count should match actual occurrences");
    }
}

#[test]
fn test_count_with_variable_target_unchanged() {
    let mut m = Model::default();
    let vars = vec![m.int(1, 5), m.int(1, 5), m.int(1, 5)];
    let var_target = m.int(1, 5);  // Variable target with range
    let count = m.int(0, 3);
    
    // Also works with variable targets (unchanged from before)
    m.count(&vars, var_target, count);
    
    if let Ok(solution) = m.solve() {
        let target_val = solution[var_target].as_int().unwrap();
        let c = solution[count].as_int().unwrap();
        let matches = vars.iter()
            .filter(|&&v| solution[v].as_int().unwrap() == target_val)
            .count();
        assert_eq!(c as usize, matches, "Count should match actual occurrences");
    }
}

#[test]
fn test_count_with_computed_target_expression() {
    let mut m = Model::default();
    let vars = vec![m.int(1, 5), m.int(1, 5), m.int(1, 5)];
    let x = m.int(1, 3);
    let computed_target = m.add(x, Val::int(1));  // x + 1 as target
    let count = m.int(0, 3);
    
    // Can use computed expressions as targets (they're Views!)
    m.count(&vars, computed_target, count);
    
    if let Ok(solution) = m.solve() {
        let x_val = solution[x].as_int().unwrap();
        let target_val = solution[computed_target].as_int().unwrap();
        let c = solution[count].as_int().unwrap();
        
        assert_eq!(target_val, x_val + 1, "Computed target should be x + 1");
        
        let matches = vars.iter()
            .filter(|&&v| solution[v].as_int().unwrap() == target_val)
            .count();
        assert_eq!(c as usize, matches, "Count should match actual occurrences");
    }
}

#[test]
fn test_count_api_consistency_with_arithmetic() {
    // This test demonstrates API consistency:
    // Arithmetic methods use impl View, and now count() does too!
    
    let mut m = Model::default();
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    
    // Arithmetic methods accept Val constants via View
    let sum = m.add(x, Val::int(3));      // ✅ Can pass Val(3)
    let _diff = m.sub(y, Val::int(2));    // ✅ Can pass Val(2)
    let _product = m.mul(x, Val::int(4)); // ✅ Can pass Val(4)
    
    // Now count() also accepts Val constants via View
    let vars = vec![m.int(1, 5), m.int(1, 5), m.int(1, 5)];
    let count = m.int(0, 3);
    m.count(&vars, Val::int(3), count);   // ✅ Can pass Val(3)
    
    // All expressions are created successfully
    if let Ok(solution) = m.solve() {
        let sum_val = solution[sum].as_int().unwrap();
        let x_val = solution[x].as_int().unwrap();
        assert_eq!(sum_val, x_val + 3);
    }
}

#[test]
fn test_count_zero_matches_with_val() {
    let mut m = Model::default();
    let vars = vec![m.int(1, 2), m.int(1, 2), m.int(1, 2)];
    let count = m.int(0, 3);
    
    // Count something that won't appear (all vars are 1-2, looking for 5)
    m.count(&vars, Val::int(5), count);
    
    if let Ok(solution) = m.solve() {
        let c = solution[count].as_int().unwrap();
        assert_eq!(c, 0, "Should count zero matches");
    }
}

#[test]
fn test_count_all_matches_with_val() {
    let mut m = Model::default();
    let vars = vec![m.int(3, 3), m.int(3, 3), m.int(3, 3)];
    let count = m.int(0, 3);
    
    // All variables are fixed to 3, so count should be 3
    m.count(&vars, Val::int(3), count);
    
    if let Ok(solution) = m.solve() {
        let c = solution[count].as_int().unwrap();
        assert_eq!(c, 3, "Should count all three matches");
    }
}

#[test]
fn test_count_with_gcc_using_internal_val() {
    let mut m = Model::default();
    let vars = vec![m.int(1, 3), m.int(1, 3), m.int(1, 3), m.int(1, 3)];
    
    let count1 = m.int(0, 4);
    let count2 = m.int(0, 4);
    let count3 = m.int(0, 4);
    
    // gcc() now benefits from impl View support internally
    // (it uses count internally with Val constants)
    m.gcc(&vars, &[1, 2, 3], &[count1, count2, count3]);
    
    if let Ok(solution) = m.solve() {
        let c1 = solution[count1].as_int().unwrap();
        let c2 = solution[count2].as_int().unwrap();
        let c3 = solution[count3].as_int().unwrap();
        
        // Verify the counts add up to 4
        assert_eq!(c1 + c2 + c3, 4, "Total count should equal number of variables");
    }
}

#[test]
fn test_count_target_with_negative_val() {
    let mut m = Model::default();
    let vars = vec![m.int(-5, 5), m.int(-5, 5), m.int(-5, 5)];
    let count = m.int(0, 3);
    
    // count() should work with negative Val constants too
    m.count(&vars, Val::int(-3), count);
    
    if let Ok(solution) = m.solve() {
        let c = solution[count].as_int().unwrap();
        let matches = vars.iter()
            .filter(|&&v| solution[v].as_int().unwrap() == -3)
            .count();
        assert_eq!(c as usize, matches);
    }
}

#[test]
fn test_count_old_vs_new_api_both_work() {
    // Demonstrate that both ways work (old explicit creation and new Val View)
    
    let mut m = Model::default();
    let vars = vec![m.int(1, 5), m.int(1, 5), m.int(1, 5)];
    let count_old = m.int(0, 3);
    let count_new = m.int(0, 3);
    
    // Old way (still works): explicitly create fixed variable
    let target_old = m.int(3, 3);
    m.count(&vars, target_old, count_old);
    
    // New way (now supported): use Val directly via View
    m.count(&vars, Val::int(3), count_new);
    
    if let Ok(solution) = m.solve() {
        let c_old = solution[count_old].as_int().unwrap();
        let c_new = solution[count_new].as_int().unwrap();
        
        // Both should get the same count
        assert_eq!(c_old, c_new, "Both APIs should produce same count");
    }
}
