# Precision Handling Quick Reference

## TL;DR

The precision handling system ensures floating-point constraints are mathematically correct by using IEEE 754 ULP (Unit in the Last Place) arithmetic.

## Key Components

### 🔍 Problem
```rust
let x = 4.999999999999999;
assert!(x < 5.0); // ❌ Might fail due to floating-point precision!
```

### ✅ Solution
```rust
let strict_upper = UlpUtils::strict_upper_bound(5.0);
let x = strict_upper; // ✅ Guaranteed x < 5.0
```

## Core Functions

### ULP Utilities (`ulp_utils.rs`)

```rust
// Get the gap between consecutive floating-point numbers
let gap = UlpUtils::ulp(5.0);

// Next/previous representable numbers
let next = UlpUtils::next_float(5.0);    // 5.000000000000001
let prev = UlpUtils::prev_float(5.0);    // 4.999999999999999

// Strict constraint boundaries
let max_for_less_than = UlpUtils::strict_upper_bound(5.0);  // For x < 5.0
let min_for_greater_than = UlpUtils::strict_lower_bound(3.0); // For x > 3.0
```

### Constraint Metadata (`constraint_metadata.rs`)

```rust
// Automatic constraint tracking
model.less_than(x, 5.0);           // ➡️ Metadata: LessThan constraint
model.equals(y, x);                // ➡️ Metadata: Equality constraint

// Analysis
let analysis = registry.analyze_variable_constraints(x);
let (min, max) = analysis.get_effective_bounds();
```

### Precision Optimization (`precision_optimizer.rs`)

```rust
// Optimize bounds using constraint metadata
let mut optimizer = PrecisionOptimizer::new(1e-10);
let bounds = optimizer.optimize_bounds(var_id, &registry, &vars)?;

// Apply optimized bounds
ctx.try_set_max(var_id, bounds.upper_bound);
ctx.try_set_min(var_id, bounds.lower_bound);
```

## Usage Patterns

### Enable Precision Handling

```rust
let mut model = Model::new();

// Add variables and constraints normally
let x = model.new_var_float(0.0, 10.0);
model.less_than(x, 5.0);

// Enable precision optimization
model.enable_precision_optimization(1e-10);
```

### Manual Propagator Setup

```rust
// Create precision propagators for specific variables
let propagators = create_precision_propagators(&registry, 1e-12);

// Add to model
for propagator in propagators {
    model.add_propagator(propagator);
}
```

### Debug Precision Issues

```rust
#[cfg(debug_assertions)]
{
    // Automatic logging when precision bounds are adjusted:
    // "Precision adjustment for variable VarId(0): 
    //  original_upper=Some(5.0), new_upper=Some(4.999999999999999)"
}

// Check optimization statistics
let stats = optimizer.get_stats();
println!("Adjustments made: {}", stats.precision_adjustments);
```

## When to Use

### ✅ Critical for:
- Financial calculations (money, interest rates)
- Engineering simulations (tight tolerances)
- Scientific computing (precise measurements)
- Optimization problems (exact constraint satisfaction)

### ⚠️ Consider for:
- Strict inequalities (`x < bound`, `x > bound`)
- Variables with tight bounds
- Complex constraint interactions

### 🤷 Optional for:
- Games/graphics (performance over precision)
- Approximate algorithms
- Very loose constraints

## Step Size Guidelines

```rust
// Maximum precision (slower)
PrecisionOptimizer::new(1e-15);  // Financial applications

// Good balance (recommended)
PrecisionOptimizer::new(1e-10);  // Engineering, scientific

// Performance focused (faster)
PrecisionOptimizer::new(1e-6);   // Games, graphics, approximations
```

## Constraint Types Affected

```rust
// Precision-critical constraints
model.less_than(x, 5.0);              // x < 5.0  ✅ ULP-adjusted
model.greater_than(x, 3.0);           // x > 3.0  ✅ ULP-adjusted
model.less_than_or_equal(x, 5.0);     // x ≤ 5.0  ⚠️ Usually exact
model.greater_than_or_equal(x, 3.0);  // x ≥ 3.0  ⚠️ Usually exact

// Other constraints
model.equals(x, y);                    // x = y    ⚠️ Context-dependent
model.not_equals(x, 5.0);             // x ≠ 5.0  ✅ ULP-aware
```

## Common Patterns

### Boundary Detection

```rust
// Detects "round" numbers that likely need precision adjustment
fn looks_like_constraint_boundary(value: f64) -> bool {
    let rounded = (value * 10.0).round() / 10.0;
    (value - rounded).abs() < f64::EPSILON
}

// Examples:
looks_like_constraint_boundary(5.5);        // ✅ true (likely boundary)
looks_like_constraint_boundary(10.0);       // ✅ true (likely boundary)
looks_like_constraint_boundary(5.500001);   // ❌ false (computation error)
```

### Testing with ULP

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_strict_inequality() {
        let bound = 5.0;
        let max_value = UlpUtils::strict_upper_bound(bound);
        
        // This is guaranteed to work
        assert!(max_value < bound);
        
        // This might fail without ULP handling
        // assert!(4.999999999999999 < 5.0); // ❌ Risky!
    }
}
```

## Architecture Overview

```
Constraint Creation
       ↓
Metadata Collection (ConstraintRegistry)
       ↓
Solver Iteration  
       ↓
Precision Analysis (PrecisionOptimizer)
       ↓
ULP Boundary Calculation (UlpUtils)
       ↓
Bound Application (Context API)
       ↓
Solution Validation
```

## Performance Tips

1. **Cache Bounds**: Optimizer automatically caches results per variable
2. **Selective Application**: Only applies to floating-point inequality constraints  
3. **Configurable Step Size**: Balance precision vs. performance
4. **Statistics Monitoring**: Use `get_stats()` to track overhead

## Error Handling

```rust
// Bound optimization can fail
match optimizer.optimize_bounds(var_id, &registry, &vars) {
    Ok(bounds) => {
        // Apply bounds...
    },
    Err(msg) => {
        eprintln!("Precision optimization failed: {}", msg);
        // Fall back to original bounds
    }
}
```

---

**For detailed explanations, see [precision_handling.md](precision_handling.md)**
