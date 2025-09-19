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
    let _constraint1 = x.eq(5);
    let _constraint2 = y.gt(3);
    let _constraint3 = z.le(15);
    
    // Test expression building syntax
    let _expr1 = x.add(y);
    let _expr2 = x.mul(2);
    let _expr3 = y.sub(1);
    
    // Test complex constraint building
    let _constraint4 = x.add(y).eq(z);
    let _constraint5 = x.mul(2).le(10);
    
    println!("✓ All runtime constraint syntax compiles correctly");
}

#[test]
fn test_constraint_posting() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // Test that we can post constraints
    let constraint = x.gt(5);
    let _prop_id = m.post(constraint);
    
    let constraint2 = y.le(8);
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
            "gt" => x.gt(value),
            "le" => y.le(value),
            "eq" => x.eq(value),
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
    let _constraint2 = x.mul(2).add(y).le(20);
    let _constraint3 = x.sub(1).gt(y.add(2));
    
    println!("✓ Expression chaining syntax works");
}

#[test]
fn test_conversion_traits() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    
    // Test that different types can be used in expressions
    let _constraint1 = x.eq(5);        // i32
    let _constraint2 = x.gt(3.5);      // f64  
    let _constraint3 = x.add(10).le(20); // i32 in expression
    
    println!("✓ Type conversions work correctly");
}

#[test]
fn test_constraint_composition() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);

    // Test constraint boolean operations
    let c1 = x.gt(5);
    let c2 = y.le(8);

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
        "add" => expr_data.left_var.add(expr_data.right_value).eq(expr_data.target_var),
        "sub" => expr_data.left_var.sub(expr_data.right_value).eq(expr_data.target_var),
        "mul" => expr_data.left_var.mul(expr_data.right_value).eq(expr_data.target_var),
        "div" => expr_data.left_var.div(expr_data.right_value).eq(expr_data.target_var),
        _ => panic!("Unknown operation"),
    };
    
    let _prop_id = m.post(constraint);
    
    println!("✓ Runtime expression building from data works");
}