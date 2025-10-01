//! Internal variable factory methods
//!
//! This module contains low-level variable creation methods that are used internally
//! by the solver and higher-level factory methods. These methods should NOT be used
//! directly by end users - use the methods in `factory.rs` instead.

use crate::model::core::Model;
use crate::variables::{Val, VarId, VarIdBin};
use crate::core::error::SolverError;

impl Model {
    // ========================================================================
    // INTERNAL LOW-LEVEL VARIABLE CREATION
    // ========================================================================

    /// Create a new decision variable with the provided domain bounds.
    ///
    /// Both lower and upper bounds are included in the domain.
    /// In case `max < min` the bounds will be swapped.
    /// 
    /// **Note**: This is a low-level internal method. Use `int()`, `float()`, or `bool()` instead.
    #[doc(hidden)]
    pub fn new_var(&mut self, min: Val, max: Val) -> VarId {
        if min < max {
            self.new_var_unchecked(min, max)
        } else {
            self.new_var_unchecked(max, min)
        }
    }

    /// Create new decision variables, with the provided domain bounds.
    ///
    /// All created variables will have the same starting domain bounds.
    /// Both lower and upper bounds are included in the domain.
    /// In case `max < min` the bounds will be swapped.
    /// 
    /// **Note**: This is a low-level internal method. Use specific variable creation methods instead.
    #[doc(hidden)]
    pub fn new_vars(&mut self, n: usize, min: Val, max: Val) -> impl Iterator<Item = VarId> + '_ {
        let (actual_min, actual_max) = if min < max { (min, max) } else { (max, min) };
        std::iter::repeat_with(move || self.new_var_unchecked(actual_min, actual_max)).take(n)
    }

    /// Create a new integer decision variable, with the provided domain bounds.
    ///
    /// Both lower and upper bounds are included in the domain.
    /// This function assumes that `min < max`.
    /// 
    /// **Note**: This is a low-level internal method.
    #[doc(hidden)]
    pub fn new_var_unchecked(&mut self, min: Val, max: Val) -> VarId {
        match self.new_var_checked(min, max) {
            Ok(var_id) => var_id,
            Err(_error) => {
                // Memory limit exceeded during variable creation - set flag via setter
                self.set_memory_limit_exceeded();
                
                // Return a dummy VarId to keep the API consistent
                // The solve() method will detect memory_limit_exceeded and return proper error
                VarId::from_index(0)
            }
        }
    }
    
    /// Create a new variable with memory limit checking
    /// 
    /// **Note**: This is a low-level internal method.
    pub(crate) fn new_var_checked(&mut self, min: Val, max: Val) -> Result<VarId, SolverError> {
        // Check if memory limit was already exceeded
        if self.memory_limit_exceeded() {
            return Err(SolverError::MemoryLimit {
                usage_mb: Some(self.estimated_memory_mb() as usize),
                limit_mb: self.config().max_memory_mb.map(|x| x as usize),
            });
        }
        
        // Estimate memory needed for this variable
        let estimated_memory = self.estimate_variable_memory(min, max);
        
        // Check if adding this variable would exceed the limit
        self.add_memory_usage(estimated_memory)?;
        
        // Create the variable
        self.props_mut().on_new_var();
        let step_size = self.float_step_size();
        let var_id = self.vars_mut().new_var_with_bounds_and_step(min, max, step_size);
        
        Ok(var_id)
    }

    // ========================================================================
    // INTERNAL MEMORY ESTIMATION HELPERS
    // ========================================================================

    /// Add to estimated memory usage and check limits
    /// 
    /// **Note**: This is an internal helper method.
    pub(crate) fn add_memory_usage(&mut self, bytes: u64) -> Result<(), SolverError> {
        self.add_estimated_memory(bytes);
        
        if let Some(limit_mb) = self.config().max_memory_mb {
            let limit_bytes = limit_mb * 1024 * 1024;
            if self.estimated_memory_bytes() > limit_bytes {
                self.set_memory_limit_exceeded();
                return Err(SolverError::MemoryLimit {
                    usage_mb: Some(self.estimated_memory_mb() as usize),
                    limit_mb: Some(limit_mb as usize),
                });
            }
        }
        
        Ok(())
    }

    /// Estimate memory usage for a variable with improved accuracy
    /// 
    /// **Note**: This is an internal helper method.
    pub(crate) fn estimate_variable_memory(&self, min: Val, max: Val) -> u64 {
        match (min, max) {
            (Val::ValI(min_i), Val::ValI(max_i)) => {
                // Integer variable: SparseSet structure overhead + domain representation
                if min_i > max_i {
                    // Invalid range - return minimal memory estimate
                    return 96; // Just base overhead
                }
                
                // Use checked arithmetic to prevent overflow with large domains
                let domain_size = match max_i.checked_sub(min_i) {
                    Some(diff) => match diff.checked_add(1) {
                        Some(size) => size as u64,
                        None => u64::MAX, // Overflow: treat as unbounded domain
                    },
                    None => u64::MAX, // Overflow: treat as unbounded domain
                };
                
                // Base SparseSet structure overhead (dense/sparse arrays, metadata)
                let base_cost = 96; // More realistic estimate including Vec overhead
                
                let domain_cost = if domain_size > 1000 {
                    // Large domains use sparse representation
                    // Two Vec<u32> with capacity approximately equal to domain size
                    let vec_overhead = 24 * 2; // Vec metadata for dense/sparse arrays
                    let data_cost = domain_size.saturating_mul(4).saturating_mul(2); // Prevent overflow
                    vec_overhead + data_cost / 8 // Amortized for typical sparsity
                } else {
                    // Small domains use dense representation
                    let vec_overhead = 24 * 2;
                    let data_cost = domain_size.saturating_mul(4).saturating_mul(2); // Prevent overflow
                    vec_overhead + data_cost
                };
                
                base_cost + domain_cost
            }
            (Val::ValF(_), Val::ValF(_)) => {
                // Float variable: FloatInterval structure
                // Contains: min (f64), max (f64), step (f64) = 24 bytes
                // Plus wrapper overhead and alignment
                let base_cost = 32; // FloatInterval struct
                let wrapper_cost = 32; // Var enum wrapper + alignment
                base_cost + wrapper_cost
            }
            _ => {
                // Mixed types: treated as float variable
                64
            }
        }
    }

    // ========================================================================
    // INTERNAL BATCH VARIABLE CREATION
    // ========================================================================

    /// Create new integer decision variables, with the provided domain bounds.
    ///
    /// Both lower and upper bounds are included in the domain.
    /// In case `max < min` the bounds will be swapped.
    /// 
    /// **Note**: This is an internal method. Use `ints()` for public API.
    #[doc(hidden)]
    pub fn int_vars(
        &mut self,
        n: usize,
        min: i32,
        max: i32,
    ) -> impl Iterator<Item = VarId> + '_ {
        self.new_vars(n, Val::ValI(min), Val::ValI(max))
    }

    /// Create new float decision variables, with the provided domain bounds.
    ///
    /// Both lower and upper bounds are included in the domain.
    /// In case `max < min` the bounds will be swapped.
    /// 
    /// **Note**: This is an internal method. Use `floats()` for public API.
    #[doc(hidden)]
    pub fn float_vars(
        &mut self,
        n: usize,
        min: f64,
        max: f64,
    ) -> impl Iterator<Item = VarId> + '_ {
        self.new_vars(n, Val::ValF(min), Val::ValF(max))
    }

    // ========================================================================
    // INTERNAL BINARY VARIABLE CREATION
    // ========================================================================

    /// Create a new binary decision variable.
    /// 
    /// **Note**: This is an internal method. Use `bool()` for public API.
    #[doc(hidden)]
    pub fn new_var_binary(&mut self) -> VarIdBin {
        VarIdBin(self.new_var_unchecked(Val::ValI(0), Val::ValI(1)))
    }

    /// Create new binary decision variables.
    /// 
    /// **Note**: This is an internal method. Use `bools()` for public API.
    #[doc(hidden)]
    pub fn new_vars_binary(&mut self, n: usize) -> impl Iterator<Item = VarIdBin> + '_ {
        std::iter::repeat_with(|| self.new_var_binary()).take(n)
    }

    /// Create a binary variable (0 or 1) returning VarIdBin.
    /// 
    /// Creates a boolean variable that can only take values 0 or 1.
    /// Returns VarIdBin for specialized binary constraints.
    /// 
    /// **Note**: This is an internal method. Use `bool()` for public API.
    #[doc(hidden)]
    pub fn binary(&mut self) -> VarIdBin {
        self.new_var_binary()
    }
}