# BNF Gap Analysis - FlatZinc Implementation

## Date: October 1, 2025
## Current Status: 75/86 (87.2%) - Batch 01

---

## Critical Missing Features

### 1. Parameter Array Handling ⚠️ HIGH PRIORITY

**Issue**: `extract_int_array()` doesn't handle parameter array identifiers.

**Current Error**: "Expected array of integers"

**Example**:
```flatzinc
array [1..7] of int: col_left = [0, 5, 3, 3, 5, 2, 0];
constraint some_constraint(col_left);  // Fails!
```

**Problem**:
- Parameter arrays are stored in `array_map` as VarIds (constant variables)
- `extract_int_array()` only handles `Expr::ArrayLit`, not `Expr::Ident`
- Many constraints (linear, element, gcc) expect coefficient/parameter arrays by name

**Affected Files** (2 in Batch 01):
- abc_endview.fzn
- balanced_brackets.fzn

**Solution**:
```rust
// Add to MappingContext:
pub(super) param_int_arrays: HashMap<String, Vec<i32>>,
pub(super) param_bool_arrays: HashMap<String, Vec<bool>>,

// Update extract_int_array():
pub(super) fn extract_int_array(&self, expr: &Expr) -> FlatZincResult<Vec<i32>> {
    match expr {
        Expr::ArrayLit(elements) => {
            elements.iter().map(|e| self.extract_int(e)).collect()
        }
        Expr::Ident(name) => {
            // Look up parameter array by name
            self.param_int_arrays.get(name)
                .cloned()
                .ok_or_else(|| FlatZincError::MapError {
                    message: format!("Parameter array '{}' not found", name),
                    line: None,
                    column: None,
                })
        }
        _ => Err(FlatZincError::MapError {
            message: "Expected array of integers or array identifier".to_string(),
            line: None,
            column: None,
        }),
    }
}

// In map_var_decl(), detect parameter arrays:
if let Type::Array { element_type, .. } = &decl.var_type {
    if let Type::Int | Type::IntRange(..) | Type::IntSet(..) = **element_type {
        // This is a parameter int array
        if let Some(Expr::ArrayLit(elements)) = &decl.init_value {
            let values: Vec<i32> = elements.iter()
                .map(|e| self.extract_int(e))
                .collect::<Result<_, _>>()?;
            self.param_int_arrays.insert(decl.name.clone(), values);
            return Ok(());
        }
    }
}
```

**Impact**: +2 files immediately, likely many more in other batches

---

### 2. Complex Initialization Support ⚠️ MEDIUM PRIORITY

**Issue**: Array initialization with expressions not supported.

**Current Error**: "Complex initialization not yet supported"

**Example**:
```flatzinc
array [1..3] of var 0..2: x;
array [1..3] of var 0..2: y = [x[1] + 1, x[2] + 1, x[3] + 1];  // Fails!
```

**Problem**:
- `map_var_decl()` only handles `Expr::IntLit`, `Expr::FloatLit`, `Expr::Ident`
- Doesn't handle arithmetic expressions, array access, or other complex expressions

**Affected Files** (3 in Batch 01):
- all_different_modulo.fzn
- alldifferent_modulo.fzn
- and.fzn

**Complexity**: Requires expression evaluation or constraint posting

**Solution Options**:
1. **Constraint-based**: Post equality constraints: `y[i] = <expr>`
2. **Expression evaluation**: Build expression tree, evaluate during init

**Recommendation**: Use constraint-based approach (simpler, more general)

---

### 3. Mixed Arrays (Variables + Literals) ✅ PARTIALLY FIXED

**Issue**: Some contexts expect pure variable arrays, fail on mixed arrays.

**Example**:
```flatzinc
constraint some_constraint([x, 3, y, 5]);  // May fail in some contexts
```

**Status**:
- ✅ `extract_var_array()` handles mixed arrays (creates constant VarIds for literals)
- ✅ Most constraints work with this
- ❓ Some specific constraints may still expect pure variables

**Affected Files** (Possibly):
- another_kind_of_magic_square.fzn (4 files with "Expected variable identifier")

**Action**: Investigate specific failing constraints

---

### 4. Element Constraint Edge Cases ⚠️ LOW PRIORITY

**Issue**: "Unsupported value type in array_var_int_element"

**Affected Files** (1 in Batch 01):
- averbach_1.3.fzn

**Likely Cause**: Parameter array used where variable array expected, or special array type

**Action**: Investigate specific file to see what's needed

---

### 5. Domain Size Limit 🔧 CONFIGURATION ISSUE

**Issue**: "Variable has domain size 864M which exceeds maximum of 10M"

**Affected Files** (1 in Batch 01):
- arrow.fzn

**Not a BNF Issue**: This is a Selen limitation for practical memory usage

**Options**:
1. Increase `MAX_SPARSE_SET_DOMAIN_SIZE` constant
2. Use interval-based domain representation for large domains
3. Skip such files (they're impractical anyway)

**Recommendation**: Document as known limitation (not a bug)

---

## BNF Compliance Status

### Expression Types ✅
- ✅ Boolean literals (true/false)
- ✅ Integer literals (decimal, hex, octal)
- ✅ Float literals
- ✅ Set literals ({1,2,3}, 1..10)
- ✅ Variable identifiers
- ✅ Array access (x[i])
- ✅ Array literals

### Constraint Arguments ✅
- ✅ Literals in any position
- ✅ Variables in any position
- ✅ Mixed arrays [x, 3, y]
- ✅ Reified constraints accept literals

### Variable/Parameter Declarations
- ✅ Simple var declarations
- ✅ Var declarations with simple initialization
- ⚠️ **Parameter arrays** (partial - stored but not retrievable)
- ❌ **Complex initialization expressions**

### Constraint Coverage

**Implemented** (41 constraints):
- ✅ Comparison: int_eq, int_ne, int_lt, int_le, int_gt, int_ge (+ reified)
- ✅ Linear: int_lin_eq, int_lin_le, int_lin_ne (+ reified)
- ✅ Arithmetic: int_plus, int_minus, int_times, int_div, int_mod, int_abs, int_min, int_max
- ✅ Boolean: bool_clause, bool2int, bool_le, bool_eq_reif
- ✅ Array aggregations: array_int_minimum, array_int_maximum, array_bool_and, array_bool_or
- ✅ Element: array_var_int_element, array_int_element, array_var_bool_element, array_bool_element
- ✅ Counting: count, count_eq
- ✅ Global: all_different, sort, set_in, set_in_reif, global_cardinality

**Common but Missing**:
- ❓ bool_and, bool_or, bool_not, bool_xor (may not be used in FlatZinc output)
- ❓ float_* constraints (if we support float variables)
- ❓ table_int, table_bool (table constraints)
- ❓ cumulative, diffn (scheduling constraints)
- ❓ circuit (TSP/routing constraint)

---

## Recommended Priorities

### Phase 1: Parameter Arrays (Immediate - Fixes 2 files)
1. Add `param_int_arrays` HashMap to MappingContext
2. Update `extract_int_array()` to handle `Expr::Ident`
3. Update `map_var_decl()` to detect and store parameter arrays
4. Test: abc_endview.fzn, balanced_brackets.fzn

### Phase 2: Complex Initialization (Medium - Fixes 3 files)
1. Add expression-to-constraint conversion
2. Handle arithmetic expressions in initialization
3. Post equality constraints for array elements
4. Test: all_different_modulo.fzn, alldifferent_modulo.fzn, and.fzn

### Phase 3: Edge Case Investigation (Low - Fixes 4-5 files)
1. Investigate "Expected variable identifier" failures
2. Investigate element constraint value type issue
3. Test specific workarounds
4. Test: another_kind_of_magic_square.fzn, averbach_1.4.fzn, etc.

---

## Expected Impact

| Phase | Files Fixed | New Success Rate | Effort |
|-------|-------------|------------------|--------|
| Current | 75/86 | 87.2% | - |
| + Phase 1 | 77/86 | 89.5% | Low (2-3 hours) |
| + Phase 2 | 80/86 | 93.0% | Medium (4-6 hours) |
| + Phase 3 | 84/86 | 97.7% | High (investigation needed) |
| Final (realistic) | 85/86 | 98.8% | arrow.fzn stays unfixed (domain limit) |

---

## BNF Conformance Summary

✅ **Expression Syntax**: Fully compliant  
✅ **Constraint Syntax**: Fully compliant  
✅ **Literal Handling**: Fully compliant  
⚠️ **Parameter Arrays**: Partial (storage yes, retrieval no)  
❌ **Complex Init**: Not implemented  
✅ **Constraint Coverage**: 41 constraints (good coverage for typical problems)

**Overall Conformance**: ~90% (main gap is parameter array retrieval)
