# CSP Solver

[![Crates.io](https://img.shields.io/crates/v/cspsolver.svg?color=blue)](https://crates.io/crates/cspsolver)
[![Documentation](https://docs.rs/cspsolver/badge.svg)](https://docs.rs/cspsolver)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A Constraint Satisfaction Problem (CSP) solver library written in Rust.

## Overview

This library provides efficient algorithms and data structures for solving constraint satisfaction problems. CSPs are mathematical problems defined as a set of objects whose state must satisfy a number of constraints or limitations.

Type of variables: `float`, `int`, `mixed` (int and float)

Constraints supported include:
- **Arithmetic**: `add`, `sub`, `mul`, `div`, `modulo`, `abs`, `sum`
- **Comparisons**: `lt`, `le`, `gt`, `ge`, `eq`, `ne` (short names for < ≤ > ≥ = ≠)
- **Boolean Logic**: `bool_and`, `bool_or`, `bool_not` (logical AND/OR/NOT operations)
- **Global**: `all_different`, `min`, `max` (vector-based)
- **Mixed Types**: Full support for integer and float variables in all constraints

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
cspsolver = "0.3.15"
```


## Examples

```bash
cargo run --example sudoku
cargo run --example pc_builder
cargo run --example resource_allocation
cargo run --example portfolio_optimization
```

```
🧩 Solving PLATINUM puzzle:
📊 Puzzle stats: 17 clues given, 64 empty cells

Puzzle:                                 Solution:
┌───────┬───────┬───────┐               ┌───────┬───────┬───────┐
│ · · · │ · · · │ · · · │               │ 9 8 7 │ 6 5 4 │ 3 2 1 │
│ · · · │ · · 3 │ · 8 5 │               │ 2 4 6 │ 1 7 3 │ 9 8 5 │
│ · · 1 │ · 2 · │ · · · │               │ 3 5 1 │ 9 2 8 │ 7 4 6 │
├───────┼───────┼───────┤               ├───────┼───────┼───────┤
│ · · · │ 5 · 7 │ · · · │               │ 1 2 8 │ 5 3 7 │ 6 9 4 │
│ · · 4 │ · · · │ 1 · · │               │ 6 3 4 │ 8 9 2 │ 1 5 7 │
│ · 9 · │ · · · │ · · · │               │ 7 9 5 │ 4 6 1 │ 8 3 2 │
├───────┼───────┼───────┤               ├───────┼───────┼───────┤
│ 5 · · │ · · · │ · 7 3 │               │ 5 1 9 │ 2 8 6 │ 4 7 3 │
│ · · 2 │ · 1 · │ · · · │               │ 4 7 2 │ 3 1 9 │ 5 6 8 │
│ · · · │ · 4 · │ · · 9 │               │ 8 6 3 │ 7 4 5 │ 2 1 9 │
└───────┴───────┴───────┘               └───────┴───────┴───────┘

✅ Solution found in 144330.511ms!
📊 Statistics: 638 propagations, 54 nodes explored
🔍 Efficiency: 11.8 propagations/node

```



### Basic Usage

```rust
use cspsolver::prelude::*;

fn main() {
    // constraint: v0(int) * 1.5 < 5.0
    // solving for maximum v0
    let mut m = Model::default();

    let v0 = m.new_var_int(1, 3);
    println!("v0 domain: [1, 3]");

    m.lt(v0.times_pos(float(1.5)), float(5.0));

    let solution = m.maximize(v0).unwrap();
    let x = match solution[v0] {
        Val::ValI(int_val) => int_val,
        _ => panic!("Expected integer value"),
    };

    assert!(x == 3);
    println!("Found optimal value: {}", x);
}
```

### Short Constraint Names

For more concise code, you can use short method names:

```rust
use cspsolver::prelude::*;

fn main() {
    let mut model = Model::default();
    let x = model.new_var_int(1, 10);
    let y = model.new_var_int(5, 15);
    
    // Short constraint names instead of long ones
    model.le(x, y);        // instead of less_than_or_equals
    model.ge(x, int(3));   // instead of greater_than_or_equals
    model.eq(y, int(8));   // instead of equals
    model.ne(x, y);        // instead of not_equals
    
    let solution = model.solve().unwrap();
    println!("x = {:?}, y = {:?}", solution[x], solution[y]);
}
```


## Status

The library is currently in active development. Features and APIs may change as we refine the implementation and add new functionality.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

