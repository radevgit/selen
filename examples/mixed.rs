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
            
            // Extract objective value from solution
            let obj_value = match sol[objective] {
                Val::ValI(v) => v as f64,
                Val::ValF(v) => v,
            };
            
            println!("\n========== Solver Statistics ==========\n");
            let stats = sol.stats();
            
            println!("Propagations: {}", stats.propagation_count);
            println!("Nodes: {}", stats.node_count);
            println!("Objective: {}", obj_value);
            println!("Objective bound: {}", stats.objective_bound);
            
            println!("Total variables: {}", stats.variables);
            println!("Integer variables: {}", stats.int_variables);
            println!("Boolean variables: {}", stats.bool_variables);
            println!("Float variables: {}", stats.float_variables);
            
            println!("Constraints: {}", stats.constraint_count);
            println!("Propagators: {}", stats.propagators);
            
            println!("Solve time (ms): {:.3}", stats.solve_time.as_secs_f64() * 1000.0);
            println!("Init time (ms): {:.3}", stats.init_time.as_secs_f64() * 1000.0);
            println!("Peak memory (MB): {}", stats.peak_memory_mb);
            
            println!("LP solver used: {}", stats.lp_solver_used);
            println!("LP constraints: {}", stats.lp_constraint_count);
            println!("LP variables: {}", stats.lp_variable_count);
            
            if let Some(ref lp_stats) = stats.lp_stats {
                println!("LP solve time (ms): {:.2}", lp_stats.solve_time_ms);
                println!("LP phase1 time (ms): {:.2}", lp_stats.phase1_time_ms);
                println!("LP phase2 time (ms): {:.2}", lp_stats.phase2_time_ms);
                println!("LP phase1 iterations: {}", lp_stats.phase1_iterations);
                println!("LP phase2 iterations: {}", lp_stats.phase2_iterations);
                println!("LP factorizations: {}", lp_stats.factorizations);
                println!("LP n_variables: {}", lp_stats.n_variables);
                println!("LP n_constraints: {}", lp_stats.n_constraints);
                println!("LP peak memory (MB): {:.2}", lp_stats.peak_memory_mb);
                println!("LP phase1 needed: {}", lp_stats.phase1_needed);
            }
            
            println!("\n======================================\n");
        }
        Err(e) => {
            println!("No solution found: {:?}", e);
        }
    }
}
