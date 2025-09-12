use cspsolver::prelude::*;

#[test]
fn test_precision_6_detailed_metrics() {
    println!("=== Testing Precision 6 with Detailed Metrics ===");
    
    let mut model = Model::with_float_precision(6);

    let x = model.float(1.0, 10.0);
    model.lt(x, float(5.5));
    
    println!("Model setup complete:");
    println!("  Variable x: [1.0, 10.0] with precision 6");
    println!("  Constraint: x < 5.5");
    
    let start_time = std::time::Instant::now();
    
    // Use maximize_with_callback to capture statistics
    let mut captured_stats = None;
    let solution = model.maximize_with_callback(x, |stats| {
        captured_stats = Some((stats.node_count, stats.propagation_count));
    }).expect("Should have solution");
    
    let duration = start_time.elapsed();
    let (node_count, propagation_count) = captured_stats.expect("Stats should be captured");
    
    let Val::ValF(x_val) = solution[x] else { unreachable!() };
    
    println!("\n=== Solution Results ===");
    println!("  Solution found: x = {}", x_val);
    println!("  Execution time: {:?}", duration);
    println!("  Constraint satisfied: x < 5.5 = {}", x_val < 5.5);
    println!("  Near-optimal: x > 5.4 = {}", x_val > 5.4);
    
    println!("\n=== Search Statistics ===");
    println!("  Nodes explored: {}", node_count);
    println!("  Propagations: {}", propagation_count);
    
    // Verify the results
    assert!(x_val < 5.5, "Solution should satisfy constraint x < 5.5");
    assert!(x_val > 5.4, "Solution should be near-optimal (> 5.4)");
    
    println!("\n=== Performance Analysis ===");
    if node_count == 0 {
        println!("  ✅ Direct optimization - no search nodes needed!");
        println!("  ✅ Step 2.4 precision handling working perfectly");
    } else {
        println!("  Search nodes used: {}", node_count);
        if node_count < 10 {
            println!("  ✅ Very efficient - minimal search required");
        } else if node_count < 100 {
            println!("  ⚠️ Moderate search - could be optimized");
        } else {
            println!("  ❌ High search overhead - optimization needed");
        }
    }
    
    if propagation_count < 10 {
        println!("  ✅ Minimal propagations - excellent efficiency");
    } else if propagation_count < 100 {
        println!("  ⚠️ Moderate propagations - acceptable performance");
    } else {
        println!("  ❌ High propagation count - optimization needed");
    }
}
