use cspsolver::prelude::*;

#[test]
fn debug_ulp_precision() {
    let mut model = Model::with_float_precision(4); // 1e-4 precision
    
    let x = model.float(1.0, 2.0);
    model.gt(x, float(1.5));
    
    let solution = model.minimize(x).expect("Should have solution");
    
    let Val::ValF(x_val) = solution[x] else { panic!("Expected float") };
    
    println!("ULP-based result: {:.17}", x_val);
    println!("Expected step-based: 1.50010000000000000");
    println!("Difference: {:.17}", (x_val - 1.5001).abs());
    
    // Let's also check what the actual next float after 1.5 is
    let next_float = 1.5_f64.next_up();
    println!("Next float after 1.5: {:.17}", next_float);
    
    // Check if our optimization is returning the mathematically correct value
    assert!(x_val > 1.5, "Result should be greater than 1.5");
    assert_eq!(x_val, next_float, "Result should be the next float after 1.5");
}
