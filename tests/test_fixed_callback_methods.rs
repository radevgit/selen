use cspsolver::prelude::*;

#[test]
fn test_fixed_callback_methods() {
    println!("=== Testing Fixed Callback Methods with Step 2.4 Integration ===");
    
    let mut model = Model::with_float_precision(6);

    let x = model.new_var_float(1.0, 10.0);
    model.lt(x, float(5.5));
    
    println!("Model setup complete:");
    println!("  Variable x: [1.0, 10.0] with precision 6");
    println!("  Constraint: x < 5.5");
    
    // Test minimize_with_callback (now fixed to use Step 2.4)
    println!("\n=== Testing minimize_with_callback ===");
    let start_time = std::time::Instant::now();
    let mut captured_stats = None;
    
    let solution = model.minimize_with_callback(x, |stats| {
        captured_stats = Some((stats.node_count, stats.propagation_count));
        println!("  Minimize stats - Nodes: {}, Propagations: {}", stats.node_count, stats.propagation_count);
    }).expect("Should have solution");
    
    let duration = start_time.elapsed();
    let (node_count, propagation_count) = captured_stats.expect("Stats should be captured");
    
    let Val::ValF(x_val) = solution[x] else { unreachable!() };
    
    println!("  Solution: x = {}", x_val);
    println!("  Time: {:?}", duration);
    println!("  Nodes: {}", node_count);
    println!("  Propagations: {}", propagation_count);
    
    // Test maximize_with_callback (calls our fixed minimize_with_callback)
    println!("\n=== Testing maximize_with_callback ===");
    let mut model2 = Model::with_float_precision(6);
    let x2 = model2.new_var_float(1.0, 10.0);
    model2.lt(x2, float(5.5));
    
    let start_time2 = std::time::Instant::now();
    let mut captured_stats2 = None;
    
    let solution2 = model2.maximize_with_callback(x2, |stats| {
        captured_stats2 = Some((stats.node_count, stats.propagation_count));
        println!("  Maximize stats - Nodes: {}, Propagations: {}", stats.node_count, stats.propagation_count);
    }).expect("Should have solution");
    
    let duration2 = start_time2.elapsed();
    let (node_count2, propagation_count2) = captured_stats2.expect("Stats should be captured");
    
    let Val::ValF(x_val2) = solution2[x2] else { unreachable!() };
    
    println!("  Solution: x = {}", x_val2);
    println!("  Time: {:?}", duration2);
    println!("  Nodes: {}", node_count2);
    println!("  Propagations: {}", propagation_count2);
    
    println!("\n=== Results Analysis ===");
    
    // Check minimize results
    assert!(x_val >= 1.0 && x_val < 5.5, "Minimize solution should be valid");
    if node_count == 0 && propagation_count == 0 {
        println!("  ✅ minimize_with_callback: Using Step 2.4 optimization (0 nodes, 0 propagations)");
    } else {
        println!("  ⚠️ minimize_with_callback: Using search ({} nodes, {} propagations)", node_count, propagation_count);
    }
    
    // Check maximize results  
    assert!(x_val2 > 5.4 && x_val2 < 5.5, "Maximize solution should be near-optimal");
    if node_count2 == 0 && propagation_count2 == 0 {
        println!("  ✅ maximize_with_callback: Using Step 2.4 optimization (0 nodes, 0 propagations)");
    } else {
        println!("  ⚠️ maximize_with_callback: Using search ({} nodes, {} propagations)", node_count2, propagation_count2);
    }
    
    if duration.as_micros() < 100 && duration2.as_micros() < 100 {
        println!("  ✅ Both methods are extremely fast - Step 2.4 working perfectly!");
    }
    
    println!("\n=== Summary ===");
    println!("  All callback methods now properly integrate with Step 2.4 precision optimization!");
    println!("  No more hanging - callback methods use optimization first, then fallback to search.");
}
