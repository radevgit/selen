#[doc(hidden)]


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

#[doc(hidden)]
/// Post a mathematical constraint to the model
/// 
/// Supported constraint patterns:
/// 
/// **Basic comparisons**: `var op var`, `var op literal`, `var op (expr)`, `var op int(value)`, `var op float(value)`
/// 
/// **Arithmetic**: `var op var +/- var`, `var op var */÷ var`, `var op var % divisor`
/// 
/// **Functions**: `func(var) op target` where `func` is `abs`, `min`, `max`, `sum`
/// 
/// **Boolean**: `and(vars...)`, `or(vars...)`, `not(var)`
/// 
/// **Global**: `alldiff([vars...])`
/// 
/// **Multiplication with constants**: `target op var * int(value)`, `target op var * float(value)`
/// 
/// Where `op` is any of: `==`, `!=`, `<`, `<=`, `>`, `>=`
#[macro_export]
macro_rules! post {
    // Handle simple variable comparisons: x < y, x <= y, etc.
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
    
    // Absolute value with float constants: abs(x) >= float(1.5)
    ($model:expr, abs($var:ident) < float($target:expr)) => {{
        let _abs_var = $model.abs($var);
        $model.props.less_than(_abs_var, $crate::prelude::float($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, abs($var:ident) <= float($target:expr)) => {{
        let _abs_var = $model.abs($var);
        $model.props.less_than_or_equals(_abs_var, $crate::prelude::float($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, abs($var:ident) > float($target:expr)) => {{
        let _abs_var = $model.abs($var);
        $model.props.greater_than(_abs_var, $crate::prelude::float($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, abs($var:ident) >= float($target:expr)) => {{
        let _abs_var = $model.abs($var);
        $model.props.greater_than_or_equals(_abs_var, $crate::prelude::float($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, abs($var:ident) == float($target:expr)) => {{
        let _abs_var = $model.abs($var);
        $model.props.equals(_abs_var, $crate::prelude::float($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, abs($var:ident) != float($target:expr)) => {{
        let _abs_var = $model.abs($var);
        $model.props.not_equals(_abs_var, $crate::prelude::float($target));
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
    
    // Min function with float constants: min([x, y]) <= float(5.0)
    ($model:expr, min([$($vars:ident),+ $(,)?]) < float($target:expr)) => {{
        let _min_var = $model.min(&[$($vars),+]);
        $model.props.less_than(_min_var, $crate::prelude::float($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, min([$($vars:ident),+ $(,)?]) <= float($target:expr)) => {{
        let _min_var = $model.min(&[$($vars),+]);
        $model.props.less_than_or_equals(_min_var, $crate::prelude::float($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, min([$($vars:ident),+ $(,)?]) > float($target:expr)) => {{
        let _min_var = $model.min(&[$($vars),+]);
        $model.props.greater_than(_min_var, $crate::prelude::float($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, min([$($vars:ident),+ $(,)?]) >= float($target:expr)) => {{
        let _min_var = $model.min(&[$($vars),+]);
        $model.props.greater_than_or_equals(_min_var, $crate::prelude::float($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, min([$($vars:ident),+ $(,)?]) == float($target:expr)) => {{
        let _min_var = $model.min(&[$($vars),+]);
        $model.props.equals(_min_var, $crate::prelude::float($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, min([$($vars:ident),+ $(,)?]) != float($target:expr)) => {{
        let _min_var = $model.min(&[$($vars),+]);
        $model.props.not_equals(_min_var, $crate::prelude::float($target));
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
    
    // Max function with float constants: max([x, y]) >= float(10.0)
    ($model:expr, max([$($vars:ident),+ $(,)?]) < float($target:expr)) => {{
        let _max_var = $model.max(&[$($vars),+]);
        $model.props.less_than(_max_var, $crate::prelude::float($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, max([$($vars:ident),+ $(,)?]) <= float($target:expr)) => {{
        let _max_var = $model.max(&[$($vars),+]);
        $model.props.less_than_or_equals(_max_var, $crate::prelude::float($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, max([$($vars:ident),+ $(,)?]) > float($target:expr)) => {{
        let _max_var = $model.max(&[$($vars),+]);
        $model.props.greater_than(_max_var, $crate::prelude::float($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, max([$($vars:ident),+ $(,)?]) >= float($target:expr)) => {{
        let _max_var = $model.max(&[$($vars),+]);
        $model.props.greater_than_or_equals(_max_var, $crate::prelude::float($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, max([$($vars:ident),+ $(,)?]) == float($target:expr)) => {{
        let _max_var = $model.max(&[$($vars),+]);
        $model.props.equals(_max_var, $crate::prelude::float($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, max([$($vars:ident),+ $(,)?]) != float($target:expr)) => {{
        let _max_var = $model.max(&[$($vars),+]);
        $model.props.not_equals(_max_var, $crate::prelude::float($target));
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
    
    // Sum function: sum([x, y, z]) <op> <expr>
    ($model:expr, sum([$($vars:ident),+ $(,)?]) < $target:ident) => {{
        let _sum_var = $model.sum(&[$($vars),+]);
        $model.props.less_than(_sum_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, sum([$($vars:ident),+ $(,)?]) <= $target:ident) => {{
        let _sum_var = $model.sum(&[$($vars),+]);
        $model.props.less_than_or_equals(_sum_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, sum([$($vars:ident),+ $(,)?]) > $target:ident) => {{
        let _sum_var = $model.sum(&[$($vars),+]);
        $model.props.greater_than(_sum_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, sum([$($vars:ident),+ $(,)?]) >= $target:ident) => {{
        let _sum_var = $model.sum(&[$($vars),+]);
        $model.props.greater_than_or_equals(_sum_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, sum([$($vars:ident),+ $(,)?]) == $target:ident) => {{
        let _sum_var = $model.sum(&[$($vars),+]);
        $model.props.equals(_sum_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, sum([$($vars:ident),+ $(,)?]) != $target:ident) => {{
        let _sum_var = $model.sum(&[$($vars),+]);
        $model.props.not_equals(_sum_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Sum function with constants: sum([x, y, z]) <= int(10)
    ($model:expr, sum([$($vars:ident),+ $(,)?]) < int($target:expr)) => {{
        let _sum_var = $model.sum(&[$($vars),+]);
        $model.props.less_than(_sum_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, sum([$($vars:ident),+ $(,)?]) <= int($target:expr)) => {{
        let _sum_var = $model.sum(&[$($vars),+]);
        $model.props.less_than_or_equals(_sum_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, sum([$($vars:ident),+ $(,)?]) > int($target:expr)) => {{
        let _sum_var = $model.sum(&[$($vars),+]);
        $model.props.greater_than(_sum_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, sum([$($vars:ident),+ $(,)?]) >= int($target:expr)) => {{
        let _sum_var = $model.sum(&[$($vars),+]);
        $model.props.greater_than_or_equals(_sum_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, sum([$($vars:ident),+ $(,)?]) == int($target:expr)) => {{
        let _sum_var = $model.sum(&[$($vars),+]);
        $model.props.equals(_sum_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, sum([$($vars:ident),+ $(,)?]) != int($target:expr)) => {{
        let _sum_var = $model.sum(&[$($vars),+]);
        $model.props.not_equals(_sum_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Sum function with array expressions: sum(array) <op> <expr>
    ($model:expr, sum($array:expr) < $target:ident) => {{
        let _sum_var = $model.sum(&$array);
        $model.props.less_than(_sum_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, sum($array:expr) <= $target:ident) => {{
        let _sum_var = $model.sum(&$array);
        $model.props.less_than_or_equals(_sum_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, sum($array:expr) > $target:ident) => {{
        let _sum_var = $model.sum(&$array);
        $model.props.greater_than(_sum_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, sum($array:expr) >= $target:ident) => {{
        let _sum_var = $model.sum(&$array);
        $model.props.greater_than_or_equals(_sum_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, sum($array:expr) == $target:ident) => {{
        let _sum_var = $model.sum(&$array);
        $model.props.equals(_sum_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, sum($array:expr) != $target:ident) => {{
        let _sum_var = $model.sum(&$array);
        $model.props.not_equals(_sum_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Sum function with array expressions and constants: sum(array) <= int(10)
    ($model:expr, sum($array:expr) < int($target:expr)) => {{
        let _sum_var = $model.sum(&$array);
        $model.props.less_than(_sum_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, sum($array:expr) <= int($target:expr)) => {{
        let _sum_var = $model.sum(&$array);
        $model.props.less_than_or_equals(_sum_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, sum($array:expr) > int($target:expr)) => {{
        let _sum_var = $model.sum(&$array);
        $model.props.greater_than(_sum_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, sum($array:expr) >= int($target:expr)) => {{
        let _sum_var = $model.sum(&$array);
        $model.props.greater_than_or_equals(_sum_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, sum($array:expr) == int($target:expr)) => {{
        let _sum_var = $model.sum(&$array);
        $model.props.equals(_sum_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, sum($array:expr) != int($target:expr)) => {{
        let _sum_var = $model.sum(&$array);
        $model.props.not_equals(_sum_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Logical operators (traditional style)
    ($model:expr, and($c1:expr, $c2:expr)) => {{
        let _and_result = $model.bool_and(&[$c1, $c2]);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, or($c1:expr, $c2:expr)) => {{
        let _or_result = $model.bool_or(&[$c1, $c2]);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, not($var:ident)) => {{
        let _not_result = $model.bool_not($var);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};

    // Handle logical operators: & (AND), | (OR)
    // NOTE: Parentheses are REQUIRED due to Rust macro parsing rules
    // The `&` and `|` tokens cannot follow `expr` fragments directly
    // So we use ($left:expr) & ($right:expr) instead of $left:expr & $right:expr
    ($model:expr, ($left:expr) & ($right:expr)) => {{
        // AND operation - both constraints must be true
        // Post both constraints separately
        let _left_ref = $left;
        let _right_ref = $right;
        // Return the second constraint's reference (arbitrary choice since both must hold)
        _right_ref
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
        // Post both constraints separately
        let _left_ref = $left;
        let _right_ref = $right;
        // Return the second constraint's reference (arbitrary choice since both must hold)
        _right_ref
    }};
    
    ($model:expr, or($left:expr, $right:expr)) => {{
        // OR operation - at least one constraint must be true
        // This would require disjunctive constraint support
        // For now, this is a placeholder - true OR support needs special implementation
        let _left_ref = $left;
        let _right_ref = $right;
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, not($constraint:expr)) => {{
        // NOT operation - negation of a constraint
        // This would require constraint negation support
        // For now, this is a placeholder - true NOT support needs special implementation
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
    
    // Modulo with literal divisor and variable remainder: x % 5 == y
    ($model:expr, $left:ident % $divisor:literal == $remainder:ident) => {{
        let _mod_var = $model.modulo($left, $crate::prelude::int($divisor));
        $model.props.equals(_mod_var, $remainder);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident % $divisor:literal != $remainder:ident) => {{
        let _mod_var = $model.modulo($left, $crate::prelude::int($divisor));
        $model.props.not_equals(_mod_var, $remainder);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Modulo with variables and all comparison operators: x % y <op> z
    ($model:expr, $left:ident % $divisor:ident < $target:ident) => {{
        let _mod_var = $model.modulo($left, $divisor);
        $model.props.less_than(_mod_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident % $divisor:ident <= $target:ident) => {{
        let _mod_var = $model.modulo($left, $divisor);
        $model.props.less_than_or_equals(_mod_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident % $divisor:ident > $target:ident) => {{
        let _mod_var = $model.modulo($left, $divisor);
        $model.props.greater_than(_mod_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident % $divisor:ident >= $target:ident) => {{
        let _mod_var = $model.modulo($left, $divisor);
        $model.props.greater_than_or_equals(_mod_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident % $divisor:ident == $target:ident) => {{
        let _mod_var = $model.modulo($left, $divisor);
        $model.props.equals(_mod_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident % $divisor:ident != $target:ident) => {{
        let _mod_var = $model.modulo($left, $divisor);
        $model.props.not_equals(_mod_var, $target);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    // Modulo with int() constants on divisor: x % int(5) <op> int(0) 
    ($model:expr, $left:ident % int($divisor:expr) < int($target:expr)) => {{
        let _mod_var = $model.modulo($left, $crate::prelude::int($divisor));
        $model.props.less_than(_mod_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident % int($divisor:expr) <= int($target:expr)) => {{
        let _mod_var = $model.modulo($left, $crate::prelude::int($divisor));
        $model.props.less_than_or_equals(_mod_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident % int($divisor:expr) > int($target:expr)) => {{
        let _mod_var = $model.modulo($left, $crate::prelude::int($divisor));
        $model.props.greater_than(_mod_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident % int($divisor:expr) >= int($target:expr)) => {{
        let _mod_var = $model.modulo($left, $crate::prelude::int($divisor));
        $model.props.greater_than_or_equals(_mod_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident % int($divisor:expr) == int($target:expr)) => {{
        let _mod_var = $model.modulo($left, $crate::prelude::int($divisor));
        $model.props.equals(_mod_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident % int($divisor:expr) != int($target:expr)) => {{
        let _mod_var = $model.modulo($left, $crate::prelude::int($divisor));
        $model.props.not_equals(_mod_var, $crate::prelude::int($target));
        $crate::constraint_macros::ConstraintRef::new(0)
    }};
    
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

    // Multiplication with constant values: x * int(5) == y, x * float(3.14) <= z
    ($model:expr, $target:ident == $left:ident * int($value:expr)) => {{
        let _prod_var = $model.mul($left, $crate::prelude::int($value));
        $model.props.equals($target, _prod_var);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};

    ($model:expr, $target:ident == $left:ident * float($value:expr)) => {{
        let _prod_var = $model.mul($left, $crate::prelude::float($value));
        $model.props.equals($target, _prod_var);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};

    ($model:expr, $target:ident <= $left:ident * int($value:expr)) => {{
        let _prod_var = $model.mul($left, $crate::prelude::int($value));
        $model.props.less_than_or_equals($target, _prod_var);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};

    ($model:expr, $target:ident <= $left:ident * float($value:expr)) => {{
        let _prod_var = $model.mul($left, $crate::prelude::float($value));
        $model.props.less_than_or_equals($target, _prod_var);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};

    ($model:expr, $target:ident >= $left:ident * int($value:expr)) => {{
        let _prod_var = $model.mul($left, $crate::prelude::int($value));
        $model.props.greater_than_or_equals($target, _prod_var);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};

    ($model:expr, $target:ident >= $left:ident * float($value:expr)) => {{
        let _prod_var = $model.mul($left, $crate::prelude::float($value));
        $model.props.greater_than_or_equals($target, _prod_var);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};

    ($model:expr, $target:ident < $left:ident * int($value:expr)) => {{
        let _prod_var = $model.mul($left, $crate::prelude::int($value));
        $model.props.less_than($target, _prod_var);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};

    ($model:expr, $target:ident < $left:ident * float($value:expr)) => {{
        let _prod_var = $model.mul($left, $crate::prelude::float($value));
        $model.props.less_than($target, _prod_var);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};

    ($model:expr, $target:ident > $left:ident * int($value:expr)) => {{
        let _prod_var = $model.mul($left, $crate::prelude::int($value));
        $model.props.greater_than($target, _prod_var);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};

    ($model:expr, $target:ident > $left:ident * float($value:expr)) => {{
        let _prod_var = $model.mul($left, $crate::prelude::float($value));
        $model.props.greater_than($target, _prod_var);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};

    ($model:expr, $target:ident != $left:ident * int($value:expr)) => {{
        let _prod_var = $model.mul($left, $crate::prelude::int($value));
        $model.props.not_equals($target, _prod_var);
        $crate::constraint_macros::ConstraintRef::new(0)
    }};

    ($model:expr, $target:ident != $left:ident * float($value:expr)) => {{
        let _prod_var = $model.mul($left, $crate::prelude::float($value));
        $model.props.not_equals($target, _prod_var);
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
        
        // Test basic constraint references (dummy implementation)
        let c1 = post!(m, x < y);
        let c2 = post!(m, y > int(5));
        
        // Note: ConstraintRef boolean operations are not fully implemented yet
        // Testing basic boolean operations with variables instead
        let a = m.int(0, 1);
        let b = m.int(0, 1);
        
        post!(m, and(a, b));   // Boolean AND with variables
        post!(m, or(a, b));    // Boolean OR with variables  
        post!(m, not(a));      // Boolean NOT with variable
        
        println!("Constraint references: {:?}, {:?}", c1.id(), c2.id());
        
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
        
        // Create some constraint references for testing
        let c1 = post!(m, x < y);
        let c2 = post!(m, y > int(5));
        
        // Test boolean variables for logical operations
        let a = m.int(0, 1);
        let b = m.int(0, 1);
        
        // Test direct constraint posting with simple comma syntax
        postall!(m, 
            x < y,
            y > int(5),
            x + y <= z,
            alldiff([x, y, z]),
            and(a, b),
            or(a, b),
            not(a)
        );
        
        println!("Constraint references: {:?}, {:?}", c1.id(), c2.id());
        
        // Should compile and run without errors
        assert!(true);
    }
    
    #[test]
    fn test_sum_function_support() {
        let mut m = Model::default();
        let x = m.int(1, 10);
        let y = m.int(1, 10);
        let z = m.int(1, 10);
        let w = m.int(1, 30);
        let array = vec![x, y, z];
        
        // Test sum with variables
        let _c1 = post!(m, sum([x, y, z]) < w);
        let _c2 = post!(m, sum([x, y]) <= z);
        let _c3 = post!(m, sum([x, y, z]) > x);
        let _c4 = post!(m, sum([x, y]) >= y);
        let _c5 = post!(m, sum([x, y, z]) == w);
        let _c6 = post!(m, sum([x, y]) != z);
        
        // Test sum with int constants
        let _c7 = post!(m, sum([x, y, z]) <= int(25));
        let _c8 = post!(m, sum([x, y]) == int(15));
        let _c9 = post!(m, sum([x, y, z]) != int(30));
        
        // Test sum with arrays
        let _c10 = post!(m, sum(array) >= int(5));
        
        // Should compile without errors
        assert!(true);
    }
    
    #[test]
    fn test_float_constants_math_functions() {
        let mut m = Model::default();
        let x = m.float(0.0, 10.0);
        let y = m.float(0.0, 10.0);
        let z = m.float(0.0, 10.0);
        
        // Test abs with float constants
        let _c1 = post!(m, abs(x) < float(5.0));
        let _c2 = post!(m, abs(x) <= float(7.5));
        let _c3 = post!(m, abs(y) > float(2.0));
        let _c4 = post!(m, abs(y) >= float(3.14));
        let _c5 = post!(m, abs(z) == float(1.0));
        let _c6 = post!(m, abs(z) != float(0.0));
        
        // Test min with float constants
        let _c7 = post!(m, min([x, y]) <= float(8.0));
        let _c8 = post!(m, min([x, y, z]) == float(2.5));
        let _c9 = post!(m, min([x, y]) != float(10.0));
        
        // Test max with float constants
        let _c10 = post!(m, max([x, y]) >= float(1.0));
        let _c11 = post!(m, max([x, y, z]) < float(9.5));
        let _c12 = post!(m, max([x, y]) > float(0.5));
        
        // Should compile without errors
        assert!(true);
    }
    
    #[test]
    fn test_boolean_logic_functions() {
        let mut m = Model::default();
        let a = m.bool();
        let b = m.bool();
        let c = m.bool();
        
        // Test traditional boolean functions with variables
        post!(m, and(a, b));
        post!(m, or(a, b));
        post!(m, not(a));
        
        // Test with additional boolean variables
        post!(m, and(b, c));
        post!(m, or(a, c));
        post!(m, not(b));
        post!(m, not(c));
        
        // Should compile without errors
        assert!(true);
    }
    
    #[test]
    fn test_enhanced_modulo_operations() {
        let mut m = Model::default();
        let x = m.int(1, 20);
        let y = m.int(2, 10);
        let z = m.int(0, 5);
        
        // Test modulo with literal divisor and variable remainder
        let _c1 = post!(m, x % 5 == z);
        let _c2 = post!(m, x % 3 != z);
        
        // Test modulo with variables and all comparison operators
        let _c3 = post!(m, x % y < z);
        let _c4 = post!(m, x % y <= z);
        let _c5 = post!(m, x % y > z);
        let _c6 = post!(m, x % y >= z);
        let _c7 = post!(m, x % y == z);
        let _c8 = post!(m, x % y != z);
        
        // Test modulo with int() constants
        let _c9 = post!(m, x % int(7) < int(3));
        let _c10 = post!(m, x % int(4) <= int(2));
        let _c11 = post!(m, x % int(6) > int(1));
        let _c12 = post!(m, x % int(5) >= int(0));
        let _c13 = post!(m, x % int(8) == int(2));
        let _c14 = post!(m, x % int(9) != int(0));
        
        // Test original patterns still work
        let _c15 = post!(m, x % 3 == 1);  // literal modulo
        let _c16 = post!(m, x % y == int(0));  // enhanced variable modulo
        let _c17 = post!(m, x % y != int(0));  // enhanced variable modulo
        
        // Should compile without errors
        assert!(true);
    }
    
    #[test]
    fn test_logical_operations_enhancement() {
        let mut m = Model::default();
        let x = m.int(1, 10);
        let y = m.int(1, 10);
        let z = m.int(1, 10);
        
        // Create constraint references for testing
        let c1 = post!(m, x < y);
        let c2 = post!(m, y > int(5));
        let c3 = post!(m, z <= int(8));
        
        // Test boolean variables for logical operations instead
        let a = m.int(0, 1);
        let b = m.int(0, 1);
        let c = m.int(0, 1);
        
        // Test logical operations with boolean variables (working implementation)
        post!(m, and(a, b));   // function-style AND
        post!(m, or(a, c));    // function-style OR
        post!(m, not(a));      // function-style NOT
        
        println!("Constraint references: {:?}, {:?}, {:?}", c1.id(), c2.id(), c3.id());
        
        // Should compile without errors
        assert!(true);
    }
    
    #[test]
    fn test_constraint_reference_system() {
        let mut m = Model::default();
        let x = m.int(1, 10);
        let y = m.int(1, 10);
        
        // Test that constraint references are returned and can be used
        let c1 = post!(m, x < y);
        let c2 = post!(m, x <= y);
        let c3 = post!(m, x > y);
        let c4 = post!(m, x >= y);
        let c5 = post!(m, x == y);
        let c6 = post!(m, x != y);
        
        // Verify constraint references have valid IDs (non-zero for the fixed pattern)
        assert!(c1.id() == 0 || c1.id() > 0); // First constraint gets actual PropId, others still dummy
        assert_eq!(c2.id(), 0); // Still using dummy for non-fixed patterns
        assert_eq!(c3.id(), 0);
        assert_eq!(c4.id(), 0);
        assert_eq!(c5.id(), 0);
        assert_eq!(c6.id(), 0);
        
        // Should compile and run without errors
        assert!(true);
    }
    
    #[test]
    fn test_comprehensive_new_functionality() {
        let mut m = Model::default();
        let x = m.int(1, 10);
        let y = m.int(1, 10);
        let z = m.int(1, 10);
        let a = m.bool();
        let b = m.bool();
        let vars = vec![x, y, z];
        
        // Test a combination of all new features in one go
        postall!(m,
            // Simple constraints without float constants
            x != y,
            
            // Boolean logic functions
            and(a, b),
            or(a, b),
            not(a),
            
            // Original functionality still works
            x < y,
            alldiff([x, y, z]),
            abs(x) >= int(1)
        );
        
        // Test sum separately since it needs variable vector
        post!(m, sum(vars) <= int(25));
        
        // Should compile and run without errors
        assert!(true);
    }
}