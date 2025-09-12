use cspsolver::prelude::*;

#[test]
fn trace_simple_cases() {
    println!("=== BASELINE: Simple constraint without optimization ===");
    let mut model = Model::with_float_precision(4);
    let x = model.float(1.0, 10.0);
    model.le(x, float(5.5)); // Use <= instead of <
    
    let solution = model.solve_with_callback(|stats| {
        println!("Simple solve - Propagations: {}, Nodes: {}", 
                stats.propagation_count, stats.node_count);
    });
    
    if let Some(sol) = solution {
        let Val::ValF(x_val) = sol[x] else { panic!("Expected float") };
        println!("Simple solve result: x = {}", x_val);
    }
    
    println!("\n=== BASELINE: Simple maximization with <= constraint ===");
    let mut model2 = Model::with_float_precision(4);
    let x2 = model2.float(1.0, 10.0);
    model2.le(x2, float(5.5)); // Use <= instead of <
    
    let solution2 = model2.maximize_with_callback(x2, |stats| {
        println!("Simple maximize <= - Propagations: {}, Nodes: {}", 
                stats.propagation_count, stats.node_count);
    });
    
    if let Some(sol) = solution2 {
        let Val::ValF(x_val) = sol[x2] else { panic!("Expected float") };
        println!("Simple maximize <= result: x = {}", x_val);
    }
    
    println!("\n=== PROBLEMATIC: Maximization with < constraint ===");
    let mut model3 = Model::with_float_precision(4);
    let x3 = model3.float(1.0, 10.0);
    model3.lt(x3, float(5.5)); // Use < (this creates Next view)
    
    let solution3 = model3.maximize_with_callback(x3, |stats| {
        println!("Problematic maximize < - Propagations: {}, Nodes: {}", 
                stats.propagation_count, stats.node_count);
    });
    
    if let Some(sol) = solution3 {
        let Val::ValF(x_val) = sol[x3] else { panic!("Expected float") };
        println!("Problematic maximize < result: x = {}", x_val);
    }
}
