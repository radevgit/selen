//! Tests for the runtime constraint API

use crate::prelude::*;
use crate::runtime_api::{VarIdExt, ModelExt};

#[test]
fn test_basic_runtime_constraint_syntax() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let z = m.int(0, 20);

    // Test basic constraint syntax compilation
    let _constraint1 = x.eq(int(5));
    let _constraint2 = y.gt(int(3));
    let _constraint3 = z.le(int(15));
    
    // Test expression building syntax
    let _expr1 = x.add(y);
    let _expr2 = x.mul(int(2));
    let _expr3 = y.sub(int(1));
    
    // Test complex constraint building
    let _constraint4 = x.add(y).eq(z);
    let _constraint5 = x.mul(int(2)).le(int(10));
    
    println!("✓ All runtime constraint syntax compiles correctly");
}

#[test]
fn test_constraint_posting() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // Test that we can post constraints
    let constraint = x.gt(int(5));
    let _prop_id = m.post(constraint);
    
    let constraint2 = y.le(int(8));
    let _prop_id2 = m.post(constraint2);
    
    println!("✓ Constraints can be posted to model");
}

#[test]
fn test_dynamic_constraint_building() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // Test building constraints from runtime data
    let operations = vec![
        ("gt", 5),
        ("le", 8),
        ("eq", 3),
    ];
    
    for (op, value) in operations {
        let constraint = match op {
            "gt" => x.gt(int(value)),
            "le" => y.le(int(value)),
            "eq" => x.eq(int(value)),
            _ => panic!("Unknown operator"),
        };
        let _prop_id = m.post(constraint);
    }
    
    println!("✓ Dynamic constraint building from data works");
}

#[test]
fn test_expression_chaining() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let z = m.int(0, 20);
    
    // Test that expression chaining compiles
    let _constraint = x.add(y).eq(z);
    let _constraint2 = x.mul(int(2)).add(y).le(int(20));
    let _constraint3 = x.sub(int(1)).gt(y.add(int(2)));
    
    println!("✓ Expression chaining syntax works");
}

#[test]
fn test_conversion_traits() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    
    // Test that different types can be used in expressions
    let _constraint1 = x.eq(int(5));        // int() wrapper
    let _constraint2 = x.gt(float(3.5));    // float() wrapper  
    let _constraint3 = x.add(int(10)).le(int(20)); // int() in expression
    
    println!("✓ Type conversions work correctly");
}

#[test]
fn test_constraint_composition() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);

    // Test constraint boolean operations
    let c1 = x.gt(int(5));
    let c2 = y.le(int(8));

    let _combined_and = c1.clone().and(c2.clone());
    let _combined_or = c1.clone().or(c2.clone());
    let _negated = c1.not();

    println!("✓ Constraint composition works");
}#[test]
fn test_runtime_expression_building() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let _y = m.int(0, 10);
    let z = m.int(0, 20);
    
    // Test building expressions completely from runtime data
    struct ExpressionData {
        operation: String,
        left_var: VarId,
        right_value: i32,
        target_var: VarId,
    }
    
    let expr_data = ExpressionData {
        operation: "add".to_string(),
        left_var: x,
        right_value: 10,
        target_var: z,
    };
    
    // Build constraint dynamically from data
    let constraint = match expr_data.operation.as_str() {
        "add" => expr_data.left_var.add(int(expr_data.right_value)).eq(expr_data.target_var),
        "sub" => expr_data.left_var.sub(int(expr_data.right_value)).eq(expr_data.target_var),
        "mul" => expr_data.left_var.mul(int(expr_data.right_value)).eq(expr_data.target_var),
        "div" => expr_data.left_var.div(int(expr_data.right_value)).eq(expr_data.target_var),
        _ => panic!("Unknown operation"),
    };
    
    let _prop_id = m.post(constraint);
    
    println!("✓ Runtime expression building from data works");
}

// =================== PHASE 2 TESTS ===================

#[test]
fn test_model_c_method() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // Test Model::c() ultra-short syntax
    m.c(x).eq(int(5));                    // m.c(x).eq(int(5))
    m.c(y).gt(int(3));                    // m.c(y).gt(int(3))
    m.c(x).add(y).le(int(15));           // m.c(x).add(y).le(int(15))
    m.c(x).mul(int(2)).sub(int(1)).ne(y); // m.c(x).mul(int(2)).sub(int(1)).ne(y)
    
    println!("✓ Model::c() ultra-short syntax works");
}

#[test]
fn test_builder_fluent_interface() {
    use crate::runtime_api::Builder;
    
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // Test fluent Builder interface
    let builder1 = Builder::new(&mut m, x);
    builder1.add(int(5)).eq(int(10));
    
    let builder2 = Builder::new(&mut m, y);
    builder2.mul(int(2)).sub(int(3)).ge(x);
    
    // Test chaining operations
    Builder::new(&mut m, x).add(y).div(int(2)).lt(int(5));
    
    println!("✓ Builder fluent interface works");
}

#[test]
fn test_global_constraint_shortcuts() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let z = m.int(0, 10);
    let w = m.int(0, 10);
    
    // Test ultra-short global constraint methods
    m.alldiff(&[x, y, z]);           // All different
    m.alleq(&[y, z]);                // All equal
    
    // Test element constraint
    let array = vec![x, y, z];
    let index = m.int(0, 2);
    m.elem(&array, index, w);        // array[index] == w
    
    // Test count constraint
    let vars = vec![x, y, z, w];
    let count_result = m.int(0, 4);
    m.count(&vars, 5, count_result); // count(vars, value=5) == count_result
    
    println!("✓ Global constraint shortcuts work");
}

#[test]
fn test_enhanced_type_conversions() {
    let mut m = Model::default();
    let x = m.int(0, 100);
    
    // Test enhanced type conversions from Phase 2
    m.c(x).eq(int(5));      // Use int() wrapper
    m.c(x).gt(int(10));     // Use int() wrapper  
    m.c(x).le(int(200));    // Use int() wrapper
    m.c(x).ne(int(50));     // Use int() wrapper
    m.c(x).ge(float(3.5));  // Use float() wrapper
    
    // Test reference conversions
    let y = m.int(0, 50);
    m.c(x).add(&y).eq(int(25)); // Use int() wrapper
    
    println!("✓ Enhanced type conversions work");
}

#[test]
fn test_phase2_integration() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    let z = m.int(1, 10);
    
    // Test mixed Phase 1 and Phase 2 syntax
    let constraint1 = x.add(y).eq(z);  // Phase 1: direct constraint creation
    m.post(constraint1);               // Phase 1: explicit posting
    
    m.c(x).mul(int(2)).le(y.add(int(5))); // Phase 2: auto-posting builder
    
    // Test global constraints with constraint building
    m.alldiff(&[x, y, z]);
    m.c(x).add(y).add(z).eq(int(15));
    
    println!("✓ Phase 1 and Phase 2 integration works");
}

// =================== PHASE 3: BOOLEAN LOGIC TESTS ===================

#[test]
fn test_constraint_and_composition() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // Create two constraints using the existing API
    let c1 = x.ge(int(5));  // x >= 5
    let c2 = y.le(int(8));  // y <= 8
    
    // Combine with AND
    let combined = c1.and(c2);
    m.post(combined);
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(solution) = result {
        let x_val = solution.get_int(x);    // Clean: get_int() method
        let y_val = solution.get_int(y);    // Clean: get_int() method  
        assert!(x_val >= 5);
        assert!(y_val <= 8);
    }
    
    println!("✓ Constraint AND composition works");
}

#[test]
fn test_constraint_or_composition() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    
    // Create two constraints: x <= 3 OR x >= 7
    let c1 = x.le(int(3));  // x <= 3
    let c2 = x.ge(int(7));  // x >= 7
    
    // Combine with OR
    let combined = c1.or(c2);
    m.post(combined);
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(solution) = result {
        let x_val = solution[x].as_int().unwrap();  // Alternative: indexing + as_int()
        assert!(x_val <= 3 || x_val >= 7);
    }
    
    println!("✓ Constraint OR composition works");
}

#[test]
fn test_constraint_not_composition() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    
    // Create constraint x == 5, then negate it
    let c1 = x.eq(int(5));
    let not_c1 = c1.not();
    
    m.post(not_c1);
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(solution) = result {
        // Most direct approach - no unwrap needed since we know it's an int
        let x_val = solution.get_int(x);
        assert_ne!(x_val, 5);
    }
    
    println!("✓ Constraint NOT composition works");
}

#[test]
fn test_constraint_and_all() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let z = m.int(0, 10);
    
    let constraints = vec![
        x.ge(int(3)),  // x >= 3
        y.le(int(7)),  // y <= 7
        z.eq(int(5)),  // z == 5
    ];
    
    if let Some(combined) = Constraint::and_all(constraints) {
        m.post(combined);
    }
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(solution) = result {
        let x_val = solution.get_value(x).as_int().unwrap();
        let y_val = solution.get_value(y).as_int().unwrap();
        let z_val = solution.get_value(z).as_int().unwrap();
        assert!(x_val >= 3);
        assert!(y_val <= 7);
        assert_eq!(z_val, 5);
    }
    
    println!("✓ Constraint and_all() works");
}

#[test]
fn test_constraint_or_all() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    
    let constraints = vec![
        x.eq(int(2)),  // x == 2
        x.eq(int(5)),  // x == 5
        x.eq(int(8)),  // x == 8
    ];
    
    if let Some(combined) = Constraint::or_all(constraints) {
        m.post(combined);
    }
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(solution) = result {
        let x_val = solution.get_value(x).as_int().unwrap();
        assert!(x_val == 2 || x_val == 5 || x_val == 8);
    }
    
    println!("✓ Constraint or_all() works");
}

#[test]
fn test_constraint_vec_and_all() {
    use crate::runtime_api::ConstraintVecExt;
    
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    let constraints = vec![
        x.ge(int(4)),  // x >= 4
        y.le(int(6)),  // y <= 6
    ];
    
    if let Some(combined) = constraints.and_all() {
        m.post(combined);
    }
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(solution) = result {
        let x_val = solution.get_value(x).as_int().unwrap();
        let y_val = solution.get_value(y).as_int().unwrap();
        assert!(x_val >= 4);
        assert!(y_val <= 6);
    }
    
    println!("✓ ConstraintVecExt and_all() works");
}

#[test]
fn test_constraint_vec_or_all() {
    use crate::runtime_api::ConstraintVecExt;
    
    let mut m = Model::default();
    let x = m.int(0, 10);
    
    let constraints = vec![
        x.eq(int(1)),  // x == 1
        x.eq(int(9)),  // x == 9
    ];
    
    if let Some(combined) = constraints.or_all() {
        m.post(combined);
    }
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(solution) = result {
        let x_val = solution.get_value(x).as_int().unwrap();
        assert!(x_val == 1 || x_val == 9);
    }
    
    println!("✓ ConstraintVecExt or_all() works");
}

#[test]
fn test_model_post_all() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    let constraints = vec![
        x.ge(int(5)),  // x >= 5
        y.le(int(5)),  // y <= 5
    ];
    
    let prop_ids = m.post_all(constraints);
    assert_eq!(prop_ids.len(), 2);
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(solution) = result {
        let x_val = solution.get_value(x).as_int().unwrap();
        let y_val = solution.get_value(y).as_int().unwrap();
        assert!(x_val >= 5);
        assert!(y_val <= 5);
    }
    
    println!("✓ Model post_all() works");
}

#[test]
fn test_model_post_and() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    
    let constraints = vec![
        x.ge(int(3)),  // x >= 3
        x.le(int(7)),  // x <= 7
    ];
    
    if let Some(_prop_id) = m.post_and(constraints) {
        // Successfully posted combined constraint
    } else {
        panic!("Expected constraint to be posted");
    }
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(solution) = result {
        let x_val = solution.get_value(x).as_int().unwrap();
        assert!(x_val >= 3 && x_val <= 7);
    }
    
    println!("✓ Model post_and() works");
}

#[test]
fn test_model_post_or() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    
    let constraints = vec![
        x.le(int(2)),  // x <= 2
        x.ge(int(8)),  // x >= 8
    ];
    
    if let Some(_prop_id) = m.post_or(constraints) {
        // Successfully posted combined constraint
    } else {
        panic!("Expected constraint to be posted");
    }
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(solution) = result {
        let x_val = solution.get_value(x).as_int().unwrap();
        assert!(x_val <= 2 || x_val >= 8);
    }
    
    println!("✓ Model post_or() works");
}

#[test]
fn test_phase3_helper_functions() {
    use crate::runtime_api::{and_all, or_all, all_of, any_of};
    
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    let constraints1 = vec![
        x.ge(int(3)),  // x >= 3
        y.le(int(7)),  // y <= 7
    ];
    
    let constraints2 = vec![
        x.eq(int(1)),  // x == 1
        x.eq(int(9)),  // x == 9
    ];
    
    // Test helper functions
    if let Some(combined_and) = and_all(constraints1.clone()) {
        m.post(combined_and);
    }
    
    if let Some(combined_or) = or_all(constraints2.clone()) {
        m.post(combined_or);
    }
    
    // Test aliases
    if let Some(_combined_all_of) = all_of(constraints1) {
        // all_of is alias for and_all
    }
    
    if let Some(_combined_any_of) = any_of(constraints2) {
        // any_of is alias for or_all
    }
    
    println!("✓ Phase 3 helper functions work");
}

// =================== PHASE 3: BOOLEAN LOGIC TESTS ===================

#[test]
fn test_constraint_and_composition() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // Create two constraints using the existing API
    let c1 = x.ge(int(5));  // x >= 5
    let c2 = y.le(int(8));  // y <= 8
    
    // Combine with AND
    let combined = c1.and(c2);
    m.post(combined);
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(Some(solution)) = result {
        let x_val = solution.get_value(x).as_int().unwrap();
        let y_val = solution.get_value(y).as_int().unwrap();
        assert!(x_val >= 5);
        assert!(y_val <= 8);
    }
    
    println!("✓ Constraint AND composition works");
}

#[test]
fn test_constraint_or_composition() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    
    // Create two constraints: x <= 3 OR x >= 7
    let c1 = x.le(int(3));  // x <= 3
    let c2 = x.ge(int(7));  // x >= 7
    
    // Combine with OR
    let combined = c1.or(c2);
    m.post(combined);
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(Some(solution)) = result {
        let x_val = solution.get_value(x).as_int().unwrap();
        assert!(x_val <= 3 || x_val >= 7);
    }
    
    println!("✓ Constraint OR composition works");
}

#[test]
fn test_constraint_not_composition() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    
    // Create constraint x == 5, then negate it
    let c1 = x.eq(int(5));
    let not_c1 = c1.not();
    
    m.post(not_c1);
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(Some(solution)) = result {
        let x_val = solution.get_value(x).as_int().unwrap();
        assert_ne!(x_val, 5);
    }
    
    println!("✓ Constraint NOT composition works");
}

#[test]
fn test_constraint_and_all() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let z = m.int(0, 10);
    
    let constraints = vec![
        x.ge(int(3)),  // x >= 3
        y.le(int(7)),  // y <= 7
        z.eq(int(5)),  // z == 5
    ];
    
    if let Some(combined) = Constraint::and_all(constraints) {
        m.post(combined);
    }
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(Some(solution)) = result {
        let x_val = solution.get_value(x).as_int().unwrap();
        let y_val = solution.get_value(y).as_int().unwrap();
        let z_val = solution.get_value(z).as_int().unwrap();
        assert!(x_val >= 3);
        assert!(y_val <= 7);
        assert_eq!(z_val, 5);
    }
    
    println!("✓ Constraint and_all() works");
}

#[test]
fn test_constraint_or_all() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    
    let constraints = vec![
        x.eq(int(2)),  // x == 2
        x.eq(int(5)),  // x == 5
        x.eq(int(8)),  // x == 8
    ];
    
    if let Some(combined) = Constraint::or_all(constraints) {
        m.post(combined);
    }
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(Some(solution)) = result {
        let x_val = solution.get_value(x).as_int().unwrap();
        assert!(x_val == 2 || x_val == 5 || x_val == 8);
    }
    
    println!("✓ Constraint or_all() works");
}

#[test]
fn test_constraint_vec_and_all() {
    use crate::runtime_api::ConstraintVecExt;
    
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    let constraints = vec![
        x.ge(int(4)),  // x >= 4
        y.le(int(6)),  // y <= 6
    ];
    
    if let Some(combined) = constraints.and_all() {
        m.post(combined);
    }
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(Some(solution)) = result {
        let x_val = solution.get_value(x).as_int().unwrap();
        let y_val = solution.get_value(y).as_int().unwrap();
        assert!(x_val >= 4);
        assert!(y_val <= 6);
    }
    
    println!("✓ ConstraintVecExt and_all() works");
}

#[test]
fn test_constraint_vec_or_all() {
    use crate::runtime_api::ConstraintVecExt;
    
    let mut m = Model::default();
    let x = m.int(0, 10);
    
    let constraints = vec![
        x.eq(int(1)),  // x == 1
        x.eq(int(9)),  // x == 9
    ];
    
    if let Some(combined) = constraints.or_all() {
        m.post(combined);
    }
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(Some(solution)) = result {
        let x_val = solution.get_value(x).as_int().unwrap();
        assert!(x_val == 1 || x_val == 9);
    }
    
    println!("✓ ConstraintVecExt or_all() works");
}

#[test]
fn test_model_post_all() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    let constraints = vec![
        x.ge(int(5)),  // x >= 5
        y.le(int(5)),  // y <= 5
    ];
    
    let prop_ids = m.post_all(constraints);
    assert_eq!(prop_ids.len(), 2);
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(Some(solution)) = result {
        let x_val = solution.get_value(x).as_int().unwrap();
        let y_val = solution.get_value(y).as_int().unwrap();
        assert!(x_val >= 5);
        assert!(y_val <= 5);
    }
    
    println!("✓ Model post_all() works");
}

#[test]
fn test_model_post_and() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    
    let constraints = vec![
        x.ge(int(3)),  // x >= 3
        x.le(int(7)),  // x <= 7
    ];
    
    if let Some(_prop_id) = m.post_and(constraints) {
        // Successfully posted combined constraint
    } else {
        panic!("Expected constraint to be posted");
    }
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(Some(solution)) = result {
        let x_val = solution.get_value(x).as_int().unwrap();
        assert!(x_val >= 3 && x_val <= 7);
    }
    
    println!("✓ Model post_and() works");
}

#[test]
fn test_model_post_or() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    
    let constraints = vec![
        x.le(int(2)),  // x <= 2
        x.ge(int(8)),  // x >= 8
    ];
    
    if let Some(_prop_id) = m.post_or(constraints) {
        // Successfully posted combined constraint
    } else {
        panic!("Expected constraint to be posted");
    }
    
    let result = m.solve();
    assert!(result.is_ok());
    
    if let Ok(Some(solution)) = result {
        let x_val = solution.get_value(x).as_int().unwrap();
        assert!(x_val <= 2 || x_val >= 8);
    }
    
    println!("✓ Model post_or() works");
}

#[test]
fn test_phase3_helper_functions() {
    use crate::runtime_api::{and_all, or_all, all_of, any_of};
    
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    let constraints1 = vec![
        x.ge(int(3)),  // x >= 3
        y.le(int(7)),  // y <= 7
    ];
    
    let constraints2 = vec![
        x.eq(int(1)),  // x == 1
        x.eq(int(9)),  // x == 9
    ];
    
    // Test helper functions
    if let Some(combined_and) = and_all(constraints1.clone()) {
        m.post(combined_and);
    }
    
    if let Some(combined_or) = or_all(constraints2.clone()) {
        m.post(combined_or);
    }
    
    // Test aliases
    if let Some(_combined_all_of) = all_of(constraints1) {
        // all_of is alias for and_all
    }
    
    if let Some(_combined_any_of) = any_of(constraints2) {
        // any_of is alias for or_all
    }
    
    println!("✓ Phase 3 helper functions work");
}

#[test]
fn test_constraint_or_composition() {
    use crate::runtime_api::{Constraint, ComparisonOp};
    
    let mut m = Model::default();
    let x = m.int(0, 10);
    
    // Create two constraints: x <= 3 OR x >= 7
    let c1 = Constraint::new(x.into(), ComparisonOp::Leq, int(3).into());
    let c2 = Constraint::new(x.into(), ComparisonOp::Geq, int(7).into());
    
    // Combine with OR
    let combined = c1.or(c2);
    m.post(combined);
    
    let result = m.solve();
    assert!(result.is_some());
    
    if let Some(solution) = result {
        let x_val = solution.get_value(x).as_int().unwrap();
        assert!(x_val <= 3 || x_val >= 7);
    }
    
    println!("✓ Constraint OR composition works");
}

#[test]
fn test_constraint_not_composition() {
    use crate::runtime_api::{Constraint, ComparisonOp};
    
    let mut m = Model::default();
    let x = m.int(0, 10);
    
    // Create constraint x == 5, then negate it
    let c1 = Constraint::new(x.into(), ComparisonOp::Eq, int(5).into());
    let not_c1 = c1.not();
    
    m.post(not_c1);
    
    let result = m.solve();
    assert!(result.is_some());
    
    if let Some(solution) = result {
        let x_val = solution.get_value(x).as_int().unwrap();
        assert_ne!(x_val, 5);
    }
    
    println!("✓ Constraint NOT composition works");
}

#[test]
fn test_constraint_and_all() {
    use crate::runtime_api::{Constraint, ComparisonOp};
    
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let z = m.int(0, 10);
    
    let constraints = vec![
        Constraint::new(x.into(), ComparisonOp::Geq, int(3).into()),
        Constraint::new(y.into(), ComparisonOp::Leq, int(7).into()),
        Constraint::new(z.into(), ComparisonOp::Eq, int(5).into()),
    ];
    
    if let Some(combined) = Constraint::and_all(constraints) {
        m.post(combined);
    }
    
    let result = m.solve();
    assert!(result.is_some());
    
    if let Some(solution) = result {
        let x_val = solution.get_value(x).as_int().unwrap();
        let y_val = solution.get_value(y).as_int().unwrap();
        let z_val = solution.get_value(z).as_int().unwrap();
        assert!(x_val >= 3);
        assert!(y_val <= 7);
        assert_eq!(z_val, 5);
    }
    
    println!("✓ Constraint and_all() works");
}

#[test]
fn test_constraint_or_all() {
    use crate::runtime_api::{Constraint, ComparisonOp};
    
    let mut m = Model::default();
    let x = m.int(0, 10);
    
    let constraints = vec![
        Constraint::new(x.into(), ComparisonOp::Eq, int(2).into()),
        Constraint::new(x.into(), ComparisonOp::Eq, int(5).into()),
        Constraint::new(x.into(), ComparisonOp::Eq, int(8).into()),
    ];
    
    if let Some(combined) = Constraint::or_all(constraints) {
        m.post(combined);
    }
    
    let result = m.solve();
    assert!(result.is_some());
    
    if let Some(solution) = result {
        let x_val = solution.get_value(x).as_int().unwrap();
        assert!(x_val == 2 || x_val == 5 || x_val == 8);
    }
    
    println!("✓ Constraint or_all() works");
}

#[test]
fn test_constraint_vec_and_all() {
    use crate::runtime_api::{Constraint, ComparisonOp, ConstraintVecExt};
    
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    let constraints = vec![
        Constraint::new(x.into(), ComparisonOp::Geq, int(4).into()),
        Constraint::new(y.into(), ComparisonOp::Leq, int(6).into()),
    ];
    
    if let Some(combined) = constraints.and_all() {
        m.post(combined);
    }
    
    let result = m.solve();
    assert!(result.is_some());
    
    if let Some(solution) = result {
        let x_val = solution.get_value(x).as_int().unwrap();
        let y_val = solution.get_value(y).as_int().unwrap();
        assert!(x_val >= 4);
        assert!(y_val <= 6);
    }
    
    println!("✓ ConstraintVecExt and_all() works");
}

#[test]
fn test_constraint_vec_or_all() {
    use crate::runtime_api::{Constraint, ComparisonOp, ConstraintVecExt};
    
    let mut m = Model::default();
    let x = m.int(0, 10);
    
    let constraints = vec![
        Constraint::new(x.into(), ComparisonOp::Eq, int(1).into()),
        Constraint::new(x.into(), ComparisonOp::Eq, int(9).into()),
    ];
    
    if let Some(combined) = constraints.or_all() {
        m.post(combined);
    }
    
    let result = m.solve();
    assert!(result.is_some());
    
    if let Some(solution) = result {
        let x_val = solution.get_value(x).as_int().unwrap();
        assert!(x_val == 1 || x_val == 9);
    }
    
    println!("✓ ConstraintVecExt or_all() works");
}

#[test]
fn test_model_post_all() {
    use crate::runtime_api::{Constraint, ComparisonOp};
    
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    let constraints = vec![
        Constraint::new(x.into(), ComparisonOp::Geq, int(5).into()),
        Constraint::new(y.into(), ComparisonOp::Leq, int(5).into()),
    ];
    
    let prop_ids = m.post_all(constraints);
    assert_eq!(prop_ids.len(), 2);
    
    let result = m.solve();
    assert!(result.is_some());
    
    if let Some(solution) = result {
        let x_val = solution.get_value(x).as_int().unwrap();
        let y_val = solution.get_value(y).as_int().unwrap();
        assert!(x_val >= 5);
        assert!(y_val <= 5);
    }
    
    println!("✓ Model post_all() works");
}

#[test]
fn test_model_post_and() {
    use crate::runtime_api::{Constraint, ComparisonOp};
    
    let mut m = Model::default();
    let x = m.int(0, 10);
    
    let constraints = vec![
        Constraint::new(x.into(), ComparisonOp::Geq, int(3).into()),
        Constraint::new(x.into(), ComparisonOp::Leq, int(7).into()),
    ];
    
    if let Some(_prop_id) = m.post_and(constraints) {
        // Successfully posted combined constraint
    } else {
        panic!("Expected constraint to be posted");
    }
    
    let result = m.solve();
    assert!(result.is_some());
    
    if let Some(solution) = result {
        let x_val = solution.get_value(x).as_int().unwrap();
        assert!(x_val >= 3 && x_val <= 7);
    }
    
    println!("✓ Model post_and() works");
}

#[test]
fn test_model_post_or() {
    use crate::runtime_api::{Constraint, ComparisonOp};
    
    let mut m = Model::default();
    let x = m.int(0, 10);
    
    let constraints = vec![
        Constraint::new(x.into(), ComparisonOp::Leq, int(2).into()),
        Constraint::new(x.into(), ComparisonOp::Geq, int(8).into()),
    ];
    
    if let Some(_prop_id) = m.post_or(constraints) {
        // Successfully posted combined constraint
    } else {
        panic!("Expected constraint to be posted");
    }
    
    let result = m.solve();
    assert!(result.is_some());
    
    if let Some(solution) = result {
        let x_val = solution.get_value(x).as_int().unwrap();
        assert!(x_val <= 2 || x_val >= 8);
    }
    
    println!("✓ Model post_or() works");
}

#[test]
fn test_phase3_helper_functions() {
    use crate::runtime_api::{Constraint, ComparisonOp, and_all, or_all, all_of, any_of};
    
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    let constraints1 = vec![
        Constraint::new(x.into(), ComparisonOp::Geq, int(3).into()),
        Constraint::new(y.into(), ComparisonOp::Leq, int(7).into()),
    ];
    
    let constraints2 = vec![
        Constraint::new(x.into(), ComparisonOp::Eq, int(1).into()),
        Constraint::new(x.into(), ComparisonOp::Eq, int(9).into()),
    ];
    
    // Test helper functions
    if let Some(combined_and) = and_all(constraints1.clone()) {
        m.post(combined_and);
    }
    
    if let Some(combined_or) = or_all(constraints2.clone()) {
        m.post(combined_or);
    }
    
    // Test aliases
    if let Some(_combined_all_of) = all_of(constraints1) {
        // all_of is alias for and_all
    }
    
    if let Some(_combined_any_of) = any_of(constraints2) {
        // any_of is alias for or_all
    }
    
    println!("✓ Phase 3 helper functions work");
}