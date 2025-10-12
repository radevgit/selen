# agprice_full.rs Analysis

## Problem Summary

**File**: `examples/agprice_new_api.rs` (migrated from `/tmp/agprice_new_api.rs`)  
**Type**: Agricultural pricing optimization (quadratic programming)  
**Objective**: Maximize revenue  
**Size**: 254 variables, 247 constraints  

## Migration Status

✅ **API Migration**: SUCCESSFUL
- All `float_lin_*` methods converted to `lin_*` methods
- File compiles successfully
- No API errors

## Execution Results

### Initial Attempt
❌ **Execution**: TIMED OUT in LP BUILD phase (>60 seconds)

### After LP BUILD Optimization
⚠️ **Execution**: TIMES OUT in SIMPLEX Phase I (>30 seconds)

**What happens:**
```
Solving...
LP: Starting with 223 AST-extracted constraints (runtime API)
LP: Found 247 propagator constraints (old API)
LP: Extracted 470 total constraints, 225 variables
LP: Extracted objective: variable VarId(224), minimize=false
LP: Set objective for variable at index 224 (minimize=false)
LP: is_suitable_for_lp() = true, lp_has_objective = true
LP: System is suitable for LP with objective, solving...
LP BUILD: Processing 470 constraints with 225 variables (output suppressed for performance)...
LP BUILD: Final problem: 225 variables (excluding 0 constants), 486 constraints
LP: Problem has 225 vars, 486 constraints
SIMPLEX: Starting solve for problem with 225 vars, 486 constraints
SIMPLEX: Problem validated
SIMPLEX: Converting to standard form...
SIMPLEX: Standard form has 711 rows, 936 cols
SIMPLEX: Starting Phase I...
SIMPLEX Phase I: m=711, n=936
[... Phase I iterations taking >30 seconds ...]
```

## Root Cause Analysis

### 1. Problem Scale
- **225 variables** in LP system
- **470 constraints** (247 original × 2 for equality handling)
- Each constraint creates a row of **225 floats**
- Total memory for constraint matrix: ~470 MB (470 × 225 × 8 bytes)

### 2. LP BUILD Performance Issue (RESOLVED)
~~The solver gets stuck in the **LP BUILD phase**~~ **FIXED!**

**Problem**: Debug printing of 225-element float vectors for each of 470 constraints was extremely slow.

**Solution**: 
- Pre-allocate constraint matrix vectors
- Suppress debug output for large problems (>20 constraints)
- Add progress message instead

**Result**: LP BUILD now completes in <1 second!

### 3. SIMPLEX Phase I Performance Issue (CURRENT)
After LP BUILD optimization, the solver now reaches the **SIMPLEX Phase I** but gets stuck there:
```
LP: Problem has 225 vars, 486 constraints
SIMPLEX: Starting solve for problem with 225 vars, 486 constraints
SIMPLEX: Standard form has 711 rows, 936 cols
SIMPLEX Phase I: m=711, n=936
[... Phase I iterations taking >30 seconds ...]
```

The standard form expansion creates a 711×936 problem which is too large for the current simplex implementation.

### 3. Problem Characteristics
The problem includes **quadratic terms**:
- `milksq` (milk squared)
- `buttsq` (butter squared)
- `chasq` (cheese A squared)
- `chbsq` (cheese B squared)
- `qsq` (q squared)

The LP solver can only handle the **linear relaxation** of this problem, which may not give the true optimal solution anyway.

## Performance Comparison

| Problem Type | Variables | Constraints | LP Build Time | Simplex Time | Result |
|--------------|-----------|-------------|---------------|--------------|--------|
| test_lp_large_domains | 2 | 2 | <1ms | <1ms | ✅ Optimal in <1s |
| test_minimal_ast | 3 | 2 | <1ms | <1ms | ✅ Optimal instantly |
| agprice (before opt) | 225 | 470 | >60s | N/A | ❌ Timeout in BUILD |
| agprice (after opt) | 225 | 486 | <1s | >30s | ⚠️ Timeout in Phase I |

**Progress**: ✅ LP BUILD optimization successful (60s → <1s)  
**Remaining bottleneck**: SIMPLEX Phase I with 711×936 standard form

## Recommendations

### For This Specific Problem

1. **Not suitable for current LP solver** due to:
   - Scale (225 vars × 470 constraints)
   - Quadratic objective function
   - LP BUILD performance bottleneck

2. **Alternative approaches**:
   - Use traditional CP propagation (disable LP solver)
   - Wait for LP solver performance optimizations
   - Consider problem reformulation to reduce constraints

### For Future LP Solver Development

1. **✅ Optimize LP BUILD phase** (DONE):
   - ✅ Pre-allocate constraint matrix vectors
   - ✅ Suppress verbose debug output for large problems
   - 🔄 TODO: Sparse matrix representation (most coefficients are 0)
   - 🔄 TODO: Parallel constraint conversion
   - 🔄 TODO: Consider using specialized linear algebra libraries

2. **🔄 Optimize SIMPLEX solver** (NEXT):
   - Revised simplex method for better numerical stability
   - More efficient basis updates
   - Better initial basis selection
   - Consider external solver (e.g., highs, clarabel)

3. **Add early termination**:
   - Timeout during LP BUILD phase
   - Fall back to CP propagation if LP takes too long

3. **Problem size heuristics**:
   - Automatically disable LP for problems > threshold
   - Example: Skip LP if vars × constraints > 10,000

## Test Configuration

The migrated file includes proper timeouts:
```rust
let config = SolverConfig {
    timeout_ms: Some(300_000), // 5 minute timeout
    max_memory_mb: Some(4096), // 4GB memory limit
    ..Default::default()
};
```

But times out before these limits are reached due to LP BUILD performance.

## Conclusion

**Migration**: ✅ Documentation and tooling work correctly  
**LP Solver**: ⚠️ Not yet optimized for problems of this scale  
**Next Steps**: Profile and optimize LP BUILD phase for large problems

The migration guide correctly warns users about large-scale problems. This case validates that warning.

---

**Date**: October 10, 2025  
**Test Command**: `cargo run --release --example agprice_new_api`  
**Timeout Used**: 60 seconds  
**Result**: LP BUILD phase incomplete
