# Selen - CSP Solver

[![Crates.io](https://img.shields.io/crates/v/selen.svg?color=blue)](https://crates.io/crates/selen)
[![Documentation](https://docs.rs/selen/badge.svg)](https://docs.rs/selen)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A Constraint Satisfaction Problem (CSP) solver library written in Rust with zero external dependencies. 


## Overview

This library provides efficient algorithms and data structures for solving constraint satisfaction problems. CSPs are mathematical problems defined as a set of objects whose state must satisfy a number of constraints or limitations.


**Variable Types**: `int`, `float`, mixed constraints

**Constraint Categories**:
- **Mathematical**: `+`, `-`, `*`, `/`, `%`, `abs()`, `min()`, `max()`, `sum()`
- **Comparison**: `==`, `!=`, `<`, `<=`, `>`, `>=` (natural syntax)
- **Boolean Logic**: `and()`, `or()`, `not()` with array syntax `and([a,b,c])` and variadic syntax `and(a,b,c,d)`
- **Global**: `alldiff()`, `allequal()`, element `x[y] = z`, `count(vars, value, count)`, `table(vars, tuples)`
- **Ordering**: `a <= b <= c`, `a < b < c`, `a >= b >= c`, `a > b > c` (natural `between` constraints) 
- **Cardinality**: `at_least(vars, value, count)`, `at_most(vars, value, count)`, `exactly(vars, value, count)`
- **Conditional**: `if_then(condition, constraint)`, `if_then_else(condition, then_constraint, else_constraint)`

**Programmatic version of constraints**
```
m.new(x.lt(y));                        // x < y
m.new(y.le(z));                        // y <= z
m.new(z.gt(5));                        // z > 5
m.new(x.add(y).le(z));                 // x + y <= z
m.new(y.sub(x).ge(0));                 // y - x >= 0
m.new(x.mul(y).eq(12));                // x * y == 12
m.new(z.div(y).ne(0));                 // z / y != 0
```

**Mathematical syntax with post! macro**
```
post!(m, x < y);                        // x < y
post!(m, y <= z);                       // y <= z
post!(m, z > int(5));                   // z > 5
post!(m, x + y <= z);                   // x + y <= z
post!(m, y - x >= int(0));              // y - x >= 0
post!(m, x * y == int(12));             // x * y == 12
post!(m, z / y != int(0));              // z / y != 0
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
selen = "0.8.6"
```

```
🧩 Solving Platinum Blonde puzzle:
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

✅ Solution found in 2289.885ms!
📊 Statistics: 538 propagations, 25 nodes explored
🔍 Efficiency: 21.5 propagations/node

```

## Examples

### Core Problems
```bash
cargo run --release --example sudoku                 # Classic 9x9 Sudoku solver
cargo run --release --example n_queens               # N-Queens backtracking
cargo run --release --example send_more_money        # Cryptarithmetic puzzle
cargo run --release --example graph_coloring         # Graph constraint problems
cargo run --release --example zebra_puzzle           # Logic puzzle solving
```

### Constraint Types
```bash
cargo run --release --example constraint_global      # AllEqual, Count, AllDiff
cargo run --release --example constraint_element     # Element constraint usage
cargo run --release --example constraint_table       # Table constraints
cargo run --release --example constraint_boolean     # Boolean arrays and logic
```

### Advanced Features
```bash
cargo run --release --example advanced_runtime_api   # Dynamic constraint building
cargo run --release --example advanced_memory_limits # Memory management demo
cargo run --release --example advanced_timeout       # Timeout handling
```

### Real Applications
```bash
cargo run --release --example app_manufacturing      # Industrial optimization
cargo run --release --example app_portfolio          # Financial modeling
cargo run --release --example app_resource_allocation # Resource planning
```

### Basic Usage

```rust
use selen::prelude::*;

fn main() {
    let mut m = Model::default();

    // Create variables
    let x = m.int(1, 10);       // Integer variable from 1 to 10
    let y = m.int(5, 15);       // Integer variable from 5 to 15

    // Add constraints
    post!(m, x < y);            // x must be less than y
    post!(m, x + y == int(12)); // x + y must equal 12
    
    // Solve the problem
    if let Ok(solution) = m.solve() {
        println!("x = {:?}", solution[x]);  // x = ValI(1)  
        println!("y = {:?}", solution[y]);  // y = ValI(11)
    }
}
```

### Programmatic API Example

For developers who prefer a more explicit, programmatic approach, the same constraints can be built using the runtime API:

```rust
use selen::prelude::*;

fn main() {
    let mut m = Model::default();

    // Create variables
    let x = m.int(1, 10);
    let y = m.int(5, 15);

    // Add constraints using programmatic API
    m.new(x.lt(y));                    // x < y
    m.new(x.add(y).eq(12));            // x + y == 12

    // Global constraints
    let vars = vec![m.int(1, 5), m.int(1, 5), m.int(1, 5)];
    m.alldiff(&vars.clone());       // All different
    
    // Mathematical functions
    let abs_result = m.abs(x);
    m.new(abs_result.ge(1));           // abs(x) >= 1
    
    // Solve the problem
    if let Ok(solution) = m.solve() {
        println!("x = {:?}", solution[x]);
        println!("y = {:?}", solution[y]);
    }
}
```




## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

