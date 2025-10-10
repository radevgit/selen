//! # CSP Solver
//!
//! A constraint satisfaction problem (CSP) solver library in Rust.
//!
//! ## Safety & Resource Management
//!
//! All models have **automatic memory and timeout limits** to prevent system exhaustion:
//! - **Default memory limit**: 2GB
//! - **Default timeout**: 60000 milliseconds (60 seconds)
//! - **Memory tracking**: During variable creation (prevents crashes)
//! - **Early failure**: Clear error messages when limits exceeded
//!
//! ```rust
//! use selen::prelude::*;
//!
//! // Automatic safety limits
//! let mut m = Model::default(); // 2GB memory, 60000ms (60s) timeout
//!
//! // Custom limits
//! let config = SolverConfig::default()
//!     .with_max_memory_mb(512)      // 512MB limit
//!     .with_timeout_ms(30000);      // 30000ms = 30 second timeout
//! let mut m = Model::with_config(config);
//! ```
//!
//! ## Variable Types
//!
//! - **Integer variables**: `m.int(min, max)` - continuous range
//! - **Float variables**: `m.float(min, max)` - continuous range with precision control
//! - **Custom domains**: `m.intset(vec![values])` - specific integer values only
//! - **Boolean variables**: `m.bool()` - equivalent to `m.int(0, 1)`
//!
//! ## Bulk Variable Creation
//!
//! - **Multiple integers**: `m.ints(n, min, max)` - create n integer variables with same bounds
//! - **Multiple floats**: `m.floats(n, min, max)` - create n float variables with same bounds  
//! - **Multiple booleans**: `m.bools(n)` - create n boolean variables
//!
//! ## Constraint API
//!
//! ```rust
//! use selen::prelude::*;
//! # fn main() {
//! let mut m = Model::default();
//! let (x, y, z) = (m.int(0, 10), m.int(0, 10), m.int(0, 10));
//!
//! // Comparison constraints (via runtime API)
//! m.new(x.lt(y));                        // x < y
//! m.new(y.le(z));                        // y <= z
//! m.new(z.gt(5));                        // z > 5
//! m.new(x.eq(10));                       // x == 10
//! m.new(x.ne(y));                        // x != y
//!
//! // Arithmetic operations (return new variables)
//! let sum = m.add(x, y);                 // sum = x + y
//! let diff = m.sub(x, y);                // diff = x - y
//! let product = m.mul(x, y);             // product = x * y
//! let quotient = m.div(x, y);            // quotient = x / y
//! let absolute = m.abs(x);               // absolute = |x|
//!
//! // Aggregate operations
//! let minimum = m.min(&[x, y, z]).unwrap();  // minimum of variables
//! let maximum = m.max(&[x, y, z]).unwrap();  // maximum of variables
//! let total = m.sum(&[x, y, z]);             // sum of variables
//!
//! // Global constraints
//! m.alldiff(&[x, y, z]);                 // all variables different
//! m.alleq(&[x, y, z]);                   // all variables equal
//!
//! // Boolean operations (return boolean variables)
//! let (a, b) = (m.bool(), m.bool());
//! let and_result = m.bool_and(&[a, b]);  // a AND b
//! let or_result = m.bool_or(&[a, b]);    // a OR b
//! let not_result = m.bool_not(a);        // NOT a
//!
//! // Fluent expression building
//! m.new(x.add(y).le(z));                 // x + y <= z
//! m.new(y.sub(x).ge(0));                 // y - x >= 0
//!
//! // Linear constraints (weighted sums) - generic for int and float
//! m.lin_eq(&[2, 3], &[x, y], 10);        // 2x + 3y == 10
//! m.lin_le(&[1, -1], &[x, y], 5);        // x - y <= 5
//! m.lin_ne(&[2, 1], &[x, y], 8);         // 2x + y != 8
//!
//! // Reified constraints (with boolean result) - generic for int and float
//! let b = m.bool();
//! m.eq_reif(x, y, b);                    // b ↔ (x == y)
//! m.ne_reif(x, y, b);                    // b ↔ (x != y)
//! m.lt_reif(x, y, b);                    // b ↔ (x < y)
//! m.le_reif(x, y, b);                    // b ↔ (x <= y)
//! m.gt_reif(x, y, b);                    // b ↔ (x > y)
//! m.ge_reif(x, y, b);                    // b ↔ (x >= y)
//! # }
//! ```
//!
//! 
//!
//! ## Example 1: Basic Integer Problem
//!
//! ```rust
//! use selen::prelude::*;
//!
//! let mut m = Model::default();
//! let x = m.int(1, 10);
//! let y = m.int(1, 10);
//!
//! let sum = m.add(x, y);
//! m.new(sum.eq(12));
//! m.new(x.gt(y));
//!
//! if let Ok(solution) = m.solve() {
//!     println!("x = {:?}, y = {:?}", solution[x], solution[y]);
//! }
//! ```
//!
//! ## Example 2: Mixed Integer-Float Optimization
//!
//! ```rust
//! use selen::prelude::*;
//!
//! let mut m = Model::default();
//! let items = m.int(1, 100);        // Number of items
//! let cost = m.float(0.0, 1000.0);  // Total cost
//!
//! // Use constraint API methods
//! m.lin_eq(&vec![1.0, -12.5], &vec![cost, items], 0.0);  // cost = items * 12.5
//! m.new(cost.le(500.0));                                  // Budget constraint
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
//! use selen::prelude::*;
//!
//! let mut m = Model::default();
//! 
//! // Variables with custom domains
//! let red = m.intset(vec![1, 3, 5, 7]);      // Odd numbers
//! let blue = m.intset(vec![2, 4, 6, 8]);     // Even numbers  
//! let green = m.intset(vec![2, 3, 5, 7]);    // Prime numbers
//!
//! // All must be different using constraint API
//! m.alldiff(&[red, blue, green]);
//!
//! if let Ok(solution) = m.solve() {
//!     println!("Red: {:?}, Blue: {:?}, Green: {:?}",
//!              solution[red], solution[blue], solution[green]);
//! }
//! ```
//!
//! ## Example 4: Bulk Variable Creation
//!
//! Create multiple variables efficiently with the same domain:
//!
//! ```rust
//! use selen::prelude::*;
//!
//! let mut m = Model::default();
//! 
//! // Create 5 integer variables, each with domain [1, 10]
//! let vars = m.ints(5, 1, 10);
//! 
//! // Create 3 boolean variables  
//! let flags = m.bools(3);
//! 
//! // Create 4 float variables with same bounds
//! let weights = m.floats(4, 0.0, 1.0);
//!
//! // All variables in vars must be different
//! m.alldiff(&vars);
//! 
//! // First flag must be true
//! m.new(flags[0].eq(1));
//!
//! if let Ok(solution) = m.solve() {
//!     println!("Solution found with {} variables!", 
//!              vars.len() + flags.len() + weights.len());
//! }
//! ```
//!
//! ## Example 5: Programmatic API - Basic Constraints
//!
//! For developers who prefer explicit, method-based constraint building:
//!
//! ```rust
//! use selen::prelude::*;
//!
//! let mut m = Model::default();
//! let x = m.int(1, 10);
//! let y = m.int(1, 10);
//!
//! // Build constraints programmatically
//! m.new(x.add(y).eq(12));        // x + y == 12
//! m.new(x.gt(y));                // x > y
//! m.new(x.mul(2).le(15));        // x * 2 <= 15
//!
//! if let Ok(solution) = m.solve() {
//!     println!("x = {:?}, y = {:?}", solution[x], solution[y]);
//! }
//! ```
//!
//! ## Example 6: Programmatic API - Global Constraints
//!
//! ```rust
//! use selen::prelude::*;
//!
//! let mut m = Model::default();
//! let vars = vec![m.int(1, 5), m.int(1, 5), m.int(1, 5)];
//!
//! // Global constraints using programmatic API
//! m.alldiff(&vars);               // All variables must be different
//!
//! // Mathematical functions
//! let sum_result = m.sum(&vars);
//! m.new(sum_result.le(10));      // sum(vars) <= 10
//!
//! let max_result = m.max(&vars).expect("non-empty variable list");
//! m.new(max_result.ge(3));       // max(vars) >= 3
//!
//! if let Ok(solution) = m.solve() {
//!     println!("Variables: {:?}", vars.iter().map(|&v| solution[v]).collect::<Vec<_>>());
//! }
//! ```
//!
//! ## Example 7: Programmatic API - Complex Operations
//!
//! ```rust
//! use selen::prelude::*;
//!
//! let mut m = Model::default();
//! let x = m.int(-10, 10);
//! let y = m.int(1, 10);
//! let z = m.int(1, 20);
//!
//! // Arithmetic and mathematical functions
//! let abs_x = m.abs(x);
//! m.new(abs_x.ge(5));            // abs(x) >= 5
//!
//! // Modulo operations
//! let mod_result = m.modulo(z, Val::from(3));
//! m.new(mod_result.eq(1));       // z % 3 == 1
//!
//! // Logical operations on constraints
//! let constraint1 = x.gt(0);
//! let constraint2 = y.lt(5);
//! m.new(constraint1.and(constraint2));  // (x > 0) && (y < 5)
//!
//! if let Ok(solution) = m.solve() {
//!     println!("x = {:?}, y = {:?}, z = {:?}", solution[x], solution[y], solution[z]);
//! }
//! ```


// Core functionality
pub mod core;
#[doc(hidden)]
pub mod utils;

// Domain-specific modules  
pub mod model;
pub mod solvers;
#[doc(hidden)]
pub mod variables;
#[doc(hidden)]
pub mod constraints;
#[doc(hidden)]
pub mod search;
#[doc(hidden)]
pub mod optimization;

// LP solver (internal use for optimization)
#[doc(hidden)]
pub mod lpsolver;

// API and convenience modules
pub mod api;
pub mod prelude;
pub mod runtime_api;

// Development and testing modules
#[doc(hidden)]
pub mod benchmarks;

#[doc(hidden)]
#[cfg(test)]
mod debug;



