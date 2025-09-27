//! Builder API module
//!
//! This module contains consolidated constraint building APIs organized by functionality.

// Re-export constraint builder functionality
pub use crate::constraints::boolean_operators::*;
pub use crate::constraints::math_syntax::*;

// Modular organization
pub mod mathematical;
