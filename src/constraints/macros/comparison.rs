#[macro_export]#[macro_export]

macro_rules! post_comparison {macro_rules! post_comparison {

    // Basic variable comparisons    // ============================================================================

    ($model:expr, $left:ident == $right:ident) => {{    // BASIC COMPARISON PATTERNS

        $model.props.equals($left, $right);    // ============================================================================

        $crate::constraints::macros::ConstraintRef::new(0)

    }};    

    

    ($model:expr, $left:ident != $right:ident) => {{    // x == y    // BASIC COMPARISON PATTERNS

        $model.props.not_equals($left, $right);

        $crate::constraints::macros::ConstraintRef::new(0)    ($model:expr, $left:ident == $right:ident) => {{

    }};

            $model.props.equals($left, $right);    // ============================================================================    // ============================================================================    // Basic comparisons

    ($model:expr, $left:ident < $right:ident) => {{

        $model.props.less_than($left, $right);        $crate::constraints::macros::ConstraintRef::new(0)

        $crate::constraints::macros::ConstraintRef::new(0)

    }};    }};    

    

    ($model:expr, $left:ident <= $right:ident) => {{    

        $model.props.less_than_or_equals($left, $right);

        $crate::constraints::macros::ConstraintRef::new(0)    // x != y    // x == y    // BASIC COMPARISON PATTERNS - variables to variables    ($model:expr, $left:ident == $right:ident) => {{

    }};

        ($model:expr, $left:ident != $right:ident) => {{

    ($model:expr, $left:ident > $right:ident) => {{

        $model.props.greater_than($left, $right);        $model.props.not_equals($left, $right);    ($model:expr, $left:ident == $right:ident) => {{

        $crate::constraints::macros::ConstraintRef::new(0)

    }};        $crate::constraints::macros::ConstraintRef::new(0)

    

    ($model:expr, $left:ident >= $right:ident) => {{    }};        $model.props.equals($left, $right);    // ============================================================================        $model.props.equals($left, $right);

        $model.props.greater_than_or_equals($left, $right);

        $crate::constraints::macros::ConstraintRef::new(0)    

    }};

        // x < y        $crate::constraints::macros::ConstraintRef::new(0)

    // Variable vs literal

    ($model:expr, $left:ident == $right:literal) => {{    ($model:expr, $left:ident < $right:ident) => {{

        $model.props.equals($left, $crate::variables::Val::from($right));

        $crate::constraints::macros::ConstraintRef::new(0)        $model.props.less_than($left, $right);    }};            $crate::constraints::macros::ConstraintRef::new(0)

    }};

            $crate::constraints::macros::ConstraintRef::new(0)

    ($model:expr, $left:ident != $right:literal) => {{

        $model.props.not_equals($left, $crate::variables::Val::from($right));    }};    

        $crate::constraints::macros::ConstraintRef::new(0)

    }};    

    

    ($model:expr, $left:ident < $right:literal) => {{    // x <= y

        $model.props.less_than($left, $crate::variables::Val::from($right));    ($model:expr, $left:ident <= $right:ident) => {{

        $crate::constraints::macros::ConstraintRef::new(0)        $model.props.less_than_or_equals($left, $right);

    }};        $crate::constraints::macros::ConstraintRef::new(0)

        }};

    ($model:expr, $left:ident <= $right:literal) => {{    

        $model.props.less_than_or_equals($left, $crate::variables::Val::from($right));    // x != y

        $crate::constraints::macros::ConstraintRef::new(0)    ($model:expr, $left:ident != $right:ident) => {{

    }};        $model.props.not_equals($left, $right);

            $crate::constraints::macros::ConstraintRef::new(0)

    ($model:expr, $left:ident > $right:literal) => {{    }};

        $model.props.greater_than($left, $crate::variables::Val::from($right));    

        $crate::constraints::macros::ConstraintRef::new(0)    // x == y

    }};    ($model:expr, $left:ident == $right:ident) => {{

            $model.props.equals($left, $right);

    ($model:expr, $left:ident >= $right:literal) => {{        $crate::constraints::macros::ConstraintRef::new(0)

        $model.props.greater_than_or_equals($left, $crate::variables::Val::from($right));    }};    

        $crate::constraints::macros::ConstraintRef::new(0)

    }};    

    

    // Variable vs int() expressions    // x > y

    ($model:expr, $left:ident == int($right:expr)) => {{    ($model:expr, $left:ident > $right:ident) => {{

        $model.props.equals($left, $crate::prelude::int($right));        $model.props.greater_than($left, $right);

        $crate::constraints::macros::ConstraintRef::new(0)        $crate::constraints::macros::ConstraintRef::new(0)

    }};    }};

        

    ($model:expr, $left:ident != int($right:expr)) => {{    // x >= y

        $model.props.not_equals($left, $crate::prelude::int($right));    ($model:expr, $left:ident >= $right:ident) => {{

        $crate::constraints::macros::ConstraintRef::new(0)        $model.props.greater_than_or_equals($left, $right);

    }};        $crate::constraints::macros::ConstraintRef::new(0)

        }};

    ($model:expr, $left:ident < int($right:expr)) => {{    

        $model.props.less_than($left, $crate::prelude::int($right));    // x < y

        $crate::constraints::macros::ConstraintRef::new(0)    ($model:expr, $left:ident < $right:ident) => {{

    }};        $model.props.less_than($left, $right);

            $crate::constraints::macros::ConstraintRef::new(0)

    ($model:expr, $left:ident <= int($right:expr)) => {{    }};

        $model.props.less_than_or_equals($left, $crate::prelude::int($right));    

        $crate::constraints::macros::ConstraintRef::new(0)    // ============================================================================

    }};    // CONSTANTS WITH INT() AND FLOAT()

        // ============================================================================

    ($model:expr, $left:ident > int($right:expr)) => {{    

        $model.props.greater_than($left, $crate::prelude::int($right));    // x == int(5)    

        $crate::constraints::macros::ConstraintRef::new(0)

    }};    ($model:expr, $left:ident == int($right:expr)) => {{

    

    ($model:expr, $left:ident >= int($right:expr)) => {{        $model.props.equals($left, $crate::prelude::int($right));

        $model.props.greater_than_or_equals($left, $crate::prelude::int($right));        $crate::constraints::macros::ConstraintRef::new(0)

        $crate::constraints::macros::ConstraintRef::new(0)    }};

    }};    

        // x <= y

    // Variable vs float() expressions    ($model:expr, $left:ident <= $right:ident) => {{

    ($model:expr, $left:ident == float($right:expr)) => {{        $model.props.less_than_or_equals($left, $right);

        $model.props.equals($left, $crate::prelude::float($right));        $crate::constraints::macros::ConstraintRef::new(0)

        $crate::constraints::macros::ConstraintRef::new(0)    }};

    }};    

        // x != int(5)

    ($model:expr, $left:ident <= float($right:expr)) => {{    ($model:expr, $left:ident != int($right:expr)) => {{

        $model.props.less_than_or_equals($left, $crate::prelude::float($right));        $model.props.not_equals($left, $crate::prelude::int($right));

        $crate::constraints::macros::ConstraintRef::new(0)        $crate::constraints::macros::ConstraintRef::new(0)

    }};    }};

        

    ($model:expr, $left:ident >= float($right:expr)) => {{    // x < int(5)

        $model.props.greater_than_or_equals($left, $crate::prelude::float($right));    ($model:expr, $left:ident < int($right:expr)) => {{

        $crate::constraints::macros::ConstraintRef::new(0)        $model.props.less_than($left, $crate::prelude::int($right));

    }};        $crate::constraints::macros::ConstraintRef::new(0)

}
        $crate::constraints::macros::ConstraintRef::new(0)

    }};    ($model:expr, $left:ident > $right:ident) => {{

    

    // x <= int(5)        $model.props.greater_than($left, $right);    }};    }};

    ($model:expr, $left:ident <= int($right:expr)) => {{

        $model.props.less_than_or_equals($left, $crate::prelude::int($right));        $crate::constraints::macros::ConstraintRef::new(0)

        $crate::constraints::macros::ConstraintRef::new(0)

    }};    }};        

    

    // x > int(5)    

    ($model:expr, $left:ident > int($right:expr)) => {{

        $model.props.greater_than($left, $crate::prelude::int($right));    // x >= y    // x < y    ($model:expr, $left:ident <= $right:ident) => {{

        $crate::constraints::macros::ConstraintRef::new(0)

    }};    ($model:expr, $left:ident >= $right:ident) => {{

    

    // x >= int(5)        $model.props.greater_than_or_equals($left, $right);    ($model:expr, $left:ident < $right:ident) => {{        $model.props.less_than_or_equals($left, $right);

    ($model:expr, $left:ident >= int($right:expr)) => {{

        $model.props.greater_than_or_equals($left, $crate::prelude::int($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // ============================================================================
    // FLOAT CONSTANTS
    // ============================================================================

    

    // x == float(3.14)    // CONSTANTS WITH INT() AND FLOAT()

    ($model:expr, $left:ident == float($right:expr)) => {{

        $model.props.equals($left, $crate::prelude::float($right));    // ============================================================================    }};    

        $crate::constraints::macros::ConstraintRef::new(0)

    }};    

    

    // x <= float(3.14)    // x == int(5)        ($model:expr, $left:ident > $right:ident) => {{

    ($model:expr, $left:ident <= float($right:expr)) => {{

        $model.props.less_than_or_equals($left, $crate::prelude::float($right));    ($model:expr, $left:ident == int($right:expr)) => {{

        $crate::constraints::macros::ConstraintRef::new(0)

    }};        $model.props.equals($left, $crate::prelude::int($right));    // x <= y        $model.props.greater_than($left, $right);

    

    // x >= float(3.14)        $crate::constraints::macros::ConstraintRef::new(0)

    ($model:expr, $left:ident >= float($right:expr)) => {{

        $model.props.greater_than_or_equals($left, $crate::prelude::float($right));    }};    ($model:expr, $left:ident <= $right:ident) => {{        $crate::constraints::macros::ConstraintRef::new(0)

        $crate::constraints::macros::ConstraintRef::new(0)

    }};    

    

    // ============================================================================    // x != int(5)        $model.props.less_than_or_equals($left, $right);    }};

    // LITERAL VALUES

    // ============================================================================    ($model:expr, $left:ident != int($right:expr)) => {{

    

    // x == 5
    ($model:expr, $left:ident == $right:literal) => {{
        $model.props.equals($left, $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x != 5
    ($model:expr, $left:ident != $right:literal) => {{
        $model.props.not_equals($left, $crate::variables::Val::from($right));

        $crate::constraints::macros::ConstraintRef::new(0)

    }};    ($model:expr, $left:ident < int($right:expr)) => {{

    // x < 5
    ($model:expr, $left:ident < $right:literal) => {{
        $model.props.less_than($left, $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x > y
    ($model:expr, $left:ident > $right:ident) => {{
        $model.props.greater_than($left, $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x <= 5
    ($model:expr, $left:ident <= $right:literal) => {{
        $model.props.less_than_or_equals($left, $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};

    

    // x > 5        $model.props.less_than_or_equals($left, $crate::prelude::int($right));        $crate::constraints::macros::ConstraintRef::new(0)

    ($model:expr, $left:ident > $right:literal) => {{

        $model.props.greater_than($left, $crate::variables::Val::from($right));        $crate::constraints::macros::ConstraintRef::new(0)    }};

        $crate::constraints::macros::ConstraintRef::new(0)

    }};    }};    

    // x >= 5 (literal)
    ($model:expr, $left:ident >= $right:literal) => {{
        $model.props.greater_than_or_equals($left, $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x > int(5)
    ($model:expr, $left:ident > int($right:expr)) => {{
        $model.props.greater_than($left, $crate::prelude::int($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // ============================================================================
    // ARRAY ACCESS PATTERNS
    // ============================================================================

    

    // vars[i] == vars[j]    }};    

    ($model:expr, $left_array:ident[$left_index:expr] == $right_array:ident[$right_index:expr]) => {{

        $model.props.equals($left_array[$left_index], $right_array[$right_index]);        // ============================================================================

        $crate::constraints::macros::ConstraintRef::new(0)

    }};    // x >= int(5)    // VARIABLE vs LITERAL PATTERNS

    

    // vars[i] == x    ($model:expr, $left:ident >= int($right:expr)) => {{    // ============================================================================

    ($model:expr, $left_array:ident[$left_index:expr] == $right:ident) => {{

        $model.props.equals($left_array[$left_index], $right);        $model.props.greater_than_or_equals($left, $crate::prelude::int($right));    

        $crate::constraints::macros::ConstraintRef::new(0)

    }};        $crate::constraints::macros::ConstraintRef::new(0)    // x == 5 (literal)

    

    // x == vars[i]    }};    ($model:expr, $left:ident == $right:literal) => {{

    ($model:expr, $left:ident == $right_array:ident[$right_index:expr]) => {{

        $model.props.equals($left, $right_array[$right_index]);            $model.props.equals($left, $crate::variables::Val::from($right));

        $crate::constraints::macros::ConstraintRef::new(0)

    }};    // ============================================================================        $crate::constraints::macros::ConstraintRef::new(0)

    

    // vars[i] == 5    // FLOAT CONSTANTS    }};

    ($model:expr, $left_array:ident[$left_index:expr] == $right:literal) => {{

        $model.props.equals($left_array[$left_index], $crate::variables::Val::from($right));    // ============================================================================    

        $crate::constraints::macros::ConstraintRef::new(0)

    }};        // x != 5 (literal)

    

    // vars[i] == int(5)
    ($model:expr, $left_array:ident[$left_index:expr] == int($right:expr)) => {{
        $model.props.equals($left_array[$left_index], $crate::prelude::int($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x < 5 (literal)
    ($model:expr, $left:ident < $right:literal) => {{
        $model.props.less_than($left, $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x <= float(3.14)
    ($model:expr, $left:ident <= float($right:expr)) => {{
        $model.props.less_than_or_equals($left, $crate::prelude::float($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x >= float(3.14)
    ($model:expr, $left:ident >= float($right:expr)) => {{
        $model.props.greater_than_or_equals($left, $crate::prelude::float($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // ============================================================================
    // LITERAL VALUES
    // ============================================================================
    
    // x > 5 (literal)
    ($model:expr, $left:ident > $right:literal) => {{
        $model.props.greater_than($left, $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x == 5
    ($model:expr, $left:ident == $right:literal) => {{
        $model.props.equals($left, $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x != 5
    ($model:expr, $left:ident != $right:literal) => {{
        $model.props.not_equals($left, $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x >= 5 (literal)
    ($model:expr, $left:ident >= $right:literal) => {{
        $model.props.greater_than_or_equals($left, $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // ============================================================================
    // VARIABLE vs FLOAT/INT EXPRESSION PATTERNS
    // ============================================================================
    
    // x == float(5.0)
    ($model:expr, $left:ident == float($right:expr)) => {{
        $model.props.equals($left, $crate::prelude::float($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x < 5
    ($model:expr, $left:ident < $right:literal) => {{
        $model.props.less_than($left, $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x != float(5.0)
    ($model:expr, $left:ident != float($right:expr)) => {{

    ($model:expr, $left:ident <= $right:literal) => {{        $model.props.not_equals($left, $crate::prelude::float($right));

        $model.props.less_than_or_equals($left, $crate::variables::Val::from($right));        $crate::constraints::macros::ConstraintRef::new(0)

        $crate::constraints::macros::ConstraintRef::new(0)    }};

    }};    

        // x < float(5.0)

    // x < float(5.0)
    ($model:expr, $left:ident < float($right:expr)) => {{
        $model.props.less_than($left, $crate::prelude::float($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x > 5
    ($model:expr, $left:ident > $right:literal) => {{
        $model.props.greater_than($left, $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x <= float(5.0)
    ($model:expr, $left:ident <= float($right:expr)) => {{
        $model.props.less_than_or_equals($left, $crate::prelude::float($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x >= 5
    ($model:expr, $left:ident >= $right:literal) => {{
        $model.props.greater_than_or_equals($left, $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x > float(5.0)
    ($model:expr, $left:ident > float($right:expr)) => {{
        $model.props.greater_than($left, $crate::prelude::float($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // ============================================================================
    // ARRAY ACCESS PATTERNS
    // ============================================================================

    // vars[i] == vars[j]    

    // vars[i] == vars[j]
    ($model:expr, $left_array:ident[$left_index:expr] == $right_array:ident[$right_index:expr]) => {{
        $model.props.equals($left_array[$left_index], $right_array[$right_index]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // vars[i] == x
    ($model:expr, $left_array:ident[$left_index:expr] == $right:ident) => {{
        $model.props.equals($left_array[$left_index], $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x == vars[i]
    ($model:expr, $left:ident == $right_array:ident[$right_index:expr]) => {{
        $model.props.equals($left, $right_array[$right_index]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // vars[i] == 5
    ($model:expr, $left_array:ident[$left_index:expr] == $right:literal) => {{
        $model.props.equals($left_array[$left_index], $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // vars[i] == int(5)
    ($model:expr, $left_array:ident[$left_index:expr] == int($right:expr)) => {{
        $model.props.equals($left_array[$left_index], $crate::prelude::int($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    // x > int(5)
    ($model:expr, $left:ident > int($right:expr)) => {{
        $model.props.greater_than($left, $crate::prelude::int($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x >= int(5)
    ($model:expr, $left:ident >= int($right:expr)) => {{
        $model.props.greater_than_or_equals($left, $crate::prelude::int($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // ============================================================================
    // VARIABLE vs PARENTHESIZED EXPRESSION PATTERNS
    // ============================================================================
    
    // x < (expr)
    ($model:expr, $left:ident < ($right:expr)) => {{
        $model.props.less_than($left, $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x <= (expr)
    ($model:expr, $left:ident <= ($right:expr)) => {{
        $model.props.less_than_or_equals($left, $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x > (expr)
    ($model:expr, $left:ident > ($right:expr)) => {{
        $model.props.greater_than($left, $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x >= (expr)
    ($model:expr, $left:ident >= ($right:expr)) => {{
        $model.props.greater_than_or_equals($left, $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x == (expr)
    ($model:expr, $left:ident == ($right:expr)) => {{
        $model.props.equals($left, $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // x != (expr)
    ($model:expr, $left:ident != ($right:expr)) => {{
        $model.props.not_equals($left, $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
}
