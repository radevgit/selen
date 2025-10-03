# Zelen Migration Guide

**Date:** October 3, 2025
**Purpose:** Split FlatZinc functionality from Selen into separate Zelen crate

## Overview

This guide documents the plan to split the Selen constraint solver project:
- **Selen** (this repo): Core CSP solver library
- **Zelen** (new repo): FlatZinc parser/frontend that depends on Selen

## Why Split?

1. ✅ Separation of concerns - Core solver vs FlatZinc frontend
2. ✅ Smaller dependencies for users who don't need FlatZinc
3. ✅ Independent versioning
4. ✅ Clear API boundary
5. ✅ Future CLI executable in Zelen

## What Moves to Zelen

### Source Code (~6,190 lines)
```
selen/src/flatzinc/                    → zelen/src/
├── ast.rs                             → ast.rs
├── error.rs                           → error.rs  
├── tokenizer.rs                       → tokenizer.rs
├── parser.rs                          → parser.rs
├── output.rs                          → output.rs
├── mod.rs                             → lib.rs (modified)
├── mapper.rs                          → mapper.rs
├── mapper_old_backup.rs               → DELETE
└── mapper/                            → mapper/
    ├── constraint_mappers.rs
    ├── helpers.rs
    └── constraints/
        ├── arithmetic.rs
        ├── array.rs
        ├── boolean.rs
        ├── comparison.rs
        ├── counting.rs
        ├── element.rs
        ├── global.rs
        ├── global_cardinality.rs
        ├── linear.rs
        ├── reified.rs
        ├── set.rs
        └── mod.rs
```

### Integration Code
```
selen/src/model/flatzinc_integration.rs → zelen/src/integration.rs
```

### Test Files (19 files)
```
selen/tests/test_flatzinc_*.rs         → zelen/tests/
selen/tests/test_batch_*.rs            → zelen/tests/
```

### Test Data (77MB)
```
selen/zinc/                            → zelen/zinc/
```

### Documentation
```
selen/docs/development/ZINC.md         → zelen/docs/FLATZINC.md
```

## What Stays in Selen

- ✅ Core CSP solver (`src/core/`, `src/search/`, `src/constraints/`)
- ✅ Variable system (`src/variables/`)
- ✅ Model system (`src/model/` minus flatzinc_integration.rs)
- ✅ Runtime API (`src/runtime_api/`)
- ✅ Optimization (`src/optimization/`)
- ✅ All non-FlatZinc tests
- ✅ Core documentation

## Step-by-Step Migration

### Phase 1: Manual Setup in Zelen (Your Part)

1. **Copy FlatZinc source code:**
```bash
cd /home/ross/devpublic
cp -r selen/src/flatzinc/* zelen/src/
rm zelen/src/mapper_old_backup.rs  # Remove backup file
```

2. **Copy integration code:**
```bash
cp selen/src/model/flatzinc_integration.rs zelen/src/integration.rs
```

3. **Copy tests:**
```bash
cp selen/tests/test_flatzinc_*.rs zelen/tests/
cp selen/tests/test_batch_*.rs zelen/tests/
```

4. **Copy test data:**
```bash
cp -r selen/zinc zelen/
```

5. **Copy documentation:**
```bash
cp selen/docs/development/ZINC.md zelen/docs/FLATZINC.md
```

6. **Update Zelen's Cargo.toml** (see below for complete file)

7. **Create Zelen's lib.rs** (see below for complete file)

### Phase 2: Automated Cleanup in Selen (AI Assistant)

Once Phase 1 is complete, switch to Selen workspace and the assistant will:

1. Remove `src/flatzinc/` directory
2. Remove `src/model/flatzinc_integration.rs`
3. Remove FlatZinc tests
4. Remove `zinc/` directory
5. Update `src/lib.rs` to remove flatzinc module
6. Update `src/prelude.rs` if needed
7. Update `src/model/mod.rs` if needed
8. Update version to 0.9.0 in Cargo.toml
9. Update README.md
10. Update CHANGELOG.md

### Phase 3: Update Zelen to Work with Selen

After Selen cleanup, switch to Zelen workspace and the assistant will:

1. Update imports to use `selen::` prefix
2. Create proper lib.rs structure
3. Update integration.rs
4. Update tests
5. Verify compilation
6. Run test suite (819/851 files should pass)

## Zelen Structure (Target)

```
zelen/
├── Cargo.toml
├── README.md
├── LICENSE
├── src/
│   ├── lib.rs              # Public API
│   ├── ast.rs
│   ├── error.rs
│   ├── tokenizer.rs
│   ├── parser.rs
│   ├── mapper.rs
│   ├── output.rs
│   ├── integration.rs      # Extends selen::Model
│   └── mapper/
│       ├── mod.rs
│       ├── constraint_mappers.rs
│       ├── helpers.rs
│       └── constraints/
│           ├── mod.rs
│           ├── arithmetic.rs
│           ├── array.rs
│           ├── boolean.rs
│           ├── comparison.rs
│           ├── counting.rs
│           ├── element.rs
│           ├── global.rs
│           ├── global_cardinality.rs
│           ├── linear.rs
│           ├── reified.rs
│           └── set.rs
├── tests/
│   ├── test_flatzinc_*.rs
│   └── test_batch_*.rs
├── zinc/                   # 77MB test data
│   └── ortools/
├── docs/
│   └── FLATZINC.md
└── examples/              # Future
    └── solve_flatzinc.rs  # CLI example
```

## Key Files for Zelen

### Cargo.toml
```toml
[package]
name = "zelen"
version = "0.1.0"
edition = "2024"
description = "FlatZinc parser and solver frontend for Selen CSP solver"
rust-version = "1.88"
categories = ["algorithms", "parser-implementations", "mathematics"]
keywords = ["flatzinc", "minizinc", "constraint-solver", "csp", "parser"]
license = "MIT"
homepage = "https://github.com/radevgit/zelen"
repository = "https://github.com/radevgit/zelen"
documentation = "https://docs.rs/zelen"

[dependencies]
# Use local path during development
selen = { version = "0.9", path = "../selen" }
# Or from crates.io after publishing:
# selen = "0.9"

[lib]
crate-type = ["lib"]

# Future CLI executable
[[bin]]
name = "zelen"
path = "src/bin/main.rs"
required-features = ["cli"]

[features]
default = []
cli = []  # Enable CLI executable

[dev-dependencies]
# Add if needed for tests
```

### lib.rs (Initial - Will be updated by assistant)
```rust
//! # Zelen - FlatZinc Frontend for Selen
//! 
//! Zelen provides FlatZinc parsing and integration with the Selen constraint solver.
//! 
//! ## Example
//! 
//! ```rust,ignore
//! use zelen::parse_flatzinc_file;
//! use selen::prelude::*;
//! 
//! let mut model = Model::default();
//! model.from_flatzinc_file("problem.fzn")?;
//! let solution = model.solve()?;
//! ```

pub mod ast;
pub mod error;
pub mod tokenizer;
pub mod parser;
pub mod mapper;
pub mod output;
pub mod integration;

pub use error::{FlatZincError, FlatZincResult};

// Re-export selen for convenience
pub use selen;

/// Prelude module for common imports
pub mod prelude {
    pub use crate::error::{FlatZincError, FlatZincResult};
    pub use crate::integration::*;
    pub use selen::prelude::*;
}
```

## Test Results Baseline

Before migration:
- **Selen**: 275+ tests passing in ~0.5s
- **FlatZinc**: 819/851 files passing (96.2%)
  - Batch 01: 86/86 (100%)
  - Batch 02: 81/86 (94.2%)
  - Batch 03: 85/86 (98.8%)
  - Batch 04: 85/86 (98.8%)
  - Batch 05: 80/86 (93.0%)
  - Batch 06: 83/86 (96.5%)
  - Batch 07: 76/86 (88.4%)
  - Batch 08: 81/86 (94.2%)
  - Batch 09: 81/86 (94.2%)
  - Batch 10: 81/81 (100%)

After migration, both should maintain these numbers.

## Important Notes

1. **Version Changes:**
   - Selen: 0.8.7 → 0.9.0 (breaking change - removed FlatZinc)
   - Zelen: Start at 0.1.0

2. **Dependencies:**
   - Use path dependency during development: `path = "../selen"`
   - Switch to version after publishing: `version = "0.9"`

3. **Tests:**
   - All 12 slow FlatZinc tests already marked with `#[ignore]`
   - Run with: `cargo test -- --ignored`

4. **Future CLI:**
   - Can add `src/bin/main.rs` later
   - Enable with `--features cli`

## Workflow

1. ✅ Read this guide
2. ✅ Do manual Phase 1 (copy files)
3. ✅ Stay in Selen workspace, let AI clean up
4. ✅ Switch to Zelen workspace, let AI setup
5. ✅ Test both projects
6. ✅ Commit changes to both repos

## Questions?

Refer back to this guide or the conversation context from October 3, 2025.
