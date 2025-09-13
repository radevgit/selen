# Missing Constraints in post! Macro

## Currently Supported in post! Macro:
✅ Basic comparisons: `x < y`, `x <= y`, `x > y`, `x >= y`, `x == y`, `x != y`
✅ Typed constants: `x >= int(5)`, `y <= float(3.14)`
✅ Simple modulo: `x % 3 == 1` 
✅ Logical operations: `and(c1, c2)`, `or(c1, c2)`, `not(c1)`

## Missing Constraints:

### 1. Arithmetic Operations (Expression Variables)
```rust
// Current Model API:
let sum_var = model.add(x, y);     // Creates variable x + y
let diff_var = model.sub(x, y);    // Creates variable x - y  
let prod_var = model.mul(x, y);    // Creates variable x * y
let quot_var = model.div(x, y);    // Creates variable x / y
let total = model.sum(&[x, y, z]); // Creates variable x + y + z

// Proposed post! syntax:
post!(m, add(x, y) < z);           // (x + y) < z
post!(m, sub(x, y) >= int(0));     // (x - y) >= 0
post!(m, mul(x, y) == int(12));    // (x * y) == 12
post!(m, div(x, y) <= int(5));     // (x / y) <= 5
post!(m, sum([x, y, z]) == int(10)); // (x + y + z) == 10

// Or even more natural:
post!(m, x + y < z);               // Direct arithmetic syntax
post!(m, x - y >= int(0));
post!(m, x * y == int(12));
post!(m, x / y <= int(5));
```

### 2. Mathematical Functions
```rust
// Current Model API:
let abs_var = model.abs(x);        // Creates variable |x|
let min_var = model.min(&[x, y]);  // Creates variable min(x, y)
let max_var = model.max(&[x, y]);  // Creates variable max(x, y)

// Proposed post! syntax:
post!(m, abs(x) >= int(1));        // |x| >= 1
post!(m, min([x, y]) == int(5));   // min(x, y) == 5
post!(m, max([x, y]) <= int(10));  // max(x, y) <= 10
```

### 3. Boolean Operations
```rust
// Current Model API:
let and_var = model.bool_and(&[a, b, c]); // Creates variable (a AND b AND c)
let or_var = model.bool_or(&[a, b, c]);   // Creates variable (a OR b OR c)  
let not_var = model.bool_not(a);          // Creates variable (NOT a)

// Proposed post! syntax:
post!(m, bool_and([a, b, c]) == int(1));  // (a AND b AND c) == true
post!(m, bool_or([a, b, c]) == int(0));   // (a OR b OR c) == false
post!(m, bool_not(a) == int(1));          // (NOT a) == true
```

### 4. Global Constraints
```rust
// Current Model API:
model.alldifferent(vec![x, y, z]);  // All variables have different values

// Proposed post! syntax:
post!(m, alldifferent([x, y, z]));  // All variables different
post!(m, all_different([x, y, z])); // Alternative syntax
```

### 5. Enhanced Modulo Support
```rust
// Current support (literals only):
post!(m, x % 3 == 1);              // ✅ Works

// Missing support:
post!(m, x % y == int(0));          // x is divisible by y  
post!(m, modulo(x, y) != int(0));   // x is not divisible by y
```

## Implementation Priority:

### High Priority (Natural Mathematical Syntax):
1. **Arithmetic expressions**: `x + y < z`, `x * 2 <= int(10)`
2. **Mathematical functions**: `abs(x) >= int(1)`, `max(x, y) <= int(15)`
3. **Enhanced modulo**: `x % y == int(0)`

### Medium Priority (Global Constraints):
4. **All-different**: `alldifferent([x, y, z])`
5. **Boolean operations**: `bool_and([a, b]) == int(1)`

### Implementation Notes:
- Arithmetic operations create intermediate variables automatically
- Need to handle precedence correctly in expressions
- Boolean operations work with 0/1 integer variables
- Global constraints may need special syntax patterns

## Example Enhanced post! Usage:
```rust
use cspsolver::prelude::*;
use cspsolver::constraint_macros::*;

let mut m = Model::new();
let x = m.int(1, 10);
let y = m.int(1, 10); 
let z = m.int(1, 10);

// Enhanced arithmetic syntax
post!(m, x + y == z);              // Sum constraint
post!(m, x * 2 <= int(20));        // Multiplication  
post!(m, abs(x - y) >= int(1));    // Absolute difference

// Global constraints
post!(m, alldifferent([x, y, z])); // All different values

// Complex expressions
post!(m, max(x, y) + min(x, y) == x + y); // Mathematical identity
```