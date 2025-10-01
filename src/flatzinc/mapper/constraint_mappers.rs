//! Constraint mapping functions
//!
//! Maps individual FlatZinc constraint predicates to Selen constraints.
//!
//! ## Organization
//! 
//! This file contains constraint mappers organized by category:
//! 
//! 1. **Comparison Constraints** (lines ~15-220)
//!    - int_eq, int_ne, int_lt, int_le, int_gt, int_ge
//! 
//! 2. **Linear Constraints** (lines ~220-300)
//!    - int_lin_eq, int_lin_le, int_lin_ne
//! 
//! 3. **Global Constraints** (lines ~300-315)
//!    - all_different
//! 
//! 4. **Reified Constraints** (lines ~315-545)
//!    - int_eq_reif, int_ne_reif, int_lt_reif, int_le_reif, int_gt_reif, int_ge_reif
//! 
//! 5. **Boolean Constraints** (lines ~545-690)
//!    - bool_clause, array_bool_and, array_bool_or, bool2int, bool_le
//! 
//! 6. **Array Constraints** (lines ~575-645)
//!    - array_int_minimum, array_int_maximum
//! 
//! 7. **Counting Constraints** (lines ~690-720)
//!    - count_eq
//! 
//! 8. **Element Constraints** (lines ~720-900)
//!    - array_var_int_element, array_int_element, array_var_bool_element, array_bool_element
//! 
//! 9. **Arithmetic Constraints** (lines ~900-1100)
//!    - int_abs, int_plus, int_minus, int_times, int_div, int_mod, int_max, int_min
//!
//! TODO: Consider splitting into separate modules when file exceeds ~1500 lines

use crate::flatzinc::ast::*;
use crate::flatzinc::error::{FlatZincError, FlatZincResult};
use crate::flatzinc::mapper::MappingContext;
use crate::runtime_api::{VarIdExt, ModelExt};
use crate::variables::VarId;

impl<'a> MappingContext<'a> {
    // ═════════════════════════════════════════════════════════════════════════
    // 📊 COMPARISON CONSTRAINTS
    // ═════════════════════════════════════════════════════════════════════════
    
    /// Map int_eq constraint: x = y or x = constant
    pub(super) fn map_int_eq(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
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
    
    pub(super) fn map_int_ne(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
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
    
    pub(super) fn map_int_lt(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
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
    
    pub(super) fn map_int_le(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
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
    
    pub(super) fn map_int_gt(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
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
    
    pub(super) fn map_int_ge(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
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
    
    /// Map int_lin_eq: Σ(coeffs[i] * vars[i]) = constant
    pub(super) fn map_int_lin_eq(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
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
    pub(super) fn map_int_lin_le(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
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
    pub(super) fn map_int_lin_ne(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
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
    
    // ═════════════════════════════════════════════════════════════════════════
    // 🌐 GLOBAL CONSTRAINTS
    // ═════════════════════════════════════════════════════════════════════════
    
    /// Map all_different constraint
    pub(super) fn map_all_different(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 1 {
            return Err(FlatZincError::MapError {
                message: "all_different requires 1 argument (array of variables)".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let var_ids = self.extract_var_array(&constraint.args[0])?;
        self.model.alldiff(&var_ids);
        Ok(())
    }
    
    // ═════════════════════════════════════════════════════════════════════════
    // 🔄 REIFIED CONSTRAINTS
    // ═════════════════════════════════════════════════════════════════════════
    
    /// Map int_eq_reif: b ⇔ (x = y)
    pub(super) fn map_int_eq_reif(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "int_eq_reif requires 3 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let b = self.get_var(&constraint.args[2])?;
        
        match (&constraint.args[0], &constraint.args[1]) {
            (Expr::Ident(_) | Expr::ArrayAccess { .. }, Expr::Ident(_) | Expr::ArrayAccess { .. }) => {
                let x = self.get_var(&constraint.args[0])?;
                let y = self.get_var(&constraint.args[1])?;
                self.model.int_eq_reif(x, y, b);
            }
            (Expr::Ident(_) | Expr::ArrayAccess { .. }, Expr::IntLit(val)) => {
                let x = self.get_var(&constraint.args[0])?;
                let const_var = self.model.int(*val as i32, *val as i32);
                self.model.int_eq_reif(x, const_var, b);
            }
            (Expr::IntLit(val), Expr::Ident(_) | Expr::ArrayAccess { .. }) => {
                let y = self.get_var(&constraint.args[1])?;
                let const_var = self.model.int(*val as i32, *val as i32);
                self.model.int_eq_reif(const_var, y, b);
            }
            _ => {
                return Err(FlatZincError::MapError {
                    message: "Unsupported argument types for int_eq_reif".to_string(),
                    line: Some(constraint.location.line),
                    column: Some(constraint.location.column),
                });
            }
        }
        Ok(())
    }
    
    pub(super) fn map_int_ne_reif(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "int_ne_reif requires 3 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let b = self.get_var(&constraint.args[2])?;
        
        match (&constraint.args[0], &constraint.args[1]) {
            (Expr::Ident(_) | Expr::ArrayAccess { .. }, Expr::Ident(_) | Expr::ArrayAccess { .. }) => {
                let x = self.get_var(&constraint.args[0])?;
                let y = self.get_var(&constraint.args[1])?;
                self.model.int_ne_reif(x, y, b);
            }
            (Expr::Ident(_) | Expr::ArrayAccess { .. }, Expr::IntLit(val)) => {
                let x = self.get_var(&constraint.args[0])?;
                let const_var = self.model.int(*val as i32, *val as i32);
                self.model.int_ne_reif(x, const_var, b);
            }
            (Expr::IntLit(val), Expr::Ident(_) | Expr::ArrayAccess { .. }) => {
                let y = self.get_var(&constraint.args[1])?;
                let const_var = self.model.int(*val as i32, *val as i32);
                self.model.int_ne_reif(const_var, y, b);
            }
            _ => {
                return Err(FlatZincError::MapError {
                    message: "Unsupported argument types for int_ne_reif".to_string(),
                    line: Some(constraint.location.line),
                    column: Some(constraint.location.column),
                });
            }
        }
        Ok(())
    }
    
    pub(super) fn map_int_lt_reif(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "int_lt_reif requires 3 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let b = self.get_var(&constraint.args[2])?;
        
        match (&constraint.args[0], &constraint.args[1]) {
            (Expr::Ident(_) | Expr::ArrayAccess { .. }, Expr::Ident(_) | Expr::ArrayAccess { .. }) => {
                let x = self.get_var(&constraint.args[0])?;
                let y = self.get_var(&constraint.args[1])?;
                self.model.int_lt_reif(x, y, b);
            }
            (Expr::Ident(_) | Expr::ArrayAccess { .. }, Expr::IntLit(val)) => {
                let x = self.get_var(&constraint.args[0])?;
                let const_var = self.model.int(*val as i32, *val as i32);
                self.model.int_lt_reif(x, const_var, b);
            }
            (Expr::IntLit(val), Expr::Ident(_) | Expr::ArrayAccess { .. }) => {
                let y = self.get_var(&constraint.args[1])?;
                let const_var = self.model.int(*val as i32, *val as i32);
                self.model.int_lt_reif(const_var, y, b);
            }
            _ => {
                return Err(FlatZincError::MapError {
                    message: "Unsupported argument types for int_lt_reif".to_string(),
                    line: Some(constraint.location.line),
                    column: Some(constraint.location.column),
                });
            }
        }
        Ok(())
    }
    
    pub(super) fn map_int_le_reif(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "int_le_reif requires 3 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let b = self.get_var(&constraint.args[2])?;
        
        match (&constraint.args[0], &constraint.args[1]) {
            (Expr::Ident(_) | Expr::ArrayAccess { .. }, Expr::Ident(_) | Expr::ArrayAccess { .. }) => {
                let x = self.get_var(&constraint.args[0])?;
                let y = self.get_var(&constraint.args[1])?;
                self.model.int_le_reif(x, y, b);
            }
            (Expr::Ident(_) | Expr::ArrayAccess { .. }, Expr::IntLit(val)) => {
                let x = self.get_var(&constraint.args[0])?;
                let const_var = self.model.int(*val as i32, *val as i32);
                self.model.int_le_reif(x, const_var, b);
            }
            (Expr::IntLit(val), Expr::Ident(_) | Expr::ArrayAccess { .. }) => {
                let y = self.get_var(&constraint.args[1])?;
                let const_var = self.model.int(*val as i32, *val as i32);
                self.model.int_le_reif(const_var, y, b);
            }
            _ => {
                return Err(FlatZincError::MapError {
                    message: "Unsupported argument types for int_le_reif".to_string(),
                    line: Some(constraint.location.line),
                    column: Some(constraint.location.column),
                });
            }
        }
        Ok(())
    }
    
    pub(super) fn map_int_gt_reif(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "int_gt_reif requires 3 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let b = self.get_var(&constraint.args[2])?;
        
        match (&constraint.args[0], &constraint.args[1]) {
            (Expr::Ident(_) | Expr::ArrayAccess { .. }, Expr::Ident(_) | Expr::ArrayAccess { .. }) => {
                let x = self.get_var(&constraint.args[0])?;
                let y = self.get_var(&constraint.args[1])?;
                self.model.int_gt_reif(x, y, b);
            }
            (Expr::Ident(_) | Expr::ArrayAccess { .. }, Expr::IntLit(val)) => {
                let x = self.get_var(&constraint.args[0])?;
                let const_var = self.model.int(*val as i32, *val as i32);
                self.model.int_gt_reif(x, const_var, b);
            }
            (Expr::IntLit(val), Expr::Ident(_) | Expr::ArrayAccess { .. }) => {
                let y = self.get_var(&constraint.args[1])?;
                let const_var = self.model.int(*val as i32, *val as i32);
                self.model.int_gt_reif(const_var, y, b);
            }
            _ => {
                return Err(FlatZincError::MapError {
                    message: "Unsupported argument types for int_gt_reif".to_string(),
                    line: Some(constraint.location.line),
                    column: Some(constraint.location.column),
                });
            }
        }
        Ok(())
    }
    
    pub(super) fn map_int_ge_reif(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "int_ge_reif requires 3 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let b = self.get_var(&constraint.args[2])?;
        
        match (&constraint.args[0], &constraint.args[1]) {
            (Expr::Ident(_) | Expr::ArrayAccess { .. }, Expr::Ident(_) | Expr::ArrayAccess { .. }) => {
                let x = self.get_var(&constraint.args[0])?;
                let y = self.get_var(&constraint.args[1])?;
                self.model.int_ge_reif(x, y, b);
            }
            (Expr::Ident(_) | Expr::ArrayAccess { .. }, Expr::IntLit(val)) => {
                let x = self.get_var(&constraint.args[0])?;
                let const_var = self.model.int(*val as i32, *val as i32);
                self.model.int_ge_reif(x, const_var, b);
            }
            (Expr::IntLit(val), Expr::Ident(_) | Expr::ArrayAccess { .. }) => {
                let y = self.get_var(&constraint.args[1])?;
                let const_var = self.model.int(*val as i32, *val as i32);
                self.model.int_ge_reif(const_var, y, b);
            }
            _ => {
                return Err(FlatZincError::MapError {
                    message: "Unsupported argument types for int_ge_reif".to_string(),
                    line: Some(constraint.location.line),
                    column: Some(constraint.location.column),
                });
            }
        }
        Ok(())
    }
    
    /// Map bool_clause: (∨ pos[i]) ∨ (∨ ¬neg[i])
    pub(super) fn map_bool_clause(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
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
    
    /// Map array_int_minimum: min = minimum(array)
    pub(super) fn map_array_int_minimum(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "array_int_minimum requires 2 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let min_var = self.get_var(&constraint.args[0])?;
        let arr_vars = self.extract_var_array(&constraint.args[1])?;
        let min_result = self.model.min(&arr_vars).map_err(|e| FlatZincError::MapError {
            message: format!("Failed to create min: {}", e),
            line: Some(constraint.location.line),
            column: Some(constraint.location.column),
        })?;
        self.model.new(min_var.eq(min_result));
        Ok(())
    }
    
    /// Map array_int_maximum: max = maximum(array)
    pub(super) fn map_array_int_maximum(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "array_int_maximum requires 2 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let max_var = self.get_var(&constraint.args[0])?;
        let arr_vars = self.extract_var_array(&constraint.args[1])?;
        let max_result = self.model.max(&arr_vars).map_err(|e| FlatZincError::MapError {
            message: format!("Failed to create max: {}", e),
            line: Some(constraint.location.line),
            column: Some(constraint.location.column),
        })?;
        self.model.new(max_var.eq(max_result));
        Ok(())
    }
    
    /// Map array_bool_and: result = AND of all array elements
    pub(super) fn map_array_bool_and(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "array_bool_and requires 2 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let arr_vars = self.extract_var_array(&constraint.args[0])?;
        let result_var = self.get_var(&constraint.args[1])?;
        
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
    pub(super) fn map_array_bool_or(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "array_bool_or requires 2 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let arr_vars = self.extract_var_array(&constraint.args[0])?;
        let result_var = self.get_var(&constraint.args[1])?;
        
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
    pub(super) fn map_bool2int(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "bool2int requires 2 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let bool_var = self.get_var(&constraint.args[0])?;
        let int_var = self.get_var(&constraint.args[1])?;
        // bool2int: int_var = bool_var (bool is 0/1 in Selen)
        self.model.new(int_var.eq(bool_var));
        Ok(())
    }
    
    /// Map count_eq: count = |{i : array[i] = value}|
    pub(super) fn map_count_eq(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "count_eq requires 3 arguments (array, value, count)".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let arr_vars = self.extract_var_array(&constraint.args[0])?;
        let value = self.extract_int(&constraint.args[1])?;
        let count_var = self.get_var(&constraint.args[2])?;
        
        // Use Selen's count constraint
        self.model.count(&arr_vars, value, count_var);
        Ok(())
    }
    
    /// Map array_var_int_element: array[index] = value
    /// FlatZinc signature: array_var_int_element(index, array, value)
    /// Note: FlatZinc uses 1-based indexing, Selen uses 0-based
    pub(super) fn map_array_var_int_element(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "array_var_int_element requires 3 arguments (index, array, value)".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        // Get index variable (1-based in FlatZinc)
        let index_1based = self.get_var(&constraint.args[0])?;
        
        // Convert to 0-based index for Selen
        // Create: index_0based = index_1based - 1
        let index_0based = self.model.sub(index_1based, crate::variables::Val::ValI(1));
        
        // Get array
        let array = self.extract_var_array(&constraint.args[1])?;
        
        // Get value (can be variable or constant)
        let value = match &constraint.args[2] {
            Expr::Ident(_) => self.get_var(&constraint.args[2])?,
            Expr::IntLit(val) => {
                // Convert constant to fixed variable
                self.model.int(*val as i32, *val as i32)
            }
            _ => {
                return Err(FlatZincError::MapError {
                    message: "Unsupported value type in array_var_int_element".to_string(),
                    line: Some(constraint.location.line),
                    column: Some(constraint.location.column),
                });
            }
        };
        
        // Apply element constraint: array[index_0based] = value
        self.model.elem(&array, index_0based, value);
        Ok(())
    }
    
    /// Map array_int_element: array[index] = value (with constant array)
    /// FlatZinc signature: array_int_element(index, array, value)
    /// Note: FlatZinc uses 1-based indexing, Selen uses 0-based
    pub(super) fn map_array_int_element(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "array_int_element requires 3 arguments (index, array, value)".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        // Get index variable (1-based in FlatZinc)
        let index_1based = self.get_var(&constraint.args[0])?;
        
        // Convert to 0-based index for Selen
        let index_0based = self.model.sub(index_1based, crate::variables::Val::ValI(1));
        
        // Get array of constants and convert to fixed variables
        let const_array = self.extract_int_array(&constraint.args[1])?;
        let array: Vec<VarId> = const_array.iter()
            .map(|&val| self.model.int(val, val))
            .collect();
        
        // Get value (can be variable or constant)
        let value = match &constraint.args[2] {
            Expr::Ident(_) => self.get_var(&constraint.args[2])?,
            Expr::IntLit(val) => {
                self.model.int(*val as i32, *val as i32)
            }
            _ => {
                return Err(FlatZincError::MapError {
                    message: "Unsupported value type in array_int_element".to_string(),
                    line: Some(constraint.location.line),
                    column: Some(constraint.location.column),
                });
            }
        };
        
        // Apply element constraint: array[index_0based] = value
        self.model.elem(&array, index_0based, value);
        Ok(())
    }
    
    /// Map array_var_bool_element: array[index] = value (boolean version)
    /// FlatZinc signature: array_var_bool_element(index, array, value)
    /// Note: FlatZinc uses 1-based indexing, Selen uses 0-based
    pub(super) fn map_array_var_bool_element(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "array_var_bool_element requires 3 arguments (index, array, value)".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        // Get index variable (1-based in FlatZinc)
        let index_1based = self.get_var(&constraint.args[0])?;
        
        // Convert to 0-based index for Selen
        let index_0based = self.model.sub(index_1based, crate::variables::Val::ValI(1));
        
        // Get array (booleans are represented as 0/1 variables)
        let array = self.extract_var_array(&constraint.args[1])?;
        
        // Get value (can be variable or constant)
        let value = match &constraint.args[2] {
            Expr::Ident(_) => self.get_var(&constraint.args[2])?,
            Expr::BoolLit(b) => {
                let val = if *b { 1 } else { 0 };
                self.model.int(val, val)
            }
            _ => {
                return Err(FlatZincError::MapError {
                    message: "Unsupported value type in array_var_bool_element".to_string(),
                    line: Some(constraint.location.line),
                    column: Some(constraint.location.column),
                });
            }
        };
        
        // Apply element constraint: array[index_0based] = value
        self.model.elem(&array, index_0based, value);
        Ok(())
    }
    
    /// Map array_bool_element: array[index] = value (with constant boolean array)
    /// FlatZinc signature: array_bool_element(index, array, value)
    /// Note: FlatZinc uses 1-based indexing, Selen uses 0-based
    pub(super) fn map_array_bool_element(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "array_bool_element requires 3 arguments (index, array, value)".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        // Get index variable (1-based in FlatZinc)
        let index_1based = self.get_var(&constraint.args[0])?;
        
        // Convert to 0-based index for Selen
        let index_0based = self.model.sub(index_1based, crate::variables::Val::ValI(1));
        
        // Get array of boolean constants and convert to 0/1 fixed variables
        let array: Vec<VarId> = if let Expr::ArrayLit(elements) = &constraint.args[1] {
            elements.iter()
                .map(|elem| {
                    if let Expr::BoolLit(b) = elem {
                        let val = if *b { 1 } else { 0 };
                        Ok(self.model.int(val, val))
                    } else {
                        Err(FlatZincError::MapError {
                            message: "Expected boolean literal in array_bool_element array".to_string(),
                            line: Some(constraint.location.line),
                            column: Some(constraint.location.column),
                        })
                    }
                })
                .collect::<FlatZincResult<Vec<VarId>>>()?
        } else {
            return Err(FlatZincError::MapError {
                message: "Expected array literal in array_bool_element".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        };
        
        // Get value (can be variable or constant)
        let value = match &constraint.args[2] {
            Expr::Ident(_) => self.get_var(&constraint.args[2])?,
            Expr::BoolLit(b) => {
                let val = if *b { 1 } else { 0 };
                self.model.int(val, val)
            }
            _ => {
                return Err(FlatZincError::MapError {
                    message: "Unsupported value type in array_bool_element".to_string(),
                    line: Some(constraint.location.line),
                    column: Some(constraint.location.column),
                });
            }
        };
        
        // Apply element constraint: array[index_0based] = value
        self.model.elem(&array, index_0based, value);
        Ok(())
    }
    
    // ═════════════════════════════════════════════════════════════════════════
    // ➕ ARITHMETIC CONSTRAINTS
    // ═════════════════════════════════════════════════════════════════════════
    
    /// Map int_abs: result = |x|
    /// FlatZinc signature: int_abs(x, result)
    pub(super) fn map_int_abs(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
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
    pub(super) fn map_int_plus(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
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
    pub(super) fn map_int_minus(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
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
    pub(super) fn map_int_times(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
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
    pub(super) fn map_int_div(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
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
    pub(super) fn map_int_mod(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
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
    pub(super) fn map_int_max(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
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
    pub(super) fn map_int_min(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
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

    /// Map bool_le: x <= y for boolean variables
    /// FlatZinc signature: bool_le(x, y)
    pub(super) fn map_bool_le(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "bool_le requires 2 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var(&constraint.args[0])?;
        let y = self.get_var(&constraint.args[1])?;
        
        // For boolean variables: x <= y is equivalent to (not x) or y
        // Which is the same as x => y (implication)
        self.model.new(x.le(y));
        Ok(())
    }
}
