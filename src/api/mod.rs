//! API module
//!
//! This module consolidates all public API functionality including:
//! - Prelude for common imports
//! - Builder patterns for fluent constraint creation
//! - Runtime API for dynamic constraint handling

// Re-export prelude functionality
pub use crate::prelude::*;

// API submodules
pub mod prelude;
pub mod builder;
pub mod runtime;