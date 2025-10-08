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
//! ## Constraint Types
//!
//! - **Arithmetic**: `+`, `-`, `*`, `/`, `%`, `abs()`, `min()`, `max()`, `sum()`
//! - **Comparison**: `==`, `!=`, `<`, `<=`, `>`, `>=`
//! - **Boolean logic**: `and()`, `or()`, `not()` - supports array and variadic syntax
//! - **Global constraints**: `alldiff()`, `allequal()`, `element()`
//!
//! 
//! ## Post a mathematical constraint to the model
//! 
//! post() - Post single contrraint to the model. 
//! postall() -  Post multiple constraints to the model in a single call.
//! Accepts comma-separated constraint expressions, each following the same patterns as `post!`
//!
//! Supported constraint patterns:
//! **Basic comparisons**: `var op var`, `var op literal`, `var op (expr)`, `var op int(value)`, `var op float(value)`
//! **Arithmetic**: `var op var +/- var`, `var op var */÷ var`, `var op var % divisor`
//! **Functions**: `func(var) op target` where `func` is `abs`, `min`, `max`, `sum` 
//! **Boolean**: `and(vars...)`, `or(vars...)`, `not(var)` - supports arrays `and([a,b,c])` and variadic `and(a,b,c,d)`
//! **Global**: `alldiff([vars...])`, `allequal([vars...])`, `element(array, index, value)`
//! **Multiplication with constants**: `target op var * int(value)`, `target op var * float(value)`
//! 
//! Where `op` is any of: `==`, `!=`, `<`, `<=`, `>`, `>=`
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
//! use selen::prelude::*;
//!
//! let mut m = Model::default();
//! let items = m.int(1, 100);        // Number of items
//! let cost = m.float(0.0, 1000.0);  // Total cost
//!
//! // Use constraint API methods
//! m.float_lin_eq(&vec![1.0, -12.5], &vec![cost, items], 0.0);  // cost = items * 12.5
//! m.new(cost.le(500.0));                                        // Budget constraint
//!
//! // Maximize number of items within budget
//! if let Some(solution) = m.maximize(items) {
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
//! if let Some(solution) = m.solve() {
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
//! post!(m, alldiff(&vars));
//! 
//! // At least one flag must be true (using slice syntax)
//! post!(m, or([flags[0], flags[1], flags[2]]));
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



