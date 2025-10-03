# FlatZinc Constraint Mapper Coverage Report

**Test Suite**: Batch 01 (72/86 passing files)  
**Generated**: October 1, 2025  
**Analysis**: Actual constraint usage in passing test files

---

## Coverage Summary

### ✅ High Usage Constraints (>100 occurrences)

| Constraint | Count | Status | Notes |
|------------|-------|--------|-------|
| `array_bool_and` | 2,368 | ✅ Implemented | Fixed to accept bool literals |
| `int_eq_reif` | 1,640 | ✅ Implemented | Fixed to accept bool literals in result |
| `bool_le` | 1,079 | ✅ Implemented | Boolean implication |
| `int_ne` | 982 | ✅ Implemented | Added const-const handling |
| `int_lt_reif` | 905 | ✅ Implemented | Fixed to accept bool literals in result |
| `int_lin_eq_reif` | 863 | ✅ **NEW** | Reified linear equality |
| `int_eq` | 632 | ✅ Implemented | |
| `array_bool_or` | 565 | ✅ Implemented | Fixed to accept bool literals |
| `int_lin_eq` | 526 | ✅ Implemented | |
| `int_ne_reif` | 243 | ✅ Implemented | Fixed to accept bool literals in result |
| `bool2int` | 223 | ✅ Implemented | |
| `count` | 133 | ✅ **NEW** | Added as alias for count_eq |

### ✅ Medium Usage Constraints (10-100 occurrences)

| Constraint | Count | Status | Notes |
|------------|-------|--------|-------|
| `int_le_reif` | 98 | ✅ Implemented | Fixed to accept bool literals in result |
| `int_abs` | 96 | ✅ **NEW** | Absolute value (Phase 3 addition) |
| `int_le` | 90 | ✅ Implemented | |
| `int_lin_le` | 67 | ✅ Implemented | |
| `fzn_all_different_int` | 48 | ✅ Implemented | |
| `array_var_int_element` | 39 | ✅ Implemented | |
| `int_lt` | 26 | ✅ Implemented | |
| `int_plus` | 20 | ✅ **NEW** | Addition (Phase 3 addition) |
| `bool_eq_reif` | 16 | ✅ **NEW** | Boolean equality reification |

### ✅ Low Usage Constraints (<10 occurrences)

| Constraint | Count | Status | Notes |
|------------|-------|--------|-------|
| `int_max` | 7 | ✅ **NEW** | Maximum of 2 integers (Phase 3 addition) |
| `int_mod` | 5 | ✅ **NEW** | Modulo operation (Phase 3 addition) |
| `int_lin_le_reif` | 5 | ✅ **NEW** | Reified linear inequality |
| `int_min` | 4 | ✅ **NEW** | Minimum of 2 integers (Phase 3 addition) |
| `int_times` | 1 | ✅ **NEW** | Multiplication (Phase 3 addition) |
| `int_lin_ne` | 1 | ✅ Implemented | |

---

## New Implementations This Session

### High Impact (Phase 3 Critical Fixes)

1. **`get_var_or_const()` helper** - Handles mixed literal/variable arguments
   - **Impact**: Enabled 863 `int_lin_eq_reif` constraints
   - **Coverage**: Used by all reified constraints accepting boolean results

2. **Reified Comparison Constraints** - Boolean literal support
   - `int_eq_reif`: 1,640 uses
   - `int_lt_reif`: 905 uses
   - `int_ne_reif`: 243 uses
   - `int_le_reif`: 98 uses
   - **Total Impact**: 2,786+ constraint instances

3. **`count` constraint** - Alias for count_eq
   - **Uses**: 133 instances
   - **Files Fixed**: 5 battleship files

4. **Boolean literal in arrays** - Per BNF spec
   - **Impact**: Fixed `array_bool_and`, `array_bool_or`
   - **Total Uses**: 2,933 constraint instances

### Medium Impact (Phase 3 Arithmetic)

5. **`int_lin_eq_reif`** - Reified linear equality
   - **Uses**: 863 instances (HIGH!)
   - **Files**: Multiple puzzle files

6. **`int_lin_le_reif`** - Reified linear inequality
   - **Uses**: 5 instances
   - **Files**: Optimization problems

7. **`bool_eq_reif`** - Boolean equality reification
   - **Uses**: 16 instances
   - **Files**: Logic puzzles

### Low Impact (Phase 3 Arithmetic - Complete coverage)

8. **Arithmetic Operations** (Phase 3 additions)
   - `int_abs`: 96 uses
   - `int_plus`: 20 uses
   - `int_max`: 7 uses
   - `int_mod`: 5 uses
   - `int_min`: 4 uses
   - `int_times`: 1 use

---

## Constraints NOT Yet Tested in Batch 01

These were implemented but don't appear in the 72 passing files:

| Constraint | Status | Expected Usage |
|------------|--------|----------------|
| `int_minus` | ✅ Implemented | Should appear in subtraction problems |
| `int_div` | ✅ Implemented | Should appear in division problems |
| `int_gt` | ✅ Implemented | Alternative to int_lt |
| `int_ge` | ✅ Implemented | Alternative to int_le |
| `int_gt_reif` | ✅ Implemented | Alternative to int_lt_reif |
| `int_ge_reif` | ✅ Implemented | Alternative to int_le_reif |
| `array_int_minimum` | ✅ Implemented | Array aggregation |
| `array_int_maximum` | ✅ Implemented | Array aggregation |
| `array_int_element` | ✅ Implemented | Constant array indexing |
| `array_var_bool_element` | ✅ Implemented | Boolean array indexing |
| `array_bool_element` | ✅ Implemented | Constant boolean array indexing |
| `bool_clause` | ✅ Implemented | Clause constraints |
| `count_eq` | ✅ Implemented | Used via `count` alias |

**Note**: These constraints may be tested in other batches (02-10).

---

## Missing Constraints (Blocking 14 Files)

From the 14 failing files in Batch 01:

| Constraint | Occurrences | Priority |
|------------|-------------|----------|
| `global_cardinality` | 2 files | HIGH |
| `sort` | 1 file | MEDIUM |
| `set_in_reif` | 1 file | MEDIUM |

Other failures:
- 4 files: "Expected variable identifier or array access" (needs investigation)
- 3 files: "Complex initialization not yet supported"
- 1 file: Domain size limit issue
- 1 file: "Expected array of integers"
- 1 file: "Unsupported value type in array_var_int_element"

---

## Test Statistics

### Overall Coverage (Batch 01)
- **Total Files**: 86
- **Passing**: 72 (83.7%)
- **Failing**: 14 (16.3%)
- **Improvement This Session**: +18 files (+20.9 pp)

### Constraint Types Covered
- **Comparison**: 6 base + 6 reified = 12 ✅
- **Linear**: 3 base + 2 reified = 5 ✅
- **Arithmetic**: 8 operations ✅
- **Boolean**: 5 constraints ✅
- **Array**: 2 aggregations + 4 element = 6 ✅
- **Global**: 1 (all_different) ✅
- **Counting**: 1 (count/count_eq) ✅

**Total**: 38 constraint types implemented

---

## Recommendations

### High Priority (Next Session)
1. Investigate 4 "Expected variable identifier" errors - likely edge cases in expression handling
2. Add `global_cardinality` constraint (blocks 2 files)
3. Fix "Complex initialization" issues (blocks 3 files)

### Medium Priority
1. Add `sort` constraint (blocks 1 file)
2. Add `set_in_reif` constraint (blocks 1 file)
3. Test all constraint types in Batches 02-10 to find unused implementations

### Coverage Validation
Run coverage analysis on all 10 batches to identify:
- Which implemented constraints are actually used
- Dead code that can be removed or simplified
- Missing constraints that appear in other batches
