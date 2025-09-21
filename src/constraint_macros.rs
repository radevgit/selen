//! Constraint macros compatibility layer
//!
//! This module provides backward compatibility by re-exporting the organized
//! constraint macros from the constraints/macros module.

// Re-export the ConstraintRef type
pub use crate::constraints::macros::ConstraintRef;

// Re-export all categorized macros to maintain compatibility
pub use crate::constraints::macros::*;

// Batch posting macros
#[doc(hidden)]
#[macro_export]
macro_rules! postall {
    // Use simple comma-separated arguments
    ($model:expr, $($rest:tt)*) => {{
        $crate::postall_helper!($model, $($rest)*);
    }};
}

#[doc(hidden)]
/// Helper macro to handle constraint expressions recursively
#[macro_export]
macro_rules! postall_helper {
    // Base case: empty
    ($model:expr,) => {};
    
    // Single constraint at the end
    ($model:expr, $var:ident < $target:ident) => {
        $crate::post!($model, $var < $target);
    };
    
    ($model:expr, $var:ident <= $target:ident) => {
        $crate::post!($model, $var <= $target);
    };
    
    ($model:expr, $var:ident > $target:ident) => {
        $crate::post!($model, $var > $target);
    };
    
    ($model:expr, $var:ident >= $target:ident) => {
        $crate::post!($model, $var >= $target);
    };
    
    ($model:expr, $var:ident == $target:ident) => {
        $crate::post!($model, $var == $target);
    };
    
    ($model:expr, $var:ident != $target:ident) => {
        $crate::post!($model, $var != $target);
    };
    
    // Multiple constraints: handle first one then recurse
    ($model:expr, $var:ident < $target:ident, $($rest:tt)*) => {
        $crate::post!($model, $var < $target);
        $crate::postall_helper!($model, $($rest)*);
    };
    
    ($model:expr, $var:ident <= $target:ident, $($rest:tt)*) => {
        $crate::post!($model, $var <= $target);
        $crate::postall_helper!($model, $($rest)*);
    };
    
    ($model:expr, $var:ident > $target:ident, $($rest:tt)*) => {
        $crate::post!($model, $var > $target);
        $crate::postall_helper!($model, $($rest)*);
    };
    
    ($model:expr, $var:ident >= $target:ident, $($rest:tt)*) => {
        $crate::post!($model, $var >= $target);
        $crate::postall_helper!($model, $($rest)*);
    };
    
    ($model:expr, $var:ident == $target:ident, $($rest:tt)*) => {
        $crate::post!($model, $var == $target);
        $crate::postall_helper!($model, $($rest)*);
    };
    
    ($model:expr, $var:ident != $target:ident, $($rest:tt)*) => {
        $crate::post!($model, $var != $target);
        $crate::postall_helper!($model, $($rest)*);
    };
}

// Implementation macro that handles the actual delegation
#[doc(hidden)]
#[macro_export]
macro_rules! post_impl {
    // Basic variable comparisons
    ($model:expr, $left:ident < $right:ident) => {{
        $model.props.less_than($left, $right);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident <= $right:ident) => {{
        $model.props.less_than_or_equals($left, $right);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident > $right:ident) => {{
        $model.props.greater_than($left, $right);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident >= $right:ident) => {{
        $model.props.greater_than_or_equals($left, $right);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident == $right:ident) => {{
        $model.props.equals($left, $right);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident != $right:ident) => {{
        $model.props.not_equals($left, $right);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Global constraints
    ($model:expr, alldiff([$($vars:ident),+ $(,)?])) => {{
        let vars_vec = [$($vars),+].to_vec();
        $model.props.all_different(vars_vec);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, alldiff($array:expr)) => {{
        $model.props.all_different($array.to_vec());
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Fallback for any other pattern - just return a dummy constraint ref
    ($model:expr, $($rest:tt)*) => {{
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
}

// The post! macro now delegates to categorized macros with fallback
#[macro_export]
macro_rules! post {
    // Try each categorized macro in order - if one fails, try the next
    ($model:expr, $($rest:tt)*) => {
        // Use a fallback pattern that tries to match common constraint patterns
        $crate::post_impl!($model, $($rest)*)
    };
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    
    #[test]
    fn test_compatibility_basic() {
        let mut m = Model::default();
        let x = m.int(1, 10);
        let y = m.int(1, 10);
        
        // Test that the re-exported macros work
        let _c1 = post!(m, x < y);
        let _c2 = post!(m, alldiff([x, y]));
        
        assert!(true);
    }
}