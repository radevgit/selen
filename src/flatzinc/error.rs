//! Error types for FlatZinc parsing and integration

use std::fmt;

/// Result type for FlatZinc operations
pub type FlatZincResult<T> = Result<T, FlatZincError>;

/// Errors that can occur during FlatZinc parsing and mapping
#[derive(Debug, Clone)]
pub enum FlatZincError {
    /// I/O error reading file
    IoError(String),
    
    /// Lexical error during tokenization
    LexError {
        message: String,
        line: usize,
        column: usize,
    },
    
    /// Syntax error during parsing
    ParseError {
        message: String,
        line: usize,
        column: usize,
    },
    
    /// Semantic error during AST to Model mapping
    MapError {
        message: String,
        line: Option<usize>,
        column: Option<usize>,
    },
    
    /// Unsupported FlatZinc feature
    UnsupportedFeature {
        feature: String,
        line: Option<usize>,
        column: Option<usize>,
    },
}

impl fmt::Display for FlatZincError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlatZincError::IoError(msg) => write!(f, "I/O Error: {}", msg),
            FlatZincError::LexError { message, line, column } => {
                write!(f, "Lexical Error at line {}, column {}: {}", line, column, message)
            }
            FlatZincError::ParseError { message, line, column } => {
                write!(f, "Parse Error at line {}, column {}: {}", line, column, message)
            }
            FlatZincError::MapError { message, line, column } => {
                match (line, column) {
                    (Some(l), Some(c)) => write!(f, "Mapping Error at line {}, column {}: {}", l, c, message),
                    _ => write!(f, "Mapping Error: {}", message),
                }
            }
            FlatZincError::UnsupportedFeature { feature, line, column } => {
                match (line, column) {
                    (Some(l), Some(c)) => {
                        write!(f, "Unsupported Feature '{}' at line {}, column {}", feature, l, c)
                    }
                    _ => write!(f, "Unsupported Feature '{}'", feature),
                }
            }
        }
    }
}

impl std::error::Error for FlatZincError {}
