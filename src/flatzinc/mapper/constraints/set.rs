//! Set constraint mappers
//!
//! Maps FlatZinc set constraints to Selen constraint model.

use crate::flatzinc::ast::*;
use crate::flatzinc::error::{FlatZincError, FlatZincResult};
use crate::flatzinc::mapper::MappingContext;
use crate::runtime_api::{VarIdExt, ModelExt};

impl<'a> MappingContext<'a> {
    /// Map set_in_reif: b ⇔ (value ∈ set)
    /// FlatZinc signature: set_in_reif(value, set, b)
    /// 
    /// Where:
    /// - value: int variable or literal
    /// - set: set literal {1,2,3} or range 1..10
    /// - b: boolean variable indicating membership
    pub(in crate::flatzinc::mapper) fn map_set_in_reif(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "set_in_reif requires 3 arguments (value, set, bool)".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        // Get the value (can be variable or constant)
        let value = self.get_var_or_const(&constraint.args[0])?;
        
        // Get the boolean result variable
        let b = self.get_var_or_const(&constraint.args[2])?;
        
        // Parse the set (can be SetLit or Range)
        match &constraint.args[1] {
            Expr::Range(min_expr, max_expr) => {
                // Handle range like 1..10
                let min = self.extract_int(min_expr)?;
                let max = self.extract_int(max_expr)?;
                
                // b ⇔ (value ∈ [min, max])
                // This is equivalent to: b ⇔ (value >= min AND value <= max)
                // We can decompose into: (value >= min) AND (value <= max) ⇔ b
                
                // Create: b1 ⇔ (value >= min)
                let min_var = self.model.int(min, min);
                let b1 = self.model.bool();
                self.model.int_ge_reif(value, min_var, b1);
                
                // Create: b2 ⇔ (value <= max)
                let max_var = self.model.int(max, max);
                let b2 = self.model.bool();
                self.model.int_le_reif(value, max_var, b2);
                
                // Create: b ⇔ (b1 AND b2)
                let and_result = self.model.bool_and(&[b1, b2]);
                self.model.new(b.eq(and_result));
                
                Ok(())
            }
            Expr::SetLit(elements) => {
                // Handle explicit set like {1, 2, 3}
                // b ⇔ (value = elements[0] OR value = elements[1] OR ...)
                
                if elements.is_empty() {
                    // Empty set: b must be false
                    self.model.new(b.eq(0));
                    return Ok(());
                }
                
                // Create b_i ⇔ (value = element[i]) for each element
                let mut membership_vars = Vec::new();
                for elem in elements {
                    let elem_val = self.extract_int(elem)?;
                    let elem_var = self.model.int(elem_val, elem_val);
                    let bi = self.model.bool();
                    self.model.int_eq_reif(value, elem_var, bi);
                    membership_vars.push(bi);
                }
                
                // b ⇔ OR of all membership variables
                let or_result = self.model.bool_or(&membership_vars);
                self.model.new(b.eq(or_result));
                
                Ok(())
            }
            _ => Err(FlatZincError::MapError {
                message: format!("Unsupported set type in set_in_reif: {:?}", constraint.args[1]),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            }),
        }
    }
    
    /// Map set_in: value ∈ set (non-reified version)
    /// FlatZinc signature: set_in(value, set)
    pub(in crate::flatzinc::mapper) fn map_set_in(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "set_in requires 2 arguments (value, set)".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        // Get the value
        let value = self.get_var_or_const(&constraint.args[0])?;
        
        // Parse the set
        match &constraint.args[1] {
            Expr::Range(min_expr, max_expr) => {
                // Handle range like 1..10
                let min = self.extract_int(min_expr)?;
                let max = self.extract_int(max_expr)?;
                
                // value ∈ [min, max] => (value >= min) AND (value <= max)
                self.model.new(value.ge(min));
                self.model.new(value.le(max));
                
                Ok(())
            }
            Expr::SetLit(elements) => {
                // Handle explicit set like {1, 2, 3}
                // value ∈ {e1, e2, ...} => (value = e1) OR (value = e2) OR ...
                
                if elements.is_empty() {
                    // Empty set: contradiction
                    return Err(FlatZincError::MapError {
                        message: "set_in with empty set is unsatisfiable".to_string(),
                        line: Some(constraint.location.line),
                        column: Some(constraint.location.column),
                    });
                }
                
                // Create (value = element[i]) for each element
                let mut membership_vars = Vec::new();
                for elem in elements {
                    let elem_val = self.extract_int(elem)?;
                    let elem_var = self.model.int(elem_val, elem_val);
                    let bi = self.model.bool();
                    self.model.int_eq_reif(value, elem_var, bi);
                    membership_vars.push(bi);
                }
                
                // At least one must be true
                let or_result = self.model.bool_or(&membership_vars);
                self.model.new(or_result.eq(1));
                
                Ok(())
            }
            _ => Err(FlatZincError::MapError {
                message: format!("Unsupported set type in set_in: {:?}", constraint.args[1]),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            }),
        }
    }
}
