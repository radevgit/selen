//! Variable system module
//!
//! This module will contain the restructured variable system components.
//! Currently re-exports from existing variable modules for compatibility.

// Re-export everything from existing variable modules
pub use crate::vars::*;
pub use crate::views::*;

// Future modular organization (will be populated in Phase 5)
// pub mod core;
// pub mod domains;
// pub mod views;
// pub mod operations;
// pub mod values;