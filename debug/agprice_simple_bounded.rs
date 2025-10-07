// Simplified agprice with bounded variables - using maximize() directly
use selen::prelude::*;
use selen::variables::Val;

fn main() {
    use selen::utils::config::SolverConfig;
    let config = SolverConfig {
        timeout_ms: Some(120_000), // 120 second timeout
        max_memory_mb: Some(4096),
        ..Default::default()
    };
    let mut model = Model::with_config(config);

    println!("Creating bounded agprice model...");
    
    // Use reasonable bounds based on problem domain
    let bound = 10000.0;
    
    // Main decision variables
    let milk = model.float(-bound, bound);
    let butt = model.float(-bound, bound);
    let cha = model.float(-bound, bound);
    let chb = model.float(-bound, bound);
    let xm = model.float(-bound, bound);
    let xb = model.float(-bound, bound);
    let xca = model.float(-bound, bound);
    let xcb = model.float(-bound, bound);
    let q = model.float(-bound, bound);
    
    // Squared variables
    let milksq = model.float(-bound, bound);
    let buttsq = model.float(-bound, bound);
    let chasq = model.float(-bound, bound);
    let chbsq = model.float(-bound, bound);
    let qsq = model.float(-bound, bound);
    
    // Revenue variable
    let revenue = model.float(-bound, bound);
    
    println!("Adding constraints...");
    
    // Non-negativity constraints (from original MiniZinc)
    model.float_lin_le(&vec![-1.0], &vec![milk], 0.0);    // milk >= 0
    model.float_lin_le(&vec![-1.0], &vec![milksq], 0.0);  // milksq >= 0
    model.float_lin_le(&vec![-1.0], &vec![butt], 0.0);    // butt >= 0
    model.float_lin_le(&vec![-1.0], &vec![buttsq], 0.0);  // buttsq >= 0
    model.float_lin_le(&vec![-1.0], &vec![cha], 0.0);     // cha >= 0
    model.float_lin_le(&vec![-1.0], &vec![chasq], 0.0);   // chasq >= 0
    model.float_lin_le(&vec![-1.0], &vec![chb], 0.0);     // chb >= 0
    model.float_lin_le(&vec![-1.0], &vec![chbsq], 0.0);   // chbsq >= 0
    model.float_lin_le(&vec![-1.0], &vec![xm], 0.0);      // xm >= 0
    model.float_lin_le(&vec![-1.0], &vec![xb], 0.0);      // xb >= 0
    model.float_lin_le(&vec![-1.0], &vec![xca], 0.0);     // xca >= 0
    model.float_lin_le(&vec![-1.0], &vec![xcb], 0.0);     // xcb >= 0
    model.float_lin_le(&vec![-1.0], &vec![qsq], 0.0);     // qsq >= 0
    
    // Resource constraints (critical for bounding!)
    model.float_lin_le(&vec![0.35, 0.8, 0.04, 0.25], &vec![xca, xb, xm, xcb], 0.6);
    model.float_lin_le(&vec![0.3, 0.02, 0.09, 0.4], &vec![xca, xb, xm, xcb], 0.75);
    model.float_lin_le(&vec![0.21, 0.32, 4.82, 0.07], &vec![cha, butt, milk, chb], 1.939);
    
    // Balance equations
    model.float_lin_eq(&vec![0.2074688796680498, 1.346801346801347], &vec![xm, milk], 1.4);
    model.float_lin_eq(&vec![3.125, 3.75], &vec![xb, butt], 3.7);
    model.float_lin_eq(&vec![1.047619047619048, 4.761904761904762, -0.1226993865030675], &vec![cha, xca, chb], 2.0);
    model.float_lin_eq(&vec![0.49079754601227, 14.28571428571428, -0.3809523809523809], &vec![chb, xcb, cha], 1.0);
    model.float_lin_eq(&vec![-1.0, 1.0, -0.195], &vec![chb, cha, q], 0.0);
    
    // Revenue equation
    model.float_lin_eq(&vec![420.0, 1185.0, 6748.0, -1.0, -8.0, -194.0, -1200.0, -6492.0, 70.0, -1.0], 
                       &vec![cha, butt, milk, qsq, chbsq, chasq, buttsq, milksq, chb, revenue], 0.0);
    
    println!("Solving with maximize()...");
    let start = std::time::Instant::now();
    
    match model.maximize(revenue) {
        Ok(solution) => {
            let elapsed = start.elapsed();
            println!("\n✅ Solution found in {:.2}s!", elapsed.as_secs_f64());
            println!("===================\n");
            
            let rev = match solution[revenue] { Val::ValF(v) => v, _ => 0.0 };
            println!("  revenue = {}", rev);
            println!();
            
            // Key variables
            println!("Key Variables:");
            if let Val::ValF(v) = solution[cha] { println!("  cha (cheese 1 price) = {:.4}", v); }
            if let Val::ValF(v) = solution[chb] { println!("  chb (cheese 2 price) = {:.4}", v); }
            if let Val::ValF(v) = solution[milk] { println!("  milk (price) = {:.4}", v); }
            if let Val::ValF(v) = solution[butt] { println!("  butt (butter price) = {:.4}", v); }
            println!();
            
            println!("Production:");
            if let Val::ValF(v) = solution[xm] { println!("  xm (milk prod) = {:.4}", v); }
            if let Val::ValF(v) = solution[xb] { println!("  xb (butter prod) = {:.4}", v); }
            if let Val::ValF(v) = solution[xca] { println!("  xca (cheese 1 prod) = {:.4}", v); }
            if let Val::ValF(v) = solution[xcb] { println!("  xcb (cheese 2 prod) = {:.4}", v); }
        }
        Err(e) => {
            println!("\n❌ No solution found: {:?}", e);
        }
    }
}
