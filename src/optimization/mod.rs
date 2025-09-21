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

#[doc(hidden)]
pub mod classification;
#[doc(hidden)]
pub mod float_direct;
#[doc(hidden)]
pub mod constraint_integration;
#[doc(hidden)]
pub mod precision_handling;
#[doc(hidden)]
pub mod model_integration;
#[doc(hidden)]
pub mod constraint_metadata;
#[doc(hidden)]
pub mod precision_optimizer;
#[doc(hidden)]
pub mod variable_partitioning;
#[doc(hidden)]
pub mod subproblem_solving;
#[doc(hidden)]
pub mod solution_integration;
#[doc(hidden)]
pub mod precision_propagator;
#[doc(hidden)]
pub mod ulp_utils;

#[doc(hidden)]
pub use classification::*;
#[doc(hidden)]
pub use float_direct::*;
#[doc(hidden)]
pub use constraint_integration::*;
#[doc(hidden)]
pub use model_integration::*;
pub use precision_optimizer::*;
pub use precision_propagator::*;  // Re-enabled - dependencies exist
pub use ulp_utils::*;
pub use subproblem_solving::*;
pub use solution_integration::*;
