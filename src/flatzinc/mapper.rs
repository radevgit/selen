//! AST to Selen Model Mapper
//!
//! Converts FlatZinc AST into a Selen constraint model.

use crate::flatzinc::ast::*;
use crate::flatzinc::error::{FlatZincError, FlatZincResult};
use crate::prelude::Model;
use crate::variables::VarId;
use crate::runtime_api::{VarIdExt, ModelExt};
use std::collections::HashMap;

// Sub-modules for organization
mod constraint_mappers;
mod helpers;

// Re-export is not needed as methods are already on MappingContext

/// Context for mapping AST to Model
pub struct MappingContext<'a> {
    pub(super) model: &'a mut Model,
    pub(super) var_map: HashMap<String, VarId>,
    /// Maps array names to their variable lists
    pub(super) array_map: HashMap<String, Vec<VarId>>,
    /// Inferred bounds for unbounded integer variables
    pub(super) unbounded_int_bounds: (i32, i32),
}

impl<'a> MappingContext<'a> {
    pub fn new(model: &'a mut Model, unbounded_bounds: (i32, i32)) -> Self {
        MappingContext {
            model,
            var_map: HashMap::new(),
            array_map: HashMap::new(),
            unbounded_int_bounds: unbounded_bounds,
        }
    }
    
    /// Map variable declarations to Selen variables
    fn map_var_decl(&mut self, decl: &VarDecl) -> FlatZincResult<()> {
        let var_id = match &decl.var_type {
            Type::Var(inner_type) => match **inner_type {
                Type::Bool => self.model.bool(),
                Type::Int => {
                    // Unbounded integer variables are approximated using inferred bounds
                    // from other bounded variables in the model
                    let (min_bound, max_bound) = self.unbounded_int_bounds;
                    self.model.int(min_bound, max_bound)
                }
                Type::IntRange(min, max) => {
                    // Validate domain size against Selen's SparseSet limit
                    // Use checked arithmetic to handle potential overflow
                    let domain_size = match max.checked_sub(min) {
                        Some(diff) => match diff.checked_add(1) {
                            Some(size) => size as u64,
                            None => u64::MAX, // Overflow means it's too large
                        },
                        None => u64::MAX, // Overflow means it's too large
                    };
                    
                    const MAX_DOMAIN: u64 = crate::variables::domain::MAX_SPARSE_SET_DOMAIN_SIZE;
                    if domain_size > MAX_DOMAIN {
                        return Err(FlatZincError::MapError {
                            message: format!(
                                "Variable '{}' has domain [{}, {}] with size {} which exceeds maximum of {}",
                                decl.name, min, max, domain_size, MAX_DOMAIN
                            ),
                            line: Some(decl.location.line),
                            column: Some(decl.location.column),
                        });
                    }
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
                                    // Unbounded integer arrays are approximated using inferred bounds
                                    let (min_bound, max_bound) = self.unbounded_int_bounds;
                                    
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
                                        .map(|_| self.model.int(min_bound, max_bound))
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
                Expr::Ident(var_name) => {
                    // Variable-to-variable initialization: var int: c4 = M;
                    // Post an equality constraint: c4 = M
                    let source_var = self.var_map.get(var_name).ok_or_else(|| {
                        FlatZincError::MapError {
                            message: format!("Variable '{}' not found for initialization", var_name),
                            line: Some(decl.location.line),
                            column: Some(decl.location.column),
                        }
                    })?;
                    self.model.new(var_id.eq(*source_var));
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
            // Element constraints (array indexing)
            "array_var_int_element" => self.map_array_var_int_element(constraint),
            "array_int_element" => self.map_array_int_element(constraint),
            "array_var_bool_element" => self.map_array_var_bool_element(constraint),
            "array_bool_element" => self.map_array_bool_element(constraint),
            // Math operations
            "int_abs" => self.map_int_abs(constraint),
            _ => {
                Err(FlatZincError::UnsupportedFeature {
                    feature: format!("Constraint: {}", constraint.predicate),
                    line: Some(constraint.location.line),
                    column: Some(constraint.location.column),
                })
            }
        }
    }
}

/// Infer reasonable bounds for unbounded integer variables by scanning the model
fn infer_unbounded_int_bounds(ast: &FlatZincModel) -> (i32, i32) {
    let mut min_bound = 0i32;
    let mut max_bound = 0i32;
    let mut found_any = false;
    
    // Scan all variable declarations to find bounded integer ranges
    for var_decl in &ast.var_decls {
        match &var_decl.var_type {
            Type::Var(inner_type) => {
                if let Type::IntRange(min, max) = **inner_type {
                    min_bound = min_bound.min(min as i32);
                    max_bound = max_bound.max(max as i32);
                    found_any = true;
                }
            }
            Type::Array { element_type, .. } => {
                if let Type::Var(inner) = &**element_type {
                    if let Type::IntRange(min, max) = **inner {
                        min_bound = min_bound.min(min as i32);
                        max_bound = max_bound.max(max as i32);
                        found_any = true;
                    }
                }
            }
            _ => {}
        }
    }
    
    // If we found bounded variables, expand their range slightly for safety
    if found_any {
        // Expand by 10x or at least to ±100
        let range = max_bound - min_bound;
        let expansion = range.max(100);
        const MAX_BOUND: i32 = (crate::variables::domain::MAX_SPARSE_SET_DOMAIN_SIZE / 2) as i32;
        min_bound = (min_bound - expansion).max(-MAX_BOUND);
        max_bound = (max_bound + expansion).min(MAX_BOUND);
        (min_bound, max_bound)
    } else {
        // No bounded variables found, use default reasonable range
        const DEFAULT_BOUND: i32 = (crate::variables::domain::MAX_SPARSE_SET_DOMAIN_SIZE / 2) as i32;
        (-DEFAULT_BOUND, DEFAULT_BOUND)
    }
}

/// Map FlatZinc AST to an existing Selen Model
pub fn map_to_model_mut(ast: FlatZincModel, model: &mut Model) -> FlatZincResult<()> {
    // First pass: infer reasonable bounds for unbounded variables
    let unbounded_bounds = infer_unbounded_int_bounds(&ast);
    
    let mut ctx = MappingContext::new(model, unbounded_bounds);
    
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

/// Map FlatZinc AST to a new Selen Model
pub fn map_to_model(ast: FlatZincModel) -> FlatZincResult<Model> {
    let mut model = Model::default();
    map_to_model_mut(ast, &mut model)?;
    Ok(model)
}
