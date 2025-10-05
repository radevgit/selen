//! Constraint posting API (re-exported from constraints::api)
//!
//! This module re-exports all constraint posting methods that are implemented
//! as extensions to the Model struct. The actual implementations are organized
//! by category in `src/constraints/api/`.
//!
//! This file exists for backward compatibility and to maintain the Model API surface.
//! All constraint methods are accessible via `Model::method_name()`.
//!
//! ## Method Organization
//!
//! Constraint methods are organized into the following categories:
//!
//! - **Arithmetic operations** (`constraints::api::arithmetic`):
//!   - Binary: add, sub, mul, div, modulo
//!   - Unary: abs
//!   - Aggregate: min, max, sum, sum_iter
//!
//! - **Boolean operations** (`constraints::api::boolean`):
//!   - Logic: bool_and, bool_or, bool_not
//!   - CNF/SAT: bool_clause
//!
//! - **Reified comparisons** (`constraints::api::reified`):
//!   - Integer: int_eq_reif, int_ne_reif, int_lt_reif, int_le_reif, int_gt_reif, int_ge_reif
//!   - Float: float_eq_reif, float_ne_reif, float_lt_reif, float_le_reif, float_gt_reif, float_ge_reif
//!
//! - **Linear constraints** (`constraints::api::linear`):
//!   - Integer: int_lin_eq, int_lin_le, int_lin_ne
//!   - Integer reified: int_lin_eq_reif, int_lin_le_reif, int_lin_ne_reif
//!   - Float: float_lin_eq, float_lin_le, float_lin_ne
//!   - Float reified: float_lin_eq_reif, float_lin_le_reif, float_lin_ne_reif
//!
//! - **Type conversions** (`constraints::api::conversion`):
//!   - int2float, float2int_floor, float2int_ceil, float2int_round
//!
//! - **Array operations** (`constraints::api::array`):
//!   - array_float_minimum, array_float_maximum, array_float_element

// This is intentionally empty - all implementations are now in src/constraints/api/*
// The methods are automatically available on Model through the trait implementations
// in those modules.
