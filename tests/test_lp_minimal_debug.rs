// Minimal test to debug CSP-LP integration

use selen::prelude::*;

#[test]
fn test_minimal() {
    let mut m = Model::default();
    let x = m.float(0.0, 10.0);
    println!("Created variable x");
    
    m.float_lin_eq(&[1.0], &[x], 5.0);
    println!("Added constraint");
    
    let system = m.extract_linear_system();
    println!("Extracted system: {} constraints, {} variables", 
             system.n_constraints(), system.n_variables());
    
    assert_eq!(system.n_constraints(), 1);
    assert_eq!(system.n_variables(), 1);
}
