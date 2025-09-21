//! Variable system with modernized modular organization.
//!
//! This module re-exports variable types from the new modular structure for backward compatibility.
//! The actual implementations have been moved to focused modules:
//! - `variables::core`: Core types (VarId, Val, Var, Vars)
//! - `variables::views`: View system for constraint implementation 
//! - `variables::operations`: Arithmetic operations on variables
//! - `variables::domains`: Domain management utilities
//! - `variables::values`: Value-specific operations

// Re-export core types for backward compatibility
pub use crate::variables::core::{Val};
