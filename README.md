# Selen - CSP Solver

[![Crates.io](https://img.shields.io/crates/v/selen.svg?color=blue)](https://crates.io/crates/selen)
[![Documentation](https://docs.rs/selen/badge.svg)](https://docs.rs/selen)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A Constraint Satisfaction Problem (CSP) solver library written in Rust with zero external dependencies.

## Overview

This library provides efficient algorithms and data structures for solving constraint satisfaction problems. CSPs are mathematical problems defined as a set of objects whose state must satisfy a number of constraints or limitations.


**Variable Types**: `int`, `float`, `bool`, mixed constraints
```rust
let x = m.int(1, 10);                  // Single: integer variable from 1 to 10
let vars = m.ints(5, 1, 10);           // Array: 5 integer variables, each from 1 to 10
let y = m.float(0.0, 100.0);           // Single: float variable from 0.0 to 100.0
let coords = m.floats(3, 0.0, 1.0);    // Array: 3 float variables (x, y, z coordinates)
let b = m.bool();                      // Single: boolean variable (0 or 1)
let flags = m.bools(8);                // Array: 8 boolean variables

// Multidimensional arrays
let matrix = m.ints_2d(3, 4, 1, 10);        // 2D: 3×4 matrix of ints [1..10]
let board = m.floats_2d(5, 5, 0.0, 1.0);    // 2D: 5×5 matrix of floats [0..1]
let flags_2d = m.bools_2d(8, 8);            // 2D: 8×8 matrix of booleans
let cube = m.ints_3d(2, 3, 4, 1, 10);       // 3D: 2×3×4 cube of ints [1..10]
let temps = m.floats_3d(12, 24, 60, -10.0, 50.0); // 3D: 12×24×60 cube of floats
let states = m.bools_3d(4, 5, 6);           // 3D: 4×5×6 cube of booleans

```

**Constraint API**
```rust
// Comparison constraints
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
m.element_2d(&matrix, row_idx, col_idx, value);  // matrix[row][col] == value
m.element_3d(&cube, d_idx, r_idx, c_idx, value); // cube[d][r][c] == value
m.table(&vars, tuples);                // table constraint (valid tuples)
m.table_2d(&matrix, tuples);           // each row matches a valid tuple
m.table_3d(&cube, tuples);             // each row in each layer matches a valid tuple
m.count(&vars, target_var, count_var); // count how many vars equal target_var
m.between(lower, middle, upper);       // lower <= middle <= upper
m.at_least(&vars, value, n);           // at least n vars == value
m.at_most(&vars, value, n);            // at most n vars == value
m.exactly(&vars, value, n);            // exactly n vars == value
m.gcc(&vars, values, counts);          // global cardinality constraint

// Boolean operations (return boolean variables)
let and_result = m.bool_and(&[a, b]);  // a AND b
let or_result = m.bool_or(&[a, b]);    // a OR b
let not_result = m.bool_not(a);        // NOT a
let xor_result = m.bool_xor(a, b);     // a XOR b
m.implies(a, b);                       // a → b (if a then b)
m.bool_clause(&[a, b], &[c]);          // a ∨ b ∨ ¬c (CNF clause)

// Fluent expression building (runtime API)
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

// Boolean linear constraints (for boolean variables)
m.bool_lin_eq(&[1, 1, 1], &[a, b, c], 2);    // a + b + c == 2
m.bool_lin_le(&[1, 1, 1], &[a, b, c], 2);    // a + b + c <= 2
m.bool_lin_ne(&[1, 1, 1], &[a, b, c], 3);    // a + b + c != 3

// Reified constraints (with boolean result) - generic for int and float
m.eq_reif(x, y, b);                    // b ↔ (x == y)
m.ne_reif(x, y, b);                    // b ↔ (x != y)
m.lt_reif(x, y, b);                    // b ↔ (x < y)
m.le_reif(x, y, b);                    // b ↔ (x <= y)
m.gt_reif(x, y, b);                    // b ↔ (x > y)
m.ge_reif(x, y, b);                    // b ↔ (x >= y)
m.lin_eq_reif(&[2, 1], &[x, y], 5, b); // b ↔ (2x + y == 5)
m.lin_le_reif(&[2, 1], &[x, y], 5, b); // b ↔ (2x + y <= 5)
m.lin_ne_reif(&[2, 1], &[x, y], 5, b); // b ↔ (2x + y != 5)

// Boolean linear reified constraints
m.bool_lin_eq_reif(&[1, 1], &[a, b], 2, c);  // c ↔ (a + b == 2)
m.bool_lin_le_reif(&[1, 1], &[a, b], 1, c);  // c ↔ (a + b <= 1)
m.bool_lin_ne_reif(&[1, 1], &[a, b], 0, c);  // c ↔ (a + b != 0)

// Type conversion functions (from constraints::functions)
let float_result = to_float(&mut m, int_var);      // convert int to float
let floor_result = floor(&mut m, float_var);       // floor(float_var)
let ceil_result = ceil(&mut m, float_var);         // ceil(float_var)
let round_result = round(&mut m, float_var);       // round(float_var)
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


**FlatZinc/MiniZinc Support**

For FlatZinc `.fzn` file support, use the separate [Zelen](https://github.com/radevgit/zelen) crate.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
selen = "0.15"
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

