//! Builder API module
//!
//! This module contains consolidated constraint building APIs organized by functionality.

// Re-export constraint builder functionality
pub use crate::constraints::boolean_operators::*;
pub use crate::constraints::math_syntax::*;

// Modular organization
pub mod fluent;
pub mod mathematical;

// Re-export modular components (commented out to fix unused warnings)
// pub use fluent::*;
// pub use mathematical::*;