use selen::prelude::*;

fn main() {
    println!("🔍 Testing Enhanced Statistics");
    
    // Simple constraint satisfaction problem
    let mut m = Model::default();
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    post!(m, x + y == int(15));
    post!(m, x < y);
    
    println!("Solving simple problem: x + y = 15, x < y");
    
    match m.solve() {
        Ok(solution) => {
            println!("✅ Solution found!");
            println!("   x = {:?}", solution[x]);
            println!("   y = {:?}", solution[y]);
            
            // Display enhanced statistics
            let stats = &solution.stats;
            println!("\n📊 Enhanced Statistics:");
            println!("   Propagations: {}", stats.propagation_count);
            println!("   Search nodes: {}", stats.node_count);
            println!("   Solve time: {:.3}ms", stats.solve_time.as_secs_f64() * 1000.0);
            println!("   Peak memory: {}MB", stats.peak_memory_mb);
            println!("   Problem size: {} variables, {} constraints", 
                     stats.variable_count, stats.constraint_count);
            
            // Test convenience methods
            if stats.node_count > 0 {
                println!("   Efficiency: {:.2} propagations/node", stats.efficiency());
                println!("   Time per node: {:.2}μs", stats.time_per_node().as_nanos() as f64 / 1000.0);
            } else {
                println!("   Efficiency: Pure propagation (no search required)");
            }
            
            if stats.propagation_count > 0 {
                println!("   Time per propagation: {:.2}μs", 
                         stats.time_per_propagation().as_nanos() as f64 / 1000.0);
            }
            
            // Test display_summary method
            println!("\n📈 Full Summary:");
            stats.display_summary();
        }
        Err(e) => {
            println!("❌ Failed to solve: {:?}", e);
        }
    }
    
    // Test enumeration with enhanced stats
    println!("\n🔍 Testing Enumeration with Enhanced Statistics");
    
    let mut m2 = Model::default();
    let a = m2.int(1, 3);
    let b = m2.int(1, 3);
    post!(m2, a + b <= int(4));
    
    let (solutions, final_stats) = m2.enumerate_with_stats();
    
    println!("✅ Found {} solutions", solutions.len());
    println!("📊 Final enumeration statistics:");
    final_stats.display_summary();
    
    // Show individual solution stats
    for (i, sol) in solutions.iter().enumerate() {
        println!("Solution {}: a={:?}, b={:?}", i+1, sol[a], sol[b]);
    }
}