//! Variable arithmetic operations and value manipulations
//!
//! This module contains operations that can be performed on variables and values,
//! including arithmetic operations, comparisons, and value transformations.
//!
//! Currently all implementations are in vars.rs. This module provides
//! organizational structure for future extraction.

// Note: Variable and value operations are currently implemented in vars.rs:
//
// Val arithmetic operations (lines 251-377):
// - Add trait implementation (lines 251-263)
// - Sub trait implementation (lines 270-282) 
// - Mul trait implementation (lines 283-295)
// - Div trait implementation (lines 296-337)
// - Rem trait implementation (lines 338-377)
// - Sum trait implementation (lines 264-269)
//
// Var operations (lines 379-443):
// - Domain access methods: min(), max(), size(), contains()
// - Domain modification: remove(), remove_below(), remove_above()
// - Backtracking support: save_state(), restore_state()
// - Display and debug implementations
//
// Vars operations (lines 446-752):
// - Variable creation: new_var_with_bounds_and_step(), new_var_with_values()
// - Access methods: count(), indexing implementations
// - Domain management integration
//
// These operations could be organized into logical groups and extracted
// to this module for better maintainability in a future refactoring phase.