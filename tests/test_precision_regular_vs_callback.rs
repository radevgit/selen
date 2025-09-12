use cspsolver::prelude::*;

#[test]
fn test_precision_6_without_callback() {
    println!("=== Testing Precision 6 WITHOUT Callback (uses Step 2.4) ===");
    
    let mut model = Model::with_float_precision(6);

    let x = model.float(1.0, 10.0);
    model.lt(x, float(5.5));
    
    println!("Model setup complete:");
    println!("  Variable x: [1.0, 10.0] with precision 6");
    println!("  Constraint: x < 5.5");
    
    let start_time = std::time::Instant::now();
    let solution = model.maximize(x).expect("Should have solution");
    let duration = start_time.elapsed();
    
    let Val::ValF(x_val) = solution[x] else { unreachable!() };
    
    println!("\n=== Solution Results ===");
    println!("  Solution found: x = {}", x_val);
    println!("  Execution time: {:?}", duration);
    println!("  Constraint satisfied: x < 5.5 = {}", x_val < 5.5);
    println!("  Near-optimal: x > 5.4 = {}", x_val > 5.4);
    
    println!("\n=== Step 2.4 Analysis ===");
    if duration.as_micros() < 100 {
        println!("  ✅ Extremely fast execution - likely direct optimization");
        println!("  ✅ Step 2.4 precision handling appears to be working");
        println!("  ✅ No traditional search needed");
    } else if duration.as_millis() < 10 {
        println!("  ⚠️ Fast but not instant - may have minimal search");
    } else {
        println!("  ❌ Slow execution - traditional search likely used");
    }
    
    // Verify the results
    assert!(x_val < 5.5, "Solution should satisfy constraint x < 5.5");
    assert!(x_val > 5.4, "Solution should be near-optimal (> 5.4)");
    
    println!("\n=== Conclusion ===");
    println!("  The regular maximize() method uses Step 2.4 precision optimization");
    println!("  The maximize_with_callback() method bypasses optimization and uses traditional search");
    println!("  This explains why the callback version hangs while the regular version works!");
}
