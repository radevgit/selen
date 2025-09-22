#[macro_export]
macro_rules! post_logical {
    // ============================================================================
    // LOGICAL OPERATION PATTERNS
    // ============================================================================
    
    // and([vars])
    ($model:expr, and([$($vars:expr),* $(,)?])) => {{
        let vars_vec = [$($vars),*].to_vec();
        let and_result = $model.bool_and(&vars_vec);
        $model.new(and_result.eq(1));  // Constrain the result to be true
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // and(var1, var2, ...)
    ($model:expr, and($($vars:expr),* $(,)?)) => {{
        let vars_vec = [$($vars),*].to_vec();
        let and_result = $model.bool_and(&vars_vec);
        $model.new(and_result.eq(1));  // Constrain the result to be true
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // or([vars])
    ($model:expr, or([$($vars:expr),* $(,)?])) => {{
        let vars_vec = [$($vars),*].to_vec();
        let or_result = $model.bool_or(&vars_vec);
        $model.new(or_result.eq(1));  // Constrain the result to be true
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // or(var1, var2, ...)
    ($model:expr, or($($vars:expr),* $(,)?)) => {{
        let vars_vec = [$($vars),*].to_vec();
        let or_result = $model.bool_or(&vars_vec);
        $model.new(or_result.eq(1));  // Constrain the result to be true
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // not([vars]) - apply not to each variable individually
    ($model:expr, not([$($vars:expr),* $(,)?])) => {{
        let vars_vec = [$($vars),*].to_vec();
        for var in vars_vec {
            let not_result = $model.bool_not(var);
            $model.new(not_result.eq(1));  // Constrain each NOT result to be true
        }
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // not(expr) - single variable only
    ($model:expr, not($expr:expr)) => {{
        let not_result = $model.bool_not($expr);
        $model.new(not_result.eq(1));  // Constrain the result to be true
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
}
