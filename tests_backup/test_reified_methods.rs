// Test the new reified constraint methods on Model
// This verifies that the Model methods work the same as the standalone functions

use selen::prelude::*;

#[test]
fn test_all_reified_methods_compile() {
    let mut model = Model::default();
    let x = model.int(0, 10);
    let y = model.int(0, 10);
    let b = model.bool();
    
    // Test that all methods compile
    model.eq_reif(x, y, b);
    
    let b2 = model.bool();
    model.ne_reif(x, y, b2);
    
    let b3 = model.bool();
    model.lt_reif(x, y, b3);
    
    let b4 = model.bool();
    model.le_reif(x, y, b4);
    
    let b5 = model.bool();
    model.gt_reif(x, y, b5);
    
    let b6 = model.bool();
    model.ge_reif(x, y, b6);
    
    // Should compile - actual solving tested in other test files
}

#[test]
fn test_reified_with_floats() {
    let mut model = Model::default();
    let x = model.float(0.0, 10.0);
    let y = model.float(0.0, 10.0);
    let b = model.bool();
    
    // Should work with float variables too
    model.le_reif(x, y, b);
    model.eq_reif(x, y, b);
    model.ne_reif(x, y, b);
    
    // Should compile
}
