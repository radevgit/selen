//! Constraint API module
//!
//! This module provides a clean, organized API for posting constraints to the model.
//! All constraint methods are implemented as extensions to the `Model` struct.
//!
//! ## Organization
//!
//! - **arithmetic**: Mathematical operations (add, sub, mul, div, modulo, abs, min, max, sum)
//! - **boolean**: Boolean logic operations (and, or, not, clause)
//! - **reified**: Reified comparison constraints (int/float eq/ne/lt/le/gt/ge with boolean reification)
//! - **linear**: Linear (weighted sum) constraints (int/float lin_eq/le/ne, reified versions)
//! - **conversion**: Type conversion constraints (int2float, float2int_floor/ceil/round)
//! - **array**: Array operation constraints (array_float_minimum/maximum/element)

pub mod arithmetic;
pub mod boolean;
pub mod reified;
pub mod linear;
pub mod conversion;
pub mod array;
