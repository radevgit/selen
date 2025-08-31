# CSP Solver

[![Crates.io](https://img.shields.io/crates/v/cspsolver.svg?color=blue)](https://crates.io/crates/cspsolver)
[![Documentation](https://docs.rs/cspsolver/badge.svg)](https://docs.rs/cspsolver)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A Constraint Satisfaction Problem (CSP) solver library written in Rust.


## Status

The new implementation follows the design and implementation of [Copper](https://docs.rs/copper/0.1.0/copper/) v0.1.0.

The library is currently in active development. Features and APIs may change as we refine the implementation and add new functionality.

Supported types:
- Int, Float, Mixed

## Overview

This library provides efficient algorithms and data structures for solving constraint satisfaction problems. CSPs are mathematical problems defined as a set of objects whose state must satisfy a number of constraints or limitations.


## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
cspsolver = "0.3.4"
```

## Examples
```bash
cargo run --example pc_builder
cargo run --example resource_allocation
cargo run --example portfolio_optimization
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

    m.less_than(v0.times_pos(float(1.5)), float(5.0));

    let solution = m.maximize(v0).unwrap();
    let x = match solution[v0] {
        Val::ValI(int_val) => int_val,
        _ => panic!("Expected integer value"),
    };

    assert!(x == 3);
    println!("Found optimal value: {}", x);
}
```








## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

