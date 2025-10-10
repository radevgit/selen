# Reified Constraint Methods Added to Model - October 10, 2025

## Summary

Added convenience methods on `Model` for posting reified constraints, completing the API consistency work.

## Motivation

Previously, reified constraints were only available as standalone functions:
```rust
// Old way (still works)
eq_reif(&mut model, x, y, b);
```

For consistency with linear constraints (`model.lin_eq(...)`) and better ergonomics, we added Model methods:
```rust
// New way (preferred)
model.eq_reif(x, y, b);
```

## Changes Made

### File Modified
- `src/constraints/api/reified.rs` - Added 6 Model methods

### Methods Added

All methods work with both integer and float variables:

| Method | Description | Example |
|--------|-------------|---------|
| `model.eq_reif(x, y, b)` | b ⇔ (x == y) | Boolean b is true iff x equals y |
| `model.ne_reif(x, y, b)` | b ⇔ (x != y) | Boolean b is true iff x not equals y |
| `model.lt_reif(x, y, b)` | b ⇔ (x < y) | Boolean b is true iff x less than y |
| `model.le_reif(x, y, b)` | b ⇔ (x <= y) | Boolean b is true iff x less or equal y |
| `model.gt_reif(x, y, b)` | b ⇔ (x > y) | Boolean b is true iff x greater than y |
| `model.ge_reif(x, y, b)` | b ⇔ (x >= y) | Boolean b is true iff x greater or equal y |

### Implementation

Each method simply delegates to the existing function:
```rust
impl Model {
    pub fn eq_reif(&mut self, x: VarId, y: VarId, b: VarId) {
        crate::constraints::functions::eq_reif(self, x, y, b);
    }
    // ... similar for ne_reif, lt_reif, le_reif, gt_reif, ge_reif
}
```

## Testing

Created `tests/test_reified_methods.rs` with:
- Compilation tests for all 6 methods
- Tests with integer variables
- Tests with float variables
- ✅ All tests passing

## Usage Examples

### Before (still supported)
```rust
use selen::prelude::*;
use selen::constraints::functions::*;

let mut model = Model::default();
let x = model.int(0, 10);
let y = model.int(0, 10);
let b = model.bool();

eq_reif(&mut model, x, y, b);  // Standalone function
```

### After (preferred)
```rust
use selen::prelude::*;

let mut model = Model::default();
let x = model.int(0, 10);
let y = model.int(0, 10);
let b = model.bool();

model.eq_reif(x, y, b);  // Model method
```

## Benefits

1. **Consistency**: Matches the pattern of linear constraint methods
2. **Discoverability**: IDE autocomplete shows methods on Model
3. **Ergonomics**: More natural flow: `model.do_something()` vs `do_something(&mut model)`
4. **Backwards Compatible**: Old function API still works

## API Consistency Status

✅ **Complete** - All constraint types now available as Model methods:

| Constraint Type | Standalone Functions | Model Methods |
|----------------|---------------------|---------------|
| Linear constraints | ✅ `lin_eq()`, `lin_le()`, `lin_ne()` | ✅ `model.lin_eq()`, etc. |
| Linear reified | ✅ `lin_eq_reif()`, etc. | ✅ `model.lin_eq_reif()`, etc. |
| Simple reified | ✅ `eq_reif()`, `ne_reif()`, etc. | ✅ `model.eq_reif()`, etc. |
| Boolean wrappers | ✅ `bool_lin_eq()`, etc. | ✅ `model.bool_lin_eq()`, etc. |

## Documentation

All methods include:
- Full doc comments with descriptions
- Parameter documentation
- Working code examples in docstrings
- Cross-references to related methods

## Migration Guide

No migration needed - this is purely additive. Users can:
- Continue using standalone functions
- Switch to Model methods for new code
- Mix both styles as preferred

## Future Work

Consider adding Model methods for:
- Global constraints (all_different, circuit, etc.)
- Element constraints
- Table constraints
- Other specialized constraints

For now, these remain as standalone functions, which is fine for less-commonly used constraints.

---

**Status**: ✅ Complete and tested  
**Breaking Changes**: None  
**Deprecations**: None (both APIs supported)  
**Tests**: 2 tests added, all passing
