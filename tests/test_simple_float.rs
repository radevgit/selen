use cspsolver::prelude::*;

#[test]
fn test_simple_float_constraint() {
    let mut model = Model::default();
    let x = model.new_var_float(1.0, 10.0);
    
    // Just set a simple constraint without using less_than
    model.le(x, float(5.5));
    
    let solution = model.solve().expect("Should have solution");
    let Val::ValF(x_val) = solution[x] else { panic!("Expected float") };
    
    assert!(x_val <= 5.5);
    println!("Simple constraint x <= 5.5, got x = {}", x_val);
}
