//! Utility functions and configuration
//!
//! This module contains utility functions, configuration types, and helper code
//! used throughout the CSP solver.

pub mod utils;
pub mod utils64;
pub mod config;

// Re-export everything for convenience
pub use utils::*;
pub use utils64::*;
pub use config::*;