//! Reified constraint mappers
//!
//! Maps FlatZinc reified constraints (*_reif) to Selen constraint model.
//! Reified constraints have the form: b ⇔ (constraint)

use crate::flatzinc::ast::*;
use crate::flatzinc::error::{FlatZincError, FlatZincResult};
use crate::flatzinc::mapper::MappingContext;
use crate::runtime_api::ModelExt;

impl<'a> MappingContext<'a> {
    /// Map int_eq_reif: b ⇔ (x = y)
    pub(in crate::flatzinc::mapper) fn map_int_eq_reif(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "int_eq_reif requires 3 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var_or_const(&constraint.args[0])?;
        let y = self.get_var_or_const(&constraint.args[1])?;
        let b = self.get_var_or_const(&constraint.args[2])?;
        self.model.int_eq_reif(x, y, b);
        Ok(())
    }
    
    pub(in crate::flatzinc::mapper) fn map_int_ne_reif(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "int_ne_reif requires 3 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var_or_const(&constraint.args[0])?;
        let y = self.get_var_or_const(&constraint.args[1])?;
        let b = self.get_var_or_const(&constraint.args[2])?;
        self.model.int_ne_reif(x, y, b);
        Ok(())
    }
    
    pub(in crate::flatzinc::mapper) fn map_int_lt_reif(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "int_lt_reif requires 3 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var_or_const(&constraint.args[0])?;
        let y = self.get_var_or_const(&constraint.args[1])?;
        let b = self.get_var_or_const(&constraint.args[2])?;
        self.model.int_lt_reif(x, y, b);
        Ok(())
    }
    
    pub(in crate::flatzinc::mapper) fn map_int_le_reif(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "int_le_reif requires 3 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var_or_const(&constraint.args[0])?;
        let y = self.get_var_or_const(&constraint.args[1])?;
        let b = self.get_var_or_const(&constraint.args[2])?;
        self.model.int_le_reif(x, y, b);
        Ok(())
    }
    
    pub(in crate::flatzinc::mapper) fn map_int_gt_reif(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "int_gt_reif requires 3 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var_or_const(&constraint.args[0])?;
        let y = self.get_var_or_const(&constraint.args[1])?;
        let b = self.get_var_or_const(&constraint.args[2])?;
        self.model.int_gt_reif(x, y, b);
        Ok(())
    }
    
    pub(in crate::flatzinc::mapper) fn map_int_ge_reif(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "int_ge_reif requires 3 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var_or_const(&constraint.args[0])?;
        let y = self.get_var_or_const(&constraint.args[1])?;
        let b = self.get_var_or_const(&constraint.args[2])?;
        self.model.int_ge_reif(x, y, b);
        Ok(())
    }
}
