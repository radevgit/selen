# Changelog

All notable changes to this project will be documented in this file.

## [0.12.0] - 2025-10-10
- Breaking API changes: Unified (Generic) API
- Fixes in LP Solver

## [0.11.0] - 2025-10-09
- LP Solver integrated

## [0.10.0] - 2025-10-08
- Breaking API changes: remove constraint macros

## [0.9.4] - 2025-10-07
- fixes for unbounded variables

## [0.9.4] - 2025-10-06
- two fixes on unbounded variables and adaptive float step

## [0.9.3] - 2025-10-05
- additional int_lin_*, bool_lin_*, float_lin_* constraints and test coverage.

## [0.9.2] - 2025-10-04
- SolverConfig aligned with MiniZinc
- **New Feature**: Automatic bound inference for unbounded variables (integers and floats)
  - `Model::int(i32::MIN, i32::MAX)` now automatically infers reasonable bounds from context
  - `Model::float(f64::NEG_INFINITY, f64::INFINITY)` now automatically infers reasonable bounds
  - Configurable expansion factor via `SolverConfig::unbounded_inference_factor` (default: 1000x)

## [0.9.0] - 2025-10-03
- **Removed**: FlatZinc parser and integration (moved to [Zelen](https://github.com/radevgit/zelen)

## [0.8.7] - 2025-10-03
- FlatZinc Parser (deprecated - moved to Zelen in v0.9.0)

## [0.8.6] - 2025-10-01
- Linear Constraint Helpers
- Constrain propagation bug fixed
- Reified b ⇔ (x < y), b ⇔ (x ≤ y), b ⇔ (x > y), b ⇔ (x ≥ y)

## [0.8.5] - 2025-10-01
- Implemented reified constraint.

## [0.8.4] - 2025-09-27
- Fixed some outdated examples

## [0.8.3] - 2025-09-27
- Improved modularization
- Added example for Employee Scheduling

## [0.8.2] - 2025-09-26
- BitSet domain for GAC
- Performance improvement for alldiff()

## [0.8.1] - 2025-09-25
- Added specialized Sudoku solver.
- Organized benchmarks into dedicated directory structure

## [0.8.0] - 2025-09-23
- Renamed project to **Selen**, for a lot of reasons.

## [0.7.3] - 2025-09-22
- Comprehensive constraint macro system overhaul with modular architecture
- Fixed all core coverage test failures with missing constraint patterns
- Implemented graceful error handling replacing panic-based memory limit handling
- Enhanced constraint pattern matching for equality constraints (x == int(N), x == float(N))
- Organized constraint macros into logical modules (arithmetic, comparison, global, logical)
- Added comprehensive test coverage for constraint operations and edge cases
- Improved memory management with proper SolverError::MemoryLimit handling
- Enhanced validation system for complex constraint combinations

## [0.7.2] - 2025-09-21
- Modular constraint macro dispatch system implementation
- Separated constraint patterns into specialized modules for better maintainability
- Enhanced arithmetic constraint patterns (addition, multiplication, division, absolute value)
- Improved global constraint support (alldiff, allequal, element, min/max)
- Advanced logical operation patterns (and, or, not) with array syntax support
- Runtime API equivalents for all constraint macro functionality
- Comprehensive programmatic constraint building tests

## [0.7.1] - 2025-09-21
- Enhanced error handling infrastructure with proper Result types
- Improved constraint macro pattern matching for complex expressions
- Better support for array indexing in constraint expressions
- Enhanced mathematical function constraints (abs, sum, min, max)
- Optimized memory tracking and resource management
- Fixed constraint dispatch system for better performance

## [0.7.0] - 2025-09-21
- Hidden internal API methods from documentation
- Removed broken constraint builder imports
- Cleaned up public API documentation

## [0.6.4] - 2025-09-20
- Memory safety improvements - eliminated all unsafe blocks
- Result-based error handling
- Major refactoring of module structure

## [0.6.3] - 2025-09-20
- Enhanced SolveStats with timing, memory, and problem size tracking
- Advanced constraints: between, cardinality, conditional
- Runtime API improvements (betw, atleast, atmost)
- Fixed all compile warnings

## [0.6.0] - 2025-09-19
- Runtime API for dynamic constraint building
- Enhanced statistics system
- Global constraints support
- Performance improvements

## [0.5.17] - 2025-09-18
- Documentation fixes

## [0.5.15] - 2025-09-18
- Between and cardinality constraints
- Table, count, and element constraints
- AllEqual constraint with array syntax
- Refactored precision optimizer

## [0.5.11] - 2025-09-16
- New functionality implemented

## [0.5.9] - 2025-09-15
- Model validation before solving
- Better static heuristics
- Fixed multiplication constraints

## [0.5.8] - 2025-09-15
- Better static heuristics
- Moved benchmarks

## [0.5.5] - 2025-09-14
- Indexed variable support
- Timeout and memory limit handling
- SolverConfig for time and memory configuration
- Simple SolverError enum

## [0.5.3] - 2025-09-13
- Support for large domains in sparse_set
- Boolean variable initialization improvements
- Documentation fixes

## [0.5.2] - 2025-09-13
- New macro for float int

## [0.5.1] - 2025-09-13
- Renamed domain creation

## [0.5.0] - 2025-09-13
- Post! and postall! macros
- Clean syntax for constraints
- AND/OR/NOT operators
- Min/max global constraints
- Precision optimization with ULP
- Core float bounds optimizer

## [0.3.15] - 2025-09-09
- Configurable model precision
- FloatInterval struct implementation
- Fixed intervals structure

## [0.3.12] - 2025-07-08
- Document examples
- Missing doc attributes

## [0.3.11] - 2025-06-08
- new_var_with_values method for creating variables with specific values
- Removed obsolete tests

## [0.3.9] - 2025-05-07
- Sparse_set implementation
- Fixed propagation bugs

## [0.3.7] - 2025-04-07
- AllDifferent GAC and sparse set
- Sudoku examples and image assets
- Hybrid branching strategies
- Next/Prev views

## [0.3.5] - 2025-03-20
- Node count implementation
- solve_with_callback functionality

## [0.3.4] - 2025-02-20
- Tests for variable creation and domain constraints
- GitHub Actions workflow
- Fixed mixed int float constraints

## [0.3.3-alpha] - 2025-01-20
- Search functionality with state management
- Renamed package from "csp" to "cspsolver"
- Initial package setup

## [0.1.x, 0.2.x] - 2016
- Old architecture based on miniCP