# Selen Code Statistics

**Generated:** October 11, 2025  
**Total Lines of Code:** ~39,600 lines of Rust

---

## Executive Summary

Selen is a production-ready Constraint Satisfaction Problem (CSP) solver written in Rust, featuring:
- **Advanced constraint propagation** with 32% of codebase
- **Integrated LP solver** with complete simplex implementation
- **Sophisticated optimization engine** for continuous variables
- **Declarative runtime API** for easy problem modeling
- **~40K lines** of well-structured, efficient code

---

## Total Project Breakdown

| Category | Lines | Percentage |
|----------|-------|------------|
| **Source Code (src/)** | 39,614 | 93.6% |
| **Benchmarks** | 2,398 | 5.7% |
| **Examples** | 571 | 1.3% |
| **Integration Tests** | 226 | 0.5% |
| **TOTAL** | **42,809** | **100%** |

---

## Main Source Code Components (39,614 lines)

### Core Constraint Solver

#### 1. **Constraints System** (12,804 lines - 32.3%)

The largest and most critical component, implementing constraint propagation algorithms.

| Subcomponent | Lines | Description |
|--------------|-------|-------------|
| **Propagators (props/)** | 6,943 | Core propagation implementations |
| **Root Algorithms** | 4,044 | GAC algorithms, matching, domain filtering |
| **API Layer** | 1,553 | Constraint API and interfaces |
| **Framework** | 264 | Propagator framework infrastructure |

**Key Files:**
- `props/mod.rs` (1,877 lines) - LP extraction & propagation coordination
- `props/linear.rs` (1,512 lines) - Linear constraint propagation
- `props/reification.rs` (463 lines) - Reified constraints
- `props/alldiff.rs` (283 lines) - All-different constraint
- `props/bool_logic.rs` (271 lines) - Boolean logic constraints
- `props/cardinality.rs` (222 lines) - Cardinality constraints
- `gac_hybrid.rs` - Generalized Arc Consistency with BitSet/SparseSet optimization

**Constraint Types Implemented:**
- Linear constraints (integer and float)
- All-different constraints
- Element constraints
- Boolean logic (AND, OR, NOT, XOR)
- Reification (constraint ⇔ boolean)
- Cardinality constraints
- Table constraints
- Global constraints

---

#### 2. **Optimization Engine** (7,260 lines - 18.3%)

Sophisticated continuous optimization with variable classification and subproblem decomposition.

| Module | Lines | Purpose |
|--------|-------|---------|
| `model_integration.rs` | 806 | Integration with CSP model |
| `subproblem_solving.rs` | 756 | Decomposition and solving strategies |
| `constraint_integration.rs` | 586 | Constraint handling in optimization |
| `solution_integration.rs` | 481 | Solution extraction and validation |
| `float_direct.rs` | 494 | Direct float optimization |
| `constraint_metadata.rs` | 539 | Metadata collection and analysis |
| `variable_partitioning.rs` | 337 | Variable classification system |
| `classification.rs` | 415 | Optimization type classification |
| `precision_*.rs` | ~709 | Precision handling and propagation |
| `ulp_utils.rs` | 132 | ULP (Units in Last Place) utilities |
| Test files | ~1,500 | Comprehensive test coverage |

**Features:**
- Automatic variable classification (CSP/simple/composite/mixed)
- Subproblem decomposition for tractable optimization
- Precision-aware float handling
- Constraint consistency enforcement
- Multiple optimization strategies (minimize/maximize)

---

#### 3. **Variable System** (5,715 lines - 14.4%)

Efficient domain representations and variable views.

| Subcomponent | Lines | Description |
|--------------|-------|-------------|
| **Domain Implementations** | 3,363 | Efficient domain storage |
| └─ `sparse_set.rs` | 1,402 | SparseSet with O(1) operations |
| └─ `bitset_domain.rs` | 1,149 | BitSet for small domains |
| └─ `float_interval.rs` | 795 | Continuous float intervals |
| **Variable Views** | 1,258 | Transformation views |
| **Core Types** | 840 | Val, Var, Vars fundamental types |
| **View System** | 157 | Modular view framework |
| **Operations & Values** | 97 | Variable operations |

**Key Features:**
- Multiple domain representations optimized for different sizes
- O(1) domain operations with backtracking support
- Variable views for transformation without copying
- Efficient float interval arithmetic
- Mixed integer/float variable support

---

#### 4. **LP Solver** (3,888 lines - 9.8%)

Complete linear programming solver with simplex implementation.

| Module | Lines | Algorithm/Feature |
|--------|-------|-------------------|
| `simplex_primal.rs` | 933 | Primal simplex algorithm |
| `types.rs` | 635 | LP data structures and types |
| `csp_integration.rs` | 529 | Integration with CSP solver |
| `matrix.rs` | 494 | Sparse matrix operations |
| `lu.rs` | 459 | LU factorization for basis updates |
| `basis.rs` | 440 | Basis management and updates |
| `simplex_dual.rs` | 322 | Dual simplex algorithm |
| `mod.rs` | 76 | Public API and coordination |

**Capabilities:**
- Primal and dual simplex algorithms
- Sparse matrix operations
- Efficient basis updates with LU factorization
- Memory management and tracking
- Integration with CSP constraint extraction
- **Used at root node** for bound tightening in MIP problems

---

#### 5. **Runtime API** (2,335 lines - 5.9%)

Declarative constraint modeling API with AST representation.

| Component | Lines | Purpose |
|-----------|-------|---------|
| `mod.rs` | 1,748 | AST definition and materialization |
| `tests.rs` | 587 | API test coverage |

**Features:**
- Declarative constraint specification
- Expression tree (AST) building
- Lazy constraint materialization
- Type-safe operations
- Operator overloading for natural syntax
- Automatic constraint extraction for LP solver

---

#### 6. **Model** (2,337 lines - 5.9%)

High-level model API and factory methods.

| File | Lines | Purpose |
|------|-------|---------|
| `core.rs` | 1,638 | Model implementation and state |
| `factory_internal.rs` | 413 | Internal factory methods |
| `factory.rs` | 194 | Public factory API |
| `constraints.rs` | 41 | Constraint posting |
| `solving.rs` | 19 | Solving interface |
| `precision.rs` | 17 | Precision utilities |
| `mod.rs` | 15 | Module exports |

**Key Responsibilities:**
- Variable creation (integer, float, boolean)
- Constraint posting
- Configuration management (SolverConfig)
- LP solver integration control
- Search strategy coordination
- Solution extraction

---

#### 7. **Core Types** (1,513 lines - 3.8%)

Fundamental types and solution representation.

**Main Components:**
- `Solution` type with variable value extraction
- Configuration types
- Error handling
- Statistics collection (nodes, backtracks, LP stats)
- Result types

---

#### 8. **Specialized Solvers** (1,238 lines - 3.1%)

Domain-specific solver implementations.

**Sudoku Solver:**
- Optimized for 9×9 Sudoku puzzles
- Human-solving techniques:
  - Naked singles
  - Hidden singles
  - Naked pairs
- Advanced techniques (benchmarked, currently unused):
  - Box-line reduction
  - X-Wing
  - Alternating Inference Chains (AIC)
- Sparse candidate tracking
- Fast constraint propagation

---

#### 9. **Search Engine** (866 lines - 2.2%)

Backtracking search with various strategies.

**Features:**
- Depth-first search with backtracking
- Variable selection heuristics
- Value selection strategies
- Timeout support
- Memory limits
- Search statistics tracking
- LP integration at root node

---

#### 10. **Utilities** (696 lines - 1.8%)

Configuration and debugging tools.

**Components:**
- `SolverConfig` - Global solver configuration
- `LpConfig` - LP solver configuration
- Precision utilities
- Debugging helpers
- Statistics collection

---

#### 11. **API Facade** (102 lines - 0.3%)

Public API surface and exports.

---

#### 12. **Library Root** (302 lines - 0.8%)

- `lib.rs` - Crate root and module organization
- `prelude.rs` - Convenient imports
- `constraint_macros.rs` - Macro definitions

---

## Supporting Code

### Benchmarks (2,398 lines)

Performance testing and validation:
- Runtime API benchmarks
- Precision validation
- Performance regression tests
- Constraint coverage benchmarks

**Key Benchmarks:**
- `runtime_api_performance_benchmarks.rs`
- `performance_validation.rs`
- Precision validation suite

---

### Examples (571 lines)

Real-world problem demonstrations:

| Example | Lines | Description |
|---------|-------|-------------|
| `zebra_puzzle.rs` | 157 | Einstein's zebra puzzle |
| `sudoku.rs` | 127 | Sudoku solver demo |
| `n_queens.rs` | 119 | N-queens problem |
| `send_more_money.rs` | 107 | Cryptarithmetic puzzle |
| `mixed.rs` | 61 | Mixed integer/float optimization |

**Also includes:**
- Graph coloring
- Resource allocation
- Portfolio optimization
- Manufacturing scheduling
- Constraint type demonstrations

---

### Integration Tests (226 lines)

High-level integration testing:
- `test_unbounded_inference.rs` (226 lines) - Bound inference validation

---

## Architecture Highlights

### Component Distribution

```
Constraints (32.3%)  ████████████████▌
Optimization (18.3%) █████████▎
Variables (14.4%)    ███████▎
LP Solver (9.8%)     █████
Runtime API (5.9%)   ███
Model (5.9%)         ███
Core (3.8%)          ██
Solvers (3.1%)       █▌
Search (2.2%)        █
Utils (1.8%)         ▉
API (0.3%)           ▎
Root (0.8%)          ▍
```

### Largest Files (Top 10)

| Rank | File | Lines | Component |
|------|------|-------|-----------|
| 1 | `constraints/props/mod.rs` | 1,877 | Propagation coordination |
| 2 | `runtime_api/mod.rs` | 1,748 | Declarative API |
| 3 | `model/core.rs` | 1,638 | Model implementation |
| 4 | `constraints/props/linear.rs` | 1,512 | Linear constraints |
| 5 | `variables/domain/sparse_set.rs` | 1,402 | SparseSet domain |
| 6 | `variables/views.rs` | 1,258 | Variable views |
| 7 | `variables/domain/bitset_domain.rs` | 1,149 | BitSet domain |
| 8 | `lpsolver/simplex_primal.rs` | 933 | Primal simplex |
| 9 | `variables/core.rs` | 840 | Core types |
| 10 | `optimization/model_integration.rs` | 806 | Optimization integration |

---

## Code Quality Metrics

### Complexity Distribution

- **High Complexity (>1000 lines):** 7 files
  - These are core algorithms (propagation, domains, simplex)
  - Well-documented and thoroughly tested
  
- **Medium Complexity (500-1000 lines):** 12 files
  - Mostly optimization and integration modules
  
- **Low Complexity (<500 lines):** Majority of files
  - Focused, single-responsibility modules

### Documentation

- Comprehensive inline documentation
- Module-level documentation in most files
- Example code for major features
- Extensive test coverage

### Code Organization

- **Clear separation of concerns**
- **Modular architecture** with well-defined boundaries
- **Minimal coupling** between components
- **Extensive use of Rust's type system** for safety

---

## Key Insights

### 1. **Balanced Architecture**
The codebase maintains a good balance between:
- Core CSP solving (constraints, variables, search)
- Advanced features (optimization, LP integration)
- Usability (runtime API, examples)

### 2. **Efficiency Focus**
Multiple domain representations optimized for different use cases:
- BitSet for small domains (≤64 elements)
- SparseSet for medium domains (≤4096 elements)
- Float intervals for continuous domains

### 3. **Production Ready**
- Comprehensive error handling
- Memory and timeout limits
- Performance benchmarks
- Real-world examples
- Clean API design

### 4. **Research-Grade LP Integration**
Complete simplex implementation with:
- Both primal and dual algorithms
- Efficient basis updates (LU factorization)
- CSP constraint extraction
- Strategic use at root node only

### 5. **Sophisticated Optimization**
Advanced continuous optimization with:
- Automatic problem classification
- Subproblem decomposition
- Precision handling
- Constraint consistency

---

## Comparison with Similar Projects

For a ~40K line project, Selen delivers:
- ✅ **Complete CSP solver** with advanced propagators
- ✅ **Integrated LP solver** (not just a wrapper)
- ✅ **Optimization engine** for continuous variables
- ✅ **Declarative API** for ease of use
- ✅ **Production features** (timeouts, memory limits, statistics)

This is **remarkably comprehensive** for the codebase size, indicating:
- Efficient, focused implementation
- Minimal code duplication
- Well-designed abstractions
- Expert-level Rust usage

---

## Future Growth Potential

Based on the current structure, natural extensions could include:

1. **More Global Constraints** (~500-1000 lines each)
   - Cumulative, circuit, table extensions
   
2. **Advanced Search Strategies** (~500-1000 lines)
   - LDS, DBDS, portfolio search
   
3. **Parallel Search** (~1000-2000 lines)
   - Work-stealing, domain splitting
   
4. **SAT Integration** (~2000-3000 lines)
   - Lazy clause generation

The current architecture supports these additions without major refactoring.

---

## Conclusion

Selen's ~40K lines of code represent a **mature, production-ready CSP solver** with:
- Strong theoretical foundations
- Practical optimization features  
- Clean, maintainable architecture
- Comprehensive feature set

The code distribution reflects **well-prioritized development**, with the most complex logic (constraints and optimization) receiving the most attention, while maintaining simplicity in supporting modules.

**Code Density:** High - each line delivers significant value
**Maintainability:** Excellent - clear structure, good separation of concerns
**Completeness:** Very High - covers CSP, LP, optimization, and usability
**Quality:** Production-grade - proper error handling, testing, documentation

---

*This document was generated by analyzing the Selen codebase on October 11, 2025.*
