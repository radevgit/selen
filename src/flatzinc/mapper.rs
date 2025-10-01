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
}

impl<'a> MappingContext<'a> {
    pub fn new(model: &'a mut Model) -> Self {
        MappingContext {
            model,
            var_map: HashMap::new(),
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
            Type::Array { index_sets: _, element_type: _ } => {
                return Err(FlatZincError::UnsupportedFeature {
                    feature: "Array variables (use programmatic array creation)".to_string(),
                    line: Some(decl.location.line),
                    column: Some(decl.location.column),
                });
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
    
    fn extract_var_array(&self, expr: &Expr) -> FlatZincResult<Vec<VarId>> {
        match expr {
            Expr::ArrayLit(elements) => {
                elements.iter().map(|e| self.get_var(e)).collect()
            }
            Expr::Ident(name) => {
                // Handle single variable treated as array
                Ok(vec![self.var_map.get(name).copied().ok_or_else(|| {
                    FlatZincError::MapError {
                        message: format!("Unknown variable: {}", name),
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
