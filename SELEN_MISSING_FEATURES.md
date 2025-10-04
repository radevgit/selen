# Missing Features in Selen for Full FlatZinc Support

**Context**: Zelen was tested against ~900 real FlatZinc examples and achieved 95% coverage with integer constraints. However, float constraint support is incomplete in Selen.

**Date**: October 4, 2025  
**Zelen Version**: 0.1.1  
**Selen Version**: 0.9.1

---

## 1. Float Linear Constraints (CRITICAL)

### Missing from Selen's Model API:

Currently Selen has:
- ✅ `int_lin_eq(&[i32], &[VarId], i32)` - Integer linear equality
- ✅ `int_lin_le(&[i32], &[VarId], i32)` - Integer linear ≤

### ✅ IMPLEMENTED (P0 - October 2025):

```rust
// In selen/src/model/constraints.rs
impl Model {
    /// Linear equality constraint with float coefficients ✅ DONE
    /// sum(coefficients[i] * variables[i]) == constant
    pub fn float_lin_eq(&mut self, coefficients: &[f64], variables: &[VarId], constant: f64);
    
    /// Linear inequality constraint with float coefficients ✅ DONE
    /// sum(coefficients[i] * variables[i]) <= constant
    pub fn float_lin_le(&mut self, coefficients: &[f64], variables: &[VarId], constant: f64);
    
    /// Linear inequality constraint with float coefficients (not-equal) ✅ DONE
    /// sum(coefficients[i] * variables[i]) != constant
    pub fn float_lin_ne(&mut self, coefficients: &[f64], variables: &[VarId], constant: f64);
}
```

### ✅ ALSO IMPLEMENTED (P1 - October 2025):

```rust
    /// Reified float linear equality ✅ DONE
    /// reif_var <=> sum(coefficients[i] * variables[i]) == constant
    pub fn float_lin_eq_reif(&mut self, coefficients: &[f64], variables: &[VarId], constant: f64, reif_var: VarId);
    
    /// Reified float linear inequality ✅ DONE
    /// reif_var <=> sum(coefficients[i] * variables[i]) <= constant
    pub fn float_lin_le_reif(&mut self, coefficients: &[f64], variables: &[VarId], constant: f64, reif_var: VarId);
    
    /// Reified float linear not-equal ✅ DONE
    /// reif_var <=> sum(coefficients[i] * variables[i]) != constant
    pub fn float_lin_ne_reif(&mut self, coefficients: &[f64], variables: &[VarId], constant: f64, reif_var: VarId);
```

### Why Critical:

- **FlatZinc Spec Section 4.2.3** lists `float_lin_eq` and `float_lin_le` as standard builtins
- **Used extensively** in optimization problems (loan calculations, physics simulations, etc.)
- **Cannot be decomposed** efficiently - needs native solver support
- **Current workaround** (scaling floats to integers by 1000x) is:
  - ❌ Loses precision
  - ❌ Causes overflow for large values
  - ❌ Incorrect semantics

### Example FlatZinc requiring these:

```flatzinc
% From loan.fzn - financial calculation
array [1..3] of float: coeffs = [1.0, -1.0, 1.0];
var float: B1;
var float: X;
var float: R;
constraint float_lin_eq(coeffs, [B1, X, R], 0.0);
```

**Status**: ✅ **IMPLEMENTED** (October 2025) - `float_lin_eq`, `float_lin_le`, `float_lin_ne` now available in Selen v0.9.1+
- See: `tests/test_float_constraints.rs` for comprehensive tests
- See: `examples/constraint_float_linear.rs` for usage examples

---

## 2. Float Comparison Reified Constraints

### Missing from Selen:

```rust
// In selen/src/model/constraints.rs
impl Model {
    /// Reified float equality: reif_var <=> (x == y)
    pub fn float_eq_reif(&mut self, x: VarId, y: VarId, reif_var: VarId);
    
    /// Reified float not-equal: reif_var <=> (x != y)
    pub fn float_ne_reif(&mut self, x: VarId, y: VarId, reif_var: VarId);
    
    /// Reified float less-than: reif_var <=> (x < y)
    pub fn float_lt_reif(&mut self, x: VarId, y: VarId, reif_var: VarId);
    
    /// Reified float less-equal: reif_var <=> (x <= y)
    pub fn float_le_reif(&mut self, x: VarId, y: VarId, reif_var: VarId);
    
    /// Reified float greater-than: reif_var <=> (x > y)
    pub fn float_gt_reif(&mut self, x: VarId, y: VarId, reif_var: VarId);
    
    /// Reified float greater-equal: reif_var <=> (x >= y)
    pub fn float_ge_reif(&mut self, x: VarId, y: VarId, reif_var: VarId);
}
```

### Why Needed:

- Used in conditional constraints with floats
- Required for proper float constraint reification
- Common in optimization problems

---

## 3. Integer Linear Constraints - Missing Variant

### Missing from Selen:

```rust
impl Model {
    /// Integer linear not-equal constraint
    /// sum(coefficients[i] * variables[i]) != constant
    pub fn int_lin_ne(&mut self, coefficients: &[i32], variables: &[VarId], constant: i32);
}
```

### Current Workaround in Zelen:

```rust
// Works but verbose - requires creating intermediate variables
let scaled_vars: Vec<VarId> = coeffs
    .iter()
    .zip(vars.iter())
    .map(|(&coeff, &var)| self.model.mul(var, Val::ValI(coeff)))
    .collect();
let sum_var = self.model.sum(&scaled_vars);
self.model.c(sum_var).ne(constant);
```

**Better**: Native `int_lin_ne` would be more efficient.

---

## 4. Array Float Constraints ✅ **IMPLEMENTED**

### ~~Missing from Selen~~ **COMPLETED - Section 4** (v0.9.1, January 2025):

```rust
impl Model {
    /// Float array minimum: result = min(array) ✅ DONE
    pub fn array_float_minimum(&mut self, array: &[VarId]) -> Result<VarId>;
    
    /// Float array maximum: result = max(array) ✅ DONE
    pub fn array_float_maximum(&mut self, array: &[VarId]) -> Result<VarId>;
    
    /// Float array element access: result = array[index] ✅ DONE
    /// Where index is an integer variable
    pub fn array_float_element(&mut self, index: VarId, array: &[VarId], result: VarId);
}
```

### Implementation Details:

- ✅ `array_float_minimum` - Returns VarId of minimum, delegates to generic `min()`
- ✅ `array_float_maximum` - Returns VarId of maximum, delegates to generic `max()`
- ✅ `array_float_element` - Delegates to existing `props.element()` propagator
- ✅ All three methods work with existing propagators - no new constraint types needed

**FlatZinc Spec Reference**: Section 4.2.3 lists these as standard builtins added in MiniZinc 2.0.

**Status**: ✅ **IMPLEMENTED** (January 2025) - All three methods now available in Selen v0.9.1+
- See: `tests/test_array_float_constraints.rs` for 21 comprehensive tests
- See: `examples/constraint_array_float.rs` for 7 real-world examples

### Example FlatZinc Usage:

```flatzinc
% Select a price from array based on index
array[1..10] of var float: prices = [10.5, 12.3, 15.0, ...];
var 1..10: selected_idx;
var float: selected_price;
constraint array_float_element(selected_idx, prices, selected_price);
```

---

## 5. Type Conversion Constraints ✅ **IMPLEMENTED**

### ~~Missing from Selen~~ **COMPLETED - Section 5** (v0.9.1, January 2025):

```rust
impl Model {
    /// Convert integer variable to float variable
    /// float_var = int_var (implicit widening conversion)
    pub fn int2float(&mut self, int_var: VarId, float_var: VarId);
    
    /// Convert float to integer using floor rounding
    /// int_var = floor(float_var)
    pub fn float2int_floor(&mut self, float_var: VarId, int_var: VarId);
    
    /// Convert float to integer using ceiling rounding
    /// int_var = ceil(float_var)
    pub fn float2int_ceil(&mut self, float_var: VarId, int_var: VarId);
    
    /// Convert float to integer using standard rounding (round half up)
    /// int_var = round(float_var)
    pub fn float2int_round(&mut self, float_var: VarId, int_var: VarId);
}
```

### Why Needed:

- **Mixed integer/float problems** are common in FlatZinc
- **Required by FlatZinc Spec**: Section 4.2.4 lists these as standard builtins
- **Use cases**:
  - Converting float results to integer indices
  - Rounding fractional resources to whole units
  - Mixed-type optimization problems

### Example FlatZinc Usage:

```flatzinc
% Calculate float weight, round to integer for shipping
var float: weight = 67.5;
var int: rounded_weight;
constraint float2int_round(weight, rounded_weight);  % rounded_weight = 68

% Convert integer count to float for calculation
var 1..100: item_count;
var float: float_count;
constraint int2float(item_count, float_count);
constraint float_lin_eq([2.5], [float_count], 250.0);  % 2.5 * item_count = 250
```

---

## 6. Float Arithmetic Constraints

### Check if Missing from Selen:

```rust
impl Model {
    /// Float absolute value: result = |x|
    pub fn float_abs(&mut self, x: VarId, result: VarId);
    
    /// Float square root: result = sqrt(x)
    /// Requires x >= 0
    pub fn float_sqrt(&mut self, x: VarId, result: VarId);
    
    /// Float power with integer exponent: result = x^n
    pub fn float_pow(&mut self, x: VarId, n: i32, result: VarId);
}
```

### Current Status:

- **Selen's Runtime API** may already have `abs()` - verify if it works with float variables
- **Square root and power** are less common but used in:
  - Physics simulations (kinetic energy, distance calculations)
  - Financial models (compound interest)
  - Geometric constraints

### Example FlatZinc Usage:

```flatzinc
% Distance calculation: dist = sqrt(dx^2 + dy^2)
var float: dx;
var float: dy;
var float: dx_squared;
var float: dy_squared;
var float: sum_squares;
var float: distance;

constraint float_pow(dx, 2, dx_squared);
constraint float_pow(dy, 2, dy_squared);
constraint float_plus(dx_squared, dy_squared, sum_squares);
constraint float_sqrt(sum_squares, distance);
```

**Note**: If these exist in Selen's runtime API but not as explicit constraint methods, document which approach Zelen should use.

---

## 7. Implementation Notes for Selen

### Float Variable Representation

Current Selen implementation:
```rust
// selen/src/variables/domain/float_interval.rs exists
pub fn float(&mut self, min: f64, max: f64) -> VarId
```

This suggests Selen uses **interval-based float domains**. The missing constraints should:

1. **Use interval arithmetic** for propagation
2. **Maintain precision** - no arbitrary scaling
3. **Handle special float cases**:
   - NaN handling
   - Infinity bounds
   - Rounding modes for constraint propagation

### Integration Points

The float linear constraints should integrate with:

```rust
// From selen/src/optimization/float_direct.rs (exists)
// This file suggests float optimization is already partially supported
```

### Performance Considerations

- Float linear constraints are more expensive than integer
- May need **relaxation-based propagation** for efficiency
- Consider **lazy evaluation** for large coefficient arrays

---

## 8. Testing Requirements

Once implemented in Selen, verify with:

### Test Suite from Zelen:

We tested against **~900 FlatZinc files** including:
- MiniZinc tutorial examples
- Optimization problems
- Scheduling problems
- Integer constraint satisfaction

### Float-Specific Tests Needed:

1. **loan.fzn** - Financial calculations (currently fails)
2. **Physics simulations** - Kinematics equations
3. **Resource allocation** - Fractional resources
4. **Continuous optimization** - Minimize/maximize float objectives

### Verification Command:

```bash
# After implementing in Selen
cd zelen
cargo test --release
./target/release/zelen /tmp/loan.fzn  # Should show solution, not UNSATISFIABLE
```

---

## 9. Priority Ranking

### P0 - CRITICAL (Blocks float support):
1. ✅ **float_lin_eq** - Most common float constraint
2. ✅ **float_lin_le** - Required for optimization bounds
3. ✅ **float_lin_ne** - Needed for exclusion constraints

### P1 - HIGH (Common use cases):
4. **float_lin_eq_reif** - Conditional float constraints
5. **float_lin_le_reif** - Conditional bounds
6. **array_float_minimum/maximum/element** - Float aggregations and array access
7. **int2float, float2int_*** - Type conversions for mixed problems

### P2 - MEDIUM (Less common):
8. **float_eq_reif, float_ne_reif, float_lt_reif** - Other reified comparisons
9. **int_lin_ne** - Can work around, but inefficient
10. **float_abs, float_sqrt, float_pow** - Arithmetic (verify if already in runtime API)

---

## 10. Current Zelen Workaround Status

### What Works (using scaling):
- ❌ **float_eq, float_ne, float_lt, float_le** - Use runtime API (`.eq()`, `.ne()`, `.lt()`, `.le()`)
  - Works because these are simple comparisons
  - No scaling needed
  
- ⚠️ **float_lin_eq, float_lin_le, float_lin_ne** - Scale by 1000x to integers
  - **BROKEN**: Loses precision, causes overflow
  - **INCORRECT**: Not proper float semantics

- ❌ **float_plus, float_minus, float_times, float_div** - Use Selen's `add()`, `sub()`, `mul()`, `div()`
  - These work if variables are float type
  - But composed with scaled linear constraints = broken

### What Fails:
- `/tmp/loan.fzn` - Returns UNSATISFIABLE (wrong)
- Any float optimization problem
- Physics simulations
- Financial calculations

---

## 11. Documentation Updates Needed in Selen

Once implemented, update:

```rust
// selen/src/model/constraints.rs
/// # Float Constraints
///
/// Selen supports float variables with interval-based domains.
/// Float linear constraints maintain precision through interval arithmetic.
///
/// ## Example
/// ```rust
/// let x = model.float(0.0, 10.0);
/// let y = model.float(0.0, 10.0);
/// model.float_lin_eq(&[2.5, 1.5], &[x, y], 10.0);  // 2.5*x + 1.5*y = 10
/// ```
```

---

## 12. API Design Recommendation

### Consistent Naming with Integer Constraints:

```rust
// Integer (existing):
model.int_lin_eq(...)
model.int_lin_le(...)
model.int_lin_eq_reif(...)

// Float (proposed - SAME PATTERN):
model.float_lin_eq(...)
model.float_lin_le(...)
model.float_lin_eq_reif(...)
```

### Type Safety:

```rust
// Coefficients should match variable types
pub fn int_lin_eq(&mut self, coefficients: &[i32], ...);   // i32 coeffs for int vars
pub fn float_lin_eq(&mut self, coefficients: &[f64], ...); // f64 coeffs for float vars
```

### Return Error Handling:

```rust
// Consider returning Result for error cases:
pub fn float_lin_eq(&mut self, ...) -> Result<(), ConstraintError> {
    if coefficients.len() != variables.len() {
        return Err(ConstraintError::DimensionMismatch);
    }
    if coefficients.iter().any(|c| c.is_nan()) {
        return Err(ConstraintError::InvalidCoefficient);
    }
    // ... implementation
}
```

---

## Summary

**Zelen Status**: 
- ✅ 95% integer constraint coverage (~900 tests passing)
- ❌ Float constraints incomplete (blocked by Selen limitations)
- ⚠️ Current float workarounds are incorrect

**Selen Requirements**:
- **3 critical methods** needed: `float_lin_eq`, `float_lin_le`, `float_lin_ne`
- **5 high-priority methods** for full float support
- **1 optimization** for integers: native `int_lin_ne`

**Impact**: 
- Once implemented in Selen, Zelen will immediately support float problems correctly
- No changes needed in Zelen parser or mapper (already implemented)
- Just wire up to native Selen methods instead of scaling workaround

**Estimate**: 
- P0 constraints: ~2-3 days implementation + testing in Selen
- Full float support: ~1 week including reified versions

---

## Contact

When these are implemented in Selen, please provide:
1. Selen version number with float support
2. API documentation for the new methods
3. Any performance considerations or limitations

Zelen will be updated to use native methods immediately.
