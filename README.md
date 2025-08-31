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

### PC Building Optimizer

A practical example of constraint optimization - finding the best PC build within budget constraints:

```rust
use cspsolver::prelude::*;

fn main() {
    // Create a model for our PC building problem
    let mut m = Model::default();
    
    // How many monitors: at least 1, at most 3
    let n_monitors = m.new_var_int(1, 3);
    
    // Monitor specifications
    let monitor_price = int(100);
    let monitor_score = int(250);
    
    // GPU options: [budget, mid-range, high-end]
    let gpu_prices = [int(150), int(250), int(500)];
    let gpu_scores = [int(100), int(400), int(800)];
    
    // Binary variables: do we pick each GPU?
    let gpus: Vec<_> = m.new_vars_binary(gpu_prices.len()).collect();
    
    // Calculate total GPU price and score based on selection
    let gpu_price = m.sum_iter(
        gpus.iter()
            .zip(gpu_prices)
            .map(|(gpu, price)| gpu.times(price))
    );
    let gpu_score = m.sum_iter(
        gpus.iter()
            .zip(gpu_scores)
            .map(|(gpu, score)| gpu.times(score))
    );
    
    // Total build price and score
    let total_price = m.add(gpu_price, n_monitors.times(monitor_price));
    let total_score = m.add(gpu_score, n_monitors.times(monitor_score));
    
    // Constraints
    let n_gpus = m.sum(&gpus);
    m.equals(n_gpus, int(1)); // Exactly one GPU
    m.less_than_or_equals(total_price, int(600)); // Budget constraint
    
    // Find optimal solution
    let solution = m.maximize(total_score).unwrap();
    
    println!("Optimal PC Build:");
    println!("Monitors: {}", match solution[n_monitors] { 
        Val::ValI(n) => n,
        _ => 0
    });
    println!("GPU selection: {:?}", solution.get_values_binary(&gpus));
    println!("Total score: {}", match solution[total_score] { 
        Val::ValI(s) => s,
        _ => 0
    });
    println!("Total price: ${}", match solution[total_price] { 
        Val::ValI(p) => p,
        _ => 0
    });
}
```

### Resource Allocation Problem

Allocating tasks to workers with skill and capacity constraints:

```rust
use cspsolver::prelude::*;

fn main() {
    let mut m = Model::default();
    
    // 3 workers, 4 tasks
    let num_workers = 3;
    let num_tasks = 4;
    
    // Worker capacities (hours available)
    let capacities = [int(8), int(6), int(10)];
    
    // Task requirements (hours needed)
    let task_hours = [int(3), int(4), int(2), int(5)];
    
    // Task priorities (higher = more important)
    let priorities = [int(10), int(15), int(5), int(20)];
    
    // Binary variables: worker[i] assigned to task[j]?
    let mut assignments = Vec::new();
    for i in 0..num_workers {
        let mut worker_tasks = Vec::new();
        for j in 0..num_tasks {
            worker_tasks.push(m.new_var_binary());
        }
        assignments.push(worker_tasks);
    }
    
    // Constraint: Each task assigned to exactly one worker
    for j in 0..num_tasks {
        let task_assignments: Vec<_> = assignments
            .iter()
            .map(|worker| worker[j])
            .collect();
        m.equals(m.sum(&task_assignments), int(1));
    }
    
    // Constraint: Worker capacity not exceeded
    for i in 0..num_workers {
        let worker_load = m.sum_iter(
            assignments[i]
                .iter()
                .zip(task_hours.iter())
                .map(|(assigned, hours)| assigned.times(*hours))
        );
        m.less_than_or_equals(worker_load, capacities[i]);
    }
    
    // Objective: Maximize total priority of assigned tasks
    let total_priority = m.sum_iter(
        assignments
            .iter()
            .flatten()
            .zip(priorities.iter().cycle())
            .map(|(assigned, priority)| assigned.times(*priority))
    );
    
    let solution = m.maximize(total_priority).unwrap();
    
    println!("Optimal Task Assignment:");
    for i in 0..num_workers {
        print!("Worker {}: ", i + 1);
        let worker_assignments = solution.get_values_binary(&assignments[i]);
        for (j, &assigned) in worker_assignments.iter().enumerate() {
            if assigned {
                print!("Task{} ", j + 1);
            }
        }
        println!();
    }
    
    println!("Total priority: {}", match solution[total_priority] {
        Val::ValI(p) => p,
        _ => 0
    });
}
```


## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

