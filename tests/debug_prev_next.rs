use cspsolver::prelude::*;

#[test] 
fn debug_prev_next() {
    println!("=== Debug prev/next operations ===");
    
    let y = float(2.5);
    let y_next = y.next();
    println!("y = {:?}, y.next() = {:?}", y, y_next);
    
    // Test next/prev roundtrip
    if let cspsolver::vars::Val::ValF(y_next_val) = y_next {
        let y_next_prev = cspsolver::vars::Val::ValF(y_next_val).prev();
        println!("y.next().prev() = {:?}", y_next_prev);
        
        // Test with integer values that might be involved
        let three_i = cspsolver::vars::Val::ValI(3);
        let three_f = cspsolver::vars::Val::ValF(3.0);
        println!("ValI(3).prev() = {:?}", three_i.prev());
        println!("ValF(3.0).prev() = {:?}", three_f.prev());
        
        // Test what happens when we try to set max of y.next() to 3
        println!("\n=== Test Next view try_set_max ===");
        let sol_test = {
            let mut m = Model::default();
            let v0 = m.new_var_int(3, 3);
            
            // Create a constraint that will call y.next().try_set_max(3, ctx)
            // This happens in: less_than_or_equals(y.next(), v0)
            // which calls: y.next().try_set_max(v0.max(ctx), ctx)
            m.less_than_or_equals(y.next(), v0);
            
            // Let's see if this works
            m.solve()
        };
        println!("less_than_or_equals(y.next(), v0) with v0=[3,3]: {}", sol_test.is_some());
        
        // Now let's test what greater_than actually creates
        println!("\n=== What greater_than actually creates ===");
        
        // greater_than(v0, y) should create greater_than_or_equals(v0, y.next())
        // which should create less_than_or_equals(y.next(), v0)
        
        // But maybe there's a difference in how the view is constructed?
        let sol_direct = {
            let mut m = Model::default();
            let v0 = m.new_var_int(3, 3);
            
            // Test if the issue is in how greater_than constructs the Next view
            let y_view = y;  // This creates a constant view
            let y_next_view = y_view.next();  // This creates Next<ConstantView<ValF>>
            m.greater_than_or_equals(v0, y_next_view);
            m.solve()
        };
        println!("greater_than_or_equals(v0, y_view.next()): {}", sol_direct.is_some());
    }
}
