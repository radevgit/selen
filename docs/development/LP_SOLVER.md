# LP Solver Implementation Plan

## Context

Currently, optimization with large float domains (±1e6) times out because search explores ~262k combinations for 2 variables with 512 steps each. We need a Linear Programming (LP) solver for efficient optimization of problems like:

```rust
// maximize x subject to:
// x + y <= 8000
// x >= 2000
// x, y in [0, 1e6]
```

## Questions for Direction

### 1. Problem Scope
**Q: What types of optimization problems do we need to solve?**
- Pure linear objective with linear constraints? (e.g., maximize `x` subject to `x + y <= C`)
- Quadratic objectives? (e.g., maximize `x*y`)
- Non-linear constraints? (e.g., `x*y <= C`)
**A: Linear**

**Q: What's the typical problem size?**
- How many variables? (2-10? 10-100? 100+?)
- How many constraints? (5-20? 20-100? 100+?)
- Are problems typically dense or sparse?
**A: variables ~100 constraints ~100 Not sure about sparsity but lets assume dense**

### 2. Algorithm Choice

**Option A: Simplex Method**
- Classic, well-understood algorithm
- Good for small-medium problems
- Requires: pivoting operations, tableau management
- Complexity: typically O(n²m) where n=variables, m=constraints

**Option B: Interior Point Method**
- Better for large problems
- Requires: matrix factorization (Cholesky or LU)
- Complexity: O(n³) per iteration, typically 10-50 iterations
- Total: O(n³·log(1/ε)) where ε is precision
**Q: Why not SVD? Very stable on illpossed problems?**
**A: SVD is more stable BUT:**
- Cholesky: O(n³/3) operations
- LU: O(n³/3) operations  
- SVD: O(n²m + m³) operations (much slower, ~10x)
- Most LP problems are well-conditioned, don't need SVD's extra stability
- Can fall back to SVD only if Cholesky/LU numerically unstable
**What is the complexity here?**
**A: See above - O(n³) per iteration, but very predictable iteration count**

**Option C: Dual Simplex**
- Efficient for problems starting from infeasible point
- Good for incremental solving (adding constraints)
- Complexity: same as Simplex but often faster in practice
- **Key advantage**: Maintains feasibility of dual problem, not primal

**Primal vs Dual Simplex - When to Use:**

**Primal Simplex:**
- Start: feasible solution, non-optimal
- Each iteration: stays feasible, improves objective
- End: feasible + optimal
- **Best when**: Easy to find initial feasible solution
- Problem: Finding initial feasible point can be hard (needs Phase 1)

**Dual Simplex:**
- Start: optimal but infeasible (violates constraints)
- Each iteration: stays optimal for dual, reduces infeasibility
- End: feasible + optimal
- **Best when**: Adding constraints incrementally (CSP context!)
- **Best when**: Re-optimizing after adding constraints
- Problem: Need to start with dual-feasible basis

**Which is "better"?**
- **Not inherently better** - depends on context
- **CSP Integration**: Dual Simplex is VERY attractive because:
  - You often add constraints one by one during search
  - Previous solution becomes dual-feasible after adding constraint
  - Can "warm start" from previous solution
  - Much faster than re-solving from scratch

**Example for CSP:**
```rust
// Initial problem: maximize x, x ∈ [0, 100]
let solution1 = lp_solve(...); // x = 100

// Add constraint: x + y <= 50 (from CSP constraint propagation)
// Old solution x=100 is infeasible
// But it's dual-feasible!
// Dual simplex can efficiently find new solution

let solution2 = dual_simplex_with_warmstart(solution1, new_constraint);
// Much faster than solving from scratch
```

**Recommendation for selen:**
**Implement BOTH Primal and Dual Simplex** - they share 80% of code!
- Use Primal for initial solve
- Use Dual for incremental constraint addition
- Total: +200 LOC for dual variant

### Additional Questions to Guide Decision:

**Q1: For Constraint Solving Context**
In CSP solving, you typically:
- Add constraints one by one?
- Know all constraints upfront?
- Need to backtrack and remove constraints?

**A: This helps choose between:**
- Regular Simplex (all constraints known)
- Dual Simplex (incremental constraint addition)
- Need deletion? → More complex

**Q2: For Mixed-Integer Linear Programming (MILP)**
You said "float+integer if possible" - this requires:
- Base LP solver (any method)
- Branch-and-bound on top (adds ~500 LOC)
- Total complexity: LP_solver × number_of_branches

For 100 variables with 50 integers:
- Could need 2^50 branches in worst case (impractical)
- With good heuristics: ~100-1000 LP solves
- Is this acceptable? Or start with continuous-only?

**Q3: Problem Characteristics Matter**
With ~100 variables and ~100 constraints:
- **Simplex**: Could take 100-500 iterations (unpredictable)
  - Best case: O(n+m) iterations
  - Worst case: exponential (rare but possible)
  - Typical: O(n) to O(m) iterations
  - Each iteration: O(n²) work
  - Total typical: O(n³) but can be much worse

- **Interior Point**: Always 20-50 iterations (predictable)
  - Each iteration: O(n³) work (matrix factorization)
  - Total: O(n³·log(1/ε)) - guaranteed polynomial
  - For n=100: ~1 million operations per iteration
  - More stable for large problems

**Q4: Implementation Effort**
- **Simplex**: 600-1000 LOC, easier to debug
- **Interior Point**: 1200-1500 LOC, harder to debug
- **Matrix operations**: 300-400 LOC (needed for both)
- **MILP (branch-and-bound)**: +500 LOC on top

**Recommendation based on your answers:**

**Q: Which algorithm fits our use case best?**
- Do we need to solve many similar problems? (incremental solving?)
- Is warmstarting important? (reusing previous solution)

### 3. Implementation Complexity

**Simplex Implementation Needs:**
- [ ] Tableau representation (matrix)
- [ ] Pivoting operations (select entering/leaving variables)
- [ ] Basis management
- [ ] Two-phase method (for finding initial feasible solution)
- [ ] Bland's rule or similar (to avoid cycling)
- [ ] Numerical stability considerations

**Interior Point Needs:**
- [ ] Matrix factorization (Cholesky decomposition)
- [ ] KKT system solving
- [ ] Barrier function implementation
- [ ] Line search for step size
- [ ] More complex, but better scaling

**Q: How important is implementation simplicity vs performance?**
- Willing to trade some performance for simpler code? **Yes**
- Target LOC? (500 lines? 1000 lines? 2000 lines?) **Not sure about that**

### 4. Matrix Operations Required

**Basic Operations (both methods need):**
- Matrix-vector multiplication
- Vector operations (add, scale, dot product)
- Matrix transposition

**Simplex Specific:**
- Gaussian elimination (for tableau updates)
- Simple row operations
- **No need for LU decomposition** (can use direct pivoting)
- **No need for SVD** (not used in Simplex)

**Interior Point Specific:**
- **LU decomposition** (for solving linear systems)
- Cholesky decomposition (for positive definite systems)
- Possibly **SVD** for numerical stability in ill-conditioned problems

**Q: Should we implement matrix operations from scratch or use minimal dependencies?**
- Pure Rust, no dependencies? (most control, most work)
- Use a lightweight matrix library? (e.g., `nalgebra` ~50k LOC but well-tested)
**No dependecies, we need to implement everythink**

### 5. Integration with Current System

**Q: How should LP solver integrate with existing optimizer?**

Current flow:
```rust
Model::maximize(x) -> try_optimization -> precision_optimizer (fails) -> search (slow)
```

Proposed:
```rust
Model::maximize(x) -> try_optimization -> {
    if simple_single_var -> precision_optimizer
    if linear_objective_and_constraints -> lp_solver ← NEW
    else -> search
}
```
**A: not sure about the integration, but should be in separate dir in /src/lpsolver?**


**Q: Should LP solver handle mixed integer-float problems?**
- Just continuous (float) variables? (simpler)
- Also integer variables? (need branch-and-bound on top) **float+integer if possible**
- Just linear constraints? (simpler) **linear**
- Also nonlinear? (need different approach)

### 6. Fallback Strategy

**Q: What should happen when LP solver fails or is not applicable?**
- Fall back to current search? **yes**
- Return error indicating problem not suitable for LP?
- Try to approximate/relax the problem?

### 7. Numerical Precision

**Q: What precision requirements?**
- Exact feasibility required? (hard for floating point)
- Small tolerance acceptable? (e.g., 1e-6) **yes we already have parameter precision**
- How to handle numerical instability?

## Updated Recommendations (Based on Your Answers)

**Given:**
- ~100 variables, ~100 constraints (medium-large)
- No dependencies allowed (pure Rust)
- Prefer simplicity but need performance
- Want MILP eventually (float+integer)
- Fallback to search is acceptable

**Recommendation: START WITH SIMPLEX, PATH TO INTERIOR POINT**

### Phase 1: Revised Simplex (Primal + Dual) (Immediate - 2-3 weeks)
**Why Simplex first:**
- Simpler to implement and debug
- For n=100, often fast enough (if not degenerate)
- Foundation for understanding the problem
- Dual variant crucial for CSP incremental constraint addition
- ~1000-1200 LOC total

**Components:**
```
src/lpsolver/
  mod.rs              // Public API
  simplex_primal.rs   // Primal simplex (~400 LOC)
  simplex_dual.rs     // Dual simplex (~200 LOC, shares basis code)
  matrix.rs           // Dense matrix ops (~300 LOC)
  basis.rs            // Basis management (~200 LOC)
  lu.rs               // LU factorization (~200 LOC)
```

**Key features:**
- Revised Simplex (more efficient than tableau simplex)
- Bland's rule (prevent cycling)
- Two-phase method (handle infeasibility in primal)
- Dual simplex for warm-starting (CRUCIAL for CSP!)
- Uses LU factorization (for basis updates) ← Yes, Revised Simplex DOES use LU!

### Phase 2: Add LU Decomposition (~1 week)
Revised Simplex actually benefits from LU:
```
src/lpsolver/
  lu.rs               // LU decomposition (~200 LOC)
```
- Speeds up basis updates from O(n³) to O(n²)
- More numerically stable
- Essential for problems with n>50

### Phase 3: Interior Point (Future - if Simplex insufficient)
If Simplex is too slow/unstable for n=100:
```
src/lpsolver/
  interior_point.rs   // Primal-dual interior point (~500 LOC)
  cholesky.rs         // Cholesky decomposition (~200 LOC)
```
- Guaranteed O(n³·log(1/ε)) complexity
- Better for large problems
- Can reuse matrix.rs

### Phase 4: MILP via Branch-and-Bound (Future)
```
src/lpsolver/
  milp.rs             // Branch-and-bound (~500 LOC)
```
- Works with either LP solver
- Needs good branching heuristics
- Defer until continuous LP working well

## Implementation Priority

### Must Have (Phase 1):
- [x] Dense matrix operations
- [x] Revised Simplex algorithm
- [x] Basic LU decomposition (for Revised Simplex)
- [x] Two-phase method
- [x] Anti-cycling rule

### Nice to Have (Phase 2):
- [ ] Sparse matrix support (if problems are sparse)
- [ ] Better LU updates (Forrest-Tomlin)
- [ ] Preprocessing (remove redundant constraints)

### Future (Phase 3+):
- [ ] Interior Point method
- [ ] Cholesky decomposition
- [ ] MILP branch-and-bound
- [ ] SVD fallback (only if needed for stability)

## Key Decision Points

### Do you need more information about:

1. **Algorithm Details?**
   - How Simplex actually works (step-by-step)?
   - How Interior Point works?
   - Comparison of their actual performance on your problems?

2. **Implementation Strategy?**
   - Should we start with simple tableau Simplex (easier) or Revised Simplex (faster)?
   - Matrix storage: dense vs sparse?
   - Numerical precision handling?

3. **Testing Strategy?**
   - How to validate correctness?
   - Benchmark problems to test against?
   - Performance targets?

4. **Integration Details?**
   - Exactly where in the code to integrate?
   - API design for the LP solver?
   - How to detect when a problem is suitable for LP?

## Summary Table

| Method | LOC | Complexity (n=100) | Predictable? | CSP Integration | Implementation |
|--------|-----|-------------------|--------------|-----------------|----------------|
| Primal Simplex | 800 | Variable (fast to slow) | No | Good | Easier |
| + Dual Simplex | +200 | Same | No | **Excellent** (warm start!) | +Easy |
| Interior Point | 1500 | Always ~30 iterations | Yes | Poor (no warm start) | Harder |
| Primal+Dual + LU | 1200 | Better than basic | No | **Best** | Medium |

**Key insight for CSP:**
- Dual Simplex allows **warm starting** when adding constraints during search
- This is HUGE for CSP where you incrementally add constraints
- Interior Point cannot warm-start effectively

## My Concrete Recommendation:

**Start with Revised Simplex (Primal + Dual) + LU decomposition**
- Total: ~1200 LOC
- Good balance of simplicity and performance
- **Dual Simplex is CRUCIAL for CSP** - allows warm starting
- LU decomposition helps both speed and stability
- Both variants share 80% of code
- If this fails for n=100, then consider Interior Point
- MILP can wait until continuous LP is solid

**Implementation order:**
1. Matrix operations + LU decomposition (foundation)
2. Basis management (shared by both variants)
3. Primal Simplex (for initial solves)
4. Dual Simplex (for incremental constraints) ← **This is the game-changer**
5. Two-phase method (handle infeasibility)

**Do you want me to:**
1. Write a detailed Revised Simplex implementation guide?
2. Start with simpler tableau Simplex first?
3. Provide more comparison data?
4. Something else?
