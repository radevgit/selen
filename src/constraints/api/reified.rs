//! Reified constraint operations
//!
//! **Note:** The old type-specific reified methods (int_eq_reif, float_eq_reif, etc.)  
//! have been removed. Use the new generic functions instead:
//! 
//! - `eq_reif(&mut model, x, y, b)` - replaces int_eq_reif and float_eq_reif
//! - `ne_reif(&mut model, x, y, b)` - replaces int_ne_reif and float_ne_reif
//! - `lt_reif(&mut model, x, y, b)` - replaces int_lt_reif and float_lt_reif
//! - `le_reif(&mut model, x, y, b)` - replaces int_le_reif and float_le_reif
//! - `gt_reif(&mut model, x, y, b)` - replaces int_gt_reif and float_gt_reif
//! - `ge_reif(&mut model, x, y, b)` - replaces int_ge_reif and float_ge_reif
//!
//! These generic functions are available in the prelude and work for both int and float types.

// No Model methods needed - use the generic functions from constraints::functions instead
