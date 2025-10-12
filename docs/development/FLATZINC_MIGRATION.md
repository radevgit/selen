# Migrating FlatZinc-Exported Files to New Selen API

## Overview

If you have auto-generated Selen programs from FlatZinc (like `agprice_full.rs`), they use the **old type-specific API** that has been removed. This guide shows how to update these files to work with the **new generic API**.

## Problem Identification

Old FlatZinc exports use methods like:
```rust
model.float_lin_eq(&coeffs, &vars, rhs);
model.float_lin_le(&coeffs, &vars, rhs);
model.int_lin_eq(&coeffs, &vars, rhs);
model.int_lin_le(&coeffs, &vars, rhs);
```

These methods **no longer exist** in Selen. They have been replaced with generic methods that work for both int and float.

## Quick Fix Guide

### 1. Replace Old Linear Constraint Methods

**Find and Replace:**

| Old Method | New Method |
|------------|-----------|
| `model.float_lin_eq(` | `model.lin_eq(` |
| `model.float_lin_le(` | `model.lin_le(` |
| `model.float_lin_ne(` | `model.lin_ne(` |
| `model.int_lin_eq(` | `model.lin_eq(` |
| `model.int_lin_le(` | `model.lin_le(` |
| `model.int_lin_ne(` | `model.lin_ne(` |

**Example:**
```rust
// OLD (won't compile):
model.float_lin_eq(&vec![420.0, 1185.0, 6748.0, -1.0], 
                   &vec![cha, butt, milk, revenue], 
                   0.0);

// NEW (works):
model.lin_eq(&vec![420.0, 1185.0, 6748.0, -1.0], 
             &vec![cha, butt, milk, revenue], 
             0.0);
```

### 2. Quadratic Problems

If your FlatZinc export includes **quadratic terms** (like `agprice_full.rs` with `milksq`, `buttsq`, etc.), the LP solver **may not find the optimal solution** because it only handles linear constraints.

For quadratic programming:
- The model will still run (won't crash)
- You may get suboptimal solutions
- Consider using the expression API for better handling
- Future: Full quadratic programming support coming

### 4. Performance Benefits

After migration, **moderately-sized** problems with linear constraints will benefit from the **LP solver integration**:

**Before LP Integration:**
- Large domains (±1e6), small problems: 60+ seconds timeout ❌

**After LP Integration:**
- Large domains (±1e6), small problems: <1 second ✅

**Current Limitations:**
- Very large problems (250+ vars, 400+ constraints): LP BUILD phase may be slow
- Optimization work ongoing for large-scale problems
