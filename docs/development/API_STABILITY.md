# API Stability Guidelines - CSP Solver

## Overview

This document defines the API stability commitments for the CSP Solver library. These guidelines ensure users can rely on stable interfaces while allowing the library to evolve.

---

## 🔒 **API Stability Levels**

### **STABLE APIs** ✅
All current public APIs are considered stable and will remain compatible within major versions:

#### **Core Model Interface**
- `Model::default()`, `Model::with_config()`, `Model::with_float_precision()`
- Variable creation: `int()`, `float()`, `bool()`, `ints()`, `binary()`
- Constraint posting: `post!()`, `postall!()` macros
- Solving methods: `solve()`, `minimize()`, `maximize()`, `enumerate()`
- Iterator methods: `minimize_and_iterate()`, `maximize_and_iterate()`
- Embedded statistics: All solutions include `stats` field with propagation/node counts
- Solution access patterns and indexing

#### **Configuration System**
- `SolverConfig` struct and all builder methods
- Timeout and memory limit settings  
- Float precision configuration

#### **Error Handling**
- `SolverError` enum and all current variants
- `SolverResult<T>` type alias
- Error message formats (semantic meaning)

#### **Variable and Solution Types**
- `VarId`, `VarIdBin` types
- `Solution` struct and indexing operations
- `Val` enum for value representation

#### **Validation System**
- `ModelValidator` and related error types
- Automatic validation before solving

### **INTERNAL/DEVELOPMENT APIs** 🔧
These APIs are hidden from documentation and may change freely:

#### **Low-level Methods**
- Methods marked with `#[doc(hidden)]`
- Internal optimization algorithms
- Debugging and analysis utilities
- Performance benchmarking tools

---




#### **Deprecation Process**
1. **Mark deprecated** with `#[deprecated]` attribute
2. **Provide alternative** in deprecation message
3. **Maintain for 2 major versions** minimum
4. **Remove** in subsequent major version

```rust
#[deprecated(since = "0.8.0", note = "Use `solve()` instead")]
pub fn solve_any(&mut self) -> SolverResult<Solution> { ... }
```

---

## 📚 **API Documentation Standards**

### **Required Documentation Elements**
Every public API item must include:

1. **Purpose** - What the function/type does
2. **Parameters** - Description of each parameter
3. **Returns** - What the function returns
4. **Example** - Working code example
5. **Errors** - When the function can fail

### **Documentation Format**
```rust
/// Brief one-line description.
/// 
/// Longer description explaining the behavior, constraints, and usage patterns.
/// Multiple paragraphs are encouraged for complex functionality.
///
/// # Arguments
/// * `param1` - Description of first parameter
/// * `param2` - Description of second parameter
///
/// # Returns
/// * `Ok(T)` - Success case description
/// * `Err(SolverError)` - Error conditions
///
/// # Errors
/// Returns error when specific conditions occur.
///
/// # Example
/// ```
/// use selen::prelude::*;
/// let mut m = Model::default();
/// let result = m.some_method();
/// ```
pub fn some_method(&mut self) -> SolverResult<T> { ... }
```

---

