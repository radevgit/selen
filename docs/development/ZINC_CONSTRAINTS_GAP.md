# FlatZinc Constraints Gap Analysis

## Overview
This document analyzes the constraints required by FlatZinc and compares them with Selen's existing constraint library to identify gaps.

## Methodology

### Phase 1: FlatZinc Built-in Predicates (Option A)
1. Extract required built-in predicates from FlatZinc 2.8.4/2.9.4 specification
2. Categorize by constraint type (arithmetic, logical, global, etc.)
3. Map to Selen's existing constraints
4. Identify missing constraints

### Phase 2: Full MiniZinc Global Constraints (Option B - if needed)
1. Review all global constraints from https://docs.minizinc.dev/en/stable/lib-globals.html
2. Determine which are commonly used
3. Prioritize implementation

## FlatZinc Required Built-ins

(To be populated after analyzing the FlatZinc spec)

### Arithmetic Constraints
- `int_plus(x, y, z)` - z = x + y
- `int_minus(x, y, z)` - z = x - y
- `int_times(x, y, z)` - z = x * y
- `int_div(x, y, z)` - z = x / y
- `int_mod(x, y, z)` - z = x mod y
- `int_abs(x, y)` - y = |x|
- `int_min(x, y, z)` - z = min(x, y)
- `int_max(x, y, z)` - z = max(x, y)

### Comparison Constraints
- `int_eq(x, y)` - x = y
- `int_ne(x, y)` - x ≠ y
- `int_lt(x, y)` - x < y
- `int_le(x, y)` - x ≤ y
- `int_gt(x, y)` - x > y
- `int_ge(x, y)` - x ≥ y

### Logical Constraints
- `bool_not(x, y)` - y = ¬x
- `bool_and(x, y, z)` - z = x ∧ y
- `bool_or(x, y, z)` - z = x ∨ y
- `bool_xor(x, y, z)` - z = x ⊕ y
- `bool_eq(x, y)` - x ⇔ y
- `bool_clause(pos, neg)` - disjunction of literals

### Reification Constraints
- `int_eq_reif(x, y, b)` - b ⇔ (x = y)
- `int_lt_reif(x, y, b)` - b ⇔ (x < y)
- (and other reified versions)

### Global Constraints
- `all_different_int(x)` - all elements in array x are different
- `cumulative(s, d, r, b)` - cumulative scheduling
- `table_int(x, t)` - table constraint (extensional)
- `regular(x, Q, S, d, q0, F)` - regular language membership
- (more to be identified)

### Array Constraints
- `array_int_element(idx, array, value)` - value = array[idx]
- `array_var_int_element(idx, array, value)` - variable index
- (more to be identified)

### Set Constraints
- `set_in(x, S)` - x ∈ S
- `set_union(x, y, z)` - z = x ∪ y
- `set_intersect(x, y, z)` - z = x ∩ y
- (more to be identified)

## Selen's Existing Constraints

(To be populated by auditing `/src/constraints/`)

### Arithmetic
- Addition, Subtraction, Multiplication, Division
- Modulo, Absolute value
- Min, Max
- (complete list to be added)

### Comparison
- Equality, Inequality
- Less than, Less or equal
- Greater than, Greater or equal
- (complete list to be added)

### Logical
- Not, And, Or, Xor
- Implication, Equivalence
- (complete list to be added)

### Global Constraints
- All Different
- Element (array indexing)
- Table (extensional)
- (complete list to be added)

## Gap Analysis

(To be populated after both lists are complete)

### Critical Missing Constraints
(Must implement before FlatZinc integration)

### Nice-to-Have Constraints
(Can be added later)

### Mapping Strategy for Missing Constraints
(How to decompose or emulate missing constraints)

## Implementation Priority

### Phase 1: Essential Constraints
(Required for minimal viable FlatZinc support)

### Phase 2: Common Constraints
(Required for most FlatZinc examples)

### Phase 3: Advanced Constraints
(Required for full FlatZinc compatibility)

## Implementation Plan

1. Complete FlatZinc built-ins list (extract from spec)
2. Complete Selen constraints inventory (audit codebase)
3. Identify gaps and categorize by priority
4. Implement critical missing constraints
5. Test with FlatZinc examples
6. Iterate based on example failures

## References

- [FlatZinc 2.8.4 Spec](https://docs.minizinc.dev/en/latest/fzn-spec.html)
- [MiniZinc Global Constraints](https://docs.minizinc.dev/en/stable/lib-globals.html)
- Selen constraints: `/src/constraints/`
