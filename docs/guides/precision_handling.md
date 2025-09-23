# Precision Handling in CSP Solver

This document explains the precision-aware constraint boundary optimization system implemented in the CSP solver, which ensures mathematically correct handling of floating-point constraints.

## Table of Contents

1. [Overview](#overview)
2. [The Floating-Point Precision Problem](#the-floating-point-precision-problem)
3. [Architecture](#architecture)
4. [ULP (Unit in the Last Place) Foundation](#ulp-unit-in-the-last-place-foundation)
5. [Constraint Metadata Collection](#constraint-metadata-collection)
6. [Precision-Aware Optimization](#precision-aware-optimization)
7. [Integration with the Solver](#integration-with-the-solver)
8. [Usage Examples](#usage-examples)
9. [API Reference](#api-reference)

## Overview

The precision handling system addresses floating-point arithmetic precision issues in constraint satisfaction problems (CSPs). It consists of two main components:

- **Constraint Metadata Collection**: Tracks all constraints and their properties for analysis
- **Precision-Aware Optimization**: Uses ULP (Unit in the Last Place) arithmetic to ensure mathematically correct constraint boundaries

## The Floating-Point Precision Problem

### Problem Statement

Standard floating-point arithmetic can introduce precision errors that violate constraint semantics:

```rust
// Problem: Strict inequality x < 5.0
let x = 4.999999999999999; // Very close to 5.0
assert!(x < 5.0); // This might fail due to floating-point representation!
```

### Real-World Impact

- **Financial calculations**: Rounding errors can accumulate to significant amounts
- **Engineering simulations**: Small errors can compound into system instability
- **Optimization problems**: "Solutions" might not actually satisfy constraints

### Our Solution

Use IEEE 754 floating-point precision arithmetic to calculate exact constraint boundaries:

```rust
// Solution: Use ULP-aware boundaries
let strict_upper = UlpUtils::strict_upper_bound(5.0); // Largest number < 5.0
let x = strict_upper; // Guaranteed to satisfy x < 5.0
```

## Architecture

The precision handling system consists of several interconnected modules:

```
src/optimization/
├── constraint_metadata.rs    # Constraint tracking and analysis
├── precision_optimizer.rs    # ULP-aware bound optimization
├── precision_propagator.rs   # Solver integration
├── ulp_utils.rs             # IEEE 754 precision utilities
└── precision_handling.rs    # Enhanced integration layer
```

### Data Flow

1. **Constraint Creation** → Metadata collected in `ConstraintRegistry`
2. **Solver Iteration** → `PrecisionBoundaryPropagator` analyzes variables
3. **Bound Optimization** → `PrecisionOptimizer` calculates ULP-aware bounds
4. **Bound Application** → Context API applies precise bounds to variables

## ULP (Unit in the Last Place) Foundation

### What is ULP?

ULP represents the gap between consecutive floating-point numbers. For any floating-point number, the ULP is the difference to the next representable number.

### Core ULP Functions

#### `ulp(value: f64) -> f64`

Calculates the ULP for a given value:

```rust
pub fn ulp(value: f64) -> f64 {
    if value == 0.0 {
        f64::EPSILON  // Special case: smallest positive number
    } else if value.is_infinite() || value.is_nan() {
        f64::NAN  // Invalid cases
    } else {
        // Calculate gap to next representable number
        let bits = value.to_bits();
        let next_bits = if value > 0.0 { bits + 1 } else { bits - 1 };
        let next_value = f64::from_bits(next_bits);
        (next_value - value).abs()
    }
}
```

**Why EPSILON for 0.0?** Because 0.0 is special in IEEE 754. The smallest positive representable number is `f64::EPSILON` (≈ 2.22e-16).

#### `next_float(value: f64) -> f64`

Returns the next representable floating-point number:

```rust
let next = UlpUtils::next_float(5.0);
// next = 5.000000000000001 (approximately)
```

#### `prev_float(value: f64) -> f64`

Returns the previous representable floating-point number:

```rust
let prev = UlpUtils::prev_float(5.0);
// prev = 4.999999999999999 (approximately)
```

#### Strict Boundary Functions

For constraint boundaries:

```rust
// For x < bound: largest value that satisfies the constraint
let max_value = UlpUtils::strict_upper_bound(5.0);

// For x > bound: smallest value that satisfies the constraint  
let min_value = UlpUtils::strict_lower_bound(3.0);
```

## Constraint Metadata Collection

### ConstraintRegistry

The central repository for all constraint information:

```rust
pub struct ConstraintRegistry {
    constraints: HashMap<ConstraintId, ConstraintMetadata>,
    variable_constraints: HashMap<VarId, Vec<ConstraintId>>,
}
```

### ConstraintMetadata

Detailed information about each constraint:

```rust
pub struct ConstraintMetadata {
    pub constraint_type: ConstraintType,
    pub variables: Vec<VarId>,
    pub parameters: ConstraintParameters,
    pub transformations: Vec<TransformationType>,
}
```

### Constraint Types

Type-safe constraint classification:

```rust
pub enum ConstraintType {
    Equality,                    // x = y
    LessThan,                   // x < y
    LessThanOrEquals,           // x ≤ y
    GreaterThan,                // x > y
    GreaterThanOrEquals,        // x ≥ y
    NotEquals,                  // x ≠ y
    AllDifferent,              // all variables different
    Sum { target: f64 },       // sum = target
    LinearCombination { coefficients: Vec<f64>, target: f64 },
    Complex,                   // Multi-step or complex constraints
}
```

### Integration with Propagators

Every propagator method automatically collects metadata:

```rust
// In props/mod.rs
impl PropagatorContext {
    pub fn equals(&mut self, x: impl View, y: impl View) {
        // Create constraint
        let prop = Equals::new(x.into(), y.into());
        
        // Collect metadata
        let metadata = ConstraintMetadata {
            constraint_type: ConstraintType::Equality,
            variables: vec![x.variable(), y.variable()],
            parameters: ConstraintParameters::None,
            transformations: vec![],
        };
        
        self.registry.register_constraint(metadata);
        self.props.add(prop);
    }
}
```

## Precision-Aware Optimization

### PrecisionOptimizer

The core optimization engine:

```rust
pub struct PrecisionOptimizer {
    bound_cache: HashMap<VarId, PrecisionBounds>,
    step_size: f64,
    stats: OptimizationStats,
}
```

### PrecisionBounds

Optimized bounds with precision metadata:

```rust
pub struct PrecisionBounds {
    pub upper_bound: Option<f64>,        // Precision-adjusted upper bound
    pub lower_bound: Option<f64>,        // Precision-adjusted lower bound
    pub precision_adjusted: bool,        // Whether adjustment was applied
    pub original_upper: Option<f64>,     // Original constraint bound
    pub original_lower: Option<f64>,     // Original constraint bound
}
```

### Bound Optimization Process

1. **Analyze Variable Constraints**: Use metadata to understand all constraints affecting a variable
2. **Calculate Effective Bounds**: Combine multiple constraints to find the tightest bounds
3. **Apply ULP Corrections**: Use ULP arithmetic for strict inequalities
4. **Cache Results**: Store optimized bounds for performance

```rust
impl PrecisionOptimizer {
    pub fn optimize_bounds(
        &mut self,
        var_id: VarId,
        registry: &ConstraintRegistry,
        vars: &Vars,
    ) -> Result<PrecisionBounds, String> {
        // 1. Get constraint analysis
        let analysis = registry.analyze_variable_constraints(var_id);
        let (min_bound, max_bound) = analysis.get_effective_bounds();
        
        // 2. Apply precision corrections
        let precision_min = min_bound.map(|b| self.compute_precision_bounds(b, true));
        let precision_max = max_bound.map(|b| self.compute_precision_bounds(b, false));
        
        // 3. Create optimized bounds
        Ok(PrecisionBounds {
            lower_bound: precision_min,
            upper_bound: precision_max,
            precision_adjusted: precision_min.is_some() || precision_max.is_some(),
            original_lower: min_bound,
            original_upper: max_bound,
        })
    }
}
```

## Integration with the Solver

### PrecisionBoundaryPropagator

Integrates precision optimization into the constraint propagation process:

```rust
impl Propagate for PrecisionBoundaryPropagator {
    fn propagate(&mut self, registry: &ConstraintRegistry, ctx: &mut Context) -> Option<()> {
        let mut optimizer = PrecisionOptimizer::new(self.step_size);
        
        for &var_id in &self.variables {
            // Get precision-optimized bounds
            let bounds = optimizer.optimize_bounds(var_id, registry, ctx.vars()).ok()?;
            
            // Apply bounds using Context API
            self.apply_precision_bounds(var_id, &bounds, ctx).ok()?;
        }
        
        Some(())
    }
}
```

### Boundary Detection Heuristic

The `looks_like_constraint_boundary` function helps identify values that likely need precision adjustment:

```rust
fn looks_like_constraint_boundary(&self, value: f64) -> bool {
    // Check if the value is "round" (has few decimal places)
    let rounded = (value * 10.0).round() / 10.0;
    (value - rounded).abs() < f64::EPSILON
}
```

**Examples:**
- `5.5` → `true` (likely a user-defined boundary)
- `10.0` → `true` (likely a user-defined boundary)  
- `5.500000000000001` → `false` (likely floating-point error)

## Usage Examples

### Basic Usage

```rust
use selen::prelude::*;

// Create model with precision handling
let mut model = Model::new();
let x = model.new_var_float(0.0, 10.0);
let y = model.new_var_float(0.0, 10.0);

// Add constraints - metadata is automatically collected
model.less_than(x, 5.0);           // x < 5.0 (strict inequality)
model.greater_than_or_equal(y, x); // y ≥ x

// Enable precision optimization
model.enable_precision_optimization(1e-10);

// Solve - precision boundaries are automatically applied
if let Some(solution) = model.solve() {
    let x_val = solution.value(x);
    let y_val = solution.value(y);
    
    // Guaranteed to satisfy constraints with IEEE 754 precision
    assert!(x_val < 5.0);
    assert!(y_val >= x_val);
}
```

### Advanced Configuration

```rust
// Create precision propagators for specific variables
let precision_propagators = create_precision_propagators(&registry, 1e-12);

// Add to solver
for propagator in precision_propagators {
    model.add_propagator(propagator);
}

// Access optimization statistics
let stats = optimizer.get_stats();
println!("Precision adjustments: {}", stats.precision_adjustments);
println!("Cache hits: {}", stats.cache_hits);
```

### Debugging Precision Issues

```rust
// Enable debug output for precision adjustments
#[cfg(debug_assertions)]
{
    // Automatic logging when bounds are adjusted
    // Output: "Precision adjustment for variable VarId(0): 
    //         original_upper=Some(5.0), new_upper=Some(4.999999999999999)"
}
```

## API Reference

### Core Modules

#### `ulp_utils::UlpUtils`

Static utility methods for ULP calculations:

- `ulp(value: f64) -> f64` - Calculate ULP for a value
- `next_float(value: f64) -> f64` - Next representable number
- `prev_float(value: f64) -> f64` - Previous representable number
- `strict_upper_bound(bound: f64) -> f64` - For `x < bound`
- `strict_lower_bound(bound: f64) -> f64` - For `x > bound`

#### `constraint_metadata::ConstraintRegistry`

Constraint tracking and analysis:

- `register_constraint(metadata: ConstraintMetadata)` - Add constraint
- `analyze_variable_constraints(var_id: VarId) -> VariableConstraintAnalysis` - Analyze variable
- `get_constraint(id: ConstraintId) -> Option<&ConstraintMetadata>` - Get constraint details

#### `precision_optimizer::PrecisionOptimizer`

Bound optimization engine:

- `new(step_size: f64) -> Self` - Create optimizer
- `optimize_bounds(var_id, registry, vars) -> Result<PrecisionBounds, String>` - Optimize bounds
- `get_stats() -> OptimizationStats` - Get performance statistics
- `clear_cache()` - Clear cached bounds

#### `precision_propagator::PrecisionBoundaryPropagator`

Solver integration:

- `new(variables: Vec<VarId>, step_size: f64) -> Self` - Create propagator
- `for_variable(var_id: VarId, step_size: f64) -> Self` - Single variable propagator

### Helper Functions

#### `create_precision_propagators`

```rust
pub fn create_precision_propagators(
    registry: &ConstraintRegistry,
    step_size: f64,
) -> Vec<PrecisionBoundaryPropagator>
```

Creates propagators for all variables involved in floating-point constraints.

## Performance Considerations

### Caching

The system uses intelligent caching to avoid recomputing bounds:

- **Bound Cache**: Stores optimized bounds per variable
- **Cache Invalidation**: Automatically clears when constraints change
- **Statistics**: Track cache hit rates for performance monitoring

### Selective Application

Precision optimization is only applied where needed:

- **Boundary Detection**: Heuristics identify likely constraint boundaries
- **Constraint Type Filtering**: Only applies to inequality constraints
- **Variable Type Checking**: Only processes floating-point variables

### Configuration Options

Tune performance vs. precision:

- **Step Size**: Smaller values = higher precision, slower computation
- **Variable Selection**: Apply only to critical variables
- **Propagator Frequency**: Control when precision checks run

## Best Practices

1. **Use Appropriate Step Sizes**: 
   - Financial: `1e-15` for maximum precision
   - Engineering: `1e-10` for good balance
   - Games/Graphics: `1e-6` for performance

2. **Enable for Critical Constraints**:
   - Focus on strict inequalities (`<`, `>`)
   - Apply to variables with tight bounds
   - Consider constraint interaction complexity

3. **Monitor Performance**:
   - Use `get_stats()` to track optimization overhead
   - Profile constraint propagation performance
   - Adjust configuration based on problem characteristics

4. **Debug with Logging**:
   - Enable debug builds for precision adjustment logging
   - Verify constraint satisfaction in unit tests
   - Use ULP utilities for test assertions

## Conclusion

The precision handling system ensures that floating-point constraints in CSPs are mathematically correct and robust against IEEE 754 floating-point precision issues. By combining constraint metadata collection with ULP-aware optimization, the solver can guarantee that solutions actually satisfy the specified constraints, not just approximate them.

This is particularly important for applications where precision matters: financial calculations, engineering simulations, optimization problems, and any domain where small errors can compound into significant issues.
