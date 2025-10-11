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
    
    /// Maximum time to spend solving (in milliseconds)
    /// Default: None (no timeout)
    /// Set this to enforce time limits matching SolverConfig::timeout_ms
    pub timeout_ms: Option<u64>,
    
    /// Maximum memory usage (in MB) during solving
    /// Default: None (no memory limit)
    /// Set this to enforce memory limits matching SolverConfig::max_memory_mb
    pub max_memory_mb: Option<u64>,
}

impl Default for LpConfig {
    fn default() -> Self {
        Self {
            feasibility_tol: 1e-6,
            optimality_tol: 1e-6,
            max_iterations: 10000,
            check_stability: true,
            timeout_ms: Some(60000),  // Default 60 seconds (matches SolverConfig)
            max_memory_mb: Some(2048), // Default 2GB (matches SolverConfig)
        }
    }
}

impl LpConfig {
    /// Create an unlimited configuration (no timeout or memory limits)
    ///
    /// Use with caution - problems may run indefinitely
    ///
    /// # Example
    /// ```ignore
    /// let config = LpConfig::unlimited();
    /// ```
    #[must_use]
    pub fn unlimited() -> Self {
        Self {
            feasibility_tol: 1e-6,
            optimality_tol: 1e-6,
            max_iterations: 10000,
            check_stability: true,
            timeout_ms: None,
            max_memory_mb: None,
        }
    }
    
    /// Set the timeout in milliseconds
    ///
    /// This matches the SolverConfig::timeout_ms parameter
    ///
    /// # Example
    /// ```ignore
    /// let config = LpConfig::default().with_timeout_ms(5000); // 5 second timeout
    /// ```
    #[must_use]
    pub fn with_timeout_ms(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = Some(timeout_ms);
        self
    }
    
    /// Remove timeout limit
    ///
    /// # Example
    /// ```ignore
    /// let config = LpConfig::default().without_timeout();
    /// ```
    #[must_use]
    pub fn without_timeout(mut self) -> Self {
        self.timeout_ms = None;
        self
    }
    
    /// Set the maximum memory usage in MB
    ///
    /// This matches the SolverConfig::max_memory_mb parameter
    ///
    /// # Example
    /// ```ignore
    /// let config = LpConfig::default().with_max_memory_mb(1024); // 1GB limit
    /// ```
    #[must_use]
    pub fn with_max_memory_mb(mut self, max_memory_mb: u64) -> Self {
        self.max_memory_mb = Some(max_memory_mb);
        self
    }
    
    /// Remove memory limit
    ///
    /// # Example
    /// ```ignore
    /// let config = LpConfig::default().without_memory_limit();
    /// ```
    #[must_use]
    pub fn without_memory_limit(mut self) -> Self {
        self.max_memory_mb = None;
        self
    }
}

/// Linear Programming problem in standard form
///
/// Maximize: c^T x
/// Subject to: Ax <= b, l <= x <= u
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
    
    /// Optional: Basic variable indices for warm-starting (length m_constraints)
    /// Used by Dual Simplex to start from a previous solution
    pub basic_indices: Option<Vec<usize>>,
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
            basic_indices: None,
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

/// Solution to LP problem with comprehensive statistics
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
    
    /// Statistics about the solve process
    pub stats: LpStats,
}

/// Statistics collected during LP solving
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LpStats {
    /// Total time spent solving (including both Phase I and Phase II)
    pub solve_time_ms: f64,
    
    /// Time spent in Phase I (finding initial feasible solution)
    pub phase1_time_ms: f64,
    
    /// Time spent in Phase II (optimization)
    pub phase2_time_ms: f64,
    
    /// Number of iterations in Phase I
    pub phase1_iterations: usize,
    
    /// Number of iterations in Phase II  
    pub phase2_iterations: usize,
    
    /// Peak memory usage during solving (MB)
    pub peak_memory_mb: f64,
    
    /// Number of variables in the problem (original count)
    pub n_variables: usize,
    
    /// Number of constraints in the problem (original count)
    pub n_constraints: usize,
    
    /// Number of basis factorizations performed
    pub factorizations: usize,
    
    /// Whether Phase I was needed (false = initial basis was feasible)
    pub phase1_needed: bool,
}

impl LpStats {
    /// Create new LP statistics
    pub fn new(
        solve_time_ms: f64,
        phase1_time_ms: f64,
        phase2_time_ms: f64,
        phase1_iterations: usize,
        phase2_iterations: usize,
        peak_memory_mb: f64,
        n_variables: usize,
        n_constraints: usize,
        factorizations: usize,
        phase1_needed: bool,
    ) -> Self {
        Self {
            solve_time_ms,
            phase1_time_ms,
            phase2_time_ms,
            phase1_iterations,
            phase2_iterations,
            peak_memory_mb,
            n_variables,
            n_constraints,
            factorizations,
            phase1_needed,
        }
    }
    
    /// Get total iterations (Phase I + Phase II)
    pub fn total_iterations(&self) -> usize {
        self.phase1_iterations + self.phase2_iterations
    }
    
    /// Get average time per iteration (microseconds)
    pub fn time_per_iteration_us(&self) -> f64 {
        if self.total_iterations() > 0 {
            (self.solve_time_ms * 1000.0) / self.total_iterations() as f64
        } else {
            0.0
        }
    }
    
    /// Get average time per factorization (milliseconds)
    pub fn time_per_factorization_ms(&self) -> f64 {
        if self.factorizations > 0 {
            self.solve_time_ms / self.factorizations as f64
        } else {
            0.0
        }
    }
    
    /// Display a comprehensive summary of the LP solving statistics
    pub fn display_summary(&self) {
        println!("=== LP Solver Statistics ===");
        println!("Problem size: {} variables, {} constraints", self.n_variables, self.n_constraints);
        println!("Total time: {:.3}ms", self.solve_time_ms);
        println!("Peak memory: {:.2}MB", self.peak_memory_mb);
        println!();
        
        if self.phase1_needed {
            println!("Phase I (feasibility): {:.3}ms, {} iterations", 
                     self.phase1_time_ms, self.phase1_iterations);
        } else {
            println!("Phase I: Skipped (initial basis feasible)");
        }
        println!("Phase II (optimization): {:.3}ms, {} iterations", 
                 self.phase2_time_ms, self.phase2_iterations);
        println!();
        
        println!("Total iterations: {}", self.total_iterations());
        println!("Basis factorizations: {}", self.factorizations);
        println!("Average: {:.2}μs/iteration", self.time_per_iteration_us());
        if self.factorizations > 0 {
            println!("Average: {:.3}ms/factorization", self.time_per_factorization_ms());
        }
        println!("============================");
    }
}

impl LpSolution {
    /// Create a new solution with statistics
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
            stats: LpStats::default(),
        }
    }
    
    /// Create a new solution with full statistics
    pub fn with_stats(
        status: LpStatus,
        objective: f64,
        x: Vec<f64>,
        iterations: usize,
        basic_indices: Vec<usize>,
        stats: LpStats,
    ) -> Self {
        Self {
            status,
            objective,
            x,
            iterations,
            basic_indices,
            stats,
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
    
    /// Timeout exceeded during solve
    TimeoutExceeded { elapsed_ms: u64, limit_ms: u64 },
    
    /// Memory limit exceeded during solve
    MemoryExceeded { usage_mb: u64, limit_mb: u64 },
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
            LpError::TimeoutExceeded { elapsed_ms, limit_ms } => {
                write!(f, "Timeout exceeded: {}ms elapsed, limit was {}ms", elapsed_ms, limit_ms)
            }
            LpError::MemoryExceeded { usage_mb, limit_mb } => {
                write!(f, "Memory limit exceeded: {}MB used, limit was {}MB", usage_mb, limit_mb)
            }
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
