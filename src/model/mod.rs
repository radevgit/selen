//! Model core module
//!
//! This module contains the decomposed Model functionality organized by purpose.

// Core model functionality (the main Model struct moved from model_core.rs)
mod core;

// Organized model functionality
pub mod factory;
pub mod constraints;
pub mod solving;
pub mod precision;

// Re-export everything for backward compatibility
pub use core::*;