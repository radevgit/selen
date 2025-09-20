//! Builder API module
//!
//! This module contains consolidated constraint building APIs organized by functionality.

// Re-export constraint builder functionality for backward compatibility
pub use crate::constraint_builder::*;
pub use crate::boolean_operators::*;
pub use crate::math_syntax::*;

// Modular organization
pub mod fluent;
pub mod mathematical;

// Re-export modular components
pub use fluent::*;
pub use mathematical::*;