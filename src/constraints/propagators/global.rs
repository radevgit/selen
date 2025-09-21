//! Global constraint propagators
//!
//! This module organizes global constraint propagators. The actual implementations
//! are currently in the main props module. This provides a logical organization for
//! future refactoring when the props module is split.

// Note: Global constraint propagators are currently implemented in:
// - crate::constraints::props::alldiff (for all-different constraints)
// - crate::constraints::props::allequal (for all-equal constraints)  
// - crate::constraints::props::element (for element constraints)
// - crate::constraints::props::count (for counting constraints)
// - crate::constraints::props::table (for table constraints)
//
// These will be moved here in a future phase of the modularization.