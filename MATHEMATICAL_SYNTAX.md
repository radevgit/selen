# Mathematical Constraint Syntax

A natural mathematical syntax for the CSP solver using `post!` and `postall!` macros.

## Basic Usage

```rust
use cspsolver::prelude::*;
use cspsolver::math_syntax::*;
use cspsolver::constraint_macros::*;

// Create model and variables
let mut m = Model::new();
let x = m.var_int(1, 10);
let y = m.var_int(1, 20);

// Post constraints using mathematical syntax
post!(m, x < y);
post!(m, x >= int(3));
post!(m, y % 3 == 1);
```

## Supported Features

### Basic Comparisons
```rust
post!(m, x < y);    // Less than
post!(m, x <= y);   // Less than or equal
post!(m, x > y);    // Greater than
post!(m, x >= y);   // Greater than or equal
post!(m, x == y);   // Equal
post!(m, x != y);   // Not equal
```

### Typed Constants
Use helper functions for explicit type specification:
```rust
post!(m, x >= int(5));     // Integer constant
post!(m, y <= float(3.14)); // Float constant
```

### Modulo Constraints
```rust
post!(m, x % 3 == 1);      // x mod 3 equals 1
post!(m, y % 7 != int(0)); // y is not divisible by 7
```

### Logical Operations
Two syntax styles are supported:

#### Function Style (Recommended)
```rust
post!(m, and(x < y, y >= int(5)));
post!(m, or(x == int(1), x == int(3)));
post!(m, not(x == y));
```

#### Operator Style
```rust
post!(m, (x < y) & (y >= int(5)));
post!(m, (x == int(1)) | (x == int(3)));
post!(m, !(x == y));
```

### Mathematical Functions
Basic functions are available:
```rust
// Helper functions for clarity
let sum_expr = max(x, y);
let abs_expr = abs(x);
let min_expr = min(x, y);
```

## Multiple Constraints

Use `postall!` for multiple constraints:
```rust
postall!(m, [
    x < y,
    y <= int(15),
    x >= int(1),
    y % 3 == 1
]);
```

## Complete Example

```rust
use cspsolver::prelude::*;
use cspsolver::math_syntax::*;
use cspsolver::constraint_macros::*;

fn main() {
    let mut m = Model::new();
    
    // Variables
    let x = m.var_int(1, 10);
    let y = m.var_int(1, 20);
    let z = m.var_int(1, 15);
    
    // Mathematical constraints
    post!(m, x < y);
    post!(m, and(y <= int(15), z >= int(5)));
    post!(m, x % 3 == 1);
    post!(m, or(x == int(1), x == int(7)));
    
    // Solve
    let solution = m.solve().expect("Solution found");
    println!("x = {}, y = {}, z = {}", 
             solution.get_int(x), 
             solution.get_int(y), 
             solution.get_int(z));
}
```

## Future Enhancements

Planned syntax extensions include:
- Arithmetic expressions: `post!(m, x + 3 < y)`
- Multiplication: `post!(m, x * 2 <= 10)`
- Function integration: `post!(m, abs(x) >= int(1))`
- Complex expressions: `post!(m, max(x, y) <= int(15))`
- Nested logical operations: `post!(m, and(or(c1, c2), not(c3)))`

## Implementation Details

The syntax is implemented through:
- `math_syntax.rs`: Helper functions and type definitions
- `constraint_macros.rs`: Macro implementations for `post!` and `postall!`
- Integration with the existing Model constraint API

The macros parse mathematical expressions and translate them to appropriate Model method calls (`lt()`, `eq()`, `and()`, etc.).