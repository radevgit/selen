use cspsolver::prelude::*;
use cspsolver::runtime_api::{VarIdExt, ModelExt};

fn main() {
    println!("🔍 Demonstrating Enhanced Statistics Output");
    println!("{}", "=".repeat(50));
    
    // Example 1: Basic doctest example
    println!("\n📊 Example 1: Basic Enhanced Statistics");
    println!("{}", "-".repeat(40));
    
    let mut m = Model::default();
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    post!(m, x + y == int(15));

    // Solve and get solution with enhanced statistics
    let solution = m.solve().unwrap();

    // Access solution values
    println!("x = {:?}", solution[x]);
    println!("y = {:?}", solution[y]);

    // Access all enhanced statistics fields
    let stats = &solution.stats;
    println!("Propagations: {}", stats.propagation_count);
    println!("Search nodes: {}", stats.node_count);
    println!("Solve time: {:.3}ms", stats.solve_time.as_secs_f64() * 1000.0);
    println!("Peak memory usage: {}MB", stats.peak_memory_mb);
    println!("Problem size: {} variables, {} constraints", 
             stats.variable_count, stats.constraint_count);

    // Use convenience analysis methods
    println!("Search efficiency: {:.1} propagations/node", stats.efficiency());
    println!("Time per propagation: {:.2}μs", 
             stats.time_per_propagation().as_nanos() as f64 / 1000.0);
    println!("Time per search node: {:.2}μs", 
             stats.time_per_node().as_nanos() as f64 / 1000.0);

    // Display comprehensive summary
    stats.display_summary();

    // Example 2: Runtime API doctest example
    println!("\n📊 Example 2: Runtime API Enhanced Statistics");
    println!("{}", "-".repeat(45));

    let mut m = Model::default();
    let x = m.int(1, 10);
    let y = m.int(1, 10);

    // Build constraints programmatically
    m.post(x.add(y).eq(15));
    m.post(x.mul(2).le(y));

    // Solve and access solution with comprehensive statistics
    let solution = m.solve().unwrap();
    println!("x = {:?}, y = {:?}", solution[x], solution[y]);

    // Access all enhanced statistics fields
    let stats = &solution.stats;
    println!("Core metrics:");
    println!("  Propagations: {}", stats.propagation_count);
    println!("  Search nodes: {}", stats.node_count);
    
    println!("Performance metrics:");
    println!("  Solve time: {:.3}ms", stats.solve_time.as_secs_f64() * 1000.0);
    println!("  Peak memory: {}MB", stats.peak_memory_mb);
    
    println!("Problem characteristics:");
    println!("  Variables: {}", stats.variable_count);
    println!("  Constraints: {}", stats.constraint_count);
    
    // Use all convenience analysis methods
    if stats.node_count > 0 {
        println!("Efficiency analysis:");
        println!("  {:.2} propagations/node", stats.efficiency());
        println!("  {:.2}μs/propagation", stats.time_per_propagation().as_nanos() as f64 / 1000.0);
        println!("  {:.2}μs/node", stats.time_per_node().as_nanos() as f64 / 1000.0);
    }
    
    // Display complete formatted summary
    stats.display_summary();
    
    // Create statistics manually using constructor
    println!("\n📊 Example 3: Manual Statistics Construction");
    println!("{}", "-".repeat(45));
    let custom_stats = SolveStats::new(100, 10, 
        std::time::Duration::from_millis(5), 3, 20, 15, 256);
    println!("Custom stats efficiency: {:.1}", custom_stats.efficiency());
    custom_stats.display_summary();
}