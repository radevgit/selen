//! Arithmetic constraint mappers
//!
//! Maps FlatZinc arithmetic constraints (int_plus, int_minus, int_times, int_div, int_mod, int_abs, int_min, int_max)
//! to Selen constraint model.

use crate::flatzinc::ast::*;
use crate::flatzinc::error::{FlatZincError, FlatZincResult};
use crate::flatzinc::mapper::MappingContext;
use crate::runtime_api::{VarIdExt, ModelExt};

impl<'a> MappingContext<'a> {
    /// Map int_abs: result = |x|
    /// FlatZinc signature: int_abs(x, result)
    pub(in crate::flatzinc::mapper) fn map_int_abs(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "int_abs requires 2 arguments (x, result)".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var(&constraint.args[0])?;
        let result = self.get_var(&constraint.args[1])?;
        
        // Use Selen's abs constraint
        let abs_x = self.model.abs(x);
        
        // Constrain result to equal abs(x)
        self.model.new(abs_x.eq(result));
        Ok(())
    }

    /// Map int_plus: z = x + y
    /// FlatZinc signature: int_plus(x, y, z)
    pub(in crate::flatzinc::mapper) fn map_int_plus(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "int_plus requires 3 arguments (x, y, z)".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var(&constraint.args[0])?;
        let y = self.get_var(&constraint.args[1])?;
        let z = self.get_var(&constraint.args[2])?;
        
        // Use Selen's add constraint: z = x + y
        let sum = self.model.add(x, y);
        self.model.new(sum.eq(z));
        Ok(())
    }

    /// Map int_minus: z = x - y
    /// FlatZinc signature: int_minus(x, y, z)
    pub(in crate::flatzinc::mapper) fn map_int_minus(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "int_minus requires 3 arguments (x, y, z)".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var(&constraint.args[0])?;
        let y = self.get_var(&constraint.args[1])?;
        let z = self.get_var(&constraint.args[2])?;
        
        // Use Selen's sub constraint: z = x - y
        let diff = self.model.sub(x, y);
        self.model.new(diff.eq(z));
        Ok(())
    }

    /// Map int_times: z = x * y
    /// FlatZinc signature: int_times(x, y, z)
    pub(in crate::flatzinc::mapper) fn map_int_times(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "int_times requires 3 arguments (x, y, z)".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var(&constraint.args[0])?;
        let y = self.get_var(&constraint.args[1])?;
        let z = self.get_var(&constraint.args[2])?;
        
        // Use Selen's mul constraint: z = x * y
        let product = self.model.mul(x, y);
        self.model.new(product.eq(z));
        Ok(())
    }

    /// Map int_div: z = x / y
    /// FlatZinc signature: int_div(x, y, z)
    pub(in crate::flatzinc::mapper) fn map_int_div(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "int_div requires 3 arguments (x, y, z)".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var(&constraint.args[0])?;
        let y = self.get_var(&constraint.args[1])?;
        let z = self.get_var(&constraint.args[2])?;
        
        // Use Selen's div constraint: z = x / y
        let quotient = self.model.div(x, y);
        self.model.new(quotient.eq(z));
        Ok(())
    }

    /// Map int_mod: z = x mod y
    /// FlatZinc signature: int_mod(x, y, z)
    pub(in crate::flatzinc::mapper) fn map_int_mod(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "int_mod requires 3 arguments (x, y, z)".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var(&constraint.args[0])?;
        let y = self.get_var(&constraint.args[1])?;
        let z = self.get_var(&constraint.args[2])?;
        
        // Use Selen's mod constraint: z = x mod y
        let remainder = self.model.modulo(x, y);
        self.model.new(remainder.eq(z));
        Ok(())
    }

    /// Map int_max: z = max(x, y)
    /// FlatZinc signature: int_max(x, y, z)
    pub(in crate::flatzinc::mapper) fn map_int_max(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "int_max requires 3 arguments (x, y, z)".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var(&constraint.args[0])?;
        let y = self.get_var(&constraint.args[1])?;
        let z = self.get_var(&constraint.args[2])?;
        
        // Use Selen's max constraint: z = max([x, y])
        let max_xy = self.model.max(&[x, y])
            .map_err(|e| FlatZincError::MapError {
                message: format!("Error creating max constraint: {}", e),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            })?;
        self.model.new(max_xy.eq(z));
        Ok(())
    }

    /// Map int_min: z = min(x, y)
    /// FlatZinc signature: int_min(x, y, z)
    pub(in crate::flatzinc::mapper) fn map_int_min(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "int_min requires 3 arguments (x, y, z)".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var(&constraint.args[0])?;
        let y = self.get_var(&constraint.args[1])?;
        let z = self.get_var(&constraint.args[2])?;
        
        // Use Selen's min constraint: z = min([x, y])
        let min_xy = self.model.min(&[x, y])
            .map_err(|e| FlatZincError::MapError {
                message: format!("Error creating min constraint: {}", e),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            })?;
        self.model.new(min_xy.eq(z));
        Ok(())
    }
}
