use cspsolver::prelude::*;

#[test] 
fn debug_simple() {
    println!("=== Simple Test ===");
    
    let y = float(2.5);
    println!("y = {:?}, y.next() = {:?}", y, y.next());
    
    // Test the main issue: greater_than should work for v0=3 > 2.5
    let sol_failing = {
        let mut m = Model::default();
        let v0 = m.new_var_int(3, 3);
        m.greater_than(v0, y);
        m.solve()
    };
    println!("greater_than(v0=3, y=2.5): {}", sol_failing.is_some());
    
    let sol_working = {
        let mut m = Model::default();
        let v0 = m.new_var_int(3, 3);
        m.greater_than_or_equals(v0, y.next());
        m.solve()
    };
    println!("greater_than_or_equals(v0=3, y.next()=2.5000026): {}", sol_working.is_some());
    
    // Test with larger domain
    let sol_large = {
        let mut m = Model::default();
        let v0 = m.new_var_int(1, 10);
        m.greater_than(v0, y);
        m.solve()
    };
    println!("greater_than(v0[1,10], y=2.5): has solution? {}", sol_large.is_some());
    if let Some(_) = sol_large {
        println!("   (solution exists)");
    }
}
