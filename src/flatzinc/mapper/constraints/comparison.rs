//! Comparison constraint mappers
//!
//! Maps FlatZinc comparison constraints (int_eq, int_ne, int_lt, int_le, int_gt, int_ge)
//! to Selen constraint model.

use crate::flatzinc::ast::*;
use crate::flatzinc::error::{FlatZincError, FlatZincResult};
use crate::flatzinc::mapper::MappingContext;
use crate::runtime_api::{VarIdExt, ModelExt};

impl<'a> MappingContext<'a> {
    /// Map int_eq constraint: x = y or x = constant
    pub(in crate::flatzinc::mapper) fn map_int_eq(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "int_eq requires 2 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        // Handle both: int_eq(var, const) and int_eq(const, var)
        match (&constraint.args[0], &constraint.args[1]) {
            // var = var
            (Expr::Ident(_) | Expr::ArrayAccess { .. }, Expr::Ident(_) | Expr::ArrayAccess { .. }) => {
                let x = self.get_var(&constraint.args[0])?;
                let y = self.get_var(&constraint.args[1])?;
                self.model.new(x.eq(y));
            }
            // var = const
            (Expr::Ident(_) | Expr::ArrayAccess { .. }, Expr::IntLit(val)) => {
                let x = self.get_var(&constraint.args[0])?;
                self.model.new(x.eq(*val as i32));
            }
            // const = var (swap to var = const)
            (Expr::IntLit(val), Expr::Ident(_) | Expr::ArrayAccess { .. }) => {
                let y = self.get_var(&constraint.args[1])?;
                self.model.new(y.eq(*val as i32));
            }
            _ => {
                return Err(FlatZincError::MapError {
                    message: "Unsupported argument types for int_eq".to_string(),
                    line: Some(constraint.location.line),
                    column: Some(constraint.location.column),
                });
            }
        }
        
        Ok(())
    }
    
    pub(in crate::flatzinc::mapper) fn map_int_ne(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "int_ne requires 2 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        match (&constraint.args[0], &constraint.args[1]) {
            (Expr::Ident(_) | Expr::ArrayAccess { .. }, Expr::Ident(_) | Expr::ArrayAccess { .. }) => {
                let x = self.get_var(&constraint.args[0])?;
                let y = self.get_var(&constraint.args[1])?;
                self.model.new(x.ne(y));
            }
            (Expr::Ident(_) | Expr::ArrayAccess { .. }, Expr::IntLit(val)) => {
                let x = self.get_var(&constraint.args[0])?;
                self.model.new(x.ne(*val as i32));
            }
            (Expr::IntLit(val), Expr::Ident(_) | Expr::ArrayAccess { .. }) => {
                let y = self.get_var(&constraint.args[1])?;
                self.model.new(y.ne(*val as i32));
            }
            _ => {
                return Err(FlatZincError::MapError {
                    message: "Unsupported argument types for int_ne".to_string(),
                    line: Some(constraint.location.line),
                    column: Some(constraint.location.column),
                });
            }
        }
        Ok(())
    }
    
    pub(in crate::flatzinc::mapper) fn map_int_lt(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "int_lt requires 2 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        match (&constraint.args[0], &constraint.args[1]) {
            (Expr::Ident(_) | Expr::ArrayAccess { .. }, Expr::Ident(_) | Expr::ArrayAccess { .. }) => {
                let x = self.get_var(&constraint.args[0])?;
                let y = self.get_var(&constraint.args[1])?;
                self.model.new(x.lt(y));
            }
            (Expr::Ident(_) | Expr::ArrayAccess { .. }, Expr::IntLit(val)) => {
                let x = self.get_var(&constraint.args[0])?;
                self.model.new(x.lt(*val as i32));
            }
            (Expr::IntLit(val), Expr::Ident(_) | Expr::ArrayAccess { .. }) => {
                let y = self.get_var(&constraint.args[1])?;
                self.model.new(y.gt(*val as i32)); // const < var => var > const
            }
            _ => {
                return Err(FlatZincError::MapError {
                    message: "Unsupported argument types for int_lt".to_string(),
                    line: Some(constraint.location.line),
                    column: Some(constraint.location.column),
                });
            }
        }
        Ok(())
    }
    
    pub(in crate::flatzinc::mapper) fn map_int_le(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "int_le requires 2 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        match (&constraint.args[0], &constraint.args[1]) {
            (Expr::Ident(_) | Expr::ArrayAccess { .. }, Expr::Ident(_) | Expr::ArrayAccess { .. }) => {
                let x = self.get_var(&constraint.args[0])?;
                let y = self.get_var(&constraint.args[1])?;
                self.model.new(x.le(y));
            }
            (Expr::Ident(_) | Expr::ArrayAccess { .. }, Expr::IntLit(val)) => {
                let x = self.get_var(&constraint.args[0])?;
                self.model.new(x.le(*val as i32));
            }
            (Expr::IntLit(val), Expr::Ident(_) | Expr::ArrayAccess { .. }) => {
                let y = self.get_var(&constraint.args[1])?;
                self.model.new(y.ge(*val as i32)); // const <= var => var >= const
            }
            _ => {
                return Err(FlatZincError::MapError {
                    message: "Unsupported argument types for int_le".to_string(),
                    line: Some(constraint.location.line),
                    column: Some(constraint.location.column),
                });
            }
        }
        Ok(())
    }
    
    pub(in crate::flatzinc::mapper) fn map_int_gt(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "int_gt requires 2 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        match (&constraint.args[0], &constraint.args[1]) {
            (Expr::Ident(_) | Expr::ArrayAccess { .. }, Expr::Ident(_) | Expr::ArrayAccess { .. }) => {
                let x = self.get_var(&constraint.args[0])?;
                let y = self.get_var(&constraint.args[1])?;
                self.model.new(x.gt(y));
            }
            (Expr::Ident(_) | Expr::ArrayAccess { .. }, Expr::IntLit(val)) => {
                let x = self.get_var(&constraint.args[0])?;
                self.model.new(x.gt(*val as i32));
            }
            (Expr::IntLit(val), Expr::Ident(_) | Expr::ArrayAccess { .. }) => {
                let y = self.get_var(&constraint.args[1])?;
                self.model.new(y.lt(*val as i32)); // const > var => var < const
            }
            _ => {
                return Err(FlatZincError::MapError {
                    message: "Unsupported argument types for int_gt".to_string(),
                    line: Some(constraint.location.line),
                    column: Some(constraint.location.column),
                });
            }
        }
        Ok(())
    }
    
    pub(in crate::flatzinc::mapper) fn map_int_ge(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "int_ge requires 2 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        match (&constraint.args[0], &constraint.args[1]) {
            (Expr::Ident(_) | Expr::ArrayAccess { .. }, Expr::Ident(_) | Expr::ArrayAccess { .. }) => {
                let x = self.get_var(&constraint.args[0])?;
                let y = self.get_var(&constraint.args[1])?;
                self.model.new(x.ge(y));
            }
            (Expr::Ident(_) | Expr::ArrayAccess { .. }, Expr::IntLit(val)) => {
                let x = self.get_var(&constraint.args[0])?;
                self.model.new(x.ge(*val as i32));
            }
            (Expr::IntLit(val), Expr::Ident(_) | Expr::ArrayAccess { .. }) => {
                let y = self.get_var(&constraint.args[1])?;
                self.model.new(y.le(*val as i32)); // const >= var => var <= const
            }
            _ => {
                return Err(FlatZincError::MapError {
                    message: "Unsupported argument types for int_ge".to_string(),
                    line: Some(constraint.location.line),
                    column: Some(constraint.location.column),
                });
            }
        }
        Ok(())
    }
}
