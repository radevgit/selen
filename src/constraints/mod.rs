//! Constraint system module
//!
//! This module contains the constraint system components including:
//! - Constraint API for posting constraints (organized by category)
//! - Constraint macros for mathematical syntax
//! - Propagators for constraint enforcement
//! - Constraint builders for fluent API

// Constraint API - organized user-facing methods
pub mod api;

// Core constraint modules
#[deprecated(since = "0.9.3", note = "Constraint macros are difficult to maintain and have limited capabilities. Use the constraint API methods directly (e.g., `model.add(x, y)` instead of `post!(model, x + y)`). This module will be removed in a future release.")]
pub mod macros;

// Modular propagator system (final modularization phase)
pub mod propagators;

// GAC modules
pub mod gac_hybrid;
pub mod gac_sparseset;
pub mod gac_bitset;

// Moved files
pub mod boolean_operators;
pub mod math_syntax;
pub mod operators;

// Re-export everything from the organized constraint macros
#[deprecated(since = "0.9.3", note = "Constraint macros are deprecated. Use the constraint API methods directly.")]
pub use macros::*;

// Re-export GAC modules (gac_hybrid contains all common types)
pub use gac_hybrid::*;
pub use gac_sparseset::*;

// Re-export moved files
pub use boolean_operators::*;
pub use math_syntax::*;
pub use operators::*;

// Props module  
pub mod props;
pub use props::*;