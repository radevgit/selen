//! Runtime API module
//!
//! This module contains consolidated runtime constraint APIs organized by functionality.

// Re-export everything from the original runtime_api for backward compatibility
pub use crate::runtime_api::*;

// Modular organization
pub mod dynamic;
pub mod extensions;

// Re-export modular components (commented out to fix unused warnings)
// pub use dynamic::*;
// pub use extensions::*;