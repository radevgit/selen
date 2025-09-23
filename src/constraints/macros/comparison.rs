//! Comparison constraint macros for the Selen CSP solver.

#[doc(hidden)]
#[macro_export]
macro_rules! post_comparison {
    // ============================================================================
    // ARRAY ELEMENT TO LITERAL COMPARISONS
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
    
    // ============================================================================
    // ARRAY ELEMENT TO ARRAY ELEMENT COMPARISONS
    // ============================================================================
    
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
    
    // ============================================================================
    // ARRAY ELEMENT TO VARIABLE COMPARISONS
    // ============================================================================
    
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
    
    // ============================================================================
    // VARIABLE TO ARRAY ELEMENT COMPARISONS
    // ============================================================================
    
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
    
    // ============================================================================
    // VARIABLE TO LITERAL COMPARISONS
    // ============================================================================
    
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
    
    // ============================================================================
    // VARIABLE TO VARIABLE COMPARISONS
    // ============================================================================
    
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
}
