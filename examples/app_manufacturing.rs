use cspsolver::benchmarks::manufacturing_constraints::*;

fn main() {
    println!("🏭 Manufacturing Constraints Demo");
    println!("=================================");
    println!("Testing realistic CNC tool clearance optimization...\n");
    
    let result = benchmark_tool_clearance_constraints();
    
    println!("=== {} ===", result.constraint_type);
    println!("Problem Scale: {}", result.scale);
    println!("Duration: {:.2} seconds ({} μs)", 
             result.duration.as_secs_f64(), 
             result.duration.as_micros());
    println!("Success: {}", if result.success { "✅ SOLVED" } else { "❌ FAILED" });
    println!("Feasibility Score: {:.1}/10", result.feasibility_score);
    
    // Performance classification
    let seconds = result.duration.as_secs_f64();
    let performance_class = if seconds < 0.5 { "⚡ Real-time capable" }
                           else if seconds < 5.0 { "🚀 CAM software ready" }
                           else if seconds < 30.0 { "📊 Production planning ready" }
                           else { "⏰ Offline optimization only" };
    
    println!("Performance: {}", performance_class);
    
    if result.success {
        println!("\n🎯 Manufacturing Analysis:");
        if result.feasibility_score > 9.0 {
            println!("   ✅ Excellent - Ready for production deployment");
            println!("   💰 Cost savings: High precision, minimal rework");
            println!("   ⚡ Speed: Optimized tool paths reduce cycle time");
        } else if result.feasibility_score > 8.0 {
            println!("   ⚠️  Good - Minor parameter tuning recommended");
            println!("   🔧 Requires: Tool path validation and testing");
        } else {
            println!("   ❌ Needs improvement - Constraint refinement required");
        }
        
        if seconds > 5.0 {
            println!("\n⏱️  Performance Note:");
            println!("   Solving took {:.1}s - this is a complex 3D tool clearance problem", seconds);
            println!("   with 50 tool positions and hundreds of distance constraints.");
            println!("   In production: Pre-compute solutions for common part geometries.");
        }
    } else {
        println!("\n❌ Problem Analysis:");
        println!("   The constraints are too restrictive or conflicting.");
        println!("   In manufacturing: Adjust tool parameters, workpiece size,");
        println!("   or clearance requirements to find feasible solution.");
    }
    
    println!("\n📈 Benchmark completed!");
}