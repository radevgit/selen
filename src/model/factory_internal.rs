//! Internal variable factory methods
//!
//! This module contains low-level variable creation methods that are used internally
//! by the solver and higher-level factory methods. These methods should NOT be used
//! directly by end users - use the methods in `factory.rs` instead.

use crate::model::core::Model;
use crate::variables::{Val, VarId, VarIdBin};
use crate::core::error::SolverError;

// ============================================================================
// FALLBACK BOUNDS FOR UNBOUNDED VARIABLES
// ============================================================================

/// Default fallback bounds for unbounded integer variables (when no context exists)
/// 
/// **Value**: `(-100_000, 100_000)` → domain size of 200,001 elements
/// 
/// **Rationale**:
/// - Must respect `MAX_SPARSE_SET_DOMAIN_SIZE` (1 million elements) strictly
/// - Large enough for most CSP and optimization problems
/// - Domain size ~200K is well within 1M limit, leaves room for propagation
/// - Small enough to ensure efficient sparse set operations
const DEFAULT_INT_FALLBACK_MIN: i32 = -100_000;
const DEFAULT_INT_FALLBACK_MAX: i32 = 100_000;

/// Default fallback bounds for unbounded float variables (when no context exists)
/// 
/// **Value**: `(-1e9, 1e9)` → ±1 billion
/// 
/// **Rationale**:
/// - Large enough for real-world optimization problems:
///   - Financial modeling (millions/billions in currency)
///   - Engineering (stress, forces, dimensions)
///   - Operations research (costs, profits, resources)
/// - Small enough to maintain numerical stability
/// - Step size is automatically adapted to maintain reasonable domain size
const DEFAULT_FLOAT_FALLBACK_MIN: f64 = -1e9;
const DEFAULT_FLOAT_FALLBACK_MAX: f64 = 1e9;

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
        
        // Infer bounds if variable is unbounded
        let (inferred_min, inferred_max) = self.infer_bounds(min, max);
        
        // Estimate memory needed for this variable
        let estimated_memory = self.estimate_variable_memory(inferred_min, inferred_max);
        
        // Check if adding this variable would exceed the limit
        self.add_memory_usage(estimated_memory)?;
        
        // Create the variable
        self.props_mut().on_new_var();
        
        // Use standard step size
        let step_size = self.float_step_size();
        
        let var_id = self.vars_mut().new_var_with_bounds_and_step(inferred_min, inferred_max, step_size);
        
        Ok(var_id)
    }

    // ========================================================================
    // INTERNAL BOUND INFERENCE
    // ========================================================================

    /// Infer reasonable bounds for unbounded variables
    /// 
    /// This method detects unbounded variables (i32::MIN/MAX or f64::INFINITY) and infers
    /// reasonable finite bounds based on existing bounded variables in the model.
    /// 
    /// **Algorithm**:
    /// 1. Check if variable is unbounded
    /// 2. If bounded variables of same type exist: expand their range by 1000x
    /// 3. Otherwise: use fallback bounds (integers: ±100,000; floats: ±1e9)
    /// 4. Apply type-specific constraints (i32 range, domain size limits)
    /// 
    /// **Note**: This is an internal helper method.
    fn infer_bounds(&self, min: Val, max: Val) -> (Val, Val) {
        match (min, max) {
            (Val::ValI(min_i), Val::ValI(max_i)) => {
                // Check if integer variable is unbounded
                let is_unbounded = min_i == i32::MIN || max_i == i32::MAX;
                
                if !is_unbounded {
                    return (min, max); // Already bounded
                }
                
                // Scan existing integer variables for context
                let mut global_min: Option<i32> = None;
                let mut global_max: Option<i32> = None;
                
                for var_idx in 0..self.vars.count() {
                    let var_id = crate::variables::VarId::from_index(var_idx);
                    match &self.vars[var_id] {
                        crate::variables::Var::VarI(sparse_set) => {
                            if !sparse_set.is_empty() {
                                let v_min = sparse_set.min();
                                let v_max = sparse_set.max();
                                
                                // Skip if this variable itself looks unbounded
                                if v_min != i32::MIN && v_max != i32::MAX {
                                    global_min = Some(global_min.map_or(v_min, |gmin| gmin.min(v_min)));
                                    global_max = Some(global_max.map_or(v_max, |gmax| gmax.max(v_max)));
                                }
                            }
                        }
                        _ => {} // Skip float variables
                    }
                }
                
                let (inferred_min, inferred_max) = if let (Some(gmin), Some(gmax)) = (global_min, global_max) {
                    // We have context - expand by configured factor (default 1000x)
                    let factor = self.config().unbounded_inference_factor as i64;
                    let span = (gmax as i64) - (gmin as i64);
                    let expansion = span.saturating_mul(factor);
                    
                    let new_min = ((gmin as i64).saturating_sub(expansion)).max(i32::MIN as i64 + 1) as i32;
                    let new_max = ((gmax as i64).saturating_add(expansion)).min(i32::MAX as i64 - 1) as i32;
                    
                    // Check domain size - but allow tight inferred bounds
                    // Rationale: If inference is based on existing variables, trust it
                    // Only enforce limit for fallback case
                    let domain_size = (new_max as i64) - (new_min as i64) + 1;
                    if domain_size > crate::variables::domain::MAX_SPARSE_SET_DOMAIN_SIZE as i64 {
                        // Domain too large, but this is context-based inference
                        // Clamp to a reasonable range around the context
                        // Use ±500K from the context center (adjusted for inclusive bounds)
                        let center = ((gmin as i64 + gmax as i64) / 2) as i32;
                        let half_limit = (crate::variables::domain::MAX_SPARSE_SET_DOMAIN_SIZE / 2) as i32;
                        // Subtract 1 from one side to ensure exactly 1M elements
                        // Domain size = (max - min + 1), so if we want size = 1M:
                        // max - min = 999,999, so use [center - 500K, center + 499,999]
                        (center.saturating_sub(half_limit), center.saturating_add(half_limit - 1))
                    } else {
                        (new_min, new_max)
                    }
                } else {
                    // No context - use fallback
                    // Fallback must respect domain size limit strictly
                    (DEFAULT_INT_FALLBACK_MIN, DEFAULT_INT_FALLBACK_MAX)
                };
                
                (Val::ValI(inferred_min), Val::ValI(inferred_max))
            }
            (Val::ValF(min_f), Val::ValF(max_f)) => {
                // Check if float variable is unbounded
                let is_unbounded = min_f.is_infinite() || max_f.is_infinite() || 
                                   min_f.is_nan() || max_f.is_nan();
                
                if !is_unbounded {
                    return (min, max); // Already bounded
                }
                
                // Scan existing float variables for context
                let mut global_min: Option<f64> = None;
                let mut global_max: Option<f64> = None;
                
                for var_idx in 0..self.vars.count() {
                    let var_id = crate::variables::VarId::from_index(var_idx);
                    match &self.vars[var_id] {
                        crate::variables::Var::VarF(interval) => {
                            let v_min = interval.min;
                            let v_max = interval.max;
                            
                            // Skip if this variable itself looks unbounded
                            if v_min.is_finite() && v_max.is_finite() {
                                global_min = Some(global_min.map_or(v_min, |gmin| gmin.min(v_min)));
                                global_max = Some(global_max.map_or(v_max, |gmax| gmax.max(v_max)));
                            }
                        }
                        _ => {} // Skip integer variables
                    }
                }
                
                let (inferred_min, inferred_max) = if let (Some(gmin), Some(gmax)) = (global_min, global_max) {
                    // We have context - expand by configured factor (default 1000x)
                    let factor = self.config().unbounded_inference_factor as f64;
                    let span = gmax - gmin;
                    let expansion = span * factor;
                    
                    let new_min = (gmin - expansion).max(-1e308);
                    let new_max = (gmax + expansion).min(1e308);
                    
                    (new_min, new_max)
                } else {
                    // No context - use fallback bounds
                    (DEFAULT_FLOAT_FALLBACK_MIN, DEFAULT_FLOAT_FALLBACK_MAX)
                };
                
                (Val::ValF(inferred_min), Val::ValF(inferred_max))
            }
            (Val::ValI(min_i), Val::ValF(max_f)) => {
                // Mixed type - treat as float
                let min_as_float = min_i as f64;
                self.infer_bounds(Val::ValF(min_as_float), Val::ValF(max_f))
            }
            (Val::ValF(min_f), Val::ValI(max_i)) => {
                // Mixed type - treat as float
                let max_as_float = max_i as f64;
                self.infer_bounds(Val::ValF(min_f), Val::ValF(max_as_float))
            }
        }
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