# Runtime API Performance Guide

## Overview

The Runtime Constraint API provides flexible, data-driven constraint building at the cost of some performance overhead compared to the compile-time `post!` macro. This guide helps you understand the performance characteristics and choose the right API for your use case.

## Performance Measurements

Based on comprehensive benchmarking with 5,000 constraints in release mode:

| Operation | Runtime API | post! macro | Overhead |
|-----------|-------------|-------------|----------|
| Simple constraints (x > 50) | 1.38M/sec | 1.90M/sec | **1.38x** |
| Complex expressions | 70K/sec | 100K/sec | **1.40x** |
| Batch posting | 1.28M/sec | N/A | **Optimal** |
| Global constraints | Minimal overhead | N/A | **~1.1x** |

## Decision Matrix

### Use Runtime API When:

✅ **Data-driven constraints**
```rust
// Loading constraint rules from configuration
for rule in config.constraint_rules {
    model.post(vars[rule.var_index].gt(rule.threshold));
}
```

✅ **Complex mathematical expressions**
```rust
// Business rules with runtime coefficients
let profit = revenue.mul(margin_rate).sub(costs.mul(tax_rate));
model.post(profit.ge(min_profit_target));
```

✅ **Dynamic constraint generation**
```rust
// Constraint patterns based on problem size
for i in 0..problem_size {
    model.post(vars[i].add(vars[i+1]).le(capacity));
}
```

✅ **Global constraint patterns**
```rust
// Highly optimized global constraints
model.alldiff(&schedule_vars);
model.count(&team_assignments, team_id, team_size);
```

✅ **Batch operations**
```rust
// Efficient batch posting
let constraints: Vec<_> = vars.iter().map(|&v| v.gt(0)).collect();
model.post_all(constraints); // Much faster than individual posts
```

### Use post! Macro When:

✅ **Simple, static constraints**
```rust
// Known at compile time
post!(model, x + y <= 100);
post!(model, x > 0);
```

✅ **Performance-critical loops**
```rust
// Maximum performance for basic operations
for var in vars {
    post!(model, var > 0);
}
```

✅ **Direct mathematical notation**
```rust
// Natural mathematical expressions
post!(model, 2*x + 3*y - z == 42);
```

## Optimization Techniques

### 1. Batch Constraint Posting

**❌ Slow: Individual posting**
```rust
for &var in &vars {
    model.post(var.gt(threshold));
}
```

**✅ Fast: Batch posting**
```rust
let constraints: Vec<_> = vars.iter().map(|&var| var.gt(threshold)).collect();
model.post_all(constraints);
```

### 2. Global Constraint Usage

**❌ Slow: Manual implementation**
```rust
for i in 0..vars.len() {
    for j in i+1..vars.len() {
        model.post(vars[i].ne(vars[j]));
    }
}
```

**✅ Fast: Global constraint**
```rust
model.alldiff(&vars);
```

### 3. Expression Building Optimization

**❌ Inefficient: Separate operations**
```rust
let temp1 = x.add(y);
let temp2 = temp1.mul(2);
model.post(temp2.le(100));
```

**✅ Efficient: Chained operations**
```rust
model.post(x.add(y).mul(2).le(100));
```

### 4. Memory Management

**✅ Pre-allocate vectors**
```rust
let mut constraints = Vec::with_capacity(expected_size);
for condition in conditions {
    constraints.push(build_constraint(condition));
}
model.post_all(constraints);
```

## Performance Regression Testing

The codebase includes comprehensive performance regression tests to ensure optimizations don't degrade over time:

```bash
# Run performance regression tests
cargo test --test runtime_api_performance_regression --release

# Run performance benchmarks
cargo run --release --example runtime_api_performance_simple
```

## Real-World Performance Guidelines

### Small Problems (< 1,000 constraints)
- **3-4x overhead** is typical
- Consider `post!` macro for simple constraints
- Runtime API acceptable for complex expressions

### Medium Problems (1,000 - 10,000 constraints)
- **1.4-2x overhead** typical
- Batch posting becomes very beneficial
- Runtime API competitive for most use cases

### Large Problems (> 10,000 constraints)
- **1.2-1.5x overhead** typical
- Batch posting essential
- Runtime API recommended for maintainability

## Memory Usage

The Runtime API optimizations include:

- **Constant folding**: `Val(2) * Val(3)` → `Val(6)` at build time
- **Identity elimination**: `expr * 1` → `expr` 
- **Pre-allocation**: `Vec::with_capacity()` for batch operations
- **Inline functions**: Critical path functions marked `#[inline]`

## Conclusion

The Runtime API provides excellent flexibility for dynamic constraint building with acceptable performance overhead. Use the decision matrix above to choose the right API for each situation, and follow the optimization techniques for best performance.

For performance-critical applications with simple constraints, stick with the `post!` macro. For complex, data-driven constraint building, the Runtime API is the clear choice.