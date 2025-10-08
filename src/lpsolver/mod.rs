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
// mod simplex_primal;
// mod simplex_dual;

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
pub fn solve_with_config(_problem: &LpProblem, _config: &LpConfig) -> Result<LpSolution, LpError> {
    // TODO: Implement once simplex_primal module is created
    // problem.validate()?;
    // simplex_primal::PrimalSimplex::solve(problem.clone(), config)
    todo!("Implement Primal Simplex solver")
}

/// Solve LP problem using Dual Simplex with warm starting
///
/// Use this when adding constraints to a previously solved problem.
/// Much faster than solving from scratch.
pub fn solve_warmstart(
    _problem: &LpProblem,
    _previous: &LpSolution,
    _config: &LpConfig,
) -> Result<LpSolution, LpError> {
    // TODO: Implement once simplex_dual module is created
    // problem.validate()?;
    // simplex_dual::DualSimplex::warm_start(problem.clone(), previous, config)
    todo!("Implement Dual Simplex solver")
}
