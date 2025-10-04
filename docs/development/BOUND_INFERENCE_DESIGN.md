# Bound Inference Algorithm Design

## Problem Statement

When variables are declared with unbounded or infinite bounds (e.g., `i32::MIN..i32::MAX` or `f64::NEG_INFINITY..f64::INFINITY`), the solver needs to infer reasonable finite bounds based on:
1. Context from other variables in the model
2. Conservative fallback values
3. Domain size constraints (1,000,000 limit for integers)

## General Strategy (Works for Both Integers and Floats)

### Phase 1: Detect Unbounded Variables

A variable is considered "unbounded" if:
- **Integer**: `min == i32::MIN` OR `max == i32::MAX`
- **Float**: `min.is_infinite()` OR `max.is_infinite()` OR `min.is_nan()` OR `max.is_nan()`

### Phase 2: Scan Existing Bounded Variables

1. Collect all bounded variables of the **same type** (int or float)
2. Find their min/max range: `[global_min, global_max]`
3. Calculate the range span: `span = global_max - global_min`

### Phase 3: Infer Bounds

**If bounded variables exist:**
- Expand the range by a safety factor (1000x for exploration)
- `inferred_min = global_min - 1000 * span`
- `inferred_max = global_max + 1000 * span`

**If no bounded variables exist (first variable):**
- Use conservative fallback bounds
- **Integers**: `[-10000, 10000]`
- **Floats**: `[-10000.0, 10000.0]`

### Phase 4: Apply Type-Specific Constraints

**For Integers:**
1. Clamp to valid i32 range: `[max(i32::MIN+1, inferred_min), min(i32::MAX-1, inferred_max)]`
2. Check domain size: `(max - min + 1) <= 1_000_000`
3. **If domain too large BUT bounds were inferred from context**: Allow it (tight inference)
4. **If domain too large from fallback**: Return error

**For Floats:**
1. Clamp to reasonable finite range: `[max(-1e308, inferred_min), min(1e308, inferred_max)]`
2. No strict domain size limit (continuous domain)

## Key Design Decisions

### 1. Why 1000x Expansion Factor (Default)?

- **Logarithmic middle ground**: On log scale, 1000 ≈ √(10 * 100,000), balancing conservation vs exploration
- **Conservative enough**: Handles most optimization problems where unbounded variables explore beyond constrained ones
- **Not too aggressive**: Avoids creating unnecessarily large domains that hit the 1M limit
- **Empirically validated**: Based on CP literature and practical experience

**Why not smaller (e.g., 300x)?**
- May be too conservative for optimization problems
- Unbounded variables often represent derived quantities needing wider ranges

**Why not larger (e.g., 10,000x)?**
- Easily exceeds 1M domain limit for integers
- Slower propagation with larger domains

**Configurable**: Advanced users can tune via `SolverConfig::with_unbounded_inference_factor(factor)`

### 2. Why Allow Tight Inferred Bounds to Exceed Domain Limit?

Consider this scenario:
```rust
let x = m.int(100, 200);        // Bounded: 100 elements
let y = m.int(i32::MIN, i32::MAX); // Unbounded, inferred from x
// Inference: y ∈ [100 - 1000*100, 200 + 1000*100] = [-99900, 100200]
// Domain size: 200,101 elements (< 1M, OK!)
```

But if we have:
```rust
let a = m.int(0, 500_000);     // 500K elements
let b = m.int(i32::MIN, i32::MAX); // Inferred from a
// Inference: b ∈ [0 - 1000*500K, 500K + 1000*500K] 
//             = [-500M, 500.5M] — clamped to i32 range
// This SHOULD succeed because inference is tight to the model
```

**Rule**: If bounds are inferred from existing variables (not fallback), trust the inference even if it exceeds 1M domain limit.

### 3. Why Separate Integer and Float Handling?

- **Different representations**: SparseSet vs FloatInterval
- **Different constraints**: Domain size limit vs precision concerns
- **Different fallback values**: Can use same magnitude but different types

## Implementation Location

**File**: `src/model/factory_internal.rs`

Add new method:
```rust
impl Model {
    /// Infer bounds for unbounded variables based on existing bounded variables
    fn infer_bounds(&self, min: Val, max: Val) -> (Val, Val) {
        // Implementation here
    }
}
```

Modify:
- `new_var_checked()` to call `infer_bounds()` before creating variable

## Testing Strategy

1. **Test unbounded integers**: `int(i32::MIN, i32::MAX)`
2. **Test unbounded floats**: `float(f64::NEG_INFINITY, f64::INFINITY)`
3. **Test mixed bounded/unbounded**: Some vars bounded, others not
4. **Test inference with context**: Create bounded vars first, then unbounded
5. **Test fallback**: First variable is unbounded (no context)
6. **Test domain size limit**: Inferred bounds near/at 1M limit
7. **Test real problems**: Loan problem from UNBOUNDED_FLOAT_VARIABLES.md

## Success Criteria

✅ Unbounded integers inferred correctly  
✅ Unbounded floats inferred correctly  
✅ Mixed scenarios work  
✅ Domain size limit respected for fallback bounds  
✅ Tight inferred bounds allowed even if > 1M  
✅ Loan problem produces sensible results  
✅ No performance regression  
