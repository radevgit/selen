//! # CSP Solver
//!
//! A constraint satisfaction problem (CSP) solver library.
//!
//! This library provides tools for solving constraint satisfaction problems,
//! including constraint propagation and search algorithms.
//!
//! ## Features
//!
//! - Constraint propagation
//! - Backtracking search
//! - Domain filtering
//! - Support for various constraint types
//!
//! ## Example
//!
//! ```rust
//! use csp::*;
//!
//! // Example usage will be added as the library develops
//! ```

// pub mod constraints;
// pub mod domain;
// pub mod propagation;
// pub mod search;
// pub mod solver;
// pub mod variable;

// Re-export commonly used types
// pub use constraints::*;
// pub use domain::*;
// pub use propagation::*;
// pub use search::*;
// pub use solver::*;
// pub use variable::*;

/// CSP Solver error types
#[derive(Debug, Clone, PartialEq)]
pub enum CspError {
    /// No solution exists for the given constraints
    NoSolution,
    /// Invalid constraint specification
    InvalidConstraint(String),
    /// Invalid variable specification
    InvalidVariable(String),
    /// Domain is empty or invalid
    InvalidDomain(String),
    /// Generic solver error
    SolverError(String),
}

impl std::fmt::Display for CspError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CspError::NoSolution => write!(f, "No solution exists"),
            CspError::InvalidConstraint(msg) => write!(f, "Invalid constraint: {}", msg),
            CspError::InvalidVariable(msg) => write!(f, "Invalid variable: {}", msg),
            CspError::InvalidDomain(msg) => write!(f, "Invalid domain: {}", msg),
            CspError::SolverError(msg) => write!(f, "Solver error: {}", msg),
        }
    }
}

impl std::error::Error for CspError {}

/// Result type for CSP operations
pub type CspResult<T> = Result<T, CspError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csp_error_display() {
        let error = CspError::NoSolution;
        assert_eq!(error.to_string(), "No solution exists");
    }
}
