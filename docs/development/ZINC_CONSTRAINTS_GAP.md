# FlatZinc Constraints Gap Analysis

## Overview
This document analyzes the constraints required by FlatZinc and compares them with Selen's existing constraint library to identify gaps.

NOTE: We will extend the programmatic API only and not the macro `post!` one.

## Methodology

### Phase 1: FlatZinc Built-in Predicates (Option A) ✓
1. ✓ Extract required built-in predicates from FlatZinc 2.8.4/2.9.4 specification
2. ✓ Categorize by constraint type (arithmetic, logical, global, etc.)
3. ✓ Map to Selen's existing constraints
4. ✓ Identify missing constraints

### Phase 2: Full MiniZinc Global Constraints (Option B - if needed)
1. Review all global constraints from https://docs.minizinc.dev/en/stable/lib-globals.html
2. Determine which are commonly used
3. Prioritize implementation

## FlatZinc Required Built-ins

Based on the FlatZinc specification (https://docs.minizinc.dev/en/latest/fzn-spec.html) and the standard library reference (https://docs.minizinc.dev/en/latest/lib-flatzinc.html), FlatZinc solvers must support the following built-in predicates:

### Arithmetic Constraints (Integer)
- `int_plus(x, y, z)` - z = x + y
- `int_minus(x, y, z)` - z = x - y
- `int_times(x, y, z)` - z = x * y
- `int_div(x, y, z)` - z = x / y (integer division)
- `int_mod(x, y, z)` - z = x mod y
- `int_abs(x, y)` - y = |x|
- `int_min(x, y, z)` - z = min(x, y)
- `int_max(x, y, z)` - z = max(x, y)
- `int_negate(x, y)` - y = -x
- `int_lin_eq(coeffs, vars, constant)` - linear equation: Σ(coeffs[i] * vars[i]) = constant
- `int_lin_ne(coeffs, vars, constant)` - linear inequality: Σ(coeffs[i] * vars[i]) ≠ constant
- `int_lin_le(coeffs, vars, constant)` - linear inequality: Σ(coeffs[i] * vars[i]) ≤ constant

### Arithmetic Constraints (Float)
- `float_plus(x, y, z)` - z = x + y
- `float_minus(x, y, z)` - z = x - y
- `float_times(x, y, z)` - z = x * y
- `float_div(x, y, z)` - z = x / y
- `float_abs(x, y)` - y = |x|
- `float_min(x, y, z)` - z = min(x, y)
- `float_max(x, y, z)` - z = max(x, y)
- `float_negate(x, y)` - y = -x
- `float_lin_eq(coeffs, vars, constant)` - linear equation
- `float_lin_le(coeffs, vars, constant)` - linear inequality

### Comparison Constraints (Integer)
- `int_eq(x, y)` - x = y
- `int_ne(x, y)` - x ≠ y
- `int_lt(x, y)` - x < y
- `int_le(x, y)` - x ≤ y
- `int_gt(x, y)` - x > y
- `int_ge(x, y)` - x ≥ y

### Comparison Constraints (Float)
- `float_eq(x, y)` - x = y
- `float_ne(x, y)` - x ≠ y
- `float_lt(x, y)` - x < y
- `float_le(x, y)` - x ≤ y
- `float_gt(x, y)` - x > y
- `float_ge(x, y)` - x ≥ y

### Comparison Constraints (Boolean)
- `bool_eq(x, y)` - x ⇔ y
- `bool_ne(x, y)` - x ⊕ y
- `bool_lt(x, y)` - x < y (false < true)
- `bool_le(x, y)` - x ≤ y

### Logical Constraints
- `bool_not(x, y)` - y = ¬x
- `bool_and(x, y, z)` - z = x ∧ y
- `bool_or(x, y, z)` - z = x ∨ y
- `bool_xor(x, y, z)` - z = x ⊕ y
- `bool_clause(pos, neg)` - disjunction of literals: (∨ pos[i]) ∨ (∨ ¬neg[i])
- `bool_array_and(arr, result)` - result = ∧ arr[i]
- `bool_array_or(arr, result)` - result = ∨ arr[i]

### Reification Constraints
Reified versions with boolean result variable `b`:
- `int_eq_reif(x, y, b)` - b ⇔ (x = y)
- `int_ne_reif(x, y, b)` - b ⇔ (x ≠ y)
- `int_lt_reif(x, y, b)` - b ⇔ (x < y)
- `int_le_reif(x, y, b)` - b ⇔ (x ≤ y)
- `int_gt_reif(x, y, b)` - b ⇔ (x > y)
- `int_ge_reif(x, y, b)` - b ⇔ (x ≥ y)
- `int_lin_eq_reif(coeffs, vars, constant, b)` - b ⇔ linear equation holds
- `int_lin_le_reif(coeffs, vars, constant, b)` - b ⇔ linear inequality holds
- `int_lin_ne_reif(coeffs, vars, constant, b)` - b ⇔ linear inequality holds

Similar for float and boolean reification.

### Half-Reification Constraints
Half-reified versions with boolean implication:
- `int_eq_imp(x, y, b)` - b → (x = y)
- `int_lt_imp(x, y, b)` - b → (x < y)
- (and others...)

### Global Constraints
- `all_different_int(x)` - all elements in array x are pairwise different
- `all_equal_int(x)` - all elements in array x are equal
- `table_int(x, t)` - table constraint (extensional): tuple(x) ∈ t
- `array_int_element(idx, array, value)` - value = array[idx] (constant index)
- `array_var_int_element(idx, array, value)` - value = array[idx] (variable index)
- `array_int_minimum(min, array)` - min = minimum(array)
- `array_int_maximum(max, array)` - max = maximum(array)
- `count_eq(array, value, count)` - count = |{i : array[i] = value}|
- `cumulative(start, duration, resource, bound)` - cumulative scheduling constraint
- `regular(x, Q, S, d, q0, F)` - regular language membership
- `circuit(x)` - Hamiltonian circuit constraint
- `lex_lesseq(x, y)` - lexicographic ordering: x ≤ₗₑₓ y

### Set Constraints
- `set_in(x, S)` - x ∈ S (membership)
- `set_subset(x, y)` - x ⊆ y
- `set_union(x, y, z)` - z = x ∪ y
- `set_intersect(x, y, z)` - z = x ∩ y
- `set_diff(x, y, z)` - z = x \ y
- `set_symdiff(x, y, z)` - z = x △ y (symmetric difference)
- `set_card(s, c)` - c = |s| (cardinality)

### Array Constraints
- `array_bool_element(idx, array, value)` - array element for booleans
- `array_float_element(idx, array, value)` - array element for floats
- `array_set_element(idx, array, value)` - array element for sets

## Selen's Existing Constraints

Based on audit of `/src/constraints/`, `/src/model/`, and `/src/runtime_api/`:

### Arithmetic Operations ✓
- ✓ Addition (`add`, `+`)
- ✓ Subtraction (`sub`, `-`)
- ✓ Multiplication (`mul`, `*`)
- ✓ Division (`div`, `/`)
- ✓ Modulo (`modulo`, `%`)
- ✓ Absolute value (`abs`)
- ✓ Min (binary and n-ary: `min`)
- ✓ Max (binary and n-ary: `max`)
- ✓ Negation (unary `-`)
- ✓ Sum (n-ary: `sum`)

### Comparison Operators ✓
- ✓ Equality (`==`)
- ✓ Inequality (`!=`)
- ✓ Less than (`<`)
- ✓ Less or equal (`<=`)
- ✓ Greater than (`>`)
- ✓ Greater or equal (`>=`)

### Logical Operators ✓
- ✓ Not (`!`, `not`)
- ✓ And (`&&`, `and`)
- ✓ Or (`||`, `or`)
- ✓ Xor (`xor`)
- ✓ Implication (`implies`)
- ✓ Equivalence (via `==` for booleans)

### Global Constraints (Partial Support)
- ✓ All Different (`all_different`)
- ✓ All Equal (`all_equal`)
- ✓ Element (`element`) - array indexing with variable index
- ✓ Table (`table_constraint`) - extensional constraint
- ✓ Count (`count_constraint`) - count occurrences of value
- ✓ Between (`between_constraint`) - lower ≤ middle ≤ upper
- ❌ Cumulative - NOT supported
- ❌ Regular - NOT supported
- ❌ Circuit - NOT supported
- ❌ Lex_lesseq - NOT supported

### Float Support ✓
- ✓ Float variables with domain bounds
- ✓ Float arithmetic (add, sub, mul, div)
- ✓ Float comparisons
- ✓ Mixed int/float expressions

### Set Support ❌
- ❌ Set variables - NOT supported
- ❌ Set operations - NOT supported

### Reification Support ❌
- ❌ Reified constraints - NOT natively supported
- ❌ Half-reified constraints - NOT natively supported
- Note: May need to decompose reified constraints

### Linear Constraints (Partial)
- ✓ Can express via sum + comparison
- ❌ No specialized `int_lin_eq`, `int_lin_le` predicates
- Note: Can be implemented via decomposition

## Gap Analysis

### ✅ Fully Supported (No Action Required)
1. **Basic Arithmetic** - int/float: add, sub, mul, div, mod, abs, min, max, negate
2. **Basic Comparisons** - int/float/bool: eq, ne, lt, le, gt, ge
3. **Boolean Logic** - not, and, or, xor
4. **Core Global Constraints** - all_different, all_equal, element, table, count

### ⚠️ Partially Supported (Needs Extension or Decomposition)
1. **Linear Constraints** - Can express via `sum` + comparison, but FlatZinc has specialized predicates:
   - `int_lin_eq(coeffs, vars, constant)`
   - `int_lin_le(coeffs, vars, constant)`
   - `int_lin_ne(coeffs, vars, constant)`
   - **Action**: Implement specialized linear constraint predicates or map to sum
   
2. **Array Aggregates** - FlatZinc has specialized predicates:
   - `array_int_minimum(min, array)`
   - `array_int_maximum(max, array)`
   - **Action**: Map to Selen's `min(array)` and `max(array)`

3. **Boolean Array Operations**:
   - `bool_array_and(arr, result)`
   - `bool_array_or(arr, result)`
   - **Action**: Decompose to pairwise operations or implement specialized

### ❌ Not Supported (Must Implement or Decompose)
1. **Reification** - Critical for FlatZinc models:
   - `int_eq_reif(x, y, b)` and similar
   - Half-reification: `int_eq_imp(x, y, b)`
   - **Priority**: HIGH - Many FlatZinc models use reification
   - **Strategy**: 
     - Option A: Implement native reification support in Selen
     - Option B: Decompose to auxiliary variables and implications
     - **Recommendation**: Start with decomposition, optimize later

2. **Set Constraints** - FlatZinc supports set variables:
   - `set_in`, `set_union`, `set_intersect`, `set_diff`, `set_card`
   - **Priority**: MEDIUM - Not all models use sets
   - **Strategy**: 
     - Option A: Add set variable support to Selen
     - Option B: Encode sets as boolean arrays
     - **Recommendation**: Option B (boolean encoding) for initial support

3. **Specialized Global Constraints**:
   - `cumulative(start, duration, resource, bound)` - scheduling
   - `regular(x, Q, S, d, q0, F)` - automaton constraint
   - `circuit(x)` - Hamiltonian circuit
   - `lex_lesseq(x, y)` - lexicographic ordering
   - **Priority**: LOW - Less common, can defer
   - **Strategy**: 
     - Implement decompositions
     - Mark as unsupported initially
     - Add native support based on demand

4. **Bool Clause** - Disjunctive clauses:
   - `bool_clause(pos, neg)` - (∨ pos[i]) ∨ (∨ ¬neg[i])
   - **Priority**: MEDIUM - Used in SAT-like models
   - **Strategy**: Decompose to boolean operations

### Mapping Strategy for Missing Constraints

#### Reification Decomposition
```rust
// FlatZinc: int_eq_reif(x, y, b)  means  b ⇔ (x = y)
// Decomposition:
//   b = true  →  x = y
//   b = false →  x ≠ y
// Can be expressed with conditional constraints or auxiliary variables
```

#### Set Variable Encoding
```rust
// FlatZinc: var set of 1..n: s
// Encoding: array[1..n] of var bool: s_bits
// where s_bits[i] = true iff i ∈ s
```

#### Linear Constraint Mapping
```rust
// FlatZinc: int_lin_eq([2, 3], [x, y], 10)  means  2x + 3y = 10
// Selen: 
let sum = model.sum(&[model.mul(2, x), model.mul(3, y)]);
model.constraint_eq(sum, 10);
```

#### Bool Clause Decomposition
```rust
// FlatZinc: bool_clause([a, b], [c, d])  means  a ∨ b ∨ ¬c ∨ ¬d
// Selen:
let result = model.or(a, model.or(b, model.or(model.not(c), model.not(d))));
model.constraint_eq(result, true);
```

## Implementation Priority

### Phase 1: Essential for Minimal FlatZinc Support (Critical)
1. ✅ **Basic arithmetic and comparisons** - Already supported
2. ✅ **Boolean logic** - Already supported
3. ✅ **all_different** - Already supported
4. ✅ **element** - Already supported
5. ⚠️ **Linear constraints** - Implement mapping/decomposition to sum
6. ❌ **Reification (basic)** - Implement decomposition for `int_eq_reif`, `int_lt_reif`, etc.
7. ❌ **Bool clause** - Implement decomposition

**Estimated Effort**: 2-3 days
**Goal**: Run simple FlatZinc models (N-Queens, Sudoku, basic optimization)

### Phase 2: Common for Most FlatZinc Examples (High Priority)
1. ⚠️ **Array aggregates** - Map `array_int_minimum/maximum` to Selen's min/max
2. ⚠️ **Boolean array operations** - Implement `bool_array_and`, `bool_array_or`
3. ❌ **Set variable encoding** - Boolean array encoding for sets
4. ❌ **Set operations** - Decompose `set_in`, `set_union`, etc. using encoding
5. ❌ **Half-reification** - Implement decomposition for `_imp` variants
6. ❌ **Float reification** - Extend reification to float constraints

**Estimated Effort**: 3-5 days
**Goal**: Run ~70-80% of FlatZinc examples from test suite

### Phase 3: Advanced for Full Compatibility (Lower Priority)
1. ❌ **Cumulative** - Implement decomposition or native support
2. ❌ **Regular** - Implement automaton constraint (complex)
3. ❌ **Circuit** - Implement Hamiltonian circuit constraint
4. ❌ **Lex_lesseq** - Implement lexicographic ordering
5. ❌ **Advanced reification** - Linear constraint reification
6. ❌ **Native set variables** - Full set domain implementation (if needed)

**Estimated Effort**: 5-10 days
**Goal**: Full FlatZinc compatibility, run all examples

### Phase 4: Optimization (Future)
1. ❌ **Specialized propagators** - For linear constraints, cumulative, etc.
2. ❌ **Native reification** - Avoid decomposition overhead
3. ❌ **Native set domains** - More efficient than boolean encoding

## Recommendations

### Immediate Actions (Before Parser Implementation)
1. **Decide on Reification Strategy** - Native vs. decomposition
   - **Recommendation**: Start with decomposition to unblock parser work
   - Can optimize with native reification later

2. **Implement Linear Constraint Mapping** - Map `int_lin_*` to sum operations
   - Should be straightforward given existing `sum` API

3. **Document Unsupported Features** - Create clear error messages
   - Example: "Set constraints not yet supported in Selen FlatZinc integration"

### Iterative Approach
1. **Phase 1**: Implement essential constraints, test with simple examples
2. **Phase 2**: Add common constraints as needed by test suite
3. **Phase 3**: Implement advanced constraints on demand

### Testing Strategy
1. Start with hand-crafted minimal FlatZinc files
2. Progress to simple models (N-Queens, Sudoku)
3. Run subset of OR-Tools examples
4. Identify missing constraints from failures
5. Implement and iterate

## Implementation Checklist

### Before Parser Implementation
- [x] ✓ Audit Selen's existing constraints
- [x] ✓ Identify FlatZinc required builtins
- [x] ✓ Categorize gaps by priority
- [ ] Implement reification decomposition helpers
- [ ] Implement linear constraint mapping helpers
- [ ] Implement bool_clause decomposition
- [ ] Test constraint mapping with mock AST

### During Parser Implementation
- [ ] Add constraint mapping for each FlatZinc builtin
- [ ] Handle unsupported constraints gracefully
- [ ] Provide clear error messages with constraint names
- [ ] Log unsupported features for future work

### After Initial Integration
- [ ] Run FlatZinc test suite
- [ ] Collect statistics on unsupported features
- [ ] Prioritize missing constraints by usage frequency
- [ ] Implement Phase 2 and Phase 3 constraints as needed

## Constraint Mapping Table

| FlatZinc Predicate | Selen Mapping | Status | Priority |
|---|---|---|---|
| `int_plus(x, y, z)` | `model.add(x, y)` | ✓ | - |
| `int_minus(x, y, z)` | `model.sub(x, y)` | ✓ | - |
| `int_times(x, y, z)` | `model.mul(x, y)` | ✓ | - |
| `int_div(x, y, z)` | `model.div(x, y)` | ✓ | - |
| `int_mod(x, y, z)` | `model.modulo(x, y)` | ✓ | - |
| `int_abs(x, y)` | `model.abs(x)` | ✓ | - |
| `int_min(x, y, z)` | `model.min([x, y])` | ✓ | - |
| `int_max(x, y, z)` | `model.max([x, y])` | ✓ | - |
| `int_eq(x, y)` | `post!(model, x == y)` | ✓ | - |
| `int_ne(x, y)` | `post!(model, x != y)` | ✓ | - |
| `int_lt(x, y)` | `post!(model, x < y)` | ✓ | - |
| `int_le(x, y)` | `post!(model, x <= y)` | ✓ | - |
| `int_lin_eq(c, v, k)` | Decompose to sum | ⚠️ | HIGH |
| `int_lin_le(c, v, k)` | Decompose to sum | ⚠️ | HIGH |
| `int_eq_reif(x, y, b)` | Decompose | ❌ | HIGH |
| `int_lt_reif(x, y, b)` | Decompose | ❌ | HIGH |
| `bool_not(x, y)` | `model.not(x)` | ✓ | - |
| `bool_and(x, y, z)` | `model.and(x, y)` | ✓ | - |
| `bool_or(x, y, z)` | `model.or(x, y)` | ✓ | - |
| `bool_xor(x, y, z)` | `model.xor(x, y)` | ✓ | - |
| `bool_clause(p, n)` | Decompose | ❌ | MEDIUM |
| `all_different_int(x)` | `all_different(x)` | ✓ | - |
| `table_int(x, t)` | `table_constraint(x, t)` | ✓ | - |
| `array_int_element(i, a, v)` | `element(a, i, v)` | ✓ | - |
| `array_var_int_element(i, a, v)` | `element(a, i, v)` | ✓ | - |
| `array_int_minimum(m, a)` | `model.min(a)` | ✓ | - |
| `array_int_maximum(m, a)` | `model.max(a)` | ✓ | - |
| `count_eq(a, v, c)` | `count_constraint(a, v, c)` | ✓ | - |
| `cumulative(s, d, r, b)` | Not supported | ❌ | LOW |
| `regular(x, Q, S, d, q0, F)` | Not supported | ❌ | LOW |
| `circuit(x)` | Not supported | ❌ | LOW |
| `set_in(x, S)` | Encode as boolean | ❌ | MEDIUM |
| `set_union(x, y, z)` | Encode + decompose | ❌ | MEDIUM |

## Implementation Plan

1. ✓ Complete FlatZinc built-ins list (extracted from spec)
2. ✓ Complete Selen constraints inventory (audited codebase)
3. ✓ Identify gaps and categorize by priority
4. → **Next**: Implement critical missing constraint mappings
5. → **Next**: Test with minimal FlatZinc examples
6. Iterate based on example failures

## References

- [FlatZinc 2.8.4 Spec](https://docs.minizinc.dev/en/latest/fzn-spec.html)
- [FlatZinc Standard Library](https://docs.minizinc.dev/en/latest/lib-flatzinc.html)
- [MiniZinc Global Constraints](https://docs.minizinc.dev/en/stable/lib-globals.html)
- Selen constraints: `/src/constraints/`, `/src/model/`, `/src/runtime_api/`
