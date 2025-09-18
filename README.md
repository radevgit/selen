# CSP Solver

[![Crates.io](https://img.shields.io/crates/v/cspsolver.svg?color=blue)](https://crates.io/crates/cspsolver)
[![Documentation](https://docs.rs/cspsolver/badge.svg)](https://docs.rs/cspsolver)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A Constraint Satisfaction Problem (CSP) solver library written in Rust.

## Overview

This library provides efficient algorithms and data structures for solving constraint satisfaction problems. CSPs are mathematical problems defined as a set of objects whose state must satisfy a number of constraints or limitations.

**Variable Types**: `int`, `float`, mixed constraints

**Constraint Categories**:
- **Mathematical**: `+`, `-`, `*`, `/`, `%`, `abs()`, `min()`, `max()`, `sum()`
- **Comparison**: `==`, `!=`, `<`, `<=`, `>`, `>=` (natural syntax)
- **Boolean Logic**: `and()`, `or()`, `not()` with array syntax `and([a,b,c])` and variadic syntax `and(a,b,c,d)`
- **Global**: `alldiff()`, `allequal()`, element `x[y] = z`

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
cspsolver = "0.5.11"
```


## Examples

```bash
cargo run --release --example sudoku
cargo run --release --example n_queens
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

📊 Statistics: 638 propagations, 54 nodes explored
🔍 Efficiency: 11.8 propagations/node

```



### Basic Usage

```rust
use cspsolver::prelude::*;

fn main() {
    let mut m = Model::default();

    // Create variables with clean syntax
    let x = m.int(1, 10);       // Integer variable
    let y = m.int(5, 15);       // Integer variable  
    let z = m.int(0, 20);       // Integer variable for array compatibility

    // Mathematical constraints using post! macro
    post!(m, x < y);            // Comparison
    post!(m, x + y >= int(10)); // Arithmetic
    post!(m, abs(z) <= int(15)); // Math functions
    
    // Enhanced constraint features
    post!(m, sum([x, y]) == int(12));     // Sum function
    post!(m, x % int(3) != int(0));       // Modulo operations
    
    // Global constraints
    post!(m, alldiff([x, y]));  // All different
    post!(m, allequal([x, y])); // All equal
    
    // Element constraint (array indexing)
    let array = vec![x, y, z];
    let index = m.int(0, 2);
    let value = m.int(1, 20);
    post!(m, array[index] == value); // Natural x[y] = z syntax

    if let Ok(solution) = m.solve() {
        println!("x = {:?}", solution[x]);
        println!("y = {:?}", solution[y]);
        println!("z = {:?}", solution[z]);
    }
}
```

### Advanced Constraint Examples

```rust
use cspsolver::prelude::*;

fn main() {
    let mut m = Model::default();
    let vars = vec![m.int(1, 5), m.int(1, 5), m.int(1, 5)];
    
    // Complex mathematical expressions
    post!(m, sum(vars.clone()) <= int(12));
    post!(m, max(vars.clone()) >= int(3));  // Maximum of vars >= 3
    post!(m, min(vars.clone()) <= int(4));  // Minimum of vars <= 4
    
    // Boolean logic with traditional syntax  
    let a = m.bool();
    let b = m.bool();
    let c = m.bool();
    let d = m.bool();
    
    post!(m, and(a, b));                    // Traditional 2-argument AND
    post!(m, or(a, b));                     // Boolean OR  
    post!(m, not(b));                       // Boolean NOT
    post!(m, and([a, b, c, d]));           // Array syntax for multiple variables
    post!(m, or(a, b, c, d));              // Variadic syntax for multiple variables
    post!(m, not([a, b, c]));              // Array NOT (applies to each variable)
    
    // Mixed type constraints with float
    let float_var = m.float(1.0, 10.0);
    post!(m, abs(float_var) <= float(12.0));
    
    if let Ok(solution) = m.solve() {
        println!("Solution found!");
    }
}
```



## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

