//! Constraint system module
//!
//! This module contains the constraint system components including:
//! - Constraint macros for mathematical syntax
//! - Propagators for constraint enforcement
//! - Constraint builders for fluent API

// Core constraint modules
pub mod macros;
pub mod propagators;

// Moved files
pub mod boolean_operators;
pub mod math_syntax;
pub mod builder_legacy;
pub mod operators;
pub mod gac;

// Re-export everything from the organized constraint macros
pub use macros::*;

// Re-export moved files
pub use boolean_operators::*;
pub use math_syntax::*;
pub use builder_legacy::*;
pub use operators::*;
pub use gac::*;

// Props module  
pub mod props;
pub use props::*;