# Getting Started with CSP Solver

Welcome! This guide will teach you everything you need to know to start solving constraint satisfaction problems with the CSP Solver library.

## 🎯 What is CSP Solving?

**Constraint Satisfaction Problems (CSPs)** are mathematical problems where you need to find values for variables that satisfy a set of constraints or rules.

### Real-World Examples

- **Scheduling**: Assign shifts to employees without conflicts
- **Resource Allocation**: Distribute limited resources optimally  
- **Puzzle Solving**: Sudoku, N-Queens, logic puzzles
- **Configuration**: Product configuration with compatibility rules
- **Planning**: Route planning, task scheduling

### When to Use CSP Solving

✅ **Use CSP when you have:**
- Multiple variables with possible values
- Rules/constraints that must be satisfied
- Need to find feasible solutions or optimize

❌ **Don't use CSP for:**
- Simple calculations or direct formulas
- Problems with continuous optimization only
- When you need approximate/heuristic solutions

## 🚀 Your First CSP Program

Let's solve a simple problem: **Find two numbers where one is less than the other and they sum to 12.**

### Step 1: Set Up Your Project

Add to your `Cargo.toml`:
```toml
```toml
selen = "0.8.0"
```
```

### Step 2: Write Your First Solver

```rust
use selen::prelude::*;

fn main() {
    // 1. Create a new constraint model
    let mut m = Model::default();

    // 2. Create variables with their possible ranges
    let x = m.int(1, 10);       // x can be 1, 2, 3, ..., 10
    let y = m.int(5, 15);       // y can be 5, 6, 7, ..., 15

    // 3. Add constraints (rules that must be satisfied)
    post!(m, x < y);            // x must be less than y
    post!(m, x + y == int(12)); // x + y must equal 12
    
    // 4. Solve the problem
    match m.solve() {
        Ok(solution) => {
            println!("Found solution!");
            println!("x = {:?}", solution[x]);  // x = ValI(1)  
            println!("y = {:?}", solution[y]);  // y = ValI(11)
        }
        Err(e) => {
            println!("No solution found: {:?}", e);
        }
    }
}
```

### Step 3: Run It

```bash
cargo run
```

**Output:**
```
Found solution!
x = ValI(1)
y = ValI(11)
```

🎉 **Congratulations!** You've solved your first constraint satisfaction problem!

## 📊 Understanding Variables

Variables are the unknowns in your problem. The CSP Solver supports several types:

### Integer Variables
```rust
let x = m.int(1, 10);           // x ∈ {1, 2, 3, ..., 10}
let y = m.int(-5, 5);           // y ∈ {-5, -4, ..., 4, 5}
```

### Float Variables
```rust
let price = m.float(0.0, 100.0);    // price ∈ [0.0, 100.0]
let weight = m.float(1.5, 25.0);    // weight ∈ [1.5, 25.0]
```

### Custom Domains (Specific Values)
```rust
let day = m.ints(vec![1, 3, 5, 7]);     // Only odd numbers
let color = m.ints(vec![10, 20, 30]);   // Only these values
```

### Boolean Variables
```rust
let is_active = m.bool();       // is_active ∈ {0, 1} (false, true)
```

## 🔧 Adding Constraints - Two Ways

The CSP Solver provides two syntax styles for building constraints:

### 1. Mathematical Syntax (Recommended for Beginners)

Use the `post!` macro with natural mathematical notation:

```rust
// Basic comparisons
post!(m, x < y);
post!(m, x >= int(5));
post!(m, y != int(0));

// Arithmetic expressions
post!(m, x + y == int(12));
post!(m, x * y <= int(50));
post!(m, x - y >= int(2));

// Complex expressions
post!(m, x + y * int(2) == z);
post!(m, abs(x - y) <= int(3));
```

### 2. Programmatic API (For Dynamic Constraints)

Use method calls for runtime constraint building:

```rust
// Basic comparisons
m.post(x.lt(y));
m.post(x.ge(5));
m.post(y.ne(0));

// Arithmetic expressions  
m.post(x.add(y).eq(12));
m.post(x.mul(y).le(50));
m.post(x.sub(y).ge(2));

// Complex expressions
m.post(x.add(y.mul(2)).eq(z));
let abs_diff = m.abs(x.sub(y));
m.post(abs_diff.le(3));
```

**When to use each:**
- **Mathematical syntax**: Static constraints known at compile time
- **Programmatic API**: Dynamic constraints built from data or user input

## 🔗 Common Constraint Patterns

### Arithmetic Constraints
```rust
post!(m, x + y == int(10));         // Sum equals 10
post!(m, x * y <= int(50));         // Product at most 50
post!(m, x - y >= int(2));          // Difference at least 2
post!(m, x / y == int(3));          // x divided by y equals 3
post!(m, x % int(3) == int(1));     // x mod 3 equals 1
```

### Comparison Constraints
```rust
post!(m, x < y);                    // Less than
post!(m, x <= y);                   // Less than or equal
post!(m, x > int(5));               // Greater than
post!(m, x >= int(0));              // Greater than or equal
post!(m, x == y);                   // Equal
post!(m, x != int(0));              // Not equal
```

### Global Constraints
```rust
// All variables must have different values
let vars = vec![x, y, z];
post!(m, alldiff(vars));

// All variables must have the same value
post!(m, allequal(vars));

// Sum of variables
post!(m, sum(vars) == int(15));

// Element constraint: array[index] = value
let array = vec![m.int(1, 10), m.int(1, 10), m.int(1, 10)];
let index = m.int(0, 2);
let value = m.int(1, 10);
post!(m, element(array, index, value));
```

### Boolean Logic
```rust
// Multiple constraints (implicit AND: all must be true)
post!(m, x > int(5));
post!(m, y < int(10));

// Boolean variables with explicit logic
let condition1 = m.bool();  // Represents: x > 5
let condition2 = m.bool();  // Represents: y < 10
post!(m, condition1 == (x > int(5)));
post!(m, condition2 == (y < int(10)));
post!(m, and([condition1, condition2]));  // Both conditions must be true

// Alternative: Use runtime API for constraint combinations
// m.post(x.gt(5).and(y.lt(10)));  // (x > 5) AND (y < 10)

// OR logic with boolean variables
let options = vec![m.bool(), m.bool(), m.bool()];
post!(m, options[0] == (x == int(1)));
post!(m, options[1] == (x == int(5)));
post!(m, options[2] == (x == int(9)));
post!(m, or(options));  // At least one condition must be true

// NOT logic
let is_equal = m.bool();
post!(m, is_equal == (x == y));
post!(m, not(is_equal));  // x must NOT equal y
```

## 🎯 Solving and Reading Results

### Basic Solving
```rust
match m.solve() {
    Ok(solution) => {
        println!("x = {:?}", solution[x]);
        println!("y = {:?}", solution[y]);
    }
    Err(e) => println!("No solution: {:?}", e),
}
```

### Optimization
```rust
// Find the solution that maximizes x
match m.maximize(x) {
    Ok(solution) => {
        println!("Maximum x = {:?}", solution[x]);
        println!("Corresponding y = {:?}", solution[y]);
    }
    Err(e) => println!("No solution: {:?}", e),
}

// Find the solution that minimizes the sum x + y
let sum_var = m.int(0, 100);
post!(m, sum_var == x + y);
match m.minimize(sum_var) {
    Ok(solution) => {
        println!("Minimum sum = {:?}", solution[sum_var]);
    }
    Err(e) => println!("No solution: {:?}", e),
}
```

### Finding All Solutions
```rust
// Find all possible solutions (useful for small problems)
match m.enumerate() {
    Ok(solutions) => {
        println!("Found {} solutions:", solutions.len());
        for (i, solution) in solutions.iter().enumerate() {
            println!("Solution {}: x = {:?}, y = {:?}", 
                     i + 1, solution[x], solution[y]);
        }
    }
    Err(e) => println!("No solutions: {:?}", e),
}

// Example output:
// Found 3 solutions:
// Solution 1: x = ValI(1), y = ValI(11)
// Solution 2: x = ValI(2), y = ValI(10)  
// Solution 3: x = ValI(3), y = ValI(9)
```

### Reading Solution Values
```rust
if let Ok(solution) = m.solve() {
    // Get the actual integer value
    if let Val::ValI(int_value) = solution[x] {
        println!("x as integer: {}", int_value);
    }
    
    // Get the actual float value  
    if let Val::ValF(float_value) = solution[price] {
        println!("price as float: {}", float_value);
    }
    
    // Debug format (shows type)
    println!("x = {:?}", solution[x]);  // ValI(5)
}
```

## 🛡️ Safety and Configuration

The CSP Solver includes automatic safety features to prevent system crashes:

### Default Safety Limits
```rust
let m = Model::default();
// ↳ Memory limit: 2GB
// ↳ Timeout: 60 seconds
```

### Custom Configuration
```rust
// Conservative limits for shared systems
let config = SolverConfig::default()
    .with_max_memory_mb(512)      // 512MB memory limit
    .with_timeout_seconds(30);    // 30 second timeout

let mut m = Model::with_config(config);

// For dedicated systems
let config = SolverConfig::default()
    .with_max_memory_mb(4096)     // 4GB memory limit  
    .with_timeout_seconds(300);   // 5 minute timeout

let mut m = Model::with_config(config);
```

### Unlimited (Use with Caution)
```rust
// Remove all limits - only for trusted environments!
let config = SolverConfig::unlimited();
let mut m = Model::with_config(config);
```

## 📚 Try These Examples

Now that you understand the basics, try running these progressively more complex examples:

### Beginner Examples
```bash
# Simple arithmetic puzzle
cargo run --release --example send_more_money

# Basic graph coloring
cargo run --release --example graph_coloring
```

### Intermediate Examples
```bash
# Classic N-Queens problem
cargo run --release --example n_queens

# Boolean logic constraints
cargo run --release --example constraint_boolean
```

### Advanced Examples
```bash
# Dynamic constraint building
cargo run --release --example advanced_runtime_api

# Memory management
cargo run --release --example advanced_memory_limits

# Real-world resource allocation
cargo run --release --example app_resource_allocation
```

## 🎓 Complete Example: Simple Scheduling

Let's solve a practical problem: **Schedule 3 tasks (A, B, C) with different durations and constraints.**

```rust
use selen::prelude::*;

fn main() {
    let mut m = Model::default();
    
    // Task start times (0-10 time units)
    let task_a = m.int(0, 10);
    let task_b = m.int(0, 10);  
    let task_c = m.int(0, 10);
    
    // Task durations
    let duration_a = 3;
    let duration_b = 2;
    let duration_c = 4;
    
    // Constraints:
    // 1. Task A must finish before Task B starts
    post!(m, task_a + int(duration_a) <= task_b);
    
    // 2. Task B must finish before Task C starts  
    post!(m, task_b + int(duration_b) <= task_c);
    
    // 3. All tasks must complete by time 10
    post!(m, task_c + int(duration_c) <= int(10));
    
    // Solve: minimize the total schedule length
    let makespan = m.int(0, 15);
    post!(m, makespan == task_c + int(duration_c));
    
    match m.minimize(makespan) {
        Ok(solution) => {
            println!("📅 Optimal Schedule:");
            println!("Task A: starts at {}, ends at {}", 
                     solution[task_a], 
                     if let Val::ValI(start) = solution[task_a] { start + duration_a } else { 0 });
            println!("Task B: starts at {}, ends at {}", 
                     solution[task_b], 
                     if let Val::ValI(start) = solution[task_b] { start + duration_b } else { 0 });
            println!("Task C: starts at {}, ends at {}", 
                     solution[task_c], 
                     if let Val::ValI(start) = solution[task_c] { start + duration_c } else { 0 });
            println!("Total time: {:?}", solution[makespan]);
        }
        Err(e) => println!("❌ Cannot schedule tasks: {:?}", e),
    }
}
```

**Expected Output:**
```
📅 Optimal Schedule:
Task A: starts at 0, ends at 3
Task B: starts at 3, ends at 5  
Task C: starts at 5, ends at 9
Total time: 9
```

## 🎯 Next Steps

Congratulations! You now know the fundamentals of constraint satisfaction problem solving. Here's where to go next:

### 📖 Specialized Guides
- **[Memory Management Guide](memory_management.md)** - Learn about safety limits and resource management
- **[Mathematical Syntax Guide](mathematical_syntax.md)** - Master advanced constraint syntax
- **[Precision Handling Guide](precision_handling.md)** - Working with floating-point precision

### 🔍 API Documentation
- **[API Documentation](https://docs.rs/selen)** - Complete API reference
- **[Examples Directory](../../examples/)** - 15+ complete example programs

### 🧩 Problem Types to Explore
- **Combinatorial Problems**: Sudoku, N-Queens, graph coloring
- **Optimization Problems**: Resource allocation, scheduling, portfolio optimization
- **Logic Puzzles**: Zebra puzzle, cryptarithmetic, Boolean satisfiability

### 💡 Advanced Topics
- **Performance Monitoring**: Solve statistics, memory tracking, and performance analysis
- **Resource Management**: Memory limits, timeouts, and batch processing configuration
- **Precision Control**: Floating-point precision and engineering-scale optimization

---

**Happy constraint solving!** 🎉

If you run into issues, check the [examples](../../examples/) directory for working code, 
or consult the [API documentation](https://docs.rs/selen) for detailed reference material.