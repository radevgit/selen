# Integration with FlatZinc 

## Overview

This document provides a high-level overview of the FlatZinc integration with Selen. Detailed design documents are linked below.

**Implementation Directory**: `/src/zinc`

**Target Version**: FlatZinc 2.8.x/2.9.x (latest spec)

**Status**: Planning phase

## What is FlatZinc?

MiniZinc can export constraint satisfaction problems to FlatZinc format (*.fzn):
- **Flattening Process**: https://docs.minizinc.dev/en/stable/flattening.html
- **FlatZinc Specification**: https://docs.minizinc.dev/en/latest/fzn-spec.html
  - BNF grammar is at the end of the spec document
- **Latest MiniZinc Release**: 2.9.4

## Resources

### Examples
- **Google OR-Tools Examples**: https://github.com/google/or-tools/tree/stable/examples/flatzinc
- **Håkan Kjellerstrand's Examples**: https://www.hakank.org/minizinc/
- **GitHub Repository**: https://github.com/hakank/hakank/tree/master/minizinc
- **Local Examples**: `/zinc/ortools` (hidden from git, ~900 examples from small to large)

## Architecture

### High-Level Flow
1. **Import** `.fzn` model file
2. **Parse** using tokenizer + recursive-descent parser → AST
3. **Map** AST to Selen's Model API
4. **Solve** using Selen's solver
5. **Output** results in FlatZinc format

### Detailed Design Documents
- **[ZINC_PARSING.md](ZINC_PARSING.md)** - Parser design, tokenizer, recursive-descent vs hybrid approach
- **[ZINC_AST.md](ZINC_AST.md)** - AST structure, node types, trait-based design
- **[ZINC_CONSTRAINTS_GAP.md](ZINC_CONSTRAINTS_GAP.md)** - Constraint audit, gap analysis, implementation priority
- **[ZINC_MAPPING.md](ZINC_MAPPING.md)** - AST to Selen Model mapping strategy
- **[ZINC_OUTPUT.md](ZINC_OUTPUT.md)** - FlatZinc output format specification

## Design Decisions

### Versioning Strategy
- Target FlatZinc 2.8.x/2.9.x specification
- Parser should detect version (if specified in file)
- Design for extensibility to support future spec changes
- **Note**: FlatZinc spec change frequency and change tracking location TBD

### Parser Approach
- **Tokenizer**: Handle comments, whitespace, line/column tracking
- **Recursive-Descent**: Top-level statement parsing
- **Expression Parser**: To be decided (recursive-descent or Pratt for complex precedence)
- **No external dependencies**: Implement using Rust stdlib only

### Modularity
- Separate files for: tokenizer, parser, AST, mapper, output formatter
- Trait-based design: `AstNode`, `MapToModel`, `FlatZincFormatter`
- Clear boundaries between components
- Independently testable modules

### Constraint Coverage
- **Phase 1**: Audit FlatZinc required builtins (Option A)
- **Phase 2**: Consider full MiniZinc global constraints (Option B) if needed
- Implement missing critical constraints before integration
- See [ZINC_CONSTRAINTS_GAP.md](ZINC_CONSTRAINTS_GAP.md) for details

### Output Format
- Follow FlatZinc solution output specification
- Support satisfaction status and variable assignments
- Include solver statistics (optional)
- Support multiple solutions via function parameter (enumerate_all)
- No CLI flags for now (library-first approach)

### Error Handling
- **Fail fast**: Stop on first critical error
- **Line/column tracking**: Every token and AST node includes location
- **Clear error messages**: "Expected ';' at line 42, column 15 in constraint declaration"
- **No external error libraries**: Custom error types using Rust stdlib

### Testing Strategy
- All tests must pass even if FlatZinc example files are not present
- Cannot include example `.fzn` files in repo (legal reasons)
- Test with local examples in `/src/zinc/flatzinc` during development
- Unit tests for tokenizer, parser, mapper components
- Integration tests with representative models

### Integration Point
- Library API (no CLI tool initially)
- Public functions: `import_flatzinc_file()`, `import_flatzinc_str()`
- Returns fully constructed Selen `Model`
- Custom error type: `ZincImportError`

## Implementation Plan

### Phase 1: Analysis & Planning ✓
1. ✓ Review FlatZinc specification
2. ✓ Survey existing parsers (none found in Rust)
3. ✓ Design API and integration points
4. ✓ Create detailed design documents

### Phase 2: Constraint Audit (Current Phase)
1. Extract FlatZinc required builtins from spec
2. Audit Selen's existing constraints
3. Identify gaps and prioritize implementation
4. Implement missing critical constraints

### Phase 3: Parser Implementation
1. Implement tokenizer with location tracking
2. Implement recursive-descent parser
3. Build AST structures
4. Add comprehensive error handling
5. Test with simple FlatZinc examples

### Phase 4: Mapping & Solver Integration
1. Implement AST to Selen Model mapping
2. Handle variable declarations and arrays
3. Map constraints to Selen API
4. Handle solve goals (satisfy, minimize, maximize)
5. Test with complex FlatZinc examples

### Phase 5: Output Formatting
1. Implement FlatZinc output formatter
2. Support all variable types
3. Handle multiple solutions
4. Add optional solver statistics
5. Test output against FlatZinc spec

### Phase 6: Testing & Refinement
1. Run all ~900 local examples
2. Fix bugs and edge cases
3. Optimize performance if needed
4. Document supported features and limitations
5. Add examples to demonstrate usage

## Open Questions

### Versioning
- How frequently does FlatZinc spec change?
- Where are spec changes tracked/documented?
- Should we support multiple spec versions simultaneously?

### Constraints
- Which FlatZinc builtins are most critical?
- Can we decompose missing global constraints?
- What's the fallback for unsupported constraints?

### Implementation
- Should we preserve comments in AST (for round-tripping)?
- How to handle unknown/unsupported annotations?
- Do we need incremental parsing for very large files?

### Testing
- Can we create synthetic minimal examples for CI?
- How to validate correctness without reference solver?

## Q&A Summary

**Q: What constraints/functionality is missing in Selen for the integration?**
A: To be determined in Phase 2 (constraint audit). See [ZINC_CONSTRAINTS_GAP.md](ZINC_CONSTRAINTS_GAP.md).

**Q: How to make the implementation modular?**
A: Separate files for each component (tokenizer, parser, AST, mapper, formatter). Trait-based design for extensibility. See design documents.

**Q: Do we need JSON format?**
A: No. Focus on FlatZinc text format only.

**Q: Combinator parser or recursive-descent?**
A: Tokenizer + recursive-descent for statements. Expression parser TBD. See [ZINC_PARSING.md](ZINC_PARSING.md) for detailed comparison.

## Next Steps

1. ✓ Create detailed design documents
2. **→ Fetch and analyze FlatZinc spec** (extract builtins, BNF grammar)
3. **→ Audit Selen constraints** (complete gap analysis)
4. Implement missing critical constraints
5. Begin parser implementation

## References

- [FlatZinc 2.8.4 Specification](https://docs.minizinc.dev/en/latest/fzn-spec.html)
- [MiniZinc Documentation](https://docs.minizinc.dev/)
- [MiniZinc Global Constraints](https://docs.minizinc.dev/en/stable/lib-globals.html)


FlatZinc examples:


Top FlatZinc Test Collections:
1. MiniZinc Benchmarks (Official - Best Source!)
URL: https://github.com/MiniZinc/minizinc-benchmarks
Size: ~100+ benchmark categories
Quality: Official MiniZinc Challenge benchmarks (2008-2012)
Content: Contains .mzn files + data that can be compiled to .fzn
Categories: Scheduling, routing, packing, graph coloring, puzzles, etc.
Examples: N-Queens, Sudoku, Job Shop, Vehicle Routing, Bin Packing, and many more
2. MiniZinc Examples Repository
URL: https://github.com/MiniZinc/minizinc-examples
Contains tutorial examples and small test cases
3. Hakank's MiniZinc Collection
URL: http://hakank.org/minizinc/
Size: 600+ MiniZinc models
Coverage: Huge variety - puzzles, optimization, scheduling, etc.
You saw references in your docs to open_global_cardinality_low_up.mzn
4. CSPLib (Constraint Satisfaction Problem Library)
URL: https://www.csplib.org/
Many problems have MiniZinc/FlatZinc versions available
5. Academic Solver Repositories
Chuffed: https://github.com/chuffed/chuffed (includes test cases)
Gecode: https://github.com/Gecode/gecode (MiniZinc integration tests)
OR-Tools: Google's optimization toolkit with FlatZinc support: https://github.com/google/or-tools/tree/stable/examples/flatzinc
6. MiniZinc Challenge Archives
URL: https://www.minizinc.org/challenge.html
Annual competition instances (more complex, larger scale)







