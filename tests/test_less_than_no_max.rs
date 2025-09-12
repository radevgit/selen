use cspsolver::prelude::*;

#[test]
fn test_less_than_no_maximize() {
    let mut model = Model::default(); // precision 6
    let x = model.float(1.0, 10.0);
    
    model.lt(x, float(5.5));
    
    // Just solve without maximizing
    let solution = model.solve().expect("Should have solution");
    let Val::ValF(x_val) = solution[x] else { panic!("Expected float") };
    
    assert!(x_val < 5.5);
    println!("Constraint x < 5.5, got x = {}", x_val);
}
