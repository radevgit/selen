## Advanced/Complicated Examples


# Programmatic API Guide

This guide explains how to use the Selen CSP solver's programmatic API for building constraints dynamically at runtime. This is useful when constraints depend on user input, data, or are not known at compile time.

## When to Use the Programmatic API
- When you need to generate constraints based on data or user input
- When constraints are not static or known in advance
- For advanced use cases where mathematical syntax is not flexible enough

## Key Concepts
- **Fluent builder pattern**: Chain methods to build complex expressions
- **m.new()**: The primary method to post programmatic constraints
- **int() / float()**: Always wrap constants in constraints

## Basic Usage

```rust
use selen::prelude::*;

fn main() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    let y = m.int(1, 10);

    // Comparison constraint: x < y
    m.new(x.lt(y));

    // Comparison with constant: x >= 5
    m.new(x.ge(int(5)));

    // Arithmetic constraint: x + y == 12
    m.new(x.add(y).eq(int(12)));

    // Chained arithmetic: x * 2 + y <= 10
    m.new(x.mul(int(2)).add(y).le(int(10)));
}
```

## Building Expressions

You can chain methods to build complex expressions:

```rust
// (x * 3 + y) % 4 == 1
m.new(x.mul(int(3)).add(y).modulo(int(4)).eq(int(1)));

// abs(x - y) <= 2
m.new(x.sub(y).abs().le(int(2)));

// (x + y) * z >= 20
m.new(x.add(y).mul(z).ge(int(20)));
```

## Dynamic Constraint Generation

You can generate constraints in loops or based on data:

```rust
let vars: Vec<_> = (0..5).map(|_| m.int(1, 10)).collect();

// All variables must be different
m.new(alldiff(vars.clone()));

// Post constraints for each variable
for v in &vars {
    m.new(v.ge(int(3)));
}
```

## Boolean Logic

You can build boolean expressions and combine them:

```rust
let a = m.bool();
let b = m.bool();

// a == (x > 5)
m.new(a.eq(x.gt(int(5))));
// b == (y < 7)
m.new(b.eq(y.lt(int(7))));
// a AND b
m.new(and([a, b]));
// a OR b
m.new(or([a, b]));
// NOT a
m.new(not(a));
```

## Global Constraints

```rust
let vars = vec![x, y, z];
// All different
m.new(alldiff(vars.clone()));
// All equal
m.new(allequal(vars.clone()));
// Sum equals 15
m.new(sum(vars.clone()).eq(int(15)));
```

## Tips
- Always use `int()` or `float()` for constants in constraints
- Use method chaining to build up complex expressions
- Use `m.new()` to post each constraint
- For static constraints, the constraint API provides a natural fluent syntax

---

For more details, see the [API documentation](https://docs.rs/selen) and the `examples/advanced_runtime_api.rs` file in the repository.


## Advanced/Complicated Examples

Here are some advanced examples demonstrating the flexibility of the programmatic API:

```rust
use selen::prelude::*;

fn main() {
    let mut m = Model::default();
    let n = 5;
    let vars = m.ints(1, 20, n);
    let z = m.int(1, 100);

    // 1. All variables are pairwise different and their sum is even
    m.new(alldiff(vars.clone()));
    let sum_var = m.int(0, 100);
    m.new(sum(vars.clone()).eq(sum_var));
    m.new(sum_var.modulo(int(2)).eq(int(0)));

    // 2. For each i, vars[i] * (i+1) <= z
    for (i, v) in vars.iter().enumerate() {
        m.new(v.mul(int((i+1) as i32)).le(z));
    }

    // 3. At least 2 variables are greater than 10
    let bools = m.bools(n);
    for (b, v) in bools.iter().zip(vars.iter()) {
        m.new(b.eq(v.gt(int(10))));
    }
    m.new(sum(bools.clone()).ge(int(2)));

    // 4. Nested boolean logic: (x > 5 && y < 7) || z == 42
    let x = vars[0];
    let y = vars[1];
    let cond1 = m.bool();
    let cond2 = m.bool();
    let cond3 = m.bool();
    m.new(cond1.eq(x.gt(int(5))));
    m.new(cond2.eq(y.lt(int(7))));
    m.new(cond3.eq(z.eq(int(42))));
    let and_cond = m.bool();
    m.new(and_cond.eq(and([cond1, cond2])));
    m.new(or([and_cond, cond3]));

    // 5. Complex arithmetic: product of all variables >= 1000
    let mut prod = vars[0].clone();
    for v in vars.iter().skip(1) {
        prod = prod.mul(*v);
    }
    m.new(prod.ge(int(1000)));

    // 6. Absolute difference constraints: |x - y| + |y - z| <= 15
    let abs_sum = x.sub(y).abs().add(y.sub(z).abs());
    m.new(abs_sum.le(int(15)));

    // 7. Conditional constraint: if x > 10 then z < 50
    let cond = m.bool();
    m.new(cond.eq(x.gt(int(10))));
    m.new(implies(cond, z.lt(int(50))));
}
```
