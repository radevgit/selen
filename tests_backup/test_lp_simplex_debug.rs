// Debug test for LP simplex solver
use selen::lpsolver::{LpProblem, solve};

#[test]
fn test_simplex_simple_problem() {
    // Just test the actual large domain test now that we fixed to_lp_problem
    // The real test will use extract_linear_system which uses to_lp_problem internally
    println!("This test is superseded by test_large_domain_optimization_linear");
    println!("Skipping direct LP test - run the full CSP test instead");
}

#[test]
fn test_simplex_even_simpler() {
    // Even simpler: just x + y <= 10, x >= 0, y >= 0
    let n_vars = 2;
    let n_constraints = 1;
    
    let c = vec![0.0, 0.0];
    let a = vec![
        vec![1.0, 1.0],  // x + y <= 10
    ];
    let b = vec![10.0];
    
    let lower_bounds = vec![0.0, 0.0];
    let upper_bounds = vec![f64::INFINITY, f64::INFINITY];
    
    let problem = LpProblem::new(n_vars, n_constraints, c, a, b, lower_bounds, upper_bounds);
    
    let solution = solve(&problem).expect("Should find a solution");
    
    println!("Simple problem solution: {:?}", solution.x);
    println!("Status: {:?}", solution.status);
    
    let x = solution.x[0];
    let y = solution.x[1];
    
    println!("x = {}, y = {}, x + y = {}", x, y, x + y);
    assert!(x + y <= 10.0 + 1e-3, "Constraint violated: {} + {} = {} > 10", x, y, x + y);
}
