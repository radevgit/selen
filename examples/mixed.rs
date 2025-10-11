use selen::prelude::*;

fn main() {
    let mut m = Model::default();

    // Integer variables
    let wn = m.int(0, 100);
    let hn = m.int(0, 100);
    
    // Float constants as variables (singleton domains)
    let wp = m.float(200.0, 200.0);
    let hp = m.float(300.0, 300.0);
    let wpad = m.float(30.0, 30.0);
    let hpad = m.float(23.0, 23.0);
    let w = m.float(3500.0, 3500.0);
    let h = m.float(2500.0, 2500.0);

    // Constraint: wn*wp + 2*wpad <= w
    m.new(wn.mul(wp).add(wpad.mul(2.0)).le(w));
    
    // Constraint: hn*hp + 2*hpad <= h
    m.new(hn.mul(hp).add(hpad.mul(2.0)).le(h));
    
    // Create objective: maximize wn + hn
    let objective = m.int(0, 200);  // Sum of wn (0-100) + hn (0-100)
    m.new(wn.add(hn).eq(objective));
    
    // Solve the model with maximization
    match m.maximize(objective) {
        Ok(sol) => {
            println!("Optimal solution found!");
            println!("wn = {:?}", sol[wn]);
            println!("hn = {:?}", sol[hn]);
            println!("Total (wn + hn) = {:?}", sol[objective]);
            println!("\nSolver Statistics:");
            println!("==================");
            let stats = sol.stats();
            println!("Propagation count: {}", stats.propagation_count);
            println!("Node count: {}", stats.node_count);
            println!("Solve time: {:?}", stats.solve_time);
            println!("Variable count: {}", stats.variable_count);
            println!("Constraint count: {}", stats.constraint_count);
            println!("Peak memory (MB): {}", stats.peak_memory_mb);
            println!("LP solver used: {}", stats.lp_solver_used);
            println!("LP constraint count: {}", stats.lp_constraint_count);
            if let Some(ref lp_stats) = stats.lp_stats {
                println!("\nLP Solver Statistics:");
                println!("  Total solve time: {:.2}ms", lp_stats.solve_time_ms);
                println!("  Phase I time: {:.2}ms", lp_stats.phase1_time_ms);
                println!("  Phase II time: {:.2}ms", lp_stats.phase2_time_ms);
                println!("  Phase I iterations: {}", lp_stats.phase1_iterations);
                println!("  Phase II iterations: {}", lp_stats.phase2_iterations);
                println!("  Variables: {}", lp_stats.n_variables);
                println!("  Constraints: {}", lp_stats.n_constraints);
            }
        }
        Err(e) => {
            println!("No solution found: {:?}", e);
        }
    }
}
