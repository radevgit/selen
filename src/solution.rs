//! Solution representation and solving statistics.
//!
//! This module provides types for representing solutions to CSP problems and
//! collecting statistics about the solving process.
//!
//! # Solution Access
//!
//! Solutions are represented by the `Solution` struct, which allows indexed access
//! to variable values using the original `VarId` handles.
//!
//! # Statistics
//!
//! The solver collects statistics about the solving process, including the number
//! of propagations performed and search nodes explored.
//!
//! # Example
//!
//! ```rust
//! use cspsolver::prelude::*;
//!
//! let mut m = Model::default();
//! let x = m.int(1, 10);
//! let y = m.int(1, 10);
//! post!(m, x + y == int(15));
//!
//! // Solve with statistics callback
//! let solution = m.solve_with_callback(|stats| {
//!     println!("Propagations: {}", stats.propagation_count);
//!     println!("Search nodes: {}", stats.node_count);
//! }).unwrap();
//!
//! // Access solution values
//! println!("x = {:?}", solution[x]);
//! println!("y = {:?}", solution[y]);
//! ```

use std::borrow::Borrow;
use std::ops::Index;
use std::time::Duration;

use crate::vars::{Val, VarId, VarIdBin};

/// Statistics collected during the solving process.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SolveStats {
    /// Number of propagation steps performed during solving
    pub propagation_count: usize,
    /// Number of search nodes (branching points) explored during solving
    pub node_count: usize,
}

/// Enhanced statistics with detailed timing information for performance analysis.
#[derive(Clone, Debug, Default)]
pub struct EnhancedSolveStats {
    /// Number of propagation steps performed during solving
    pub propagation_count: usize,
    /// Number of search nodes (branching points) explored during solving
    pub node_count: usize,
    /// Total time spent in search/solving
    pub total_time: Duration,
    /// Time spent in constraint propagation
    pub propagation_time: Duration,
    /// Time spent in search/branching
    pub search_time: Duration,
    /// Time spent in variable domain operations
    pub domain_time: Duration,
    /// Time spent in constraint evaluation
    pub constraint_time: Duration,
    /// Number of backtracking operations
    pub backtrack_count: usize,
    /// Number of constraint checks performed
    pub constraint_checks: usize,
    /// Peak memory usage approximation (number of active search states)
    pub peak_search_depth: usize,
}

impl EnhancedSolveStats {
    /// Create a new enhanced stats tracker
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Get average time per propagation step
    pub fn avg_propagation_time(&self) -> Duration {
        if self.propagation_count > 0 {
            self.propagation_time / self.propagation_count as u32
        } else {
            Duration::ZERO
        }
    }
    
    /// Get average time per search node
    pub fn avg_search_time(&self) -> Duration {
        if self.node_count > 0 {
            self.search_time / self.node_count as u32
        } else {
            Duration::ZERO
        }
    }
    
    /// Convert to basic SolveStats for compatibility
    pub fn to_basic(&self) -> SolveStats {
        SolveStats {
            propagation_count: self.propagation_count,
            node_count: self.node_count,
        }
    }
    
    /// Display detailed performance analysis
    pub fn display_analysis(&self) {
        println!("=== Performance Analysis ===");
        println!("Total time: {:.3}ms", self.total_time.as_secs_f64() * 1000.0);
        println!("Propagation: {} steps, {:.3}ms total, {:.6}ms avg", 
                 self.propagation_count, 
                 self.propagation_time.as_secs_f64() * 1000.0,
                 self.avg_propagation_time().as_secs_f64() * 1000.0);
        println!("Search: {} nodes, {:.3}ms total, {:.6}ms avg", 
                 self.node_count,
                 self.search_time.as_secs_f64() * 1000.0,
                 self.avg_search_time().as_secs_f64() * 1000.0);
        println!("Domain ops: {:.3}ms", self.domain_time.as_secs_f64() * 1000.0);
        println!("Constraints: {:.3}ms ({} checks)", 
                 self.constraint_time.as_secs_f64() * 1000.0,
                 self.constraint_checks);
        println!("Backtracking: {} operations", self.backtrack_count);
        println!("Peak search depth: {}", self.peak_search_depth);
        
        if self.total_time.as_nanos() > 0 {
            let prop_pct = (self.propagation_time.as_nanos() * 100) / self.total_time.as_nanos();
            let search_pct = (self.search_time.as_nanos() * 100) / self.total_time.as_nanos();
            let domain_pct = (self.domain_time.as_nanos() * 100) / self.total_time.as_nanos();
            let constraint_pct = (self.constraint_time.as_nanos() * 100) / self.total_time.as_nanos();
            
            println!("Time breakdown: {}% propagation, {}% search, {}% domain, {}% constraints",
                     prop_pct, search_pct, domain_pct, constraint_pct);
        }
        println!("=============================");
    }
}

/// Assignment for decision variables that satisfies all constraints.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Solution(Vec<Val>);

impl Index<VarId> for Solution {
    type Output = Val;

    fn index(&self, index: VarId) -> &Self::Output {
        &self.0[index]
    }
}

impl Solution {
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
        self.0[v.borrow().0] == Val::ValI(1)
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
}


impl From<Vec<Val>> for Solution {
    fn from(value: Vec<Val>) -> Self {
        Self(value)
    }
}