# Efficient Float Solver Implementation Plan

## Overview
Transform the CSP solver from a general discrete approach to a hybrid solver that uses optimal algorithms for each problem type:
- **Pure float problems**: O(1) analytical solutions instead of O(log n) binary search
- **Pure integer problems**: Keep existing approach (works well for discrete domains)
- **Mixed problems**: Use MINLP techniques with efficient float subproblem solving

## Implementation Strategy

### Phase 1: Foundation & Analysis (1-2 steps)
1. ✅ **Problem Classification System** - Detect if a problem is pure float, pure integer, or mixed
2. **Benchmark Current Approach** - Establish baseline metrics for comparison

### Phase 2: Pure Float Optimization (2-3 steps)  
3. **Direct Bounds Optimization** - The core O(1) float optimization algorithm
4. **Integration with Existing Solver** - Hook into current solve/maximize methods
5. **Comprehensive Testing** - Verify performance gains

### Phase 3: Mixed Problem Support (2-3 steps)
6. **Separable Mixed Problems** - Handle independent float/int variables
7. **Basic MINLP Algorithm** - Branch-and-bound for coupled problems
8. **Performance Validation** - End-to-end benchmarking

## Directory Structure
```
src/
├── lib.rs
├── model.rs
├── domain/          # (existing)
├── search/          # (existing) 
├── optimization/    # (new) - efficient algorithms
│   ├── mod.rs
│   ├── classification.rs    # Problem type detection
│   ├── float_direct.rs      # O(1) float optimization  
│   ├── bounds_consistency.rs # Interval propagation
│   └── minlp.rs            # Mixed integer-float algorithms
└── benchmarks/      # (new) - performance testing
    ├── mod.rs
    └── comparison.rs        # Before/after metrics
```

## Design Decisions Made

### Integration Approach
- **Automatic classification**: Classification happens transparently when `solve()`/`maximize()` is called
- **No API changes**: Existing user code continues to work unchanged
- **Future-proof heuristics**: Classification logic can be updated as new constraints are added

### Problem Classification Categories
1. **Pure Float Optimization**: Single float variable with bounds constraints → Direct analytical solution
2. **Pure Integer Problem**: Only integer variables → Use existing discrete CSP solver
3. **Separable Mixed**: Float and integer variables with no coupling → Solve independently
4. **Coupled Mixed**: Integer-float interactions → Use MINLP branch-and-bound

## Current Status

### Step 1: Problem Classification System ⏳
**Goal**: Automatically detect problem type to route to optimal algorithm

**What we're creating**: 
- Analyze `Model` structure to understand constraint storage
- Build classifier examining variable types and constraint patterns  
- Create foundation for routing problems to appropriate solvers

**Implementation approach**:
1. Create `/src/optimization/mod.rs` and `classification.rs`
2. Understand existing constraint representation
3. Implement problem type detection logic
4. Add comprehensive tests for classification accuracy

## Expected Performance Improvements

| Problem Type | Current Approach | Efficient Approach | Expected Speedup |
|--------------|------------------|-------------------|------------------|
| Pure Float | O(log n) binary search | O(1) analytical | **100-1000x** |
| Pure Integer | O(log n) binary search | O(log n) binary search | No change |
| Separable Mixed | O(log n₁ × log n₂) | O(1 + log n₁) | **10-100x** |
| Coupled Mixed | O(log n₁ × log n₂) | O(2^n₁ × 1) | **Major if few integers** |

## Target Problems
- **Immediate**: `test_less_than_with_floats` - from 287 propagations to 1 operation
- **Portfolio optimization**: Multiple float constraints with bounds
- **Mixed scheduling**: Integer decisions + continuous resource allocation
- **Engineering design**: Discrete choices + continuous parameters
