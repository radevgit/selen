

#[macro_export]
macro_rules! post_comparison {

        // ============================================================================
    // COMPARISON CONSTRAINTS
    // ============================================================================
    // Chained comparisons for between constraints: a <= b <= c, a < b < c, etc.
    ($model:expr, $lower:ident <= $middle:ident <= $upper:ident) => {{
        $model.props.between_constraint($lower, $middle, $upper);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $lower:ident < $middle:ident < $upper:ident) => {{
        $model.props.less_than($lower, $middle);
        $model.props.less_than($middle, $upper);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $lower:ident >= $middle:ident >= $upper:ident) => {{
        $model.props.greater_than_or_equals($lower, $middle);
        $model.props.greater_than_or_equals($middle, $upper);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $lower:ident > $middle:ident > $upper:ident) => {{
        $model.props.greater_than($lower, $middle);
        $model.props.greater_than($middle, $upper);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};

    // Handle simple variable comparisons: x < y, x <= y, etc.
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
    
    ($model:expr, $left:ident == $right:ident) => {{
        $model.props.equals($left, $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident != $right:ident) => {{
        $model.props.not_equals($left, $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};

    // Element constraint syntax: array[variable] == value
    // These patterns must come BEFORE general array indexing to match variable indices
    ($model:expr, $array:ident[$index:ident] == $value:ident) => {{
        $model.props.element($array.to_vec(), $index, $value);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $value:ident == $array:ident[$index:ident]) => {{
        $model.props.element($array.to_vec(), $index, $value);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};

    // Handle array indexing: vars[i] < vars[j], vars[0] == x, etc.
    ($model:expr, $left_array:ident[$left_index:expr] < $right_array:ident[$right_index:expr]) => {{
        $model.props.less_than($left_array[$left_index], $right_array[$right_index]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left_array:ident[$left_index:expr] <= $right_array:ident[$right_index:expr]) => {{
        $model.props.less_than_or_equals($left_array[$left_index], $right_array[$right_index]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left_array:ident[$left_index:expr] > $right_array:ident[$right_index:expr]) => {{
        $model.props.greater_than($left_array[$left_index], $right_array[$right_index]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left_array:ident[$left_index:expr] >= $right_array:ident[$right_index:expr]) => {{
        $model.props.greater_than_or_equals($left_array[$left_index], $right_array[$right_index]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left_array:ident[$left_index:expr] == $right_array:ident[$right_index:expr]) => {{
        $model.props.equals($left_array[$left_index], $right_array[$right_index]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left_array:ident[$left_index:expr] != $right_array:ident[$right_index:expr]) => {{
        $model.props.not_equals($left_array[$left_index], $right_array[$right_index]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};

    // Handle 2D array indexing: grid[i][j] < grid[k][l], grid[0][1] == x, etc.
    ($model:expr, $left_array:ident[$left_i:expr][$left_j:expr] < $right_array:ident[$right_i:expr][$right_j:expr]) => {{
        $model.props.less_than($left_array[$left_i][$left_j], $right_array[$right_i][$right_j]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left_array:ident[$left_i:expr][$left_j:expr] <= $right_array:ident[$right_i:expr][$right_j:expr]) => {{
        $model.props.less_than_or_equals($left_array[$left_i][$left_j], $right_array[$right_i][$right_j]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left_array:ident[$left_i:expr][$left_j:expr] > $right_array:ident[$right_i:expr][$right_j:expr]) => {{
        $model.props.greater_than($left_array[$left_i][$left_j], $right_array[$right_i][$right_j]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left_array:ident[$left_i:expr][$left_j:expr] >= $right_array:ident[$right_i:expr][$right_j:expr]) => {{
        $model.props.greater_than_or_equals($left_array[$left_i][$left_j], $right_array[$right_i][$right_j]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left_array:ident[$left_i:expr][$left_j:expr] == $right_array:ident[$right_i:expr][$right_j:expr]) => {{
        $model.props.equals($left_array[$left_i][$left_j], $right_array[$right_i][$right_j]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left_array:ident[$left_i:expr][$left_j:expr] != $right_array:ident[$right_i:expr][$right_j:expr]) => {{
        $model.props.not_equals($left_array[$left_i][$left_j], $right_array[$right_i][$right_j]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};

    // Handle 2D array vs variable: grid[i][j] < x, x == grid[0][1]
    ($model:expr, $left_array:ident[$left_i:expr][$left_j:expr] < $right:ident) => {{
        $model.props.less_than($left_array[$left_i][$left_j], $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left_array:ident[$left_i:expr][$left_j:expr] <= $right:ident) => {{
        $model.props.less_than_or_equals($left_array[$left_i][$left_j], $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left_array:ident[$left_i:expr][$left_j:expr] > $right:ident) => {{
        $model.props.greater_than($left_array[$left_i][$left_j], $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left_array:ident[$left_i:expr][$left_j:expr] >= $right:ident) => {{
        $model.props.greater_than_or_equals($left_array[$left_i][$left_j], $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left_array:ident[$left_i:expr][$left_j:expr] == $right:ident) => {{
        $model.props.equals($left_array[$left_i][$left_j], $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left_array:ident[$left_i:expr][$left_j:expr] != $right:ident) => {{
        $model.props.not_equals($left_array[$left_i][$left_j], $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};

    ($model:expr, $left:ident < $right_array:ident[$right_i:expr][$right_j:expr]) => {{
        $model.props.less_than($left, $right_array[$right_i][$right_j]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident <= $right_array:ident[$right_i:expr][$right_j:expr]) => {{
        $model.props.less_than_or_equals($left, $right_array[$right_i][$right_j]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident > $right_array:ident[$right_i:expr][$right_j:expr]) => {{
        $model.props.greater_than($left, $right_array[$right_i][$right_j]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident >= $right_array:ident[$right_i:expr][$right_j:expr]) => {{
        $model.props.greater_than_or_equals($left, $right_array[$right_i][$right_j]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident == $right_array:ident[$right_i:expr][$right_j:expr]) => {{
        $model.props.equals($left, $right_array[$right_i][$right_j]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident != $right_array:ident[$right_i:expr][$right_j:expr]) => {{
        $model.props.not_equals($left, $right_array[$right_i][$right_j]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};

    // Handle 2D array vs expression: grid[i][j] == int(5), grid[0][1] != int(3)
    ($model:expr, $left_array:ident[$left_i:expr][$left_j:expr] < $right:expr) => {{
        $model.props.less_than($left_array[$left_i][$left_j], $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left_array:ident[$left_i:expr][$left_j:expr] <= $right:expr) => {{
        $model.props.less_than_or_equals($left_array[$left_i][$left_j], $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left_array:ident[$left_i:expr][$left_j:expr] > $right:expr) => {{
        $model.props.greater_than($left_array[$left_i][$left_j], $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left_array:ident[$left_i:expr][$left_j:expr] >= $right:expr) => {{
        $model.props.greater_than_or_equals($left_array[$left_i][$left_j], $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left_array:ident[$left_i:expr][$left_j:expr] == $right:expr) => {{
        $model.props.equals($left_array[$left_i][$left_j], $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left_array:ident[$left_i:expr][$left_j:expr] != $right:expr) => {{
        $model.props.not_equals($left_array[$left_i][$left_j], $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};

    // Handle array vs variable: vars[i] < x, x == vars[0]
    ($model:expr, $left_array:ident[$left_index:expr] < $right:ident) => {{
        $model.props.less_than($left_array[$left_index], $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left_array:ident[$left_index:expr] <= $right:ident) => {{
        $model.props.less_than_or_equals($left_array[$left_index], $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left_array:ident[$left_index:expr] > $right:ident) => {{
        $model.props.greater_than($left_array[$left_index], $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left_array:ident[$left_index:expr] >= $right:ident) => {{
        $model.props.greater_than_or_equals($left_array[$left_index], $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left_array:ident[$left_index:expr] == $right:ident) => {{
        $model.props.equals($left_array[$left_index], $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left_array:ident[$left_index:expr] != $right:ident) => {{
        $model.props.not_equals($left_array[$left_index], $right);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};

    // Handle variable vs array: x < vars[i], y == vars[0]
    ($model:expr, $left:ident < $right_array:ident[$right_index:expr]) => {{
        $model.props.less_than($left, $right_array[$right_index]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident <= $right_array:ident[$right_index:expr]) => {{
        $model.props.less_than_or_equals($left, $right_array[$right_index]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident > $right_array:ident[$right_index:expr]) => {{
        $model.props.greater_than($left, $right_array[$right_index]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident >= $right_array:ident[$right_index:expr]) => {{
        $model.props.greater_than_or_equals($left, $right_array[$right_index]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident == $right_array:ident[$right_index:expr]) => {{
        $model.props.equals($left, $right_array[$right_index]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident != $right_array:ident[$right_index:expr]) => {{
        $model.props.not_equals($left, $right_array[$right_index]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};

    // Handle array vs literal: vars[i] < 5, vars[0] == 42
    ($model:expr, $left_array:ident[$left_index:expr] < $right:literal) => {{
        $model.props.less_than($left_array[$left_index], $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left_array:ident[$left_index:expr] <= $right:literal) => {{
        $model.props.less_than_or_equals($left_array[$left_index], $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left_array:ident[$left_index:expr] > $right:literal) => {{
        $model.props.greater_than($left_array[$left_index], $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left_array:ident[$left_index:expr] >= $right:literal) => {{
        $model.props.greater_than_or_equals($left_array[$left_index], $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left_array:ident[$left_index:expr] == $right:literal) => {{
        $model.props.equals($left_array[$left_index], $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left_array:ident[$left_index:expr] != $right:literal) => {{
        $model.props.not_equals($left_array[$left_index], $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};

    // Handle literal vs array: 5 < vars[i], 42 == vars[0]
    ($model:expr, $left:literal < $right_array:ident[$right_index:expr]) => {{
        $model.props.less_than($crate::variables::Val::from($left), $right_array[$right_index]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:literal <= $right_array:ident[$right_index:expr]) => {{
        $model.props.less_than_or_equals($crate::variables::Val::from($left), $right_array[$right_index]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:literal > $right_array:ident[$right_index:expr]) => {{
        $model.props.greater_than($crate::variables::Val::from($left), $right_array[$right_index]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:literal >= $right_array:ident[$right_index:expr]) => {{
        $model.props.greater_than_or_equals($crate::variables::Val::from($left), $right_array[$right_index]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:literal == $right_array:ident[$right_index:expr]) => {{
        $model.props.equals($crate::variables::Val::from($left), $right_array[$right_index]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:literal != $right_array:ident[$right_index:expr]) => {{
        $model.props.not_equals($crate::variables::Val::from($left), $right_array[$right_index]);
        $crate::constraints::macros::ConstraintRef::new(0)
    }};

    // Handle variable vs bare literal: x < 5, y >= 3.14
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
    
    ($model:expr, $left:ident == $right:literal) => {{
        $model.props.equals($left, $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident != $right:literal) => {{
        $model.props.not_equals($left, $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // Handle variable vs expression in parentheses: x < (y + 1)
    ($model:expr, $left:ident < ($right:expr)) => {{
        $model.props.less_than($left, $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident <= ($right:expr)) => {{
        $model.props.less_than_or_equals($left, $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident > ($right:expr)) => {{
        $model.props.greater_than($left, $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident >= ($right:expr)) => {{
        $model.props.greater_than_or_equals($left, $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident == ($right:expr)) => {{
        $model.props.equals($left, $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident != ($right:expr)) => {{
        $model.props.not_equals($left, $crate::variables::Val::from($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // Handle variable vs constant: x < int(5), y >= float(3.14)
    ($model:expr, $left:ident < int($right:expr)) => {{
        $model.props.less_than($left, $crate::prelude::int($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident <= int($right:expr)) => {{
        $model.props.less_than_or_equals($left, $crate::prelude::int($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident > int($right:expr)) => {{
        $model.props.greater_than($left, $crate::prelude::int($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident >= int($right:expr)) => {{
        $model.props.greater_than_or_equals($left, $crate::prelude::int($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident == int($right:expr)) => {{
        $model.props.equals($left, $crate::prelude::int($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident != int($right:expr)) => {{
        $model.props.not_equals($left, $crate::prelude::int($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    // Handle float constants
    ($model:expr, $left:ident < float($right:expr)) => {{
        $model.props.less_than($left, $crate::prelude::float($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident <= float($right:expr)) => {{
        $model.props.less_than_or_equals($left, $crate::prelude::float($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident > float($right:expr)) => {{
        $model.props.greater_than($left, $crate::prelude::float($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident >= float($right:expr)) => {{
        $model.props.greater_than_or_equals($left, $crate::prelude::float($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident == float($right:expr)) => {{
        $model.props.equals($left, $crate::prelude::float($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};
    
    ($model:expr, $left:ident != float($right:expr)) => {{
        $model.props.not_equals($left, $crate::prelude::float($right));
        $crate::constraints::macros::ConstraintRef::new(0)
    }};

}