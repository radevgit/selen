# Step 6: Separable Mixed Problems - Detailed Implementation Plan

## Overview
**Goal**: Handle independent float/int variables efficiently by solving them separately and combining solutions.

**Current Status**: We have successfully completed Phases 1 & 2 (pure float optimization). Now we need to extend this to handle problems that mix integer and float variables but don't have coupling between them.

## Problem Definition
**Separable Mixed Problems** are those where:
- Some variables are integers, some are floats
- Integer variables have constraints only with other integers
- Float variables have constraints only with other floats  
- **No cross-type constraints** (int variable < float variable)

Example:
```rust
let int_x = model.new_var_int(1, 10);     // Integer scheduling variable
let int_y = model.new_var_int(1, 5);      // Integer resource count
let float_a = model.new_var_float(0.0, 100.0);  // Continuous position
let float_b = model.new_var_float(50.0, 200.0); // Continuous weight

// Integer constraints (solved with CSP)
model.not_equals(int_x, int_y);
model.less_than(int_x, int(8));

// Float constraints (solved with precision optimization) 
model.greater_than(float_a, float(25.5));
model.less_than(float_b, float(175.3));

// NO mixed constraints like: model.less_than(int_x, float_a)
```

---

## Step 6 Breakdown

### **Step 6.1: Mixed Problem Detection** (1 day)
**Goal**: Extend classification system to detect separable mixed problems

**Tasks**:
- Enhance `ProblemClassifier` to identify mixed variable types
- Add constraint analysis to detect cross-type dependencies
- Distinguish separable vs coupled mixed problems
- Add comprehensive test cases

**Files to modify**:
- `src/optimization/classification.rs`
- Add tests in `tests/test_mixed_classification.rs`

**Deliverable**: Classifier that correctly identifies:
- Pure float problems → Use precision optimization
- Pure integer problems → Use existing CSP  
- Separable mixed → Use new dual solver
- Coupled mixed → Fall back to existing CSP

---

### **Step 6.2: Variable Partitioning** (1 day)
**Goal**: Separate mixed problems into independent float and integer subproblems

**Tasks**:
- Create variable group analyzer
- Partition constraints by variable types
- Build separate submodels for each type
- Validate partitioning correctness

**Files to create/modify**:
- `src/optimization/variable_partitioner.rs`
- `src/optimization/submodel_builder.rs`
- Add tests in `tests/test_variable_partitioning.rs`

**Deliverable**: System that takes a mixed model and produces:
- Integer submodel with all integer variables and constraints
- Float submodel with all float variables and constraints
- Mapping between original and submodel variables

---

### **Step 6.3: Dual Solver Implementation** (2 days)
**Goal**: Solve integer and float subproblems independently

**Tasks**:
- Create mixed problem solver that coordinates both solvers
- Implement integer subproblem solving (use existing CSP)
- Implement float subproblem solving (use precision optimization)
- Handle solver failure cases and fallbacks

**Files to create/modify**:
- `src/optimization/mixed_solver.rs`
- `src/optimization/dual_coordination.rs`
- Enhance `src/optimization/model_integration.rs`

**Deliverable**: Solver that:
- Routes integer subproblems to existing CSP solver
- Routes float subproblems to precision optimization
- Handles cases where one solver succeeds and other fails

---

### **Step 6.4: Solution Merging** (1 day)
**Goal**: Combine independent solutions into a unified result

**Tasks**:
- Create solution merger that combines integer and float results
- Map submodel variable IDs back to original model
- Handle partial solutions (one subproblem unsolvable)
- Validate merged solution correctness

**Files to create/modify**:
- `src/optimization/solution_merger.rs`
- Enhance `src/solution.rs` if needed
- Add tests in `tests/test_solution_merging.rs`

**Deliverable**: System that produces a complete `Solution` containing:
- Values for all integer variables from integer subproblem
- Values for all float variables from float subproblem  
- Proper variable ID mapping to original model

---

### **Step 6.5: Integration & Testing** (1-2 days)
**Goal**: Integrate mixed solver into main solving pipeline

**Tasks**:
- Hook mixed solver into `Model::solve()` method
- Add automatic routing based on problem classification
- Create comprehensive test suite for mixed problems
- Performance benchmarking vs existing approach

**Files to modify**:
- `src/model.rs` - Update solve method
- `src/optimization/model_integration.rs` - Add mixed routing
- Create `tests/test_mixed_problems_comprehensive.rs`
- Add mixed problem benchmarks

**Deliverable**: Fully integrated system where:
- `model.solve()` automatically detects and optimizes mixed problems
- Performance gains demonstrated for separable problems
- All existing functionality remains unchanged

---

### **Step 6.6: Performance Validation** (0.5 day)
**Goal**: Validate expected performance improvements

**Tasks**:
- Create benchmarks comparing mixed vs traditional approach
- Measure performance on various separable problem sizes
- Document performance characteristics and limitations
- Validate expected 10-100x speedup claims

**Files to create**:
- `benchmarks/mixed_problem_performance.rs`
- Update documentation with performance results

**Deliverable**: Performance validation showing:
- Separable mixed problems solve faster than traditional CSP
- Float portions get precision optimization benefits
- Integer portions get existing CSP performance
- Clear documentation of when optimization applies

---

## Total Effort Estimate

| Subtask | Effort | Dependencies |
|---------|--------|--------------|
| 6.1 Mixed Detection | 1 day | None (extends existing classification) |
| 6.2 Variable Partitioning | 1 day | 6.1 complete |
| 6.3 Dual Solver | 2 days | 6.1, 6.2 complete |
| 6.4 Solution Merging | 1 day | 6.3 complete |
| 6.5 Integration & Testing | 1-2 days | 6.1-6.4 complete |
| 6.6 Performance Validation | 0.5 day | 6.5 complete |

**Total: 6.5-7.5 days** for complete separable mixed problem support

## Expected Results

### Performance Improvements:
- **Separable mixed problems**: 10-100x speedup (as predicted in original plan)
- **Float portions**: Get microsecond-level precision optimization
- **Integer portions**: Get existing CSP performance (no degradation)

### Engineering Applications:
- **Mixed scheduling**: Integer time slots + continuous resource allocation
- **Design optimization**: Discrete material choices + continuous dimensions
- **Manufacturing**: Integer part counts + continuous positioning/sizing

This breakdown makes Step 6 much more manageable while maintaining the ambitious performance goals of the original plan.
