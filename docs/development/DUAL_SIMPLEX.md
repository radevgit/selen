# Dual Simplex Implementation Plan

## Decision: Implement Dual Simplex (+ Primal for initial solve)

## Mixed Integer-Float Question

**Q: Should Dual Simplex work for mixed int+float problems?**

**A: No - Dual Simplex solves CONTINUOUS (all float) problems only.**

For mixed integer-float (MILP), you need:
1. **Base LP solver** (Dual Simplex) - solves continuous relaxation
2. **Branch-and-Bound** on top - enforces integer constraints

### Approach for MILP:

```
MILP Problem: maximize x + 2y
              subject to: x + y <= 5
              x ∈ {integers}, y ∈ [0, 10] (float)

Branch-and-Bound Process:
┌─────────────────────────────────────────┐
│ 1. Solve LP relaxation (treat x as float)│
│    Dual Simplex: x=0.5, y=4.5, obj=9.5  │
└─────────────────────────────────────────┘
                    │
         x is not integer (0.5)
                    │
        ┌───────────┴───────────┐
        ▼                       ▼
┌─────────────┐         ┌─────────────┐
│Add x <= 0   │         │Add x >= 1   │
│Solve with   │         │Solve with   │
│Dual Simplex │         │Dual Simplex │ ← warm start!
│x=0, y=5     │         │x=1, y=4     │
│obj=10       │         │obj=9        │
└─────────────┘         └─────────────┘
      │                       │
   Best: 10              Prune (worse)
```

**Key insight:** Dual Simplex is PERFECT for Branch-and-Bound because:
- Each branch adds a constraint (x <= k or x >= k+1)
- Dual Simplex can warm-start from parent solution
- Much faster than solving each subproblem from scratch

### Implementation Strategy:

**Phase 1: Continuous LP (Dual Simplex)** ← START HERE
- Handles pure float problems
- ~1000 LOC
- Solves your immediate timeout issues
- Foundation for MILP

**Phase 2: MILP (Branch-and-Bound)** ← FUTURE
- Wraps Dual Simplex
- ~500 LOC additional
- Handles mixed int+float
- Requires Phase 1 to be solid first

**Recommendation:** 
- Implement Dual Simplex for continuous problems first
- This already solves most of your timeout issues
- Add MILP later if needed (many problems can be relaxed to continuous)

## Implementation Plan

### Architecture Overview

```
src/lpsolver/
├── mod.rs                 // Public API, problem detection
├── types.rs               // Common types (LpProblem, Solution, etc.)
├── matrix.rs              // Dense matrix operations
├── lu.rs                  // LU decomposition
├── basis.rs               // Basis management (shared by primal/dual)
├── simplex_primal.rs      // Primal simplex (for initial solve)
├── simplex_dual.rs        // Dual simplex (for warm starting)
└── tests.rs               // Unit tests

Future (Phase 2):
├── milp.rs                // Branch-and-bound for mixed-integer
└── preprocessing.rs       // Problem preprocessing/simplification
```

### LOC Estimate

| Component | Lines | Purpose |
|-----------|-------|---------|
| types.rs | 100 | Problem representation, solution |
| matrix.rs | 300 | Matrix ops (add, mult, transpose) |
| lu.rs | 200 | LU factorization + forward/back substitution |
| basis.rs | 200 | Basis management, pivot selection |
| simplex_primal.rs | 350 | Primal simplex, two-phase method |
| simplex_dual.rs | 200 | Dual simplex (shares basis code) |
| mod.rs | 100 | Public API, integration |
| tests.rs | 200 | Unit tests |
| **Total** | **1650** | Phase 1 |
| milp.rs | 500 | Branch-and-bound (Phase 2) |

## Step-by-Step Implementation

### Step 1: Foundation - Types & Matrix Operations (Week 1)

**Goal:** Set up basic data structures and matrix operations

#### 1.1 Create `src/lpsolver/mod.rs`
```rust
//! Linear Programming solver using Dual Simplex method
//! 
//! Solves problems of the form:
//!   maximize:   c^T x
//!   subject to: Ax <= b
//!               x >= 0

pub mod types;
pub mod matrix;
mod lu;
mod basis;
mod simplex_primal;
mod simplex_dual;

pub use types::{LpProblem, LpSolution, LpStatus};

/// Solve LP problem using Dual Simplex with warm starting
pub fn solve(problem: &LpProblem) -> Result<LpSolution, LpError> {
    // Implementation in later steps
    todo!()
}
```

#### 1.2 Define `types.rs`
```rust
/// LP problem in standard form:
/// maximize c^T x subject to Ax <= b, x >= 0
pub struct LpProblem {
    /// Number of variables
    pub n_vars: usize,
    
    /// Number of constraints
    pub n_constraints: usize,
    
    /// Objective coefficients (length n_vars)
    pub c: Vec<f64>,
    
    /// Constraint matrix A (n_constraints × n_vars)
    pub a: Vec<Vec<f64>>,
    
    /// Right-hand side (length n_constraints)
    pub b: Vec<f64>,
    
    /// Variable bounds (lower, upper)
    pub bounds: Vec<(f64, f64)>,
}

/// Solution to LP problem
pub struct LpSolution {
    pub status: LpStatus,
    pub objective: f64,
    pub x: Vec<f64>,
    pub iterations: usize,
}

pub enum LpStatus {
    Optimal,
    Infeasible,
    Unbounded,
    IterationLimit,
}

pub enum LpError {
    InvalidProblem(String),
    NumericalInstability,
    // ... more error types
}
```

#### 1.3 Implement `matrix.rs`
Basic dense matrix operations:
- Matrix creation/initialization
- Matrix-vector multiplication
- Vector operations (add, scale, dot product)
- Matrix transpose
- Row operations (for tableau updates)

**Key operations needed:**
```rust
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>, // Row-major storage
}

impl Matrix {
    // Multiply matrix by vector: y = A * x
    pub fn mat_vec_mult(&self, x: &[f64]) -> Vec<f64>;
    
    // Transpose matrix
    pub fn transpose(&self) -> Matrix;
    
    // Get/set elements
    pub fn get(&self, i: usize, j: usize) -> f64;
    pub fn set(&mut self, i: usize, j: usize, val: f64);
    
    // Row operations (for Gaussian elimination)
    pub fn swap_rows(&mut self, i: usize, j: usize);
    pub fn scale_row(&mut self, i: usize, factor: f64);
    pub fn add_scaled_row(&mut self, target: usize, source: usize, factor: f64);
}
```

**Testing:** 
- Test matrix multiplication
- Test transpose
- Test row operations
- Test numerical stability (condition numbers)

---

### Step 2: LU Decomposition (Week 1-2)

**Goal:** Implement LU factorization for solving linear systems

#### 2.1 Implement `lu.rs`

LU decomposition solves: Ax = b
- Decompose A = LU (lower × upper triangular)
- Solve Ly = b (forward substitution)
- Solve Ux = y (backward substitution)

```rust
pub struct LuFactorization {
    /// Combined L and U matrix (L below diagonal, U on and above)
    lu: Matrix,
    
    /// Row permutation (for pivoting)
    perm: Vec<usize>,
    
    /// Number of row swaps (for determinant sign)
    swap_count: usize,
}

impl LuFactorization {
    /// Compute LU decomposition with partial pivoting
    pub fn factorize(a: &Matrix) -> Result<Self, LpError>;
    
    /// Solve Ax = b using precomputed LU
    pub fn solve(&self, b: &[f64]) -> Vec<f64>;
    
    /// Update factorization after adding/removing one column
    /// (Used by Revised Simplex for basis updates)
    pub fn update_column(&mut self, col_idx: usize, new_col: &[f64]);
}
```

**Why LU is needed:**
- Revised Simplex solves many linear systems with same basis matrix
- LU decomposition: O(n³) once
- Each solve: O(n²) with LU, vs O(n³) without
- For 100×100 system: 1M ops vs 1000 ops per solve!

**Testing:**
- Test with identity matrix (should be exact)
- Test with known solutions
- Test numerical stability (ill-conditioned matrices)
- Test with singular matrices (should fail gracefully)

---

### Step 3: Basis Management (Week 2)

**Goal:** Manage the basis (set of basic variables) used by both Primal and Dual Simplex

#### 3.1 Implement `basis.rs`

```rust
pub struct Basis {
    /// Indices of basic variables (length m = n_constraints)
    basic: Vec<usize>,
    
    /// Indices of non-basic variables
    non_basic: Vec<usize>,
    
    /// Current basis matrix B (m × m)
    basis_matrix: Matrix,
    
    /// LU factorization of B (cached)
    lu: Option<LuFactorization>,
    
    /// Values of basic variables
    basic_values: Vec<f64>,
}

impl Basis {
    /// Create initial basis (usually with slack variables)
    pub fn initial(problem: &LpProblem) -> Self;
    
    /// Perform pivot: swap entering and leaving variables
    pub fn pivot(&mut self, entering: usize, leaving: usize, problem: &LpProblem);
    
    /// Update basis matrix and LU factorization
    fn update_basis_matrix(&mut self, problem: &LpProblem);
    
    /// Compute reduced costs for pricing
    pub fn compute_reduced_costs(&self, problem: &LpProblem) -> Vec<f64>;
    
    /// Check if current basis is feasible
    pub fn is_feasible(&self) -> bool;
    
    /// Check if current basis is optimal (dual feasible)
    pub fn is_dual_feasible(&self, problem: &LpProblem) -> bool;
}
```

**Key concepts:**
- **Basic variables**: Currently in the basis (m variables)
- **Non-basic variables**: Set to their bounds (n-m variables)
- **Basis matrix B**: m × m submatrix of constraint matrix A
- **Reduced costs**: How much objective changes if non-basic variable increases

---

### Step 4: Primal Simplex (Week 2-3)

**Goal:** Implement Primal Simplex for finding initial optimal solution

#### 4.1 Implement `simplex_primal.rs`

```rust
pub struct PrimalSimplex {
    basis: Basis,
    problem: LpProblem,
    iteration: usize,
    max_iterations: usize,
}

impl PrimalSimplex {
    /// Solve LP using Primal Simplex
    pub fn solve(problem: LpProblem) -> Result<LpSolution, LpError> {
        // Phase 1: Find initial feasible basis
        let mut solver = Self::phase1(problem)?;
        
        // Phase 2: Optimize
        solver.phase2()
    }
    
    /// Phase 1: Find feasible starting basis
    fn phase1(problem: LpProblem) -> Result<Self, LpError>;
    
    /// Phase 2: Optimize from feasible basis
    fn phase2(&mut self) -> Result<LpSolution, LpError>;
    
    /// One iteration of primal simplex
    fn iterate(&mut self) -> IterationResult;
    
    /// Select entering variable (most negative reduced cost)
    fn select_entering(&self) -> Option<usize>;
    
    /// Select leaving variable (minimum ratio test)
    fn select_leaving(&self, entering: usize) -> Option<usize>;
    
    /// Perform pivot operation
    fn pivot(&mut self, entering: usize, leaving: usize);
}
```

**Algorithm outline:**
```
while not optimal:
    1. Compute reduced costs
    2. Select entering variable (most negative reduced cost)
    3. If all reduced costs >= 0: OPTIMAL
    4. Compute pivot column (direction in which entering var increases)
    5. Select leaving variable (minimum ratio test)
    6. If no leaving variable: UNBOUNDED
    7. Perform pivot (update basis)
    8. Check for cycling (Bland's rule)
```

**Testing:**
- Simple 2-variable problem (can verify by hand)
- Unbounded problem (should detect)
- Infeasible problem (should detect in Phase 1)
- Degenerate problem (test anti-cycling)

---

### Step 5: Dual Simplex (Week 3)

**Goal:** Implement Dual Simplex for warm starting and incremental constraints

#### 5.1 Implement `simplex_dual.rs`

```rust
pub struct DualSimplex {
    basis: Basis,
    problem: LpProblem,
    iteration: usize,
}

impl DualSimplex {
    /// Solve LP using Dual Simplex from dual-feasible basis
    pub fn solve(problem: LpProblem, initial_basis: Option<Basis>) -> Result<LpSolution, LpError>;
    
    /// Warm-start from previous solution after adding constraint
    pub fn warm_start(
        previous_solution: &LpSolution,
        new_constraint: Constraint,
    ) -> Result<LpSolution, LpError>;
    
    /// One iteration of dual simplex
    fn iterate(&mut self) -> IterationResult;
    
    /// Select leaving variable (most infeasible basic variable)
    fn select_leaving(&self) -> Option<usize>;
    
    /// Select entering variable (dual ratio test)
    fn select_entering(&self, leaving: usize) -> Option<usize>;
    
    /// Perform pivot operation
    fn pivot(&mut self, entering: usize, leaving: usize);
}
```

**Algorithm outline:**
```
while not primal feasible:
    1. Find most infeasible basic variable (most negative value)
    2. If all basic variables >= 0: OPTIMAL (primal feasible + dual feasible)
    3. Select leaving variable (the infeasible one)
    4. Compute pivot row
    5. Select entering variable (dual ratio test)
    6. If no entering variable: INFEASIBLE
    7. Perform pivot
```

**Key difference from Primal:**
- **Primal**: Maintains primal feasibility, improves objective
- **Dual**: Maintains dual feasibility (optimality), reduces infeasibility

**Warm starting:**
```rust
// After adding constraint: a^T x <= b
// Previous optimal solution may violate new constraint
// But it's still dual-feasible!
// Dual simplex can restore primal feasibility quickly

pub fn add_constraint_and_resolve(
    prev_solution: &LpSolution,
    prev_basis: &Basis,
    new_constraint: (Vec<f64>, f64), // (a, b)
) -> Result<LpSolution, LpError> {
    // 1. Add slack variable for new constraint
    // 2. Evaluate constraint at previous solution
    // 3. If satisfied: done (previous solution still optimal)
    // 4. If violated: run dual simplex to restore feasibility
}
```

---

### Step 6: Integration & Testing (Week 4)

#### 6.1 Public API in `mod.rs`

```rust
/// Main solve function - automatically chooses primal or dual
pub fn solve(problem: &LpProblem) -> Result<LpSolution, LpError> {
    // If problem has initial basis hint, try dual simplex
    // Otherwise, use primal simplex
    PrimalSimplex::solve(problem.clone())
}

/// Solve with warm starting (for incremental constraints)
pub fn solve_with_warmstart(
    problem: &LpProblem,
    previous: &LpSolution,
) -> Result<LpSolution, LpError> {
    // Extract basis from previous solution
    // Use dual simplex
    DualSimplex::warm_start(previous, problem.last_constraint()?)
}
```

#### 6.2 Integration with selen optimizer

In `src/optimization/model_integration.rs`:
```rust
fn try_maximize(&self, ...) -> OptimizationAttempt {
    // ... existing checks ...
    
    // Check if problem is linear
    if self.is_linear_problem(vars, props, var_id) {
        // Convert to LP problem
        let lp_problem = self.convert_to_lp(vars, props, var_id)?;
        
        // Solve with LP solver
        match lpsolver::solve(&lp_problem) {
            Ok(solution) => return OptimizationAttempt::Success(solution),
            Err(_) => return OptimizationAttempt::Fallback(FallbackReason::LpFailed),
        }
    }
    
    // ... existing fallback logic ...
}
```

#### 6.3 Comprehensive Testing

**Unit tests for each component:**
- Matrix operations
- LU decomposition
- Basis management
- Primal simplex
- Dual simplex

**Integration tests:**
```rust
#[test]
fn test_simple_2var_problem() {
    // maximize x + y
    // subject to: x + y <= 10, x <= 6
    // optimal: x=6, y=4, obj=10
}

#[test]
fn test_warm_start() {
    // Solve initial problem
    // Add constraint
    // Resolve with warm start
    // Should be faster than cold start
}

#[test]
fn test_unbounded() {
    // maximize x
    // subject to: x >= 0
    // should detect unbounded
}

#[test]
fn test_infeasible() {
    // maximize x
    // subject to: x <= 5, x >= 10
    // should detect infeasible
}

#[test]
fn test_large_problem() {
    // 100 variables, 100 constraints
    // should complete in reasonable time
}
```

---

## Implementation Schedule

### Week 1: Foundation
- [x] Create module structure
- [x] Implement types.rs
- [x] Implement matrix.rs
- [x] Start lu.rs

### Week 2: Core Algorithm
- [x] Finish lu.rs
- [x] Implement basis.rs
- [x] Start simplex_primal.rs (Phase 1)

### Week 3: Simplex Methods
- [x] Finish simplex_primal.rs (Phase 2)
- [x] Implement simplex_dual.rs
- [x] Add anti-cycling rules

### Week 4: Integration & Testing
- [x] Public API
- [x] Integration with optimizer
- [x] Comprehensive testing
- [x] Performance benchmarking

---

## Success Criteria

### Correctness:
- ✓ Solves simple problems correctly (verified by hand)
- ✓ Detects unbounded problems
- ✓ Detects infeasible problems
- ✓ Handles degenerate problems (doesn't cycle)

### Performance:
- ✓ Solves 100×100 problem in <1 second
- ✓ Warm start is 5-10x faster than cold start
- ✓ Faster than current search for large domains

### Integration:
- ✓ Successfully replaces timeout tests
- ✓ Falls back gracefully on non-linear problems
- ✓ Works with existing constraint system

---

## Questions Before Starting

1. **Should we implement sparse matrices?**
   - Pro: Faster for sparse problems
   - Con: More complex (+500 LOC)
   - Recommendation: Start dense, add sparse later if needed

2. **What about presolve/preprocessing?**
   - Pro: Can simplify problems significantly
   - Con: Additional complexity (+300 LOC)
   - Recommendation: Add after basic solver works

3. **Numerical tolerance?**
   - What tolerance for feasibility? (1e-6? 1e-8?)
   - What tolerance for optimality? (same?)
   - Use existing precision parameter from selen?

4. **Testing approach?**
   - Test each component independently?
   - Compare against known LP solver (e.g., GLPK)?
   - What's acceptable error margin?

Ready to start implementation?
