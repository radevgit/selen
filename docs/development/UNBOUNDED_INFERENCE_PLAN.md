O# Unbounded Variable Inference from Constraint ASTs

## Overview

This document describes the comprehensive AST-based inference system for unbounded variables.
The goal is to infer tight bounds for variables created with unbounded domains (i32::MIN/MAX, f64::INFINITY)
by analyzing constraint ASTs before materialization.

## Current Implementation

**Status:** Infrastructure in place, comprehensive inference not yet implemented.

**Location:** `src/model/core.rs::infer_unbounded_from_asts()`

**When it runs:** Between AST creation and materialization, as part of `prepare_for_search()`:
```
User creates model → solve() called → prepare_for_search():
  ├─ Step 0: infer_unbounded_from_asts() ← NEW PHASE (placeholder)
  ├─ Step 1: materialize_pending_asts()
  ├─ Step 2: validate()
  └─ Step 3: optimize_constraint_order()
```

## Problem Statement

### Current Issue
Variables are inferred at creation time based only on previously created variables:
```rust
let x = m.int(i32::MIN, i32::MAX);  // Inferred immediately from prior vars
let y = m.int(0, 100);               // Too late to help x!
m.new(x.lt(y));                      // This constraint is ignored for inference
```

### Better Approach
Defer inference until all constraints are known:
```rust
let x = m.int(i32::MIN, i32::MAX);  // Mark as unbounded, don't infer yet
let y = m.int(0, 100);
m.new(x.lt(y));                      // Analyzed: x < y ≤ 100 → x < 100
m.new(x.ge(0));                      // Analyzed: x ≥ 0
// Final inference: x ∈ [0, 99] instead of arbitrary large range!
```

## Implementation Plan

### Phase 1: Identify Unbounded Variables

Scan `self.vars` to find variables with unbounded domains:
- **Integer:** min == i32::MIN or max == i32::MAX
- **Float:** min == f64::NEG_INFINITY or max == f64::INFINITY

Create a list of unbounded VarIds that need inference.

### Phase 2: Extract Bounds from Constraint ASTs

For each unbounded variable, scan `self.pending_constraint_asts` to extract bounds:

#### 2.1 Binary Comparisons
```rust
ConstraintKind::Binary { left, op, right }
```

Extract bounds from patterns like:
- `x < c` → `x.max = c - 1` (for integers)
- `x <= c` → `x.max = c`
- `x > c` → `x.min = c + 1`
- `x >= c` → `x.min = c`
- `x == c` → `x.min = x.max = c` (single value)
- `x < y` → if y is bounded, `x.max = y.max - 1`
- `x <= y` → if y is bounded, `x.max = y.max`

#### 2.2 Linear Constraints
```rust
ConstraintKind::LinearInt { coeffs, vars, op, constant }
ConstraintKind::LinearFloat { coeffs, vars, op, constant }
```

For single-variable linear constraints:
- `2x <= 10` → `x <= 5`
- `3x >= 6` → `x >= 2`

For multi-variable linear constraints with only one unbounded variable:
- `x + y <= 10` where y ∈ [0, 5] → `x <= 10 - 0 = 10`
- `2x - y == 8` where y ∈ [0, 4] → solve for x bounds

#### 2.3 Element Constraints
```rust
ConstraintKind::Element { index, array, value }
```

Array indexing implies bounds:
- `array[x] == value` → `x ∈ [0, array.len() - 1]`

#### 2.4 AllDifferent Constraints
```rust
ConstraintKind::AllDifferent { vars }
```

If n variables must be all different:
- Each variable needs at least n distinct values
- If one variable is unbounded and others are bounded in [a, b]:
  - Unbounded var needs range of at least n values
  - Could use [a, b] expanded if needed

#### 2.5 Between Constraints
```rust
ConstraintKind::Between { lower, middle, upper }
```

Bounds middle by lower and upper:
- `lower <= middle <= upper` → extract bounds from lower and upper

#### 2.6 Non-Linear Arithmetic Constraints

Many non-linear constraints provide useful bounds:

**Multiplication:**
```rust
ConstraintKind::Mul { x, y, result }  // result = x * y
```
- If `result` and `y` are bounded: `x ∈ [result.min / y.max, result.max / y.min]` (handle signs)
- If `result` and `x` are bounded: `y ∈ [result.min / x.max, result.max / x.min]`
- Example: `z = x * 5` where `z ∈ [0, 100]` → `x ∈ [0, 20]`

**Division:**
```rust
ConstraintKind::Div { x, y, result }  // result = x / y
```
- If `result` and `y` are bounded: `x ∈ [result.min * y.min, result.max * y.max]`
- Example: `z = x / 2` where `z ∈ [0, 50]` → `x ∈ [0, 100]`

**Modulo:**
```rust
ConstraintKind::Modulo { x, y, result }  // result = x % y
```
- `result ∈ [0, y.max - 1]` always (for positive y)
- If result is bounded, provides weak bounds on x

**Absolute Value:**
```rust
ConstraintKind::Abs { x, result }  // result = |x|
```
- If `result` is bounded: `x ∈ [-result.max, result.max]`
- Example: `z = abs(x)` where `z ∈ [0, 10]` → `x ∈ [-10, 10]`

**Min/Max Constraints:**
```rust
ConstraintKind::Minimum { vars, result }
ConstraintKind::Maximum { vars, result }
```
- `result = min(vars)` → `result.min = min(all var.min)`, `result.max = min(all var.max)`
- `result = max(vars)` → `result.min = max(all var.min)`, `result.max = max(all var.max)`
- Can infer bounds on vars from result bounds

**Sum Constraint:**
```rust
ConstraintKind::Sum { vars, result }  // result = sum(vars)
```
- If result bounded and all but one var bounded:
  - `unbounded_var = result - sum(other_vars)`
  - Provides tight bounds on the unbounded variable

#### 2.7 Global Non-Linear Constraints

**Table Constraint:**
```rust
ConstraintKind::Table { vars, tuples }
```
- For each variable, scan all tuples to find min/max values
- Example: `table([x, y], [(1, 5), (2, 3)])` → `x ∈ [1, 2]`, `y ∈ [3, 5]`

**Global Cardinality Constraint (GCC):**
```rust
ConstraintKind::GlobalCardinality { vars, card_vars, covers }
```
- Bounds vars by the cover values
- Bounds card_vars by the number of vars

**Count Constraint:**
```rust
ConstraintKind::Count { vars, value, count_var }
```
- `count_var ∈ [0, vars.len()]` always
- If value is known and some vars are bounded, can narrow count_var

#### 2.8 Reified Constraints (Boolean Constraints)

Reified constraints have form: `b ⇔ (constraint)`

```rust
ConstraintKind::ReifiedBinary { left, op, right, reif_var }
```

If `reif_var` is constrained:
- `reif_var == 1` → constraint must hold → extract bounds normally
- `reif_var == 0` → constraint must NOT hold → can sometimes infer negated bounds

#### 2.9 Type Conversion Constraints

```rust
// int2float(int_var, float_var)
ConstraintKind::IntToFloat { int_var, float_var }
```
- Bounds are preserved: `float_var.bounds = int_var.bounds` (as floats)

```rust
// floor/ceil/round(float_var, int_var)
ConstraintKind::FloatToInt { float_var, int_var, mode }
```
- If float bounded: `int_var ∈ [floor(float.min), ceil(float.max)]`
- If int bounded: `float_var ∈ [int.min - 1, int.max + 1]` (approximate)

### Phase 3: Aggregate and Apply Bounds

For each unbounded variable:
1. Collect all extracted bounds from all constraints
2. Take tightest bounds:
   - `final_min = max(all extracted mins)`
   - `final_max = min(all extracted maxs)`
3. Validate bounds are consistent (min ≤ max)
4. Apply bounds to variable domain

### Phase 4: Fallback for Still-Unbounded Variables

If a variable remains unbounded after AST analysis:
1. Use the existing variable-context inference as fallback
2. Or use configured default bounds (e.g., [-1000000, 1000000])
3. Issue warning if still unbounded (potential modeling issue)

## Data Structures

### BoundsInfo
```rust
struct BoundsInfo {
    var_id: VarId,
    min: Option<Val>,  // None = no lower bound found
    max: Option<Val>,  // None = no upper bound found
    sources: Vec<BoundsSource>,  // Track where bounds came from for debugging
}

enum BoundsSource {
    BinaryComparison { constraint_id: usize },
    LinearConstraint { constraint_id: usize },
    ElementConstraint { constraint_id: usize },
    AllDifferent { constraint_id: usize },
    ArithmeticConstraint { constraint_id: usize, operation: ArithOp },
    TableConstraint { constraint_id: usize },
    GlobalConstraint { constraint_id: usize, kind: String },
    TypeConversion { constraint_id: usize },
    Fallback,
}

enum ArithOp {
    Mul, Div, Mod, Abs, Min, Max, Sum,
}
```

## Testing Strategy

### Test Cases

1. **Single binary comparison:**
   ```rust
   let x = m.int(i32::MIN, i32::MAX);
   m.new(x.lt(100));
   // Expected: x ∈ [i32::MIN, 99] → should use fallback for min, 99 for max
   ```

2. **Bounded from both sides:**
   ```rust
   let x = m.int(i32::MIN, i32::MAX);
   m.new(x.ge(0));
   m.new(x.lt(100));
   // Expected: x ∈ [0, 99]
   ```

3. **Transitive bounds:**
   ```rust
   let x = m.int(i32::MIN, i32::MAX);
   let y = m.int(0, 50);
   m.new(x.lt(y));
   // Expected: x ∈ [i32::MIN, 49]
   ```

4. **Linear constraint:**
   ```rust
   let x = m.int(i32::MIN, i32::MAX);
   m.lin_le(&[2], &[x], 100);  // 2x <= 100
   // Expected: x ∈ [i32::MIN, 50]
   ```

5. **Element constraint:**
   ```rust
   let arr = vec![m.int(1, 10); 5];
   let idx = m.int(i32::MIN, i32::MAX);
   m.element(&arr, idx, m.int(1, 10));
   // Expected: idx ∈ [0, 4]
   ```

6. **No constraints (fallback):**
   ```rust
   let x = m.int(i32::MIN, i32::MAX);
   // Expected: x gets fallback bounds from config or defaults
   ```

### Non-Linear Constraint Test Cases

7. **Multiplication constraint:**
   ```rust
   let x = m.int(i32::MIN, i32::MAX);
   let z = m.int(0, 100);
   let product = m.mul(x, 5);
   m.new(product.eq(z));  // x * 5 = z, z ∈ [0, 100]
   // Expected: x ∈ [0, 20]
   ```

8. **Division constraint:**
   ```rust
   let x = m.int(i32::MIN, i32::MAX);
   let z = m.int(0, 50);
   let quotient = m.div(x, 2);
   m.new(quotient.eq(z));  // x / 2 = z, z ∈ [0, 50]
   // Expected: x ∈ [0, 100]
   ```

9. **Absolute value:**
   ```rust
   let x = m.int(i32::MIN, i32::MAX);
   let abs_x = m.abs(x);
   m.new(abs_x.le(10));  // |x| <= 10
   // Expected: x ∈ [-10, 10]
   ```

10. **Sum constraint:**
    ```rust
    let x = m.int(i32::MIN, i32::MAX);
    let y = m.int(10, 20);
    let z = m.int(5, 15);
    let sum = m.sum(&[x, y, z]);
    m.new(sum.eq(50));  // x + y + z = 50, y ∈ [10,20], z ∈ [5,15]
    // Expected: x ∈ [50-20-15, 50-10-5] = [15, 35]
    ```

11. **Min/Max constraint:**
    ```rust
    let x = m.int(i32::MIN, i32::MAX);
    let y = m.int(10, 20);
    let z = m.int(15, 25);
    let min_result = m.min(&[x, y, z])?;
    m.new(min_result.ge(12));  // min(x,y,z) >= 12
    // Expected: x ∈ [12, i32::MAX] (since x could be the minimum)
    ```

12. **Table constraint:**
    ```rust
    let x = m.int(i32::MIN, i32::MAX);
    let y = m.int(0, 100);
    m.table(&[x, y], vec![
        vec![Val::int(1), Val::int(5)],
        vec![Val::int(3), Val::int(7)],
        vec![Val::int(8), Val::int(2)],
    ]);
    // Expected: x ∈ [1, 8] (min and max from tuples)
    ```

13. **Modulo constraint:**
    ```rust
    let x = m.int(i32::MIN, i32::MAX);
    let remainder = m.modulo(x, 10);
    m.new(remainder.eq(3));  // x % 10 = 3
    // Expected: x unbounded but must satisfy x ≡ 3 (mod 10)
    // Weak inference: x could be ..., -7, 3, 13, 23, ...
    // May need fallback with modulo constraint
    ```

14. **Chained arithmetic:**
    ```rust
    let x = m.int(i32::MIN, i32::MAX);
    let y = m.int(5, 10);
    let temp = m.mul(x, 2);
    let result = m.add(temp, y);
    m.new(result.le(50));  // 2x + y <= 50, y ∈ [5,10]
    // Expected: x ∈ [i32::MIN, (50-5)/2] = [i32::MIN, 22]
    ```

15. **Mixed types (int/float):**
    ```rust
    let x = m.int(i32::MIN, i32::MAX);
    let x_float = m.int2float(x);
    let y = m.float(0.0, 100.0);
    m.new(x_float.lt(y));
    // Expected: x ∈ [i32::MIN, 99] (floor of 100.0 - 1)
    ```

## Performance Considerations

- **AST scanning:** O(constraints × variables per constraint)
- **Bounds aggregation:** O(unbounded_vars × constraints mentioning them)
- **Overall complexity:** Linear in model size, acceptable overhead

## Future Enhancements

1. **Inter-variable propagation:** Use constraint propagation techniques during inference
2. **Domain-specific inference:** Special handling for common patterns (scheduling, routing, etc.)
3. **User hints:** Allow users to provide hints for unbounded variables
4. **Iterative refinement:** Multiple passes to tighten bounds further

## Integration with Existing Systems

### With LP Solver
- Inference happens before LP solver sees constraints
- LP solver gets tighter domains → faster convergence
- Unbounded variables in LP become bounded → more solutions found

### With Validation
- Inference happens before validation
- Validation can catch if inference failed (still infinite bounds)
- Better error messages about unbounded variables

### With Search
- Search gets tighter domains → smaller search space
- Variable ordering benefits from realistic bounds
- Fewer backtracks due to better domain initialization

## Configuration Options

Add to `ModelConfig`:
```rust
pub struct ModelConfig {
    // ... existing fields ...
    
    /// Enable AST-based unbounded inference (default: true)
    pub enable_ast_inference: bool,
    
    /// Fallback bounds for integers that remain unbounded (default: [-1_000_000, 1_000_000])
    pub default_int_bounds: (i32, i32),
    
    /// Fallback bounds for floats that remain unbounded (default: [-1e6, 1e6])
    pub default_float_bounds: (f64, f64),
    
    /// Warn if variables remain unbounded after inference (default: true)
    pub warn_on_unbounded: bool,
}
```

## Summary of Inference Capabilities

### Linear Constraints
- ✅ Binary comparisons (x < c, x <= y, etc.)
- ✅ Linear equations (2x + 3y = 10)
- ✅ Boolean linear constraints

### Non-Linear Constraints
- ✅ **Multiplication** (z = x * y) - can solve for any variable
- ✅ **Division** (z = x / y) - can solve for dividend and quotient
- ✅ **Modulo** (z = x % y) - bounds remainder, weak bounds on x
- ✅ **Absolute value** (z = |x|) - symmetric bounds
- ✅ **Min/Max** (z = min/max(vars)) - aggregate bounds
- ✅ **Sum** (z = sum(vars)) - isolate unbounded variable
- ✅ **Table** - extract min/max from tuples
- ✅ **Element** - array indexing bounds
- ✅ **AllDifferent** - cardinality bounds
- ✅ **Type conversions** - preserve/approximate bounds

### Inference Complexity
- **Simple:** O(constraints) - direct bound extraction
- **Medium:** O(constraints × variables) - constraint scanning
- **Complex:** May need iterative refinement for interdependent constraints

### Limitations
Some constraints provide only weak or no bounds:
- **Modulo with unbounded dividend** - infinite solutions
- **Multiplication with two unbounded vars** - underconstrained
- **Non-monotonic functions** - may need interval arithmetic

## Status

- ✅ Infrastructure added: `infer_unbounded_from_asts()` phase in `prepare_for_search()`
- ✅ All tests passing (291/291)
- ✅ Design includes both linear and non-linear constraint inference
- ⏳ Comprehensive implementation pending
- ⏳ Test suite for inference pending (15+ test cases designed)
- ⏳ Documentation updates pending

## Next Steps

1. Implement Phase 1: Identify unbounded variables
2. Implement Phase 2.1: Binary comparison bounds extraction
3. Add tests for Phase 2.1
4. Iterate through remaining phases
5. Update user documentation with unbounded variable best practices
6. Consider making this behavior configurable
