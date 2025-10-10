# Selen API Quick Reference - Preferred Style

## Variable Creation

```rust
let mut m = Model::default();

// Single variables
let x = m.int(1, 10);        // Integer in range [1, 10]
let f = m.float(0.0, 1.0);   // Float in range [0.0, 1.0]
let b = m.bool();            // Boolean variable

// Multiple variables
let vars = m.ints(5, 1, 10); // 5 integers in range [1, 10]
let floats = m.floats(3, 0.0, 1.0); // 3 floats
let bools = m.bools(4);      // 4 booleans
```

## Constants

Always use explicit type constructors:

```rust
int(5)        // Integer constant 5
float(3.14)   // Float constant 3.14
```

❌ **Never** use raw literals in constraints: `x.eq(5)` won't work!  
✅ **Always** use type constructors: `x.eq(int(5))`

## Arithmetic Expressions

Build expressions with functions that return `ExprBuilder`:

```rust
add(x, y)              // x + y
sub(x, y)              // x - y
mul(x, y)              // x * y
div(x, y)              // x / y
modulo(x, y)           // x % y

// Compose expressions
add(mul(x, int(2)), y)           // 2*x + y
add(add(x, y), z)                // x + y + z
```

## Posting Constraints (Runtime API - PREFERRED!)

Use `m.new()` with the runtime API methods on variables:

```rust
// Equality and inequality
m.new(x.eq(y));           // x == y
m.new(x.eq(int(5)));      // x == 5
m.new(x.ne(y));           // x != y

// Comparisons
m.new(x.lt(y));           // x < y
m.new(x.le(int(10)));     // x <= 10
m.new(x.gt(y));           // x > y
m.new(x.ge(int(0)));      // x >= 0

// With expressions
m.new(add(x, y).eq(z));                    // x + y == z
m.new(mul(x, int(2)).lt(int(20)));        // 2*x < 20
m.new(sub(x, y).eq(int(0)));              // x - y == 0
```

## Global Constraints (Method Syntax)

Call methods on the model:

```rust
// All different - all variables must have different values
m.alldiff(&[x, y, z]);

// All equal - all variables must have the same value
m.alleq(&[x, y, z]);
```

## Linear Constraints

For linear equations with coefficients:

```rust
// Integer linear: 2*x + 3*y = 10
lin_eq(&mut m, &[2, 3], &[x, y], 10);

// Float linear: 1.5*a + 2.5*b <= 10.0
lin_le(&mut m, &[1.5, 2.5], &[a, b], 10.0);

// Linear inequality: x + 2*y != 5
lin_ne(&mut m, &[1, 2], &[x, y], 5);
```

## Special Functions

```rust
// Absolute value - returns a new variable
let abs_x = abs(&mut m, x);
m.new(abs_x.eq(int(5)));

// Absolute value of expression
let diff = sub(x, y);
let abs_diff = abs(&mut m, diff);
m.new(abs_diff.eq(int(1)));  // |x - y| == 1
```

## Solving

```rust
match m.solve() {
    Ok(solution) => {
        let val = solution.get_int(x);
        let fval = solution.get_float(f);
        let bval = solution.get_bool(b);
        println!("x = {}", val);
    }
    Err(e) => {
        println!("No solution: {:?}", e);
    }
}
```

## Complete Example

```rust
use selen::prelude::*;

fn main() {
    let mut m = Model::default();
    
    // Create variables
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    let z = m.int(1, 20);
    
    // Post constraints using runtime API
    m.new(x.lt(y));                    // x < y
    m.new(add(x, y).eq(z));           // x + y = z
    m.new(z.le(int(15)));             // z <= 15
    
    // Solve
    match m.solve() {
        Ok(sol) => {
            println!("x={}, y={}, z={}", 
                sol.get_int(x), 
                sol.get_int(y), 
                sol.get_int(z));
        }
        Err(_) => println!("No solution"),
    }
}
```

## Style Guidelines Summary

✅ **DO:**
- Use `m.new(x.eq(y))` for constraints (runtime API)
- Use `int(5)` and `float(3.14)` for constants
- Use `add()`, `mul()`, etc. for expressions
- Use `m.alldiff()` for global constraints
- Use short model names: `m` or `mm`

❌ **DON'T:**
- Use `eq(&mut m, x, y)` - too verbose
- Use raw literals: `x.eq(5)` - won't work!
- Mix up expressions and constraints

## Why This Style?

**Clean and Fluent:**
```rust
m.new(x.eq(y));              // Reads naturally: "post that x equals y"
```

**Composable:**
```rust
m.new(add(x, y).eq(z));      // Build complex expressions easily
```

**Consistent:**
```rust
m.new(x.eq(y));              // Same pattern everywhere
m.new(x.lt(int(5)));
m.new(add(x, y).eq(z));
```
