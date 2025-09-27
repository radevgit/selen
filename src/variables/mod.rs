//! Variable system module
//!
//! This module contains the restructured variable system components organized by functionality.

// Core variable files
mod vars;
pub mod views;

// Modular view system (final modularization phase)
pub mod view_system;

// Domain management
pub mod domain;

// Additional modules 
pub mod core;
pub mod operations;
pub mod values;

// Re-export everything for backward compatibility
pub use views::*;
pub use domain::*;
pub use core::*;