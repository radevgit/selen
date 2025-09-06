use cspsolver::prelude::*;

#[test] 
fn debug_next_view() {
    println!("=== Debug Next View Implementation ===");
    
    let y = float(2.5);
    let y_next = y.next();
    println!("y = {:?}, y.next() = {:?}", y, y_next);
    
    // The specific constraint that's failing is:
    // less_than_or_equals(y.next(), v0) where v0 = [3,3]
    // This constraint calls:
    // 1. y.next().try_set_max(v0.max(), ctx) -> y.next().try_set_max(3, ctx)
    // 2. v0.try_set_min(y.next().min(), ctx) -> v0.try_set_min(2.5000026, ctx)
    
    // Let's test step 1 in isolation: y.next().try_set_max(3, ctx)
    println!("\n=== Testing y.next().try_set_max(3) ===");
    
    // Create a minimal test case
    let sol_test_max = {
        let mut m = Model::default();
        
        // Create a constraint that will force y.next().try_set_max(3) to be called
        // We can't call try_set_max directly, but we can create a constraint that does
        
        // Actually, let's test it differently - create two variables and constrain them
        let v1 = m.new_var_float(2.5, 2.5);  // Fixed at 2.5
        let v2 = m.new_var_int(3, 3);        // Fixed at 3
        
        // This should call: v1.next().try_set_max(3, ctx)
        m.less_than_or_equals(v1.next(), v2);
        m.solve()
    };
    println!("less_than_or_equals(v1.next(), v2) with v1=2.5, v2=3: {}", sol_test_max.is_some());
    
    // Let's also test if the issue is specifically with constants vs variables
    println!("\n=== Testing constant vs variable ===");
    
    let sol_const = {
        let mut m = Model::default();
        let v0 = m.new_var_int(3, 3);
        // Direct constant - this works
        m.less_than_or_equals(float(2.5000026), v0);
        m.solve()
    };
    println!("less_than_or_equals(float(2.5000026), v0): {}", sol_const.is_some());
    
    let sol_next_const = {
        let mut m = Model::default();
        let v0 = m.new_var_int(3, 3);
        // Next of constant - this might fail!
        m.less_than_or_equals(float(2.5).next(), v0);
        m.solve()
    };
    println!("less_than_or_equals(float(2.5).next(), v0): {}", sol_next_const.is_some());
    
    // Let's also verify the Next view returns the expected value
    println!("\n=== Next view value verification ===");
    if let cspsolver::vars::Val::ValF(next_val) = y_next {
        println!("Expected y.next() value: {}", next_val);
        
        // Manual check: does 2.5000026 <= 3?
        let manual_check = next_val <= 3.0;
        println!("Manual check: {} <= 3.0 = {}", next_val, manual_check);
    }
}
