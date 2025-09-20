//! Constraint macros module
//!
//! This module contains constraint macros organized by category.

// Re-export everything from the original constraint_macros for backward compatibility
pub use crate::constraint_macros::*;

// Organized macro categories
pub mod arithmetic;
pub mod comparison; 
pub mod logical;
pub mod global;

// Re-export organized modules
pub use arithmetic::*;
pub use comparison::*;
pub use logical::*;
pub use global::*;