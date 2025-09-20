//! Arithmetic constraint propagators
//!
//! This module organizes arithmetic constraint propagators. The actual implementations
//! are currently in the main props module. This provides a logical organization for
//! future refactoring when the props module is split.

// Note: Arithmetic propagators are currently implemented in:
// - crate::props::add (for addition constraints)
// - crate::props::mul (for multiplication constraints)  
// - crate::props::div (for division constraints)
// - crate::props::modulo (for modulo constraints)
//
// These will be moved here in a future phase of the modularization.