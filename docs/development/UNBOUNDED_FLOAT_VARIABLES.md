# Unbounded Float Variables - Implementation Required

**Date**: October 4, 2025  
**Priority**: HIGH  
**Context**: Zelen (FlatZinc parser) integration  
**Issue**: Selen's `model.float(min, max)` does not properly handle unbounded float variables

---

## Problem Statement

When Zelen parses FlatZinc files with unbounded float variables like:

```flatzinc
var float: R;
var float: P;
var 0.0..10.0: I;
```

Zelen currently calls:
```rust
self.model.float(f64::NEG_INFINITY, f64::INFINITY)  // For unbounded
self.model.float(0.0, 10.0)                         // For bounded
```

**Current Behavior**: Zelen must use finite bounds as a workaround:
```rust
self.model.float(-1e9, 1e9)  // Temporary hack
```

**Result**: Solver finds technically valid but meaningless solutions with extreme values:
```
R = -909836065.5737699      # Expected: ~65.78
P = -90909090.90909092      # Expected: ~1000.00
I = 9.999999999999998       # Expected: ~4.0
```

**Expected Behavior** (from Coin-BC solver):
```
Borrowing 1000.00 at 4.0% interest, and repaying 260.00
per quarter for 1 year leaves 65.78 owing
```

---

## Root Cause

This is a **solver responsibility**, not a parser responsibility:

1. **Separation of Concerns**: Zelen should only parse FlatZinc and map to Selen API
2. **Solver Knowledge**: Selen knows its internal float representation and precision limits
3. **Architecture**: Selen already handles integer domain discretization internally

The `Model::float(min, max)` method needs to:
- Accept infinite bounds gracefully
- Infer reasonable finite bounds internally when needed
- Use problem context to determine appropriate ranges

---

## Recommended Solution

### Option 1: Automatic Bound Inference (RECOMMENDED)

Implement intelligent bound inference inside `Model::float()`:

```rust
impl Model {
    pub fn float(&mut self, min: f64, max: f64) -> VarId {
        let (actual_min, actual_max) = if min.is_infinite() || max.is_infinite() {
            // Infer bounds from problem context
            self.infer_float_bounds()
        } else {
            (min, max)
        };
        
        // Create float variable with actual bounds
        self.create_float_variable(actual_min, actual_max)
    }
    
    fn infer_float_bounds(&self) -> (f64, f64) {
        let mut min_bound = 0.0;
        let mut max_bound = 0.0;
        let mut found_any = false;
        
        // Scan all existing float variables for bounded ranges
        for var_id in &self.float_variables {
            if let Some((min, max)) = self.get_float_bounds(var_id) {
                if min.is_finite() && max.is_finite() {
                    min_bound = min_bound.min(min);
                    max_bound = max_bound.max(max);
                    found_any = true;
                }
            }
        }
        
        if found_any {
            // Expand by 1000x to give solver reasonable search space
            let range = (max_bound - min_bound).abs().max(1.0);
            let expansion = range * 1000.0;
            let inferred_min = (min_bound - expansion).max(-1e12);
            let inferred_max = (max_bound + expansion).min(1e12);
            (inferred_min, inferred_max)
        } else {
            // No context available, use conservative defaults
            // This is reasonable for most practical problems
            (-10000.0, 10000.0)
        }
    }
}
```

**Advantages**:
- ✅ Proper separation of concerns
- ✅ Solver maintains control over its internal representation
- ✅ Works automatically for all users (not just Zelen)
- ✅ Can be improved/tuned without changing parser

### Option 2: Lazy Bound Inference (ALTERNATIVE)

Defer bound inference until first constraint is posted:

```rust
impl Model {
    pub fn float(&mut self, min: f64, max: f64) -> VarId {
        // Store unbounded flag
        let var_id = self.create_float_variable_internal(min, max, min.is_infinite() || max.is_infinite());
        
        if min.is_infinite() || max.is_infinite() {
            self.unbounded_float_vars.push(var_id);
        }
        
        var_id
    }
    
    fn finalize_unbounded_floats(&mut self) {
        // Called before solving
        let inferred_bounds = self.infer_float_bounds();
        
        for var_id in &self.unbounded_float_vars {
            self.update_float_bounds(var_id, inferred_bounds.0, inferred_bounds.1);
        }
        
        self.unbounded_float_vars.clear();
    }
}
```

**Advantages**:
- ✅ Can analyze complete model before setting bounds
- ✅ More accurate inference with full problem context

**Disadvantages**:
- ⚠️ More complex implementation
- ⚠️ Requires tracking unbounded variables

---

## Implementation Details

### Current File Locations

**Selen codebase structure**:
```
src/
  model/
    mod.rs              # Model struct and variable creation methods
    constraints.rs      # Constraint posting methods (float_lin_eq, etc.)
  variables/
    mod.rs              # VarId and variable types
    domain.rs           # Domain representations
  constraints/
    propagators/        # Constraint propagators
```

### Where to Make Changes

**File**: `src/model/mod.rs` (or wherever `Model::float()` is defined)

**Current implementation** (approximate):
```rust
pub fn float(&mut self, min: f64, max: f64) -> VarId {
    // Discretize float interval into grid
    let var_id = self.variables.new_float_var(min, max);
    var_id
}
```

**Updated implementation**:
```rust
pub fn float(&mut self, min: f64, max: f64) -> VarId {
    // Handle infinite bounds
    let (actual_min, actual_max) = if min.is_infinite() || max.is_infinite() {
        self.infer_float_bounds_for_unbounded()
    } else {
        (min, max)
    };
    
    // Discretize float interval into grid
    let var_id = self.variables.new_float_var(actual_min, actual_max);
    var_id
}

fn infer_float_bounds_for_unbounded(&self) -> (f64, f64) {
    // Implementation from Option 1 above
    // ...
}
```

---

## Testing Requirements

### Test Case 1: Unbounded Float Variables
```rust
#[test]
fn test_unbounded_float_variables() {
    let mut model = Model::new();
    
    // Should not panic or use infinite bounds
    let x = model.float(f64::NEG_INFINITY, f64::INFINITY);
    let y = model.float(-10.0, 10.0);
    
    // Should infer reasonable bounds from y
    model.float_lin_eq(&[1.0, 1.0], &[x, y], 0.0);
    
    let result = model.solve();
    assert!(result.is_ok());
}
```

### Test Case 2: Loan Problem (Real-World)
```rust
#[test]
fn test_loan_problem_unbounded() {
    let mut model = Model::new();
    
    // Unbounded float variables (principal, balance)
    let p = model.float(f64::NEG_INFINITY, f64::INFINITY);
    let r = model.float(f64::NEG_INFINITY, f64::INFINITY);
    
    // Bounded interest rate
    let i = model.float(0.0, 10.0);
    
    // Constraints (simplified)
    model.float_lin_eq(&[1.0, -0.01], &[p, i], 1000.0);
    
    let solution = model.solve().unwrap();
    
    // Should find reasonable values, not extreme ones
    let p_val = solution.get_float_value(p);
    assert!(p_val > 0.0 && p_val < 100000.0, "Principal should be reasonable");
}
```

### Test Case 3: Mixed Bounded/Unbounded
```rust
#[test]
fn test_mixed_bounded_unbounded() {
    let mut model = Model::new();
    
    let bounded = model.float(0.0, 100.0);
    let unbounded1 = model.float(f64::NEG_INFINITY, f64::INFINITY);
    let unbounded2 = model.float(f64::NEG_INFINITY, f64::INFINITY);
    
    // Unbounded variables should infer from bounded
    model.float_lin_eq(&[1.0, 1.0, 1.0], &[bounded, unbounded1, unbounded2], 0.0);
    
    let result = model.solve();
    assert!(result.is_ok());
}
```

---

## Performance Considerations

1. **Bound Inference Cost**: Should be O(n) where n = number of existing float variables
2. **Memory**: No additional memory overhead (just computing bounds)
3. **Precision**: Ensure expanded bounds don't cause discretization issues
4. **Caching**: Consider caching inferred bounds if multiple unbounded variables created

---

## Alternative: Document Limitation

If implementing bound inference is too complex for now, document the limitation:

```rust
impl Model {
    /// Create a float variable with the given bounds.
    /// 
    /// # Important: Bounded Floats Required
    /// 
    /// Selen requires finite bounds for float variables. If you have an
    /// "unbounded" float in your problem, you must provide reasonable finite
    /// bounds based on problem context.
    /// 
    /// **Common defaults:**
    /// - Financial problems: `-1e6` to `1e6`
    /// - Scientific computing: `-1e9` to `1e9`
    /// - General purpose: `-10000.0` to `10000.0`
    /// 
    /// # Panics
    /// 
    /// Panics if `min` or `max` is infinite or NaN.
    pub fn float(&mut self, min: f64, max: f64) -> VarId {
        assert!(min.is_finite(), "Float variable minimum bound must be finite");
        assert!(max.is_finite(), "Float variable maximum bound must be finite");
        assert!(min < max, "Float variable minimum must be less than maximum");
        
        self.create_float_variable(min, max)
    }
}
```

Then Zelen can handle the inference (less ideal, but functional).

---

## Priority and Timeline

**Priority**: HIGH - Blocks proper FlatZinc float support in Zelen

**Estimated Effort**:
- Option 1 (Inference): 2-4 hours
- Option 2 (Lazy): 4-6 hours
- Documentation only: 30 minutes

**Recommendation**: Start with Option 1 (simple inference), can enhance later if needed.

---

## Current Workaround in Zelen

**File**: `/home/ross/devpublic/zelen/src/mapper.rs` (line ~97)

```rust
Type::Float => {
    // TODO: Selen should handle unbounded floats internally
    // For now, use conservative finite bounds as a workaround
    // This is a solver responsibility, not a parser responsibility
    self.model.float(-1e9, 1e9)  // ❌ TEMPORARY HACK
}
```

Once Selen properly handles unbounded floats, change to:
```rust
Type::Float => self.model.float(f64::NEG_INFINITY, f64::INFINITY)  // ✅ CORRECT
```

---

## Questions to Answer

Before implementing, consider:

1. **What is Selen's float discretization strategy?**
   - Fixed-precision grid?
   - Adaptive precision?
   - Interval arithmetic?

2. **What are reasonable default bounds for Selen's internal representation?**
   - Does discretization work well with ±1e12 range?
   - What's the practical precision limit?

3. **Should inference be per-variable or global?**
   - Global: All unbounded floats get same inferred bounds
   - Per-variable: Each could have different inference based on local constraints

4. **Should this be configurable?**
   - Allow user to override default unbounded behavior?
   - `ModelConfig { unbounded_float_bounds: (-1e6, 1e6) }`?

---

## Success Criteria

✅ `model.float(f64::NEG_INFINITY, f64::INFINITY)` does not panic  
✅ Unbounded float variables get reasonable finite bounds automatically  
✅ Loan problem (from Zelen) produces sensible results  
✅ Performance impact < 1% on existing benchmarks  
✅ Documentation clearly explains unbounded float handling  

---

## Contact

When implementing this in Selen, please update this document with:
- ✅ Implementation approach chosen (Option 1, 2, or other)
- ✅ File locations modified
- ✅ Test cases added
- ✅ Any API changes or breaking changes

Then return to Zelen project and remove the workaround.
