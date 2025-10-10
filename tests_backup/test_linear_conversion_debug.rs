use selen::prelude::*;

#[test]
fn test_linear_conversion_simple() {
    eprintln!("\n=== TEST: Simple x + y = z ===");
    let mut model = Model::default();
    
    let x = model.float(0.0, 10.0);
    let y = model.float(0.0, 10.0);
    let z = model.float(0.0, 20.0);
    
    eprintln!("Posting x.add(y).eq(z)...");
    model.new(x.add(y).eq(z));
    
    eprintln!("Pending ASTs: {}", model.pending_constraint_asts.len());
    eprintln!("Pending LP constraints: {}", model.pending_lp_constraints.len());
    
    // Don't solve, just check if conversion happened
    assert_eq!(model.pending_constraint_asts.len(), 1, "Should have 1 AST");
    assert_eq!(model.pending_lp_constraints.len(), 1, "Should have 1 LP constraint");
}

#[test]
fn test_linear_conversion_with_coefficients() {
    eprintln!("\n=== TEST: 2*x + 3*y = 10 ===");
    let mut model = Model::default();
    
    let x = model.int(0, 10);
    let y = model.int(0, 10);
    
    eprintln!("Posting mul(x,2).add(mul(y,3)).eq(10)...");
    model.new(x.mul(int(2)).add(y.mul(int(3))).eq(int(10)));
    
    eprintln!("Pending ASTs: {}", model.pending_constraint_asts.len());
    eprintln!("Pending LP constraints: {}", model.pending_lp_constraints.len());
    
    assert_eq!(model.pending_constraint_asts.len(), 1);
    assert_eq!(model.pending_lp_constraints.len(), 1);
}
