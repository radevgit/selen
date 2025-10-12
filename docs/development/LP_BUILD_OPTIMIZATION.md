# LP BUILD Optimization - October 10, 2025

## Problem
The `agprice_full.rs` problem (254 variables, 247 constraints) was timing out during the **LP BUILD phase** (>60 seconds).

## Root Cause Analysis

### Bottleneck 1: Debug Output
The LP BUILD phase was printing **verbose debug output** for every constraint:
```rust
eprintln!("LP BUILD: Constraint row = {:?}, rhs = {}", row, rhs_adjusted);
```

For agprice:
- 470 constraints (247 original × 2 for equality expansion)
- Each row contains 225 float values
- Printing 470 × 225 floats = **105,750 float values** to stderr
- Each `eprintln!` with `{:?}` on a large vector is VERY slow

### Bottleneck 2: No Pre-allocation
```rust
let mut a = Vec::new();  // No capacity hint
let mut b = Vec::new();
```

This caused multiple reallocations as constraints were added.

## Solution Implemented

### 1. Pre-allocate Vectors
```rust
// Pre-allocate: estimate 2 rows per constraint (for equality constraints)
let estimated_rows = self.constraints.len() * 2;
let mut a = Vec::with_capacity(estimated_rows);
let mut b = Vec::with_capacity(estimated_rows);
```

### 2. Suppress Verbose Output for Large Problems
```rust
// Only print detailed info for small problems (avoid performance hit)
if self.constraints.len() <= 20 {
    eprintln!("LP BUILD: Converting constraint with {} vars, relation {:?}, rhs {}", 
        constraint.variables.len(), constraint.relation, constraint.rhs);
}

// ... later ...

// Only print rows for small problems (printing 225-element vectors is SLOW!)
if self.constraints.len() <= 20 {
    eprintln!("LP BUILD: Constraint row = {:?}, rhs = {}", row, rhs_adjusted);
}
```

### 3. Add Progress Message for Large Problems
```rust
if self.constraints.len() > 20 {
    eprintln!("LP BUILD: Processing {} constraints with {} variables (output suppressed for performance)...", 
        self.constraints.len(), n_vars);
}
```

## Results

### Before Optimization
```
LP: System is suitable for LP with objective, solving...
LP BUILD: Converting constraint with 1 vars, relation GreaterOrEqual, rhs -0
LP BUILD: Constraint row = [-1.0, 0.0, 0.0, ...(225 floats)...], rhs = 0
LP BUILD: Converting constraint with 1 vars, relation GreaterOrEqual, rhs -0
LP BUILD: Constraint row = [0.0, -1.0, 0.0, ...(225 floats)...], rhs = 0
[... 470 times, taking >60 seconds ...]
❌ TIMEOUT - never finishes LP BUILD
```

### After Optimization
```
LP: System is suitable for LP with objective, solving...
LP BUILD: Processing 470 constraints with 225 variables (output suppressed for performance)...
LP BUILD: Final problem: 225 variables (excluding 0 constants), 486 constraints
LP: Problem has 225 vars, 486 constraints
SIMPLEX: Starting solve for problem with 225 vars, 486 constraints
SIMPLEX: Problem validated
SIMPLEX: Converting to standard form...
SIMPLEX: Standard form has 711 rows, 936 cols
SIMPLEX: Starting Phase I...
✅ LP BUILD completes in <1 second!
⚠️ Now stuck in SIMPLEX Phase I (different bottleneck)
```

## Performance Impact

| Phase | Before | After | Improvement |
|-------|--------|-------|-------------|
| LP BUILD | >60s (timeout) | <1s | **60x faster** |
| SIMPLEX Phase I | N/A (never reached) | >30s | Now the bottleneck |

## Files Modified

**`src/lpsolver/csp_integration.rs`** (lines 233-267):
- Added `Vec::with_capacity()` for pre-allocation
- Added conditional debug output based on problem size
- Added progress message for large problems

## Code Changes

```rust
// Before
let mut a = Vec::new();
let mut b = Vec::new();

for constraint in &self.constraints {
    eprintln!("LP BUILD: Converting constraint...");
    // ... build row ...
    eprintln!("LP BUILD: Constraint row = {:?}, rhs = {}", row, rhs);  // SLOW!
    a.push(row);
    b.push(rhs);
}

// After  
let estimated_rows = self.constraints.len() * 2;
let mut a = Vec::with_capacity(estimated_rows);  // Pre-allocate
let mut b = Vec::with_capacity(estimated_rows);

if self.constraints.len() > 20 {
    eprintln!("LP BUILD: Processing {} constraints... (output suppressed)", 
        self.constraints.len());
}

for constraint in &self.constraints {
    if self.constraints.len() <= 20 {  // Only for small problems
        eprintln!("LP BUILD: Converting constraint...");
    }
    // ... build row ...
    if self.constraints.len() <= 20 {  // Only for small problems
        eprintln!("LP BUILD: Constraint row = {:?}, rhs = {}", row, rhs);
    }
    a.push(row);
    b.push(rhs);
}
```

## Next Steps

The agprice problem now reaches the **SIMPLEX Phase I** but times out there:
- Standard form expansion: 225 vars → 711 rows × 936 cols
- Phase I solving with this size is slow

**Future optimizations needed**:
1. SIMPLEX Phase I/II performance
2. Better initial basis selection
3. Consider external solver integration (e.g., HiGHS, Clarabel)
4. Sparse matrix representation throughout

## Verification

Small problems (≤20 constraints) still get full debug output for debugging purposes.
Large problems (>20 constraints) get efficient processing with progress messages.

**Status**: ✅ LP BUILD optimization complete and validated
