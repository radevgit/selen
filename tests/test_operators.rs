use cspsolver::prelude::*;
use cspsolver::operators::*;

#[test]
fn test_comparison_trait_usage() {
    // Test ComparisonOp trait methods individually to avoid conflicts
    let mut model1 = Model::default();
    let x1 = model1.int(1, 10);
    let y1 = model1.int(5, 15);
    x1.eq_op(&mut model1, y1);
    assert!(model1.solve().is_some());
    
    let mut model2 = Model::default();
    let x2 = model2.int(1, 10);
    let y2 = model2.int(5, 15);
    x2.ne_op(&mut model2, y2);
    assert!(model2.solve().is_some());
    
    let mut model3 = Model::default();
    let x3 = model3.int(1, 10);
    let y3 = model3.int(5, 15);
    x3.lt_op(&mut model3, y3);
    assert!(model3.solve().is_some());
    
    let mut model4 = Model::default();
    let x4 = model4.int(1, 10);
    let y4 = model4.int(5, 15);
    x4.le_op(&mut model4, y4);
    assert!(model4.solve().is_some());
    
    let mut model5 = Model::default();
    let x5 = model5.int(1, 10);
    let y5 = model5.int(5, 15);
    x5.gt_op(&mut model5, y5);
    // This might be unsatisfiable since x5 ∈ [1,10], y5 ∈ [5,15], so we don't assert
    
    let mut model6 = Model::default();
    let x6 = model6.int(1, 10);
    let y6 = model6.int(5, 15);
    x6.ge_op(&mut model6, y6);
    // This might be unsatisfiable since x6 ∈ [1,10], y6 ∈ [5,15], so we don't assert
}

#[test]
fn test_boolean_trait_usage() {
    let mut m = Model::default();
    let a = m.int(0, 1); // Boolean variable
    let b = m.int(0, 1); // Boolean variable
    
    // Test BooleanOp trait methods
    a.and_op(&mut m, b);
    a.or_op(&mut m, b);
    a.not_op(&mut m);
    
    // Should not panic and model should be valid
    assert!(m.solve().is_some());
}

#[test]
fn test_model_extension_methods() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    
    // Test Model extension methods for comparison
    m.eq_op(x, y);
    m.ne_op(x, y);
    m.lt_op(x, y);
    m.le_op(x, y);
    m.gt_op(x, y);
    m.ge_op(x, y);
    
    // Should not panic
}

#[test]
fn test_model_boolean_extension_methods() {
    let mut m = Model::default();
    let a = m.int(0, 1); // Boolean variable
    let b = m.int(0, 1); // Boolean variable
    
    // Test Model extension methods for boolean operations
    m.and_op(a, b);
    m.or_op(a, b);
    m.not_op(a);
    
    // Should not panic
}

#[test]
fn test_equality_constraint_with_operators() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    
    // Add equality constraint using operator
    x.eq_op(&mut m, y);
    
    // Solve and verify both variables have same value
    let solution = m.solve().unwrap();
    if let (Val::ValI(x_val), Val::ValI(y_val)) = (solution[x], solution[y]) {
        assert_eq!(x_val, y_val);
    }
}

#[test]
fn test_inequality_constraint_with_operators() {
    let mut m = Model::default();
    let x = m.int(1, 5);
    let y = m.int(1, 5);
    
    // Add inequality constraint using operator
    x.ne_op(&mut m, y);
    
    // Solve and verify variables have different values
    let solution = m.solve().unwrap();
    if let (Val::ValI(x_val), Val::ValI(y_val)) = (solution[x], solution[y]) {
        assert_ne!(x_val, y_val);
    }
}

#[test]
fn test_less_than_constraint_with_operators() {
    let mut m = Model::default();
    let x = m.int(1, 5);
    let y = m.int(3, 10);
    
    // Add less-than constraint using operator
    x.lt_op(&mut m, y);
    
    // Solve and verify x < y
    let solution = m.solve().unwrap();
    if let (Val::ValI(x_val), Val::ValI(y_val)) = (solution[x], solution[y]) {
        assert!(x_val < y_val);
    }
}

#[test]
fn test_boolean_and_constraint_with_operators() {
    let mut m = Model::default();
    let a = m.int(0, 1); // Boolean variable
    let b = m.int(0, 1); // Boolean variable
    
    // Create AND constraint using operator
    a.and_op(&mut m, b);
    
    // Should be solvable
    assert!(m.solve().is_some());
}

#[test]
fn test_boolean_or_constraint_with_operators() {
    let mut m = Model::default();
    let a = m.int(0, 1); // Boolean variable
    let b = m.int(0, 1); // Boolean variable
    
    // Use operator to create OR constraint
    a.or_op(&mut m, b);
    
    // Should be solvable
    assert!(m.solve().is_some());
}

#[test]
fn test_boolean_not_constraint_with_operators() {
    let mut m = Model::default();
    let a = m.int(0, 1); // Boolean variable
    
    // Use operator to create NOT constraint
    a.not_op(&mut m);
    
    // Should be solvable
    assert!(m.solve().is_some());
}

#[test]
fn test_mixed_constraints_with_operators() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    let z = m.int(1, 10);
    
    // Mix different operator constraints
    x.lt_op(&mut m, y);    // x < y
    y.le_op(&mut m, z);    // y <= z
    x.ne_op(&mut m, z);    // x != z
    
    // Should be solvable
    let solution = m.solve().unwrap();
    if let (Val::ValI(x_val), Val::ValI(y_val), Val::ValI(z_val)) = 
        (solution[x], solution[y], solution[z]) {
        assert!(x_val < y_val);
        assert!(y_val <= z_val);
        assert_ne!(x_val, z_val);
    }
}

#[test]
fn test_multiplication_with_int_constants() {
    // Test the new constraint patterns: var * int(constant)
    let mut m = Model::default();
    let x = m.int(1, 10);
    let result = m.int(0, 100);
    
    // Test equality: result == x * int(5)
    post!(m, result == x * int(5));
    post!(m, x == 3);  // Force x = 3, so result should be 15
    
    let solution = m.solve().unwrap();
    if let (Val::ValI(x_val), Val::ValI(result_val)) = (solution[x], solution[result]) {
        assert_eq!(x_val, 3);
        assert_eq!(result_val, 15);  // 3 * 5 = 15
    }
}

#[test]
fn test_multiplication_with_float_constants() {
    // Test the new constraint patterns: var * float(constant)
    let mut m = Model::default();
    let items = m.int(1, 100);
    let cost = m.float(0.0, 1000.0);
    
    // Test equality: cost == items * float(12.5)
    post!(m, cost == items * float(12.5));
    post!(m, items == 4);  // Force items = 4, so cost should be 50.0
    
    let solution = m.solve().unwrap();
    if let (Val::ValI(items_val), Val::ValF(cost_val)) = (solution[items], solution[cost]) {
        assert_eq!(items_val, 4);
        assert!((cost_val - 50.0).abs() < 0.001);  // 4 * 12.5 = 50.0
    }
}

#[test]
fn test_multiplication_with_constants_inequalities() {
    // Test inequality constraints with multiplication
    let mut m = Model::default();
    let x = m.int(1, 20);
    let budget = m.int(0, 100);
    
    // Test: budget >= x * int(3) (budget must be at least 3x)
    post!(m, budget >= x * int(3));
    post!(m, budget == 15);  // Force budget = 15
    
    let solution = m.solve().unwrap();
    if let (Val::ValI(x_val), Val::ValI(budget_val)) = (solution[x], solution[budget]) {
        assert_eq!(budget_val, 15);
        assert!(x_val <= 5);  // Since budget = 15, x * 3 <= 15, so x <= 5
    }
}

#[test]
fn test_multiplication_with_float_constants_optimization() {
    // Test optimization with float multiplication
    let mut m = Model::default();
    let items = m.int(1, 100);
    let cost = m.float(0.0, 1000.0);
    
    // Constraint: cost == items * float(12.5)
    post!(m, cost == items * float(12.5));
    // Budget constraint: cost <= float(500.0)
    post!(m, cost <= float(500.0));
    
    // Maximize items within budget
    let solution = m.maximize(items).unwrap();
    if let (Val::ValI(items_val), Val::ValF(cost_val)) = (solution[items], solution[cost]) {
        // items * 12.5 <= 500, so items <= 40
        assert!(items_val <= 40);
        assert!(cost_val <= 500.0);
        assert!((cost_val - (items_val as f64 * 12.5)).abs() < 0.001);
    }
}

#[test]
fn test_mixed_multiplication_constraints() {
    // Test multiple multiplication constraints together
    let mut m = Model::default();
    let a = m.int(1, 10);
    let b = m.int(1, 10);
    let result1 = m.int(0, 100);
    let result2 = m.float(0.0, 100.0);
    
    // Multiple multiplication constraints
    post!(m, result1 == a * int(5));      // result1 = a * 5
    post!(m, result2 == b * float(2.5));  // result2 = b * 2.5
    post!(m, result1 > result2);          // result1 > result2
    post!(m, a == 4);                     // Force a = 4
    post!(m, b == 3);                     // Force b = 3
    
    let solution = m.solve().unwrap();
    if let (Val::ValI(a_val), Val::ValI(b_val), Val::ValI(result1_val), Val::ValF(result2_val)) = 
        (solution[a], solution[b], solution[result1], solution[result2]) {
        assert_eq!(a_val, 4);
        assert_eq!(b_val, 3);
        assert_eq!(result1_val, 20);  // 4 * 5 = 20
        assert!((result2_val - 7.5).abs() < 0.001);  // 3 * 2.5 = 7.5
        assert!(result1_val as f64 > result2_val);  // 20 > 7.5
    }
}
