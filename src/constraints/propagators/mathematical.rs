//! Mathematical function constraint propagators
//!
//! This module organizes mathematical function constraint propagators. The actual
//! implementations are currently in the main props module. This provides a logical 
//! organization for future refactoring when the props module is split.

// Note: Mathematical function propagators are currently implemented in:
// - crate::props::abs (for absolute value constraints)
// - crate::props::min (for minimum constraints)
// - crate::props::max (for maximum constraints)
// - crate::props::sum (for summation constraints)
//
// These will be moved here in a future phase of the modularization.