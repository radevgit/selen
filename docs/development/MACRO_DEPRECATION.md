# Constraint Macros Deprecation Guide

**Version**: 0.9.3  
**Status**: DEPRECATED  
**Removal Target**: Future release (likely 1.0.0)

## ⚠️ What's Deprecated

The constraint macro system (`post!`, `postall!`, and related macros) is now deprecated due to:
- Difficult maintenance
- Limited capabilities compared to direct API
- Complexity in error messages
- Inconsistent behavior with Rust's type system

## 📋 Deprecated Items

### Modules
- `constraints::macros` - The entire macro system module
- `constraint_macros` - The compatibility re-export module

### Types
- `ConstraintRef` - Used internally by macros

### Macros
- `post!` - Main constraint posting macro
- `postall!` - Batch constraint posting macro
- `post_arithmetic!` - Internal dispatch macro
- `post_comparison!` - Internal dispatch macro
- `post_logical!` - Internal dispatch macro
- `post_global!` - Internal dispatch macro

## 🔄 Migration Guide

### Arithmetic Operations

**Before (Deprecated):**
```rust
post!(model, x + y == z);
post!(model, a - b == c);
post!(model, x * y == z);
```

**After (Recommended):**
```rust
let sum = model.add(x, y);
model.props.equals(sum, z);

let diff = model.sub(a, b);
model.props.equals(diff, c);

let product = model.mul(x, y);
model.props.equals(product, z);
```

### Comparisons

**Before (Deprecated):**
```rust
post!(model, x == y);
post!(model, x < y);
post!(model, x <= y);
```

**After (Recommended):**
```rust
model.props.equals(x, y);
model.props.less_than(x, y);
model.props.less_than_or_equals(x, y);
```

### Sum Constraints

**Before (Deprecated):**
```rust
post!(model, sum([x, y, z]) == 10);
```

**After (Recommended):**
```rust
let total = model.sum(&[x, y, z]);
model.props.equals(total, Val::ValI(10));
```

### Linear Constraints

**Before (Deprecated):**
```rust
// This was difficult/impossible with macros
```

**After (Recommended):**
```rust
// Direct, clean API
model.int_lin_eq(&[2, 3, -1], &[x, y, z], 10); // 2x + 3y - z = 10
model.int_lin_le(&[1, 1, 1], &[x, y, z], 20);  // x + y + z ≤ 20
```

### Global Constraints

**Before (Deprecated):**
```rust
post!(model, alldiff [x, y, z]);
post!(model, element [idx, array, result]);
```

**After (Recommended):**
```rust
model.props.all_different(&[x, y, z]);
model.props.element(array.to_vec(), idx, result);
```

### Boolean Logic

**Before (Deprecated):**
```rust
post!(model, and [a, b, c] == result);
post!(model, or [a, b] == result);
```

**After (Recommended):**
```rust
let result = model.bool_and(&[a, b, c]);
let result = model.bool_or(&[a, b]);
```

### Batch Posting

**Before (Deprecated):**
```rust
postall!(model,
    x < y,
    y < z,
    x + y == 10
);
```

**After (Recommended):**
```rust
model.props.less_than(x, y);
model.props.less_than(y, z);
let sum = model.add(x, y);
model.props.equals(sum, Val::ValI(10));
```

## ✅ Benefits of Direct API

1. **Better Type Safety**: Rust's type system works naturally
2. **Clearer Error Messages**: Standard Rust compilation errors
3. **IDE Support**: Full autocomplete and documentation
4. **More Capabilities**: Access to all constraint variants (reified, linear, etc.)
5. **Easier to Maintain**: Standard Rust code, not macro magic
6. **Better Performance**: No macro expansion overhead

## 📚 API Reference

All constraint methods are now organized in `constraints::api`:

- **Arithmetic**: `constraints::api::arithmetic`
  - `add`, `sub`, `mul`, `div`, `modulo`, `abs`, `min`, `max`, `sum`

- **Boolean**: `constraints::api::boolean`
  - `bool_and`, `bool_or`, `bool_not`, `bool_clause`

- **Reified**: `constraints::api::reified`
  - `int_eq_reif`, `int_ne_reif`, `int_lt_reif`, `int_le_reif`, etc.
  - `float_eq_reif`, `float_ne_reif`, `float_lt_reif`, etc.

- **Linear**: `constraints::api::linear`
  - `int_lin_eq`, `int_lin_le`, `int_lin_ne` (and reified versions)
  - `float_lin_eq`, `float_lin_le`, `float_lin_ne` (and reified versions)

- **Conversion**: `constraints::api::conversion`
  - `int2float`, `float2int_floor`, `float2int_ceil`, `float2int_round`

- **Array**: `constraints::api::array`
  - `array_float_minimum`, `array_float_maximum`, `array_float_element`

## 🗓️ Timeline

- **0.9.3**: Macros marked as deprecated (current)
- **0.10.x**: Macros removed (target)

## 💡 Need Help?

If you have code using the macro system and need help migrating:
1. Check the examples in `examples/` directory for direct API usage
2. See the constraint API documentation in `src/constraints/api/`
3. The deprecation warnings show the recommended alternatives
4. All macro functionality is available (and more) through the direct API
