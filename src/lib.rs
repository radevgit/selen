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
//! - Support for various constraint types: `+`, `-`, `*`, `/`, `==`, `!=`, `<`, `<=`, `>`, `>=`, all_different
//! - Type of variables: `float`, `int`, `mixed` (int and float)
//!
//! ## Basic Example
//!
//! ```rust
//! use cspsolver::prelude::*;
//!
//! // Create a new model
//! let mut model = Model::default();
//!
//! // Create a variable x in [1, 10]
//! let v = model.new_var_int(1, 10);
//!
//! // Add constraint: x > 2.5
//! model.greater_than(v, float(2.5));
//!
//! // Solve the problem minimizing x
//! let solution = model.minimize(v).unwrap();
//! if let Val::ValI(x) = solution[v] {
//!     assert_eq!(x, 3);
//! }
//! ```
//!
//! ## Variables with Predefined Values
//!
//! ```rust
//! use cspsolver::prelude::*;
//!
//! // Create a new model  
//! let mut model = Model::default();
//!
//! // Create variables with specific allowed values
//! let even_var = model.new_var_with_values(vec![2, 4, 6, 8]);
//! let odd_var = model.new_var_with_values(vec![1, 3, 5, 7]);
//!
//! // Add constraint: variables must be different
//! model.not_equals(even_var, odd_var);
//!
//! // Solve the problem
//! let solution = model.solve().unwrap();
//! if let (Val::ValI(even), Val::ValI(odd)) = (solution[even_var], solution[odd_var]) {
//!     assert!(even % 2 == 0);  // even number
//!     assert!(odd % 2 == 1);   // odd number  
//!     assert_ne!(even, odd);   // different values
//! }
//! ```

// pub mod constraints;
// pub mod domain;
// pub mod propagation;
// pub mod search;
// pub mod solver;
// pub mod variable;

pub mod model;
pub mod vars;
pub mod solution;

#[doc(hidden)]
pub mod utils;
#[doc(hidden)]
pub mod utils64;


#[doc(hidden)]
pub mod views;

#[doc(hidden)]
pub mod props;
#[doc(hidden)]
pub mod search;
#[doc(hidden)]
pub mod gac;
#[doc(hidden)]
pub mod domain;
pub mod prelude;


#[cfg(test)]
mod tests;
#[cfg(test)]
mod debug;



