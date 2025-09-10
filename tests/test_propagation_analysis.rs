use cspsolver::prelude::*;

#[test]
fn test_direct_constraint_propagation() {
    println!("=== TESTING DIRECT CONSTRAINT PROPAGATION (NO SEARCH) ===");
    
    // Test 1: Simple <= constraint should require minimal propagation
    let mut model = Model::with_float_precision(4);
    let x = model.new_var_float(1.0, 10.0);
    model.less_than_or_equals(x, float(5.5));
    
    let solution = model.solve_with_callback(|stats| {
        println!("Direct <= constraint - Propagations: {}, Nodes: {}", 
                stats.propagation_count, stats.node_count);
    });
    
    if let Some(sol) = solution {
        let Val::ValF(x_val) = sol[x] else { panic!("Expected float") };
        println!("Direct <= result: x can be up to {}", x_val);
        // Should be able to find any valid value, not necessarily the maximum
    }
    
    println!("\n=== TESTING MAXIMIZE (REQUIRES SEARCH) ===");
    
    // Test 2: Maximization requires search - this is where propagations should come from
    let mut model2 = Model::with_float_precision(4);
    let x2 = model2.new_var_float(1.0, 10.0);
    model2.less_than_or_equals(x2, float(5.5));
    
    let solution2 = model2.maximize_with_callback(x2, |stats| {
        println!("Maximize <= constraint - Propagations: {}, Nodes: {}", 
                stats.propagation_count, stats.node_count);
    });
    
    if let Some(sol) = solution2 {
        let Val::ValF(x_val) = sol[x2] else { panic!("Expected float") };
        println!("Maximize <= result: x = {}", x_val);
    }
    
    println!("\n=== TESTING < CONSTRAINT (THE PROBLEMATIC ONE) ===");
    
    // Test 3: < constraint with maximization (the hanging case)
    let mut model3 = Model::with_float_precision(4);
    let x3 = model3.new_var_float(1.0, 10.0);
    model3.less_than(x3, float(5.5)); // This creates Next view
    
    let solution3 = model3.maximize_with_callback(x3, |stats| {
        println!("Maximize < constraint - Propagations: {}, Nodes: {}", 
                stats.propagation_count, stats.node_count);
    });
    
    if let Some(sol) = solution3 {
        let Val::ValF(x_val) = sol[x3] else { panic!("Expected float") };
        println!("Maximize < result: x = {}", x_val);
    }
}
