//! Comparison constraint propagators
//!
//! This module organizes comparison constraint propagators. The actual implementations
//! are currently in the main props module. This provides a logical organization for
//! future refactoring when the props module is split.

// Note: Comparison propagators are currently implemented in:
// - crate::constraints::props::eq (for equality constraints)
// - crate::constraints::props::neq (for inequality constraints)
// - crate::constraints::props::leq (for less-than-or-equal constraints)
//
// These will be moved here in a future phase of the modularization.