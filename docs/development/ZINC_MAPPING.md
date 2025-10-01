# FlatZinc to Selen Model Mapping

## Overview
This document describes the strategy for mapping FlatZinc AST nodes to Selen's Model API.

## Mapping Architecture

### Trait-Based Approach
```rust
// Pseudo-code
trait MapToModel {
    fn map(&self, model: &mut Model, context: &mut MappingContext) -> Result<(), MappingError>;
}

struct MappingContext {
    variables: HashMap<String, VarId>,
    arrays: HashMap<String, Vec<VarId>>,
    // Other context needed during mapping
}
```

### Two-Phase Mapping
1. **Phase 1: Variable Creation**
   - Map all variable declarations to Selen variables
   - Store mapping in context (FlatZinc name → Selen VarId)
   - Handle arrays and multi-dimensional arrays

2. **Phase 2: Constraint Posting**
   - Map constraint calls to Selen constraint API
   - Resolve variable references using context
   - Handle annotations (e.g., domain consistency)

## Variable Mapping

### Simple Variables

#### FlatZinc Boolean Variable
```
var bool: x;
```
**Selen API:**
```rust
let x = model.bool();
context.variables.insert("x".to_string(), x);
```

#### FlatZinc Integer Variable
```
var 1..10: x;
```
**Selen API:**
```rust
let x = model.int(1, 10);
context.variables.insert("x".to_string(), x);
```

#### FlatZinc Integer Variable (Set Domain)
```
var {1, 3, 5, 7}: x;
```
**Selen API:**
```rust
// Need to check if Selen supports sparse domains
// If not, may need to use range and post additional constraints
let x = model.int_with_domain(&[1, 3, 5, 7]);
context.variables.insert("x".to_string(), x);
```

#### FlatZinc Float Variable
```
var 0.0..1.0: x;
```
**Selen API:**
```rust
let x = model.float(0.0, 1.0);
context.variables.insert("x".to_string(), x);
```

### Array Variables

#### FlatZinc 1D Array
```
array[1..3] of var 1..10: x;
```
**Selen API:**
```rust
let x: Vec<VarId> = (0..3).map(|_| model.int(1, 10)).collect();
context.arrays.insert("x".to_string(), x);
```

#### FlatZinc 2D Array
```
array[1..2, 1..3] of var 1..10: x;
```
**Selen API:**
```rust
// Flatten to 1D internally
let x: Vec<VarId> = (0..6).map(|_| model.int(1, 10)).collect();
context.arrays.insert("x".to_string(), x);
// Store dimension info for proper indexing
```

## Constraint Mapping

### Arithmetic Constraints

#### FlatZinc: `int_plus(x, y, z)`
**Meaning:** z = x + y

**Selen API:**
```rust
// Assuming Selen has post! macro or programmatic API
post!(model, z == x + y);
// OR
model.constraint_eq(z, model.add(x, y))?;
```

#### FlatZinc: `int_times(x, y, z)`
**Meaning:** z = x * y

**Selen API:**
```rust
post!(model, z == x * y);
```

### Comparison Constraints

#### FlatZinc: `int_lt(x, y)`
**Meaning:** x < y

**Selen API:**
```rust
post!(model, x < y);
```

### Logical Constraints

#### FlatZinc: `bool_and(x, y, z)`
**Meaning:** z = x ∧ y

**Selen API:**
```rust
post!(model, z == (x && y));
```

### Global Constraints

#### FlatZinc: `all_different_int(x)`
**Meaning:** All elements in array x are different

**Selen API:**
```rust
let vars = context.arrays.get("x").unwrap();
model.all_different(vars)?;
```

#### FlatZinc: `array_int_element(idx, array, value)`
**Meaning:** value = array[idx]

**Selen API:**
```rust
let array_vars = context.arrays.get("array").unwrap();
let idx = context.variables.get("idx").unwrap();
let value = context.variables.get("value").unwrap();
model.element(*idx, array_vars, *value)?;
```

### Reification Constraints

#### FlatZinc: `int_eq_reif(x, y, b)`
**Meaning:** b ⇔ (x = y)

**Selen API:**
```rust
// Need to check if Selen supports reification
// If yes:
model.reify_eq(x, y, b)?;
// If no, may need to implement or decompose
```

## Solve Goal Mapping

### Satisfy
```
solve satisfy;
```
**Selen API:**
```rust
let solution = model.solve()?;
```

### Minimize
```
solve minimize x;
```
**Selen API:**
```rust
let x = context.variables.get("x").unwrap();
let solution = model.minimize(*x)?;
```

### Maximize
```
solve maximize x;
```
**Selen API:**
```rust
let x = context.variables.get("x").unwrap();
let solution = model.maximize(*x)?;
```

## Annotations

### Variable Annotations
FlatZinc variables can have annotations like:
```
var 1..10: x :: output_var;
```

**Handling:**
- Parse and store annotations
- Some annotations affect output format
- Some annotations are solver hints (may ignore initially)

### Constraint Annotations
```
constraint int_eq(x, y) :: domain;
```

**Handling:**
- Map to Selen's consistency levels if supported
- Otherwise, ignore non-critical annotations

## Error Handling

### Mapping Errors
```rust
enum MappingError {
    UndefinedVariable(String),
    UndefinedArray(String),
    UnsupportedConstraint(String),
    TypeMismatch { expected: String, found: String },
    InvalidDomain(String),
}
```

### Error Recovery
- Fail fast on critical errors (undefined variables, unsupported core constraints)
- Warn on unsupported annotations (continue mapping)

## Special Cases

### Parameter Variables (Constants)
FlatZinc distinguishes between `var` and `par` (parameters/constants):
```
par int: n = 5;
```

**Handling:**
- Store as constant in context (not a Selen variable)
- Substitute value when used in constraints

### Sets
FlatZinc supports set variables:
```
var set of 1..10: s;
```

**Handling:**
- Check if Selen supports set variables
- If not, may need to decompose to boolean array

## Implementation Plan

1. Implement `MappingContext` to track variables and arrays
2. Implement `MapToModel` trait for each AST node type
3. Implement two-phase mapping (variables, then constraints)
4. Add comprehensive error handling and validation
5. Test with FlatZinc examples
6. Handle special cases (parameters, sets, etc.)

## Open Questions

- Does Selen support sparse domains (e.g., `{1, 3, 5, 7}`)?
- Does Selen support reification for all comparison operators?
- Does Selen support set variables?
- How to handle multi-dimensional arrays (flatten vs native support)?

## References

- [FlatZinc 2.8.4 Spec](https://docs.minizinc.dev/en/latest/fzn-spec.html)
- Selen Model API: `/src/model/`
- Selen Constraints: `/src/constraints/`
