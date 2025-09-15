//! # CSP Solver
//!
//! A constraint satisfaction problem (CSP) solver library.
//!
//! ## Variable Types
//!
//! - **Integer variables**: `m.int(min, max)` - continuous range
//! - **Float variables**: `m.float(min, max)` - continuous range with precision control
//! - **Custom domains**: `m.ints(vec![values])` - specific integer values only
//! - **Boolean variables**: `m.bool()` - equivalent to `m.int(0, 1)`
//!
//! ## Constraint Types
//!
//! - **Arithmetic**: `+`, `-`, `*`, `/`, `%`, `abs()`, `min()`, `max()`, `sum()`
//! - **Comparison**: `==`, `!=`, `<`, `<=`, `>`, `>=`
//! - **Boolean logic**: `and()`, `or()`, `not()`
//! - **Global constraints**: `alldiff()`
//!
//! 
//! ## post() - Post a mathematical constraint to the model
//!
//! Supported constraint patterns:
//!
//! **Basic comparisons**: `var op var`, `var op literal`, `var op (expr)`, `var op int(value)`, `var op float(value)`
//!
//! **Arithmetic**: `var op var +/- var`, `var op var */÷ var`, `var op var % divisor`
//! 
//! **Functions**: `func(var) op target` where `func` is `abs`, `min`, `max`, `sum`
//! 
//! **Boolean**: `and(vars...)`, `or(vars...)`, `not(var)`
//! 
//! **Global**: `alldiff([vars...])`
//! 
//! **Multiplication with constants**: `target op var * int(value)`, `target op var * float(value)`
//! 
//! Where `op` is any of: `==`, `!=`, `<`, `<=`, `>`, `>=`
//! 
//! 
//! 
//! ## postall() - Post multiple constraints to the model in a single call
//!
//! Accepts comma-separated constraint expressions, each following the same patterns as `post!`:
//! 
//! **Basic comparisons**: `var op var`, `var op literal`, `var op (expr)`, `var op int(value)`, `var op float(value)`
//! 
//! **Arithmetic**: `var op var +/- var`, `var op var */÷ var`, `var op var % divisor`
//! 
//! **Functions**: `func(var) op target` where `func` is `abs`, `min`, `max`, `sum`
//! 
//! **Boolean**: `and(vars...)`, `or(vars...)`, `not(var)`
//! 
//! **Global**: `alldiff([vars...])`
//! 
//! **Multiplication with constants**: `target op var * int(value)`, `target op var * float(value)`
//! 
//! Where `op` is any of: `==`, `!=`, `<`, `<=`, `>`, `>=`
//! 
//! 
//!
//! ## Example 1: Basic Integer Problem
//!
//! ```rust
//! use cspsolver::prelude::*;
//!
//! let mut m = Model::default();
//! let x = m.int(1, 10);
//! let y = m.int(1, 10);
//!
//! post!(m, x + y == int(12));
//! post!(m, x > y);
//!
//! if let Ok(solution) = m.solve() {
//!     println!("x = {:?}, y = {:?}", solution[x], solution[y]);
//! }
//! ```
//!
//! ## Example 2: Mixed Integer-Float Optimization
//!
//! ```rust
//! use cspsolver::prelude::*;
//!
//! let mut m = Model::default();
//! let items = m.int(1, 100);        // Number of items
//! let cost = m.float(0.0, 1000.0);  // Total cost
//!
//! post!(m, cost == items * float(12.5));  // $12.50 per item
//! post!(m, cost <= float(500.0));         // Budget constraint
//!
//! // Maximize number of items within budget
//! if let Ok(solution) = m.maximize(items) {
//!     println!("Optimal: {:?} items, cost: {:?}", 
//!              solution[items], solution[cost]);
//! }
//! ```
//!
//! ## Example 3: Custom Domains and Global Constraints
//!
//! ```rust
//! use cspsolver::prelude::*;
//!
//! let mut m = Model::default();
//! 
//! // Variables with custom domains
//! let red = m.ints(vec![1, 3, 5, 7]);      // Odd numbers
//! let blue = m.ints(vec![2, 4, 6, 8]);     // Even numbers  
//! let green = m.ints(vec![2, 3, 5, 7]);    // Prime numbers
//!
//! // All must be different
//! post!(m, alldiff([red, blue, green]));
//!
//! if let Ok(solution) = m.solve() {
//!     println!("Red: {:?}, Blue: {:?}, Green: {:?}",
//!              solution[red], solution[blue], solution[green]);
//! }
//! ```


pub mod model;
pub mod vars;
pub mod solution;
pub mod config;
pub mod error;
pub mod validation;
#[doc(hidden)]
pub mod operators;

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
#[doc(hidden)]
pub mod optimization;
pub mod prelude;

#[doc(hidden)]
// Clean constraint API modules
pub mod constraint_builder;
pub mod boolean_operators;
#[doc(hidden)]
pub mod math_syntax;

#[doc(hidden)]
pub mod constraint_macros;


#[cfg(test)]
mod tests;
#[cfg(test)]
mod debug;

// Benchmarks module for performance validation
#[doc(hidden)]
pub mod benchmarks;



