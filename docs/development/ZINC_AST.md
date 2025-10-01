# FlatZinc AST Design

## Overview
This document defines the Abstract Syntax Tree (AST) structure for representing parsed FlatZinc models.

## Design Goals
- Strongly-typed representation of FlatZinc constructs
- Trait-based for modularity and extensibility
- Easy to map to Selen's Model API
- Preserve source location for error reporting

## AST Node Trait

```rust
// Pseudo-code
trait AstNode {
    fn location(&self) -> SourceLocation;
    fn validate(&self) -> Result<(), ValidationError>;
}

struct SourceLocation {
    line: usize,
    column: usize,
    span: (usize, usize), // start and end byte offsets
}
```

## Top-Level Structure

```rust
// Pseudo-code
struct FlatZincModel {
    predicates: Vec<PredicateDecl>,
    variables: Vec<VarDecl>,
    constraints: Vec<Constraint>,
    solve: SolveGoal,
    output: Option<OutputSpec>,
    location: SourceLocation,
}

impl AstNode for FlatZincModel { /* ... */ }
```

## Predicate Declarations

```rust
struct PredicateDecl {
    name: String,
    parameters: Vec<ParamDecl>,
    location: SourceLocation,
}

struct ParamDecl {
    name: String,
    param_type: ParamType,
    location: SourceLocation,
}

enum ParamType {
    VarInt,
    VarBool,
    VarFloat,
    ParInt,
    ParBool,
    ParFloat,
    VarSetOfInt,
    ParSetOfInt,
    Array { index_set: IndexSet, elem_type: Box<ParamType> },
}
```

## Variable Declarations

```rust
struct VarDecl {
    name: String,
    var_type: VarType,
    annotations: Vec<Annotation>,
    location: SourceLocation,
}

enum VarType {
    VarBool,
    VarInt { domain: IntDomain },
    VarFloat { domain: FloatDomain },
    VarSet { domain: SetDomain },
    Array { index_set: IndexSet, elem_type: Box<VarType> },
}

enum IntDomain {
    Range(i64, i64),
    Set(Vec<i64>),
}

enum FloatDomain {
    Range(f64, f64),
}

enum SetDomain {
    Range(i64, i64),
    Set(Vec<i64>),
}
```

## Constraints

```rust
struct Constraint {
    id: String, // predicate/constraint name
    arguments: Vec<Expr>,
    annotations: Vec<Annotation>,
    location: SourceLocation,
}
```

## Expressions

```rust
enum Expr {
    // Literals
    BoolLit(bool),
    IntLit(i64),
    FloatLit(f64),
    StringLit(String),
    
    // Variables
    Ident(String),
    
    // Arrays
    ArrayLit(Vec<Expr>),
    ArrayAccess { array: Box<Expr>, index: Box<Expr> },
    
    // Sets
    SetLit(Vec<i64>),
    SetRange(i64, i64),
    
    // Operations (if needed for complex expressions)
    BinaryOp { op: BinOp, left: Box<Expr>, right: Box<Expr> },
    UnaryOp { op: UnOp, operand: Box<Expr> },
}

enum BinOp {
    Add, Sub, Mul, Div, Mod,
    Eq, Neq, Lt, Le, Gt, Ge,
    And, Or, Xor,
}

enum UnOp {
    Neg, Not,
}
```

## Solve Goal

```rust
enum SolveGoal {
    Satisfy {
        annotations: Vec<Annotation>,
        location: SourceLocation,
    },
    Minimize {
        objective: Expr,
        annotations: Vec<Annotation>,
        location: SourceLocation,
    },
    Maximize {
        objective: Expr,
        annotations: Vec<Annotation>,
        location: SourceLocation,
    },
}
```

## Annotations

```rust
struct Annotation {
    name: String,
    arguments: Vec<Expr>,
    location: SourceLocation,
}
```

## Output Specification

```rust
struct OutputSpec {
    items: Vec<OutputItem>,
    location: SourceLocation,
}

enum OutputItem {
    Array(Vec<String>), // array of identifiers or expressions
    // To be refined based on FlatZinc output spec
}
```

## Design Decisions

### Strongly-Typed vs Enum-Based
- **Decision**: Use strongly-typed structs with trait implementations
- **Rationale**: 
  - Better type safety
  - Easier to extend
  - Clear mapping to Selen's API
  - Rust's pattern matching works well with enums

### Location Tracking
- Every AST node includes `SourceLocation`
- Enables precise error reporting during validation and mapping

### Validation
- AST nodes implement `validate()` method
- Checks type consistency, domain validity, etc.
- Separate from parsing (parser builds AST, then validate)

## Implementation Plan

1. Define core AST types (structs and enums)
2. Implement `AstNode` trait for each type
3. Add builder methods for easier construction (if needed)
4. Implement `Display` trait for debugging/pretty-printing
5. Add validation logic

## Open Questions

- Should we preserve comments in the AST (for round-tripping)?
- How to handle unknown/unsupported annotations?
- Should we have a separate IR (Intermediate Representation) layer?

## References

- [FlatZinc 2.8.4 Spec](https://docs.minizinc.dev/en/latest/fzn-spec.html)
