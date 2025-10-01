# FlatZinc Specification Reference

**Source**: https://docs.minizinc.dev/en/latest/fzn-spec.html  
**Fetched**: October 1, 2025

## Key Points for Implementation

### Expression Types (`<expr>`)

According to the grammar, expressions can be:

```bnf
<basic-literal-expr> ::= <bool-literal>
                       | <int-literal>
                       | <float-literal>
                       | <set-literal>

<basic-expr> ::= <basic-literal-expr>
              | <var-par-identifier>

<expr>       ::= <basic-expr>
               | <array-literal>
```

**Important**: In constraint arguments (`<constraint-item>`), expressions can be:
- Literals (bool, int, float, set)
- Identifiers (variables or parameters)
- Array literals

This means **literals can appear in ANY argument position**, not just the second argument!

### Constraint Syntax

```bnf
<constraint-item> ::= "constraint" <identifier> "(" [ <expr> "," ... ] ")" <annotations> ";"
```

Where each `<expr>` can be:
1. A literal value (int, float, bool, set)
2. A variable/parameter identifier
3. An array literal

### Common Patterns We Need to Support

1. **Symmetric constraints**: `int_eq(1, x)` and `int_eq(x, 1)` are both valid
2. **Reified with literals**: `int_eq_reif(5, x, b)` and `int_eq_reif(x, 5, b)` are both valid
3. **Mixed literals**: `int_lin_eq([2, 3], [x, y], 10)` - array of ints, array of vars, int literal

### Array Expressions

```bnf
<array-literal> ::= "[" [ <basic-expr> "," ... ] "]"
```

Array elements can be:
- Literals
- Identifiers (variables/parameters)
- **Mix of both**: `[x, 3, y, 5]` is valid!

## Testing Coverage Gaps

Based on the BNF, we should verify our mapper handles:

1. ✅ **Literals in first position**: `int_eq(5, x)` - FIXED
2. ✅ **Literals in reified constraints**: `int_eq_reif(5, x, b)` - FIXED
3. ❓ **Mixed arrays**: `constraint example([x, 3, y])` - Need to check
4. ❓ **Set literals**: `constraint set_in(x, {1, 3, 5})` - Need to check
5. ❓ **Float literals**: `constraint float_le(x, 3.14)` - Need to check
6. ❓ **Array access in constraints**: `constraint int_eq(arr[i], 5)` - Should work
7. ❓ **Empty arrays**: `constraint example([])` - Need to check

## Current Implementation Status

### Comparison Constraints
- ✅ `int_eq`, `int_ne`, `int_lt`, `int_le`, `int_gt`, `int_ge` - Support literals in either position
- ✅ All `*_reif` variants - Support literals in either position

### Linear Constraints
- ❓ `int_lin_eq`, `int_lin_le`, `int_lin_ne` - Need to verify array handling

### Array Constraints
- ❓ Need to verify mixed literal/variable arrays

### Missing Constraint Types
Based on error analysis:
- `int_plus`, `int_times`, `int_minus`, `int_div`, `int_mod` - Arithmetic operations
- `maximum_int`, `minimum_int` - Array aggregations
- `set_in` - Set membership
- `bool_le` - Boolean comparison

## BNF Grammar (Simplified)

```bnf
% Full model structure
<model> ::= 
  [ <predicate-item> ]*
  [ <par-decl-item> ]*
  [ <var-decl-item> ]*
  [ <constraint-item> ]*
  <solve-item>

% Constraint item
<constraint-item> ::= "constraint" <identifier> "(" [ <expr> "," ... ] ")" <annotations> ";"

% Expression types
<expr> ::= <basic-expr> | <array-literal>
<basic-expr> ::= <basic-literal-expr> | <var-par-identifier>
<basic-literal-expr> ::= <bool-literal> | <int-literal> | <float-literal> | <set-literal>

% Variable/parameter identifiers
<var-par-identifier> ::= [A-Za-z_][A-Za-z0-9_]*

% Array literals
<array-literal> ::= "[" [ <basic-expr> "," ... ] "]"

% Literals
<bool-literal> ::= "false" | "true"
<int-literal> ::= [-]?[0-9]+ | [-]?0x[0-9A-Fa-f]+ | [-]?0o[0-7]+
<float-literal> ::= [-]?[0-9]+.[0-9]+ | [-]?[0-9]+.[0-9]+[Ee][-+]?[0-9]+ | [-]?[0-9]+[Ee][-+]?[0-9]+
<set-literal> ::= "{" [ <int-literal> "," ... ] "}" | <int-literal> ".." <int-literal>
```

## Action Items

1. ✅ Support literals in first argument position for all comparison constraints
2. ✅ Support literals in reified constraints
3. ❓ Test and fix array literal handling with mixed identifiers/literals
4. ❓ Implement missing arithmetic constraints (`int_plus`, `int_times`, etc.)
5. ❓ Implement array aggregation constraints (`maximum_int`, `minimum_int`)
6. ❓ Implement set constraints (`set_in`)
7. ❓ Test edge cases: empty arrays, large numbers, hex/octal integers
