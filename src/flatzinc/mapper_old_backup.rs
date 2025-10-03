//! AST to Selen Model Mapper
//!
//! Converts FlatZinc AST into a Selen constraint model.

use crate::flatzinc::ast::*;
use crate::flatzinc::error::{FlatZincError, FlatZincResult};
use crate::prelude::Model;
use crate::variables::VarId;
use crate::runtime_api::{VarIdExt, ModelExt};
use std::collections::HashMap;

/// Context for mapping AST to Model
pub struct MappingContext<'a> {
    model: &'a mut Model,
    var_map: HashMap<String, VarId>,
    /// Maps array names to their variable lists
    array_map: HashMap<String, Vec<VarId>>,
}

impl<'a> MappingContext<'a> {
    pub fn new(model: &'a mut Model) -> Self {
        MappingContext {
            model,
            var_map: HashMap::new(),
            array_map: HashMap::new(),
        }
    }
    
    /// Map variable declarations to Selen variables
    fn map_var_decl(&mut self, decl: &VarDecl) -> FlatZincResult<()> {
        let var_id = match &decl.var_type {
            Type::Var(inner_type) => match **inner_type {
                Type::Bool => self.model.bool(),
                Type::Int => self.model.int(i32::MIN, i32::MAX),
                Type::IntRange(min, max) => {
                    self.model.int(min as i32, max as i32)
                }
                Type::IntSet(ref values) => {
                    if values.is_empty() {
                        return Err(FlatZincError::MapError {
                            message: format!("Empty domain for variable {}", decl.name),
                            line: Some(decl.location.line),
                            column: Some(decl.location.column),
                        });
                    }
                    let min = *values.iter().min().unwrap();
                    let max = *values.iter().max().unwrap();
                    // TODO: Handle sparse domains more efficiently
                    self.model.int(min as i32, max as i32)
                }
                Type::Float => self.model.float(f64::NEG_INFINITY, f64::INFINITY),
                Type::FloatRange(min, max) => self.model.float(min, max),
                _ => {
                    return Err(FlatZincError::UnsupportedFeature {
                        feature: format!("Variable type: {:?}", inner_type),
                        line: Some(decl.location.line),
                        column: Some(decl.location.column),
                    });
                }
            },
            Type::Array { index_sets, element_type } => {
                // Two cases for array variables:
                // 1. Collecting existing variables: array [...] = [var1, var2, ...]
                // 2. Creating new array of variables: array [1..n] of var int: arr
                
                if let Some(ref init) = decl.init_value {
                    // Case 1: Array collects existing variables/constants
                    match init {
                        Expr::ArrayLit(elements) => {
                            let mut var_ids = Vec::new();
                            for elem in elements {
                                match elem {
                                    Expr::Ident(name) => {
                                        // Reference to existing variable
                                        let var_id = self.var_map.get(name).ok_or_else(|| {
                                            FlatZincError::MapError {
                                                message: format!("Undefined variable '{}' in array", name),
                                                line: Some(decl.location.line),
                                                column: Some(decl.location.column),
                                            }
                                        })?;
                                        var_ids.push(*var_id);
                                    }
                                    Expr::IntLit(val) => {
                                        // Constant integer - create a fixed variable
                                        let const_var = self.model.int(*val as i32, *val as i32);
                                        var_ids.push(const_var);
                                    }
                                    _ => {
                                        return Err(FlatZincError::UnsupportedFeature {
                                            feature: format!("Array element expression: {:?}", elem),
                                            line: Some(decl.location.line),
                                            column: Some(decl.location.column),
                                        });
                                    }
                                }
                            }
                            // Store the array mapping
                            self.array_map.insert(decl.name.clone(), var_ids);
                            return Ok(()); // Arrays don't create new variables
                        }
                        _ => {
                            return Err(FlatZincError::UnsupportedFeature {
                                feature: format!("Array initialization: {:?}", init),
                                line: Some(decl.location.line),
                                column: Some(decl.location.column),
                            });
                        }
                    }
                } else {
                    // Case 2: Create new array of variables (no initialization)
                    // e.g., array [1..5] of var 1..5: animal
                    match **element_type {
                        Type::Var(ref inner) => {
                            match **inner {
                                Type::IntRange(min, max) => {
                                    // Determine array size from index_sets
                                    // For now, assume single index set [1..n]
                                    let size = if let Some(IndexSet::Range(start, end)) = index_sets.first() {
                                        (end - start + 1) as usize
                                    } else {
                                        return Err(FlatZincError::UnsupportedFeature {
                                            feature: "Array with complex index sets".to_string(),
                                            line: Some(decl.location.line),
                                            column: Some(decl.location.column),
                                        });
                                    };
                                    
                                    // Create variables for each array element
                                    let var_ids: Vec<VarId> = (0..size)
                                        .map(|_| self.model.int(min as i32, max as i32))
                                        .collect();
                                    
                                    self.array_map.insert(decl.name.clone(), var_ids);
                                    return Ok(());
                                }
                                Type::Int => {
                                    // Full integer domain
                                    let size = if let Some(IndexSet::Range(start, end)) = index_sets.first() {
                                        (end - start + 1) as usize
                                    } else {
                                        return Err(FlatZincError::UnsupportedFeature {
                                            feature: "Array with complex index sets".to_string(),
                                            line: Some(decl.location.line),
                                            column: Some(decl.location.column),
                                        });
                                    };
                                    
                                    let var_ids: Vec<VarId> = (0..size)
                                        .map(|_| self.model.int(i32::MIN, i32::MAX))
                                        .collect();
                                    
                                    self.array_map.insert(decl.name.clone(), var_ids);
                                    return Ok(());
                                }
                                _ => {
                                    return Err(FlatZincError::UnsupportedFeature {
                                        feature: format!("Array element type: {:?}", inner),
                                        line: Some(decl.location.line),
                                        column: Some(decl.location.column),
                                    });
                                }
                            }
                        }
                        _ => {
                            return Err(FlatZincError::UnsupportedFeature {
                                feature: format!("Array element type: {:?}", element_type),
                                line: Some(decl.location.line),
                                column: Some(decl.location.column),
                            });
                        }
                    }
                }
            }
            _ => {
                return Err(FlatZincError::MapError {
                    message: format!("Unexpected variable type: {:?}", decl.var_type),
                    line: Some(decl.location.line),
                    column: Some(decl.location.column),
                });
            }
        };
        
        // Handle initialization
        if let Some(ref init) = decl.init_value {
            match init {
                Expr::IntLit(val) => {
                    self.model.new(var_id.eq(*val as i32));
                }
                Expr::BoolLit(val) => {
                    self.model.new(var_id.eq(if *val { 1 } else { 0 }));
                }
                Expr::FloatLit(val) => {
                    self.model.new(var_id.eq(*val));
                }
                _ => {
                    return Err(FlatZincError::MapError {
                        message: "Complex initialization not yet supported".to_string(),
                        line: Some(decl.location.line),
                        column: Some(decl.location.column),
                    });
                }
            }
        }
        
        self.var_map.insert(decl.name.clone(), var_id);
        Ok(())
    }
    
    /// Map a constraint to Selen constraint
    fn map_constraint(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        match constraint.predicate.as_str() {
            "int_eq" => self.map_int_eq(constraint),
            "int_ne" => self.map_int_ne(constraint),
            "int_lt" => self.map_int_lt(constraint),
            "int_le" => self.map_int_le(constraint),
            "int_gt" => self.map_int_gt(constraint),
            "int_ge" => self.map_int_ge(constraint),
            "int_lin_eq" => self.map_int_lin_eq(constraint),
            "int_lin_le" => self.map_int_lin_le(constraint),
            "int_lin_ne" => self.map_int_lin_ne(constraint),
            "fzn_all_different_int" | "all_different_int" | "all_different" => self.map_all_different(constraint),
            "int_eq_reif" => self.map_int_eq_reif(constraint),
            "int_ne_reif" => self.map_int_ne_reif(constraint),
            "int_lt_reif" => self.map_int_lt_reif(constraint),
            "int_le_reif" => self.map_int_le_reif(constraint),
            "int_gt_reif" => self.map_int_gt_reif(constraint),
            "int_ge_reif" => self.map_int_ge_reif(constraint),
            "bool_clause" => self.map_bool_clause(constraint),
            // Array aggregations
            "array_int_minimum" => self.map_array_int_minimum(constraint),
            "array_int_maximum" => self.map_array_int_maximum(constraint),
            "array_bool_and" => self.map_array_bool_and(constraint),
            "array_bool_or" => self.map_array_bool_or(constraint),
            // Bool-int conversion
            "bool2int" => self.map_bool2int(constraint),
            // Count constraints
            "count_eq" => self.map_count_eq(constraint),
            _ => {
                Err(FlatZincError::UnsupportedFeature {
                    feature: format!("Constraint: {}", constraint.predicate),
                    line: Some(constraint.location.line),
                    column: Some(constraint.location.column),
                })
            }
        }
    }
    
    fn get_var(&self, expr: &Expr) -> FlatZincResult<VarId> {
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
    
    fn map_int_eq(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "int_eq requires 2 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var(&constraint.args[0])?;
        
        match &constraint.args[1] {
            Expr::Ident(_) => {
                let y = self.get_var(&constraint.args[1])?;
                self.model.new(x.eq(y));
            }
            Expr::IntLit(val) => {
                self.model.new(x.eq(*val as i32));
            }
            _ => {
                return Err(FlatZincError::MapError {
                    message: "Unsupported argument type for int_eq".to_string(),
                    line: Some(constraint.location.line),
                    column: Some(constraint.location.column),
                });
            }
        }
        
        Ok(())
    }
    
    fn map_int_ne(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "int_ne requires 2 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var(&constraint.args[0])?;
        let y = self.get_var(&constraint.args[1])?;
        self.model.new(x.ne(y));
        Ok(())
    }
    
    fn map_int_lt(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "int_lt requires 2 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var(&constraint.args[0])?;
        let y = self.get_var(&constraint.args[1])?;
        self.model.new(x.lt(y));
        Ok(())
    }
    
    fn map_int_le(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "int_le requires 2 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var(&constraint.args[0])?;
        let y = self.get_var(&constraint.args[1])?;
        self.model.new(x.le(y));
        Ok(())
    }
    
    fn map_int_gt(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "int_gt requires 2 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var(&constraint.args[0])?;
        let y = self.get_var(&constraint.args[1])?;
        self.model.new(x.gt(y));
        Ok(())
    }
    
    fn map_int_ge(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "int_ge requires 2 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var(&constraint.args[0])?;
        let y = self.get_var(&constraint.args[1])?;
        self.model.new(x.ge(y));
        Ok(())
    }
    
    fn map_int_lin_eq(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        // int_lin_eq([coeffs], [vars], constant)
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
        
        if coeffs.len() != var_ids.len() {
            return Err(FlatZincError::MapError {
                message: "Coefficient and variable array lengths must match".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        self.model.int_lin_eq(&coeffs, &var_ids, constant);
        Ok(())
    }
    
    fn map_int_lin_le(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        // int_lin_le([coeffs], [vars], constant)
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
        
        self.model.int_lin_le(&coeffs, &var_ids, constant);
        Ok(())
    }
    
    fn map_int_lin_ne(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        // int_lin_ne([coeffs], [vars], constant)
        // Decompose as: sum(coeffs[i] * vars[i]) ≠ constant
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
        
        // Create sum using Model's API
        let scaled_vars: Vec<VarId> = coeffs
            .iter()
            .zip(var_ids.iter())
            .map(|(&coeff, &var)| self.model.mul(var, crate::variables::Val::ValI(coeff)))
            .collect();
        
        let sum_var = self.model.sum(&scaled_vars);
        
        // Use runtime API to post not-equals constraint: sum ≠ constant
        use crate::runtime_api::ModelExt;
        self.model.c(sum_var).ne(constant);
        Ok(())
    }
    
    fn map_all_different(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
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
    
    fn map_int_eq_reif(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "int_eq_reif requires 3 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var(&constraint.args[0])?;
        let y = self.get_var(&constraint.args[1])?;
        let b = self.get_var(&constraint.args[2])?;
        self.model.int_eq_reif(x, y, b);
        Ok(())
    }
    
    fn map_int_ne_reif(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "int_ne_reif requires 3 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var(&constraint.args[0])?;
        let y = self.get_var(&constraint.args[1])?;
        let b = self.get_var(&constraint.args[2])?;
        self.model.int_ne_reif(x, y, b);
        Ok(())
    }
    
    fn map_int_lt_reif(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "int_lt_reif requires 3 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var(&constraint.args[0])?;
        let y = self.get_var(&constraint.args[1])?;
        let b = self.get_var(&constraint.args[2])?;
        self.model.int_lt_reif(x, y, b);
        Ok(())
    }
    
    fn map_int_le_reif(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "int_le_reif requires 3 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var(&constraint.args[0])?;
        let y = self.get_var(&constraint.args[1])?;
        let b = self.get_var(&constraint.args[2])?;
        self.model.int_le_reif(x, y, b);
        Ok(())
    }
    
    fn map_int_gt_reif(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "int_gt_reif requires 3 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var(&constraint.args[0])?;
        let y = self.get_var(&constraint.args[1])?;
        let b = self.get_var(&constraint.args[2])?;
        self.model.int_gt_reif(x, y, b);
        Ok(())
    }
    
    fn map_int_ge_reif(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "int_ge_reif requires 3 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.get_var(&constraint.args[0])?;
        let y = self.get_var(&constraint.args[1])?;
        let b = self.get_var(&constraint.args[2])?;
        self.model.int_ge_reif(x, y, b);
        Ok(())
    }
    
    fn map_bool_clause(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "bool_clause requires 2 arguments".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let pos = self.extract_var_array(&constraint.args[0])?;
        let neg = self.extract_var_array(&constraint.args[1])?;
        self.model.bool_clause(&pos, &neg);
        Ok(())
    }
    
    fn extract_int(&self, expr: &Expr) -> FlatZincResult<i32> {
        match expr {
            Expr::IntLit(val) => Ok(*val as i32),
            _ => Err(FlatZincError::MapError {
                message: "Expected integer literal".to_string(),
                line: None,
                column: None,
            }),
        }
    }
    
    fn map_array_int_minimum(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
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
    
    fn map_array_int_maximum(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
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
    
    fn map_array_bool_and(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
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
    
    fn map_array_bool_or(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
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
    
    fn map_bool2int(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
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
    
    fn map_count_eq(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
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
        use crate::runtime_api::ModelExt;
        self.model.count(&arr_vars, value, count_var);
        Ok(())
    }
    
    fn extract_int_array(&self, expr: &Expr) -> FlatZincResult<Vec<i32>> {
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
    
    fn extract_var_array(&mut self, expr: &Expr) -> FlatZincResult<Vec<VarId>> {
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

/// Map FlatZinc AST to an existing Selen Model
pub fn map_to_model_mut(ast: FlatZincModel, model: &mut Model) -> FlatZincResult<()> {
    let mut ctx = MappingContext::new(model);
    
    // Map variable declarations
    for var_decl in &ast.var_decls {
        ctx.map_var_decl(var_decl)?;
    }
    
    // Map constraints
    for constraint in &ast.constraints {
        ctx.map_constraint(constraint)?;
    }
    
    // TODO: Handle solve goal (minimize/maximize)
    
    Ok(())
}
