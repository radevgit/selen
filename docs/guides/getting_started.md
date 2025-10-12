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
selen = "0.8.3"
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
    m.new(x.lt(y));             // x must be less than y
    m.new(x.add(y).eq(12));     // x + y must equal 12
    
    // 4. Solve the problem
    match m.solve() {
        Ok(solution) => {
            println!("Found solution!");
            println!("x = {}", solution.get_int(x));  // x = 1
            println!("y = {}", solution.get_int(y));  // y = 11
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
x = 1
y = 11
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
let day = m.intset(vec![1, 3, 5, 7]);     // Only odd numbers
let color = m.intset(vec![10, 20, 30]);   // Only these values
```

### Boolean Variables
```rust
let is_active = m.bool();       // is_active ∈ {0, 1} (false, true)
```

## 🔧 Adding Constraints

Use the runtime API with method calls to build your constraints:

```rust
// Basic comparisons
m.new(x.lt(y));                    // x < y
m.new(x.ge(5));                    // x >= 5
m.new(y.ne(0));                    // y != 0

// Arithmetic expressions
m.new(x.add(y).eq(12));            // x + y == 12
m.new(x.mul(y).le(50));            // x * y <= 50
m.new(x.sub(y).ge(2));             // x - y >= 2
m.new(x.div(y).eq(3));             // x / y == 3
m.new(x.modulo(y).eq(1));          // x % y == 1 (divisor must be variable)

// For constant divisors, create a bounded variable:
let divisor = m.int(5, 5);         // Constant 5
m.new(x.modulo(divisor).eq(2));    // x % 5 == 2

// Complex expressions
m.new(x.add(y.mul(2.0)).eq(z));    // x + y * 2 == z
m.new(x.sub(y).abs().le(3));       // abs(x - y) <= 3

// Complex chaining
m.new(x.mul(2.0).add(y).le(10));   // x * 2 + y <= 10
```

## 🔗 Common Constraint Patterns

### Arithmetic Constraints
```rust
m.new(x.add(y).eq(10));             // Sum equals 10
m.new(x.mul(y).le(50));             // Product at most 50
m.new(x.sub(y).ge(2));              // Difference at least 2
m.new(x.div(y).eq(3));              // x divided by y equals 3
m.new(x.modulo(y).eq(1));           // x % y equals 1 (remainder)
```

### Comparison Constraints
```rust
m.new(x.lt(y));                     // Less than
m.new(x.le(y));                     // Less than or equal
m.new(x.gt(5));                     // Greater than
m.new(x.ge(0));                     // Greater than or equal
m.new(x.eq(y));                     // Equal
m.new(x.ne(0));                     // Not equal
```

### Global Constraints
```rust
// All variables must have different values
let vars = vec![x, y, z];
m.alldiff(&vars);

// Element constraint: array[index] = value
// Example: if index=2, then value must equal array[2]
let array = vec![x, y, z];
let index = m.int(0, 2);        // Can be 0, 1, or 2
let value = m.int(0, 10);
m.elem(&array, index, value);   // value = array[index]

// All must be equal
let equal_vars = vec![a, b, c];
m.alleq(&equal_vars);           // a = b = c
```

### Boolean Logic
```rust
// Multiple constraints (implicit AND: all must be true)
m.new(x.gt(5));
m.new(y.lt(10));
// Both constraints must be satisfied

// Boolean reification (constraint ⇔ boolean variable)
let condition = m.bool();
m.new(condition.eq(x.gt(5)));       // condition is true iff x > 5

// OR logic: at least one constraint must be true
m.new(x.gt(10).or(y.lt(5)));        // x > 10 OR y < 5

// AND logic: both constraints must be true
m.new(x.gt(5).and(y.lt(10)));       // x > 5 AND y < 10

// NOT logic: constraint must be false
m.new(x.eq(5).not());               // x != 5 (same as x.ne(5))

// Complex boolean expressions
m.new(x.gt(5).and(y.lt(10)).or(z.eq(0)));  // (x > 5 AND y < 10) OR z = 0

// Implication: if condition then constraint
// "if x > 5 then y must be < 10"
let cond = m.bool();
m.new(cond.eq(x.gt(5)));            // cond ⇔ (x > 5)
m.new(cond.eq(0).or(y.lt(10)));     // NOT cond OR (y < 10) ≡ cond → (y < 10)
```

## 🎯 Solving and Reading Results

### Basic Solving
```rust
match m.solve() {
    Ok(solution) => {
        println!("x = {}", solution.get_int(x));
        println!("y = {}", solution.get_int(y));
    }
    Err(e) => println!("No solution: {:?}", e),
}
```

### Optimization
```rust
// Find the solution that maximizes x
match m.maximize(x) {
    Ok(solution) => {
        println!("Maximum x = {}", solution.get_int(x));
        println!("Corresponding y = {}", solution.get_int(y));
    }
    Err(e) => println!("No solution: {:?}", e),
}

// Find the solution that minimizes the sum x + y
let sum_var = m.int(0, 100);
m.new(sum_var.eq(x.add(y)));
match m.minimize(sum_var) {
    Ok(solution) => {
        println!("Minimum sum = {}", solution.get_int(sum_var));
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
            println!("Solution {}: x = {}, y = {}", 
                     i + 1, solution.get_int(x), solution.get_int(y));
        }
    }
    Err(e) => println!("No solutions: {:?}", e),
}

// Example output:
// Found 3 solutions:
// Solution 1: x = 1, y = 11
// Solution 2: x = 2, y = 10  
// Solution 3: x = 3, y = 9
```

### Reading Solution Values
```rust
if let Ok(solution) = m.solve() {
    // Get integer value using get_int()
    let x_value = solution.get_int(x);
    println!("x as integer: {}", x_value);
    
    // Get float value using get_float()
    let price_value = solution.get_float(price);
    println!("price as float: {}", price_value);
    
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
// ↳ Timeout: 60000 milliseconds (60 seconds)
```

### Custom Configuration
```rust
// Conservative limits for shared systems
let config = SolverConfig::default()
    .with_max_memory_mb(512)      // 512MB memory limit
    .with_timeout_ms(30000);      // 30000ms = 30 second timeout

let mut m = Model::with_config(config);

// For dedicated systems
let config = SolverConfig::default()
    .with_max_memory_mb(4096)     // 4GB memory limit  
    .with_timeout_ms(300000);     // 300000ms = 5 minute timeout

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
    m.new(task_a.add(duration_a).le(task_b));
    
    // 2. Task B must finish before Task C starts  
    m.new(task_b.add(duration_b).le(task_c));
    
    // 3. All tasks must complete by time 10
    m.new(task_c.add(duration_c).le(10));
    
    // Solve: minimize the total schedule length
    let makespan = m.int(0, 15);
    m.new(makespan.eq(task_c.add(duration_c)));
    
    match m.minimize(makespan) {
        Ok(solution) => {
            println!("📅 Optimal Schedule:");
            let a_start = solution.get_int(task_a);
            let b_start = solution.get_int(task_b);
            let c_start = solution.get_int(task_c);
            println!("Task A: starts at {}, ends at {}", a_start, a_start + duration_a);
            println!("Task B: starts at {}, ends at {}", b_start, b_start + duration_b);
            println!("Task C: starts at {}, ends at {}", c_start, c_start + duration_c);
            println!("Total time: {}", solution.get_int(makespan));
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