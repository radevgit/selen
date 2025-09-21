//! Logical constraint propagators
//!
//! This module organizes logical constraint propagators. The actual implementations
//! are currently in the main props module. This provides a logical organization for
//! future refactoring when the props module is split.

// Note: Logical propagators are currently implemented in:
// - crate::constraints::props::bool_logic (for boolean operations)
// - crate::constraints::props::conditional (for conditional constraints)
//
// These will be moved here in a future phase of the modularization.