//! Model core module
//!
//! This module contains the decomposed Model functionality organized by purpose.

// Re-export everything from the original model_core module for backward compatibility
pub use crate::model_core::*;

// Organized model functionality
pub mod factory;
pub mod constraints;
pub mod solving;
pub mod precision;

// Re-export organized modules (they contain only documentation and future placeholders for now)
pub use factory::*;
pub use constraints::*;
pub use solving::*;
pub use precision::*;