# Performance Optimization Plan: Memory Allocation & Macro Usage

**Date:** September 21, 2025  
**Focus:** Address Health Check Issue #15 - Memory Allocation Patterns  
**Priority:** Medium → High (Performance Critical)  

## Executive Summary

The CSP Solver currently uses inefficient memory allocation patterns that create performance bottlenecks in high-frequency solving scenarios. This plan addresses systematic optimization of memory allocation, elimination of slow macros, and implementation of memory pooling strategies.

## Current Performance Issues

### 1. Excessive `vec!` Macro Usage
**Problem:** `vec!` macro creates new allocations without size hints
**Impact:** Repeated allocations, memory fragmentation, cache misses
**Locations:** Throughout constraint building, domain operations, propagation

### 2. No Vector Preallocation  
**Problem:** Vectors grow dynamically causing multiple reallocations
**Impact:** O(log n) reallocations per vector, performance degradation
**Examples:**
- Constraint collection: `Vec<PropId>` grows one by one
- Domain iteration: `Vec<Val>` created without size hints
- Variable collections: `Vec<VarId>` built incrementally

### 3. Heavy HashMap Usage Without Capacity Hints
**Problem:** HashMaps start small and rehash multiple times
**Impact:** Expensive rehashing operations, memory fragmentation
**Locations:** Variable mapping, constraint indexing, solution storage

### 4. No Object Pooling
**Problem:** Temporary objects allocated/deallocated repeatedly
**Impact:** GC pressure, allocation overhead
**Examples:** Domain backup objects, propagation working sets

## Optimization Strategy

### Phase 1: Immediate Wins (1-2 weeks)

#### 1.1 Replace `vec!` with Preallocated Vectors
**Target Files:**
- `src/constraints/macros.rs` (3,061 lines)
- `src/model/core.rs` (1,480 lines) 
- `src/variables/domain/sparse_set.rs`

**Before:**
```rust
let mut constraints = vec![];
for var in vars {
    constraints.push(create_constraint(var));
}
```

**After:**
```rust
let mut constraints = Vec::with_capacity(vars.len());
for var in vars {
    constraints.push(create_constraint(var));
}
```

**Expected Impact:** 30-50% reduction in allocation overhead

#### 1.2 Add Capacity Hints to HashMaps
**Target:** Variable storage, constraint mapping

**Before:**
```rust
let mut var_map = HashMap::new();
```

**After:**
```rust
let mut var_map = HashMap::with_capacity(estimated_vars);
```

#### 1.3 Optimize Domain Operations
**Target:** `SparseSet` operations in domain manipulation

**Strategy:**
- Preallocate working vectors for domain intersection
- Reuse temporary vectors across operations
- Add size hints to iterator collections

### Phase 2: Memory Pooling (2-3 weeks)

#### 2.1 Implement Vector Pool
```rust
pub struct VectorPool<T> {
    pools: Vec<Vec<Vec<T>>>,  // Pools by capacity (powers of 2)
    max_capacity: usize,
}

impl<T> VectorPool<T> {
    pub fn get_vec(&mut self, capacity_hint: usize) -> Vec<T> { /* ... */ }
    pub fn return_vec(&mut self, mut vec: Vec<T>) { /* ... */ }
}
```

**Usage Pattern:**
```rust
// Get pooled vector
let mut working_vec = pool.get_vec(expected_size);
// Use vector
working_vec.extend(some_iter);
// Return to pool
pool.return_vec(working_vec);
```

#### 2.2 Implement Constraint Building Cache
**Problem:** Constraint objects created repeatedly for similar patterns
**Solution:** Cache commonly used constraint configurations

```rust
pub struct ConstraintCache {
    arithmetic_cache: HashMap<ArithmeticPattern, PropId>,
    comparison_cache: HashMap<ComparisonPattern, PropId>,
    reuse_threshold: usize,
}
```

#### 2.3 Domain Operation Optimization
**Strategy:** Reuse domain backup objects
```rust
pub struct DomainPool {
    sparse_set_backups: Vec<SparseSetBackup>,
    interval_backups: Vec<IntervalBackup>,
}
```

### Phase 3: Advanced Optimizations (3-4 weeks)

#### 3.1 Arena Allocation for Temporary Objects
**Target:** Short-lived objects during solving
**Implementation:** Use `bumpalo` crate or custom arena

```rust
pub struct SolverArena {
    bump: bumpalo::Bump,
}

impl SolverArena {
    pub fn alloc_vec<T>(&self, capacity: usize) -> &mut Vec<T> {
        self.bump.alloc(Vec::with_capacity(capacity))
    }
}
```

#### 3.2 Batch Processing Optimization
**Strategy:** Process constraints in batches to amortize allocation costs

```rust
pub struct BatchProcessor {
    batch_size: usize,
    working_memory: Vec<WorkingSet>,
}
```

#### 3.3 Copy-on-Write for Domain Operations
**Problem:** Domain objects copied unnecessarily
**Solution:** Use Cow<'_, Vec<T>> for domain representations

### Phase 4: Macro Elimination (2-3 weeks)

#### 4.1 Replace `vec!` Calls Systematically
**Tools:** 
- `grep -r "vec\!" src/` to find all instances
- Create helper functions for common patterns

**Common Patterns:**
```rust
// Pattern 1: Known size collections
fn create_var_vec(vars: &[VarId]) -> Vec<VarId> {
    let mut result = Vec::with_capacity(vars.len());
    result.extend_from_slice(vars);
    result
}

// Pattern 2: Iterative building
fn collect_with_capacity<T, I>(iter: I, hint: usize) -> Vec<T> 
where I: Iterator<Item = T> {
    let mut result = Vec::with_capacity(hint);
    result.extend(iter);
    result
}
```

#### 4.2 Optimize Constraint Macro Expansion
**Target:** `post!` macro and similar constraint building macros
**Strategy:** Generate preallocated collections in macro expansion

**Before:**
```rust
macro_rules! post {
    ($model:expr, $($constraint:expr),*) => {
        {
            let constraints = vec![$($constraint),*];
            $model.post_all(constraints)
        }
    };
}
```

**After:**
```rust
macro_rules! post {
    ($model:expr, $($constraint:expr),*) => {
        {
            let mut constraints = Vec::with_capacity(count_exprs!($($constraint)*));
            $(constraints.push($constraint);)*
            $model.post_all(constraints)
        }
    };
}
```

## Performance Benchmarking

### Metrics to Track
1. **Allocation Rate:** bytes/second allocated during solving
2. **Peak Memory:** Maximum memory usage during complex problems
3. **Solve Time:** End-to-end solving time for standard benchmarks
4. **GC Pressure:** Time spent in allocation/deallocation

### Benchmark Problems
1. **N-Queens (N=20):** Combinatorial explosion test
2. **Sudoku Variants:** Multiple difficulty levels
3. **Graph Coloring:** Large graphs (1000+ nodes)
4. **Manufacturing Optimization:** Real-world constraint density

### Expected Improvements
- **25-40% reduction** in solve time for medium problems
- **40-60% reduction** in memory allocation rate
- **15-30% reduction** in peak memory usage
- **Improved cache locality** leading to better CPU utilization

## Implementation Plan

### Week 1-2: Foundation
- [ ] Audit all `vec!` usage across codebase
- [ ] Implement basic vector preallocation helpers
- [ ] Add capacity hints to HashMap creation
- [ ] Create performance benchmark suite

### Week 3-4: Vector Pool Implementation  
- [ ] Design and implement VectorPool
- [ ] Integrate VectorPool into constraint building
- [ ] Optimize domain operations with pooling
- [ ] Performance testing and validation

### Week 5-6: Advanced Pooling
- [ ] Implement ConstraintCache for common patterns
- [ ] Add DomainPool for backup objects
- [ ] Arena allocation for temporary objects
- [ ] Batch processing optimization

### Week 7-8: Macro Optimization
- [ ] Systematically replace `vec!` calls
- [ ] Optimize constraint macro expansion
- [ ] Copy-on-Write implementation for domains
- [ ] Final performance validation

### Week 9: Integration & Testing
- [ ] Integration testing with existing test suite
- [ ] Performance regression testing
- [ ] Documentation updates
- [ ] Benchmarking report

## Risk Assessment

### Low Risk
- Vector preallocation (minimal API changes)
- HashMap capacity hints (internal optimization)
- Performance benchmarking infrastructure

### Medium Risk  
- Memory pooling (requires careful lifetime management)
- Macro optimization (potential compilation impact)
- Arena allocation (new dependency considerations)

### High Risk
- Copy-on-Write domains (significant API changes)
- Constraint caching (complex invalidation logic)

## Success Metrics

### Performance Targets
- [ ] **Allocation rate reduced by 40%**
- [ ] **Peak memory reduced by 25%** 
- [ ] **Solve time improved by 30%** for medium problems
- [ ] **No performance regression** on any existing benchmark

### Code Quality
- [ ] All existing tests pass
- [ ] No new clippy warnings introduced
- [ ] Documentation updated for new patterns
- [ ] Memory safety maintained (no new unsafe code)

## Alternative Approaches

### 1. RAII Memory Management
Use custom RAII wrappers for automatic pool management

### 2. Compile-Time Size Calculation
Leverage const generics for known-size collections

### 3. Zero-Copy Constraint Building
Avoid intermediate collections in constraint macros

### 4. Custom Allocator Integration
Use jemalloc or mimalloc for better allocation patterns

## Long-Term Vision

This optimization plan positions the CSP Solver for:
- **High-frequency solving** scenarios
- **Large-scale problems** with thousands of variables
- **Real-time applications** requiring predictable performance
- **Memory-constrained environments** 

The systematic approach ensures performance improvements while maintaining the library's safety guarantees and clean API design.

---

**Next Steps:** Begin with Phase 1 implementation focusing on immediate wins through vector preallocation and capacity hints.