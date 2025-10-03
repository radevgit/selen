//! Comparison constraint mappers
//!
//! Maps FlatZinc comparison constraints (int_eq, int_ne, int_lt, int_le, int_gt, int_ge)
//! to Selen constraint model.

use crate::flatzinc::ast::*;
use crate::flatzinc::error::{FlatZincError, FlatZincResult};
use crate::flatzinc::mapper::MappingContext;
use crate::runtime_api::{VarIdExt, ModelExt};

impl<'a> MappingContext<'a> {
    /// Map int_eq constraint: x = y
    /// Supports variables, array access, and integer literals for both arguments
    pub(in crate::flatzinc::mapper) fn map_int_eq(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "int_eq requires 2 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var_or_const(&constraint.args[0])?;
        let y = self.get_var_or_const(&constraint.args[1])?;
        self.model.new(x.eq(y));
        
        Ok(())
    }
    
    /// Map int_ne constraint: x ≠ y
    /// Supports variables, array access, and integer literals for both arguments
    pub(in crate::flatzinc::mapper) fn map_int_ne(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "int_ne requires 2 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var_or_const(&constraint.args[0])?;
        let y = self.get_var_or_const(&constraint.args[1])?;
        self.model.new(x.ne(y));
        Ok(())
    }
    
    /// Map int_lt constraint: x < y
    /// Supports variables, array access, and integer literals for both arguments
    pub(in crate::flatzinc::mapper) fn map_int_lt(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "int_lt requires 2 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var_or_const(&constraint.args[0])?;
        let y = self.get_var_or_const(&constraint.args[1])?;
        self.model.new(x.lt(y));
        Ok(())
    }
    
    /// Map int_le constraint: x ≤ y
    /// Supports variables, array access, and integer literals for both arguments
    pub(in crate::flatzinc::mapper) fn map_int_le(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "int_le requires 2 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var_or_const(&constraint.args[0])?;
        let y = self.get_var_or_const(&constraint.args[1])?;
        self.model.new(x.le(y));
        Ok(())
    }
    
    /// Map int_gt constraint: x > y
    /// Supports variables, array access, and integer literals for both arguments
    pub(in crate::flatzinc::mapper) fn map_int_gt(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "int_gt requires 2 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var_or_const(&constraint.args[0])?;
        let y = self.get_var_or_const(&constraint.args[1])?;
        self.model.new(x.gt(y));
        Ok(())
    }
    
    /// Map int_ge constraint: x ≥ y
    /// Supports variables, array access, and integer literals for both arguments
    pub(in crate::flatzinc::mapper) fn map_int_ge(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "int_ge requires 2 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var_or_const(&constraint.args[0])?;
        let y = self.get_var_or_const(&constraint.args[1])?;
        self.model.new(x.ge(y));
        Ok(())
    }
}
