//! Domain representation and manipulation for CSP variables
//!
//! This module provides efficient data structures for representing and manipulating
//! variable domains in constraint satisfaction problems.

#[doc(hidden)]
pub mod sparse_set;
#[doc(hidden)]
pub use sparse_set::SparseSet;