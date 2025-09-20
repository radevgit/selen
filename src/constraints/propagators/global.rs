//! Global constraint propagators
//!
//! This module organizes global constraint propagators. The actual implementations
//! are currently in the main props module. This provides a logical organization for
//! future refactoring when the props module is split.

// Note: Global constraint propagators are currently implemented in:
// - crate::props::alldiff (for all-different constraints)
// - crate::props::allequal (for all-equal constraints)  
// - crate::props::element (for element constraints)
// - crate::props::count (for counting constraints)
// - crate::props::table (for table constraints)
//
// These will be moved here in a future phase of the modularization.