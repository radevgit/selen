/// Simple Selen test to investigate why int_variables shows 0
/// This tests the problem directly: create 2 int variables and solve

use selen::prelude::*;

fn main() {
    println!("=== Testing Selen Statistics for Integer Variables ===\n");

    let mut model = Model::default();

    // Create 2 integer variables with domain [1..10]
    let x = model.int(1, 10);
    let y = model.int(1, 10);

    println!("Created 2 integer variables: x and y in [1..10]");

    // Add constraint: x + y = 15
    let sum = x.add(y);
    let target = model.int(15, 15);
    model.new(sum.eq(target));
    
    println!("Added constraint: x + y = 15");

    // Solve
    match model.solve() {
        Ok(solution) => {
            println!("\n=== Solution ===");
            println!("x = {:?}", solution[x]);
            println!("y = {:?}", solution[y]);

            println!("\n=== Statistics ===");
            let stats = solution.stats();
            println!("variables: {}", stats.variables);
            println!("int_variables: {} (EXPECTED: 2)", stats.int_variables);
            println!("bool_variables: {}", stats.bool_variables);
            println!("float_variables: {}", stats.float_variables);
            println!("set_variables: {}", stats.set_variables);
            println!("propagators: {}", stats.propagators);
            println!("propagation_count: {}", stats.propagation_count);
            println!("node_count: {}", stats.node_count);
            println!("constraint_count: {}", stats.constraint_count);
            println!("objective: {}", stats.objective);
            println!("objective_bound: {}", stats.objective_bound);
            println!("init_time: {:.6}s", stats.init_time.as_secs_f64());
            println!("solve_time: {:.6}s", stats.solve_time.as_secs_f64());
            println!("peak_memory_mb: {} MB", stats.peak_memory_mb);
            
            if stats.int_variables == 0 {
                println!("\n⚠️  ISSUE FOUND: int_variables is 0 even though we created 2 int variables!");
                println!("This needs to be fixed in Selen's SolveStats tracking.");
            }
        }
        Err(e) => {
            println!("No solution found: {:?}", e);
        }
    }
}
