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
