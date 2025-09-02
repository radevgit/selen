//! # CSP Solver
//!
//! A constraint satisfaction problem (CSP) solver library.
//!
//! This library provides tools for solving constraint satisfaction problems,
//! including constraint propagation and search algorithms.
//!
//! ## Features
//!
//! - Constraint propagation
//! - Backtracking search
//! - Domain filtering
//! - Support for various constraint types
//!
//! ## Example
//!
//! ```rust
//! use cspsolver::prelude::*;
//!
//! // Example usage will be added as the library develops
//! ```

// pub mod constraints;
// pub mod domain;
// pub mod propagation;
// pub mod search;
// pub mod solver;
// pub mod variable;

pub mod utils;
pub mod vars;
pub mod model;
pub mod views;
pub mod solution;
pub mod props;
pub mod search;
pub mod prelude;


#[cfg(test)]
mod tests;


