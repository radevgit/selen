//! Arithmetic constraint propagators
//!
//! This module organizes arithmetic constraint propagators. The actual implementations
//! are currently in the main props module. This provides a logical organization for
//! future refactoring when the props module is split.

// Note: Arithmetic propagators are currently implemented in:
// - crate::constraints::props::add (for addition constraints)
// - crate::constraints::props::mul (for multiplication constraints)  
// - crate::constraints::props::div (for division constraints)
// - crate::constraints::props::modulo (for modulo constraints)
//
// These will be moved here in a future phase of the modularization.