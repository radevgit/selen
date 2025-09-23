//! Solution representation and solving statistics.
//!
//! This module provides types for representing solutions to CSP problems and
//! collecting statistics about the solving process.
//!
//! # Solution Access
//!
//! Solutions are represented by the `Solution` struct, which allows indexed access
//! to variable values using the original `VarId` handles. Every solution includes
//! statistics about how it was found.
//!
//! # Statistics
//!
//! The solver collects comprehensive statistics about the solving process, including:
//! - **Search metrics**: propagation steps, search nodes, and backtracking operations
//! - **Performance data**: total solve time, time per operation, and memory usage
//! - **Problem characteristics**: number of variables and constraints
//! 
//!
//! # Example
//!
//! ```rust
//! use selen::prelude::*;
//!
//! let mut m = Model::default();
//! let x = m.int(1, 10);
//! let y = m.int(1, 10);
//! post!(m, x + y == int(15));
//!
//! // Solve and get solution with enhanced statistics
//! let solution = m.solve().unwrap();
//!
//! // Access solution values
//! println!("x = {:?}", solution[x]);
//! println!("y = {:?}", solution[y]);
//!
//! // Access all enhanced statistics fields
//! let stats = &solution.stats;
//! println!("Propagations: {}", stats.propagation_count);
//! println!("Search nodes: {}", stats.node_count);
//! println!("Solve time: {:.3}ms", stats.solve_time.as_secs_f64() * 1000.0);
//! println!("Peak memory usage: {}MB", stats.peak_memory_mb);
//! println!("Problem size: {} variables, {} constraints", 
//!          stats.variable_count, stats.constraint_count);
//!
//! // Use convenience analysis methods
//! println!("Search efficiency: {:.1} propagations/node", stats.efficiency());
//! println!("Time per propagation: {:.2}μs", 
//!          stats.time_per_propagation().as_nanos() as f64 / 1000.0);
//! println!("Time per search node: {:.2}μs", 
//!          stats.time_per_node().as_nanos() as f64 / 1000.0);
//!
//! // Display comprehensive summary
//! stats.display_summary();
//! ```
//!
//! # Runtime API Example
//!
//! ```rust
//! use selen::prelude::*;
//!
//! let mut m = Model::default();
//! let x = m.int(1, 10);
//! let y = m.int(1, 10);
//!
//! // Build constraints programmatically
//! m.new(x.add(y).eq(15));
//! m.new(x.mul(2).le(y));
//!
//! // Solve and access solution with comprehensive statistics
//! let solution = m.solve().unwrap();
//! println!("x = {:?}, y = {:?}", solution[x], solution[y]);
//!
//! // Access all enhanced statistics fields
//! let stats = &solution.stats;
//! println!("Core metrics:");
//! println!("  Propagations: {}", stats.propagation_count);
//! println!("  Search nodes: {}", stats.node_count);
//! 
//! println!("Performance metrics:");
//! println!("  Solve time: {:.3}ms", stats.solve_time.as_secs_f64() * 1000.0);
//! println!("  Peak memory: {}MB", stats.peak_memory_mb);
//! 
//! println!("Problem characteristics:");
//! println!("  Variables: {}", stats.variable_count);
//! println!("  Constraints: {}", stats.constraint_count);
//! 
//! // Use all convenience analysis methods
//! if stats.node_count > 0 {
//!     println!("Efficiency analysis:");
//!     println!("  {:.2} propagations/node", stats.efficiency());
//!     println!("  {:.2}μs/propagation", stats.time_per_propagation().as_nanos() as f64 / 1000.0);
//!     println!("  {:.2}μs/node", stats.time_per_node().as_nanos() as f64 / 1000.0);
//! }
//! 
//! // Display complete formatted summary
//! stats.display_summary();
//! 
//! // Create statistics manually using constructor
//! let custom_stats = SolveStats::new(100, 10, 
//!     std::time::Duration::from_millis(5), 20, 15, 8);
//! println!("Custom stats efficiency: {:.1}", custom_stats.efficiency());
//! ```

use std::borrow::Borrow;
use std::ops::Index;
use std::time::Duration;

use crate::variables::{Val, VarId, VarIdBin};

/// Statistics collected during the solving process.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SolveStats {
    /// Number of propagation steps performed during solving
    pub propagation_count: usize,
    /// Number of search nodes (branching points) explored during solving
    pub node_count: usize,
    /// Total time spent solving
    pub solve_time: Duration,
    /// Number of variables in the problem
    pub variable_count: usize,
    /// Number of constraints in the problem
    pub constraint_count: usize,
    /// Peak memory usage estimate during solving (in MB)
    pub peak_memory_mb: usize,
}

impl SolveStats {
    /// Create new statistics with all fields
    pub fn new(
        propagation_count: usize,
        node_count: usize,
        solve_time: Duration,
        variable_count: usize,
        constraint_count: usize,
        peak_memory_mb: usize,
    ) -> Self {
        Self {
            propagation_count,
            node_count,
            solve_time,
            variable_count,
            constraint_count,
            peak_memory_mb,
        }
    }

    /// Get solving efficiency as propagations per node
    pub fn efficiency(&self) -> f64 {
        if self.node_count > 0 {
            self.propagation_count as f64 / self.node_count as f64
        } else {
            0.0
        }
    }

    /// Get average time per propagation step
    pub fn time_per_propagation(&self) -> Duration {
        if self.propagation_count > 0 {
            self.solve_time / self.propagation_count as u32
        } else {
            Duration::ZERO
        }
    }

    /// Get average time per search node
    pub fn time_per_node(&self) -> Duration {
        if self.node_count > 0 {
            self.solve_time / self.node_count as u32
        } else {
            Duration::ZERO
        }
    }

    /// Display a summary of the solving statistics
    pub fn display_summary(&self) {
        println!("=== Solving Statistics ===");
        println!("Time: {:.3}ms", self.solve_time.as_secs_f64() * 1000.0);
        println!("Memory: {}MB peak usage", self.peak_memory_mb);
        println!("Problem: {} variables, {} constraints", self.variable_count, self.constraint_count);
        println!("Search: {} propagations, {} nodes", 
                 self.propagation_count, self.node_count);
        
        if self.node_count > 0 {
            println!("Efficiency: {:.1} propagations/node", self.efficiency());
        } else {
            println!("Efficiency: Pure propagation (no search required)");
        }
        
        if self.propagation_count > 0 {
            println!("Performance: {:.2}μs/propagation", 
                     self.time_per_propagation().as_nanos() as f64 / 1000.0);
        }
        println!("==========================");
    }
}

/// Assignment for decision variables that satisfies all constraints.
#[derive(Debug, PartialEq)]
pub struct Solution {
    values: Vec<Val>,
    /// Statistics collected during the solving process
    pub stats: SolveStats,
}

impl Index<VarId> for Solution {
    type Output = Val;

    fn index(&self, index: VarId) -> &Self::Output {
        &self.values[index]
    }
}

impl Solution {
    /// Create a new solution with values and statistics
    pub fn new(values: Vec<Val>, stats: SolveStats) -> Self {
        Self { values, stats }
    }

    /// Create a solution from values with default (empty) statistics
    pub fn from_values(values: Vec<Val>) -> Self {
        Self {
            values,
            stats: SolveStats::default(),
        }
    }

    /// Get a reference to the solving statistics
    pub fn stats(&self) -> &SolveStats {
        &self.stats
    }

    /// Get assignments for the decision variables provided as a slice.
    #[must_use]
    pub fn get_values(&self, vs: &[VarId]) -> Vec<Val> {
        self.get_values_iter(vs.iter().copied()).collect()
    }

    /// Get assignments for the decision variables provided as a reference to an array.
    #[must_use]
    pub fn get_values_array<const N: usize>(&self, vs: &[VarId; N]) -> [Val; N] {
        vs.map(|v| self[v])
    }

    /// Get assignments for the provided decision variables.
    pub fn get_values_iter<'a, I>(&'a self, vs: I) -> impl Iterator<Item = Val> + 'a
    where
        I: IntoIterator + 'a,
        I::Item: Borrow<VarId>,
    {
        vs.into_iter().map(|v| self[*v.borrow()])
    }

    /// Get binary assignment for the provided decision variable.
    #[must_use]
    pub fn get_value_binary(&self, v: impl Borrow<VarIdBin>) -> bool {
        self.values[v.borrow().0] == Val::ValI(1)
    }

    /// Get binary assignments for the decision variables provided as a slice.
    #[must_use]
    pub fn get_values_binary(&self, vs: &[VarIdBin]) -> Vec<bool> {
        self.get_values_binary_iter(vs.iter().copied()).collect()
    }

    /// Get binary assignments for the decision variables provided as a reference to an array.
    #[must_use]
    pub fn get_values_binary_array<const N: usize>(&self, vs: &[VarIdBin; N]) -> [bool; N] {
        vs.map(|v| self.get_value_binary(v))
    }

    /// Get binary assignments for the provided decision variables.
    pub fn get_values_binary_iter<'a, I>(&'a self, vs: I) -> impl Iterator<Item = bool> + 'a
    where
        I: IntoIterator + 'a,
        I::Item: Borrow<VarIdBin>,
    {
        vs.into_iter().map(|v| self.get_value_binary(v))
    }
    
    /// Get the integer value for a variable (convenience method)
    /// Returns the integer value if the variable contains an integer, panics otherwise
    #[must_use]
    pub fn get_int(&self, var: VarId) -> i32 {
        match self[var] {
            Val::ValI(i) => i,
            Val::ValF(_) => panic!("Variable {:?} contains a float value, not an integer", var),
        }
    }
    
    /// Get the float value for a variable (convenience method)
    /// Returns the float value if the variable contains a float, panics otherwise
    #[must_use] 
    pub fn get_float(&self, var: VarId) -> f64 {
        match self[var] {
            Val::ValF(f) => f,
            Val::ValI(_) => panic!("Variable {:?} contains an integer value, not a float", var),
        }
    }
    
    /// Get the value for a variable as an integer if possible
    /// Returns Some(i32) if the value is an integer, None otherwise
    #[must_use]
    pub fn try_get_int(&self, var: VarId) -> Option<i32> {
        match self[var] {
            Val::ValI(i) => Some(i),
            Val::ValF(_) => None,
        }
    }
    
    /// Get the value for a variable as a float if possible
    /// Returns Some(f64) if the value is a float, None otherwise
    #[must_use]
    pub fn try_get_float(&self, var: VarId) -> Option<f64> {
        match self[var] {
            Val::ValF(f) => Some(f),
            Val::ValI(_) => None,
        }
    }
    
    /// Generic get method using type inference
    /// This allows `let x: i32 = solution.get(var);` syntax
    pub fn get<T>(&self, var: VarId) -> T 
    where 
        Self: GetValue<T>
    {
        self.get_value(var)
    }
}

/// Trait for type-safe value extraction
pub trait GetValue<T> {
    fn get_value(&self, var: VarId) -> T;
}

impl GetValue<i32> for Solution {
    fn get_value(&self, var: VarId) -> i32 {
        self.get_int(var)
    }
}

impl GetValue<f64> for Solution {
    fn get_value(&self, var: VarId) -> f64 {
        self.get_float(var)
    }
}

impl GetValue<Option<i32>> for Solution {
    fn get_value(&self, var: VarId) -> Option<i32> {
        self.try_get_int(var)
    }
}

impl GetValue<Option<f64>> for Solution {
    fn get_value(&self, var: VarId) -> Option<f64> {
        self.try_get_float(var)
    }
}


impl From<Vec<Val>> for Solution {
    fn from(values: Vec<Val>) -> Self {
        Self::from_values(values)
    }
}