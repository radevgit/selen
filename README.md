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
- **Global**: `alldiff()`, `allequal()`, element `x[y] = z`, `count(vars, value, count)`, `table(vars, tuples)`

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
cargo run --release --example count_demo      # Count constraint demonstrations
cargo run --release --example table_demo      # Table constraint with practical examples
cargo run --release --example magic_square    # Magic squares with enhanced constraints
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
    
    // Count constraint - count how many variables equal a value
    let workers = vec![m.int(1, 3), m.int(1, 3), m.int(1, 3)]; // 1=day, 2=evening, 3=night
    let night_count = m.int(1, 2); // 1-2 workers on night shift
    post!(m, count(workers, int(3), night_count)); // Count night shift workers
    
    // Table constraint - specify valid combinations explicitly  
    let cpu = m.int(1, 2);     // 1=Intel, 2=AMD
    let gpu = m.int(1, 2);     // 1=NVIDIA, 2=AMD
    let compatible_configs = vec![
        vec![int(1), int(1)],  // Intel CPU + NVIDIA GPU
        vec![int(1), int(2)],  // Intel CPU + AMD GPU  
        vec![int(2), int(2)],  // AMD CPU + AMD GPU
        // Note: AMD CPU + NVIDIA GPU not included (incompatible)
    ];
    post!(m, table([cpu, gpu], compatible_configs)); // Only valid combinations allowed
    
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
    
    // Count constraints - powerful cardinality constraints
    let students = vec![m.int(1, 3), m.int(1, 3), m.int(1, 3), m.int(1, 3)]; // 4 students, 3 sections
    let section1_count = m.int(2, 2); // Exactly 2 students in section 1
    let section2_count = m.int(1, 2); // 1-2 students in section 2
    
    post!(m, count(students.clone(), int(1), section1_count)); // Count students in section 1
    post!(m, count(students, int(2), section2_count));         // Count students in section 2
    
    // Table constraints - express complex relationships with lookup tables
    let time = m.int(1, 4);    // Time slots: 1=9AM, 2=11AM, 3=1PM, 4=3PM
    let room = m.int(1, 3);    // Rooms: 1=Lab, 2=Classroom, 3=Auditorium
    let capacity = m.int(10, 100); // Room capacity
    
    // Room availability and capacity table: (time, room, capacity)
    let schedule_table = vec![
        vec![int(1), int(1), int(20)],  // 9AM: Lab has 20 capacity
        vec![int(1), int(2), int(30)],  // 9AM: Classroom has 30 capacity
        vec![int(2), int(2), int(30)],  // 11AM: Classroom has 30 capacity  
        vec![int(2), int(3), int(100)], // 11AM: Auditorium has 100 capacity
        vec![int(3), int(1), int(20)],  // 1PM: Lab has 20 capacity
        vec![int(4), int(3), int(100)], // 3PM: Auditorium has 100 capacity
        // Note: Some time/room combinations unavailable (maintenance, etc.)
    ];
    post!(m, table([time, room, capacity], schedule_table));
    
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

