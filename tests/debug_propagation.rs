use cspsolver::prelude::*;

#[test] 
fn debug_propagation() {
    println!("=== Debug Constraint Propagation ===");
    
    let y = float(2.5);
    let y_next = y.next();
    println!("y = {:?}, y.next() = {:?}", y, y_next);
    
    // Test 1: Manual chain exactly like greater_than should do
    println!("\n1. Manual implementation of greater_than:");
    let sol1 = {
        let mut m = Model::default();
        let v0 = m.new_var_int(3, 3);
        // greater_than(v0, y) should be equivalent to:
        // greater_than_or_equals(v0, y.next()) which is:
        // less_than_or_equals(y.next(), v0)
        m.less_than_or_equals(y_next, v0);
        m.solve()
    };
    println!("   less_than_or_equals(y.next(), v0): {}", sol1.is_some());
    
    // Test 2: Actual greater_than call
    println!("\n2. Actual greater_than call:");
    let sol2 = {
        let mut m = Model::default();
        let v0 = m.new_var_int(3, 3);
        m.greater_than(v0, y);
        m.solve()
    };
    println!("   greater_than(v0, y): {}", sol2.is_some());
    
    // Test 3: Let's see if the issue is with how Next is handled in constraint creation
    println!("\n3. Step-by-step equivalent:");
    let sol3 = {
        let mut m = Model::default();
        let v0 = m.new_var_int(3, 3);
        // Step 1: Create y.next()
        let y_next_val = y.next();
        // Step 2: Call greater_than_or_equals
        m.greater_than_or_equals(v0, y_next_val);
        m.solve()
    };
    println!("   greater_than_or_equals(v0, y.next()): {}", sol3.is_some());
    
    // Test 4: Test if the issue is with the specific value 2.5000026
    if let cspsolver::vars::Val::ValF(next_val) = y_next {
        let sol4 = {
            let mut m = Model::default();
            let v0 = m.new_var_int(3, 3);
            m.greater_than_or_equals(v0, float(next_val));
            m.solve()
        };
        println!("   greater_than_or_equals(v0, float({})): {}", next_val, sol4.is_some());
        
        // Test 5: And try the direct less_than_or_equals
        let sol5 = {
            let mut m = Model::default();
            let v0 = m.new_var_int(3, 3);
            m.less_than_or_equals(float(next_val), v0);
            m.solve()
        };
        println!("   less_than_or_equals(float({}), v0): {}", next_val, sol5.is_some());
    }
}
