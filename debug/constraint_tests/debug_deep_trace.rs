use cspsolver::prelude::*;

#[test]
fn debug_deep_trace() {
    println!("=== Deep Trace of greater_than Implementation ===");
    
    // Test what happens step by step
    println!("1. Creating model and variable:");
    let mut m = Model::default();
    let v0 = m.new_var_int(3, 3);
    println!("   v0 created with domain [3, 3]");
    
    println!("\n2. Getting y value and y.next():");
    let y = float(2.5);
    let y_next = y.next(); // This creates a Next<Val> view
    println!("   y = {:?}", y);
    println!("   y.next() = {:?}", y_next);
    
    println!("\n3. What greater_than(x, y) should do:");
    println!("   greater_than(v0, y) => greater_than_or_equals(v0, y.next())");
    println!("   greater_than_or_equals(v0, y.next()) => less_than_or_equals(y.next(), v0)");
    
    println!("\n4. Testing less_than_or_equals(y.next(), v0) directly:");
    let sol_direct = {
        let mut m_test = Model::default();
        let v0_test = m_test.new_var_int(3, 3);
        m_test.less_than_or_equals(y_next, v0_test);
        m_test.solve()
    };
    println!("   less_than_or_equals({:?}, v0): solvable? {}", y_next, sol_direct.is_some());
    
    println!("\n5. Testing the problematic greater_than:");
    let sol_problem = {
        let mut m_prob = Model::default();
        let v0_prob = m_prob.new_var_int(3, 3);
        m_prob.greater_than(v0_prob, y);
        m_prob.solve()
    };
    println!("   greater_than(v0, {:?}): solvable? {}", y, sol_problem.is_some());
    
    // Maybe there's a difference between using a Next view vs the computed next value?
    println!("\n6. Testing Next view vs computed value:");
    
    // Method 1: Use the Next view directly
    let sol_next_view = {
        let mut m1 = Model::default();
        let v0_1 = m1.new_var_int(3, 3);
        m1.greater_than_or_equals(v0_1, y.next()); // Next<Val>
        m1.solve()
    };
    println!("   Using Next view y.next(): solvable? {}", sol_next_view.is_some());
    
    // Method 2: Use the computed next value directly
    if let cspsolver::vars::Val::ValF(next_val) = y_next {
        let sol_computed = {
            let mut m2 = Model::default();
            let v0_2 = m2.new_var_int(3, 3);
            m2.greater_than_or_equals(v0_2, float(next_val)); // Direct Val
            m2.solve()
        };
        println!("   Using computed value float({}): solvable? {}", next_val, sol_computed.is_some());
    }
}
