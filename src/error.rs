/// Simple error types for the CSP solver
///
/// This enum covers the basic failure modes that can occur during solving.
/// It's designed to be simple and lightweight for a small solver.
#[derive(Debug, Clone, PartialEq)]
pub enum SolverError {
    /// No solution exists for the given constraints
    NoSolution,
    
    /// Solving operation timed out
    Timeout,
    
    /// Memory limit was exceeded during solving
    MemoryLimit,
    
    /// Invalid constraint was provided
    InvalidConstraint(String),
    
    /// Conflicting constraints detected (unsatisfiable)
    ConflictingConstraints,
    
    /// Invalid variable domain (e.g., min > max)
    InvalidDomain(String),
    
    /// Variable ID is invalid or out of bounds
    InvalidVariable(String),
    
    /// Internal solver error (should not happen in normal use)
    InternalError(String),
}

impl std::fmt::Display for SolverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSolution => write!(f, "No solution found"),
            Self::Timeout => write!(f, "Solving timed out"),
            Self::MemoryLimit => write!(f, "Memory limit exceeded"),
            Self::InvalidConstraint(msg) => write!(f, "Invalid constraint: {}", msg),
            Self::ConflictingConstraints => write!(f, "Conflicting constraints detected"),
            Self::InvalidDomain(msg) => write!(f, "Invalid domain: {}", msg),
            Self::InvalidVariable(msg) => write!(f, "Invalid variable: {}", msg),
            Self::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for SolverError {}

/// Convenience type alias for Results that can fail with SolverError
pub type SolverResult<T> = Result<T, SolverError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        assert_eq!(SolverError::NoSolution.to_string(), "No solution found");
        assert_eq!(SolverError::Timeout.to_string(), "Solving timed out");
        assert_eq!(
            SolverError::InvalidConstraint("x > x".to_string()).to_string(),
            "Invalid constraint: x > x"
        );
    }

    #[test]
    fn test_error_equality() {
        assert_eq!(SolverError::NoSolution, SolverError::NoSolution);
        assert_eq!(
            SolverError::InvalidDomain("empty".to_string()),
            SolverError::InvalidDomain("empty".to_string())
        );
    }

    #[test]
    fn test_error_is_error_trait() {
        let err: Box<dyn std::error::Error> = Box::new(SolverError::NoSolution);
        assert_eq!(err.to_string(), "No solution found");
    }
}