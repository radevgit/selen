//! Helper functions for extracting values from FlatZinc AST expressions

use crate::flatzinc::ast::*;
use crate::flatzinc::error::{FlatZincError, FlatZincResult};
use crate::flatzinc::mapper::MappingContext;
use crate::variables::VarId;

impl<'a> MappingContext<'a> {
    /// Get a variable by identifier
    pub(super) fn get_var(&self, expr: &Expr) -> FlatZincResult<VarId> {
        match expr {
            Expr::Ident(name) => {
                self.var_map.get(name).copied().ok_or_else(|| {
                    FlatZincError::MapError {
                        message: format!("Unknown variable: {}", name),
                        line: None,
                        column: None,
                    }
                })
            }
            _ => Err(FlatZincError::MapError {
                message: "Expected variable identifier".to_string(),
                line: None,
                column: None,
            }),
        }
    }
    
    /// Extract an integer value from an expression
    pub(super) fn extract_int(&self, expr: &Expr) -> FlatZincResult<i32> {
        match expr {
            Expr::IntLit(val) => Ok(*val as i32),
            Expr::Ident(name) => {
                // Could be a parameter - for now, just error
                Err(FlatZincError::MapError {
                    message: format!("Expected integer literal, got identifier: {}", name),
                    line: None,
                    column: None,
                })
            }
            _ => Err(FlatZincError::MapError {
                message: "Expected integer literal".to_string(),
                line: None,
                column: None,
            }),
        }
    }
    
    /// Extract an array of integers from an expression
    pub(super) fn extract_int_array(&self, expr: &Expr) -> FlatZincResult<Vec<i32>> {
        match expr {
            Expr::ArrayLit(elements) => {
                elements.iter().map(|e| self.extract_int(e)).collect()
            }
            _ => Err(FlatZincError::MapError {
                message: "Expected array of integers".to_string(),
                line: None,
                column: None,
            }),
        }
    }
    
    /// Extract an array of variables from an expression
    /// 
    /// Handles:
    /// - Array literals like `[x, y, z]` (may contain variables or integer constants)
    /// - Array identifiers that reference previously declared arrays
    /// - Single variable identifiers (treated as single-element array)
    pub(super) fn extract_var_array(&mut self, expr: &Expr) -> FlatZincResult<Vec<VarId>> {
        match expr {
            Expr::ArrayLit(elements) => {
                // Handle array literals that may contain variables or integer constants
                let mut var_ids = Vec::new();
                for elem in elements {
                    match elem {
                        Expr::Ident(name) => {
                            // Variable reference
                            let var_id = self.var_map.get(name).copied().ok_or_else(|| {
                                FlatZincError::MapError {
                                    message: format!("Unknown variable: {}", name),
                                    line: None,
                                    column: None,
                                }
                            })?;
                            var_ids.push(var_id);
                        }
                        Expr::IntLit(val) => {
                            // Constant integer - create a fixed variable
                            let const_var = self.model.int(*val as i32, *val as i32);
                            var_ids.push(const_var);
                        }
                        _ => {
                            return Err(FlatZincError::MapError {
                                message: format!("Unsupported array element: {:?}", elem),
                                line: None,
                                column: None,
                            });
                        }
                    }
                }
                Ok(var_ids)
            }
            Expr::Ident(name) => {
                // First check if it's an array variable
                if let Some(arr) = self.array_map.get(name) {
                    return Ok(arr.clone());
                }
                // Otherwise treat as single variable
                Ok(vec![self.var_map.get(name).copied().ok_or_else(|| {
                    FlatZincError::MapError {
                        message: format!("Unknown variable or array: {}", name),
                        line: None,
                        column: None,
                    }
                })?])
            }
            _ => Err(FlatZincError::MapError {
                message: "Expected array of variables".to_string(),
                line: None,
                column: None,
            }),
        }
    }
}
