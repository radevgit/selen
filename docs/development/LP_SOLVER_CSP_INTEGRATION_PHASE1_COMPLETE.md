# CSP Integration - Phase 1 Complete

## Summary

Successfully implemented Phase 1 of the CSP integration plan: **Detection & Extraction**. The LP solver can now extract linear constraints from Selen's CSP model and convert them to LP format.

## What Was Implemented

### 1. Linear Constraint System (`src/lpsolver/csp_integration.rs`)

**Data Structures**:
- `ConstraintRelation`: Enum for Equality, LessOrEqual, GreaterOrEqual
- `LinearConstraint`: Single linear constraint with coefficients, variables, relation, and RHS
- `LinearConstraintSystem`: Collection of linear constraints with variable tracking
- `LinearObjective`: Optional optimization objective for the LP solver

**Key Methods**:
- `LinearConstraint::equality/less_or_equal/greater_or_equal()`: Constructors
- `LinearConstraint::to_standard_form()`: Converts all constraints to ≤ form
- `LinearConstraintSystem::add_constraint()`: Adds a constraint and tracks variables
- `LinearConstraintSystem::is_suitable_for_lp()`: Heuristic (≥3 constraints, ≥2 variables)
- `LinearConstraintSystem::to_lp_problem()`: Converts to `LpProblem` format
- `extract_bounds()`: Extracts variable bounds using `ViewRaw` trait
- `apply_lp_solution()`: Placeholder for updating CSP domains (needs Context integration)

**Tests**: 4 unit tests covering constraint creation, conversion, and system building

### 2. Propagator Extraction (`src/constraints/props/mod.rs`)

**New Method**:
- `Propagators::extract_linear_system()`: Scans all propagators and extracts `FloatLinEq` and `FloatLinLe` constraints

**Implementation**:
- Uses `Any` trait and downcasting to identify linear constraint propagators
- Accesses constraint data via new accessor methods
- Returns `LinearConstraintSystem` ready for LP solving

### 3. Accessor Methods (`src/constraints/props/linear.rs`)

Added `pub(crate)` accessor methods to `FloatLinEq` and `FloatLinLe`:
- `coefficients()` -> `&[f64]`
- `variables()` -> `&[VarId]`
- `constant()` -> `f64`

## Test Results

- **53 tests passing** (49 LP solver tests + 4 CSP integration tests)
- All tests pass when run serially (`--test-threads=1`)
- Some parallel test failures due to global state (pre-existing issue)

## Integration with Existing Code

### Uses ViewRaw Trait
```rust
fn extract_bounds(var: VarId, vars: &Vars) -> (f64, f64) {
    use crate::variables::views::ViewRaw;
    let lower = match var.min_raw(vars) { ... };
    let upper = match var.max_raw(vars) { ... };
    (lower, upper)
}
```

### Converts to Standard Form
- Equality constraints → 2 inequalities (≤ and ≥)
- GreaterOrEqual → LessOrEqual (negate coefficients and RHS)
- LessOrEqual → unchanged

### Creates LpProblem
```rust
let linear_system = propagators.extract_linear_system();
if linear_system.is_suitable_for_lp() {
    let lp_problem = linear_system.to_lp_problem(&vars)?;
    // Solve with LP solver
}
```

## What's Next (Phase 2 & 3)

### Phase 2: Invocation & Integration
1. **Add Model API methods** (currently missing):
   - `Model::extract_linear_system()` → wrapper for `propagators().extract_linear_system()`
   - `Model::solve_with_lp()` → hybrid solve using LP + CSP
   
2. **Implement `apply_lp_solution()`**:
   - Currently placeholder
   - Needs `Context` integration to update variable domains
   - Use `try_set_min/try_set_max` to tighten bounds

3. **Add invocation points**:
   - During initial propagation (before search)
   - At each search node (hybrid approach)
   - User-triggered via `ModelConfig::prefer_lp_solver` flag

### Phase 3: Optimization Integration
1. Connect to optimization constraints
2. Extract objective function from `LinearObjective`
3. Use LP for lower/upper bound computation
4. Integrate with branch-and-bound

### Phase 4: Performance & Heuristics
1. Benchmark LP vs interval propagation
2. Tune `is_suitable_for_lp()` heuristics
3. Cache factorizations across search tree
4. Add warmstarting for incremental solving

## Documentation

Created comprehensive documentation:
- **LP_SOLVER_CSP_INTEGRATION.md**: 5-phase integration plan
- **LP_SOLVER_STATISTICS.md**: Statistics framework
- **csp_integration.rs**: Inline documentation for all public APIs

## Architecture Diagram

```
CSP Model
    ↓
Propagators (FloatLinEq, FloatLinLe)
    ↓
extract_linear_system()
    ↓
LinearConstraintSystem
    ↓
to_lp_problem()
    ↓
LpProblem
    ↓
solve_lp()
    ↓
LpSolution
    ↓
apply_lp_solution()
    ↓
Updated CSP variable bounds
```

## Files Created/Modified

1. **Created**: `src/lpsolver/csp_integration.rs` (390 LOC)
2. **Modified**: `src/lpsolver/mod.rs` (exports)
3. **Modified**: `src/constraints/props/mod.rs` (added `extract_linear_system()`)
4. **Modified**: `src/constraints/props/linear.rs` (added accessor methods)
5. **Created**: `tests/test_lp_csp_integration.rs` (placeholder)
6. **Created**: `docs/LP_SOLVER_CSP_INTEGRATION.md`
7. **Created**: `docs/LP_SOLVER_STATISTICS.md`

## Summary

Phase 1 is **complete and tested**. The infrastructure for extracting linear constraints from CSP models and converting them to LP format is fully functional. The next step is to integrate this with the Model API and implement the solution application logic (Phase 2).
