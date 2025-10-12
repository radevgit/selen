# API Migration Guide: Old Type-Specific Methods → New Generic API

This guide helps you migrate from the old type-specific constraint methods to the new unified generic API introduced in Phase 2.

## Summary of Changes

We removed **24 old type-specific methods** and replaced them with **6 generic methods** that work with both integer and float variables through trait-based dispatch.

### What Was Removed

**Linear Constraint Methods (12 methods):**
- `int_lin_eq()`, `int_lin_le()`, `int_lin_ne()`
- `float_lin_eq()`, `float_lin_le()`, `float_lin_ne()`
- `int_lin_eq_reif()`, `int_lin_le_reif()`, `int_lin_ne_reif()`
- `float_lin_eq_reif()`, `float_lin_le_reif()`, `float_lin_ne_reif()`

**Reified Constraint Methods (12 methods):**
- `int_eq_reif()`, `int_ne_reif()`, `int_lt_reif()`, `int_le_reif()`, `int_gt_reif()`, `int_ge_reif()`
- `float_eq_reif()`, `float_ne_reif()`, `float_lt_reif()`, `float_le_reif()`, `float_gt_reif()`, `float_ge_reif()`

### What's New

**Generic Linear Methods (6 methods):**
- `lin_eq()`, `lin_le()`, `lin_ne()` - work with both i32 and f64 coefficients
- `lin_eq_reif()`, `lin_le_reif()`, `lin_ne_reif()` - reified versions

**Generic Reified Functions:**
- Use `eq_reif()`, `ne_reif()`, `lt_reif()`, `le_reif()`, `gt_reif()`, `ge_reif()` from `constraints::functions`

---

## Migration Examples

### 1. Linear Equality Constraints

**Old API:**
```rust
// Integer linear constraint
model.int_lin_eq(&[2, 3, 1], &[x, y, z], 10);

// Float linear constraint
model.float_lin_eq(&[2.5, 3.7], &[x, y], 10.2);
```

**New API:**
```rust
// Integer linear constraint (type inferred from coefficients)
model.lin_eq(&[2, 3, 1], &[x, y, z], 10);

// Float linear constraint (type inferred from coefficients)
model.lin_eq(&[2.5, 3.7], &[x, y], 10.2);
```

### 2. Linear Inequality Constraints

**Old API:**
```rust
// Integer: 2*x + 3*y <= 100
model.int_lin_le(&[2, 3], &[x, y], 100);

// Float: 1.5*x + 2.5*y <= 50.0
model.float_lin_le(&[1.5, 2.5], &[x, y], 50.0);
```

**New API:**
```rust
// Integer: 2*x + 3*y <= 100
model.lin_le(&[2, 3], &[x, y], 100);

// Float: 1.5*x + 2.5*y <= 50.0
model.lin_le(&[1.5, 2.5], &[x, y], 50.0);
```

### 3. Linear Not-Equal Constraints

**Old API:**
```rust
// Integer
model.int_lin_ne(&[2, 1], &[x, y], 5);

// Float
model.float_lin_ne(&[1.0, 1.0], &[x, y], 3.0);
```

**New API:**
```rust
// Integer
model.lin_ne(&[2, 1], &[x, y], 5);

// Float
model.lin_ne(&[1.0, 1.0], &[x, y], 3.0);
```

### 4. Reified Linear Constraints

**Old API:**
```rust
let b = model.bool_var();

// Integer reified equality
model.int_lin_eq_reif(&[2, 3], &[x, y], 10, b);

// Float reified inequality
model.float_lin_le_reif(&[1.5, 2.5], &[x, y], 50.0, b);
```

**New API:**
```rust
let b = model.bool_var();

// Integer reified equality
model.lin_eq_reif(&[2, 3], &[x, y], 10, b);

// Float reified inequality
model.lin_le_reif(&[1.5, 2.5], &[x, y], 50.0, b);
```

### 5. Reified Comparison Constraints

**Old API:**
```rust
let b = model.bool_var();

// Integer comparisons
model.int_eq_reif(x, y, b);
model.int_ne_reif(x, y, b);
model.int_lt_reif(x, y, b);
model.int_le_reif(x, y, b);
model.int_gt_reif(x, y, b);
model.int_ge_reif(x, y, b);

// Float comparisons
model.float_eq_reif(x, y, b);
model.float_ne_reif(x, y, b);
model.float_lt_reif(x, y, b);
model.float_le_reif(x, y, b);
model.float_gt_reif(x, y, b);
model.float_ge_reif(x, y, b);
```

**New API (using standalone functions):**
```rust
use selen::prelude::*;

let b = model.bool_var();

// Generic reified comparisons (work for both int and float)
eq_reif(&mut model, x, y, b);
ne_reif(&mut model, x, y, b);
lt_reif(&mut model, x, y, b);
le_reif(&mut model, x, y, b);
gt_reif(&mut model, x, y, b);
ge_reif(&mut model, x, y, b);
```

---

## Complete Migration Reference

### Linear Constraint Methods

| Old Method | New Method | Notes |
|------------|------------|-------|
| `int_lin_eq(coeffs, vars, constant)` | `lin_eq(coeffs, vars, constant)` | Type inferred from coefficients |
| `float_lin_eq(coeffs, vars, constant)` | `lin_eq(coeffs, vars, constant)` | Type inferred from coefficients |
| `int_lin_le(coeffs, vars, constant)` | `lin_le(coeffs, vars, constant)` | Type inferred from coefficients |
| `float_lin_le(coeffs, vars, constant)` | `lin_le(coeffs, vars, constant)` | Type inferred from coefficients |
| `int_lin_ne(coeffs, vars, constant)` | `lin_ne(coeffs, vars, constant)` | Type inferred from coefficients |
| `float_lin_ne(coeffs, vars, constant)` | `lin_ne(coeffs, vars, constant)` | Type inferred from coefficients |

### Reified Linear Constraint Methods

| Old Method | New Method | Notes |
|------------|------------|-------|
| `int_lin_eq_reif(coeffs, vars, constant, b)` | `lin_eq_reif(coeffs, vars, constant, b)` | Type inferred from coefficients |
| `float_lin_eq_reif(coeffs, vars, constant, b)` | `lin_eq_reif(coeffs, vars, constant, b)` | Type inferred from coefficients |
| `int_lin_le_reif(coeffs, vars, constant, b)` | `lin_le_reif(coeffs, vars, constant, b)` | Type inferred from coefficients |
| `float_lin_le_reif(coeffs, vars, constant, b)` | `lin_le_reif(coeffs, vars, constant, b)` | Type inferred from coefficients |
| `int_lin_ne_reif(coeffs, vars, constant, b)` | `lin_ne_reif(coeffs, vars, constant, b)` | Type inferred from coefficients |
| `float_lin_ne_reif(coeffs, vars, constant, b)` | `lin_ne_reif(coeffs, vars, constant, b)` | Type inferred from coefficients |

### Reified Comparison Functions

| Old Method | New Function | Import |
|------------|--------------|--------|
| `int_eq_reif(x, y, b)` | `eq_reif(&mut model, x, y, b)` | `use selen::prelude::*;` |
| `float_eq_reif(x, y, b)` | `eq_reif(&mut model, x, y, b)` | `use selen::prelude::*;` |
| `int_ne_reif(x, y, b)` | `ne_reif(&mut model, x, y, b)` | `use selen::prelude::*;` |
| `float_ne_reif(x, y, b)` | `ne_reif(&mut model, x, y, b)` | `use selen::prelude::*;` |
| `int_lt_reif(x, y, b)` | `lt_reif(&mut model, x, y, b)` | `use selen::prelude::*;` |
| `float_lt_reif(x, y, b)` | `lt_reif(&mut model, x, y, b)` | `use selen::prelude::*;` |
| `int_le_reif(x, y, b)` | `le_reif(&mut model, x, y, b)` | `use selen::prelude::*;` |
| `float_le_reif(x, y, b)` | `le_reif(&mut model, x, y, b)` | `use selen::prelude::*;` |
| `int_gt_reif(x, y, b)` | `gt_reif(&mut model, x, y, b)` | `use selen::prelude::*;` |
| `float_gt_reif(x, y, b)` | `gt_reif(&mut model, x, y, b)` | `use selen::prelude::*;` |
| `int_ge_reif(x, y, b)` | `ge_reif(&mut model, x, y, b)` | `use selen::prelude::*;` |
| `float_ge_reif(x, y, b)` | `ge_reif(&mut model, x, y, b)` | `use selen::prelude::*;` |

---

## Technical Details

### How Type Inference Works

The new API uses Rust's trait system for type-based dispatch:

```rust
pub trait LinearCoeff: Copy {
    fn post_lin_eq(model: &mut Model, coeffs: &[Self], vars: &[VarId], constant: Self);
    fn post_lin_le(model: &mut Model, coeffs: &[Self], vars: &[VarId], constant: Self);
    fn post_lin_ne(model: &mut Model, coeffs: &[Self], vars: &[VarId], constant: Self);
    // ... reified versions
}

impl LinearCoeff for i32 { /* ... */ }
impl LinearCoeff for f64 { /* ... */ }
```

The compiler automatically selects the correct implementation based on the type of the coefficients and constant.

### AST Node Creation

All new linear constraint methods now create AST nodes instead of calling propagators directly:

- **Integer constraints** → `ConstraintKind::LinearInt`
- **Float constraints** → `ConstraintKind::LinearFloat`
- **Reified integer** → `ConstraintKind::ReifiedLinearInt`
- **Reified float** → `ConstraintKind::ReifiedLinearFloat`

This enables:
1. **LP solver integration** - Linear constraints are automatically extracted for LP solving
2. **Expression-to-linear conversion** - Complex expressions can be detected and converted to linear form
3. **Better optimization** - The solver can analyze constraint structure before materialization

---

## Benefits of the New API

1. **Less Verbose**: No need to specify type in method name
2. **Type Safety**: Rust's type system ensures correctness
3. **Unified Interface**: Single API for integers and floats
4. **Better Performance**: AST-based approach enables optimizations
5. **LP Integration**: Automatic extraction for linear programming
6. **Future-Proof**: Easier to extend with new constraint types

---

## Need Help?

If you encounter issues during migration:

1. **Check prelude import**: Make sure you have `use selen::prelude::*;`
2. **Type inference**: If ambiguous, explicitly type your arrays: `&[2_i32, 3, 1]` or `&[2.0_f64, 3.0]`
3. **Reified functions**: Import from `selen::constraints::functions` or use prelude

For questions or issues, please file a GitHub issue.
