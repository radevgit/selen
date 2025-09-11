//! Efficient optimization algorithms for different problem types
//!
//! This module provides specialized solving algorithms based on automatic problem
//! classification. Instead of using a one-size-fits-all approach, problems are
//! analyzed and routed to the most appropriate algorithm:
//!
//! - **Pure float problems**: Direct bounds optimization (O(1) solutions)
//! - **Pure integer problems**: Existing binary search (already efficient)  
//! - **Mixed problems**: MINLP techniques with problem decomposition
//!
//! The classification happens automatically when solve() or maximize() is called,
//! ensuring optimal performance without requiring user intervention.

pub mod classification;
pub mod float_direct;
pub mod constraint_integration;
pub mod precision_handling;
pub mod model_integration;
pub mod constraint_metadata;

pub use classification::*;
pub use float_direct::*;
pub use constraint_integration::*;
pub use model_integration::*;
