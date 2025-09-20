//! Model and VarId extensions for runtime API
//!
//! This module provides extension traits for Model and VarId.
//! Currently re-exports from existing runtime_api module for compatibility.

// Re-export extension traits
pub use crate::runtime_api::{ModelExt, VarIdExt, ConstraintVecExt};