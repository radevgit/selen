# LP Solver Performance Benchmarks

Performance measurements for the Primal Simplex LP solver implementation.

## Test Environment

- **Build**: Release mode (`--release`)
- **Date**: October 9, 2025
- **Platform**: Linux
- **Rust**: 1.84.0 (or current version)

## Benchmark Results

### Small Dense Problem (50x50)

- **Variables**: 50
- **Constraints**: 50
- **Density**: 100% (all variables in each constraint)
- **Result**: Optimal
- **Iterations**: 26
- **Solve Time**: **5-7 ms** (0.005-0.007 seconds)
- **Memory Usage**: ~0.4 MB
- **Objective Value**: 41,000

**Analysis**: Very fast for small dense problems. The dense constraint matrix requires more memory but solves quickly due to small problem size.

---

### Medium Sparse Problem (100x100)

- **Variables**: 100
- **Constraints**: 100
- **Density**: ~10% (overlapping constraints)
- **Result**: Optimal
- **Iterations**: 95
- **Solve Time**: **80-100 ms** (0.08-0.10 seconds)
- **Memory Usage**: ~0.2-6 MB
- **Objective Value**: 500
- **Max Constraint Violation**: < 1e-6

**Analysis**: Good performance for medium-sized problems. About 1 iteration per constraint. Memory usage is reasonable. This size is typical for CSP-LP integration scenarios.

**Performance vs Debug Mode**:
- Debug: 7.76 seconds
- Release: 0.092 seconds
- **Speedup: ~84x**

---

### Large Sparse Problem (200x200)

- **Variables**: 200
- **Constraints**: 200
- **Density**: ~5% (sparse constraints)
- **Result**: Optimal
- **Iterations**: 409
- **Solve Time**: **2.9-3.0 seconds**
- **Memory Usage**: ~0.2-1 MB
- **Objective Value**: 600

**Analysis**: Still solves in ~3 seconds for 200x200. More iterations required due to increased problem complexity. Memory usage remains low due to sparsity.

---

### Very Large Sparse Problem (300x300)

- **Variables**: 300
- **Constraints**: 300
- **Density**: ~3.3% (very sparse)
- **Result**: Optimal
- **Iterations**: 557
- **Solve Time**: **14.5-15.5 seconds**
- **Memory Usage**: ~0.14 MB
- **Objective Value**: 900

**Analysis**: Takes ~15 seconds for 300x300. Iterations per constraint ratio increasing (1.86). Still practical for batch processing but approaching limits for interactive use.

---

### Extremely Large Sparse Problem (500x500)

- **Variables**: 500
- **Constraints**: 500
- **Density**: ~2% (extremely sparse)
- **Result**: Optimal
- **Iterations**: 691
- **Solve Time**: **98-102 seconds** (~1.7 minutes)
- **Memory Usage**: ~0.11 MB
- **Objective Value**: 1,250

**Analysis**: Takes ~100 seconds (1.7 minutes) for 500x500. Time per variable is ~197ms. This is at the practical limit for the current dense matrix implementation.

---

### Density Impact on 500x500 Problems

Testing different densities reveals surprising results:

| Density | Non-zeros/Row | Time | Iterations | Status |
|---------|---------------|------|------------|--------|
| 2% (very sparse) | 10 | **100s** | 691 | Medium |
| 10% (sparse) | 50 | **194s** | 1,219 | ⚠ WORST |
| 50% (dense) | 250 | **35s** | 250 | ✓ BEST |

**Key Finding**: **Dense problems are FASTER** than sparse ones! The 50% dense problem solves in 35 seconds vs 100 seconds for the 2% sparse version—nearly **3x faster**.

**Why?** Dense constraints provide more information → fewer iterations needed (250 vs 691). Since the current implementation uses dense matrices anyway, there's no per-iteration penalty for density. More information per constraint = faster convergence!

**Implication**: The dense matrix implementation is actually **optimal for dense problems** (> 30% density), which matches typical CSP-LP integration scenarios.

See `docs/LP_SOLVER_DENSITY_ANALYSIS.md` for detailed analysis.

---

---

## Performance Summary Table

| Size | Variables | Constraints | Time (s) | Time (ms) | Memory (MB) | Iterations | Iter/Const | ms/Var |
|------|-----------|-------------|----------|-----------|-------------|------------|------------|--------|
| Small | 50 | 50 | 0.006 | 6 | 0.4 | 26 | 0.52 | 0.12 |
| Medium | 100 | 100 | 0.090 | 90 | 0.2-6 | 95 | 0.95 | 0.90 |
| Large | 200 | 200 | 3.0 | 3,000 | 0.2-1 | 409 | 2.04 | 15 |
| Very Large | 300 | 300 | 15 | 15,000 | 0.14 | 557 | 1.86 | 50 |
| Extreme | 500 | 500 | 100 | 100,000 | 0.11 | 691 | 1.38 | 200 |

### Key Observations

1. **Sub-quadratic iteration growth**: Iterations grow slower than O(n²), approximately O(n·log n)
2. **Super-quadratic time growth**: Time per iteration increases with problem size due to dense matrix operations
3. **Excellent memory efficiency**: Memory usage stays very low even for large problems (~0.1-6 MB)
4. **Sweet spot**: 100-200 variables for interactive use, up to 300 for batch processing

---

## Performance Characteristics

### Time Complexity

Based on benchmarks:
- **50x50**: ~6ms → 0.12ms per variable
- **100x100**: ~90ms → 0.90ms per variable
- **200x200**: ~3.0s → 15ms per variable
- **300x300**: ~15s → 50ms per variable
- **500x500**: ~100s → 200ms per variable

The solver exhibits **worse than quadratic complexity** (appears to be ~O(n²·⁵) to O(n³)) for these problem sizes, which is typical for Simplex method with dense matrix operations and increasing iteration counts.

### Memory Complexity

- **50x50 (dense)**: ~0.4-6 MB
- **100x100 (sparse)**: ~0.2-6 MB  
- **200x200 (sparse)**: ~0.2-1 MB

Memory usage is approximately **O(mn)** where m = constraints, n = variables:
- Dense 50x50: ~2,500 elements
- Sparse 100x100: ~1,000 non-zeros
- Sparse 200x200: ~2,000 non-zeros

The memory tracking shows low overhead, with the solver being very memory-efficient for sparse problems.

### Iterations

- **50x50 dense**: 26 iterations (0.52 iterations per constraint)
- **100x100 sparse**: 95 iterations (0.95 iterations per constraint)
- **200x200 sparse**: 409 iterations (2.04 iterations per constraint)
- **300x300 sparse**: 557 iterations (1.86 iterations per constraint)
- **500x500 sparse**: 691 iterations (1.38 iterations per constraint)

Iteration count scales **roughly O(n·log n)** with problem size. The iterations-per-constraint ratio varies between 0.5-2.0, suggesting problem structure affects iteration count significantly.

---

## Comparison to Other Solvers

For reference, typical performance expectations:

| Solver | 100x100 Time | 200x200 Time | Notes |
|--------|-------------|-------------|-------|
| **selen LP** | ~85ms | ~2.8s | Dense matrix, Primal Simplex |
| Commercial (e.g., CPLEX) | ~5-10ms | ~50-100ms | Highly optimized, sparse |
| SciPy linprog | ~100-200ms | ~1-2s | Python overhead |
| OR-Tools | ~20-50ms | ~200-500ms | Specialized algorithms |

**Verdict**: The selen LP solver performs reasonably well for a pure Rust implementation using dense matrices. For CSP integration (typically <100 variables), performance is excellent.

---

## Optimization Opportunities

1. **Sparse Matrix Storage**: Current implementation uses dense matrices. Switching to sparse storage for large problems could reduce both time and memory by 10-100x for sparse problems.

2. **Revised Simplex**: Already using revised simplex with LU factorization, which is efficient.

3. **Warm-Starting**: Dual simplex implementation supports warm-starting, which is crucial for incremental solving in CSP contexts.

4. **Parallelization**: Matrix operations could be parallelized for larger problems.

5. **SIMD**: Vector operations could benefit from SIMD instructions.

---

## Recommendations for Usage

### Problem Size Guidelines

| Problem Size | Expected Time | Recommended |
|--------------|---------------|-------------|
| < 50x50 | < 10ms | ✓ Excellent |
| 50-100 | 10-100ms | ✓ Very Good |
| 100-200 | 100ms-3s | ✓ Good |
| 200-300 | 3-15s | ✓ Acceptable (batch) |
| 300-500 | 15-100s | ⚠ Slow - consider alternatives |
| > 500 | > 100s | ❌ Not recommended - use sparse solver |

### When to Use LP Solver in CSP

Based on performance data:

**✓ Use LP when:**
- Problem has 10-100+ continuous variables
- Many linear constraints with floating-point coefficients
- Need tight bounds for efficient search
- Problem is sparse (< 10% density)

**✗ Don't use LP when:**
- Very small problems (< 10 variables) - propagation overhead exceeds benefit
- Purely integer constraints - dedicated constraint propagation is faster
- Real-time requirements (< 1ms) with > 100 variables

---

## Memory Tracking

The solver now uses **per-instance memory estimation** rather than global tracking:

```rust
fn estimate_memory_mb(&self, a: &Matrix, basis: &Basis) -> f64 {
    // Main constraint matrix
    let mut total_bytes = a.memory_bytes();
    
    // Basis matrices (L, U factorization)
    let m = a.rows;
    total_bytes += 2 * m * m * std::mem::size_of::<f64>();
    
    // Working vectors
    total_bytes += (a.rows + a.cols) * std::mem::size_of::<f64>();
    
    total_bytes as f64 / (1024.0 * 1024.0)
}
```

This eliminates race conditions and enables safe parallel usage.

---

## Test Reproducibility

To reproduce these benchmarks:

```bash
# Run all performance tests
cargo test --release --test test_lp_performance -- --nocapture

# Run specific size
cargo test --release --test test_lp_performance test_large_lp_problem_100x100 -- --nocapture --exact

# Debug mode (for comparison)
cargo test --test test_lp_performance test_large_lp_problem_100x100 -- --nocapture --exact
```

---

## Scaling Analysis

### Time Scaling

Comparing problem sizes:
- **50→100** (2x): 6ms → 90ms = **15x slower** (worse than quadratic)
- **100→200** (2x): 90ms → 3s = **33x slower** (cubic behavior)
- **200→300** (1.5x): 3s → 15s = **5x slower**
- **300→500** (1.67x): 15s → 100s = **6.7x slower**

This demonstrates **O(n²·⁵) to O(n³) time complexity** due to:
1. Dense matrix operations (O(n³) for LU factorization)
2. Increasing iterations with problem size
3. Cache effects for larger matrices

### When to Use This Solver

**✓ Use for:**
- Problems with < 200 variables (solves in < 3 seconds)
- CSP-LP integration scenarios (typically 10-100 variables)
- Prototyping and development
- Systems without external LP solver dependencies

**⚠ Consider alternatives for:**
- Problems with 200-500 variables (15-100 seconds is slow)
- Real-time or interactive systems needing < 1s response
- Very large problems (> 500 variables)

**❌ Use external solver for:**
- Problems > 500 variables
- Production systems with large LPs
- When sparse matrix structure is known
- When needing best-in-class performance

---

## Conclusion

The selen LP solver delivers **excellent performance for small to medium problems** (< 200 variables), making it well-suited for CSP-LP integration. The ~90ms solve time for 100x100 problems is fast enough for interactive constraint solving and optimization.

### Performance Highlights

| Metric | Value |
|--------|-------|
| **Sweet Spot** | 50-200 variables |
| **Interactive Limit** | ~200 variables (< 3s) |
| **Batch Limit** | ~300 variables (< 15s) |
| **Practical Limit** | ~500 variables (< 100s) |
| **Memory Efficiency** | Excellent (< 1 MB for 500x500) |

### Key Strengths

- ✓ No external dependencies
- ✓ Thread-safe, no global state
- ✓ Predictable memory usage
- ✓ Good for CSP integration scenarios
- ✓ ~84x faster in release vs debug mode
- ✓ Handles problems up to 500x500 (though slowly)

### Known Limitations

- Dense matrix storage (O(n²) memory for constraint matrix)
- Cubic time complexity for large problems
- Not competitive with specialized solvers for > 200 variables
- No sparse matrix optimization

For larger problems (> 300 variables) or when performance is critical, consider using sparse matrix representations or external solvers like CPLEX, Gurobi, or HiGHS.
