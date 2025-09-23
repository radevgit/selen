#[doc(hidden)]
#[macro_export]
macro_rules! post_global {
        // ============================================================================
    // ALLDIFF PATTERNS
    // ============================================================================
    
    // alldiff([x, y, z])
    ($model:expr, alldiff([$($vars:ident),+ $(,)?])) => {{
        let vars_vec = [$($vars),+].to_vec();
        $model.props.all_different(vars_vec);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // alldiff(vec![x, y, z])
    ($model:expr, alldiff(vec![$($vars:ident),+ $(,)?])) => {{
        let vars_vec = vec![$($vars),+];
        $model.props.all_different(vars_vec);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // alldiff(array) - for direct array types like [VarId; N]
    ($model:expr, alldiff($expr:expr)) => {{
        $model.props.all_different($expr.to_vec());
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // ============================================================================
    // ALLEQUAL PATTERNS
    // ============================================================================
    
    // allequal([x, y, z])
    ($model:expr, allequal([$($vars:ident),+ $(,)?])) => {{
        let vars_vec = [$($vars),+].to_vec();
        $model.props.all_equal(vars_vec);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // ============================================================================
    // ELEMENT PATTERNS
    // ============================================================================
    
    // element([array], index, value)
    ($model:expr, element([$($array:ident),+ $(,)?], $index:ident, $value:ident)) => {{
        let array_vec = [$($array),+].to_vec();
        $model.props.element(array_vec, $index, $value);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // element(expr, index, value)
    ($model:expr, element($array:expr, $index:ident, $value:ident)) => {{
        $model.props.element($array, $index, $value);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // ============================================================================
    // MIN/MAX PATTERNS
    // ============================================================================
    
    // min([vars]) == target
    ($model:expr, min([$($vars:ident),+ $(,)?]) == $target:ident) => {{
        let vars_vec = [$($vars),+].to_vec();
        let _min_var = $model.min(&vars_vec).unwrap();
        $model.props.equals(_min_var, $target);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // max([vars]) == target
    ($model:expr, max([$($vars:ident),+ $(,)?]) == $target:ident) => {{
        let vars_vec = [$($vars),+].to_vec();
        let _max_var = $model.max(&vars_vec).unwrap();
        $model.props.equals(_max_var, $target);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // min(expr) == target
    ($model:expr, min($expr:expr) == $target:ident) => {{
        let _min_var = $model.min(&$expr).unwrap();
        $model.props.equals(_min_var, $target);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // max(expr) == target
    ($model:expr, max($expr:expr) == $target:ident) => {{
        let _max_var = $model.max(&$expr).unwrap();
        $model.props.equals(_max_var, $target);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
}
