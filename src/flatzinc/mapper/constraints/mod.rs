//! Constraint mapping modules
//!
//! This module organizes FlatZinc constraint mappers by category.

pub(super) mod comparison;
pub(super) mod linear;
pub(super) mod global;
pub(super) mod reified;
pub(super) mod boolean;
pub(super) mod array;
pub(super) mod element;
pub(super) mod arithmetic;
pub(super) mod counting;
pub(super) mod set;
pub(super) mod global_cardinality;
