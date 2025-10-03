//! Boolean constraint mappers
//!
//! Maps FlatZinc boolean constraints (bool_clause, array_bool_and, array_bool_or, bool2int, bool_le)
//! to Selen constraint model.

use crate::flatzinc::ast::*;
use crate::flatzinc::error::{FlatZincError, FlatZincResult};
use crate::flatzinc::mapper::MappingContext;
use crate::runtime_api::{VarIdExt, ModelExt};

impl<'a> MappingContext<'a> {
    /// Map bool_clause: (∨ pos[i]) ∨ (∨ ¬neg[i])
    pub(in crate::flatzinc::mapper) fn map_bool_clause(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "bool_clause requires 2 arguments (positive and negative literals)".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let pos_vars = self.extract_var_array(&constraint.args[0])?;
        let neg_vars = self.extract_var_array(&constraint.args[1])?;
        
        // Build clause: (∨ pos[i]) ∨ (∨ ¬neg[i])
        // For negated literals, create: (1 - var) which gives NOT
        let mut all_literals = pos_vars;
        
        for &var in &neg_vars {
            // Create (1 - var) for negation (since bool is 0/1)
            let one_minus_var = self.model.sub(crate::variables::Val::ValI(1), var);
            all_literals.push(one_minus_var);
        }
        
        if !all_literals.is_empty() {
            let clause_result = self.model.bool_or(&all_literals);
            // The clause must be true
            self.model.new(clause_result.eq(1));
        }
        
        Ok(())
    }
    
    /// Map array_bool_and: result = AND of all array elements
    pub(in crate::flatzinc::mapper) fn map_array_bool_and(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "array_bool_and requires 2 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let arr_vars = self.extract_var_array(&constraint.args[0])?;
        let result_var = self.get_var_or_const(&constraint.args[1])?;
        
        // result = AND of all elements: result ⇔ (x[0] ∧ x[1] ∧ ... ∧ x[n])
        if arr_vars.is_empty() {
            // Empty array: result = true
            self.model.new(result_var.eq(1));
        } else if arr_vars.len() == 1 {
            self.model.new(result_var.eq(arr_vars[0]));
        } else {
            // Use Model's bool_and for n-ary conjunction
            let and_result = self.model.bool_and(&arr_vars);
            self.model.new(result_var.eq(and_result));
        }
        Ok(())
    }
    
    /// Map array_bool_or: result = OR of all array elements
    pub(in crate::flatzinc::mapper) fn map_array_bool_or(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "array_bool_or requires 2 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let arr_vars = self.extract_var_array(&constraint.args[0])?;
        let result_var = self.get_var_or_const(&constraint.args[1])?;
        
        // result = OR of all elements: result ⇔ (x[0] ∨ x[1] ∨ ... ∨ x[n])
        if arr_vars.is_empty() {
            // Empty array: result = false
            self.model.new(result_var.eq(0));
        } else if arr_vars.len() == 1 {
            self.model.new(result_var.eq(arr_vars[0]));
        } else {
            // Use Model's bool_or for n-ary disjunction
            let or_result = self.model.bool_or(&arr_vars);
            self.model.new(result_var.eq(or_result));
        }
        Ok(())
    }
    
    /// Map bool2int: int_var = bool_var (bool is 0/1)
    pub(in crate::flatzinc::mapper) fn map_bool2int(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "bool2int requires 2 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let bool_var = self.get_var_or_const(&constraint.args[0])?;
        let int_var = self.get_var_or_const(&constraint.args[1])?;
        // bool2int: int_var = bool_var (bool is 0/1 in Selen)
        self.model.new(int_var.eq(bool_var));
        Ok(())
    }
    
    /// Map bool_le: x <= y for boolean variables
    /// FlatZinc signature: bool_le(x, y)
    pub(in crate::flatzinc::mapper) fn map_bool_le(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "bool_le requires 2 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var_or_const(&constraint.args[0])?;
        let y = self.get_var_or_const(&constraint.args[1])?;
        
        // For boolean variables: x <= y is equivalent to (not x) or y
        // Which is the same as x => y (implication)
        self.model.new(x.le(y));
        Ok(())
    }
    
    /// Map bool_eq_reif: r ⇔ (x = y) for boolean variables
    /// FlatZinc signature: bool_eq_reif(x, y, r)
    pub(in crate::flatzinc::mapper) fn map_bool_eq_reif(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "bool_eq_reif requires 3 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var_or_const(&constraint.args[0])?;
        let y = self.get_var_or_const(&constraint.args[1])?;
        let r = self.get_var_or_const(&constraint.args[2])?;
        
        // For booleans (0/1): r ⇔ (x = y)
        // Since booleans are represented as 0/1 integers in Selen, we can use int_eq_reif
        self.model.int_eq_reif(x, y, r);
        Ok(())
    }
    
    /// Map bool_eq: x = y for boolean variables
    /// FlatZinc signature: bool_eq(x, y)
    pub(in crate::flatzinc::mapper) fn map_bool_eq(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "bool_eq requires 2 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var_or_const(&constraint.args[0])?;
        let y = self.get_var_or_const(&constraint.args[1])?;
        
        // x = y for booleans
        self.model.new(x.eq(y));
        Ok(())
    }
    
    /// Map bool_le_reif: r ⇔ (x ≤ y) for boolean variables
    /// FlatZinc signature: bool_le_reif(x, y, r)
    pub(in crate::flatzinc::mapper) fn map_bool_le_reif(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "bool_le_reif requires 3 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var_or_const(&constraint.args[0])?;
        let y = self.get_var_or_const(&constraint.args[1])?;
        let r = self.get_var_or_const(&constraint.args[2])?;
        
        // For booleans (0/1): r ⇔ (x ≤ y)
        self.model.int_le_reif(x, y, r);
        Ok(())
    }
    
    /// Map bool_not: y = ¬x for boolean variables
    /// FlatZinc signature: bool_not(x, y)
    pub(in crate::flatzinc::mapper) fn map_bool_not(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "bool_not requires 2 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var_or_const(&constraint.args[0])?;
        let y = self.get_var_or_const(&constraint.args[1])?;
        
        // y = NOT x  →  y = 1 - x (for boolean 0/1)
        let not_x = self.model.sub(crate::variables::Val::ValI(1), x);
        self.model.new(y.eq(not_x));
        Ok(())
    }
    
    /// Map bool_xor: z = x XOR y for boolean variables
    /// FlatZinc signature: bool_xor(x, y, z)
    pub(in crate::flatzinc::mapper) fn map_bool_xor(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "bool_xor requires 3 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var_or_const(&constraint.args[0])?;
        let y = self.get_var_or_const(&constraint.args[1])?;
        let z = self.get_var_or_const(&constraint.args[2])?;
        
        // z = x XOR y
        // For booleans: x XOR y = (x + y) mod 2 = x + y - 2*(x*y)
        // Or equivalently: z ⇔ (x ≠ y)
        self.model.int_ne_reif(x, y, z);
        Ok(())
    }
}
