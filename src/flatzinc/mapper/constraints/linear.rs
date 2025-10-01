//! Linear constraint mappers
//!
//! Maps FlatZinc linear constraints (int_lin_eq, int_lin_le, int_lin_ne)
//! to Selen constraint model.

use crate::flatzinc::ast::*;
use crate::flatzinc::error::{FlatZincError, FlatZincResult};
use crate::flatzinc::mapper::MappingContext;
use crate::runtime_api::{VarIdExt, ModelExt};
use crate::variables::VarId;

impl<'a> MappingContext<'a> {
    /// Map int_lin_eq: Σ(coeffs[i] * vars[i]) = constant
    pub(in crate::flatzinc::mapper) fn map_int_lin_eq(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "int_lin_eq requires 3 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let coeffs = self.extract_int_array(&constraint.args[0])?;
        let var_ids = self.extract_var_array(&constraint.args[1])?;
        let constant = self.extract_int(&constraint.args[2])?;
        
        // Create sum using Model's API
        let scaled_vars: Vec<VarId> = coeffs
            .iter()
            .zip(var_ids.iter())
            .map(|(&coeff, &var)| self.model.mul(var, crate::variables::Val::ValI(coeff)))
            .collect();
        
        let sum_var = self.model.sum(&scaled_vars);
        self.model.new(sum_var.eq(constant));
        Ok(())
    }
    
    /// Map int_lin_le: Σ(coeffs[i] * vars[i]) ≤ constant
    pub(in crate::flatzinc::mapper) fn map_int_lin_le(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "int_lin_le requires 3 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let coeffs = self.extract_int_array(&constraint.args[0])?;
        let var_ids = self.extract_var_array(&constraint.args[1])?;
        let constant = self.extract_int(&constraint.args[2])?;
        
        let scaled_vars: Vec<VarId> = coeffs
            .iter()
            .zip(var_ids.iter())
            .map(|(&coeff, &var)| self.model.mul(var, crate::variables::Val::ValI(coeff)))
            .collect();
        
        let sum_var = self.model.sum(&scaled_vars);
        self.model.new(sum_var.le(constant));
        Ok(())
    }
    
    /// Map int_lin_ne: Σ(coeffs[i] * vars[i]) ≠ constant
    pub(in crate::flatzinc::mapper) fn map_int_lin_ne(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "int_lin_ne requires 3 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let coeffs = self.extract_int_array(&constraint.args[0])?;
        let var_ids = self.extract_var_array(&constraint.args[1])?;
        let constant = self.extract_int(&constraint.args[2])?;
        
        let scaled_vars: Vec<VarId> = coeffs
            .iter()
            .zip(var_ids.iter())
            .map(|(&coeff, &var)| self.model.mul(var, crate::variables::Val::ValI(coeff)))
            .collect();
        
        let sum_var = self.model.sum(&scaled_vars);
        
        // Use runtime API to post not-equals constraint: sum ≠ constant
        self.model.c(sum_var).ne(constant);
        Ok(())
    }
}
