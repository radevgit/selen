//! Error handling for the CSP solver.
//!
//! This module provides comprehensive error types for all failure modes that can occur
//! during constraint solving. All public solver methods return `Result<T, SolverError>`
//! for consistent error handling.
//!
//! # Error Categories
//!
//! - **No Solution**: The constraints are unsatisfiable
//! - **Timeout**: Solving took longer than the configured limit
//! - **Memory Limit**: Solver exceeded memory usage limits
//! - **Invalid Input**: Problems with constraints or variable definitions
//! - **Internal Errors**: Unexpected solver failures
//!
//! # Example
//!
//! ```rust
//! use selen::prelude::*;
//!
//! let mut m = Model::default();
//! let x = m.int(1, 5);
//! let y = m.int(1, 5);
//! m.new(x.gt(y));
//! m.new(y.gt(x));  // Contradictory constraint
//!
//! match m.solve() {
//!     Ok(solution) => println!("Found solution: x={:?}, y={:?}", solution[x], solution[y]),
//!     Err(SolverError::NoSolution { context, .. }) => {
//!         println!("No solution exists: {:?}", context);
//!     }
//!     Err(SolverError::Timeout { elapsed_seconds, .. }) => {
//!         println!("Timeout after {:?} seconds", elapsed_seconds);
//!     }
//!     Err(e) => println!("Other error: {}", e),
//! }
//! ```

/// Simple error types for the CSP solver
///
/// This enum covers the basic failure modes that can occur during solving.
/// Each error includes contextual information to help with debugging.
#[derive(Debug, Clone, PartialEq)]
pub enum SolverError {
    /// No solution exists for the given constraints
    NoSolution {
        /// Additional context about why no solution was found
        context: Option<String>,
        /// Number of variables involved
        variable_count: Option<usize>,
        /// Number of constraints checked
        constraint_count: Option<usize>,
    },
    
    /// Solving operation timed out
    Timeout {
        /// How long the solver ran before timing out
        elapsed_seconds: Option<f64>,
        /// What operation was being performed when timeout occurred
        operation: Option<String>,
    },
    
    /// Memory limit was exceeded during solving
    MemoryLimit {
        /// Approximate memory usage when limit was hit (in MB)
        usage_mb: Option<usize>,
        /// Memory limit that was exceeded (in MB)
        limit_mb: Option<usize>,
    },
    
    /// Invalid constraint was provided
    InvalidConstraint {
        /// Description of what makes the constraint invalid
        message: String,
        /// Name or identifier of the problematic constraint
        constraint_name: Option<String>,
        /// Variables involved in the constraint
        variables: Option<Vec<String>>,
    },
    
    /// Conflicting constraints detected (unsatisfiable)
    ConflictingConstraints {
        /// Names or descriptions of the conflicting constraints
        constraint_names: Option<Vec<String>>,
        /// Variables involved in the conflict
        variables: Option<Vec<String>>,
        /// Additional context about the conflict
        context: Option<String>,
    },
    
    /// Invalid variable domain (e.g., min > max)
    InvalidDomain {
        /// Description of the domain problem
        message: String,
        /// Name or identifier of the variable
        variable_name: Option<String>,
        /// The problematic domain bounds
        domain_info: Option<String>,
    },
    
    /// Variable ID is invalid or out of bounds
    InvalidVariable {
        /// Description of what makes the variable invalid
        message: String,
        /// Variable identifier that was invalid
        variable_id: Option<String>,
        /// Expected range or valid identifiers
        expected: Option<String>,
    },
    
    /// Internal solver error (should not happen in normal use)
    InternalError {
        /// Description of the internal error
        message: String,
        /// File and line where the error occurred
        location: Option<String>,
        /// Additional debugging context
        debug_info: Option<String>,
    },
    
    /// Invalid input provided to a function
    InvalidInput {
        /// Description of what makes the input invalid
        message: String,
        /// Name of the function that received invalid input
        function_name: Option<String>,
        /// Expected input format or constraints
        expected: Option<String>,
    },
}

impl std::fmt::Display for SolverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSolution { context, variable_count, constraint_count } => {
                write!(f, "No solution found")?;
                if let Some(ctx) = context {
                    write!(f, " ({})", ctx)?;
                }
                if let (Some(vars), Some(constraints)) = (variable_count, constraint_count) {
                    write!(f, " [{} variables, {} constraints]", vars, constraints)?;
                }
                Ok(())
            },
            Self::Timeout { elapsed_seconds, operation } => {
                write!(f, "Solving timed out")?;
                if let Some(elapsed) = elapsed_seconds {
                    write!(f, " after {:.2}s", elapsed)?;
                }
                if let Some(op) = operation {
                    write!(f, " during {}", op)?;
                }
                Ok(())
            },
            Self::MemoryLimit { usage_mb, limit_mb } => {
                write!(f, "Memory limit exceeded")?;
                if let (Some(usage), Some(limit)) = (usage_mb, limit_mb) {
                    write!(f, " (used: {}MB, limit: {}MB)", usage, limit)?;
                }
                Ok(())
            },
            Self::InvalidConstraint { message, constraint_name, variables } => {
                write!(f, "Invalid constraint: {}", message)?;
                if let Some(name) = constraint_name {
                    write!(f, " (constraint: {})", name)?;
                }
                if let Some(vars) = variables {
                    if !vars.is_empty() {
                        write!(f, " [variables: {}]", vars.join(", "))?;
                    }
                }
                Ok(())
            },
            Self::ConflictingConstraints { constraint_names, variables, context } => {
                write!(f, "Conflicting constraints detected")?;
                if let Some(names) = constraint_names {
                    if !names.is_empty() {
                        write!(f, " ({})", names.join(" vs "))?;
                    }
                }
                if let Some(vars) = variables {
                    if !vars.is_empty() {
                        write!(f, " [variables: {}]", vars.join(", "))?;
                    }
                }
                if let Some(ctx) = context {
                    write!(f, " - {}", ctx)?;
                }
                Ok(())
            },
            Self::InvalidDomain { message, variable_name, domain_info } => {
                write!(f, "Invalid domain: {}", message)?;
                if let Some(var) = variable_name {
                    write!(f, " (variable: {})", var)?;
                }
                if let Some(domain) = domain_info {
                    write!(f, " [{}]", domain)?;
                }
                Ok(())
            },
            Self::InvalidVariable { message, variable_id, expected } => {
                write!(f, "Invalid variable: {}", message)?;
                if let Some(id) = variable_id {
                    write!(f, " (id: {})", id)?;
                }
                if let Some(exp) = expected {
                    write!(f, " [expected: {}]", exp)?;
                }
                Ok(())
            },
            Self::InternalError { message, location, debug_info } => {
                write!(f, "Internal error: {}", message)?;
                if let Some(loc) = location {
                    write!(f, " at {}", loc)?;
                }
                if let Some(debug) = debug_info {
                    write!(f, " [{}]", debug)?;
                }
                Ok(())
            },
            Self::InvalidInput { message, function_name, expected } => {
                write!(f, "Invalid input: {}", message)?;
                if let Some(func) = function_name {
                    write!(f, " in function '{}'", func)?;
                }
                if let Some(exp) = expected {
                    write!(f, " (expected: {})", exp)?;
                }
                Ok(())
            },
        }
    }
}

impl std::error::Error for SolverError {}

impl SolverError {
    /// Create a simple NoSolution error without context
    pub fn no_solution() -> Self {
        Self::NoSolution {
            context: None,
            variable_count: None,
            constraint_count: None,
        }
    }
    
    /// Create a NoSolution error with context
    pub fn no_solution_with_context(context: impl Into<String>, var_count: usize, constraint_count: usize) -> Self {
        Self::NoSolution {
            context: Some(context.into()),
            variable_count: Some(var_count),
            constraint_count: Some(constraint_count),
        }
    }
    
    /// Create a simple Timeout error
    pub fn timeout() -> Self {
        Self::Timeout {
            elapsed_seconds: None,
            operation: None,
        }
    }
    
    /// Create a Timeout error with context
    pub fn timeout_with_context(elapsed_seconds: f64, operation: impl Into<String>) -> Self {
        Self::Timeout {
            elapsed_seconds: Some(elapsed_seconds),
            operation: Some(operation.into()),
        }
    }
    
    /// Create a simple MemoryLimit error
    pub fn memory_limit() -> Self {
        Self::MemoryLimit {
            usage_mb: None,
            limit_mb: None,
        }
    }
    
    /// Create a MemoryLimit error with usage info
    pub fn memory_limit_with_context(usage_mb: usize, limit_mb: usize) -> Self {
        Self::MemoryLimit {
            usage_mb: Some(usage_mb),
            limit_mb: Some(limit_mb),
        }
    }
    
    /// Create an InvalidConstraint error with minimal context
    pub fn invalid_constraint(message: impl Into<String>) -> Self {
        Self::InvalidConstraint {
            message: message.into(),
            constraint_name: None,
            variables: None,
        }
    }
    
    /// Create an InvalidConstraint error with full context
    pub fn invalid_constraint_with_context(
        message: impl Into<String>,
        constraint_name: impl Into<String>,
        variables: Vec<String>
    ) -> Self {
        Self::InvalidConstraint {
            message: message.into(),
            constraint_name: Some(constraint_name.into()),
            variables: Some(variables),
        }
    }
    
    /// Create a ConflictingConstraints error with constraint names
    pub fn conflicting_constraints_with_names(constraint_names: Vec<String>) -> Self {
        Self::ConflictingConstraints {
            constraint_names: Some(constraint_names),
            variables: None,
            context: None,
        }
    }
    
    /// Create a ConflictingConstraints error with full context
    pub fn conflicting_constraints_with_context(
        constraint_names: Vec<String>,
        variables: Vec<String>,
        context: impl Into<String>
    ) -> Self {
        Self::ConflictingConstraints {
            constraint_names: Some(constraint_names),
            variables: Some(variables),
            context: Some(context.into()),
        }
    }
    
    /// Create an InvalidDomain error with minimal context
    pub fn invalid_domain(message: impl Into<String>) -> Self {
        Self::InvalidDomain {
            message: message.into(),
            variable_name: None,
            domain_info: None,
        }
    }
    
    /// Create an InvalidDomain error with full context
    pub fn invalid_domain_with_context(
        message: impl Into<String>,
        variable_name: impl Into<String>,
        domain_info: impl Into<String>
    ) -> Self {
        Self::InvalidDomain {
            message: message.into(),
            variable_name: Some(variable_name.into()),
            domain_info: Some(domain_info.into()),
        }
    }
    
    /// Create an InvalidVariable error with minimal context
    pub fn invalid_variable(message: impl Into<String>) -> Self {
        Self::InvalidVariable {
            message: message.into(),
            variable_id: None,
            expected: None,
        }
    }
    
    /// Create an InvalidVariable error with full context
    pub fn invalid_variable_with_context(
        message: impl Into<String>,
        variable_id: impl Into<String>,
        expected: impl Into<String>
    ) -> Self {
        Self::InvalidVariable {
            message: message.into(),
            variable_id: Some(variable_id.into()),
            expected: Some(expected.into()),
        }
    }
    
    /// Create an InternalError with location context (typically called with `file!()` and `line!()`)
    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::InternalError {
            message: message.into(),
            location: None,
            debug_info: None,
        }
    }
    
    /// Create an InternalError with full context
    pub fn internal_error_with_context(
        message: impl Into<String>,
        file: &str,
        line: u32,
        debug_info: impl Into<String>
    ) -> Self {
        Self::InternalError {
            message: message.into(),
            location: Some(format!("{}:{}", file, line)),
            debug_info: Some(debug_info.into()),
        }
    }
}

/// Convenience type alias for Results that can fail with SolverError
pub type SolverResult<T> = Result<T, SolverError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        // Basic error without context
        assert_eq!(
            SolverError::NoSolution { 
                context: None, 
                variable_count: None, 
                constraint_count: None 
            }.to_string(), 
            "No solution found"
        );
        
        // Error with full context
        assert_eq!(
            SolverError::NoSolution { 
                context: Some("constraints too restrictive".to_string()), 
                variable_count: Some(10), 
                constraint_count: Some(5) 
            }.to_string(), 
            "No solution found (constraints too restrictive) [10 variables, 5 constraints]"
        );
        
        // Timeout with context
        assert_eq!(
            SolverError::Timeout { 
                elapsed_seconds: Some(30.5), 
                operation: Some("optimization".to_string()) 
            }.to_string(), 
            "Solving timed out after 30.50s during optimization"
        );
        
        // Invalid constraint with full context
        assert_eq!(
            SolverError::InvalidConstraint { 
                message: "x > x".to_string(),
                constraint_name: Some("reflexive_constraint".to_string()),
                variables: Some(vec!["x".to_string()])
            }.to_string(),
            "Invalid constraint: x > x (constraint: reflexive_constraint) [variables: x]"
        );
    }

    #[test]
    fn test_error_equality() {
        let error1 = SolverError::NoSolution { 
            context: None, 
            variable_count: None, 
            constraint_count: None 
        };
        let error2 = SolverError::NoSolution { 
            context: None, 
            variable_count: None, 
            constraint_count: None 
        };
        assert_eq!(error1, error2);
        
        let error3 = SolverError::InvalidDomain { 
            message: "empty".to_string(),
            variable_name: None,
            domain_info: None
        };
        let error4 = SolverError::InvalidDomain { 
            message: "empty".to_string(),
            variable_name: None,
            domain_info: None
        };
        assert_eq!(error3, error4);
    }

    #[test]
    fn test_error_is_error_trait() {
        let err: Box<dyn std::error::Error> = Box::new(SolverError::NoSolution { 
            context: None, 
            variable_count: None, 
            constraint_count: None 
        });
        assert_eq!(err.to_string(), "No solution found");
    }

    #[test]
    fn test_error_context_rich_formatting() {
        // Test conflicting constraints with full context
        let error = SolverError::ConflictingConstraints {
            constraint_names: Some(vec!["x > 5".to_string(), "x < 3".to_string()]),
            variables: Some(vec!["x".to_string()]),
            context: Some("bounds incompatible".to_string())
        };
        assert_eq!(
            error.to_string(),
            "Conflicting constraints detected (x > 5 vs x < 3) [variables: x] - bounds incompatible"
        );
        
        // Test memory limit with usage info
        let error = SolverError::MemoryLimit {
            usage_mb: Some(2048),
            limit_mb: Some(1024)
        };
        assert_eq!(
            error.to_string(),
            "Memory limit exceeded (used: 2048MB, limit: 1024MB)"
        );
    }
}