# CSP Solver

[![Crates.io](https://img.shields.io/crates/v/csp.svg)](https://crates.io/crates/csp)
[![Documentation](https://docs.rs/csp/badge.svg)](https://docs.rs/csp)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A high-performance Constraint Satisfaction Problem (CSP) solver library written in Rust.

We are starting the implementation from scratch.

## Overview

This library provides efficient algorithms and data structures for solving constraint satisfaction problems. CSPs are mathematical problems defined as a set of objects whose state must satisfy a number of constraints or limitations.

## Features

- **Constraint Propagation**: Advanced constraint propagation algorithms for domain reduction
- **Backtracking Search**: Efficient backtracking search with various heuristics
- **Domain Management**: Flexible domain representation and manipulation
- **Multiple Constraint Types**: Support for various constraint types (binary, n-ary, global)
- **Search Strategies**: Multiple search strategies and variable ordering heuristics
- **High Performance**: Optimized for speed with minimal memory allocation

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
csp = "0.0.1"
```

## Quick Start

```rust
use csp::*;

fn main() -> CspResult<()> {
    // Create a simple CSP
    let mut solver = CspSolver::new();
    
    // Add variables with domains
    let var1 = solver.add_variable("x", vec![1, 2, 3])?;
    let var2 = solver.add_variable("y", vec![1, 2, 3])?;
    
    // Add constraints
    solver.add_constraint(NotEqualConstraint::new(var1, var2))?;
    
    // Solve
    if let Some(solution) = solver.solve()? {
        println!("Solution found: {:?}", solution);
    } else {
        println!("No solution exists");
    }
    
    Ok(())
}
```

## Examples

### N-Queens Problem

```rust
use csp::*;

fn solve_n_queens(n: usize) -> CspResult<Option<Solution>> {
    let mut solver = CspSolver::new();
    
    // Create variables for each queen (column position in each row)
    let mut queens = Vec::new();
    for i in 0..n {
        let var = solver.add_variable(
            &format!("queen_{}", i), 
            (1..=n).collect()
        )?;
        queens.push(var);
    }
    
    // Add constraints: no two queens can attack each other
    for i in 0..n {
        for j in i + 1..n {
            // Different columns
            solver.add_constraint(NotEqualConstraint::new(queens[i], queens[j]))?;
            
            // Different diagonals
            solver.add_constraint(DiagonalConstraint::new(queens[i], queens[j], i, j))?;
        }
    }
    
    solver.solve()
}
```

### Graph Coloring

```rust
use csp::*;

fn solve_graph_coloring(graph: &Graph, colors: usize) -> CspResult<Option<Solution>> {
    let mut solver = CspSolver::new();
    
    // Create variable for each node
    let mut nodes = Vec::new();
    for node in graph.nodes() {
        let var = solver.add_variable(
            &format!("node_{}", node.id()), 
            (1..=colors).collect()
        )?;
        nodes.push(var);
    }
    
    // Add constraints: adjacent nodes must have different colors
    for edge in graph.edges() {
        solver.add_constraint(NotEqualConstraint::new(
            nodes[edge.from()], 
            nodes[edge.to()]
        ))?;
    }
    
    solver.solve()
}
```

## Architecture

The library is organized into several key modules:

- **`solver`**: Main CSP solver implementation
- **`variable`**: Variable representation and management
- **`domain`**: Domain operations and constraints
- **`constraints`**: Various constraint types and implementations
- **`propagation`**: Constraint propagation algorithms
- **`search`**: Search strategies and heuristics

## Constraint Types

- **Binary Constraints**: Constraints between two variables
- **N-ary Constraints**: Constraints involving multiple variables
- **Global Constraints**: Specialized constraints for common patterns
- **Custom Constraints**: Support for user-defined constraints

## Search Strategies

- **Backtracking**: Basic chronological backtracking
- **Forward Checking**: Backtracking with look-ahead
- **MAC (Maintaining Arc Consistency)**: Advanced consistency checking
- **Variable Ordering**: Various heuristics (MRV, degree, etc.)
- **Value Ordering**: Least constraining value, etc.

## Performance

The library is designed for high performance:

- Zero-cost abstractions where possible
- Efficient memory usage
- Optimized algorithms
- Benchmarks included in the repository

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Setup

1. Clone the repository
2. Install Rust (latest stable version)
3. Run tests: `cargo test`
4. Run benchmarks: `cargo bench`

### Code Style

This project follows standard Rust conventions:

- Use `rustfmt` for formatting
- Use `clippy` for linting
- Write documentation for public APIs
- Include tests for new functionality

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by classical CSP algorithms and research
- Built with performance and usability in mind
- Thanks to the Rust community for excellent tooling and libraries

## Related Projects

- [constraint](https://crates.io/crates/constraint) - Another Rust CSP library
- [satisfiability](https://crates.io/crates/satisfiability) - SAT solver
- [good_lp](https://crates.io/crates/good_lp) - Linear programming

## References

- Artificial Intelligence: A Modern Approach (Russell & Norvig)
- Constraint Processing (Rina Dechter)
- Handbook of Constraint Programming (Rossi, van Beek, Walsh)

## Status

The library is currently in active development. Features and APIs may change as we refine the implementation and add new functionality.
