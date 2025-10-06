//! Core variable types and fundamental operations.
//!
//! This module contains the fundamental types that form the variable system:
//! - `VarId`: Unique identifier for variables
//! - `Val`: Value type (integer or float) 
//! - `Var`: Variable domain representation
//! - `Vars`: Collection of variables

use crate::variables::domain::{SparseSet, sparse_set::SparseSetState, float_interval::FloatInterval};
use crate::constraints::props::PropId;
use crate::core::solution::Solution;
use std::ops::{Index, IndexMut};

/// Value type that can represent either an integer or a floating-point number.
#[derive(Copy, Clone, Debug)]
pub enum Val {
    /// Single integer value
    ValI(i32),
    /// Single floating-point value
    ValF(f64),
}

impl Val {
    /// Create an integer value
    pub const fn int(value: i32) -> Self {
        Val::ValI(value)
    }

    /// Create a floating-point value
    pub const fn float(value: f64) -> Self {
        Val::ValF(value)
    }

    /// Get the previous representable value
    pub fn prev(self) -> Self {
        match self {
            Val::ValI(i) => Val::ValI(i - 1),
            Val::ValF(_f) => {
                // For single values, we can't use prev/next without knowing the interval
                // This would need to be handled at the variable level
                self // Return unchanged for now
            }
        }
    }

    /// Get the next representable value  
    pub fn next(self) -> Self {
        match self {
            Val::ValI(i) => Val::ValI(i + 1),
            Val::ValF(_f) => {
                // For single values, we can't use prev/next without knowing the interval
                // This would need to be handled at the variable level
                self // Return unchanged for now
            }
        }
    }
    
    /// Extract integer value if this is an integer, None otherwise
    pub fn as_int(self) -> Option<i32> {
        match self {
            Val::ValI(i) => Some(i),
            Val::ValF(_) => None,
        }
    }
    
    /// Extract float value if this is a float, None otherwise  
    pub fn as_float(self) -> Option<f64> {
        match self {
            Val::ValF(f) => Some(f),
            Val::ValI(_) => None,
        }
    }

    /// Check if this value is safe to divide by (not zero or close to zero)
    pub fn is_safe_divisor(self) -> bool {
        match self {
            Val::ValI(i) => i != 0,
            Val::ValF(f) => f.abs() >= f64::EPSILON * 1000.0, // Use a larger epsilon for safety
        }
    }

    /// Safe division that returns None if divisor is too close to zero
    pub fn safe_div(self, other: Val) -> Option<Val> {
        if !other.is_safe_divisor() {
            return None;
        }
        Some(self / other)
    }

    /// Safe modulo that returns None if divisor is too close to zero
    pub fn safe_mod(self, other: Val) -> Option<Val> {
        if !other.is_safe_divisor() {
            return None;
        }
        Some(self % other)
    }

    /// Check if the range [min, max] contains zero or values close to zero
    pub fn range_contains_unsafe_divisor(min: Val, max: Val) -> bool {
        match (min, max) {
            (Val::ValI(min_i), Val::ValI(max_i)) => min_i <= 0 && max_i >= 0,
            (Val::ValF(min_f), Val::ValF(max_f)) => min_f <= f64::EPSILON && max_f >= -f64::EPSILON,
            (Val::ValI(min_i), Val::ValF(max_f)) => min_i as f64 <= f64::EPSILON && max_f >= -f64::EPSILON,
            (Val::ValF(min_f), Val::ValI(max_i)) => min_f <= f64::EPSILON && max_i as f64 >= -f64::EPSILON,
        }
    }

    /// Compare two values with interval context for mathematically correct precision.
    /// This is the preferred method for constraint propagation where interval context is available.
    pub fn equals_with_intervals(&self, other: &Self, 
                                self_interval: Option<&FloatInterval>,
                                other_interval: Option<&FloatInterval>) -> bool {
        match (self, other) {
            (Val::ValI(a), Val::ValI(b)) => a == b,
            (Val::ValF(a), Val::ValF(b)) => {
                match (self_interval, other_interval) {
                    (Some(int_a), Some(int_b)) => {
                        // Use the sum of half-steps as tolerance - this ensures values that
                        // could represent the same conceptual value are considered equal
                        let tolerance = (int_a.step + int_b.step) / 2.0;
                        (a - b).abs() <= tolerance
                    }
                    (Some(int_a), None) => {
                        // Only one interval available, use its precision
                        let tolerance = int_a.step / 2.0;
                        (a - b).abs() <= tolerance
                    }
                    (None, Some(int_b)) => {
                        // Only one interval available, use its precision
                        let tolerance = int_b.step / 2.0;
                        (a - b).abs() <= tolerance
                    }
                    (None, None) => {
                        // No interval context - use direct equality for step-aligned values
                        // This should rarely happen in practice since variables have intervals
                        *a == *b
                    }
                }
            }
            (Val::ValI(i), Val::ValF(f)) => {
                if let Some(f_interval) = other_interval {
                    let tolerance = f_interval.step / 2.0;
                    ((*i as f64) - f).abs() <= tolerance
                } else {
                    // No context, use direct comparison for step-aligned values
                    (*i as f64) == *f
                }
            }
            (Val::ValF(f), Val::ValI(i)) => {
                if let Some(f_interval) = self_interval {
                    let tolerance = f_interval.step / 2.0;
                    (f - (*i as f64)).abs() <= tolerance
                } else {
                    // No context, use direct comparison for step-aligned values
                    *f == (*i as f64)
                }
            }
        }
    }
    
    /// Compare two values with explicit precision tolerance.
    /// Useful when you know the appropriate tolerance but don't have interval objects.
    pub fn equals_with_precision(&self, other: &Self, precision: f64) -> bool {
        match (self, other) {
            (Val::ValI(a), Val::ValI(b)) => a == b,
            (Val::ValF(a), Val::ValF(b)) => (a - b).abs() <= precision,
            (Val::ValI(a), Val::ValF(b)) => ((*a as f64) - b).abs() <= precision,
            (Val::ValF(a), Val::ValI(b)) => (a - (*b as f64)).abs() <= precision,
        }
    }
}

impl From<i32> for Val {
    fn from(value: i32) -> Self {
        Val::ValI(value)
    }
}

impl From<f64> for Val {
    fn from(value: f64) -> Self {
        Val::ValF(value)
    }
}

impl PartialEq for Val {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Val::ValI(a), Val::ValI(b)) => a == b,
            (Val::ValF(a), Val::ValF(b)) => a == b,  // Direct equality for step-aligned values
            (Val::ValI(a), Val::ValF(b)) => (*a as f64) == *b,
            (Val::ValF(a), Val::ValI(b)) => *a == (*b as f64),
        }
    }
}

impl Eq for Val {}

impl PartialOrd for Val {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Val::ValI(a), Val::ValI(b)) => a.partial_cmp(b),
            (Val::ValF(a), Val::ValF(b)) => a.partial_cmp(b),
            (Val::ValI(a), Val::ValF(b)) => (*a as f64).partial_cmp(b),
            (Val::ValF(a), Val::ValI(b)) => a.partial_cmp(&(*b as f64)),
        }
    }
}

impl Ord for Val {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl std::ops::Add for Val {
    type Output = Val;

    fn add(self, other: Val) -> Val {
        match (self, other) {
            (Val::ValI(a), Val::ValI(b)) => Val::ValI(a + b),
            (Val::ValF(a), Val::ValF(b)) => Val::ValF(a + b),
            (Val::ValI(a), Val::ValF(b)) => Val::ValF(a as f64 + b),
            (Val::ValF(a), Val::ValI(b)) => Val::ValF(a + b as f64),
        }
    }
}

impl std::iter::Sum for Val {
    fn sum<I: Iterator<Item = Val>>(iter: I) -> Self {
        iter.fold(Val::ValI(0), |acc, x| acc + x)
    }
}

impl std::ops::Sub for Val {
    type Output = Val;

    fn sub(self, other: Val) -> Val {
        match (self, other) {
            (Val::ValI(a), Val::ValI(b)) => Val::ValI(a - b),
            (Val::ValF(a), Val::ValF(b)) => Val::ValF(a - b),
            (Val::ValI(a), Val::ValF(b)) => Val::ValF(a as f64 - b),
            (Val::ValF(a), Val::ValI(b)) => Val::ValF(a - b as f64),
        }
    }
}

impl std::ops::Mul for Val {
    type Output = Val;

    fn mul(self, other: Val) -> Val {
        match (self, other) {
            (Val::ValI(a), Val::ValI(b)) => Val::ValI(a * b),
            (Val::ValF(a), Val::ValF(b)) => Val::ValF(a * b),
            (Val::ValI(a), Val::ValF(b)) => Val::ValF(a as f64 * b),
            (Val::ValF(a), Val::ValI(b)) => Val::ValF(a * b as f64),
        }
    }
}

impl std::ops::Div for Val {
    type Output = Val;

    fn div(self, other: Val) -> Val {
        match (self, other) {
            (Val::ValI(a), Val::ValI(b)) => {
                if b == 0 {
                    // Return infinity for division by zero
                    if a >= 0 { Val::ValF(f64::INFINITY) } else { Val::ValF(f64::NEG_INFINITY) }
                } else {
                    // For integer division, convert to float to avoid truncation issues
                    Val::ValF(a as f64 / b as f64)
                }
            },
            (Val::ValF(a), Val::ValF(b)) => {
                if b.abs() < f64::EPSILON {
                    // Return infinity for division by value too close to zero
                    if a >= 0.0 { Val::ValF(f64::INFINITY) } else { Val::ValF(f64::NEG_INFINITY) }
                } else {
                    Val::ValF(a / b)
                }
            },
            (Val::ValI(a), Val::ValF(b)) => {
                if b.abs() < f64::EPSILON {
                    // Return infinity for division by value too close to zero
                    if a >= 0 { Val::ValF(f64::INFINITY) } else { Val::ValF(f64::NEG_INFINITY) }
                } else {
                    Val::ValF(a as f64 / b)
                }
            },
            (Val::ValF(a), Val::ValI(b)) => {
                if b == 0 {
                    // Return infinity for division by zero
                    if a >= 0.0 { Val::ValF(f64::INFINITY) } else { Val::ValF(f64::NEG_INFINITY) }
                } else {
                    Val::ValF(a / b as f64)
                }
            },
        }
    }
}

impl std::ops::Rem for Val {
    type Output = Val;

    fn rem(self, other: Val) -> Val {
        match (self, other) {
            (Val::ValI(a), Val::ValI(b)) => {
                if b == 0 {
                    // Return NaN for modulo by zero (undefined behavior)
                    Val::ValF(f64::NAN)
                } else {
                    Val::ValI(a % b)
                }
            },
            (Val::ValF(a), Val::ValF(b)) => {
                if b.abs() < f64::EPSILON {
                    // Return NaN for modulo by value too close to zero
                    Val::ValF(f64::NAN)
                } else {
                    Val::ValF(a % b)
                }
            },
            (Val::ValI(a), Val::ValF(b)) => {
                if b.abs() < f64::EPSILON {
                    // Return NaN for modulo by value too close to zero
                    Val::ValF(f64::NAN)
                } else {
                    Val::ValF(a as f64 % b)
                }
            },
            (Val::ValF(a), Val::ValI(b)) => {
                if b == 0 {
                    // Return NaN for modulo by zero
                    Val::ValF(f64::NAN)
                } else {
                    Val::ValF(a % b as f64)
                }
            },
        }
    }
}

/// Domain for a decision variable
#[derive(Clone, Debug)]
pub enum Var {
    /// interval of floating-point numbers with fixed step size
    VarF(FloatInterval),
    /// sparse set for integer domains
    VarI(SparseSet),
}

impl Var {
    /// Assigned variables have a domain reduced to a singleton.
    #[doc(hidden)]
    pub fn is_assigned(&self) -> bool {
        match self {
            Var::VarF(interval) => interval.is_fixed(),
            Var::VarI(sparse_set) => sparse_set.is_fixed(),
        }
    }

    /// Midpoint of domain for easier binary splits.
    #[doc(hidden)]
    pub fn mid(&self) -> Val {
        match self {
            Var::VarF(interval) => Val::ValF(interval.mid()),
            Var::VarI(sparse_set) => {
                if sparse_set.is_empty() {
                    // Should not happen in a valid CSP, but provide a fallback
                    Val::ValI(0)
                } else {
                    // Use the midpoint between min and max for binary search
                    // For proper binary search, we need to ensure the midpoint is always valid
                    // and that the split makes progress
                    let min_val = sparse_set.min();
                    let max_val = sparse_set.max();
                    
                    // If domain is a single value, return that value
                    if min_val == max_val {
                        return Val::ValI(min_val);
                    }
                    
                    // Calculate midpoint - use floor division to ensure left bias
                    // This ensures that for [-1, 0], mid = -1, giving:
                    // Left: x <= -1 (just [-1])
                    // Right: x >= 0 (just [0])
                    let mid_val = min_val + (max_val - min_val) / 2;
                    Val::ValI(mid_val)
                }
            }
        }
    }

    /// Extract assignment for decision variable.
    ///
    /// # Panics
    ///
    /// This function will panic if the decision variable is not assigned.
    #[doc(hidden)]
    pub fn get_assignment(&self) -> Val {
        debug_assert!(self.is_assigned());

        match self {
            Var::VarF(interval) => Val::ValF(interval.min),
            Var::VarI(sparse_set) => {
                debug_assert!(sparse_set.is_fixed());
                // For a fixed sparse set, min == max, so we can use either
                Val::ValI(sparse_set.min())
            }
        }
    }
}

/// Decision variable handle that is not bound to a specific memory location.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct VarId(usize);

impl VarId {
    /// Create a VarId from a usize index (for internal use)
    pub(crate) fn from_index(index: usize) -> Self {
        VarId(index)
    }
    
    /// Get the internal index as usize (for internal use)
    pub(crate) fn to_index(self) -> usize {
        self.0
    }
}

impl std::fmt::Debug for VarId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VarId({})", self.0)
    }
}

/// Collection of decision variables
#[derive(Clone, Debug, Default)]
pub struct Vars(Vec<Var>);

impl Vars {
    /// Create a new empty collection of variables.
    #[doc(hidden)]
    pub fn new() -> Self {
        Vars(Vec::new())
    }

    /// Create a new decision variable.
    #[doc(hidden)]
    pub fn new_var_with_bounds(&mut self, min: Val, max: Val) -> VarId {
        let v = VarId(self.0.len());

        match (min, max) {
            (Val::ValI(min), Val::ValI(max)) => {
                // Create SparseSet for integer variables
                let sparse_set = SparseSet::new(min, max);
                self.0.push(Var::VarI(sparse_set))
            },
            (Val::ValF(min), Val::ValF(max)) => {
                let interval = FloatInterval::new(min as f64, max as f64);
                self.0.push(Var::VarF(interval))
            },
            // type coercion
            (Val::ValI(min), Val::ValF(max)) => {
                let interval = FloatInterval::new(min as f64, max as f64);
                self.0.push(Var::VarF(interval))
            },
            (Val::ValF(min), Val::ValI(max)) => {
                let interval = FloatInterval::new(min as f64, max as f64);
                self.0.push(Var::VarF(interval))
            },
        }

        v
    }

    /// Create a new decision variable with custom float step size.
    #[doc(hidden)]
    pub fn new_var_with_bounds_and_step(&mut self, min: Val, max: Val, float_step: f64) -> VarId {
        let v = VarId(self.0.len());

        match (min, max) {
            (Val::ValI(min), Val::ValI(max)) => {
                // Create SparseSet for integer variables - use unchecked to preserve invalid bounds
                let sparse_set = SparseSet::new_unchecked(min, max);
                self.0.push(Var::VarI(sparse_set))
            },
            (Val::ValF(min), Val::ValF(max)) => {
                let interval = FloatInterval::with_step_unchecked(min as f64, max as f64, float_step);
                self.0.push(Var::VarF(interval))
            },
            // type coercion
            (Val::ValI(min), Val::ValF(max)) => {
                let interval = FloatInterval::with_step_unchecked(min as f64, max as f64, float_step);
                self.0.push(Var::VarF(interval))
            },
            (Val::ValF(min), Val::ValI(max)) => {
                let interval = FloatInterval::with_step_unchecked(min as f64, max as f64, float_step);
                self.0.push(Var::VarF(interval))
            },
        }

        v
    }

    /// Create a new integer decision variable from a vector of specific values.
    /// This is useful for creating variables with non-contiguous domains.
    /// 
    /// # Arguments
    /// * `values` - Vector of integer values that the variable can take
    /// 
    /// # Returns
    /// A new VarId for the created variable
    /// 
    /// # Example
    /// ```
    /// use selen::prelude::*;
    /// let mut vars = Vars::new();
    /// let var = vars.new_var_with_values(vec![2, 4, 6, 8]); // Even numbers only
    /// ```
    #[doc(hidden)]
    pub fn new_var_with_values(&mut self, values: Vec<i32>) -> VarId {
        let v = VarId(self.0.len());
        let sparse_set = SparseSet::new_from_values(values);
        self.0.push(Var::VarI(sparse_set));
        v
    }

    /// Get handle to an unassigned decision variable using Most Restricted Variable (MRV) heuristic.
    /// 
    /// Get the first unassigned variable.
    #[doc(hidden)]
    pub fn get_unassigned_var(&self) -> Option<VarId> {
        for (index, var) in self.0.iter().enumerate() {
            if !var.is_assigned() {
                return Some(VarId(index));
            }
        }
        
        None
    }

    /// Determine if all decision variables are assigned.
    #[doc(hidden)]
    pub fn is_assigned_all(&self) -> bool {
        self.get_unassigned_var().is_none()
    }
    
    /// Get the number of variables in this collection.
    #[doc(hidden)]
    pub fn count(&self) -> usize {
        self.0.len()
    }

    /// Get an iterator over all variables with their indices for validation.
    #[doc(hidden)]
    pub fn iter_with_indices(&self) -> impl Iterator<Item = (usize, &Var)> {
        self.0.iter().enumerate()
    }
    
    /// Get the FloatInterval for a variable if it's a float variable.
    /// Returns None for integer variables.
    #[doc(hidden)]
    pub fn get_float_interval(&self, var_id: VarId) -> Option<&FloatInterval> {
        match &self.0[var_id.0] {
            Var::VarF(interval) => Some(interval),
            Var::VarI(_) => None,
        }
    }
    
    /// Compare two variable values with proper interval context.
    /// This is the mathematically correct way to compare values from different variables.
    #[doc(hidden)]
    pub fn values_equal(&self, var_a: VarId, val_a: &Val, var_b: VarId, val_b: &Val) -> bool {
        let interval_a = self.get_float_interval(var_a);
        let interval_b = self.get_float_interval(var_b);
        val_a.equals_with_intervals(val_b, interval_a, interval_b)
    }

    /// Extract assignment for all decision variables.
    ///
    /// # Panics
    ///
    /// This function will panic if any decision variables are not assigned.
    #[doc(hidden)]
    pub fn into_solution(self) -> Solution {
        // DEBUG: Print variable states before extraction
        println!("=== SOLUTION EXTRACTION DEBUG ===");
        for (i, v) in self.0.iter().enumerate() {
            match v {
                Var::VarF(interval) => {
                    println!("Float var {}: min={}, max={}, step={}, is_fixed={}, step_count={}", 
                        i, interval.min, interval.max, interval.step, 
                        interval.is_fixed(), interval.step_count());
                }
                Var::VarI(sparse_set) => {
                    println!("Int var {}: min={}, max={}, is_fixed={}", 
                        i, sparse_set.min(), sparse_set.max(), sparse_set.is_fixed());
                }
            }
        }
        
        // Extract values for each decision variable - convert to old Val type for now
        let values: Vec<crate::variables::Val> = self.0.into_iter().map(|v| {
            let val = v.get_assignment();
            match val {
                Val::ValI(i) => crate::variables::Val::ValI(i),
                Val::ValF(f) => crate::variables::Val::ValF(f),
            }
        }).collect();

        println!("=== END SOLUTION EXTRACTION DEBUG ===");
        Solution::from(values)
    }

    /// Extract assignment for all decision variables with statistics.
    ///
    /// # Panics
    ///
    /// This function will panic if any decision variables are not assigned.
    #[doc(hidden)]
    pub fn into_solution_with_stats(self, stats: crate::core::solution::SolveStats) -> Solution {
        // DEBUG: Print variable states before extraction
        println!("=== SOLUTION EXTRACTION DEBUG (with stats) ===");
        for (i, v) in self.0.iter().enumerate() {
            match v {
                Var::VarF(interval) => {
                    println!("Float var {}: min={}, max={}, step={}, is_fixed={}, step_count={}", 
                        i, interval.min, interval.max, interval.step, 
                        interval.is_fixed(), interval.step_count());
                }
                Var::VarI(sparse_set) => {
                    println!("Int var {}: min={}, max={}, is_fixed={}", 
                        i, sparse_set.min(), sparse_set.max(), sparse_set.is_fixed());
                }
            }
        }
        
        // Extract values for each decision variable - convert to old Val type for now
        let values: Vec<crate::variables::Val> = self.0.into_iter().map(|v| {
            let val = v.get_assignment();
            match val {
                Val::ValI(i) => crate::variables::Val::ValI(i),
                Val::ValF(f) => crate::variables::Val::ValF(f),
            }
        }).collect();

        println!("=== END SOLUTION EXTRACTION DEBUG (with stats) ===");
        Solution::new(values, stats)
    }

    /// Save state of all sparse set variables for efficient backtracking
    #[doc(hidden)]
    pub fn save_sparse_states(&self) -> Vec<Option<SparseSetState>> {
        self.0.iter().map(|var| {
            match var {
                Var::VarF(_) => None, // Float variables don't need state saving
                Var::VarI(sparse_set) => Some(sparse_set.save_state()),
            }
        }).collect()
    }

    /// Restore state of all sparse set variables from saved state
    #[doc(hidden)]
    pub fn restore_sparse_states(&mut self, states: &[Option<SparseSetState>]) {
        debug_assert_eq!(self.0.len(), states.len(), "State vector size mismatch");
        
        for (var, state_opt) in self.0.iter_mut().zip(states.iter()) {
            match (var, state_opt) {
                (Var::VarF(_), None) => {
                    // Float variables don't have saved state - nothing to restore
                }
                (Var::VarI(sparse_set), Some(state)) => {
                    sparse_set.restore_state(state);
                }
                _ => {
                    debug_assert!(false, "Mismatched variable type and state");
                }
            }
        }
    }

    /// Iterator over variables for analysis
    #[doc(hidden)]
    pub fn iter(&self) -> std::slice::Iter<'_, Var> {
        self.0.iter()
    }
}

// Index implementations
impl Index<VarId> for Vars {
    type Output = Var;

    fn index(&self, index: VarId) -> &Self::Output {
        &self.0[index.0]
    }
}

impl IndexMut<VarId> for Vars {
    fn index_mut(&mut self, index: VarId) -> &mut Self::Output {
        &mut self.0[index.0]
    }
}

impl Index<VarId> for Vec<Var> {
    type Output = Var;

    fn index(&self, index: VarId) -> &Self::Output {
        &self[index.0]
    }
}

impl IndexMut<VarId> for Vec<Var> {
    fn index_mut(&mut self, index: VarId) -> &mut Self::Output {
        &mut self[index.0]
    }
}

impl Index<VarId> for Vec<Vec<PropId>> {
    type Output = Vec<PropId>;

    fn index(&self, index: VarId) -> &Self::Output {
        &self[index.0]
    }
}

impl IndexMut<VarId> for Vec<Vec<PropId>> {
    fn index_mut(&mut self, index: VarId) -> &mut Self::Output {
        &mut self[index.0]
    }
}

impl Index<VarId> for Vec<Val> {
    type Output = Val;

    fn index(&self, index: VarId) -> &Self::Output {
        &self[index.0]
    }
}

impl IndexMut<VarId> for Vec<Val> {
    fn index_mut(&mut self, index: VarId) -> &mut Self::Output {
        &mut self[index.0]
    }
}

/// Wrapper to provide specific helper methods for binary decision variables.
#[derive(Clone, Copy, Debug)]
pub struct VarIdBin(pub(crate) VarId);

impl From<VarId> for VarIdBin {
    fn from(var_id: VarId) -> Self {
        VarIdBin(var_id)
    }
}

impl From<VarIdBin> for VarId {
    fn from(var_id_bin: VarIdBin) -> Self {
        var_id_bin.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_var_with_values_basic() {
        let mut vars = Vars::new();
        let var_id = vars.new_var_with_values(vec![2, 4, 6, 8]);
        
        let var = &vars[var_id];
        let Var::VarI(sparse_set) = var else {
            assert!(false, "Expected integer variable");
            return;
        };
        assert_eq!(sparse_set.size(), 4);
        assert!(sparse_set.contains(2));
        assert!(sparse_set.contains(4));
        assert!(sparse_set.contains(6));
        assert!(sparse_set.contains(8));
        assert!(!sparse_set.contains(3));
        assert!(!sparse_set.contains(5));
    }

    #[test]
    fn test_new_var_with_values_single() {
        let mut vars = Vars::new();
        let var_id = vars.new_var_with_values(vec![42]);
        
        let var = &vars[var_id];
        let Var::VarI(sparse_set) = var else {
            assert!(false, "Expected integer variable");
            return;
        };
        assert_eq!(sparse_set.size(), 1);
        assert!(sparse_set.is_fixed());
        assert!(sparse_set.contains(42));
        assert!(!sparse_set.contains(41));
    }

    #[test]
    fn test_new_var_with_values_duplicates() {
        let mut vars = Vars::new();
        let var_id = vars.new_var_with_values(vec![1, 3, 1, 5, 3]);
        
        let var = &vars[var_id];
        let Var::VarI(sparse_set) = var else {
            assert!(false, "Expected integer variable");
            return;
        };
        assert_eq!(sparse_set.size(), 3); // Should deduplicate
        assert!(sparse_set.contains(1));
        assert!(sparse_set.contains(3));
        assert!(sparse_set.contains(5));
    }

    #[test]
    fn test_var_with_values_assignment() {
        let mut vars = Vars::new();
        let var_id = vars.new_var_with_values(vec![10, 20, 30]);
        
        let var = &vars[var_id];
        assert!(!var.is_assigned());
        
        // Test midpoint calculation
        let mid = var.mid();
        let Val::ValI(val) = mid else {
            assert!(false, "Expected integer value");
            return;
        };
        // Midpoint should be reasonable
        assert!(val >= 10 && val <= 30);
    }

    #[test]
    fn test_equivalence_with_range_creation() {
        let mut vars1 = Vars::new();
        let mut vars2 = Vars::new();
        
        // Create equivalent variables using different methods
        let var1_id = vars1.new_var_with_bounds(Val::int(1), Val::int(5));
        let var2_id = vars2.new_var_with_values(vec![1, 2, 3, 4, 5]);
        
        let var1 = &vars1[var1_id];
        let var2 = &vars2[var2_id];
        
        // Both should have the same domain
        let (Var::VarI(sparse1), Var::VarI(sparse2)) = (var1, var2) else {
            assert!(false, "Expected both to be integer variables");
            return;
        };
        assert_eq!(sparse1.size(), sparse2.size());
        assert_eq!(sparse1.min(), sparse2.min());
        assert_eq!(sparse1.max(), sparse2.max());
        
        // All values should be the same
        for i in 1..=5 {
            assert_eq!(sparse1.contains(i), sparse2.contains(i));
        }
    }
}

// Re-export core types from vars.rs


// Note: Core variable types are currently implemented in vars.rs:
//
// VarId - Opaque variable handle (lines 751-763)
// VarIdBin - Binary variable handle (line 829) 
// Val - Value type enum (lines 52-58, with implementations 59-377)
// Var - Variable domain enum (lines 43-51, with implementations 379-443)
// Vars - Variable storage struct (lines 444-752)
//
// These core types form the foundation of the variable system and could
// be moved to this module in a future refactoring phase.