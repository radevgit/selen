# Unbounded Variable Inference - Implementation Summary

## Overview

Selen now automatically infers reasonable finite bounds for variables declared with unbounded/infinite bounds. This works for **both integers and floats**.

## Usage

### Basic Usage (Default: 1000x Expansion)

```rust
use selen::prelude::*;

let mut m = Model::default();

// Integers: i32::MIN/MAX are detected as unbounded
let x = m.int(i32::MIN, i32::MAX); 
// → Infers bounds based on context or fallback [-10000, 10000]

// Floats: f64::INFINITY and NaN are detected as unbounded
let y = m.float(f64::NEG_INFINITY, f64::INFINITY);
// → Infers bounds based on context or fallback [-10000.0, 10000.0]
```

### With Context (Expands Existing Variables)

```rust
let mut m = Model::default();

// Create bounded context
let a = m.int(100, 200);  // Context: [100, 200], span = 100

// Create unbounded variable
let x = m.int(i32::MIN, i32::MAX);
// → Infers: [100 - 1000*100, 200 + 1000*100] = [-99900, 100200]
```

### Custom Expansion Factor

```rust
use selen::prelude::*;

// Conservative inference (300x)
let config = SolverConfig::default()
    .with_unbounded_inference_factor(300);
let mut m = Model::with_config(config);

let a = m.int(0, 100);  // Context
let x = m.int(i32::MIN, i32::MAX);
// → Infers: [0 - 300*100, 100 + 300*100] = [-30000, 30100]
```

## Algorithm

### Phase 1: Detection

**Integer unbounded if:**
- `min == i32::MIN` OR `max == i32::MAX`

**Float unbounded if:**
- `min.is_infinite()` OR `max.is_infinite()` OR
- `min.is_nan()` OR `max.is_nan()`

### Phase 2: Context Scanning

- Scans all existing variables of **same type**
- Finds global `[min, max]` range
- Calculates span: `max - min`

### Phase 3: Inference

**With context:**
```
inferred_min = global_min - (factor × span)
inferred_max = global_max + (factor × span)
```

**Without context (fallback):**
- Integers: `[-10000, 10000]`
- Floats: `[-10000.0, 10000.0]`

### Phase 4: Constraints

**Integers:**
1. Clamp to `[i32::MIN + 1, i32::MAX - 1]`
2. If domain > 1M elements AND context-based: clamp to ±500K around center
3. Fallback always respects 1M limit

**Floats:**
1. Clamp to `[-1e308, 1e308]` (finite range)
2. No domain size limit (continuous)

## Configuration

### Expansion Factor

**Default**: 1000x

| Factor | Use Case | Example (context [0, 100]) |
|--------|----------|----------------------------|
| 100 | Very conservative | [-10000, 10100] |
| 300 | Conservative (log middle) | [-30000, 30100] |
| 1000 | **Default** (good for most) | [-100000, 100100] |
| 5000 | Aggressive exploration | [-500000, 500100] |
| 10000 | Very aggressive (may hit limit) | [-1000000, 1000100] |

### Why 1000x?

1. **Logarithmic middle ground**: Between 10x (too small) and 100,000x (way too large)
2. **Empirically validated**: Works well for most CSP/optimization problems
3. **Respects limits**: Rarely hits the 1M domain limit for reasonable contexts
4. **Configurable**: Advanced users can tune for their domain

## Domain Size Limit Handling

### The 1,000,000 Element Limit

Selen enforces a 1M element limit on integer variable domains for performance.

**Strategy**:
- **Fallback bounds**: Always respect limit ([-10000, 10000] = 20,001 elements)
- **Context-based inference**: 
  - If inferred domain ≤ 1M: use it (trust the inference)
  - If inferred domain > 1M: clamp to ±500K around context center

**Example**:
```rust
let a = m.int(0, 10000);  // Context span = 10,000
let x = m.int(i32::MIN, i32::MAX);

// Naive: [0 - 10M, 10000 + 10M] = 20M+ elements (exceeds limit!)
// Actual: Clamps to center ±500K = [5000 - 500K, 5000 + 500K] 
//         = [-495000, 505000] = 1M elements ✓
```

## Type Isolation

- **Integer inference** only looks at integer variables
- **Float inference** only looks at float variables
- No cross-contamination

```rust
let f1 = m.float(1000.0, 2000.0);  // Float context
let x = m.int(i32::MIN, i32::MAX); // Ignores float context
// → Uses fallback [-10000, 10000], not float range

let i1 = m.int(1000, 2000);        // Integer context
let y = m.float(f64::NEG_INFINITY, f64::INFINITY); // Ignores int context
// → Uses fallback [-10000.0, 10000.0], not integer range
```

## Integration Points

### 1. Model Building (Before Validation)

Inference runs in `new_var_checked()` **before** validation, so:
- ✅ Prevents "infinite bounds" validation errors
- ✅ Prevents "domain too large" errors for reasonable inference
- ✅ Transparent to user (just works™)

### 2. FlatZinc/Zelen Integration

Zelen can now safely call:
```rust
// Integers
model.int(i32::MIN, i32::MAX)

// Floats
model.float(f64::NEG_INFINITY, f64::INFINITY)
```

Selen will automatically infer reasonable bounds.

## Files Modified

1. **`src/model/factory_internal.rs`**: Core inference logic in `infer_bounds()`
2. **`src/utils/config.rs`**: Added `unbounded_inference_factor` configuration
3. **`docs/development/BOUND_INFERENCE_DESIGN.md`**: Design documentation

## Testing

See `tests/test_unbounded_variables.rs` for comprehensive tests:
- Fallback inference (no context)
- Context-based inference
- Domain size limit handling
- Type isolation (int vs float)
- Edge cases (NaN, infinity, i32::MIN/MAX boundaries)
- Integration with solving

## Future Enhancements

1. **Per-variable factors**: Allow different factors for different variables
2. **Automatic factor tuning**: Learn from problem characteristics
3. **Statistics**: Track how often inference is used
4. **Warnings**: Optionally warn when using fallback bounds
