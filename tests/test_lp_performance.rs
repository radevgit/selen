/// Temporary performance test for LP solver
/// Tests a 100x100 problem to measure solve time and memory usage

use selen::lpsolver::{LpProblem, LpConfig};
use selen::lpsolver::simplex_primal::PrimalSimplex;

#[test]
#[ignore] // Run with: cargo test --release --ignored
fn test_large_lp_problem_100x100() {
    // Problem size
    let n_vars = 100;      // 100 variables
    let n_constraints = 100; // 100 constraints
    
    println!("\n=== LP Solver Performance Test ===");
    println!("Problem size: {} variables, {} constraints", n_vars, n_constraints);
    
    // Create a feasible LP problem
    // Maximize: sum of all variables (c = [1, 1, ..., 1])
    let c: Vec<f64> = vec![1.0; n_vars];
    
    // Constraints: Each constraint sums a subset of variables <= some bound
    // Make it structured so it's feasible and not trivial
    let mut a = Vec::new();
    let mut b = Vec::new();
    
    // Add constraints of the form: x_i + x_{i+1} + ... + x_{i+9} <= 50
    // This creates overlapping constraints
    for i in 0..n_constraints {
        let mut row = vec![0.0; n_vars];
        for j in 0..10 {
            let idx = (i + j) % n_vars;
            row[idx] = 1.0;
        }
        a.push(row);
        b.push(50.0);
    }
    
    // Variable bounds: 0 <= x_i <= 10
    let lower = vec![0.0; n_vars];
    let upper = vec![10.0; n_vars];
    
    let problem = LpProblem::new(n_vars, n_constraints, c, a, b, lower, upper);
    
    // Create solver with default config (no limits)
    let config = LpConfig::unlimited();
    let mut solver = PrimalSimplex::new(config);
    
    // Time the solve
    let start = std::time::Instant::now();
    let result = solver.solve(&problem);
    let elapsed = start.elapsed();
    
    // Check result
    assert!(result.is_ok(), "Solver should succeed: {:?}", result);
    let solution = result.unwrap();
    
    println!("\n=== Results ===");
    println!("Status: {:?}", solution.status);
    println!("Objective value: {:.6}", solution.objective);
    println!("Iterations: {}", solution.iterations);
    println!("Solve time: {:.3} seconds ({:.1} ms)", 
             elapsed.as_secs_f64(), 
             elapsed.as_millis());
    
    // Calculate some statistics
    let sum_x: f64 = solution.x.iter().sum();
    let max_x = solution.x.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let min_x = solution.x.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    
    println!("Sum of variables: {:.6}", sum_x);
    println!("Max variable value: {:.6}", max_x);
    println!("Min variable value: {:.6}", min_x);
    
    // Verify constraints are satisfied
    println!("\nVerifying constraints...");
    let mut max_violation = 0.0_f64;
    for i in 0..n_constraints {
        let lhs: f64 = problem.a[i].iter()
            .zip(solution.x.iter())
            .map(|(a_ij, x_j)| a_ij * x_j)
            .sum();
        let violation = (lhs - problem.b[i]).max(0.0);
        max_violation = max_violation.max(violation);
    }
    println!("Max constraint violation: {:.2e}", max_violation);
    assert!(max_violation < 1e-6, "Constraints should be satisfied");
    
    // Estimate memory usage from problem dimensions
    let constraint_matrix_mb = (n_vars * n_constraints * 8) as f64 / (1024.0 * 1024.0);
    let basis_matrices_mb = (2 * n_constraints * n_constraints * 8) as f64 / (1024.0 * 1024.0);
    let estimated_total_mb = constraint_matrix_mb + basis_matrices_mb;
    
    println!("\n=== Performance Summary ===");
    println!("✓ Problem solved successfully");
    println!("✓ Time: {:.3}s", elapsed.as_secs_f64());
    println!("✓ Estimated memory: ~{:.2} MB (constraint matrix: {:.2} MB, basis: {:.2} MB)", 
             estimated_total_mb, constraint_matrix_mb, basis_matrices_mb);
    println!("✓ Iterations: {}", solution.iterations);
}

#[test]
#[ignore] // Run with: cargo test --release --ignored
fn test_very_large_lp_problem_200x200() {
    // Even larger problem to stress test
    let n_vars = 200;
    let n_constraints = 200;
    
    println!("\n=== LP Solver Large Problem Test ===");
    println!("Problem size: {} variables, {} constraints", n_vars, n_constraints);
    
    let c: Vec<f64> = vec![1.0; n_vars];
    
    let mut a = Vec::new();
    let mut b = Vec::new();
    
    for i in 0..n_constraints {
        let mut row = vec![0.0; n_vars];
        // Sparse constraints - only 5% non-zero
        for j in 0..10 {
            let idx = (i * 7 + j * 13) % n_vars;
            row[idx] = 1.0;
        }
        a.push(row);
        b.push(30.0);
    }
    
    let lower = vec![0.0; n_vars];
    let upper = vec![5.0; n_vars];
    
    let problem = LpProblem::new(n_vars, n_constraints, c, a, b, lower, upper);
    
    let config = LpConfig::unlimited();
    let mut solver = PrimalSimplex::new(config);
    
    let start = std::time::Instant::now();
    let result = solver.solve(&problem);
    let elapsed = start.elapsed();
    
    assert!(result.is_ok(), "Solver should succeed");
    let solution = result.unwrap();
    
    // Estimate memory usage from problem dimensions
    let constraint_matrix_mb = (n_vars * n_constraints * 8) as f64 / (1024.0 * 1024.0);
    let basis_matrices_mb = (2 * n_constraints * n_constraints * 8) as f64 / (1024.0 * 1024.0);
    let estimated_total_mb = constraint_matrix_mb + basis_matrices_mb;
    
    println!("\n=== Results (200x200) ===");
    println!("Status: {:?}", solution.status);
    println!("Objective: {:.6}", solution.objective);
    println!("Iterations: {}", solution.iterations);
    println!("Solve time: {:.3}s ({} ms)", 
             elapsed.as_secs_f64(), 
             elapsed.as_millis());
    println!("Estimated memory: {:.2} MB", estimated_total_mb);
}

#[test]
#[ignore] // Run with: cargo test --release --ignored
fn test_dense_problem_50x50() {
    // Smaller but dense problem
    let n_vars = 50;
    let n_constraints = 50;
    
    println!("\n=== LP Solver Dense Problem Test ===");
    println!("Problem size: {} variables, {} constraints (DENSE)", n_vars, n_constraints);
    
    let c: Vec<f64> = (0..n_vars).map(|i| (i + 1) as f64).collect();
    
    let mut a = Vec::new();
    let mut b = Vec::new();
    
    // Dense constraints - all variables in each constraint
    for i in 0..n_constraints {
        let row: Vec<f64> = (0..n_vars)
            .map(|j| ((i * j + 1) % 10) as f64 / 10.0)
            .collect();
        a.push(row);
        b.push(100.0);
    }
    
    let lower = vec![0.0; n_vars];
    let upper = vec![20.0; n_vars];
    
    let problem = LpProblem::new(n_vars, n_constraints, c, a, b, lower, upper);
    
    let config = LpConfig::unlimited();
    let mut solver = PrimalSimplex::new(config);
    
    let start = std::time::Instant::now();
    let result = solver.solve(&problem);
    let elapsed = start.elapsed();
    
    assert!(result.is_ok(), "Solver should succeed");
    let solution = result.unwrap();
    
    // Estimate memory usage from problem dimensions
    let constraint_matrix_mb = (n_vars * n_constraints * 8) as f64 / (1024.0 * 1024.0);
    let basis_matrices_mb = (2 * n_constraints * n_constraints * 8) as f64 / (1024.0 * 1024.0);
    let estimated_total_mb = constraint_matrix_mb + basis_matrices_mb;
    
    println!("\n=== Results (50x50 Dense) ===");
    println!("Status: {:?}", solution.status);
    println!("Objective: {:.6}", solution.objective);
    println!("Iterations: {}", solution.iterations);
    println!("Solve time: {:.3}s ({} ms)", 
             elapsed.as_secs_f64(), 
             elapsed.as_millis());
    println!("Estimated memory: {:.2} MB", estimated_total_mb);
}

#[test]
#[ignore] // Run with: cargo test --release --ignored
fn test_very_large_problem_500x500() {
    // Very large problem
    let n_vars = 500;
    let n_constraints = 500;
    
    println!("\n=== LP Solver Very Large Problem Test ===");
    println!("Problem size: {} variables, {} constraints", n_vars, n_constraints);
    
    let c: Vec<f64> = vec![1.0; n_vars];
    
    let mut a = Vec::new();
    let mut b = Vec::new();
    
    // Sparse constraints - only ~2% non-zero
    for i in 0..n_constraints {
        let mut row = vec![0.0; n_vars];
        for j in 0..10 {
            let idx = (i * 11 + j * 17) % n_vars;
            row[idx] = 1.0;
        }
        a.push(row);
        b.push(25.0);
    }
    
    let lower = vec![0.0; n_vars];
    let upper = vec![5.0; n_vars];
    
    let problem = LpProblem::new(n_vars, n_constraints, c, a, b, lower, upper);
    
    let config = LpConfig::unlimited();
    let mut solver = PrimalSimplex::new(config);
    
    // Estimate memory usage from problem dimensions
    let constraint_matrix_mb = (n_vars * n_constraints * 8) as f64 / (1024.0 * 1024.0);
    let basis_matrices_mb = (2 * n_constraints * n_constraints * 8) as f64 / (1024.0 * 1024.0);
    let estimated_total_mb = constraint_matrix_mb + basis_matrices_mb;
    
    println!("Estimated memory: {:.2} MB", estimated_total_mb);
    
    let start = std::time::Instant::now();
    let result = solver.solve(&problem);
    let elapsed = start.elapsed();
    
    assert!(result.is_ok(), "Solver should succeed");
    let solution = result.unwrap();
    
    println!("\n=== Results (500x500) ===");
    println!("Status: {:?}", solution.status);
    println!("Objective: {:.6}", solution.objective);
    println!("Iterations: {}", solution.iterations);
    println!("Solve time: {:.3}s ({} ms)", 
             elapsed.as_secs_f64(), 
             elapsed.as_millis());
    println!("Memory: {:.2} MB", estimated_total_mb);
    
    // Performance summary
    let time_per_var = elapsed.as_secs_f64() / n_vars as f64;
    let iter_per_constraint = solution.iterations as f64 / n_constraints as f64;
    println!("\n=== Performance Metrics ===");
    println!("Time per variable: {:.6}s ({:.3} ms)", time_per_var, time_per_var * 1000.0);
    println!("Iterations per constraint: {:.2}", iter_per_constraint);
}

#[test]
#[ignore] // Run with: cargo test --release --ignored
fn test_dense_500x500_50_percent() {
    // Test 500x500 with 50% density to compare with sparse version
    let n_vars = 500;
    let n_constraints = 500;
    
    println!("\n=== LP Solver 500x500 DENSE (50%) Problem Test ===");
    println!("Problem size: {} variables, {} constraints", n_vars, n_constraints);
    println!("Density: ~50% (250 non-zeros per constraint)");
    
    let c: Vec<f64> = vec![1.0; n_vars];
    
    let mut a = Vec::new();
    let mut b = Vec::new();
    
    // 50% density - 250 non-zero entries per constraint
    for i in 0..n_constraints {
        let mut row = vec![0.0; n_vars];
        for j in 0..250 {
            let idx = (i * 3 + j * 2) % n_vars;
            row[idx] = 1.0;
        }
        a.push(row);
        b.push(125.0);
    }
    
    let lower = vec![0.0; n_vars];
    let upper = vec![1.0; n_vars];
    
    let problem = LpProblem::new(n_vars, n_constraints, c, a, b, lower, upper);
    
    let config = LpConfig::unlimited();
    let mut solver = PrimalSimplex::new(config);
    
    // Estimate memory usage from problem dimensions
    let constraint_matrix_mb = (n_vars * n_constraints * 8) as f64 / (1024.0 * 1024.0);
    let basis_matrices_mb = (2 * n_constraints * n_constraints * 8) as f64 / (1024.0 * 1024.0);
    let estimated_total_mb = constraint_matrix_mb + basis_matrices_mb;
    
    println!("Estimated memory: {:.2} MB", estimated_total_mb);
    
    let start = std::time::Instant::now();
    let result = solver.solve(&problem);
    let elapsed = start.elapsed();
    
    assert!(result.is_ok(), "Solver should succeed: {:?}", result.err());
    let solution = result.unwrap();
    
    println!("\n=== Results (500x500 DENSE 50%) ===");
    println!("Status: {:?}", solution.status);
    println!("Objective: {:.6}", solution.objective);
    println!("Iterations: {}", solution.iterations);
    println!("Solve time: {:.3}s ({} ms)", 
             elapsed.as_secs_f64(), 
             elapsed.as_millis());
    println!("Memory: {:.2} MB", estimated_total_mb);
    
    let time_per_var = elapsed.as_secs_f64() / n_vars as f64;
    let iter_per_constraint = solution.iterations as f64 / n_constraints as f64;
    println!("\n=== Performance Metrics ===");
    println!("Time per variable: {:.6}s ({:.3} ms)", time_per_var, time_per_var * 1000.0);
    println!("Iterations per constraint: {:.2}", iter_per_constraint);
    
    // Compare with sparse version
    println!("\n=== Density Impact ===");
    println!("Sparse 500x500 (~2% density): ~100 seconds");
    println!("Dense 500x500 (50% density): {:.1} seconds", elapsed.as_secs_f64());
    let slowdown = elapsed.as_secs_f64() / 100.0;
    println!("Density slowdown factor: {:.1}x", slowdown);
}

#[test]
#[ignore] // Run with: cargo test --release --ignored
fn test_dense_500x500_10_percent() {
    // Test 500x500 with 10% density
    let n_vars = 500;
    let n_constraints = 500;
    
    println!("\n=== LP Solver 500x500 MEDIUM DENSE (10%) Problem Test ===");
    println!("Problem size: {} variables, {} constraints", n_vars, n_constraints);
    println!("Density: ~10% (50 non-zeros per constraint)");
    
    let c: Vec<f64> = vec![1.0; n_vars];
    
    let mut a = Vec::new();
    let mut b = Vec::new();
    
    // 10% density - 50 non-zero entries per constraint
    for i in 0..n_constraints {
        let mut row = vec![0.0; n_vars];
        for j in 0..50 {
            let idx = (i * 7 + j * 11) % n_vars;
            row[idx] = 1.0;
        }
        a.push(row);
        b.push(25.0);
    }
    
    let lower = vec![0.0; n_vars];
    let upper = vec![1.0; n_vars];
    
    let problem = LpProblem::new(n_vars, n_constraints, c, a, b, lower, upper);
    
    let config = LpConfig::unlimited();
    let mut solver = PrimalSimplex::new(config);
    
    // Estimate memory usage from problem dimensions
    let constraint_matrix_mb = (n_vars * n_constraints * 8) as f64 / (1024.0 * 1024.0);
    let basis_matrices_mb = (2 * n_constraints * n_constraints * 8) as f64 / (1024.0 * 1024.0);
    let estimated_total_mb = constraint_matrix_mb + basis_matrices_mb;
    
    let start = std::time::Instant::now();
    let result = solver.solve(&problem);
    let elapsed = start.elapsed();
    
    assert!(result.is_ok(), "Solver should succeed: {:?}", result.err());
    let solution = result.unwrap();
    
    println!("\n=== Results (500x500 10% DENSE) ===");
    println!("Status: {:?}", solution.status);
    println!("Objective: {:.6}", solution.objective);
    println!("Iterations: {}", solution.iterations);
    println!("Solve time: {:.3}s ({} ms)", 
             elapsed.as_secs_f64(), 
             elapsed.as_millis());
    println!("Memory: {:.2} MB", estimated_total_mb);
    
    let iter_per_constraint = solution.iterations as f64 / n_constraints as f64;
    println!("Iterations per constraint: {:.2}", iter_per_constraint);
}

#[test]
#[ignore] // Run with: cargo test --release --ignored
fn test_large_problem_300x300() {
    // Large problem - sweet spot test
    let n_vars = 300;
    let n_constraints = 300;
    
    println!("\n=== LP Solver Large Problem Test (300x300) ===");
    println!("Problem size: {} variables, {} constraints", n_vars, n_constraints);
    
    let c: Vec<f64> = vec![1.0; n_vars];
    
    let mut a = Vec::new();
    let mut b = Vec::new();
    
    // Sparse constraints
    for i in 0..n_constraints {
        let mut row = vec![0.0; n_vars];
        for j in 0..10 {
            let idx = (i * 7 + j * 11) % n_vars;
            row[idx] = 1.0;
        }
        a.push(row);
        b.push(30.0);
    }
    
    let lower = vec![0.0; n_vars];
    let upper = vec![5.0; n_vars];
    
    let problem = LpProblem::new(n_vars, n_constraints, c, a, b, lower, upper);
    
    let config = LpConfig::unlimited();
    let mut solver = PrimalSimplex::new(config);
    
    // Estimate memory usage from problem dimensions
    let constraint_matrix_mb = (n_vars * n_constraints * 8) as f64 / (1024.0 * 1024.0);
    let basis_matrices_mb = (2 * n_constraints * n_constraints * 8) as f64 / (1024.0 * 1024.0);
    let estimated_total_mb = constraint_matrix_mb + basis_matrices_mb;
    
    println!("Estimated memory: {:.2} MB", estimated_total_mb);
    
    let start = std::time::Instant::now();
    let result = solver.solve(&problem);
    let elapsed = start.elapsed();
    
    assert!(result.is_ok(), "Solver should succeed");
    let solution = result.unwrap();
    
    println!("\n=== Results (300x300) ===");
    println!("Status: {:?}", solution.status);
    println!("Objective: {:.6}", solution.objective);
    println!("Iterations: {}", solution.iterations);
    println!("Solve time: {:.3}s ({} ms)", 
             elapsed.as_secs_f64(), 
             elapsed.as_millis());
    println!("Memory: {:.2} MB", estimated_total_mb);
    
    let time_per_var = elapsed.as_secs_f64() / n_vars as f64;
    let iter_per_constraint = solution.iterations as f64 / n_constraints as f64;
    println!("\n=== Performance Metrics ===");
    println!("Time per variable: {:.6}s ({:.3} ms)", time_per_var, time_per_var * 1000.0);
    println!("Iterations per constraint: {:.2}", iter_per_constraint);
}

#[test]
#[ignore] // Run with: cargo test --release --ignored
fn test_very_dense_500x500_80_percent() {
    // Dense problem - 500x500 with 80% density
    let n_vars = 500;
    let n_constraints = 500;
    
    println!("\n=== LP Solver 500x500 VERY DENSE (80%) Problem Test ===");
    println!("Problem size: {} variables, {} constraints", n_vars, n_constraints);
    println!("Density: ~80% (400 non-zeros per constraint)");
    println!("Expected: Fast convergence due to high density");
    
    let c: Vec<f64> = vec![1.0; n_vars];
    
    let mut a = Vec::new();
    let mut b = Vec::new();
    
    // 80% density - 400 non-zero entries per constraint
    for i in 0..n_constraints {
        let mut row = vec![0.0; n_vars];
        for j in 0..400 {
            let idx = (i + j) % n_vars;
            row[idx] = 1.0;
        }
        a.push(row);
        b.push(200.0);
    }
    
    let lower = vec![0.0; n_vars];
    let upper = vec![1.0; n_vars];
    
    let problem = LpProblem::new(n_vars, n_constraints, c, a, b, lower, upper);
    
    let config = LpConfig::unlimited();
    let mut solver = PrimalSimplex::new(config);
    
    let result = solver.solve(&problem);
    
    assert!(result.is_ok(), "Solver should succeed: {:?}", result.err());
    let solution = result.unwrap();
    
    println!("\n=== Results (500x500 DENSE 80%) ===");
    println!("Status: {:?}", solution.status);
    println!("Objective: {:.6}", solution.objective);
    println!("Iterations: {}", solution.iterations);
    
    // Use statistics from the LP solver!
    println!("\n=== Statistics from LP Solver ===");
    println!("Solve time: {:.3}s ({:.0} ms)", 
             solution.stats.solve_time_ms / 1000.0, 
             solution.stats.solve_time_ms);
    println!("Peak memory: {:.2} MB", solution.stats.peak_memory_mb);
    println!("Phase 1 time: {:.3}s ({} iterations)", 
             solution.stats.phase1_time_ms / 1000.0, 
             solution.stats.phase1_iterations);
    println!("Phase 2 time: {:.3}s ({} iterations)", 
             solution.stats.phase2_time_ms / 1000.0, 
             solution.stats.phase2_iterations);
    println!("Factorizations: {}", solution.stats.factorizations);
    
    let time_per_iter = solution.stats.solve_time_ms / solution.iterations as f64;
    let iter_per_constraint = solution.iterations as f64 / n_constraints as f64;
    
    println!("\n=== Performance Metrics ===");
    println!("Time per iteration: {:.2} ms", time_per_iter);
    println!("Iterations per constraint: {:.2}", iter_per_constraint);
    
    println!("\n=== Comparison ===");
    println!("500x500 @ 50% density: 35s, 250 iterations");
    println!("500x500 @ 80% density: {:.1}s, {} iterations", 
             solution.stats.solve_time_ms / 1000.0, solution.iterations);
}

#[test]
#[ignore] // Run with: cargo test --release --ignored
fn test_extreme_large_problem_1000x1000() {
    // Extremely large problem
    let n_vars = 1000;
    let n_constraints = 1000;
    
    println!("\n=== LP Solver EXTREME Large Problem Test ===");
    println!("Problem size: {} variables, {} constraints", n_vars, n_constraints);
    println!("⚠ This test may take 10+ minutes!");
    
    let c: Vec<f64> = vec![1.0; n_vars];
    
    let mut a = Vec::new();
    let mut b = Vec::new();
    
    // Very sparse constraints - only ~1% non-zero
    for i in 0..n_constraints {
        let mut row = vec![0.0; n_vars];
        for j in 0..10 {
            let idx = (i * 13 + j * 19) % n_vars;
            row[idx] = 1.0;
        }
        a.push(row);
        b.push(20.0);
    }
    
    let lower = vec![0.0; n_vars];
    let upper = vec![4.0; n_vars];
    
    let problem = LpProblem::new(n_vars, n_constraints, c, a, b, lower, upper);
    
    // Add a timeout to avoid hanging tests
    let config = LpConfig::unlimited().with_timeout_ms(600_000); // 10 minute timeout
    let mut solver = PrimalSimplex::new(config);
    
    // Estimate memory usage from problem dimensions
    let constraint_matrix_mb = (n_vars * n_constraints * 8) as f64 / (1024.0 * 1024.0);
    let basis_matrices_mb = (2 * n_constraints * n_constraints * 8) as f64 / (1024.0 * 1024.0);
    let estimated_total_mb = constraint_matrix_mb + basis_matrices_mb;
    
    println!("Estimated memory: {:.2} MB", estimated_total_mb);
    
    let start = std::time::Instant::now();
    let result = solver.solve(&problem);
    let elapsed = start.elapsed();
    
    assert!(result.is_ok(), "Solver should succeed");
    let solution = result.unwrap();
    
    println!("\n=== Results (1000x1000) ===");
    println!("Status: {:?}", solution.status);
    println!("Objective: {:.6}", solution.objective);
    println!("Iterations: {}", solution.iterations);
    println!("Solve time: {:.3}s ({} ms)", 
             elapsed.as_secs_f64(), 
             elapsed.as_millis());
    println!("Memory: {:.2} MB", estimated_total_mb);
    
    // Performance summary
    let time_per_var = elapsed.as_secs_f64() / n_vars as f64;
    let iter_per_constraint = solution.iterations as f64 / n_constraints as f64;
    println!("\n=== Performance Metrics ===");
    println!("Time per variable: {:.6}s ({:.3} ms)", time_per_var, time_per_var * 1000.0);
    println!("Iterations per constraint: {:.2}", iter_per_constraint);
    
    if elapsed.as_secs() > 60 {
        println!("\n⚠ WARNING: Solve time exceeded 1 minute - this is approaching practical limits");
    } else if elapsed.as_secs() > 30 {
        println!("\n⚠ NOTE: Solve time > 30s - consider sparse matrix implementation for this size");
    }
}


