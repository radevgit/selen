//! Model core module
//!
//! This module contains the decomposed Model functionality organized by purpose.

// Core model functionality (the main Model struct moved from model_core.rs)
mod core;

// Organized model functionality  
pub mod factory;        // Public variable factory API
mod factory_internal;   // Internal variable creation methods
pub mod constraints;
pub mod solving;
pub mod precision;
mod flatzinc_integration; // FlatZinc import methods

// Re-export everything for backward compatibility
pub use core::*;