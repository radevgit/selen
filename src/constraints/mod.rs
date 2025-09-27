//! Constraint system module
//!
//! This module contains the constraint system components including:
//! - Constraint macros for mathematical syntax
//! - Propagators for constraint enforcement
//! - Constraint builders for fluent API

// Core constraint modules
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