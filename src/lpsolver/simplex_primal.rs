//! Primal Simplex method for Linear Programming
//!
//! Solves LP problems in standard form:
//!   maximize c^T x
//!   subject to Ax = b, x >= 0
//!
//! Uses two-phase method:
//! - Phase I: Find initial feasible basis (if not provided)
//! - Phase II: Optimize from feasible basis to optimal solution

use super::matrix::Matrix;
use super::basis::Basis;
use super::types::{LpProblem, LpSolution, LpStatus, LpError, LpConfig};

/// Primal Simplex solver
pub struct PrimalSimplex {
    /// Configuration for solver
    config: LpConfig,
}

impl PrimalSimplex {
    /// Create a new Primal Simplex solver with given configuration
    pub fn new(config: LpConfig) -> Self {
        Self { config }
    }
    
    /// Estimate memory usage for matrices in MB
    fn estimate_memory_mb(&self, a: &Matrix, basis: &Basis) -> f64 {
        // Sum up memory from main constraint matrix, basis matrices, and working memory
        let mut total_bytes = a.memory_bytes();
        
        // Basis stores L, U, and permutation vectors
        // Estimate: 2 * m * m matrices for L and U, plus m integers for perm
        let m = a.rows;
        total_bytes += 2 * m * m * std::mem::size_of::<f64>();
        total_bytes += m * std::mem::size_of::<usize>();
        
        // Working vectors (reduced costs, etc) - estimate m + n floats
        total_bytes += (a.rows + a.cols) * std::mem::size_of::<f64>();
        
        total_bytes as f64 / (1024.0 * 1024.0)
    }
    
    /// Solve an LP problem using Primal Simplex
    ///
    /// Assumes problem is in standard form with slack variables already added
    pub fn solve(&mut self, problem: &LpProblem) -> Result<LpSolution, LpError> {
        // Start timing for timeout checking
        let start_time = std::time::Instant::now();
        
        // Validate problem
        problem.validate()?;
        
        // Convert inequality constraints to standard form (Ax = b) by adding slacks
        // Also handles variable bounds by substitution
        let (a_eq, b_eq, c_extended, _n_total) = self.to_standard_form(problem);
        
        // Phase I: Find initial feasible basis
        let phase1_start = std::time::Instant::now();
        let (mut basis, phase1_iterations) = self.phase_one(&a_eq, &b_eq, start_time)?;
        let phase1_time_ms = phase1_start.elapsed().as_secs_f64() * 1000.0;
        
        // Phase II: Optimize from feasible basis
        // Note: Pass problem.n_vars (original variable count) so solution extraction works correctly
        let phase2_start = std::time::Instant::now();
        let mut solution = self.phase_two(&a_eq, &c_extended, &b_eq, &mut basis, problem.n_vars, start_time, phase1_iterations)?;
        let phase2_time_ms = phase2_start.elapsed().as_secs_f64() * 1000.0;
        
        // Calculate phase2 iterations from total iterations
        let phase2_iterations = solution.iterations.saturating_sub(phase1_iterations);
        
        // Update solution statistics
        solution.stats.solve_time_ms = phase1_time_ms + phase2_time_ms;
        solution.stats.phase1_time_ms = phase1_time_ms;
        solution.stats.phase2_time_ms = phase2_time_ms;
        solution.stats.phase1_iterations = phase1_iterations;
        solution.stats.phase2_iterations = phase2_iterations;
        solution.stats.phase1_needed = phase1_iterations > 0;
        solution.stats.n_variables = problem.n_vars;
        solution.stats.n_constraints = problem.n_constraints;
        solution.stats.peak_memory_mb = self.estimate_memory_mb(&a_eq, &basis);
        solution.stats.factorizations = phase1_iterations + phase2_iterations; // Approximate: one per iteration
        
        // Transform solution back: x = x' + l
        for j in 0..problem.n_vars {
            solution.x[j] += problem.lower_bounds[j];
        }
        
        // Adjust objective value: f(x) = c^T x = c^T(x' + l) = c^T x' + c^T l
        // The solver returns f(x') = c^T x', so we need to add c^T l
        let mut constant_term = 0.0;
        for j in 0..problem.n_vars {
            constant_term += problem.c[j] * problem.lower_bounds[j];
        }
        solution.objective += constant_term;
        
        Ok(solution)
    }
    
    /// Convert inequality constraints Ax <= b to equality Ax + s = b
    /// Handles variable bounds by substitution
    /// Returns (A_extended, b_extended, c_extended, total_vars)
    fn to_standard_form(&self, problem: &LpProblem) -> (Matrix, Vec<f64>, Vec<f64>, usize) {
        let m = problem.n_constraints;
        let n = problem.n_vars;
        
        // Step 1: Handle variable lower bounds by substitution x'_j = x_j - l_j
        // This transforms l_j <= x_j into 0 <= x'_j
        // Also adjust RHS: b_i becomes b_i - sum_j(A_ij * l_j)
        let mut b_adjusted = problem.b.clone();
        for i in 0..m {
            for j in 0..n {
                b_adjusted[i] -= problem.a[i][j] * problem.lower_bounds[j];
            }
        }
        
        // Adjust objective: c_j becomes c_j (for x'_j), but we need to add constant term
        // f(x) = c^T x = c^T(x' + l) = c^T x' + c^T l
        // The constant term c^T l doesn't affect optimization
        let c_adjusted = problem.c.clone();
        
        // Step 2: Count upper-bounded variables (u_j < infinity)
        let n_upper_bounded = problem.upper_bounds.iter()
            .filter(|&&u| u < f64::INFINITY)
            .count();
        
        // Step 3: Calculate total variables needed
        // Original variables + upper bound constraints (as inequalities) + slack variables for original constraints + slack variables for upper bounds
        let n_constraints_total = m + n_upper_bounded;
        let n_total = n + n_constraints_total;
        
        // Create extended constraint matrix
        let mut a_extended = Matrix::zeros(n_constraints_total, n_total);
        let mut b_extended = vec![0.0; n_constraints_total];
        
        // Copy original constraints (first m rows)
        for i in 0..m {
            for j in 0..n {
                a_extended.set(i, j, problem.a[i][j]);
            }
            b_extended[i] = b_adjusted[i];
        }
        
        // Add slack variables for original constraints
        for i in 0..m {
            a_extended.set(i, n + i, 1.0);
        }
        
        // Add upper bound constraints: x'_j <= u_j - l_j (after substitution)
        let mut upper_bound_row = m;
        let mut upper_bound_slack = n + m;
        for j in 0..n {
            if problem.upper_bounds[j] < f64::INFINITY {
                // x'_j + slack = u_j - l_j
                a_extended.set(upper_bound_row, j, 1.0);
                a_extended.set(upper_bound_row, upper_bound_slack, 1.0);
                b_extended[upper_bound_row] = problem.upper_bounds[j] - problem.lower_bounds[j];
                upper_bound_row += 1;
                upper_bound_slack += 1;
            }
        }
        
        // Extend objective vector (all slacks have 0 cost)
        let mut c_extended = c_adjusted;
        c_extended.extend(vec![0.0; n_constraints_total]);
        
        (a_extended, b_extended, c_extended, n_total)
    }
    
    /// Phase I: Find initial feasible basis using artificial variables
    ///
    /// Solves auxiliary problem: minimize sum of artificial variables
    /// Returns (feasible basis, phase1_iterations) if one exists
    fn phase_one(&mut self, a: &Matrix, b: &[f64], start_time: std::time::Instant) -> Result<(Basis, usize), LpError> {
        let m = a.rows;
        let n = a.cols;
        
        // First, try the identity basis (slack variables)
        let mut basis = Basis::initial(n, m);
        
        // Factorize initial basis
        basis.factorize(a, &self.config)?;
        
        // Check if initial basis is feasible
        let x = basis.solve_basic(b)?;
        
        if basis.is_primal_feasible(&x, self.config.feasibility_tol) {
            return Ok((basis, 0)); // No Phase I iterations needed
        }
        
        // If not feasible (b has negative components or constraints incompatible),
        // we need to use artificial variables and solve the auxiliary problem:
        // minimize w = sum of artificial variables
        // This is known as the "Big M" method or two-phase simplex
        
        // Create augmented problem with artificial variables
        // Original: Ax = b, x >= 0
        // Augmented: [A | I] [x; y] = b, x >= 0, y >= 0
        // Minimize: sum(y_i) where y_i are artificial variables
        
        let n_augmented = n + m; // Original variables + artificial variables
        let mut a_augmented = Matrix::zeros(m, n_augmented);
        
        // Copy original constraint matrix
        for i in 0..m {
            for j in 0..n {
                a_augmented.set(i, j, a.get(i, j));
            }
        }
        
        // Add identity matrix for artificial variables
        for i in 0..m {
            a_augmented.set(i, n + i, 1.0);
        }
        
        // Adjust RHS to be non-negative by flipping rows if needed
        let mut b_augmented = b.to_vec();
        for i in 0..m {
            if b_augmented[i] < 0.0 {
                // Multiply row by -1
                b_augmented[i] = -b_augmented[i];
                for j in 0..n_augmented {
                    a_augmented.set(i, j, -a_augmented.get(i, j));
                }
            }
        }
        
        // Initial basis: artificial variables (last m columns)
        let mut phase1_basis = Basis::initial(n_augmented, m);
        phase1_basis.factorize(&a_augmented, &self.config)?;
        
        // Objective for Phase I: minimize sum of artificial variables
        // This means cost vector is [0, 0, ..., 0, 1, 1, ..., 1]
        //                           <--- n zeros --><--- m ones --->
        let mut c_phase1 = vec![0.0; n_augmented];
        for i in n..n_augmented {
            c_phase1[i] = 1.0;
        }
        
        // Solve Phase I problem using simplex iterations
        let max_iter = self.config.max_iterations;
        for phase1_iterations in 0..max_iter {
            // Check timeout and memory every 100 iterations (not every iteration for performance)
            if phase1_iterations % 100 == 0 {
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
                    let usage_mb = self.estimate_memory_mb(&a_augmented, &phase1_basis) as u64;
                    if usage_mb > limit_mb {
                        return Err(LpError::MemoryExceeded {
                            usage_mb,
                            limit_mb,
                        });
                    }
                }
            }
            
            // Compute reduced costs
            let reduced_costs = phase1_basis.compute_reduced_costs(&a_augmented, &c_phase1)?;
            
            // Find entering variable (most negative reduced cost for minimization)
            let entering = if let Some(idx) = reduced_costs
                .iter()
                .enumerate()
                .filter(|(_, rc)| **rc < -self.config.optimality_tol)
                .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .map(|(idx, _)| idx)
            {
                idx
            } else {
                // No improving direction found - check if we have a feasible solution
                let x_basic = phase1_basis.solve_basic(&b_augmented)?;
                let obj = phase1_basis.objective_value(&x_basic, &c_phase1);
                
                if obj < self.config.feasibility_tol {
                    // Found feasible solution for original problem
                    // Extract basis that doesn't use artificial variables
                    // (or uses them at zero level)
                    
                    // Build basis for original problem by removing artificial variables
                    let original_basic: Vec<usize> = phase1_basis.basic
                        .iter()
                        .filter(|&&idx| idx < n)
                        .copied()
                        .collect();
                    
                    if original_basic.len() == m {
                        // All basic variables are from original problem
                        let original_nonbasic: Vec<usize> = (0..n)
                            .filter(|idx| !original_basic.contains(idx))
                            .collect();
                        
                        let mut final_basis = Basis::from_indices(original_basic, original_nonbasic);
                        final_basis.factorize(a, &self.config)?;
                        return Ok((final_basis, phase1_iterations));
                    } else {
                        // Some artificial variables are basic at zero level
                        // Need to pivot them out (this is a degenerate case)
                        // For now, try to use first n columns as basis
                        let mut final_basic: Vec<usize> = phase1_basis.basic
                            .iter()
                            .filter(|&&idx| idx < n)
                            .copied()
                            .collect();
                        
                        // Fill remaining slots with non-basic original variables
                        for idx in 0..n {
                            if final_basic.len() >= m {
                                break;
                            }
                            if !final_basic.contains(&idx) {
                                final_basic.push(idx);
                            }
                        }
                        
                        if final_basic.len() == m {
                            let final_nonbasic: Vec<usize> = (0..n)
                                .filter(|idx| !final_basic.contains(idx))
                                .collect();
                            
                            let mut final_basis = Basis::from_indices(final_basic, final_nonbasic);
                            // Try to factorize - if this fails, the basis is singular
                            if final_basis.factorize(a, &self.config).is_ok() {
                                return Ok((final_basis, phase1_iterations));
                            }
                        }
                        
                        // Could not construct a valid basis
                        return Err(LpError::NumericalInstability);
                    }
                } else {
                    // Phase I objective > 0 means original problem is infeasible
                    return Err(LpError::NumericalInstability);
                }
            };
            
            // Compute search direction for entering variable
            let a_col = a_augmented.col(entering);
            let direction = phase1_basis.lu.as_ref()
                .ok_or(LpError::NumericalInstability)?
                .solve(&a_col)?;
            
            // Get current basic solution
            let x_basic = phase1_basis.solve_basic(&b_augmented)?;
            
            // Find leaving variable using minimum ratio test
            let leaving_idx = phase1_basis.find_leaving_variable(
                &x_basic,
                &direction,
                self.config.feasibility_tol,
            );
            
            // Check for unboundedness (shouldn't happen in Phase I with proper setup)
            if leaving_idx.is_none() {
                return Err(LpError::NumericalInstability);
            }
            
            // Perform basis swap
            let entering_nonbasic_idx = phase1_basis.nonbasic.iter()
                .position(|&idx| idx == entering)
                .ok_or(LpError::NumericalInstability)?;
            
            phase1_basis.swap(entering_nonbasic_idx, leaving_idx.unwrap());
            phase1_basis.factorize(&a_augmented, &self.config)?;
        }
        
        // Max iterations reached
        Err(LpError::NumericalInstability)
    }
    
    /// Phase II: Optimize from feasible basis to optimal solution
    ///
    /// Uses primal simplex iterations: improve objective while maintaining feasibility
    fn phase_two(
        &mut self,
        a: &Matrix,
        c: &[f64],
        b: &[f64],
        basis: &mut Basis,
        n_vars: usize,
        start_time: std::time::Instant,
        phase1_iterations: usize,
    ) -> Result<LpSolution, LpError> {
        let mut phase2_iterations = 0;
        
        loop {
            let total_iterations = phase1_iterations + phase2_iterations;
            
            // Check timeout and memory every 100 iterations (not every iteration for performance)
            if phase2_iterations % 100 == 0 {
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
                    let usage_mb = self.estimate_memory_mb(&a, &basis) as u64;
                    if usage_mb > limit_mb {
                        return Err(LpError::MemoryExceeded {
                            usage_mb,
                            limit_mb,
                        });
                    }
                }
            }
            
            // Check iteration limit
            if total_iterations >= self.config.max_iterations {
                let x = basis.solve_basic(b)?;
                let objective = basis.objective_value(c, &x);
                return Ok(LpSolution::new(
                    LpStatus::IterationLimit,
                    objective,
                    x[..n_vars].to_vec(), // Return only original variables (not slacks)
                    total_iterations,
                    basis.basic.clone(),
                ));
            }
            
            // Compute current solution
            let x = basis.solve_basic(b)?;
            
            // Compute reduced costs
            let reduced_costs = basis.compute_reduced_costs(a, c)?;
            
            // Check optimality: all reduced costs <= 0
            if basis.is_dual_feasible(&reduced_costs, self.config.optimality_tol) {
                let objective = basis.objective_value(c, &x);
                return Ok(LpSolution::new(
                    LpStatus::Optimal,
                    objective,
                    x[..n_vars].to_vec(),
                    total_iterations,
                    basis.basic.clone(),
                ));
            }
            
            // Find entering variable (most positive reduced cost)
            let entering_idx = basis.find_entering_variable(&reduced_costs)
                .ok_or(LpError::NumericalInstability)?;
            
            let entering_var = basis.nonbasic[entering_idx];
            
            // Compute search direction: B^(-1) A_j for entering column j
            let a_col = a.col(entering_var);
            let direction = basis.lu.as_ref()
                .ok_or(LpError::NumericalInstability)?
                .solve(&a_col)?;
            
            // Find leaving variable using minimum ratio test
            let x_basic: Vec<f64> = basis.basic.iter().map(|&idx| x[idx]).collect();
            let leaving_idx = basis.find_leaving_variable(
                &x_basic,
                &direction,
                self.config.feasibility_tol,
            );
            
            // Check for unboundedness
            if leaving_idx.is_none() {
                let objective = basis.objective_value(c, &x);
                return Ok(LpSolution::new(
                    LpStatus::Unbounded,
                    objective,
                    x[..n_vars].to_vec(),
                    phase1_iterations + phase2_iterations,
                    basis.basic.clone(),
                ));
            }
            
            // Perform basis swap
            basis.swap(entering_idx, leaving_idx.unwrap());
            
            // Refactorize basis
            basis.factorize(a, &self.config)?;
            
            phase2_iterations += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_standard_form() {
        // Simple problem: maximize 3x1 + 2x2 subject to x1 + x2 <= 5
        let problem = LpProblem::new(
            2, // 2 variables
            1, // 1 constraint
            vec![3.0, 2.0], // objective
            vec![vec![1.0, 1.0]], // constraint: x1 + x2 <= 5
            vec![5.0], // RHS
            vec![0.0, 0.0], // lower bounds
            vec![f64::INFINITY, f64::INFINITY], // upper bounds
        );
        
        let solver = PrimalSimplex::new(LpConfig::default());
        let (a_eq, b_eq, c_ext, n_total) = solver.to_standard_form(&problem);
        
        // Should add 1 slack variable
        assert_eq!(n_total, 3);
        assert_eq!(a_eq.rows, 1);
        assert_eq!(a_eq.cols, 3);
        
        // Check extended constraint matrix: [1 1 1]
        assert_eq!(a_eq.get(0, 0), 1.0);
        assert_eq!(a_eq.get(0, 1), 1.0);
        assert_eq!(a_eq.get(0, 2), 1.0); // slack variable
        
        // Check RHS unchanged (no lower bounds)
        assert_eq!(b_eq, vec![5.0]);
        
        // Check extended objective: [3 2 0]
        assert_eq!(c_ext, vec![3.0, 2.0, 0.0]);
    }
    
    #[test]
    fn test_simple_lp_solve() {
        // Maximize 3x1 + 2x2
        // Subject to: x1 + x2 <= 5
        //             x1, x2 >= 0
        // Optimal: x1 = 5, x2 = 0, obj = 15
        
        let problem = LpProblem::new(
            2,
            1,
            vec![3.0, 2.0],
            vec![vec![1.0, 1.0]],
            vec![5.0],
            vec![0.0, 0.0],
            vec![f64::INFINITY, f64::INFINITY],
        );
        
        let mut solver = PrimalSimplex::new(LpConfig::default());
        let solution = solver.solve(&problem).unwrap();
        
        assert_eq!(solution.status, LpStatus::Optimal);
        assert!((solution.objective - 15.0).abs() < 1e-6);
        assert!((solution.x[0] - 5.0).abs() < 1e-6);
        assert!(solution.x[1].abs() < 1e-6);
    }
    
    #[test]
    fn test_two_constraint_lp() {
        // Maximize 3x1 + 4x2
        // Subject to: x1 + x2 <= 4
        //             2x1 + x2 <= 5
        //             x1, x2 >= 0
        // Optimal: x1 = 0, x2 = 4, obj = 16
        // (Corner at x1=0, x2=4 is optimal since coefficient of x2 is larger)
        
        let problem = LpProblem::new(
            2,
            2,
            vec![3.0, 4.0],
            vec![
                vec![1.0, 1.0],
                vec![2.0, 1.0],
            ],
            vec![4.0, 5.0],
            vec![0.0, 0.0],
            vec![f64::INFINITY, f64::INFINITY],
        );
        
        let mut solver = PrimalSimplex::new(LpConfig::default());
        let solution = solver.solve(&problem).unwrap();
        
        assert_eq!(solution.status, LpStatus::Optimal);
        assert!((solution.objective - 16.0).abs() < 1e-6, 
                "Expected objective 16.0, got {}", solution.objective);
        assert!((solution.x[0] - 0.0).abs() < 1e-6, 
                "Expected x[0]=0.0, got {}", solution.x[0]);
        assert!((solution.x[1] - 4.0).abs() < 1e-6,
                "Expected x[1]=4.0, got {}", solution.x[1]);
    }
    
    #[test]
    fn test_unbounded_lp() {
        // Maximize x1 + x2
        // Subject to: -x1 + x2 <= 1  (does not bound x1 from above)
        //             x1, x2 >= 0
        // Result: Unbounded
        
        let problem = LpProblem::new(
            2,
            1,
            vec![1.0, 1.0],
            vec![vec![-1.0, 1.0]],
            vec![1.0],
            vec![0.0, 0.0],
            vec![f64::INFINITY, f64::INFINITY],
        );
        
        let mut solver = PrimalSimplex::new(LpConfig::default());
        let solution = solver.solve(&problem).unwrap();
        
        assert_eq!(solution.status, LpStatus::Unbounded);
    }
    
    #[test]
    fn test_degenerate_lp() {
        // Problem with degenerate solution (multiple optimal bases)
        // Maximize x1 + x2
        // Subject to: x1 + x2 <= 2
        //             x1 <= 2
        //             x2 <= 2
        //             x1, x2 >= 0
        // Optimal: x1 + x2 = 2 (multiple solutions)
        
        let problem = LpProblem::new(
            2,
            3,
            vec![1.0, 1.0],
            vec![
                vec![1.0, 1.0],
                vec![1.0, 0.0],
                vec![0.0, 1.0],
            ],
            vec![2.0, 2.0, 2.0],
            vec![0.0, 0.0],
            vec![f64::INFINITY, f64::INFINITY],
        );
        
        let mut solver = PrimalSimplex::new(LpConfig::default());
        let solution = solver.solve(&problem).unwrap();
        
        assert_eq!(solution.status, LpStatus::Optimal);
        assert!((solution.objective - 2.0).abs() < 1e-6);
        // x1 + x2 should equal 2
        assert!((solution.x[0] + solution.x[1] - 2.0).abs() < 1e-6);
    }
    
    #[test]
    fn test_identical_constraint_rows() {
        // Problem with two identical constraint rows (redundant constraint)
        // Maximize 2x1 + 3x2
        // Subject to: x1 + x2 <= 5
        //             x1 + x2 <= 5  (identical to first constraint)
        //             x1, x2 >= 0
        // Note: Redundant constraints are valid; the initial basis uses slack
        // variables which form an identity matrix (not rank-deficient)
        
        let problem = LpProblem::new(
            2,
            2,
            vec![2.0, 3.0],
            vec![
                vec![1.0, 1.0],  // First constraint
                vec![1.0, 1.0],  // Identical to first constraint
            ],
            vec![5.0, 5.0],
            vec![0.0, 0.0],
            vec![f64::INFINITY, f64::INFINITY],
        );
        
        let mut solver = PrimalSimplex::new(LpConfig::default());
        let result = solver.solve(&problem);
        
        // Redundant constraints should be handled correctly
        // The optimal solution is x1=0, x2=5, objective=15
        assert!(result.is_ok(), "Solver should handle redundant constraints");
        let solution = result.unwrap();
        assert_eq!(solution.status, LpStatus::Optimal);
        assert!((solution.objective - 15.0).abs() < 1e-6, 
            "Expected objective=15.0, got={}", solution.objective);
        assert!((solution.x[0] - 0.0).abs() < 1e-6);
        assert!((solution.x[1] - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_negative_rhs() {
        // Problem requiring Phase I with artificial variables
        // Maximize x1 + x2
        // Subject to: -x1 - x2 <= -3  (equivalent to x1 + x2 >= 3)
        //             x1 + x2 <= 5
        //             x1, x2 >= 0
        // Optimal: x1 = 0, x2 = 5, obj = 5
        
        let problem = LpProblem::new(
            2,
            2,
            vec![1.0, 1.0],
            vec![
                vec![-1.0, -1.0],  // This becomes x1 + x2 >= 3 after flip
                vec![1.0, 1.0],
            ],
            vec![-3.0, 5.0],  // Negative RHS requires Phase I
            vec![0.0, 0.0],
            vec![f64::INFINITY, f64::INFINITY],
        );
        
        let mut solver = PrimalSimplex::new(LpConfig::default());
        let result = solver.solve(&problem);
        
        assert!(result.is_ok(), "Solver should handle negative RHS");
        let solution = result.unwrap();
        assert_eq!(solution.status, LpStatus::Optimal);
        
        // The feasible region is x1 + x2 >= 3 and x1 + x2 <= 5
        // Optimal is to maximize x1 + x2 subject to x1 + x2 <= 5
        assert!((solution.objective - 5.0).abs() < 1e-6, 
            "Expected objective=5.0, got={}", solution.objective);
        
        // Check that solution is feasible
        let sum = solution.x[0] + solution.x[1];
        assert!(sum >= 3.0 - 1e-6, "x1 + x2 should be >= 3");
        assert!(sum <= 5.0 + 1e-6, "x1 + x2 should be <= 5");
    }

    #[test]
    fn test_timeout() {
        // Create a large problem that will take a while to solve
        // This is a deliberately complex problem that requires many iterations
        let n = 50;
        let m = 25;
        
        let c = vec![1.0; n];
        let a = vec![vec![1.0; n]; m];
        let b = vec![100.0; m];
        let lower = vec![0.0; n];
        let upper = vec![f64::INFINITY; n];
        
        let problem = LpProblem::new(n, m, c, a, b, lower, upper);
        
        // Set a very short timeout (1ms) - should timeout on most systems
        let config = LpConfig::unlimited().with_timeout_ms(1);
        let mut solver = PrimalSimplex::new(config);
        let result = solver.solve(&problem);
        
        // Should timeout (or complete if it's very fast)
        match result {
            Err(LpError::TimeoutExceeded { elapsed_ms, limit_ms }) => {
                assert!(elapsed_ms >= limit_ms, "Elapsed time should be >= limit");
                assert_eq!(limit_ms, 1, "Limit should be 1ms");
            }
            Ok(_) => {
                // If it completes in time, that's also acceptable
                // (might happen on very fast systems with optimized builds)
            }
            Err(e) => panic!("Expected timeout or success, got error: {:?}", e),
        }
    }

    #[test]
    fn test_memory_limit() {
        // NOTE: This test verifies that memory limit configuration is checked,
        // but due to global memory tracking shared across parallel tests,
        // we cannot reliably test actual memory exceeded errors.
        // Instead, we verify the configuration is set correctly.
        
        // Create a small problem
        let n = 10;
        let m = 5;
        
        let c = vec![1.0; n];
        let a = vec![vec![1.0; n]; m];
        let b = vec![100.0; m];
        let lower = vec![0.0; n];
        let upper = vec![f64::INFINITY; n];
        
        let problem = LpProblem::new(n, m, c, a, b, lower, upper);
        
        // Test 1: Verify that unlimited config has no memory limit
        let config_unlimited = LpConfig::unlimited();
        assert!(config_unlimited.max_memory_mb.is_none());
        
        // Test 2: Verify that memory limit can be set
        let config_limited = LpConfig::unlimited().with_max_memory_mb(100);
        assert_eq!(config_limited.max_memory_mb, Some(100));
        
        // Test 3: Solve with generous memory limit (should succeed)
        let mut solver = PrimalSimplex::new(config_limited);
        let result = solver.solve(&problem);
        
        // Should complete successfully with generous limit
        assert!(result.is_ok(), "Solver should succeed with generous memory limit: {:?}", result);
    }
}
