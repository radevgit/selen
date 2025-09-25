//! Specialized solvers for common problem domains.
//!
//! This module provides production-ready solvers for well-known problem types,
//! offering clean APIs without requiring manual constraint setup.

pub mod sudoku;

// Re-export main types for convenience
pub use sudoku::{SudokuSolver, SudokuResult, solve_sudoku, solve_sudoku_string};