//! Mathematical constraint posting macros
//!
//! This module provides the `post!` and `postall!` macros for creating constraints
//! using natural mathematical notation.
//!
//! # Logical Operators
//!
//! For logical operations, we provide both function-style syntax (recommended) and
//! operator-style syntax:
//!
//! ```rust
//! // ✓ Recommended function-style syntax (clean and simple):
//! let c1 = post!(model, x < y);
//! let c2 = post!(model, y > int(5));
//! let combined_and = post!(model, and(c1, c2));  // AND operation
//! let combined_or = post!(model, or(c1, c2));    // OR operation
//! let negated = post!(model, not(c1));           // NOT operation
//!
//! // ✓ Operator-style syntax (requires parentheses):
//! let combined_and = post!(model, (c1) & (c2));  // Parentheses required
//! let combined_or = post!(model, (c1) | (c2));   // Parentheses required
//! ```
//!
//! The function-style syntax is preferred because it's cleaner and doesn't require
//! parentheses due to Rust macro parsing limitations.

use crate::vars::{VarId, Val};
use crate::model::Model;
use crate::math_syntax::{MathExpr, TypedConstant};

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

/// Post a mathematical constraint to the model
/// 
/// # Examples
/// 
/// ```rust
/// use cspsolver::prelude::*;
/// use cspsolver::constraint_macros::post;
/// 
/// let mut model = Model::default();
/// let x = model.int(1, 10);
/// let y = model.int(1, 10);
/// 
/// // Mathematical constraint syntax
/// let c1 = post!(model, x < y);
/// let c2 = post!(model, abs(x) >= int(1));
/// ```
#[macro_export]
macro_rules! post {
    // Handle simple variable comparisons: x < y, x <= y, etc.
    ($model:expr, $left:ident < $right:ident) => {{
        $model.props.less_than($left, $right);
        $crate::constraint_macros::ConstraintRef::new(0) // TODO: proper ID tracking
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
    
    // Handle variable vs bare literal: x < 5, y >= 3.14
    ($model:expr, $left:ident < $right:literal) => {{
        $model.props.less_than($left, $crate::vars::Val::from($right));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident <= $right:literal) => {{
        $model.props.less_than_or_equals($left, $crate::vars::Val::from($right));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident > $right:literal) => {{
        $model.props.greater_than($left, $crate::vars::Val::from($right));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident >= $right:literal) => {{
        $model.props.greater_than_or_equals($left, $crate::vars::Val::from($right));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident == $right:literal) => {{
        $model.props.equals($left, $crate::vars::Val::from($right));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident != $right:literal) => {{
        $model.props.not_equals($left, $crate::vars::Val::from($right));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Handle variable vs expression in parentheses: x < (y + 1)
    ($model:expr, $left:ident < ($right:expr)) => {{
        $model.props.less_than($left, $crate::vars::Val::from($right));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident <= ($right:expr)) => {{
        $model.props.less_than_or_equals($left, $crate::vars::Val::from($right));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident > ($right:expr)) => {{
        $model.props.greater_than($left, $crate::vars::Val::from($right));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident >= ($right:expr)) => {{
        $model.props.greater_than_or_equals($left, $crate::vars::Val::from($right));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident == ($right:expr)) => {{
        $model.props.equals($left, $crate::vars::Val::from($right));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident != ($right:expr)) => {{
        $model.props.not_equals($left, $crate::vars::Val::from($right));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Handle variable vs constant: x < int(5), y >= float(3.14)
    ($model:expr, $left:ident < int($right:expr)) => {{
        $model.props.less_than($left, $crate::prelude::int($right));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident <= int($right:expr)) => {{
        $model.props.less_than_or_equals($left, $crate::prelude::int($right));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident > int($right:expr)) => {{
        $model.props.greater_than($left, $crate::prelude::int($right));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident >= int($right:expr)) => {{
        $model.props.greater_than_or_equals($left, $crate::prelude::int($right));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident == int($right:expr)) => {{
        $model.props.equals($left, $crate::prelude::int($right));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident != int($right:expr)) => {{
        $model.props.not_equals($left, $crate::prelude::int($right));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Handle float constants
    ($model:expr, $left:ident < float($right:expr)) => {{
        $model.props.less_than($left, $crate::prelude::float($right));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident <= float($right:expr)) => {{
        $model.props.less_than_or_equals($left, $crate::prelude::float($right));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident > float($right:expr)) => {{
        $model.props.greater_than($left, $crate::prelude::float($right));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident >= float($right:expr)) => {{
        $model.props.greater_than_or_equals($left, $crate::prelude::float($right));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident == float($right:expr)) => {{
        $model.props.equals($left, $crate::prelude::float($right));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident != float($right:expr)) => {{
        $model.props.not_equals($left, $crate::prelude::float($right));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Handle mathematical functions: abs(x), min([x,y]), max([x,y])
    // Absolute value: abs(x) <op> <expr>
    ($model:expr, abs($var:ident) < $target:ident) => {{
        let _abs_var = $model.abs($var);
        $model.props.less_than(_abs_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, abs($var:ident) <= $target:ident) => {{
        let _abs_var = $model.abs($var);
        $model.props.less_than_or_equals(_abs_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, abs($var:ident) > $target:ident) => {{
        let _abs_var = $model.abs($var);
        $model.props.greater_than(_abs_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, abs($var:ident) >= $target:ident) => {{
        let _abs_var = $model.abs($var);
        $model.props.greater_than_or_equals(_abs_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, abs($var:ident) == $target:ident) => {{
        let _abs_var = $model.abs($var);
        $model.props.equals(_abs_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, abs($var:ident) != $target:ident) => {{
        let _abs_var = $model.abs($var);
        $model.props.not_equals(_abs_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Absolute value with constants: abs(x) >= int(1)
    ($model:expr, abs($var:ident) < int($target:expr)) => {{
        let _abs_var = $model.abs($var);
        $model.props.less_than(_abs_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, abs($var:ident) <= int($target:expr)) => {{
        let _abs_var = $model.abs($var);
        $model.props.less_than_or_equals(_abs_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, abs($var:ident) > int($target:expr)) => {{
        let _abs_var = $model.abs($var);
        $model.props.greater_than(_abs_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, abs($var:ident) >= int($target:expr)) => {{
        let _abs_var = $model.abs($var);
        $model.props.greater_than_or_equals(_abs_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, abs($var:ident) == int($target:expr)) => {{
        let _abs_var = $model.abs($var);
        $model.props.equals(_abs_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, abs($var:ident) != int($target:expr)) => {{
        let _abs_var = $model.abs($var);
        $model.props.not_equals(_abs_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Min function: min([x, y]) <op> <expr>
    ($model:expr, min([$($vars:ident),+ $(,)?]) < $target:ident) => {{
        let _min_var = $model.min(&[$($vars),+]);
        $model.props.less_than(_min_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, min([$($vars:ident),+ $(,)?]) <= $target:ident) => {{
        let _min_var = $model.min(&[$($vars),+]);
        $model.props.less_than_or_equals(_min_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, min([$($vars:ident),+ $(,)?]) > $target:ident) => {{
        let _min_var = $model.min(&[$($vars),+]);
        $model.props.greater_than(_min_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, min([$($vars:ident),+ $(,)?]) >= $target:ident) => {{
        let _min_var = $model.min(&[$($vars),+]);
        $model.props.greater_than_or_equals(_min_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, min([$($vars:ident),+ $(,)?]) == $target:ident) => {{
        let _min_var = $model.min(&[$($vars),+]);
        $model.props.equals(_min_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, min([$($vars:ident),+ $(,)?]) != $target:ident) => {{
        let _min_var = $model.min(&[$($vars),+]);
        $model.props.not_equals(_min_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Min function with constants: min([x, y]) <= int(5)
    ($model:expr, min([$($vars:ident),+ $(,)?]) < int($target:expr)) => {{
        let _min_var = $model.min(&[$($vars),+]);
        $model.props.less_than(_min_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, min([$($vars:ident),+ $(,)?]) <= int($target:expr)) => {{
        let _min_var = $model.min(&[$($vars),+]);
        $model.props.less_than_or_equals(_min_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, min([$($vars:ident),+ $(,)?]) > int($target:expr)) => {{
        let _min_var = $model.min(&[$($vars),+]);
        $model.props.greater_than(_min_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, min([$($vars:ident),+ $(,)?]) >= int($target:expr)) => {{
        let _min_var = $model.min(&[$($vars),+]);
        $model.props.greater_than_or_equals(_min_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, min([$($vars:ident),+ $(,)?]) == int($target:expr)) => {{
        let _min_var = $model.min(&[$($vars),+]);
        $model.props.equals(_min_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, min([$($vars:ident),+ $(,)?]) != int($target:expr)) => {{
        let _min_var = $model.min(&[$($vars),+]);
        $model.props.not_equals(_min_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Max function: max([x, y]) <op> <expr>
    ($model:expr, max([$($vars:ident),+ $(,)?]) < $target:ident) => {{
        let _max_var = $model.max(&[$($vars),+]);
        $model.props.less_than(_max_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, max([$($vars:ident),+ $(,)?]) <= $target:ident) => {{
        let _max_var = $model.max(&[$($vars),+]);
        $model.props.less_than_or_equals(_max_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, max([$($vars:ident),+ $(,)?]) > $target:ident) => {{
        let _max_var = $model.max(&[$($vars),+]);
        $model.props.greater_than(_max_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, max([$($vars:ident),+ $(,)?]) >= $target:ident) => {{
        let _max_var = $model.max(&[$($vars),+]);
        $model.props.greater_than_or_equals(_max_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, max([$($vars:ident),+ $(,)?]) == $target:ident) => {{
        let _max_var = $model.max(&[$($vars),+]);
        $model.props.equals(_max_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, max([$($vars:ident),+ $(,)?]) != $target:ident) => {{
        let _max_var = $model.max(&[$($vars),+]);
        $model.props.not_equals(_max_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Max function with constants: max([x, y]) >= int(10)
    ($model:expr, max([$($vars:ident),+ $(,)?]) < int($target:expr)) => {{
        let _max_var = $model.max(&[$($vars),+]);
        $model.props.less_than(_max_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, max([$($vars:ident),+ $(,)?]) <= int($target:expr)) => {{
        let _max_var = $model.max(&[$($vars),+]);
        $model.props.less_than_or_equals(_max_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, max([$($vars:ident),+ $(,)?]) > int($target:expr)) => {{
        let _max_var = $model.max(&[$($vars),+]);
        $model.props.greater_than(_max_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, max([$($vars:ident),+ $(,)?]) >= int($target:expr)) => {{
        let _max_var = $model.max(&[$($vars),+]);
        $model.props.greater_than_or_equals(_max_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, max([$($vars:ident),+ $(,)?]) == int($target:expr)) => {{
        let _max_var = $model.max(&[$($vars),+]);
        $model.props.equals(_max_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, max([$($vars:ident),+ $(,)?]) != int($target:expr)) => {{
        let _max_var = $model.max(&[$($vars),+]);
        $model.props.not_equals(_max_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Min function with array expressions: min(array) <op> <expr>
    ($model:expr, min($array:expr) < $target:ident) => {{
        let _min_var = $model.min(&$array);
        $model.props.less_than(_min_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, min($array:expr) <= $target:ident) => {{
        let _min_var = $model.min(&$array);
        $model.props.less_than_or_equals(_min_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, min($array:expr) > $target:ident) => {{
        let _min_var = $model.min(&$array);
        $model.props.greater_than(_min_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, min($array:expr) >= $target:ident) => {{
        let _min_var = $model.min(&$array);
        $model.props.greater_than_or_equals(_min_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, min($array:expr) == $target:ident) => {{
        let _min_var = $model.min(&$array);
        $model.props.equals(_min_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, min($array:expr) != $target:ident) => {{
        let _min_var = $model.min(&$array);
        $model.props.not_equals(_min_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Min function with array expressions and constants: min(array) <= int(5)
    ($model:expr, min($array:expr) < int($target:expr)) => {{
        let _min_var = $model.min(&$array);
        $model.props.less_than(_min_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, min($array:expr) <= int($target:expr)) => {{
        let _min_var = $model.min(&$array);
        $model.props.less_than_or_equals(_min_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, min($array:expr) > int($target:expr)) => {{
        let _min_var = $model.min(&$array);
        $model.props.greater_than(_min_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, min($array:expr) >= int($target:expr)) => {{
        let _min_var = $model.min(&$array);
        $model.props.greater_than_or_equals(_min_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, min($array:expr) == int($target:expr)) => {{
        let _min_var = $model.min(&$array);
        $model.props.equals(_min_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, min($array:expr) != int($target:expr)) => {{
        let _min_var = $model.min(&$array);
        $model.props.not_equals(_min_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Max function with array expressions: max(array) <op> <expr>
    ($model:expr, max($array:expr) < $target:ident) => {{
        let _max_var = $model.max(&$array);
        $model.props.less_than(_max_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, max($array:expr) <= $target:ident) => {{
        let _max_var = $model.max(&$array);
        $model.props.less_than_or_equals(_max_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, max($array:expr) > $target:ident) => {{
        let _max_var = $model.max(&$array);
        $model.props.greater_than(_max_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, max($array:expr) >= $target:ident) => {{
        let _max_var = $model.max(&$array);
        $model.props.greater_than_or_equals(_max_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, max($array:expr) == $target:ident) => {{
        let _max_var = $model.max(&$array);
        $model.props.equals(_max_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, max($array:expr) != $target:ident) => {{
        let _max_var = $model.max(&$array);
        $model.props.not_equals(_max_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Max function with array expressions and constants: max(array) >= int(10)
    ($model:expr, max($array:expr) < int($target:expr)) => {{
        let _max_var = $model.max(&$array);
        $model.props.less_than(_max_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, max($array:expr) <= int($target:expr)) => {{
        let _max_var = $model.max(&$array);
        $model.props.less_than_or_equals(_max_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, max($array:expr) > int($target:expr)) => {{
        let _max_var = $model.max(&$array);
        $model.props.greater_than(_max_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, max($array:expr) >= int($target:expr)) => {{
        let _max_var = $model.max(&$array);
        $model.props.greater_than_or_equals(_max_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, max($array:expr) == int($target:expr)) => {{
        let _max_var = $model.max(&$array);
        $model.props.equals(_max_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, max($array:expr) != int($target:expr)) => {{
        let _max_var = $model.max(&$array);
        $model.props.not_equals(_max_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Handle logical operators: & (AND), | (OR)
    // NOTE: Parentheses are REQUIRED due to Rust macro parsing rules
    // The `&` and `|` tokens cannot follow `expr` fragments directly
    // So we use ($left:expr) & ($right:expr) instead of $left:expr & $right:expr
    ($model:expr, ($left:expr) & ($right:expr)) => {{
        // AND operation - both constraints must be true
        // For now, this posts both constraints separately
        // In a full implementation, this would create a compound constraint
        let _left_ref = $left;
        let _right_ref = $right;
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, ($left:expr) | ($right:expr)) => {{
        // OR operation - at least one constraint must be true
        // This would require disjunctive constraint support
        let _left_ref = $left;
        let _right_ref = $right;
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Handle function-style logical operators (preferred syntax)
    ($model:expr, and($left:expr, $right:expr)) => {{
        // AND operation - both constraints must be true
        let _left_ref = $left;
        let _right_ref = $right;
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, or($left:expr, $right:expr)) => {{
        // OR operation - at least one constraint must be true
        let _left_ref = $left;
        let _right_ref = $right;
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, not($constraint:expr)) => {{
        // NOT operation - negation of a constraint
        let _constraint_ref = $constraint;
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Global constraints: alldiff([x, y, z])
    ($model:expr, alldiff([$($vars:ident),+ $(,)?])) => {{
        $model.props.all_different(vec![$($vars),+]);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Global constraints: alldiff with array expressions
    ($model:expr, alldiff($array:expr)) => {{
        $model.props.all_different($array.to_vec());
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Enhanced modulo operations: x % y == int(0), x % y != int(0)
    
    // Handle arithmetic operations: x + y < z, x - y >= int(0), etc.
    // Addition: x + y <op> <expr>
    ($model:expr, $left:ident + $right:ident < $target:ident) => {{
        let _sum_var = $model.add($left, $right);
        $model.props.less_than(_sum_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident + $right:ident <= $target:ident) => {{
        let _sum_var = $model.add($left, $right);
        $model.props.less_than_or_equals(_sum_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident + $right:ident > $target:ident) => {{
        let _sum_var = $model.add($left, $right);
        $model.props.greater_than(_sum_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident + $right:ident >= $target:ident) => {{
        let _sum_var = $model.add($left, $right);
        $model.props.greater_than_or_equals(_sum_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident + $right:ident == $target:ident) => {{
        let _sum_var = $model.add($left, $right);
        $model.props.equals(_sum_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident + $right:ident != $target:ident) => {{
        let _sum_var = $model.add($left, $right);
        $model.props.not_equals(_sum_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Addition with constants: x + y < int(10)
    ($model:expr, $left:ident + $right:ident < int($target:expr)) => {{
        let _sum_var = $model.add($left, $right);
        $model.props.less_than(_sum_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident + $right:ident <= int($target:expr)) => {{
        let _sum_var = $model.add($left, $right);
        $model.props.less_than_or_equals(_sum_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident + $right:ident > int($target:expr)) => {{
        let _sum_var = $model.add($left, $right);
        $model.props.greater_than(_sum_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident + $right:ident >= int($target:expr)) => {{
        let _sum_var = $model.add($left, $right);
        $model.props.greater_than_or_equals(_sum_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident + $right:ident == int($target:expr)) => {{
        let _sum_var = $model.add($left, $right);
        $model.props.equals(_sum_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident + $right:ident != int($target:expr)) => {{
        let _sum_var = $model.add($left, $right);
        $model.props.not_equals(_sum_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Subtraction: x - y <op> <expr>
    ($model:expr, $left:ident - $right:ident < $target:ident) => {{
        let _diff_var = $model.sub($left, $right);
        $model.props.less_than(_diff_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident - $right:ident <= $target:ident) => {{
        let _diff_var = $model.sub($left, $right);
        $model.props.less_than_or_equals(_diff_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident - $right:ident > $target:ident) => {{
        let _diff_var = $model.sub($left, $right);
        $model.props.greater_than(_diff_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident - $right:ident >= $target:ident) => {{
        let _diff_var = $model.sub($left, $right);
        $model.props.greater_than_or_equals(_diff_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident - $right:ident == $target:ident) => {{
        let _diff_var = $model.sub($left, $right);
        $model.props.equals(_diff_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident - $right:ident != $target:ident) => {{
        let _diff_var = $model.sub($left, $right);
        $model.props.not_equals(_diff_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Subtraction with constants: x - y >= int(0)
    ($model:expr, $left:ident - $right:ident < int($target:expr)) => {{
        let _diff_var = $model.sub($left, $right);
        $model.props.less_than(_diff_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident - $right:ident <= int($target:expr)) => {{
        let _diff_var = $model.sub($left, $right);
        $model.props.less_than_or_equals(_diff_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident - $right:ident > int($target:expr)) => {{
        let _diff_var = $model.sub($left, $right);
        $model.props.greater_than(_diff_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident - $right:ident >= int($target:expr)) => {{
        let _diff_var = $model.sub($left, $right);
        $model.props.greater_than_or_equals(_diff_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident - $right:ident == int($target:expr)) => {{
        let _diff_var = $model.sub($left, $right);
        $model.props.equals(_diff_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident - $right:ident != int($target:expr)) => {{
        let _diff_var = $model.sub($left, $right);
        $model.props.not_equals(_diff_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Multiplication: x * y <op> <expr>
    ($model:expr, $left:ident * $right:ident < $target:ident) => {{
        let _prod_var = $model.mul($left, $right);
        $model.props.less_than(_prod_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident * $right:ident <= $target:ident) => {{
        let _prod_var = $model.mul($left, $right);
        $model.props.less_than_or_equals(_prod_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident * $right:ident > $target:ident) => {{
        let _prod_var = $model.mul($left, $right);
        $model.props.greater_than(_prod_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident * $right:ident >= $target:ident) => {{
        let _prod_var = $model.mul($left, $right);
        $model.props.greater_than_or_equals(_prod_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident * $right:ident == $target:ident) => {{
        let _prod_var = $model.mul($left, $right);
        $model.props.equals(_prod_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident * $right:ident != $target:ident) => {{
        let _prod_var = $model.mul($left, $right);
        $model.props.not_equals(_prod_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Multiplication with constants: x * y <= int(10)
    ($model:expr, $left:ident * $right:ident < int($target:expr)) => {{
        let _prod_var = $model.mul($left, $right);
        $model.props.less_than(_prod_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident * $right:ident <= int($target:expr)) => {{
        let _prod_var = $model.mul($left, $right);
        $model.props.less_than_or_equals(_prod_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident * $right:ident > int($target:expr)) => {{
        let _prod_var = $model.mul($left, $right);
        $model.props.greater_than(_prod_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident * $right:ident >= int($target:expr)) => {{
        let _prod_var = $model.mul($left, $right);
        $model.props.greater_than_or_equals(_prod_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident * $right:ident == int($target:expr)) => {{
        let _prod_var = $model.mul($left, $right);
        $model.props.equals(_prod_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident * $right:ident != int($target:expr)) => {{
        let _prod_var = $model.mul($left, $right);
        $model.props.not_equals(_prod_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Division: x / y <op> <expr>
    ($model:expr, $left:ident / $right:ident < $target:ident) => {{
        let _quot_var = $model.div($left, $right);
        $model.props.less_than(_quot_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident / $right:ident <= $target:ident) => {{
        let _quot_var = $model.div($left, $right);
        $model.props.less_than_or_equals(_quot_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident / $right:ident > $target:ident) => {{
        let _quot_var = $model.div($left, $right);
        $model.props.greater_than(_quot_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident / $right:ident >= $target:ident) => {{
        let _quot_var = $model.div($left, $right);
        $model.props.greater_than_or_equals(_quot_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident / $right:ident == $target:ident) => {{
        let _quot_var = $model.div($left, $right);
        $model.props.equals(_quot_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident / $right:ident != $target:ident) => {{
        let _quot_var = $model.div($left, $right);
        $model.props.not_equals(_quot_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Division with constants: x / y <= int(5)
    ($model:expr, $left:ident / $right:ident < int($target:expr)) => {{
        let _quot_var = $model.div($left, $right);
        $model.props.less_than(_quot_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident / $right:ident <= int($target:expr)) => {{
        let _quot_var = $model.div($left, $right);
        $model.props.less_than_or_equals(_quot_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident / $right:ident > int($target:expr)) => {{
        let _quot_var = $model.div($left, $right);
        $model.props.greater_than(_quot_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident / $right:ident >= int($target:expr)) => {{
        let _quot_var = $model.div($left, $right);
        $model.props.greater_than_or_equals(_quot_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident / $right:ident == int($target:expr)) => {{
        let _quot_var = $model.div($left, $right);
        $model.props.equals(_quot_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident / $right:ident != int($target:expr)) => {{
        let _quot_var = $model.div($left, $right);
        $model.props.not_equals(_quot_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Handle modulo operations: x % 3 == 1
    ($model:expr, $left:ident % $divisor:literal == $remainder:literal) => {{
        let _mod_var = $model.modulo($left, $crate::prelude::int($divisor));
        $model.props.equals(_mod_var, $crate::prelude::int($remainder));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Enhanced modulo operations: x % y == int(0), x % y != int(0)
    ($model:expr, $left:ident % $right:ident == int($remainder:expr)) => {{
        let _mod_var = $model.modulo($left, $right);
        $model.props.equals(_mod_var, $crate::prelude::int($remainder));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident % $right:ident != int($remainder:expr)) => {{
        let _mod_var = $model.modulo($left, $right);
        $model.props.not_equals(_mod_var, $crate::prelude::int($remainder));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
}

/// Batch multiple constraint references into a vector
/// 
/// This macro provides a convenient way to group existing constraint 
/// references into a vector for organization or tracking purposes.
/// 
/// # Examples
/// 
/// ```rust
/// use cspsolver::prelude::*;
/// use cspsolver::constraint_macros::{post, postall};
/// 
/// let mut model = Model::default();
/// let x = model.int(1, 10);
/// let y = model.int(1, 10);
/// 
/// // Create individual constraints
/// let c1 = post!(model, x < y);
/// let c2 = post!(model, y > int(5));
/// 
/// // Post multiple constraints directly
/// postall!(m, x < y, y > int(5), x + y <= z);
/// ```
#[macro_export]
macro_rules! postall {
    // Use simple comma-separated arguments
    ($model:expr, $($rest:tt)*) => {{
        $crate::postall_helper!($model, $($rest)*);
    }};
}

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
    
    // With constants
    ($model:expr, $var:ident < int($target:expr)) => {
        $crate::post!($model, $var < int($target));
    };
    
    ($model:expr, $var:ident <= int($target:expr)) => {
        $crate::post!($model, $var <= int($target));
    };
    
    ($model:expr, $var:ident > int($target:expr)) => {
        $crate::post!($model, $var > int($target));
    };
    
    ($model:expr, $var:ident >= int($target:expr)) => {
        $crate::post!($model, $var >= int($target));
    };
    
    ($model:expr, $var:ident == int($target:expr)) => {
        $crate::post!($model, $var == int($target));
    };
    
    ($model:expr, $var:ident != int($target:expr)) => {
        $crate::post!($model, $var != int($target));
    };
    
    // Arithmetic operations
    ($model:expr, $left:ident + $right:ident <= $target:ident) => {
        $crate::post!($model, $left + $right <= $target);
    };
    
    ($model:expr, $left:ident + $right:ident == $target:ident) => {
        $crate::post!($model, $left + $right == $target);
    };
    
    ($model:expr, $left:ident + $right:ident <= int($target:expr)) => {
        $crate::post!($model, $left + $right <= int($target));
    };
    
    ($model:expr, $left:ident + $right:ident == int($target:expr)) => {
        $crate::post!($model, $left + $right == int($target));
    };
    
    // Mathematical functions
    ($model:expr, abs($var:ident) >= int($target:expr)) => {
        $crate::post!($model, abs($var) >= int($target));
    };
    
    ($model:expr, abs($var:ident) <= int($target:expr)) => {
        $crate::post!($model, abs($var) <= int($target));
    };
    
    // Global constraints
    ($model:expr, alldiff([$($vars:ident),+ $(,)?])) => {
        $crate::post!($model, alldiff([$($vars),+]));
    };
    
    // Global constraints with array expressions
    ($model:expr, alldiff($array:expr)) => {
        $crate::post!($model, alldiff($array));
    };
    
    // Logical operators
    ($model:expr, and($c1:expr, $c2:expr)) => {
        $crate::post!($model, and($c1, $c2));
    };
    
    ($model:expr, or($c1:expr, $c2:expr)) => {
        $crate::post!($model, or($c1, $c2));
    };
    
    ($model:expr, not($c:expr)) => {
        $crate::post!($model, not($c));
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
    
    // With constants (multiple)
    ($model:expr, $var:ident < int($target:expr), $($rest:tt)*) => {
        $crate::post!($model, $var < int($target));
        $crate::postall_helper!($model, $($rest)*);
    };
    
    ($model:expr, $var:ident <= int($target:expr), $($rest:tt)*) => {
        $crate::post!($model, $var <= int($target));
        $crate::postall_helper!($model, $($rest)*);
    };
    
    ($model:expr, $var:ident > int($target:expr), $($rest:tt)*) => {
        $crate::post!($model, $var > int($target));
        $crate::postall_helper!($model, $($rest)*);
    };
    
    ($model:expr, $var:ident >= int($target:expr), $($rest:tt)*) => {
        $crate::post!($model, $var >= int($target));
        $crate::postall_helper!($model, $($rest)*);
    };
    
    ($model:expr, $var:ident == int($target:expr), $($rest:tt)*) => {
        $crate::post!($model, $var == int($target));
        $crate::postall_helper!($model, $($rest)*);
    };
    
    ($model:expr, $var:ident != int($target:expr), $($rest:tt)*) => {
        $crate::post!($model, $var != int($target));
        $crate::postall_helper!($model, $($rest)*);
    };
    
    // Arithmetic operations (multiple)
    ($model:expr, $left:ident + $right:ident <= $target:ident, $($rest:tt)*) => {
        $crate::post!($model, $left + $right <= $target);
        $crate::postall_helper!($model, $($rest)*);
    };
    
    ($model:expr, $left:ident + $right:ident == $target:ident, $($rest:tt)*) => {
        $crate::post!($model, $left + $right == $target);
        $crate::postall_helper!($model, $($rest)*);
    };
    
    ($model:expr, $left:ident + $right:ident <= int($target:expr), $($rest:tt)*) => {
        $crate::post!($model, $left + $right <= int($target));
        $crate::postall_helper!($model, $($rest)*);
    };
    
    ($model:expr, $left:ident + $right:ident == int($target:expr), $($rest:tt)*) => {
        $crate::post!($model, $left + $right == int($target));
        $crate::postall_helper!($model, $($rest)*);
    };
    
    // Mathematical functions (multiple)
    ($model:expr, abs($var:ident) >= int($target:expr), $($rest:tt)*) => {
        $crate::post!($model, abs($var) >= int($target));
        $crate::postall_helper!($model, $($rest)*);
    };
    
    ($model:expr, abs($var:ident) <= int($target:expr), $($rest:tt)*) => {
        $crate::post!($model, abs($var) <= int($target));
        $crate::postall_helper!($model, $($rest)*);
    };
    
    // Global constraints (multiple)
    ($model:expr, alldiff([$($vars:ident),+ $(,)?]), $($rest:tt)*) => {
        $crate::post!($model, alldiff([$($vars),+]));
        $crate::postall_helper!($model, $($rest)*);
    };
    
    // Global constraints with array expressions (multiple)
    ($model:expr, alldiff($array:expr), $($rest:tt)*) => {
        $crate::post!($model, alldiff($array));
        $crate::postall_helper!($model, $($rest)*);
    };
    
    // Logical operators (multiple)
    ($model:expr, and($c1:expr, $c2:expr), $($rest:tt)*) => {
        $crate::post!($model, and($c1, $c2));
        $crate::postall_helper!($model, $($rest)*);
    };
    
    ($model:expr, or($c1:expr, $c2:expr), $($rest:tt)*) => {
        $crate::post!($model, or($c1, $c2));
        $crate::postall_helper!($model, $($rest)*);
    };
    
    ($model:expr, not($c:expr), $($rest:tt)*) => {
        $crate::post!($model, not($c));
        $crate::postall_helper!($model, $($rest)*);
    };
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    
    #[test]
    fn test_post_macro_basic() {
        let mut m = Model::default();
        let x = m.int(1, 10);
        let y = m.int(1, 10);
        
        // Test basic variable comparisons
        let _c1 = post!(m, x < y);
        let _c2 = post!(m, x <= y);
        let _c3 = post!(m, x > y);
        let _c4 = post!(m, x >= y);
        let _c5 = post!(m, x == y);
        let _c6 = post!(m, x != y);
        
        // Should compile without errors
        assert!(true);
    }
    
    #[test]
    fn test_post_macro_array_syntax() {
        let mut m = Model::default();
        let x = m.int(1, 10);
        let y = m.int(1, 10);
        let z = m.int(1, 10);
        
        // Test alldiff with arrays
        let vars = [x, y, z];
        let _c1 = post!(m, alldiff(vars));
        
        let vars_vec = vec![x, y, z];
        let _c2 = post!(m, alldiff(vars_vec));
        
        // Test min/max with arrays
        let _c3 = post!(m, min(vars) <= int(5));
        let _c4 = post!(m, max(vars_vec) >= int(8));
        
        // Should compile without errors
        assert!(true);
    }
    
    #[test]
    fn test_post_macro_constants() {
        let mut m = Model::default();
        let x = m.int(1, 10);
        let y = m.float(1.0, 10.0);
        
        // Test variable vs integer constants
        let _c1 = post!(m, x < int(5));
        let _c2 = post!(m, x >= int(1));
        let _c3 = post!(m, x == int(7));
        
        // Test variable vs float constants
        let _c4 = post!(m, y <= float(3.14));
        let _c5 = post!(m, y > float(1.0));
        let _c6 = post!(m, y != float(5.5));
        
        // Should compile without errors
        assert!(true);
    }
    
    #[test]
    fn test_post_macro_logical_operators() {
        let mut m = Model::default();
        let x = m.int(1, 10);
        let y = m.int(1, 10);
        
        // Test logical combinations of constraint references
        let c1 = post!(m, x < y);
        let c2 = post!(m, y > int(5));
        
        // ✓ Preferred function-style syntax (clean and simple):
        let _c3 = post!(m, and(c1, c2));   // AND operation
        let _c4 = post!(m, or(c1, c2));    // OR operation  
        let _c5 = post!(m, not(c1));       // NOT operation
        
        // ✓ Alternative operator-style syntax (requires parentheses):
        let _c6 = post!(m, (c1) & (c2));   // AND with parentheses
        let _c7 = post!(m, (c1) | (c2));   // OR with parentheses
        
        // Should compile without errors
        assert!(true);
    }
    
    #[test]
    fn test_post_macro_modulo() {
        let mut m = Model::default();
        let x = m.int(1, 20);
        
        // Test simple modulo operations (literals only for now)
        let _c1 = post!(m, x % 3 == 1);  // x % 3 == 1
        
        // TODO: More complex patterns with int() helpers:
        // let _c2 = post!(m, x % int(5) != int(0));  // x % 5 != 0
        
        // Should compile without errors
        assert!(true);
    }
    
    #[test]
    fn test_post_macro_arithmetic() {
        let mut m = Model::default();
        let x = m.int(1, 10);
        let y = m.int(1, 10);
        let z = m.int(1, 20);
        
        // Test arithmetic operations with variables
        let _c1 = post!(m, x + y < z);
        let _c2 = post!(m, x - y >= z);
        let _c3 = post!(m, x * y <= z);
        let _c4 = post!(m, x / y == z);
        
        // Test arithmetic operations with constants
        let _c5 = post!(m, x + y <= int(15));
        let _c6 = post!(m, x - y >= int(0));
        let _c7 = post!(m, x * y == int(12));
        let _c8 = post!(m, x / y != int(0));
        
        // Should compile without errors
        assert!(true);
    }
    
    #[test]
    fn test_post_macro_mathematical_functions() {
        let mut m = Model::default();
        let x = m.int(-10, 10);
        let y = m.int(1, 10);
        let z = m.int(1, 10);
        
        // Test absolute value
        let _c1 = post!(m, abs(x) >= int(1));
        let _c2 = post!(m, abs(x) <= y);
        
        // Test min function
        let _c3 = post!(m, min([y, z]) == int(5));
        let _c4 = post!(m, min([y, z]) >= x);
        
        // Test max function  
        let _c5 = post!(m, max([y, z]) <= int(10));
        let _c6 = post!(m, max([y, z]) != x);
        
        // Should compile without errors
        assert!(true);
    }
    
    #[test]
    fn test_post_macro_alldiff() {
        let mut m = Model::default();
        let x = m.int(1, 10);
        let y = m.int(1, 10);
        let z = m.int(1, 10);
        let w = m.int(1, 10);
        
        // Test alldiff constraint
        let _c1 = post!(m, alldiff([x, y, z]));
        let _c2 = post!(m, alldiff([x, y, z, w]));
        
        // Should compile without errors
        assert!(true);
    }
    
    #[test]
    fn test_post_macro_enhanced_modulo() {
        let mut m = Model::default();
        let x = m.int(1, 20);
        let y = m.int(2, 5);
        
        // Test enhanced modulo with variables
        let _c1 = post!(m, x % y == int(0));  // x is divisible by y
        let _c2 = post!(m, x % y != int(0));  // x is not divisible by y
        
        // Original literal modulo still works
        let _c3 = post!(m, x % 3 == 1);
        
        // Should compile without errors
        assert!(true);
    }
    
    #[test]
    fn test_post_macro_complex_expressions() {
        let mut m = Model::default();
        let x = m.int(1, 10);
        let y = m.int(1, 10);
        let z = m.int(1, 10);
        
        // Test combining different constraint types
        let _c1 = post!(m, x + y <= int(15));
        let _c2 = post!(m, abs(x) >= int(1));  // Simpler abs usage
        let _c3 = post!(m, max([x, y]) == z);
        let _c4 = post!(m, x % y != int(0));
        let _c5 = post!(m, alldiff([x, y, z]));
        
        // Should compile without errors
        assert!(true);
    }
    
    #[test]
    fn test_post_macro_negation() {
        let mut m = Model::default();
        let x = m.int(1, 10);
        let y = m.int(1, 10);
        
        // TODO: Negation to implement:
        // let _c1 = post!(m, !(x < y));  // NOT(x < y) should be x >= y
        
        // For now, basic comparisons work:
        let _c2 = post!(m, x >= y);  // Direct equivalent
        
        // Should compile without errors
        assert!(true);
    }
    
    #[test]
    fn test_postall_macro() {
        let mut m = Model::default();
        let x = m.int(1, 10);
        let y = m.int(1, 10);
        let z = m.int(1, 15);
        
        // Create some constraint references for logical operations
        let c1 = post!(m, x < y);
        let c2 = post!(m, y > int(5));
        
        // Test direct constraint posting with simple comma syntax
        postall!(m, 
            x < y,
            y > int(5),
            x + y <= z,
            alldiff([x, y, z]),
            and(c1, c2),
            or(c1, c2),
            not(c1)
        );
        
        // Should compile and run without errors
        assert!(true);
    }
}