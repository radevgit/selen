use cspsolver::prelude::*;

#[test]
fn test_bool_and_basic() {
    let mut model = Model::default();
    let a = model.new_var_int(0, 1);
    let b = model.new_var_int(0, 1);
    let result = model.bool_and(&[a, b]);

    // Test: true AND true = true
    model.eq(a, Val::int(1));
    model.eq(b, Val::int(1));
    let solution = model.solve();
    assert!(solution.is_some());
    let sol = solution.unwrap();
    assert_eq!(sol[result], Val::ValI(1));
}

#[test]
fn test_bool_and_false_result() {
    let mut model = Model::default();
    let a = model.new_var_int(0, 1);
    let b = model.new_var_int(0, 1);
    let result = model.bool_and(&[a, b]);

    // Test: result = false, so at least one operand must be false
    model.eq(result, Val::int(0));
    model.eq(a, Val::int(1)); // Force a to be true, so b must be false
    
    let solution = model.solve();
    assert!(solution.is_some());
    let sol = solution.unwrap();
    assert_eq!(sol[a], Val::ValI(1));
    assert_eq!(sol[b], Val::ValI(0));
    assert_eq!(sol[result], Val::ValI(0));
}

#[test]
fn test_bool_and_three_operands() {
    let mut model = Model::default();
    let a = model.new_var_int(0, 1);
    let b = model.new_var_int(0, 1);
    let c = model.new_var_int(0, 1);
    let result = model.bool_and(&[a, b, c]);

    // All true should give true result
    model.eq(a, Val::int(1));
    model.eq(b, Val::int(1));
    model.eq(c, Val::int(1));
    
    let solution = model.solve();
    assert!(solution.is_some());
    let sol = solution.unwrap();
    assert_eq!(sol[result], Val::ValI(1));
}

#[test]
fn test_bool_and_one_false() {
    let mut model = Model::default();
    let a = model.new_var_int(0, 1);
    let b = model.new_var_int(0, 1);
    let c = model.new_var_int(0, 1);
    let result = model.bool_and(&[a, b, c]);

    // One false should give false result
    model.eq(a, Val::int(1));
    model.eq(b, Val::int(0)); // This one is false
    model.eq(c, Val::int(1));
    
    let solution = model.solve();
    assert!(solution.is_some());
    let sol = solution.unwrap();
    assert_eq!(sol[result], Val::ValI(0));
}

#[test]
fn test_bool_or_basic() {
    let mut model = Model::default();
    let a = model.new_var_int(0, 1);
    let b = model.new_var_int(0, 1);
    let result = model.bool_or(&[a, b]);

    // Test: false OR true = true
    model.eq(a, Val::int(0));
    model.eq(b, Val::int(1));
    let solution = model.solve();
    assert!(solution.is_some());
    let sol = solution.unwrap();
    assert_eq!(sol[result], Val::ValI(1));
}

#[test]
fn test_bool_or_all_false() {
    let mut model = Model::default();
    let a = model.new_var_int(0, 1);
    let b = model.new_var_int(0, 1);
    let result = model.bool_or(&[a, b]);

    // Test: false OR false = false
    model.eq(a, Val::int(0));
    model.eq(b, Val::int(0));
    let solution = model.solve();
    assert!(solution.is_some());
    let sol = solution.unwrap();
    assert_eq!(sol[result], Val::ValI(0));
}

#[test]
fn test_bool_or_propagation() {
    let mut model = Model::default();
    let a = model.new_var_int(0, 1);
    let b = model.new_var_int(0, 1);
    let result = model.bool_or(&[a, b]);

    // If result must be true and a is false, then b must be true
    model.eq(result, Val::int(1));
    model.eq(a, Val::int(0));
    
    let solution = model.solve();
    assert!(solution.is_some());
    let sol = solution.unwrap();
    assert_eq!(sol[a], Val::ValI(0));
    assert_eq!(sol[b], Val::ValI(1));
    assert_eq!(sol[result], Val::ValI(1));
}

#[test]
fn test_bool_not_basic() {
    let mut model = Model::default();
    let a = model.new_var_int(0, 1);
    let result = model.bool_not(a);

    // Test: NOT true = false
    model.eq(a, Val::int(1));
    let solution = model.solve();
    assert!(solution.is_some());
    let sol = solution.unwrap();
    assert_eq!(sol[result], Val::ValI(0));
}

#[test]
fn test_bool_not_false() {
    let mut model = Model::default();
    let a = model.new_var_int(0, 1);
    let result = model.bool_not(a);

    // Test: NOT false = true
    model.eq(a, Val::int(0));
    let solution = model.solve();
    assert!(solution.is_some());
    let sol = solution.unwrap();
    assert_eq!(sol[result], Val::ValI(1));
}

#[test]
fn test_bool_not_propagation() {
    let mut model = Model::default();
    let a = model.new_var_int(0, 1);
    let result = model.bool_not(a);

    // If result must be false, then a must be true
    model.eq(result, Val::int(0));
    
    let solution = model.solve();
    assert!(solution.is_some());
    let sol = solution.unwrap();
    assert_eq!(sol[a], Val::ValI(1));
    assert_eq!(sol[result], Val::ValI(0));
}

#[test]
fn test_bool_complex_expression() {
    let mut model = Model::default();
    let a = model.new_var_int(0, 1);
    let b = model.new_var_int(0, 1);
    let c = model.new_var_int(0, 1);
    
    // Build expression: (a AND b) OR (NOT c)
    let and_result = model.bool_and(&[a, b]);
    let not_c = model.bool_not(c);
    let final_result = model.bool_or(&[and_result, not_c]);
    
    // Set up test case: a=1, b=0, c=1
    // (1 AND 0) OR (NOT 1) = 0 OR 0 = 0
    model.eq(a, Val::int(1));
    model.eq(b, Val::int(0));
    model.eq(c, Val::int(1));
    
    let solution = model.solve();
    assert!(solution.is_some());
    let sol = solution.unwrap();
    assert_eq!(sol[and_result], Val::ValI(0)); // 1 AND 0 = 0
    assert_eq!(sol[not_c], Val::ValI(0));      // NOT 1 = 0
    assert_eq!(sol[final_result], Val::ValI(0)); // 0 OR 0 = 0
}

#[test]
fn test_bool_mixed_constraints() {
    let mut model = Model::default();
    let x = model.new_var_int(0, 10);
    let y = model.new_var_int(0, 10);
    
    // Create boolean variables for conditions
    let x_gt_5 = model.new_var_int(0, 1);
    let y_lt_3 = model.new_var_int(0, 1);
    
    // x_gt_5 should be true if x > 5, false otherwise
    // This requires using actual constraint propagation
    // For now, we'll test a simpler version
    
    // Test: if x_gt_5 AND y_lt_3, then some condition
    let both_true = model.bool_and(&[x_gt_5, y_lt_3]);
    
    model.eq(x_gt_5, Val::int(1));
    model.eq(y_lt_3, Val::int(1));
    
    let solution = model.solve();
    assert!(solution.is_some());
    let sol = solution.unwrap();
    assert_eq!(sol[both_true], Val::ValI(1));
}

#[test]
fn test_bool_empty_arrays() {
    let mut model = Model::default();
    
    // Empty AND should be true
    let empty_and = model.bool_and(&[]);
    let solution = model.solve();
    assert!(solution.is_some());
    let sol = solution.unwrap();
    assert_eq!(sol[empty_and], Val::ValI(1));
}

#[test]
fn test_bool_empty_or() {
    let mut model = Model::default();
    
    // Empty OR should be false
    let empty_or = model.bool_or(&[]);
    let solution = model.solve();
    assert!(solution.is_some());
    let sol = solution.unwrap();
    assert_eq!(sol[empty_or], Val::ValI(0));
}

#[test]
fn test_bool_unsatisfiable_and() {
    let mut model = Model::default();
    let a = model.new_var_int(0, 1);
    let b = model.new_var_int(0, 1);
    let result = model.bool_and(&[a, b]);

    // Unsatisfiable: result must be true but one operand is false
    model.eq(result, Val::int(1)); // Result must be true
    model.eq(a, Val::int(0));      // But a is false
    
    let solution = model.solve();
    assert!(solution.is_none()); // Should be unsatisfiable
}

#[test]
fn test_bool_unsatisfiable_or() {
    let mut model = Model::default();
    let a = model.new_var_int(0, 1);
    let b = model.new_var_int(0, 1);
    let result = model.bool_or(&[a, b]);

    // Unsatisfiable: result must be false but one operand is true
    model.eq(result, Val::int(0)); // Result must be false
    model.eq(a, Val::int(1));      // But a is true
    
    let solution = model.solve();
    assert!(solution.is_none()); // Should be unsatisfiable
}

#[test]
fn test_bool_with_larger_numbers() {
    let mut model = Model::default();
    let a = model.new_var_int(5, 10); // Non-zero values (true)
    let b = model.new_var_int(0, 0);  // Zero (false)
    let result = model.bool_and(&[a, b]);

    // Even though a is non-zero (true), b is zero (false), so AND should be false
    let solution = model.solve();
    assert!(solution.is_some());
    let sol = solution.unwrap();
    assert_eq!(sol[result], Val::ValI(0));
    if let Val::ValI(val_a) = sol[a] {
        assert!(val_a >= 5 && val_a <= 10);
    }
    assert_eq!(sol[b], Val::ValI(0));
}
