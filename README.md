# CSP Solver

[![Crates.io](https://img.shields.io/crates/v/cspsolver.svg?color=blue)](https://crates.io/crates/cspsolver)
[![Documentation](https://docs.rs/cspsolver/badge.svg)](https://docs.rs/cspsolver)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A Constraint Satisfaction Problem (CSP) solver library written in Rust.


## Status

The new implementation follows the design and implementation of [Copper](https://docs.rs/copper/0.1.0/copper/) v0.1.0.

The library is currently in active development. Features and APIs may change as we refine the implementation and add new functionality.

- Int type works as in Copper
- Added **float** type.

## Overview

This library provides efficient algorithms and data structures for solving constraint satisfaction problems. CSPs are mathematical problems defined as a set of objects whose state must satisfy a number of constraints or limitations.


## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
cspsolver = "0.3.3-alpha"
```

## Examples

```bash
cargo run --example pc_builder
cargo run --example resource_allocation
cargo run --example portfolio_optimization
```




## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

