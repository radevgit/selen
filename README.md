# Selen - CSP Solver

[![Crates.io](https://img.shields.io/crates/v/selen.svg?color=blue)](https://crates.io/crates/selen)
[![Documentation](https://docs.rs/selen/badge.svg)](https://docs.rs/selen)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A Constraint Satisfaction Problem (CSP) solver library written in Rust with zero external dependencies, featuring an integrated LP solver for linear optimization problems.


## Overview

This library provides efficient algorithms and data structures for solving constraint satisfaction problems. CSPs are mathematical problems defined as a set of objects whose state must satisfy a number of constraints or limitations.

**Key Features**:
- 🚀 **Zero dependencies** - Pure Rust implementation
- 🔧 **Generic API** - Works with both integers and floats
- 📊 **LP Solver Integration** - Automatic linear programming optimization
- 🎯 **Constraint Propagation** - Efficient domain reduction
- 🔄 **Hybrid Solving** - Combines CP and LP techniques


**Variable Types**: `int`, `float`, mixed constraints

**Constraint API**
```rust
// Comparison constraints (via runtime API)
m.new(x.lt(y));                        // x < y
m.new(y.le(z));                        // y <= z
m.new(z.gt(5));                        // z > 5
m.new(x.eq(10));                       // x == 10
m.new(x.ne(y));                        // x != y
m.new(x.ge(5));                        // x >= 5

// Arithmetic operations (return new variables)
let sum = m.add(x, y);                 // sum = x + y
let diff = m.sub(x, y);                // diff = x - y
let product = m.mul(x, y);             // product = x * y
let quotient = m.div(x, y);            // quotient = x / y
let remainder = m.modulo(x, y);        // remainder = x % y
let absolute = m.abs(x);               // absolute = |x|

// Aggregate operations
let minimum = m.min(&[x, y, z])?;      // minimum of variables
let maximum = m.max(&[x, y, z])?;      // maximum of variables
let total = m.sum(&[x, y, z]);         // sum of variables

// Global constraints
m.alldiff(&[x, y, z]);                 // all variables different
m.alleq(&[x, y, z]);                   // all variables equal
m.element(&array, index, value);       // array[index] == value
m.table(&vars, tuples);                // table constraint (valid tuples)
m.count(&vars, value, count_var);      // count occurrences of value
m.between(lower, middle, upper);       // lower <= middle <= upper
m.at_least(&vars, value, n);           // at least n vars == value
m.at_most(&vars, value, n);            // at most n vars == value
m.exactly(&vars, value, n);            // exactly n vars == value
m.gcc(&vars, values, counts);          // global cardinality constraint

// Boolean operations (return boolean variables)
let and_result = m.bool_and(&[a, b]);  // a AND b
let or_result = m.bool_or(&[a, b]);    // a OR b
let not_result = m.bool_not(a);        // NOT a
m.bool_clause(&[a, b], &[c]);          // a ∨ b ∨ ¬c (CNF clause)

// Fluent expression building
m.new(x.add(y).le(z));                 // x + y <= z
m.new(y.sub(x).ge(0));                 // y - x >= 0
m.new(x.mul(y).eq(12));                // x * y == 12
m.new(z.div(y).ne(0));                 // z / y != 0

// Linear constraints (weighted sums) - generic for int and float
m.lin_eq(&[2, 3], &[x, y], 10);        // 2x + 3y == 10
m.lin_le(&[1, -1], &[x, y], 5);        // x - y <= 5
m.lin_ne(&[2, 1], &[x, y], 8);         // 2x + y != 8
m.lin_eq(&[1.5, 2.0], &[x, y], 7.5);   // 1.5x + 2.0y == 7.5 (works with floats)
m.lin_le(&[0.5, 1.0], &[x, y], 3.0);   // 0.5x + y <= 3.0 (works with floats)
m.bool_lin_eq(&[1, 1, 1], &[a, b, c], 2);   // a + b + c == 2

// Reified constraints (with boolean result) - generic for int and float
m.eq_reif(x, y, b);                    // b ↔ (x == y)
m.ne_reif(x, y, b);                    // b ↔ (x != y)
m.lt_reif(x, y, b);                    // b ↔ (x < y)
m.le_reif(x, y, b);                    // b ↔ (x <= y)
m.gt_reif(x, y, b);                    // b ↔ (x > y)
m.ge_reif(x, y, b);                    // b ↔ (x >= y)
m.lin_eq_reif(&[2, 1], &[x, y], 5, b); // b ↔ (2x + y == 5)

// Type conversion constraints
m.int2float(int_var, float_var);       // float_var = int_var (as float)
m.float2int_floor(float_var, int_var); // int_var = floor(float_var)
m.float2int_ceil(float_var, int_var);  // int_var = ceil(float_var)
m.float2int_round(float_var, int_var); // int_var = round(float_var)

// Array operations
m.array_int_element(index, &array, result);     // result = array[index]
m.array_int_minimum(&array)?;                   // minimum of array
m.array_int_maximum(&array)?;                   // maximum of array
m.array_float_element(index, &array, result);   // result = array[index] (floats)
m.array_float_minimum(&array)?;                 // minimum of array (floats)
m.array_float_maximum(&array)?;                 // maximum of array (floats)
```

**Optimization**

The solver automatically uses LP (Linear Programming) techniques for linear constraints with optimization objectives:

```rust
let mut m = Model::default();
let x = m.float(0.0, 1000.0);
let y = m.float(0.0, 1000.0);

// Linear constraints
m.lin_le(&[1.0, 1.0], &[x, y], 100.0);  // x + y <= 100
m.lin_le(&[2.0, 1.0], &[x, y], 150.0);  // 2x + y <= 150

// Maximize objective
let solution = m.maximize(x).expect("Should find optimal solution");
println!("Optimal x: {:?}", solution[x]);
```

For problems with large domains (±1e6) and linear constraints, the LP solver provides dramatic performance improvements (60s+ → <1s).

**FlatZinc/MiniZinc Support**

For FlatZinc `.fzn` file support, use the separate [Zelen](https://github.com/radevgit/zelen) crate. Note: FlatZinc exports should use the new generic API (`lin_eq` instead of `float_lin_eq`). See [Migration Guide](docs/FLATZINC_MIGRATION.md).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
selen = "0.12"
```

## Examples

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

```bash
cargo run --release --example sudoku                 # Classic 9x9 Sudoku solver
cargo run --release --example n_queens               # N-Queens backtracking
cargo run --release --example send_more_money        # Cryptarithmetic puzzle
cargo run --release --example zebra_puzzle           # Logic puzzle solving
```

### Basic Usage

```rust
use selen::prelude::*;

fn main() {
    let mut m = Model::default();

    // Create variables
    let x = m.int(1, 10);       // Integer variable from 1 to 10
    let y = m.int(5, 15);       // Integer variable from 5 to 15

    // Add constraints using the constraint API
    m.new(x.lt(y));             // x must be less than y
    m.new(x.add(y).eq(12));     // x + y must equal 12
    
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

