# Phase 2 Enhancement: Expression-to-Linear AST Conversion

**Date:** October 10, 2025  
**Status:** ✅ COMPLETED

## Overview

Phase 2 has been enhanced with automatic detection and conversion of expression-based linear constraints to linear AST form. This means constraints like `add(mul(x, int(5)), mul(y, int(4))).eq(int(3))` are now automatically recognized as linear and converted to `LinearInt` AST nodes for LP solver integration.

## What Was Added

### Automatic Linear Detection

**New Function:** `try_convert_to_linear_ast()` in `src/runtime_api/mod.rs`

This function analyzes `Binary` constraint AST nodes (comparison constraints with expressions) and detects linear patterns:

```rust
// User writes this (expression-based):
m.new(add(mul(x, int(2)), mul(y, int(3))).eq(int(12)));

// Automatically converted to (linear AST):
ConstraintKind::LinearInt {
    coeffs: vec![2, 3],
    vars: vec![x, y],
    op: ComparisonOp::Eq,
    constant: 12,
}
```

### Linear Pattern Recognition

**New Function:** `try_extract_linear_form()` in `src/runtime_api/mod.rs`

Recursively analyzes expression trees to extract linear form `(coeffs, vars, constant)`:

**Supported Patterns:**
- `Var(x)` → `([1], [x], 0)`
- `Val(5)` → `([], [], 5)`
- `Mul(Var(x), Val(3))` → `([3], [x], 0)`
- `Add(Mul(Var(x), Val(2)), Var(y))` → `([2, 1], [x, y], 0)`
- `Add(Mul(Var(x), Val(2)), Val(5))` → `([2], [x], 5)`
- `Sub(x, y)` → `([1, -1], [x, y], 0)`

**Non-Linear Patterns (rejected):**
- `Mul(Var(x), Var(y))` - Variable × Variable
- `Div(Var(x), Var(y))` - Division is not linear

### Helper Types and Functions

```rust
#[derive(Clone, Copy, Debug)]
enum LinearCoefficient {
    Int(i32),
    Float(f64),
}

fn add_coefficients(a, b) -> LinearCoefficient
fn subtract_coefficients(a, b) -> LinearCoefficient  
fn negate_coefficient(a) -> LinearCoefficient
```

## The Flow

### Before Enhancement
```
add(mul(x, int(5)), mul(y, int(4))).eq(int(3))
    ↓
ConstraintKind::Binary {
    left: Add(Mul(x, 5), Mul(y, 4)),
    op: Eq,
    right: Val(3)
}
    ↓
Materialize as propagators for Add/Mul
    ↓
No LP extraction possible
```

### After Enhancement
```
add(mul(x, int(5)), mul(y, int(4))).eq(int(3))
    ↓
ConstraintKind::Binary { ... }
    ↓
try_convert_to_linear_ast()  ← NEW!
    ↓
ConstraintKind::LinearInt {
    coeffs: [5, 4],
    vars: [x, y],
    op: Eq,
    constant: 3
}
    ↓
LP solver extraction
    ↓
Propagators (fallback)
```

## Examples

### Simple Addition
```rust
m.new(add(x, y).eq(int(10)));
// Converts to: 1*x + 1*y == 10
```

### With Coefficients
```rust
m.new(add(mul(x, int(2)), mul(y, int(3))).eq(int(12)));
// Converts to: 2*x + 3*y == 12
```

### Subtraction
```rust
m.new(sub(x, y).eq(int(5)));
// Converts to: 1*x - 1*y == 5
```

### Complex Expression
```rust
m.new(sub(add(mul(x, int(2)), mul(y, int(3))), z).eq(int(10)));
// Converts to: 2*x + 3*y - 1*z == 10
```

### Float Constraints
```rust
m.new(add(mul(x, float(1.5)), mul(y, float(2.5))).eq(float(10.0)));
// Converts to: 1.5*x + 2.5*y == 10.0 (LinearFloat)
```

### Inequalities
```rust
m.new(add(x, y).le(int(100)));
// Converts to: 1*x + 1*y <= 100
```

## Test Coverage

**New Test File:** `tests/test_expression_to_linear.rs`

6 comprehensive tests:
1. `test_expression_to_linear_simple_add` - Basic x + y == 10
2. `test_expression_to_linear_with_coefficients` - 2*x + 3*y == 12
3. `test_expression_to_linear_inequality` - x + y <= 8
4. `test_expression_to_linear_subtraction` - x - y == 3
5. `test_expression_to_linear_complex` - 2*x + 3*y - z == 10
6. `test_expression_to_linear_float` - 1.5*x + 2.5*y == 10.0

**All tests pass:** ✅ 6/6

## Benefits

### 1. Unified API
Users don't need to choose between expression-based and linear constraint APIs:
```rust
// These are now equivalent internally:
m.new(add(mul(x, int(2)), y).eq(int(10)));  // Expression API
lin_eq(&mut m, &[2, 1], &[x, y], 10);        // Linear API
```

### 2. LP Solver Integration
Expression-based constraints now benefit from LP solver:
```rust
// This can now be optimized by LP solver!
m.new(add(mul(x, int(3)), mul(y, int(4))).le(int(100)));
m.minimize(add(mul(x, int(5)), mul(y, int(6))));
```

### 3. Natural Syntax
Users can write constraints naturally and still get optimal performance:
```rust
// Reads like math, performs like linear constraints
m.new(add(add(mul(x, int(2)), mul(y, int(3))), z).eq(int(50)));
```

### 4. Type Safety
Automatic detection of int vs float:
```rust
// Int coefficients → LinearInt
m.new(add(mul(x, int(2)), y).eq(int(10)));

// Float coefficients → LinearFloat  
m.new(add(mul(x, float(2.5)), y).eq(float(10.0)));

// Mixed → LinearFloat (automatic promotion)
m.new(add(mul(x, int(2)), mul(y, float(1.5))).eq(float(10.0)));
```

## Implementation Details

### Conversion Algorithm

1. **Pattern Matching:** Check if constraint is `Binary` (comparison)
2. **Linear Extraction:** Try to extract linear form from both sides
3. **Combination:** Combine left and right into single linear constraint
4. **Type Detection:** Determine if all coefficients/constant are integers
5. **AST Creation:** Create `LinearInt` or `LinearFloat` AST node

### Type Handling

The algorithm tracks whether coefficients and constants are integers or floats:
- If all are integers → `LinearInt`
- If any are floats → `LinearFloat` (with automatic promotion)

This ensures optimal representation and correct propagator selection.

### Non-Linear Detection

The algorithm returns `None` for non-linear patterns:
- `Var × Var` (quadratic)
- `Var ÷ Var` (rational)
- Complex nested expressions

These fall back to expression-based propagators.

## Bug Fixes

**Fixed:** Constant type detection bug where float constants were incorrectly converted to int constants when coefficients were all integers.

**Before:**
```rust
m.new(x.eq(float(3.14)));
// Incorrectly converted to: LinearInt { coeffs: [1], constant: 0 }
```

**After:**
```rust
m.new(x.eq(float(3.14)));
// Correctly converted to: LinearFloat { coeffs: [1.0], constant: 3.14 }
```

**Fix:** Added check for constant type in addition to coefficient types:
```rust
if !matches!(constant, LinearCoefficient::Int(_)) {
    all_ints = false;
}
```

## Performance Impact

### Minimal Overhead
- Analysis happens once during constraint posting
- No runtime overhead during solving
- Small memory overhead for coefficient tracking

### Performance Gain
- LP solver can now optimize expression-based constraints
- Better pruning from linear constraint propagators
- Potential speedup for optimization problems

## Summary

✅ **Enhancement Complete!**

**What Changed:**
- Expression-based linear constraints auto-convert to linear AST
- Unified API experience (expression vs linear is transparent)
- LP solver benefits extend to expression-based constraints

**Test Results:**
- All 17 tests pass (11 original + 6 new)
- Coverage for int/float, equality/inequality, simple/complex

**Impact:**
- Better user experience (more natural syntax)
- Better performance (LP solver integration)
- Better architecture (unified constraint representation)

---

**Phase 2 Complete with Enhancement!** 🎉
- Linear constraints create AST ✅
- Expression-to-linear conversion ✅
- LP solver integration ready ✅
