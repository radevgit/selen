# API Refactoring Plan: Constraint Functions → AST Architecture

**Date Created:** October 10, 2025  
**Branch:** `lp_solver_2`  
**Goal:** Unify constraint API through AST-based architecture with clean, generic function names

---

## 🎯 **Overview**

Replace the current dual-API system (old direct propagator API + new runtime API) with a single, unified API where:
1. **All constraints** go through standalone functions (not Model methods)
2. **All functions** create AST internally
3. **AST** flows through: Extract LP → Materialize → Propagators
4. **Result:** Clean API, no duplication, optimal LP integration

---

## 📋 **Implementation Phases**

### **Phase 1: Remove Old API & Implement New Function Names** 🔧

**Goal:** Replace 43 Model methods with 30 generic constraint functions

#### **1.1 Create New Constraint Functions Module**
- Create `src/constraints/functions.rs`
- Define all 30 generic constraint functions
- Functions return `Constraint` or `Expr` types (NOT VarId)
- Initially, functions create propagators directly (minimal change)

#### **1.2 Remove Old API Methods from Model**
Delete these from `src/constraints/api/`:
- ❌ `m.add()`, `m.sub()`, `m.mul()`, `m.div()`, `m.modulo()` → Use runtime API `x.add(y)`
- ❌ `m.min()`, `m.max()`, `m.sum()` → Use `min(vars)`, `max(vars)`, `sum(vars)`
- ❌ `m.abs()` → Use `abs(x)`
- ❌ `m.array_int_minimum()`, `m.array_float_minimum()` → Use `min(arr)`
- ❌ `m.array_int_maximum()`, `m.array_float_maximum()` → Use `max(arr)`
- ❌ `m.array_int_element()`, `m.array_float_element()` → Use `element(idx, arr, result)`
- ❌ `m.int_eq_reif()`, `m.float_eq_reif()` → Use `eq_reif(x, y, b)`
- ❌ `m.int_ne_reif()`, `m.float_ne_reif()` → Use `ne_reif(x, y, b)`
- ❌ `m.int_lt_reif()`, `m.float_lt_reif()` → Use `lt_reif(x, y, b)`
- ❌ `m.int_le_reif()`, `m.float_le_reif()` → Use `le_reif(x, y, b)`
- ❌ `m.int_gt_reif()`, `m.float_gt_reif()` → Use `gt_reif(x, y, b)`
- ❌ `m.int_ge_reif()`, `m.float_ge_reif()` → Use `ge_reif(x, y, b)`
- ❌ `m.int2float()`, `m.float2int_*()` → Use `to_float()`, `floor()`, `ceil()`, `round()`
- ✅ Keep `m.alldiff()` → Becomes `alldiff(vars)`
- ✅ Keep `m.table()` → Becomes `table(vars, tuples)`

#### **1.3 Update Prelude & Exports**
```rust
// In src/prelude.rs
pub use crate::constraints::functions::{
    // Arithmetic
    min, max, sum, abs, element,
    // Conversion
    to_float, floor, ceil, round,
    // Reified
    eq_reif, ne_reif, lt_reif, le_reif, gt_reif, ge_reif,
    // Boolean
    and, or, not, xor, implies,
    // Global
    alldiff, alleq, table, gcc, cumulative,
};
```

#### **1.4 Update All Examples & Tests**
- Update `examples/*.rs` to use new function names
- Update `tests/*.rs` to use new function names
- Keep `m.new()` as the ONLY constraint posting method

---

### **Phase 2: Route New Functions → AST Creation** 🔄

**Goal:** Make all constraint functions create AST (not propagators directly)

#### **2.1 Define Constraint & Expr Types**
```rust
// In src/constraints/functions.rs

/// A constraint that can be posted to a model
pub struct Constraint {
    pub(crate) kind: ConstraintKind,
}

/// An expression that can be used in constraints
pub enum Expr {
    Var(VarId),
    Add(Box<Expr>, Box<Expr>),
    Min(Vec<VarId>),
    Max(Vec<VarId>),
    Sum(Vec<VarId>),
    Element(VarId, Vec<VarId>),
    // ... etc
}
```

#### **2.2 Implement AST Creation for Each Function**
```rust
pub fn min(vars: &[VarId]) -> Expr {
    Expr::Min(vars.to_vec())
}

pub fn eq_reif(x: VarId, y: VarId, b: VarId) -> Constraint {
    Constraint {
        kind: ConstraintKind::Reified {
            left: ExprBuilder::Var(x),
            op: ComparisonOp::Eq,
            right: ExprBuilder::Var(y),
            bool_var: b,
        }
    }
}

pub fn alldiff(vars: &[VarId]) -> Constraint {
    Constraint {
        kind: ConstraintKind::AllDifferent(vars.to_vec())
    }
}
```

#### **2.3 Update `model.new()` to Accept Constraints**
```rust
impl Model {
    pub fn new(&mut self, constraint: Constraint) {
        // Extract LP constraints from AST
        if let Some(lp_constraint) = extract_lp_constraint(&constraint.kind) {
            self.pending_lp_constraints.push(lp_constraint);
        }
        
        // Store AST for delayed materialization
        self.pending_constraint_asts.push(constraint.kind);
    }
}
```

#### **2.4 Extend ConstraintKind Enum**
Add variants for all constraint types:
```rust
pub enum ConstraintKind {
    Binary { left: ExprBuilder, op: ComparisonOp, right: ExprBuilder },
    And(Box<ConstraintBuilder>, Box<ConstraintBuilder>),
    Or(Box<ConstraintBuilder>, Box<ConstraintBuilder>),
    Not(Box<ConstraintBuilder>),
    
    // New variants for constraint functions:
    Reified { left: ExprBuilder, op: ComparisonOp, right: ExprBuilder, bool_var: VarId },
    AllDifferent(Vec<VarId>),
    Table { vars: Vec<VarId>, tuples: Vec<Vec<i32>> },
    Element { index: VarId, array: Vec<VarId>, result: VarId },
    Conversion { from: VarId, to: VarId, op: ConversionOp },
    // ... etc
}
```

---

### **Phase 3: LP Extraction from AST** 📊

**Goal:** Extract linear constraints from ALL constraint types

#### **3.1 Implement LP Extraction**
```rust
fn extract_lp_constraint(kind: &ConstraintKind) -> Option<LinearConstraint> {
    match kind {
        ConstraintKind::Binary { left, op, right } => {
            // Extract linear equality/inequality
            extract_linear_from_binary(left, op, right)
        }
        ConstraintKind::Element { .. } => {
            // Element constraints can sometimes be linearized
            None
        }
        ConstraintKind::AllDifferent(_) => {
            // Not linear - skip
            None
        }
        // ... handle other types
    }
}
```

#### **3.2 Update Materialization**
```rust
pub(crate) fn materialize_constraint_kind(model: &mut Model, kind: &ConstraintKind) {
    match kind {
        ConstraintKind::Reified { left, op, right, bool_var } => {
            let lvar = get_expr_var(model, left);
            let rvar = get_expr_var(model, right);
            match op {
                ComparisonOp::Eq => model.props.int_eq_reif(lvar, rvar, *bool_var),
                ComparisonOp::Lt => model.props.int_lt_reif(lvar, rvar, *bool_var),
                // ... etc
            };
        }
        ConstraintKind::AllDifferent(vars) => {
            model.props.all_different(vars);
        }
        ConstraintKind::Element { index, array, result } => {
            model.props.element(array.clone(), *index, *result);
        }
        // ... handle all constraint types
    }
}
```

---

### **Phase 4: Testing & Migration** ✅

#### **4.1 Test Each Constraint Type**
- Create tests for each of the 30 functions
- Verify LP extraction works correctly
- Verify materialization creates correct propagators
- Verify solutions are correct

#### **4.2 Update Documentation**
- Update README with new API examples
- Update tutorial/guide documentation
- Create migration guide from old → new API

#### **4.3 Update Examples**
Transform all examples:
```rust
// OLD:
let sum = m.add(x, y);
m.int_eq_reif(x, y, b);

// NEW:
m.new(x.add(y).eq(sum));
m.new(eq_reif(x, y, b));
```

---

## 📊 **Complete Function Mapping Reference**

### **Arithmetic (10 functions)**
```rust
min(vars: &[VarId]) -> Expr
max(vars: &[VarId]) -> Expr
sum(vars: &[VarId]) -> Expr
abs(x: VarId) -> Expr
element(index: VarId, array: &[VarId], result: VarId) -> Constraint
// Note: add, sub, mul, div, mod already in runtime API (x.add(y))
```

### **Conversion (4 functions)**
```rust
to_float(int_var: VarId, float_var: VarId) -> Constraint
floor(float_var: VarId, int_var: VarId) -> Constraint
ceil(float_var: VarId, int_var: VarId) -> Constraint
round(float_var: VarId, int_var: VarId) -> Constraint
```

### **Reified (6 functions)**
```rust
eq_reif(x: VarId, y: VarId, b: VarId) -> Constraint
ne_reif(x: VarId, y: VarId, b: VarId) -> Constraint
lt_reif(x: VarId, y: VarId, b: VarId) -> Constraint
le_reif(x: VarId, y: VarId, b: VarId) -> Constraint
gt_reif(x: VarId, y: VarId, b: VarId) -> Constraint
ge_reif(x: VarId, y: VarId, b: VarId) -> Constraint
```

### **Boolean (5 functions)**
```rust
and(vars: &[VarId]) -> Expr
or(vars: &[VarId]) -> Expr
not(x: VarId) -> Expr
xor(x: VarId, y: VarId) -> Constraint
implies(x: VarId, y: VarId) -> Constraint
```

### **Global (5 functions)**
```rust
alldiff(vars: &[VarId]) -> Constraint
alleq(vars: &[VarId]) -> Constraint
table(vars: &[VarId], tuples: Vec<Vec<i32>>) -> Constraint
gcc(vars: &[VarId], values: &[i32], counts: &[VarId]) -> Constraint
cumulative(...) -> Constraint
```

**Total: 30 functions** (down from 43 methods = 30% reduction)

---

## 🎯 **Success Criteria**

### **Phase 1 Complete:**
- ✅ All 30 new functions exist and compile
- ✅ Old API methods removed
- ✅ All examples updated
- ✅ All tests updated and passing

### **Phase 2 Complete:**
- ✅ All functions create AST internally
- ✅ `model.new()` accepts Constraint/Expr types
- ✅ Tests still passing

### **Phase 3 Complete:**
- ✅ LP extraction works for all linear constraints
- ✅ Materialization creates correct propagators
- ✅ No duplication in LP system
- ✅ Large domain tests pass quickly (with LP optimization)

### **Phase 4 Complete:**
- ✅ All constraint types tested
- ✅ Documentation updated
- ✅ Examples demonstrate new API
- ✅ Migration guide available

---

## 🚀 **Next Steps**

1. **Start Phase 1:** Create `src/constraints/functions.rs`
2. **Implement first function:** Start with `alldiff()` as example
3. **Iteratively add functions:** Add 2-3 functions at a time
4. **Test continuously:** Keep tests green throughout
5. **Proceed to Phase 2:** Once all 30 functions exist

---

## 📝 **Notes**

- **Don't fix old API:** Since we're removing it, no need to make it create AST
- **Runtime API stays:** `x.add(y).eq(z)` already works perfectly
- **Focus on new functions:** Clean slate with AST from the start
- **Incremental approach:** Each phase is independently valuable
- **Keep tests green:** Don't break working functionality

---

## 🔗 **Related Documents**

- Current API: `src/constraints/api/`
- Runtime API: `src/runtime_api/mod.rs`
- Propagators: `src/constraints/props/mod.rs`
- LP Integration: `src/lpsolver/csp_integration.rs`
- AST Types: `src/runtime_api/mod.rs` (ConstraintKind)

---

**Status:** 📋 **PLAN APPROVED - READY TO START PHASE 1**
