# FlatZinc Parser Design

## Overview
This document describes the parsing strategy for FlatZinc files, comparing different approaches and documenting the chosen design.

## Goals
- Parse FlatZinc 2.8.x/2.9.x spec-compliant `.fzn` files
- Provide clear error messages with line/column tracking
- Modular design for future extensibility
- No external dependencies (stdlib only)

## Parser Approach Options

### Option 1: Pure Recursive-Descent
- **Description**: Hand-written recursive functions for each grammar rule
- **Pros**:
  - Simple to implement and understand
  - Fast execution
  - Easy to debug and provide clear error messages
  - Full control over parsing logic
- **Cons**:
  - More boilerplate code
  - Expression precedence requires explicit handling
  - Less composable than combinators

### Option 2: Parser Combinators
- **Description**: Build parsers by composing smaller parser functions
- **Pros**:
  - Highly modular and reusable
  - Clean mapping from BNF to code
  - Easy to extend and test
- **Cons**:
  - Requires building combinator infrastructure from scratch (no external deps)
  - Can be slower than hand-written parsers
  - Backtracking can complicate error reporting

### Option 3: Hybrid (Recommended)
- **Description**: Tokenizer + Recursive-descent for statements + Pratt parser for expressions
- **Pros**:
  - Combines benefits of both approaches
  - Tokenizer handles whitespace, comments, line/column tracking
  - Recursive-descent for simple statement parsing
  - Pratt parser handles operator precedence elegantly
- **Cons**:
  - More complex architecture
  - Requires implementing Pratt parser for expressions

## Chosen Approach: Tokenizer + Recursive-Descent

Based on FlatZinc's structure (line-oriented, regular grammar), we'll use:

1. **Tokenizer (Lexer)**: Convert input string into token stream
   - Track line and column numbers for each token
   - Handle comments, whitespace, keywords, identifiers, literals
   - No lookahead needed (simple state machine)

2. **Recursive-Descent Parser**: Top-level statement parsing
   - Parse predicate declarations
   - Parse variable declarations
   - Parse constraint statements
   - Parse solve statement
   - Parse output statement

3. **Expression Parser**: Handle arithmetic/boolean expressions
   - For simple expressions: recursive-descent is sufficient
   - For complex precedence: consider Pratt parser if needed
   - (To be decided after analyzing FlatZinc expression complexity)

## FlatZinc Grammar Structure

(To be filled after analyzing spec)

### Top-level Structure
```
model ::= (predicate_item | var_decl_item | constraint_item | solve_item | output_item)*
```

### Key Productions
- Predicate declarations
- Variable declarations (var, array)
- Constraints
- Solve goals (satisfy, minimize, maximize)
- Output specifications

## Tokenizer Design

### Token Types
```rust
// Pseudo-code (to be implemented)
enum TokenType {
    // Keywords
    Predicate, Var, Array, Constraint, Solve, Satisfy, Minimize, Maximize,
    Int, Bool, Float, Set, Of,
    
    // Literals
    IntLiteral(i64),
    FloatLiteral(f64),
    BoolLiteral(bool),
    StringLiteral(String),
    
    // Identifiers
    Identifier(String),
    
    // Symbols
    Colon, Semicolon, Comma, DotDot,
    LParen, RParen, LBracket, RBracket, LBrace, RBrace,
    Equals, Plus, Minus, Star, Slash,
    
    // Special
    Eof,
}
```

### Location Tracking
Each token should include:
- Line number (1-based)
- Column number (1-based)
- Span (start and end positions)

## Error Handling

### Parse Error Structure
```rust
struct ParseError {
    message: String,
    line: usize,
    column: usize,
    expected: Option<Vec<String>>,
    found: Option<String>,
}
```

### Error Recovery
- Fail fast: stop on first error
- Provide clear error messages with context
- Example: "Expected ';' at line 42, column 15 in constraint declaration"

## Implementation Plan

1. Implement tokenizer with location tracking
2. Implement token stream wrapper (peek, consume, expect methods)
3. Implement recursive-descent parser for each grammar rule
4. Add comprehensive error handling
5. Test with FlatZinc examples

## Open Questions

- Should we support FlatZinc extensions or non-standard syntax?
- How should we handle version-specific features?
- Do we need incremental parsing (for large files)?

## References

- [FlatZinc 2.8.4 Spec](https://docs.minizinc.dev/en/latest/fzn-spec.html)
- BNF Grammar (at end of spec document)
