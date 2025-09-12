use cspsolver::prelude::*;

#[test]
fn minimal_hanging_test() {
    println!("Starting minimal test");
    let mut model = Model::default();
    println!("Created model with step size: {}", model.float_step_size());
    
    let x = model.float(5.0, 5.6);  // Smaller range around the problem area
    println!("Created variable x with bounds [5.0, 5.6]");
    
    model.lt(x, float(5.5));
    println!("Added constraint x < 5.5");
    
    println!("Starting maximization...");
    let solution = model.maximize(x).expect("Should have solution");
    
    let Val::ValF(x_val) = solution[x] else { panic!("Expected float") };
    println!("Got solution: x = {}", x_val);
    
    assert!(x_val < 5.5);
}
