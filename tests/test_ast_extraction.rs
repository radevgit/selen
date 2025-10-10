use selen::prelude::*;

#[test]
fn test_simple_ast_extraction() {
    let mut model = Model::default();
    let x = model.float(0.0, 10.0);
    let y = model.float(0.0, 10.0);
    
    // This should extract x + y <= 5
    model.new(x.add(y).le(5.0));
    
    eprintln!("Pending LP constraints: {}", model.pending_lp_constraints.len());
    
    let result = model.minimize(x);
    assert!(result.is_ok());
}
