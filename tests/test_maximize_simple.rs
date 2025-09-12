use cspsolver::prelude::*;

#[test] 
fn test_maximize_simple_constraint() {
    let mut model = Model::default(); // precision 6
    let x = model.float(1.0, 10.0);
    
    // Use less_than_or_equals instead of less_than
    model.le(x, float(5.5));
    
    let solution = model.maximize(x).expect("Should have solution");
    let Val::ValF(x_val) = solution[x] else { panic!("Expected float") };
    
    assert!(x_val <= 5.5);
    println!("Maximized x with x <= 5.5, got x = {}", x_val);
}
