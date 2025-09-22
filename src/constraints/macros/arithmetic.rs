#[macro_export]
macro_rules! post_arithmetic {
    // ============================================================================
    // ADDITION PATTERNS
    // ============================================================================
    
    // x + y == z
    ($model:expr, $left:ident + $right:ident == $target:ident) => {{
        let _sum_var = $model.add($left, $right);
        $model.props.equals(_sum_var, $target);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x + y == int(N)
    ($model:expr, $left:ident + $right:ident == int($target:expr)) => {{
        let _sum_var = $model.add($left, $right);
        let _target_var = $model.int($target, $target);
        $model.props.equals(_sum_var, _target_var);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x + y >= z
    ($model:expr, $left:ident + $right:ident >= $target:ident) => {{
        let _sum_var = $model.add($left, $right);
        $model.props.greater_than_or_equals(_sum_var, $target);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x + y <= z
    ($model:expr, $left:ident + $right:ident <= $target:ident) => {{
        let _sum_var = $model.add($left, $right);
        $model.props.less_than_or_equals(_sum_var, $target);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x + y <= int(N)
    ($model:expr, $left:ident + $right:ident <= int($target:expr)) => {{
        let _sum_var = $model.add($left, $right);
        let _target_var = $model.int($target, $target);
        $model.props.less_than_or_equals(_sum_var, _target_var);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // Array addition: vars[i] + vars[j] == target
    ($model:expr, $left_array:ident[$left_index:expr] + $right_array:ident[$right_index:expr] == $target:ident) => {{
        let _sum_var = $model.add($left_array[$left_index], $right_array[$right_index]);
        $model.props.equals(_sum_var, $target);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // ============================================================================
    // MULTIPLICATION PATTERNS
    // ============================================================================
    
    // x * y == z
    ($model:expr, $left:ident * $right:ident == $target:ident) => {{
        let _prod_var = $model.mul($left, $right);
        $model.props.equals(_prod_var, $target);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x * int(N) == result
    ($model:expr, $left:ident * int($value:expr) == $target:ident) => {{
        let _constant_var = $model.int($value, $value);
        let _prod_var = $model.mul($left, _constant_var);
        $model.props.equals(_prod_var, $target);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x * float(N) == result
    ($model:expr, $left:ident * float($value:expr) == $target:ident) => {{
        let _constant_var = $model.float($value, $value);
        let _prod_var = $model.mul($left, _constant_var);
        $model.props.equals(_prod_var, $target);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // result == x * int(N)
    ($model:expr, $target:ident == $left:ident * int($value:expr)) => {{
        let _constant_var = $model.int($value, $value);
        let _prod_var = $model.mul($left, _constant_var);
        $model.props.equals($target, _prod_var);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // result == x * float(N)
    ($model:expr, $target:ident == $left:ident * float($value:expr)) => {{
        let _constant_var = $model.float($value, $value);
        let _prod_var = $model.mul($left, _constant_var);
        $model.props.equals($target, _prod_var);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // budget >= x * int(N)
    ($model:expr, $budget:ident >= $left:ident * int($value:expr)) => {{
        let _constant_var = $model.int($value, $value);
        let _prod_var = $model.mul($left, _constant_var);
        $model.props.greater_than_or_equals($budget, _prod_var);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // ============================================================================
    // DIVISION PATTERNS  
    // ============================================================================
    
    // x / y == result
    ($model:expr, $left:ident / $right:ident == $target:ident) => {{
        let _div_var = $model.div($left, $right);
        $model.props.equals(_div_var, $target);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // ============================================================================
    // ABSOLUTE VALUE PATTERNS
    // ============================================================================
    
    // abs(x) >= int(N)
    ($model:expr, abs($var:ident) >= int($target:expr)) => {{
        let _abs_var = $model.abs($var);
        let _target_var = $model.int($target, $target);
        $model.props.greater_than_or_equals(_abs_var, _target_var);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // abs(x) == target
    ($model:expr, abs($var:ident) == $target:ident) => {{
        let _abs_var = $model.abs($var);
        $model.props.equals(_abs_var, $target);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // ============================================================================
    // SUM PATTERNS
    // ============================================================================
    
    // sum([vars]) == int(N)
    ($model:expr, sum([$($vars:ident),+ $(,)?]) == int($target:expr)) => {{
        let _sum_var = $model.sum(&[$($vars),+]);
        let _target_var = $model.int($target, $target);
        $model.props.equals(_sum_var, _target_var);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // sum([vars]) == target
    ($model:expr, sum([$($vars:ident),+ $(,)?]) == $target:ident) => {{
        let _sum_var = $model.sum(&[$($vars),+]);
        $model.props.equals(_sum_var, $target);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // sum(expr.clone()) == target
    ($model:expr, sum($expr:expr) == $target:ident) => {{
        let _sum_var = $model.sum(&$expr);
        $model.props.equals(_sum_var, $target);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
}
