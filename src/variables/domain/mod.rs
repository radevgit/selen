//! Domain representation and manipulation for CSP variables
//!
//! This module provides efficient data structures for representing and manipulating
//! variable domains in constraint satisfaction problems.

#[doc(hidden)]
pub mod sparse_set;
#[doc(hidden)]
pub mod bitset_domain;
#[doc(hidden)]
pub mod float_interval;

#[doc(hidden)]
pub use sparse_set::SparseSet;
#[doc(hidden)]
pub use bitset_domain::BitSetDomain;
#[doc(hidden)]
pub use float_interval::FloatInterval;