//! Type definitions for LP solver

use std::fmt;

/// Configuration for LP solver
#[derive(Debug, Clone)]
pub struct LpConfig {
    /// Feasibility tolerance (default: 1e-6)
    pub feasibility_tol: f64,
    
    /// Optimality tolerance (default: 1e-6)
    pub optimality_tol: f64,
    
    /// Maximum iterations (default: 10000)
    pub max_iterations: usize,
    
    /// Enable numerical stability checks (default: true)
    pub check_stability: bool,
}

impl Default for LpConfig {
    fn default() -> Self {
        Self {
            feasibility_tol: 1e-6,
            optimality_tol: 1e-6,
            max_iterations: 10000,
            check_stability: true,
        }
    }
}

/// LP problem in standard form:
/// maximize c^T x subject to Ax <= b, l <= x <= u
#[derive(Debug, Clone)]
pub struct LpProblem {
    /// Number of variables
    pub n_vars: usize,
    
    /// Number of constraints
    pub n_constraints: usize,
    
    /// Objective coefficients (length n_vars)
    /// For minimization, negate these coefficients
    pub c: Vec<f64>,
    
    /// Constraint matrix A (n_constraints × n_vars)
    /// Each row represents one constraint: a_i^T x <= b_i
    pub a: Vec<Vec<f64>>,
    
    /// Right-hand side (length n_constraints)
    pub b: Vec<f64>,
    
    /// Variable lower bounds (length n_vars)
    pub lower_bounds: Vec<f64>,
    
    /// Variable upper bounds (length n_vars)
    pub upper_bounds: Vec<f64>,
}

impl LpProblem {
    /// Create a new LP problem
    pub fn new(
        n_vars: usize,
        n_constraints: usize,
        c: Vec<f64>,
        a: Vec<Vec<f64>>,
        b: Vec<f64>,
        lower_bounds: Vec<f64>,
        upper_bounds: Vec<f64>,
    ) -> Self {
        Self {
            n_vars,
            n_constraints,
            c,
            a,
            b,
            lower_bounds,
            upper_bounds,
        }
    }
    
    /// Validate problem dimensions and values
    pub fn validate(&self) -> Result<(), LpError> {
        // Check dimensions
        if self.c.len() != self.n_vars {
            return Err(LpError::ObjectiveDimensionMismatch {
                expected: self.n_vars,
                actual: self.c.len(),
            });
        }
        
        if self.a.len() != self.n_constraints {
            return Err(LpError::ConstraintCountMismatch {
                expected: self.n_constraints,
                actual: self.a.len(),
            });
        }
        
        for (i, row) in self.a.iter().enumerate() {
            if row.len() != self.n_vars {
                return Err(LpError::ConstraintRowDimensionMismatch {
                    row: i,
                    expected: self.n_vars,
                    actual: row.len(),
                });
            }
        }
        
        if self.b.len() != self.n_constraints {
            return Err(LpError::RhsDimensionMismatch {
                expected: self.n_constraints,
                actual: self.b.len(),
            });
        }
        
        if self.lower_bounds.len() != self.n_vars {
            return Err(LpError::LowerBoundsDimensionMismatch {
                expected: self.n_vars,
                actual: self.lower_bounds.len(),
            });
        }
        
        if self.upper_bounds.len() != self.n_vars {
            return Err(LpError::UpperBoundsDimensionMismatch {
                expected: self.n_vars,
                actual: self.upper_bounds.len(),
            });
        }
        
        // Check bounds are valid
        for i in 0..self.n_vars {
            if self.lower_bounds[i] > self.upper_bounds[i] {
                return Err(LpError::InvalidVariableBounds {
                    variable: i,
                    lower: self.lower_bounds[i],
                    upper: self.upper_bounds[i],
                });
            }
        }
        
        // Check for NaN/Inf
        for val in &self.c {
            if !val.is_finite() {
                return Err(LpError::ObjectiveNotFinite);
            }
        }
        
        for row in &self.a {
            for val in row {
                if !val.is_finite() {
                    return Err(LpError::ConstraintMatrixNotFinite);
                }
            }
        }
        
        for val in &self.b {
            if !val.is_finite() {
                return Err(LpError::RhsNotFinite);
            }
        }
        
        Ok(())
    }
}

/// Solution to LP problem
#[derive(Debug, Clone)]
pub struct LpSolution {
    /// Solution status
    pub status: LpStatus,
    
    /// Objective value at solution
    pub objective: f64,
    
    /// Variable values (length n_vars)
    pub x: Vec<f64>,
    
    /// Number of iterations performed
    pub iterations: usize,
    
    /// Indices of basic variables (for warm starting)
    pub basic_indices: Vec<usize>,
}

impl LpSolution {
    /// Create a new solution
    pub fn new(
        status: LpStatus,
        objective: f64,
        x: Vec<f64>,
        iterations: usize,
        basic_indices: Vec<usize>,
    ) -> Self {
        Self {
            status,
            objective,
            x,
            iterations,
            basic_indices,
        }
    }
}

/// Status of LP solution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LpStatus {
    /// Optimal solution found
    Optimal,
    
    /// Problem is infeasible (no solution exists)
    Infeasible,
    
    /// Problem is unbounded (objective can be infinite)
    Unbounded,
    
    /// Iteration limit reached
    IterationLimit,
    
    /// Numerical instability detected
    NumericalError,
}

impl fmt::Display for LpStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LpStatus::Optimal => write!(f, "Optimal"),
            LpStatus::Infeasible => write!(f, "Infeasible"),
            LpStatus::Unbounded => write!(f, "Unbounded"),
            LpStatus::IterationLimit => write!(f, "Iteration limit reached"),
            LpStatus::NumericalError => write!(f, "Numerical error"),
        }
    }
}

/// Errors that can occur during LP solving
#[derive(Debug, Clone, PartialEq)]
pub enum LpError {
    /// Objective vector dimension mismatch
    ObjectiveDimensionMismatch { expected: usize, actual: usize },
    
    /// Constraint matrix row count mismatch
    ConstraintCountMismatch { expected: usize, actual: usize },
    
    /// Constraint row dimension mismatch
    ConstraintRowDimensionMismatch { row: usize, expected: usize, actual: usize },
    
    /// RHS vector dimension mismatch
    RhsDimensionMismatch { expected: usize, actual: usize },
    
    /// Lower bounds dimension mismatch
    LowerBoundsDimensionMismatch { expected: usize, actual: usize },
    
    /// Upper bounds dimension mismatch
    UpperBoundsDimensionMismatch { expected: usize, actual: usize },
    
    /// Variable bounds are invalid (lower > upper)
    InvalidVariableBounds { variable: usize, lower: f64, upper: f64 },
    
    /// Objective contains NaN or Inf
    ObjectiveNotFinite,
    
    /// Constraint matrix contains NaN or Inf
    ConstraintMatrixNotFinite,
    
    /// RHS vector contains NaN or Inf
    RhsNotFinite,
    
    /// Numerical instability detected during solve
    NumericalInstability,
    
    /// Singular basis matrix encountered
    SingularBasis,
}

impl fmt::Display for LpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LpError::ObjectiveDimensionMismatch { expected, actual } => {
                write!(f, "Objective vector length {} != n_vars {}", actual, expected)
            }
            LpError::ConstraintCountMismatch { expected, actual } => {
                write!(f, "Constraint matrix rows {} != n_constraints {}", actual, expected)
            }
            LpError::ConstraintRowDimensionMismatch { row, expected, actual } => {
                write!(f, "Constraint row {} length {} != n_vars {}", row, actual, expected)
            }
            LpError::RhsDimensionMismatch { expected, actual } => {
                write!(f, "RHS vector length {} != n_constraints {}", actual, expected)
            }
            LpError::LowerBoundsDimensionMismatch { expected, actual } => {
                write!(f, "Lower bounds length {} != n_vars {}", actual, expected)
            }
            LpError::UpperBoundsDimensionMismatch { expected, actual } => {
                write!(f, "Upper bounds length {} != n_vars {}", actual, expected)
            }
            LpError::InvalidVariableBounds { variable, lower, upper } => {
                write!(f, "Variable {} has lower bound {} > upper bound {}", variable, lower, upper)
            }
            LpError::ObjectiveNotFinite => write!(f, "Objective contains NaN or Inf"),
            LpError::ConstraintMatrixNotFinite => write!(f, "Constraint matrix contains NaN or Inf"),
            LpError::RhsNotFinite => write!(f, "RHS contains NaN or Inf"),
            LpError::NumericalInstability => write!(f, "Numerical instability detected"),
            LpError::SingularBasis => write!(f, "Singular basis matrix"),
        }
    }
}

impl std::error::Error for LpError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lp_problem_validation() {
        // Valid problem
        let problem = LpProblem::new(
            2, // 2 variables
            1, // 1 constraint
            vec![1.0, 2.0], // objective
            vec![vec![1.0, 1.0]], // constraint: x + y <= 10
            vec![10.0], // RHS
            vec![0.0, 0.0], // lower bounds
            vec![f64::INFINITY, f64::INFINITY], // upper bounds
        );
        assert!(problem.validate().is_ok());
        
        // Invalid: objective wrong size
        let problem = LpProblem::new(
            2,
            1,
            vec![1.0], // Wrong size!
            vec![vec![1.0, 1.0]],
            vec![10.0],
            vec![0.0, 0.0],
            vec![f64::INFINITY, f64::INFINITY],
        );
        assert_eq!(
            problem.validate(),
            Err(LpError::ObjectiveDimensionMismatch { expected: 2, actual: 1 })
        );
        
        // Invalid: lower > upper
        let problem = LpProblem::new(
            2,
            1,
            vec![1.0, 2.0],
            vec![vec![1.0, 1.0]],
            vec![10.0],
            vec![5.0, 0.0], // lower
            vec![3.0, f64::INFINITY], // upper < lower for var 0!
        );
        assert_eq!(
            problem.validate(),
            Err(LpError::InvalidVariableBounds { variable: 0, lower: 5.0, upper: 3.0 })
        );
        
        // Invalid: constraint matrix has NaN
        let problem = LpProblem::new(
            2,
            1,
            vec![1.0, 2.0],
            vec![vec![f64::NAN, 1.0]],
            vec![10.0],
            vec![0.0, 0.0],
            vec![f64::INFINITY, f64::INFINITY],
        );
        assert_eq!(problem.validate(), Err(LpError::ConstraintMatrixNotFinite));
        
        // Invalid: RHS dimension mismatch
        let problem = LpProblem::new(
            2,
            1,
            vec![1.0, 2.0],
            vec![vec![1.0, 1.0]],
            vec![10.0, 20.0], // Wrong size!
            vec![0.0, 0.0],
            vec![f64::INFINITY, f64::INFINITY],
        );
        assert_eq!(
            problem.validate(),
            Err(LpError::RhsDimensionMismatch { expected: 1, actual: 2 })
        );
    }

    #[test]
    fn test_lp_config_default() {
        let config = LpConfig::default();
        assert_eq!(config.feasibility_tol, 1e-6);
        assert_eq!(config.optimality_tol, 1e-6);
        assert_eq!(config.max_iterations, 10000);
        assert!(config.check_stability);
    }
}
