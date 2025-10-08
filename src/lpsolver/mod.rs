//! Linear Programming solver using Dual Simplex method
//!
//! Solves problems of the form:
//!   maximize:   c^T x
//!   subject to: Ax <= b
//!               l <= x <= u (variable bounds)
//!
//! This implementation uses:
//! - Revised Simplex method (more efficient than tableau)
//! - Dual Simplex for warm starting (crucial for CSP integration)
//! - LU factorization for basis updates
//! - Dense matrix storage (suitable for ~100 variables)

pub mod types;
pub mod matrix;
pub mod lu;
pub mod basis;
pub mod simplex_primal;
pub mod simplex_dual;

// #[cfg(test)]
// mod tests;

pub use types::{LpProblem, LpSolution, LpStatus, LpError, LpConfig};
pub use matrix::Matrix;

/// Solve LP problem using Primal Simplex
///
/// Use this for initial solves when no previous solution exists.
pub fn solve(problem: &LpProblem) -> Result<LpSolution, LpError> {
    solve_with_config(problem, &LpConfig::default())
}

/// Solve LP problem with custom configuration
pub fn solve_with_config(problem: &LpProblem, config: &LpConfig) -> Result<LpSolution, LpError> {
    problem.validate()?;
    let mut solver = simplex_primal::PrimalSimplex::new(config.clone());
    solver.solve(problem)
}

/// Solve LP problem using Dual Simplex with warm starting
///
/// Use this when adding constraints to a previously solved problem.
/// Much faster than solving from scratch.
pub fn solve_warmstart(
    problem: &LpProblem,
    previous: &LpSolution,
    config: &LpConfig,
) -> Result<LpSolution, LpError> {
    problem.validate()?;
    
    // Create problem with warm start basis
    let mut problem_warmstart = problem.clone();
    problem_warmstart.basic_indices = Some(previous.basic_indices.clone());
    
    let mut solver = simplex_dual::DualSimplex::new(config.clone());
    solver.solve(&problem_warmstart)
}
