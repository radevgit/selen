//! Modular constraint propagator system (Framework)
//!
//! This module demonstrates the final modularization phase structure.
//! Framework is ready for future migration of props/mod.rs functionality.

pub mod core_framework;
pub mod manager;

// Framework modules for demonstration - not exported to avoid conflicts\n// pub use core_framework::*;

// Framework available but not activated to maintain compatibility
// Re-export everything from the props submodule for backward compatibility
pub use crate::constraints::props::*;

// Organized propagator categories (currently organizational only)
pub mod arithmetic;
pub mod comparison;
pub mod logical;
pub mod global;
pub mod mathematical;