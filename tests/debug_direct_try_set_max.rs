use cspsolver::prelude::*;

#[test] 
fn debug_direct_try_set_max() {
    println!("=== Debug Direct try_set_max Call ===");
    
    // Test what happens when we call try_set_max directly
    let sol1 = {
        let mut m = Model::default();
        let v1 = m.new_var_float(2.5, 2.5);
        
        // Try to constrain v1 <= 2.9999974 (which should work)
        m.less_than_or_equals(v1, cspsolver::vars::Val::ValF(2.9999974));
        m.solve()
    };
    println!("less_than_or_equals(v1=2.5, ValF(2.9999974)): {}", sol1.is_some());
    
    // Test the actual issue: why does the constraint fail?
    // Maybe the issue is not in try_set_max but in the constraint propagation order?
    
    let sol2 = {
        let mut m = Model::default();
        let v1 = m.new_var_float(2.5, 2.5);
        let v2 = m.new_var_int(3, 3);
        
        // This is equivalent to what should happen in less_than_or_equals(v1.next(), v2)
        // The constraint should call:
        // 1. v1.next().try_set_max(v2.max()) 
        // 2. v2.try_set_min(v1.next().min())
        
        // Let's manually implement these calls
        // v1.next().min() = 2.5000026, so v2.try_set_min(2.5000026)
        // This should set v2 >= ceil(2.5000026) = 3, which should work
        m.greater_than_or_equals(v2, cspsolver::vars::Val::ValF(2.5000026));
        m.solve()
    };
    println!("greater_than_or_equals(v2=3, ValF(2.5000026)): {}", sol2.is_some());
    
    // Test the reverse: what if the issue is in step 2, not step 1?
    let sol3 = {
        let mut m = Model::default();
        let v1 = m.new_var_float(2.5, 2.5);
        let v2 = m.new_var_int(3, 3);
        
        // v2.try_set_min(v1.next().min()) = v2.try_set_min(2.5000026)
        // This requires v2 >= ceil(2.5000026) = 3
        // Since v2 = [3,3], this should be satisfied
        
        // But let's test the other direction:
        // v1.next().try_set_max(v2.max()) with proper type conversion
        // This should call v1.try_set_max(ValF(3.0).prev()) = v1.try_set_max(2.9999974)
        
        // Let's test both constraints separately
        m.greater_than_or_equals(v2, cspsolver::vars::Val::ValF(2.5000026));  // v2 >= 2.5000026
        m.less_than_or_equals(v1, cspsolver::vars::Val::ValF(2.9999974));     // v1 <= 2.9999974
        m.solve()
    };
    println!("Both constraints together: {}", sol3.is_some());
    
    // Let's check exact values
    println!("\nExact value checks:");
    println!("3.0 >= 2.5000026: {}", 3.0 >= 2.5000026);
    println!("2.5 <= 2.9999974: {}", 2.5 <= 2.9999974);
    println!("ceil(2.5000026): {}", (2.5000026f32).ceil());
}
