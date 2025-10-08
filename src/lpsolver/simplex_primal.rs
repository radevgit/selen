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
    
    /// Solve an LP problem using Primal Simplex
    ///
    /// Assumes problem is in standard form with slack variables already added
    pub fn solve(&mut self, problem: &LpProblem) -> Result<LpSolution, LpError> {
        // Validate problem
        problem.validate()?;
        
        // Convert inequality constraints to standard form (Ax = b) by adding slacks
        let (a_eq, c_extended, n_total) = self.to_standard_form(problem);
        
        // Phase I: Find initial feasible basis
        let mut basis = self.phase_one(&a_eq, &problem.b)?;
        
        // Phase II: Optimize from feasible basis
        self.phase_two(&a_eq, &c_extended, &problem.b, &mut basis, n_total)
    }
    
    /// Convert inequality constraints Ax <= b to equality Ax + s = b
    /// Returns (A_extended, c_extended, total_vars)
    fn to_standard_form(&self, problem: &LpProblem) -> (Matrix, Vec<f64>, usize) {
        let m = problem.n_constraints;
        let n = problem.n_vars;
        let n_total = n + m; // Original variables + slack variables
        
        // Create extended constraint matrix [A | I]
        let mut a_extended = Matrix::zeros(m, n_total);
        
        // Copy original constraints
        for i in 0..m {
            for j in 0..n {
                a_extended.set(i, j, problem.a[i][j]);
            }
        }
        
        // Add slack variables (identity matrix)
        for i in 0..m {
            a_extended.set(i, n + i, 1.0);
        }
        
        // Extend objective vector (slacks have 0 cost)
        let mut c_extended = problem.c.clone();
        c_extended.extend(vec![0.0; m]);
        
        (a_extended, c_extended, n_total)
    }
    
    /// Phase I: Find initial feasible basis using artificial variables
    ///
    /// Solves auxiliary problem: minimize sum of artificial variables
    /// Returns feasible basis if one exists
    fn phase_one(&mut self, a: &Matrix, b: &[f64]) -> Result<Basis, LpError> {
        let m = a.rows;
        let n = a.cols;
        
        // First, try the identity basis (slack variables)
        let mut basis = Basis::initial(n, m);
        
        // Factorize initial basis
        basis.factorize(a, &self.config)?;
        
        // Check if initial basis is feasible
        let x = basis.solve_basic(b)?;
        
        if basis.is_primal_feasible(&x, self.config.feasibility_tol) {
            return Ok(basis);
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
        for _iter in 0..max_iter {
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
                        return Ok(final_basis);
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
                                return Ok(final_basis);
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
    ) -> Result<LpSolution, LpError> {
        let mut iterations = 0;
        
        loop {
            // Check iteration limit
            if iterations >= self.config.max_iterations {
                let x = basis.solve_basic(b)?;
                let objective = basis.objective_value(c, &x);
                return Ok(LpSolution::new(
                    LpStatus::IterationLimit,
                    objective,
                    x[..n_vars].to_vec(), // Return only original variables (not slacks)
                    iterations,
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
                    iterations,
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
                    iterations,
                    basis.basic.clone(),
                ));
            }
            
            // Perform basis swap
            basis.swap(entering_idx, leaving_idx.unwrap());
            
            // Refactorize basis
            basis.factorize(a, &self.config)?;
            
            iterations += 1;
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
        
        let mut solver = PrimalSimplex::new(LpConfig::default());
        let (a_eq, c_ext, n_total) = solver.to_standard_form(&problem);
        
        // Should add 1 slack variable
        assert_eq!(n_total, 3);
        assert_eq!(a_eq.rows, 1);
        assert_eq!(a_eq.cols, 3);
        
        // Check extended constraint matrix: [1 1 1]
        assert_eq!(a_eq.get(0, 0), 1.0);
        assert_eq!(a_eq.get(0, 1), 1.0);
        assert_eq!(a_eq.get(0, 2), 1.0); // slack variable
        
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
}
