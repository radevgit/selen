#![allow(dead_code)]

use crate::{var::VarId, state::SubjectState, sparse_set::SparseSet};

// float variable
pub struct VarF {
    pub id: VarId,
    observers: SubjectState,
    dom: SparseSet,
}