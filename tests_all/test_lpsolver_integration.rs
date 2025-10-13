//! Integration tests for LP solver
//!
//! Tests complete workflows including warm-starting,
//! larger problems, and interaction between components.

use selen::lpsolver::{solve, solve_with_config, LpConfig, LpProblem, LpStatus};

#[test]
fn test_production_problem() {
    // Maximize profit from producing two products
    // Product A: profit = $30, requires 2 hours labor, 1 unit material
    // Product B: profit = $40, requires 1 hour labor, 2 units material
    // Constraints: 100 hours labor, 80 units material
    
    let problem = LpProblem::new(
        2,
        2,
        vec![30.0, 40.0],  // Profit coefficients
        vec![
            vec![2.0, 1.0],  // Labor constraint
            vec![1.0, 2.0],  // Material constraint
        ],
        vec![100.0, 80.0],
        vec![0.0, 0.0],
        vec![f64::INFINITY, f64::INFINITY],
    );
    
    let solution = solve(&problem).unwrap();
    
    assert_eq!(solution.status, LpStatus::Optimal);
    
    // Verify solution is feasible
    let x = &solution.x;
    assert!(x[0] >= 0.0);
    assert!(x[1] >= 0.0);
    
    // Check constraints
    assert!(2.0 * x[0] + 1.0 * x[1] <= 100.0 + 1e-6);
    assert!(1.0 * x[0] + 2.0 * x[1] <= 80.0 + 1e-6);
    
    // Objective should be reasonable
    assert!(solution.objective >= 0.0);
    assert!(solution.objective <= 30.0 * 50.0 + 40.0 * 50.0); // Upper bound
}

#[test]
fn test_diet_problem() {
    // Simplified diet problem as maximization (inverted)
    // Maximize negative cost = minimize cost
    // 3 foods with different costs and nutritional values
    // Each food provides calories and protein
    // Want to maximize nutrition within budget
    
    let problem = LpProblem::new(
        3,  // 3 foods
        3,  // 3 constraints: budget + 2 nutrition minimums
        vec![200.0, 150.0, 100.0],  // Nutrition value to maximize
        vec![
            vec![2.0, 3.0, 1.5],     // Cost constraint (budget <= 100)
            vec![1.0, 0.0, 0.0],     // Food 1 amount <= 50
            vec![0.0, 1.0, 0.0],     // Food 2 amount <= 40
        ],
        vec![100.0, 50.0, 40.0],
        vec![0.0, 0.0, 0.0],
        vec![f64::INFINITY, f64::INFINITY, f64::INFINITY],
    );
    
    let solution = solve(&problem).unwrap();
    
    // Should find optimal solution
    assert_eq!(solution.status, LpStatus::Optimal);
    assert!(solution.x.iter().all(|&x| x >= -1e-6), 
        "All variables should be non-negative, got: {:?}", solution.x);
    assert!(solution.objective >= 0.0);
}

#[test]
fn test_transportation_problem() {
    // Simple 2x2 transportation problem
    // 2 suppliers, 2 customers
    // Minimize transportation cost
    
    // Variables: x11, x12, x21, x22 (amount from supplier i to customer j)
    // Minimize: 2*x11 + 3*x12 + 4*x21 + 1*x22
    
    let problem = LpProblem::new(
        4,
        4,
        vec![-2.0, -3.0, -4.0, -1.0],  // Negative for minimization
        vec![
            vec![1.0, 1.0, 0.0, 0.0],   // Supplier 1 capacity <= 20
            vec![0.0, 0.0, 1.0, 1.0],   // Supplier 2 capacity <= 30
            vec![1.0, 0.0, 1.0, 0.0],   // Customer 1 demand <= 15
            vec![0.0, 1.0, 0.0, 1.0],   // Customer 2 demand <= 25
        ],
        vec![20.0, 30.0, 15.0, 25.0],
        vec![0.0, 0.0, 0.0, 0.0],
        vec![f64::INFINITY; 4],
    );
    
    let solution = solve(&problem).unwrap();
    
    assert_eq!(solution.status, LpStatus::Optimal);
    
    // All amounts should be non-negative
    assert!(solution.x.iter().all(|&x| x >= -1e-6));
}

#[test]
fn test_custom_tolerance() {
    // Test with custom tolerance settings using builder pattern
    let config = LpConfig::default()
        .with_timeout_ms(5000);
    
    let problem = LpProblem::new(
        2,
        1,
        vec![1.0, 1.0],
        vec![vec![1.0, 1.0]],
        vec![10.0],
        vec![0.0, 0.0],
        vec![f64::INFINITY, f64::INFINITY],
    );
    
    let solution = solve_with_config(&problem, &config).unwrap();
    
    assert_eq!(solution.status, LpStatus::Optimal);
    assert!((solution.objective - 10.0).abs() < 1e-7);
}

#[test]
fn test_medium_sized_problem() {
    // Test with 10 variables and 5 constraints
    // Maximize sum of all variables
    // Subject to: each pair sum <= 15 (5 pairs)
    // So maximum total is 5 * 15 = 75
    
    let n = 10;
    let m = 5;
    
    let c = vec![1.0; n];
    
    let mut a = Vec::new();
    
    // Add 5 pair constraints (all 10 variables in 5 pairs)
    for i in 0..5 {
        let mut row = vec![0.0; n];
        row[i * 2] = 1.0;
        row[i * 2 + 1] = 1.0;
        a.push(row);
    }
    
    let b = vec![15.0; 5];  // Each pair <= 15
    
    let problem = LpProblem::new(
        n,
        m,
        c,
        a,
        b,
        vec![0.0; n],
        vec![f64::INFINITY; n],
    );
    
    let solution = solve(&problem).unwrap();
    
    assert_eq!(solution.status, LpStatus::Optimal);
    assert!(solution.objective <= 75.0 + 1e-6, 
        "Objective {} should be <= 75", solution.objective);
    assert!(solution.x.iter().all(|&x| x >= -1e-6));
    
    // Check total sum
    let sum: f64 = solution.x.iter().sum();
    assert!(sum <= 75.0 + 1e-6, "Total sum {} exceeds 75", sum);
    
    // Check all pair constraints
    for i in 0..5 {
        let pair_sum = solution.x[i * 2] + solution.x[i * 2 + 1];
        assert!(pair_sum <= 15.0 + 1e-6, "Pair {} sum {} exceeds 15", i, pair_sum);
    }
}

#[test]
fn test_multiple_active_constraints() {
    // Test problem where multiple constraints are active at optimum
    // Maximize x1 + x2
    // Subject to: x1 + 2*x2 <= 10
    //             2*x1 + x2 <= 10
    //             x1, x2 >= 0
    // Optimal at intersection: x1 = 10/3, x2 = 10/3
    
    let problem = LpProblem::new(
        2,
        2,
        vec![1.0, 1.0],
        vec![
            vec![1.0, 2.0],
            vec![2.0, 1.0],
        ],
        vec![10.0, 10.0],
        vec![0.0, 0.0],
        vec![f64::INFINITY, f64::INFINITY],
    );
    
    let result = solve(&problem);
    
    // Should find optimal solution
    assert!(result.is_ok());
    let solution = result.unwrap();
    assert_eq!(solution.status, LpStatus::Optimal);
    
    // Check solution is feasible
    assert!(solution.x[0] >= -1e-6);
    assert!(solution.x[1] >= -1e-6);
    assert!(solution.x[0] + 2.0 * solution.x[1] <= 10.0 + 1e-6);
    assert!(2.0 * solution.x[0] + solution.x[1] <= 10.0 + 1e-6);
    
    // Objective should be reasonable
    assert!(solution.objective >= 0.0);
    assert!(solution.objective <= 10.0); // Can't exceed this given constraints
}

#[test]
fn test_tight_constraints() {
    // Test with multiple constraints that are all tight at optimum
    // Maximize x1 + x2
    // x1 <= 5
    // x2 <= 5
    // x1 + x2 <= 10
    
    let problem = LpProblem::new(
        2,
        3,
        vec![1.0, 1.0],
        vec![
            vec![1.0, 0.0],
            vec![0.0, 1.0],
            vec![1.0, 1.0],
        ],
        vec![5.0, 5.0, 10.0],
        vec![0.0, 0.0],
        vec![f64::INFINITY, f64::INFINITY],
    );
    
    let solution = solve(&problem).unwrap();
    
    assert_eq!(solution.status, LpStatus::Optimal);
    assert!((solution.objective - 10.0).abs() < 1e-6);
    assert!((solution.x[0] - 5.0).abs() < 1e-6);
    assert!((solution.x[1] - 5.0).abs() < 1e-6);
}

#[test]
fn test_single_variable() {
    // Simple single variable problem
    // Maximize x subject to x <= 10
    
    let problem = LpProblem::new(
        1,
        1,
        vec![1.0],
        vec![vec![1.0]],
        vec![10.0],
        vec![0.0],
        vec![f64::INFINITY],
    );
    
    let solution = solve(&problem).unwrap();
    
    assert_eq!(solution.status, LpStatus::Optimal);
    assert!((solution.objective - 10.0).abs() < 1e-6);
    assert!((solution.x[0] - 10.0).abs() < 1e-6);
}

#[test]
fn test_zero_objective() {
    // Maximize 0 (constant objective)
    // Just find feasible solution
    
    let problem = LpProblem::new(
        2,
        1,
        vec![0.0, 0.0],
        vec![vec![1.0, 1.0]],
        vec![10.0],
        vec![0.0, 0.0],
        vec![f64::INFINITY, f64::INFINITY],
    );
    
    let solution = solve(&problem).unwrap();
    
    assert_eq!(solution.status, LpStatus::Optimal);
    assert!(solution.objective.abs() < 1e-6);
}

#[test]
fn test_solution_has_basis() {
    // Verify that solution includes basis information
    
    let problem = LpProblem::new(
        2,
        1,
        vec![1.0, 1.0],
        vec![vec![1.0, 1.0]],
        vec![10.0],
        vec![0.0, 0.0],
        vec![f64::INFINITY, f64::INFINITY],
    );
    
    let solution = solve(&problem).unwrap();
    
    assert_eq!(solution.status, LpStatus::Optimal);
    
    // Should have basis indices for warm-starting
    assert_eq!(solution.basic_indices.len(), 1); // m_constraints = 1
}

#[test]
fn test_variable_lower_bounds() {
    // Test LP with variable lower bounds > 0
    // Maximize: 2*x1 + 3*x2
    // Subject to:
    //   x1 + x2 <= 10
    //   2 <= x1 <= 8
    //   3 <= x2 <= 7
    
    let problem = LpProblem::new(
        2,
        1,
        vec![2.0, 3.0],
        vec![vec![1.0, 1.0]],
        vec![10.0],
        vec![2.0, 3.0],  // Lower bounds: x1 >= 2, x2 >= 3
        vec![8.0, 7.0],  // Upper bounds: x1 <= 8, x2 <= 7
    );
    
    let solution = solve(&problem).unwrap();
    
    assert_eq!(solution.status, LpStatus::Optimal);
    
    // Check bounds are satisfied
    assert!(solution.x[0] >= 2.0 - 1e-6, "x1 should be >= 2, got {}", solution.x[0]);
    assert!(solution.x[0] <= 8.0 + 1e-6, "x1 should be <= 8, got {}", solution.x[0]);
    assert!(solution.x[1] >= 3.0 - 1e-6, "x2 should be >= 3, got {}", solution.x[1]);
    assert!(solution.x[1] <= 7.0 + 1e-6, "x2 should be <= 7, got {}", solution.x[1]);
    
    // Check constraint
    assert!(solution.x[0] + solution.x[1] <= 10.0 + 1e-6);
    
    // Optimal solution should be at constraint boundary
    // Since we maximize 2*x1 + 3*x2 with x1 + x2 <= 10,
    // and x2 has higher coefficient, we should push x2 to its max (7)
    // then x1 = 10 - 7 = 3
    assert!((solution.x[0] - 3.0).abs() < 0.01);
    assert!((solution.x[1] - 7.0).abs() < 0.01);
    
    // Optimal objective = 2*3 + 3*7 = 6 + 21 = 27
    assert!((solution.objective - 27.0).abs() < 0.01);
}

#[test]
fn test_all_variables_bounded() {
    // Test LP where all variables have finite upper and lower bounds
    // Maximize: x1 + 2*x2 + 3*x3
    // Subject to:
    //   x1 + x2 + x3 <= 20
    //   2*x1 + x2 + x3 <= 25
    //   5 <= x1 <= 10
    //   3 <= x2 <= 12
    //   1 <= x3 <= 8
    
    let problem = LpProblem::new(
        3,
        2,
        vec![1.0, 2.0, 3.0],
        vec![
            vec![1.0, 1.0, 1.0],
            vec![2.0, 1.0, 1.0],
        ],
        vec![20.0, 25.0],
        vec![5.0, 3.0, 1.0],  // Lower bounds
        vec![10.0, 12.0, 8.0], // Upper bounds
    );
    
    let solution = solve(&problem).unwrap();
    
    assert_eq!(solution.status, LpStatus::Optimal);
    
    // Verify all bounds are satisfied
    assert!(solution.x[0] >= 5.0 - 1e-6 && solution.x[0] <= 10.0 + 1e-6);
    assert!(solution.x[1] >= 3.0 - 1e-6 && solution.x[1] <= 12.0 + 1e-6);
    assert!(solution.x[2] >= 1.0 - 1e-6 && solution.x[2] <= 8.0 + 1e-6);
    
    // Verify constraints
    let sum1 = solution.x[0] + solution.x[1] + solution.x[2];
    let sum2 = 2.0*solution.x[0] + solution.x[1] + solution.x[2];
    assert!(sum1 <= 20.0 + 1e-6, "Constraint 1 violated: {} > 20", sum1);
    assert!(sum2 <= 25.0 + 1e-6, "Constraint 2 violated: {} > 25", sum2);
    
    // Since x3 has highest coefficient (3), it should be pushed to its max (8)
    // Then x2 (coefficient 2) should be maximized
    // The solution should maximize x3 first, then x2, then x1
    assert!((solution.x[2] - 8.0).abs() < 0.1, "x3 should be near 8, got {}", solution.x[2]);
}

#[test]
fn test_mixed_bounds() {
    // Test LP with mix of bounded and unbounded variables
    // Maximize: x1 + x2 + x3
    // Subject to:
    //   x1 + x2 + x3 <= 15
    //   5 <= x1 <= infinity  // Only lower bound
    //   0 <= x2 <= 6          // Both bounds (x2 explicitly bounded above)
    //   0 <= x3 <= infinity   // Only non-negativity
    
    let problem = LpProblem::new(
        3,
        1,
        vec![1.0, 1.0, 1.0],
        vec![vec![1.0, 1.0, 1.0]],
        vec![15.0],
        vec![5.0, 0.0, 0.0],
        vec![f64::INFINITY, 6.0, f64::INFINITY],
    );
    
    let solution = solve(&problem).unwrap();
    
    assert_eq!(solution.status, LpStatus::Optimal);
    
    // Check bounds
    assert!(solution.x[0] >= 5.0 - 1e-6);
    assert!(solution.x[1] >= 0.0 - 1e-6);
    assert!(solution.x[1] <= 6.0 + 1e-6);
    assert!(solution.x[2] >= 0.0 - 1e-6);
    
    // Constraint
    assert!(solution.x[0] + solution.x[1] + solution.x[2] <= 15.0 + 1e-6);
    
    // Optimal: x1 + x2 + x3 = 15 (at constraint boundary)
    // x2 is capped at 6, so optimal is: x1=5, x2=6, x3=4
    let total = solution.x[0] + solution.x[1] + solution.x[2];
    assert!((total - 15.0).abs() < 0.01);
}
