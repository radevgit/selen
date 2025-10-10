# LP Solver Completion Plan - October 10, 2025

## Current Status

### ✅ Completed
1. **Phase 1**: Generic constraint API (30+ functions)
2. **Phase 2**: Linear constraints create AST nodes
3. **Phase 2 Enhancement**: Expression-to-linear conversion
4. **API Cleanup**: Removed 24 old methods → 6 generic methods
5. **LP Extraction Bug Fix**: Exhaustive pattern matching
6. **LP BUILD Optimization**: 60x performance improvement (60s → <1s)
7. **Option 4**: Added simple reified methods to Model
8. **Documentation**: FlatZinc migration guide, analysis documents

### ⚠️ Current Limitations
1. **SIMPLEX Phase I slow** for large problems (agprice: >30s)
2. **No sparse matrix support** (deferred)
3. **Quadratic problems** only get linear relaxation
4. **Non-linear constraints** (multiplication) ignored by LP solver

---

## Remaining LP Solver Tasks

### Priority 1: Core Functionality Completion 🎯

#### Task 1.1: Validate LP Solver on Small Problems
**Goal**: Ensure LP solver works correctly for problems it should handle

**Subtasks**:
- [ ] Create comprehensive test suite for LP integration
  - Pure linear problems (2-10 variables)
  - Optimization problems (minimize/maximize)
  - Feasibility problems
  - Edge cases (unbounded, infeasible, etc.)
- [ ] Test equality, inequality, and mixed constraints
- [ ] Validate solution quality (compare to expected optimal)
- [ ] Test with both integer and float variables

**Files to create/modify**:
- `tests/test_lp_comprehensive.rs` (new)

**Time estimate**: 2-3 hours

---

#### Task 1.2: Handle LP Solver Failures Gracefully
**Goal**: Don't crash when LP solver can't solve problem

**Subtasks**:
- [ ] Detect unbounded problems (Phase II objective → infinity)
- [ ] Detect infeasible problems (Phase I can't find feasible solution)
- [ ] Add timeout detection in Phase I/II
- [ ] Fall back to CP propagation when LP fails
- [ ] Add user-visible warnings/errors

**Files to modify**:
- `src/lpsolver/simplex_primal.rs`
- `src/lpsolver/csp_integration.rs`
- `src/optimization/model_integration.rs`

**Time estimate**: 3-4 hours

---

#### Task 1.3: Problem Size Heuristics
**Goal**: Auto-disable LP for unsuitable problems

**Subtasks**:
- [ ] Add heuristic checks before LP solving:
  ```rust
  if vars * constraints > 50_000 {
      // Too large, use CP only
      return None;
  }
  if non_linear_count > 0.5 * total_constraints {
      // Mostly non-linear, LP won't help
      return None;
  }
  ```
- [ ] Add config option to force LP on/off
- [ ] Log why LP was skipped

**Files to modify**:
- `src/lpsolver/csp_integration.rs`
- `src/model/config.rs` (add LP settings)

**Time estimate**: 1-2 hours

---

### Priority 2: Documentation & User Experience 📚

#### Task 2.1: User-Facing Documentation
**Goal**: Help users know when and how to use LP solver

**Subtasks**:
- [ ] Create `docs/LP_SOLVER_GUIDE.md`:
  - What problems benefit from LP
  - What problems don't (non-linear, quadratic)
  - How to check if LP is being used
  - Performance expectations
  - Troubleshooting guide
- [ ] Add examples showcasing LP solver:
  - `examples/lp_diet_problem.rs`
  - `examples/lp_production_planning.rs`
  - `examples/lp_transportation.rs`

**Files to create**:
- `docs/LP_SOLVER_GUIDE.md`
- `examples/lp_*.rs` (3-4 examples)

**Time estimate**: 3-4 hours

---

#### Task 2.2: Debug Output Control
**Goal**: Let users control verbosity

**Subtasks**:
- [ ] Add `LpVerbosity` enum to config:
  ```rust
  pub enum LpVerbosity {
      Silent,      // No output
      Summary,     // Only final status
      Progress,    // Current (default)
      Detailed,    // Everything (for debugging)
  }
  ```
- [ ] Replace all `eprintln!` with conditional logging
- [ ] Add `LP:` prefix to all LP-related output

**Files to modify**:
- `src/model/config.rs`
- All LP solver files (wrap debug output)

**Time estimate**: 2-3 hours

---

### Priority 3: Performance & Scalability 🚀

#### Task 3.1: SIMPLEX Algorithm Improvements
**Goal**: Make Phase I faster for medium problems (50-100 vars)

**Subtasks**:
- [ ] Implement better initial basis selection:
  - Currently uses artificial variables
  - Try slack variables first (often feasible)
- [ ] Add iteration limit with timeout:
  ```rust
  if iterations > max_iterations || elapsed > timeout {
      return Err(SimplexError::Timeout);
  }
  ```
- [ ] Cache basis factorization when possible
- [ ] Use partial pricing (don't check all columns)

**Files to modify**:
- `src/lpsolver/simplex_primal.rs`

**Time estimate**: 4-6 hours (requires understanding simplex deeply)

---

#### Task 3.2: Sparse Matrix Support (DEFERRED)
**Status**: Postponed - would help large problems but adds complexity

**When to revisit**: After core functionality is solid

---

### Priority 4: Integration & Polish 🔧

#### Task 4.1: Better Error Messages
**Goal**: Help users understand what went wrong

**Current**:
```
LP: Solution status = Infeasible
```

**Better**:
```
LP Solver: Problem is infeasible
  - 225 variables, 486 constraints
  - Consider checking constraint consistency
  - Falling back to constraint propagation
```

**Files to modify**:
- `src/lpsolver/csp_integration.rs`
- `src/optimization/model_integration.rs`

**Time estimate**: 1-2 hours

---

#### Task 4.2: Configuration Options
**Goal**: Give users control over LP solver behavior

**Add to `SolverConfig`**:
```rust
pub struct LpConfig {
    pub enabled: bool,                    // Default: true
    pub max_vars: usize,                  // Default: 1000
    pub max_constraints: usize,           // Default: 2000
    pub phase_i_iterations: usize,        // Default: 10000
    pub phase_ii_iterations: usize,       // Default: 10000
    pub timeout_ms: Option<u64>,          // Per-phase timeout
    pub verbosity: LpVerbosity,           // Default: Summary
    pub fallback_to_cp: bool,             // Default: true
}
```

**Files to modify**:
- `src/model/config.rs`
- Use config throughout LP solver

**Time estimate**: 2-3 hours

---

## Implementation Roadmap

### Week 1: Core Functionality ✅ (DONE)
- ✅ LP BUILD optimization
- ✅ Basic LP integration working
- ✅ Simple problems solving

### Week 2: Robustness & Testing (CURRENT)
- **Day 1-2**: Task 1.1 - Comprehensive tests
- **Day 3**: Task 1.2 - Error handling
- **Day 4**: Task 1.3 - Problem size heuristics
- **Day 5**: Task 2.2 - Debug output control

### Week 3: Documentation & Polish
- **Day 1-2**: Task 2.1 - User documentation & examples
- **Day 3**: Task 4.1 - Better error messages
- **Day 4**: Task 4.2 - Configuration options
- **Day 5**: Testing & bug fixes

### Week 4: Performance (Optional)
- Task 3.1 - SIMPLEX improvements (if time permits)
- Profile and optimize bottlenecks
- Consider external solver integration

---

## Success Criteria

### Minimum Viable Product (MVP) ✅ ACHIEVED
- [x] LP solver works for small linear problems
- [x] Integrates with constraint propagation
- [x] Doesn't crash on non-linear problems
- [x] Documentation for migration

### Full Release (Target)
- [ ] Comprehensive test coverage (50+ tests)
- [ ] Graceful handling of all edge cases
- [ ] User documentation with examples
- [ ] Configurable behavior
- [ ] Performance acceptable for 10-50 variable problems
- [ ] Clear error messages

### Stretch Goals
- [ ] Solves 50-100 variable problems efficiently
- [ ] Sparse matrix support
- [ ] Revised simplex algorithm
- [ ] External solver integration option

---

## What NOT to Do (Scope Control)

❌ **Don't**:
1. Try to solve all NP-hard problems optimally
2. Support non-linear optimization (out of scope)
3. Compete with commercial solvers (Gurobi, CPLEX)
4. Implement quadratic programming
5. Add mixed-integer programming (MIP) cuts

✅ **Do**:
1. Focus on **linear programming** only
2. Provide **good error messages** when LP can't help
3. Make it **easy to use** for simple problems
4. **Fall back gracefully** to CP when needed
5. **Document limitations** clearly

---

## Next Immediate Steps

### Recommended Order:
1. **Task 1.1** - Comprehensive tests (validate what works)
2. **Task 1.2** - Error handling (make it robust)
3. **Task 2.2** - Debug output control (clean up logs)
4. **Task 1.3** - Problem size heuristics (prevent bad behavior)
5. **Task 2.1** - Documentation (help users)

### Quick Wins (Do First):
- **Task 2.2** (Debug output) - 2 hours, big UX improvement
- **Task 1.3** (Heuristics) - 1 hour, prevents bad cases
- **Task 4.1** (Error messages) - 1 hour, better UX

### Hardest (Save for Later):
- **Task 3.1** (SIMPLEX improvements) - Requires deep algorithm knowledge

---

## Decision Point: What Next?

**Option A: Quick Wins** (Recommended for today)
- Do Tasks 2.2 + 1.3 + 4.1 (~4 hours)
- Clean up output, add safety checks, better errors
- **Result**: Production-ready for small problems

**Option B: Testing First** (More thorough)
- Do Task 1.1 (comprehensive tests)
- Validate correctness before adding features
- **Result**: Confidence in what works

**Option C: Call It Done** (Ship it!)
- LP solver works for basic cases
- Document current limitations
- Move to other Selen features
- **Result**: MVP shipped, iterate later

---

**What would you like to tackle next?**
