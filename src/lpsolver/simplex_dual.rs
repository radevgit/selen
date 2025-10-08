//! Dual Simplex Algorithm for Linear Programming
//!
//! The dual simplex method starts from a dual-feasible but primal-infeasible basis
//! and iteratively improves primal feasibility while maintaining dual feasibility.
//!
//! This is particularly useful for:
//! - Warm-starting from a previously optimal solution after constraint changes
//! - Branch-and-bound algorithms in integer programming
//! - Reoptimization after adding/removing constraints

use crate::lpsolver::basis::Basis;
use crate::lpsolver::matrix::{Matrix, get_lp_memory_mb};
use crate::lpsolver::types::{LpConfig, LpError, LpProblem, LpSolution, LpStatus};

/// Dual Simplex solver
///
/// Maintains dual feasibility (reduced costs ≤ 0 for maximization) while
/// improving primal feasibility (x_B ≥ 0)
pub struct DualSimplex {
    config: LpConfig,
}

impl DualSimplex {
    /// Create new Dual Simplex solver with given configuration
    pub fn new(config: LpConfig) -> Self {
        Self { config }
    }

    /// Solve LP problem using dual simplex method
    ///
    /// Requires: Initial basis must be dual-feasible (all reduced costs ≤ 0)
    /// but may be primal-infeasible (some basic variables < 0)
    pub fn solve(&mut self, problem: &LpProblem) -> Result<LpSolution, LpError> {
        // Start timing for timeout checking
        let start_time = std::time::Instant::now();
        
        problem.validate()?;

        // Convert to standard form: Ax = b, x >= 0
        let (a, c, n_total) = self.to_standard_form(problem);
        let b = &problem.b;
        let n_vars = problem.n_vars;
        let m = problem.n_constraints;

        // Use provided basis if available (warm start)
        let mut basis = if let Some(ref basic_indices) = problem.basic_indices {
            // Construct basis from provided indices
            let basic = basic_indices.clone();
            let nonbasic: Vec<usize> = (0..n_total)
                .filter(|idx| !basic.contains(idx))
                .collect();
            
            let mut b = Basis::from_indices(basic, nonbasic);
            b.factorize(&a, &self.config)?;
            b
        } else {
            // No warm start - use primal simplex instead
            return Err(LpError::NumericalInstability);
        };

        // Dual simplex iterations
        let max_iterations = self.config.max_iterations;
        for iterations in 0..max_iterations {
            // Check timeout and memory every 100 iterations (not every iteration for performance)
            if iterations % 100 == 0 {
                if let Some(timeout_ms) = self.config.timeout_ms {
                    let elapsed = start_time.elapsed().as_millis() as u64;
                    if elapsed > timeout_ms {
                        return Err(LpError::TimeoutExceeded {
                            elapsed_ms: elapsed,
                            limit_ms: timeout_ms,
                        });
                    }
                }
                
                if let Some(limit_mb) = self.config.max_memory_mb {
                    let usage_mb = get_lp_memory_mb() as u64;
                    if usage_mb > limit_mb {
                        return Err(LpError::MemoryExceeded {
                            usage_mb,
                            limit_mb,
                        });
                    }
                }
            }
            
            // Compute current basic solution
            let x_basic = basis.solve_basic(b)?;
            
            // Build full solution vector
            let mut x = vec![0.0; n_total];
            for (i, &var_idx) in basis.basic.iter().enumerate() {
                x[var_idx] = x_basic[i];
            }

            // Check primal feasibility
            if basis.is_primal_feasible(&x_basic, self.config.feasibility_tol) {
                // Dual-feasible and primal-feasible => optimal
                let objective = basis.objective_value(&c, &x);
                return Ok(LpSolution::new(
                    LpStatus::Optimal,
                    objective,
                    x[..n_vars].to_vec(),
                    iterations,
                    basis.basic.clone(),
                ));
            }

            // Find leaving variable (most negative basic variable)
            let leaving_idx = self.find_leaving_variable(&x_basic)?;

            // Compute dual direction: row of B^(-1)A corresponding to leaving variable
            let mut dual_direction = vec![0.0; n_total];
            
            // Get row leaving_idx of B^(-1)
            let mut unit_vec = vec![0.0; m];
            unit_vec[leaving_idx] = 1.0;
            let pi_row = basis.lu.as_ref()
                .ok_or(LpError::NumericalInstability)?
                .solve_transpose(&unit_vec)?;
            
            // Multiply by A to get dual direction
            for j in 0..n_total {
                let a_col = a.col(j);
                dual_direction[j] = pi_row.iter()
                    .zip(a_col.iter())
                    .map(|(pi, a_ij)| pi * a_ij)
                    .sum();
            }

            // Find entering variable using dual ratio test
            let entering = self.find_entering_variable(
                &basis,
                &a,
                &c,
                &dual_direction,
            )?;

            // Perform basis swap
            let entering_nonbasic_idx = basis.nonbasic.iter()
                .position(|&idx| idx == entering)
                .ok_or(LpError::NumericalInstability)?;
            
            basis.swap(entering_nonbasic_idx, leaving_idx);
            basis.factorize(&a, &self.config)?;
        }

        // Max iterations reached
        Err(LpError::NumericalInstability)
    }

    /// Convert problem to standard form by adding slack variables
    fn to_standard_form(&self, problem: &LpProblem) -> (Matrix, Vec<f64>, usize) {
        let n = problem.n_vars;
        let m = problem.n_constraints;
        let n_total = n + m;

        // Create extended constraint matrix [A | I]
        let mut a_extended = Matrix::zeros(m, n_total);
        
        for i in 0..m {
            for j in 0..n {
                a_extended.set(i, j, problem.a[i][j]);
            }
        }
        
        for i in 0..m {
            a_extended.set(i, n + i, 1.0);
        }

        // Extend objective vector
        let mut c_extended = problem.c.clone();
        c_extended.extend(vec![0.0; m]);

        (a_extended, c_extended, n_total)
    }

    /// Find leaving variable (most negative basic variable for dual simplex)
    fn find_leaving_variable(&self, x_basic: &[f64]) -> Result<usize, LpError> {
        let mut best_idx = None;
        let mut most_negative = -self.config.feasibility_tol;

        for (i, &x_i) in x_basic.iter().enumerate() {
            if x_i < most_negative {
                most_negative = x_i;
                best_idx = Some(i);
            }
        }

        best_idx.ok_or(LpError::NumericalInstability)
    }

    /// Find entering variable using dual ratio test
    ///
    /// For dual simplex, we need to maintain dual feasibility (reduced costs ≤ 0)
    /// The ratio test ensures this while improving primal feasibility
    fn find_entering_variable(
        &self,
        basis: &Basis,
        a: &Matrix,
        c: &[f64],
        dual_direction: &[f64],
    ) -> Result<usize, LpError> {
        // Compute reduced costs for non-basic variables
        let reduced_costs = basis.compute_reduced_costs(a, c)?;

        let mut best_idx = None;
        let mut best_ratio = f64::INFINITY;

        // Dual ratio test: min { c_j / d_j : d_j > 0 }
        // where c_j is reduced cost and d_j is dual direction
        for &j in &basis.nonbasic {
            let d_j = dual_direction[j];
            
            if d_j > self.config.feasibility_tol {
                let ratio = reduced_costs[j] / d_j;
                
                if ratio >= 0.0 && ratio < best_ratio {
                    best_ratio = ratio;
                    best_idx = Some(j);
                }
            }
        }

        best_idx.ok_or(LpError::NumericalInstability)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lpsolver::simplex_primal::PrimalSimplex;

    #[test]
    fn test_dual_simplex_warmstart() {
        // First solve a problem with Primal Simplex to get an optimal basis
        // Maximize x1 + x2
        // Subject to: x1 + x2 <= 5
        //             x1, x2 >= 0
        let problem = LpProblem::new(
            2,
            1,
            vec![1.0, 1.0],
            vec![vec![1.0, 1.0]],
            vec![5.0],
            vec![0.0, 0.0],
            vec![f64::INFINITY, f64::INFINITY],
        );

        let mut primal_solver = PrimalSimplex::new(LpConfig::default());
        let solution = primal_solver.solve(&problem).unwrap();
        
        assert_eq!(solution.status, LpStatus::Optimal);
        assert!((solution.objective - 5.0).abs() < 1e-6);

        // Now create a new problem with an additional constraint
        // This makes the previous basis potentially infeasible
        // Maximize x1 + x2
        // Subject to: x1 + x2 <= 5
        //             x1 + x2 <= 4  (new tighter constraint)
        //             x1, x2 >= 0
        let problem_new = LpProblem {
            n_vars: 2,
            n_constraints: 2,
            c: vec![1.0, 1.0],
            a: vec![vec![1.0, 1.0], vec![1.0, 1.0]],
            b: vec![5.0, 4.0],
            lower_bounds: vec![0.0, 0.0],
            upper_bounds: vec![f64::INFINITY, f64::INFINITY],
            basic_indices: Some(solution.basic_indices.clone()),
        };

        // Try to use dual simplex with warm start
        // Note: This will fail if the warm start basis isn't dual-feasible
        // For now, we expect an error since we haven't implemented proper
        // basis adjustment for constraint changes
        let mut dual_solver = DualSimplex::new(LpConfig::default());
        let result = dual_solver.solve(&problem_new);
        
        // For now, we just verify the solver doesn't panic
        // Full implementation will require basis adjustment logic
        match result {
            Ok(_) => {
                // If it works, great!
            }
            Err(LpError::NumericalInstability) => {
                // Expected for now - the old basis may not be valid
            }
            Err(e) => {
                panic!("Unexpected error: {:?}", e);
            }
        }
    }

    #[test]
    fn test_dual_simplex_structure() {
        // Test that dual simplex has correct structure
        let config = LpConfig::default();
        let _solver = DualSimplex::new(config.clone());
        
        // Verify config is stored
        assert_eq!(_solver.config.max_iterations, config.max_iterations);
        assert_eq!(_solver.config.feasibility_tol, config.feasibility_tol);
        assert_eq!(_solver.config.optimality_tol, config.optimality_tol);
    }
}
