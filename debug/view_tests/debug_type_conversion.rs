use cspsolver::prelude::*;

#[test] 
fn debug_type_conversion() {
    println!("=== Debug Type Conversion Issue ===");
    
    // Test what happens when we call prev() on integer 3
    let three_i = cspsolver::vars::Val::ValI(3);
    let three_f = cspsolver::vars::Val::ValF(3.0);
    let three_i_prev = three_i.prev();
    let three_f_prev = three_f.prev();
    
    println!("ValI(3).prev() = {:?}", three_i_prev);
    println!("ValF(3.0).prev() = {:?}", three_f_prev);
    
    // The issue: when v2 is integer [3,3], v2.max() returns ValI(3)
    // Then v1.next().try_set_max(ValI(3)) calls v1.try_set_max(ValI(3).prev()) = v1.try_set_max(ValI(2))
    // But v1 is a float variable, so we're trying to set float max to integer 2!
    
    println!("\n=== Testing direct type conversion issue ===");
    
    // Test 1: Float variable, integer max - this should work due to type conversion
    let sol1 = {
        let mut m = Model::default();
        let v1 = m.new_var_float(2.5, 2.5);
        // Try to set max to integer 2 - this should fail because 2 < 2.5
        m.less_than_or_equals(v1, cspsolver::vars::Val::ValI(2));
        m.solve()
    };
    println!("less_than_or_equals(v1=2.5, ValI(2)): {}", sol1.is_some());
    
    // Test 2: Float variable, float max 
    let sol2 = {
        let mut m = Model::default();
        let v1 = m.new_var_float(2.5, 2.5);
        // Try to set max to float 2.9999974 - this should also fail because 2.9999974 < 2.5
        m.less_than_or_equals(v1, cspsolver::vars::Val::ValF(2.9999974));
        m.solve()
    };
    println!("less_than_or_equals(v1=2.5, ValF(2.9999974)): {}", sol2.is_some());
    
    // Test 3: The real issue - when v2.max() is integer but should be converted to float
    let sol3 = {
        let mut m = Model::default();
        let v1 = m.new_var_float(2.5, 2.5);
        let v2 = m.new_var_int(3, 3);  // This causes v2.max() to return ValI(3)
        
        // Manually implement what should happen:
        // v1.next().try_set_max(v2.max()) should work
        // But v2.max() returns ValI(3), and ValI(3).prev() = ValI(2)
        // So we get v1.try_set_max(ValI(2)) which fails because 2 < 2.5
        
        m.less_than_or_equals(v1, cspsolver::vars::Val::ValI(3));
        m.solve()
    };
    println!("less_than_or_equals(v1=2.5, ValI(3)): {}", sol3.is_some());
    
    println!("\n=== The root cause ===");
    println!("When v2 is integer [3,3], v2.max() returns ValI(3)");
    println!("v1.next().try_set_max(ValI(3)) calls v1.try_set_max(ValI(3).prev())");
    println!("ValI(3).prev() = ValI(2)");
    println!("v1.try_set_max(ValI(2)) fails because v1=2.5 > 2");
    println!("But it SHOULD call v1.try_set_max(ValF(3.0).prev()) = v1.try_set_max(ValF(2.9999974))");
    println!("And v1=2.5 <= 2.9999974, so that would succeed!");
}
