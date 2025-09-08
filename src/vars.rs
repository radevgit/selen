use crate::prelude::*;
use crate::domain::SparseSet;
use crate::domain::sparse_set::SparseSetState;
use std::ops::{Index, IndexMut};

/// Domain for a decision variable
#[derive(Clone, Debug)]
pub enum Var {
    /// interval of floating-point numbers
    VarF { min: f32, max: f32 },
    /// sparse set for integer domains
    VarI(SparseSet),
}

/// Value type that can represent either an integer or a floating-point number.
#[derive(Copy, Clone, Debug)]
pub enum Val {
    /// Single integer value
    ValI(i32),
    /// Single floating-point value
    ValF(f32),
}

impl Val {
    /// Create an integer value
    pub const fn int(value: i32) -> Self {
        Val::ValI(value)
    }

    /// Create a floating-point value
    pub const fn float(value: f32) -> Self {
        Val::ValF(value)
    }

    /// Get the previous representable value using ULP-based approach
    pub fn prev(self) -> Self {
        use crate::utils::float_prev;
        match self {
            Val::ValI(i) => Val::ValI(i - 1),
            Val::ValF(f) => Val::ValF(float_prev(f)),
        }
    }

    /// Get the next representable value using ULP-based approach
    pub fn next(self) -> Self {
        use crate::utils::float_next;
        match self {
            Val::ValI(i) => Val::ValI(i + 1),
            Val::ValF(f) => Val::ValF(float_next(f)),
        }
    }
}

impl From<i32> for Val {
    fn from(value: i32) -> Self {
        Val::ValI(value)
    }
}

impl From<f32> for Val {
    fn from(value: f32) -> Self {
        Val::ValF(value)
    }
}

impl PartialEq for Val {
    fn eq(&self, other: &Self) -> bool {
        use crate::utils::float_equal;
        match (self, other) {
            (Val::ValI(a), Val::ValI(b)) => a == b,
            (Val::ValF(a), Val::ValF(b)) => float_equal(*a, *b),
            (Val::ValI(a), Val::ValF(b)) => float_equal(*a as f32, *b),
            (Val::ValF(a), Val::ValI(b)) => float_equal(*a, *b as f32),
        }
    }
}

impl Eq for Val {}

impl PartialOrd for Val {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Val::ValI(a), Val::ValI(b)) => a.partial_cmp(b),
            (Val::ValF(a), Val::ValF(b)) => a.partial_cmp(b),
            (Val::ValI(a), Val::ValF(b)) => (*a as f32).partial_cmp(b),
            (Val::ValF(a), Val::ValI(b)) => a.partial_cmp(&(*b as f32)),
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
            (Val::ValI(a), Val::ValF(b)) => Val::ValF(a as f32 + b),
            (Val::ValF(a), Val::ValI(b)) => Val::ValF(a + b as f32),
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
            (Val::ValI(a), Val::ValF(b)) => Val::ValF(a as f32 - b),
            (Val::ValF(a), Val::ValI(b)) => Val::ValF(a - b as f32),
        }
    }
}

impl Var {
    #[doc(hidden)]
    /// Assigned variables have a domain reduced to a singleton.
    pub fn is_assigned(&self) -> bool {
        use crate::utils::float_equal;
        match self {
            Var::VarF { min, max } => float_equal(*min, *max),
            Var::VarI(sparse_set) => sparse_set.is_fixed(),
        }
    }

    #[doc(hidden)]
    /// Midpoint of domain for easier binary splits.
    pub fn mid(&self) -> Val {
        match self {
            Var::VarF { min, max } => Val::ValF(*min + (*max - *min) / 2.0),
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

    #[doc(hidden)]
    /// Extract assignment for decision variable.
    ///
    /// # Panics
    ///
    /// This function will panic if the decision variable is not assigned.
    pub fn get_assignment(&self) -> Val {
        debug_assert!(self.is_assigned());

        match self {
            Var::VarF { min, .. } => Val::ValF(*min),
            Var::VarI(sparse_set) => {
                debug_assert!(sparse_set.is_fixed());
                // For a fixed sparse set, min == max, so we can use either
                Val::ValI(sparse_set.min())
            }
        }
    }
}

/// Store decision variables and expose a limited interface to operate on them.
#[doc(hidden)]
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
            (Val::ValF(min), Val::ValF(max)) => self.0.push(Var::VarF { min, max }),
            // type coercion
            (Val::ValI(min), Val::ValF(max)) => self.0.push(Var::VarF {
                min: min as f32,
                max,
            }),
            (Val::ValF(min), Val::ValI(max)) => self.0.push(Var::VarF {
                min,
                max: max as f32,
            }),
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
    /// use cspsolver::prelude::*;
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

    /// Get an iterator over all variables with their indices for validation.
    #[doc(hidden)]
    pub fn iter_with_indices(&self) -> impl Iterator<Item = (usize, &Var)> {
        self.0.iter().enumerate()
    }

    /// Extract assignment for all decision variables.
    ///
    /// # Panics
    ///
    /// This function will panic if any decision variables are not assigned.
    #[doc(hidden)]
    pub fn into_solution(self) -> Solution {
        // Extract values for each decision variable
        let values: Vec<_> = self.0.into_iter().map(|v| v.get_assignment()).collect();

        Solution::from(values)
    }

    /// Save state of all sparse set variables for efficient backtracking
    #[doc(hidden)]
    pub fn save_sparse_states(&self) -> Vec<Option<SparseSetState>> {
        self.0.iter().map(|var| {
            match var {
                Var::VarF { .. } => None, // Float variables don't need state saving
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
                (Var::VarF { .. }, None) => {
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
/// Decision variable handle that is not bound to a specific memory location.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct VarId(usize);

impl std::fmt::Debug for VarId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VarId({})", self.0)
    }
}

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
