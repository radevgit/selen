/// Demonstration of Solution with embedded statistics API
/// 
/// This example shows how the new embedded statistics API makes it easy to access
/// solving statistics directly from solutions without callbacks.

use cspsolver::prelude::*;

fn main() -> SolverResult<()> {
    println!("🧪 Solution with Embedded Statistics API Demo");
    println!("==============================================\n");

    // Test solve() with embedded statistics
    test_solve_with_embedded_stats()?;
    println!();

    // Test minimize() with embedded statistics 
    test_minimize_with_embedded_stats()?;
    println!();

    // Test maximize() with embedded statistics
    test_maximize_with_embedded_stats()?;
    println!();

    // Test enumerate() with embedded statistics
    test_enumerate_with_embedded_stats();

    println!("\n✅ All tests passed! The new embedded statistics API works perfectly.");
    Ok(())
}

fn test_solve_with_embedded_stats() -> SolverResult<()> {
    println!("🔍 Testing solve() with embedded statistics");
    
    let mut m = Model::default();
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    post!(m, x + y == int(15));
    
    let solution = m.solve()?;
    
    println!("   Solution: x={:?}, y={:?}", solution[x], solution[y]);
    println!("   Statistics: {} propagations, {} nodes", 
             solution.stats.propagation_count, solution.stats.node_count);
    println!("   ✅ Statistics accessible directly from solution");
    
    Ok(())
}

fn test_minimize_with_embedded_stats() -> SolverResult<()> {
    println!("🎯 Testing minimize() with embedded statistics");
    
    let mut m = Model::default();
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    post!(m, x + y >= int(5));
    post!(m, x <= y);
    
    let solution = m.minimize(x)?;
    
    println!("   Optimal solution: x={:?}, y={:?}", solution[x], solution[y]);
    println!("   Statistics: {} propagations, {} nodes", 
             solution.stats.propagation_count, solution.stats.node_count);
    println!("   ✅ Optimization statistics embedded in solution");
    
    Ok(())
}

fn test_maximize_with_embedded_stats() -> SolverResult<()> {
    println!("📈 Testing maximize() with embedded statistics");
    
    let mut m = Model::default();
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    post!(m, x + y <= int(15));
    post!(m, x >= y);
    
    let solution = m.maximize(x)?;
    
    println!("   Optimal solution: x={:?}, y={:?}", solution[x], solution[y]);
    println!("   Statistics: {} propagations, {} nodes", 
             solution.stats.propagation_count, solution.stats.node_count);
    println!("   ✅ Maximization statistics embedded in solution");
    
    Ok(())
}

fn test_enumerate_with_embedded_stats() {
    println!("📝 Testing enumerate() with embedded statistics");
    
    let mut m = Model::default();
    let x = m.int(1, 3);
    let y = m.int(1, 3);
    post!(m, x != y);
    
    let solutions: Vec<_> = m.enumerate().collect();
    
    println!("   Found {} solutions", solutions.len());
    
    // Each solution has its own embedded statistics
    for (i, solution) in solutions.iter().take(3).enumerate() {
        println!("   Solution {}: x={:?}, y={:?} (stats: {} propagations, {} nodes)",
                 i + 1, solution[x], solution[y], 
                 solution.stats.propagation_count, solution.stats.node_count);
    }
    
    println!("   ✅ Each solution has embedded statistics - much cleaner than callbacks!");
}