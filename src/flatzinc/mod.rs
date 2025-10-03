//! FlatZinc Parser and Integration
//!
//! This module provides functionality to parse FlatZinc files and integrate them with Selen's solver.
//!
//! # FlatZinc Format
//!
//! FlatZinc is a low-level solver input language used by the MiniZinc constraint modeling language.
//! It consists of:
//! - Predicate declarations
//! - Variable declarations (var, array)
//! - Constraint statements
//! - Solve goals (satisfy, minimize, maximize)
//! - Output specifications
//!
//! # Modules
//!
//! - `tokenizer`: Lexical analysis - converts text to tokens
//! - `ast`: Abstract Syntax Tree structures
//! - `parser`: Recursive-descent parser
//! - `mapper`: Maps AST to Selen Model
//! - `error`: Error types and handling
//! - `output`: FlatZinc output formatter
//!
//! # Example
//!
//! ```ignore
//! use selen::prelude::*;
//!
//! let fzn_code = r#"
//!     var 1..10: x;
//!     var 1..10: y;
//!     constraint int_eq(x, y);
//!     solve satisfy;
//! "#;
//!
//! let mut model = Model::default();
//! model.from_flatzinc_str(fzn_code)?;
//! match model.solve() {
//!     Ok(solution) => println!("Solution found!"),
//!     Err(e) => println!("No solution: {:?}", e),
//! }
//! ```

mod error;
mod tokenizer;
mod ast;
mod parser;
mod mapper;
mod output;

pub use error::{FlatZincError, FlatZincResult};
pub use output::format_solution;

use crate::prelude::Model;

/// Parse FlatZinc tokens into AST and map to an existing Model.
///
/// This is an internal function used by Model::from_flatzinc_* methods.
pub(crate) fn parse_and_map(content: &str, model: &mut Model) -> FlatZincResult<()> {
    // Step 1: Tokenize
    let tokens = tokenizer::tokenize(content)?;
    
    // Step 2: Parse into AST
    let ast = parser::parse(tokens)?;
    
    // Step 3: Map AST to the provided Model
    mapper::map_to_model_mut(ast, model)?;
    
    Ok(())
}
