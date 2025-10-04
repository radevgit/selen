# Float Precision Fix Summary

## Problem
Float constraint propagation was failing with small coefficient values (e.g., I=0.04) due to direct float comparisons without tolerance in bound propagation methods.

## Root Cause
The bound propagation methods (`try_set_min`, `try_set_max`) in `src/variables/views.rs` and comparison methods in `src/variables/domain/float_interval.rs` were using direct float comparisons (`>`, `<`, `>=`, `<=`) without accounting for the interval step size tolerance.

This caused constraints to incorrectly fail when:
- Propagating bounds with values close to interval boundaries
- Working with small float coefficients (< 0.1)
- Accumulating rounding errors across multiple propagation steps

## Solution
Added tolerance-based comparisons using `interval.step / 2.0` as the tolerance threshold in:

### Files Modified

#### 1. `src/variables/views.rs` (4 changes)
- **`try_set_min` for `Var::VarF` with `Val::ValF`**: Added tolerance to `min_f > interval.max` check
- **`try_set_min` for `Var::VarF` with `Val::ValI`**: Added tolerance to `min_converted > interval.max` check  
- **`try_set_max` for `Var::VarF` with `Val::ValF`**: Added tolerance to `max_f < interval.min` check
- **`try_set_max` for `Var::VarF` with `Val::ValI`**: Added tolerance to `max_converted < interval.min` check

#### 2. `src/variables/domain/float_interval.rs` (3 changes)
- **`contains`**: Added tolerance to boundary checks `value >= self.min - tolerance && value <= self.max + tolerance`
- **`remove_below`**: Added tolerance to all threshold comparisons
- **`remove_above`**: Added tolerance to all threshold comparisons

## Impact

### Before Fix
```rust
let mut model = Model::default();
let i = model.float(0.0, 10.0);
let x1 = model.float(1.0, 11.0);

model.new(i.eq(0.04));
model.float_lin_eq(&[1.0, -1.0], &[i, x1], -1.0);  // X1 = I + 1 = 1.04

match model.solve() {
    Ok(_) => println!("✓ Solution found"),
    Err(_) => println!("✗ NoSolution"),  // ← INCORRECT!
}
```
**Result**: ✗ NoSolution (WRONG - valid solution exists!)

### After Fix
```rust
// Same code...
match model.solve() {
    Ok(sol) => {
        let i_val: f64 = sol.get(i);
        let x1_val: f64 = sol.get(x1);
        println!("✓ Solution: I={}, X1={}", i_val, x1_val);
        // Output: ✓ Solution: I=0.04, X1=1.04
    }
    Err(_) => println!("✗ NoSolution"),
}
```
**Result**: ✓ Solution found correctly!

## Test Results
- ✅ All 237 library tests pass
- ✅ All 51 float-specific tests pass
- ✅ loan_problem example now works with I=0.04 (B4 ≈ 65.78)
- ✅ No regression in existing functionality

## Technical Details

### Tolerance Formula
```rust
let tolerance = interval.step / 2.0;
```

For default precision (6 decimal places):
- Step size = 1e-6
- Tolerance = 5e-7

This accounts for values that could be rounded to the same step boundary.

### Why This Works
When comparing float bounds during propagation:
- Without tolerance: `0.04000001 > 0.04` → constraint fails
- With tolerance: `0.04000001 > 0.04 + 5e-7` → false, constraint succeeds

The tolerance allows for rounding errors that naturally occur in interval arithmetic while maintaining mathematical correctness.

## Configuration
Users can adjust precision via `ModelConfig`:

```rust
// Default: 6 decimal places (tolerance = 5e-7)
let model = Model::default();

// Higher precision: 8 decimal places (tolerance = 5e-9)
let config = SolverConfig::default().with_float_precision(8);
let model = Model::with_config(config);

// Lower precision: 4 decimal places (tolerance = 5e-5)  
let config = SolverConfig::default().with_float_precision(4);
let model = Model::with_config(config);
```

## Related Documentation
- `src/variables/domain/float_interval.rs` - FloatInterval implementation
- `src/utils/config.rs` - SolverConfig precision settings
- `src/optimization/ulp_utils.rs` - ULP utilities for strict bounds
- `src/variables/core.rs` - `Val::equals_with_intervals` for value comparison
