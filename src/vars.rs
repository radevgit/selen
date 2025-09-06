use crate::prelude::*;
use std::ops::{Index, IndexMut};

/// Domain for a decision variable
#[derive(Clone, Debug)]
pub enum Var {
    /// interval of integers
    VarI { min: i32, max: i32 },
    /// interval of floating-point numbers
    VarF { min: f32, max: f32 },
}

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
    /// Assigned variables have a domain reduced to a singleton.
    pub fn is_assigned(&self) -> bool {
        use crate::utils::float_equal;
        match self {
            Var::VarI { min, max } => min == max,
            Var::VarF { min, max } => float_equal(*min, *max),
        }
    }

    /// Midpoint of domain for easier binary splits.
    pub fn mid(&self) -> Val {
        match self {
            Var::VarI { min, max } => Val::ValI(min + (max - min) / 2),
            Var::VarF { min, max } => Val::ValF(*min + (*max - *min) / 2.0),
        }
    }

    /// Extract assignment for decision variable.
    ///
    /// # Panics
    ///
    /// This function will panic if the decision variable is not assigned.
    pub fn get_assignment(&self) -> Val {
        debug_assert!(self.is_assigned());

        match self {
            Var::VarI { min, .. } => Val::ValI(*min),
            Var::VarF { min, .. } => Val::ValF(*min),
        }
    }
}

/// Store decision variables and expose a limited interface to operate on them.
#[derive(Clone, Debug, Default)]
pub struct Vars(Vec<Var>);

impl Vars {
    /// Create a new decision variable.
    pub fn new_var_with_bounds(&mut self, min: Val, max: Val) -> VarId {
        let v = VarId(self.0.len());

        match (min, max) {
            (Val::ValI(min), Val::ValI(max)) => self.0.push(Var::VarI { min, max }),
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

    /// Get handle to an unassigned decision variable.
    pub fn get_unassigned_var(&self) -> Option<VarId> {
        self.0.iter().position(|var| !var.is_assigned()).map(VarId)
    }

    /// Determine if all decision variables are assigned.
    pub fn is_assigned_all(&self) -> bool {
        self.get_unassigned_var().is_none()
    }

    /// Extract assignment for all decision variables.
    ///
    /// # Panics
    ///
    /// This function will panic if any decision variables are not assigned.
    pub fn into_solution(self) -> Solution {
        // Extract values for each decision variable
        let values: Vec<_> = self.0.into_iter().map(|v| v.get_assignment()).collect();

        Solution::from(values)
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
