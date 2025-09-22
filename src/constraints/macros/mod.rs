//! Constraint Macros Module
//!
//! This module provides constraint posting macros with a general dispatch system.

// mod comparison;  // Temporarily disabled due to corruption issues
mod arithmetic;
mod logical;
mod global;

// pub use comparison::*;  // Temporarily disabled
// pub use arithmetic::*;  // Now inline in dispatch system
// pub use logical::*;     // Now inline in dispatch system  
// pub use global::*;      // Now inline in dispatch system

#[doc(hidden)]
/// Represents a constraint reference that can be used later
#[derive(Debug, Clone, Copy)]
pub struct ConstraintRef {
    /// Internal constraint ID (for future constraint management)
    id: usize,
}

impl ConstraintRef {
    /// Create a new constraint reference
    pub fn new(id: usize) -> Self {
        Self { id }
    }
    
    /// Get the constraint ID
    pub fn id(&self) -> usize {
        self.id
    }
}

/// General constraint posting macro that dispatches to specialized macros
#[macro_export]
macro_rules! post {
    // ============================================================================
    // MATHEMATICAL FUNCTIONS - direct dispatch to arithmetic
    // ============================================================================
    ($model:expr, abs($($args:tt)*) $($rest:tt)*) => {
        $crate::post_arithmetic!($model, abs($($args)*) $($rest)*)
    };
    ($model:expr, sum($($args:tt)*) $($rest:tt)*) => {
        $crate::post_arithmetic!($model, sum($($args)*) $($rest)*)
    };
    
    // ============================================================================
    // GLOBAL CONSTRAINTS - direct dispatch to global
    // ============================================================================
    ($model:expr, alldiff $($rest:tt)*) => {
        $crate::post_global!($model, alldiff $($rest)*)
    };
    ($model:expr, allequal $($rest:tt)*) => {
        $crate::post_global!($model, allequal $($rest)*)
    };
    ($model:expr, element $($rest:tt)*) => {
        $crate::post_global!($model, element $($rest)*)
    };
    ($model:expr, min $($rest:tt)*) => {
        $crate::post_global!($model, min $($rest)*)
    };
    ($model:expr, max $($rest:tt)*) => {
        $crate::post_global!($model, max $($rest)*)
    };
    
    // ============================================================================
    // LOGICAL OPERATIONS - direct dispatch to logical
    // ============================================================================
    ($model:expr, and $($rest:tt)*) => {
        $crate::post_logical!($model, and $($rest)*)
    };
    ($model:expr, or $($rest:tt)*) => {
        $crate::post_logical!($model, or $($rest)*)
    };
    ($model:expr, not $($rest:tt)*) => {
        $crate::post_logical!($model, not $($rest)*)
    };
    
    // ============================================================================
    // ARITHMETIC EXPRESSIONS - handle patterns with arithmetic operators  
    // ============================================================================
    ($model:expr, $first:tt + $($rest:tt)*) => {
        $crate::post_arithmetic!($model, $first + $($rest)*)
    };
    ($model:expr, $first:tt * $($rest:tt)*) => {
        $crate::post_arithmetic!($model, $first * $($rest)*)
    };
    ($model:expr, $first:tt / $($rest:tt)*) => {
        $crate::post_arithmetic!($model, $first / $($rest)*)
    };
    
    // ============================================================================
    // COMPARISON PATTERNS - handle variable == literal directly
    // ============================================================================
    // Array element patterns: vars[index] op literal
    ($model:expr, $array:ident [ $index:expr ] == $right:literal) => {{
        $model.props.equals($array[$index], $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    ($model:expr, $array:ident [ $index:expr ] != $right:literal) => {{
        $model.props.not_equals($array[$index], $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    ($model:expr, $array:ident [ $index:expr ] < $right:literal) => {{
        $model.props.less_than($array[$index], $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    ($model:expr, $array:ident [ $index:expr ] <= $right:literal) => {{
        $model.props.less_than_or_equals($array[$index], $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    ($model:expr, $array:ident [ $index:expr ] > $right:literal) => {{
        $model.props.greater_than($array[$index], $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    ($model:expr, $array:ident [ $index:expr ] >= $right:literal) => {{
        $model.props.greater_than_or_equals($array[$index], $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // Array element patterns: vars[index] op vars[index2]
    ($model:expr, $array1:ident [ $index1:expr ] == $array2:ident [ $index2:expr ]) => {{
        $model.props.equals($array1[$index1], $array2[$index2]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    ($model:expr, $array1:ident [ $index1:expr ] != $array2:ident [ $index2:expr ]) => {{
        $model.props.not_equals($array1[$index1], $array2[$index2]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    ($model:expr, $array1:ident [ $index1:expr ] < $array2:ident [ $index2:expr ]) => {{
        $model.props.less_than($array1[$index1], $array2[$index2]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    ($model:expr, $array1:ident [ $index1:expr ] <= $array2:ident [ $index2:expr ]) => {{
        $model.props.less_than_or_equals($array1[$index1], $array2[$index2]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    ($model:expr, $array1:ident [ $index1:expr ] > $array2:ident [ $index2:expr ]) => {{
        $model.props.greater_than($array1[$index1], $array2[$index2]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    ($model:expr, $array1:ident [ $index1:expr ] >= $array2:ident [ $index2:expr ]) => {{
        $model.props.greater_than_or_equals($array1[$index1], $array2[$index2]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // Array element op simple variable
    ($model:expr, $array:ident [ $index:expr ] == $right:ident) => {{
        $model.props.equals($array[$index], $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    ($model:expr, $array:ident [ $index:expr ] != $right:ident) => {{
        $model.props.not_equals($array[$index], $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    ($model:expr, $array:ident [ $index:expr ] < $right:ident) => {{
        $model.props.less_than($array[$index], $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    ($model:expr, $array:ident [ $index:expr ] <= $right:ident) => {{
        $model.props.less_than_or_equals($array[$index], $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    ($model:expr, $array:ident [ $index:expr ] > $right:ident) => {{
        $model.props.greater_than($array[$index], $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    ($model:expr, $array:ident [ $index:expr ] >= $right:ident) => {{
        $model.props.greater_than_or_equals($array[$index], $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // Simple variable op array element
    ($model:expr, $left:ident == $array:ident [ $index:expr ]) => {{
        $model.props.equals($left, $array[$index]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    ($model:expr, $left:ident != $array:ident [ $index:expr ]) => {{
        $model.props.not_equals($left, $array[$index]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    ($model:expr, $left:ident < $array:ident [ $index:expr ]) => {{
        $model.props.less_than($left, $array[$index]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    ($model:expr, $left:ident <= $array:ident [ $index:expr ]) => {{
        $model.props.less_than_or_equals($left, $array[$index]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    ($model:expr, $left:ident > $array:ident [ $index:expr ]) => {{
        $model.props.greater_than($left, $array[$index]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    ($model:expr, $left:ident >= $array:ident [ $index:expr ]) => {{
        $model.props.greater_than_or_equals($left, $array[$index]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // Simple identifier patterns
    ($model:expr, $left:ident == $right:literal) => {{
        $model.props.equals($left, $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    ($model:expr, $left:ident != $right:literal) => {{
        $model.props.not_equals($left, $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    ($model:expr, $left:ident < $right:literal) => {{
        $model.props.less_than($left, $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    ($model:expr, $left:ident <= $right:literal) => {{
        $model.props.less_than_or_equals($left, $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    ($model:expr, $left:ident > $right:literal) => {{
        $model.props.greater_than($left, $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    ($model:expr, $left:ident >= $right:literal) => {{
        $model.props.greater_than_or_equals($left, $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // Variable to variable patterns
    ($model:expr, $left:ident == $right:ident) => {{
        $model.props.equals($left, $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    ($model:expr, $left:ident != $right:ident) => {{
        $model.props.not_equals($left, $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    ($model:expr, $left:ident < $right:ident) => {{
        $model.props.less_than($left, $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    ($model:expr, $left:ident <= $right:ident) => {{
        $model.props.less_than_or_equals($left, $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    ($model:expr, $left:ident > $right:ident) => {{
        $model.props.greater_than($left, $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    ($model:expr, $left:ident >= $right:ident) => {{
        $model.props.greater_than_or_equals($left, $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // ============================================================================
    // ARITHMETIC PATTERNS - handle result == arithmetic operations
    // ============================================================================
    
    // result == x * int(N)
    ($model:expr, $result:ident == $left:ident * int($value:expr)) => {{
        let _constant_var = $model.int($value, $value);
        let _prod_var = $model.mul($left, _constant_var);
        $model.props.equals($result, _prod_var);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // result == x * float(N)
    ($model:expr, $result:ident == $left:ident * float($value:expr)) => {{
        let _constant_var = $model.float($value, $value);
        let _prod_var = $model.mul($left, _constant_var);
        $model.props.equals($result, _prod_var);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x >= float(N)
    ($model:expr, $left:ident >= float($value:expr)) => {{
        let _constant_var = $model.float($value, $value);
        $model.props.greater_than_or_equals($left, _constant_var);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x <= float(N)
    ($model:expr, $left:ident <= float($value:expr)) => {{
        let _constant_var = $model.float($value, $value);
        $model.props.less_than_or_equals($left, _constant_var);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x > float(N)
    ($model:expr, $left:ident > float($value:expr)) => {{
        let _constant_var = $model.float($value, $value);
        $model.props.greater_than($left, _constant_var);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x < float(N)
    ($model:expr, $left:ident < float($value:expr)) => {{
        let _constant_var = $model.float($value, $value);
        $model.props.less_than($left, _constant_var);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x >= int(N)
    ($model:expr, $left:ident >= int($value:expr)) => {{
        let _constant_var = $model.int($value, $value);
        $model.props.greater_than_or_equals($left, _constant_var);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x <= int(N)
    ($model:expr, $left:ident <= int($value:expr)) => {{
        let _constant_var = $model.int($value, $value);
        $model.props.less_than_or_equals($left, _constant_var);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x > int(N)
    ($model:expr, $left:ident > int($value:expr)) => {{
        let _constant_var = $model.int($value, $value);
        $model.props.greater_than($left, _constant_var);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x < int(N)
    ($model:expr, $left:ident < int($value:expr)) => {{
        let _constant_var = $model.int($value, $value);
        $model.props.less_than($left, _constant_var);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x == int(N)
    ($model:expr, $left:ident == int($value:expr)) => {{
        let _constant_var = $model.int($value, $value);
        $model.props.equals($left, _constant_var);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x == float(N)
    ($model:expr, $left:ident == float($value:expr)) => {{
        let _constant_var = $model.float($value, $value);
        $model.props.equals($left, _constant_var);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // Temporary fallback for complex expressions we haven't implemented yet
    // TODO: Implement proper handling for parenthesized arithmetic expressions
    ($model:expr, $($tokens:tt)*) => {{
        // For now, just return a dummy constraint ref
        // println!("Unhandled constraint pattern: {}", stringify!($($tokens)*));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! postall {
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
    
    // ============================================================================
    // ARITHMETIC EXPRESSIONS WITH COMMAS
    // ============================================================================
    
    // x + y == z, rest...
    ($model:expr, $left:ident + $middle:ident == $right:ident, $($rest:tt)*) => {
        $crate::post!($model, $left + $middle == $right);
        $crate::postall_helper!($model, $($rest)*);
    };
    
    // x * y == z, rest...
    ($model:expr, $left:ident * $middle:ident == $right:ident, $($rest:tt)*) => {
        $crate::post!($model, $left * $middle == $right);
        $crate::postall_helper!($model, $($rest)*);
    };
    
    // x + y <= int(N), rest...
    ($model:expr, $left:ident + $middle:ident <= int($right:expr), $($rest:tt)*) => {
        $crate::post!($model, $left + $middle <= int($right));
        $crate::postall_helper!($model, $($rest)*);
    };
    
    // abs(x) == y, rest...
    ($model:expr, abs($var:ident) == $target:ident, $($rest:tt)*) => {
        $crate::post!($model, abs($var) == $target);
        $crate::postall_helper!($model, $($rest)*);
    };
    
    // ============================================================================
    // GLOBAL CONSTRAINTS WITH COMMAS
    // ============================================================================
    
    // alldiff([vars]), rest...
    ($model:expr, alldiff([$($vars:expr),*]), $($rest:tt)*) => {
        $crate::post!($model, alldiff([$($vars),*]));
        $crate::postall_helper!($model, $($rest)*);
    };
    
    // alldiff(vec![vars]), rest...
    ($model:expr, alldiff(vec![$($vars:expr),*]), $($rest:tt)*) => {
        $crate::post!($model, alldiff(vec![$($vars),*]));
        $crate::postall_helper!($model, $($rest)*);
    };
    
    // element(index, array, value), rest...
    ($model:expr, element($index:expr, $array:expr, $value:expr), $($rest:tt)*) => {
        $crate::post!($model, element($index, $array, $value));
        $crate::postall_helper!($model, $($rest)*);
    };
    
    // ============================================================================
    // LOGICAL CONSTRAINTS WITH COMMAS
    // ============================================================================
    
    // or(a, b), rest...
    ($model:expr, or($($vars:expr),+), $($rest:tt)*) => {
        $crate::post!($model, or($($vars),+));
        $crate::postall_helper!($model, $($rest)*);
    };
    
    // and(a, b), rest...
    ($model:expr, and($($vars:expr),+), $($rest:tt)*) => {
        $crate::post!($model, and($($vars),+));
        $crate::postall_helper!($model, $($rest)*);
    };
    
    // or([vars]), rest...
    ($model:expr, or([$($vars:expr),*]), $($rest:tt)*) => {
        $crate::post!($model, or([$($vars),*]));
        $crate::postall_helper!($model, $($rest)*);
    };
    
    // and([vars]), rest...
    ($model:expr, and([$($vars:expr),*]), $($rest:tt)*) => {
        $crate::post!($model, and([$($vars),*]));
        $crate::postall_helper!($model, $($rest)*);
    };
    
    // ============================================================================
    // COMPARISON PATTERNS WITH LITERALS AND EXPRESSIONS
    // ============================================================================
    
    // x == int(N), rest...
    ($model:expr, $var:ident == int($value:expr), $($rest:tt)*) => {
        $crate::post!($model, $var == int($value));
        $crate::postall_helper!($model, $($rest)*);
    };
    
    // x <= float(N), rest...
    ($model:expr, $var:ident <= float($value:expr), $($rest:tt)*) => {
        $crate::post!($model, $var <= float($value));
        $crate::postall_helper!($model, $($rest)*);
    };
    
    // vars[i] == vars[j], rest...
    ($model:expr, $left_array:ident[$left_index:expr] == $right_array:ident[$right_index:expr], $($rest:tt)*) => {
        $crate::post!($model, $left_array[$left_index] == $right_array[$right_index]);
        $crate::postall_helper!($model, $($rest)*);
    };
    
    // vars[i] == literal, rest...
    ($model:expr, $array:ident[$index:expr] == $value:literal, $($rest:tt)*) => {
        $crate::post!($model, $array[$index] == $value);
        $crate::postall_helper!($model, $($rest)*);
    };
    
    // ============================================================================
    // BASIC COMPARISON PATTERNS
    // ============================================================================
    
    // Multiple constraints: basic comparison patterns
    ($model:expr, $var:ident $op:tt $target:ident, $($rest:tt)*) => {
        $crate::post!($model, $var $op $target);
        $crate::postall_helper!($model, $($rest)*);
    };
    
    // Variable vs literal patterns
    ($model:expr, $var:ident $op:tt $value:literal, $($rest:tt)*) => {
        $crate::post!($model, $var $op $value);
        $crate::postall_helper!($model, $($rest)*);
    };
    
    // Single constraint (no comma)
    ($model:expr, $($constraint:tt)*) => {
        $crate::post!($model, $($constraint)*);
    };
}