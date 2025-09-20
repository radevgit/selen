//! Constraint system module
//!
//! This module contains the constraint system components including:
//! - Constraint macros for mathematical syntax
//! - Propagators for constraint enforcement
//! - Constraint builders for fluent API

// Re-export everything from the original constraint_macros for backward compatibility
pub use crate::constraint_macros::*;

// Re-export everything from props for backward compatibility  
pub use crate::props::*;

// Re-export constraint builder functionality
pub use crate::constraint_builder::*;
pub use crate::boolean_operators::*;
pub use crate::math_syntax::*;

// Future modular organization (will be populated in later phases)
pub mod macros;
pub mod propagators;

// Constraint builder module
mod builder;
pub use builder::*;