use cspsolver::prelude::*;

#[test]
fn test_less_than_with_floats_precision_4() {
    let mut model = Model::with_float_precision(4);

    let x = model.new_var_float(1.0, 10.0);
    model.lt(x, float(5.5));
    
    let solution = model.maximize(x).expect("Should have solution");
    
    let Val::ValF(x_val) = solution[x] else { unreachable!() };
    assert!(x_val < 5.5);
    // Should be just slightly below 5.5 due to maximization
    assert!(x_val > 5.4);
    println!("Maximized x with precision 4: {}", x_val);
}

#[test] 
fn test_less_than_with_floats_precision_6() {
    let mut model = Model::with_float_precision(6);

    let x = model.new_var_float(1.0, 10.0);
    model.lt(x, float(5.5));
    
    let solution = model.maximize(x).expect("Should have solution");
    
    let Val::ValF(x_val) = solution[x] else { unreachable!() };
    assert!(x_val < 5.5);
    // Should be just slightly below 5.5 due to maximization
    assert!(x_val > 5.4);
    println!("Maximized x with precision 6: {}", x_val);
}
