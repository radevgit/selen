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
mod constraints;
mod helpers;

// Re-export is not needed as methods are already on MappingContext

/// Context for mapping AST to Model
pub struct MappingContext<'a> {
    pub(super) model: &'a mut Model,
    pub(super) var_map: HashMap<String, VarId>,
    /// Maps array names to their variable lists
    pub(super) array_map: HashMap<String, Vec<VarId>>,
    /// Maps parameter array names to their constant integer values
    pub(super) param_int_arrays: HashMap<String, Vec<i32>>,
    /// Maps parameter array names to their constant boolean values
    pub(super) param_bool_arrays: HashMap<String, Vec<bool>>,
    /// Inferred bounds for unbounded integer variables
    pub(super) unbounded_int_bounds: (i32, i32),
}

impl<'a> MappingContext<'a> {
    pub fn new(model: &'a mut Model, unbounded_bounds: (i32, i32)) -> Self {
        MappingContext {
            model,
            var_map: HashMap::new(),
            array_map: HashMap::new(),
            param_int_arrays: HashMap::new(),
            param_bool_arrays: HashMap::new(),
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
                // Three cases for array declarations:
                // 1. Parameter arrays: array [1..n] of int: coeffs = [1, 2, 3];
                // 2. Variable arrays (collect): array [...] = [var1, var2, ...]
                // 3. Variable arrays (create): array [1..n] of var int: arr
                
                // Check if this is a parameter array (non-var type with initialization)
                if let Some(ref init) = decl.init_value {
                    // Detect parameter integer arrays
                    match **element_type {
                        Type::Int | Type::IntRange(..) | Type::IntSet(..) => {
                            // This is a parameter int array: array [1..n] of int: name = [values];
                            if let Expr::ArrayLit(elements) = init {
                                let values: Result<Vec<i32>, _> = elements.iter()
                                    .map(|e| self.extract_int(e))
                                    .collect();
                                
                                if let Ok(int_values) = values {
                                    self.param_int_arrays.insert(decl.name.clone(), int_values);
                                    return Ok(()); // Parameter arrays don't create variables
                                }
                            }
                        }
                        Type::Bool => {
                            // This is a parameter bool array: array [1..n] of bool: name = [values];
                            if let Expr::ArrayLit(elements) = init {
                                let values: Result<Vec<bool>, _> = elements.iter()
                                    .map(|e| match e {
                                        Expr::BoolLit(b) => Ok(*b),
                                        _ => Err(FlatZincError::MapError {
                                            message: "Expected boolean literal in bool array".to_string(),
                                            line: Some(decl.location.line),
                                            column: Some(decl.location.column),
                                        }),
                                    })
                                    .collect();
                                
                                if let Ok(bool_values) = values {
                                    self.param_bool_arrays.insert(decl.name.clone(), bool_values);
                                    return Ok(()); // Parameter arrays don't create variables
                                }
                            }
                        }
                        _ => {}
                    }
                }
                
                // If not a parameter array, handle as variable array
                if let Some(ref init) = decl.init_value {
                    // Case 2: Array collects existing variables/constants
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
                                    Expr::BoolLit(b) => {
                                        // Constant boolean - create a fixed variable (0 or 1)
                                        let val = if *b { 1 } else { 0 };
                                        let const_var = self.model.int(val, val);
                                        var_ids.push(const_var);
                                    }
                                    Expr::Range(start, end) => {
                                        // Range expression: expand [1..10] to [1,2,3,...,10]
                                        let start_val = match **start {
                                            Expr::IntLit(v) => v,
                                            _ => return Err(FlatZincError::MapError {
                                                message: "Range start must be integer literal".to_string(),
                                                line: Some(decl.location.line),
                                                column: Some(decl.location.column),
                                            }),
                                        };
                                        let end_val = match **end {
                                            Expr::IntLit(v) => v,
                                            _ => return Err(FlatZincError::MapError {
                                                message: "Range end must be integer literal".to_string(),
                                                line: Some(decl.location.line),
                                                column: Some(decl.location.column),
                                            }),
                                        };
                                        // Expand range into individual constants
                                        for val in start_val..=end_val {
                                            let const_var = self.model.int(val as i32, val as i32);
                                            var_ids.push(const_var);
                                        }
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
                                Type::Bool => {
                                    // Boolean array: array [1..n] of var bool: flags
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
                                        .map(|_| self.model.bool())
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
                        Type::Bool => {
                            // Non-var boolean arrays: array [1..n] of bool (should be parameter arrays)
                            // These should have been caught earlier as parameter arrays if initialized
                            return Err(FlatZincError::UnsupportedFeature {
                                feature: "Non-variable boolean arrays without initialization".to_string(),
                                line: Some(decl.location.line),
                                column: Some(decl.location.column),
                            });
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
                Expr::ArrayAccess { array, index } => {
                    // Array element initialization: var int: x = arr[3];
                    // Evaluate the array access and post an equality constraint
                    let source_var = self.evaluate_array_access(array, index)?;
                    self.model.new(var_id.eq(source_var));
                }
                _ => {
                    return Err(FlatZincError::MapError {
                        message: format!("Complex initialization not yet supported: {:?}", init),
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
            "int_lin_eq_reif" => self.map_int_lin_eq_reif(constraint),
            "int_lin_le_reif" => self.map_int_lin_le_reif(constraint),
            "fzn_all_different_int" | "all_different_int" | "all_different" => self.map_all_different(constraint),
            "sort" => self.map_sort(constraint),
            "table_int" => self.map_table_int(constraint),
            "table_bool" => self.map_table_bool(constraint),
            "lex_less" | "lex_less_int" => self.map_lex_less(constraint),
            "lex_lesseq" | "lex_lesseq_int" => self.map_lex_lesseq(constraint),
            "nvalue" => self.map_nvalue(constraint),
            "fixed_fzn_cumulative" | "cumulative" => self.map_fixed_fzn_cumulative(constraint),
            "var_fzn_cumulative" => self.map_var_fzn_cumulative(constraint),
            "int_eq_reif" => self.map_int_eq_reif(constraint),
            "int_ne_reif" => self.map_int_ne_reif(constraint),
            "int_lt_reif" => self.map_int_lt_reif(constraint),
            "int_le_reif" => self.map_int_le_reif(constraint),
            "int_gt_reif" => self.map_int_gt_reif(constraint),
            "int_ge_reif" => self.map_int_ge_reif(constraint),
            "bool_clause" => self.map_bool_clause(constraint),
            // Array aggregations
            "array_int_minimum" | "minimum_int" => self.map_array_int_minimum(constraint),
            "array_int_maximum" | "maximum_int" => self.map_array_int_maximum(constraint),
            "array_bool_and" => self.map_array_bool_and(constraint),
            "array_bool_or" => self.map_array_bool_or(constraint),
            // Bool-int conversion
            "bool2int" => self.map_bool2int(constraint),
            "bool_eq_reif" => self.map_bool_eq_reif(constraint),
            // Count constraints
            "count_eq" | "count" => self.map_count_eq(constraint),
            // Element constraints (array indexing)
            "array_var_int_element" => self.map_array_var_int_element(constraint),
            "array_int_element" => self.map_array_int_element(constraint),
            "array_var_bool_element" => self.map_array_var_bool_element(constraint),
            "array_bool_element" => self.map_array_bool_element(constraint),
            // Arithmetic operations
            "int_abs" => self.map_int_abs(constraint),
            "int_plus" => self.map_int_plus(constraint),
            "int_minus" => self.map_int_minus(constraint),
            "int_times" => self.map_int_times(constraint),
            "int_div" => self.map_int_div(constraint),
            "int_mod" => self.map_int_mod(constraint),
            "int_max" => self.map_int_max(constraint),
            "int_min" => self.map_int_min(constraint),
            // Boolean constraints
            "bool_le" => self.map_bool_le(constraint),
            "bool_le_reif" => self.map_bool_le_reif(constraint),
            "bool_eq" => self.map_bool_eq(constraint),
            "bool_not" => self.map_bool_not(constraint),
            "bool_xor" => self.map_bool_xor(constraint),
            // Set constraints
            "set_in_reif" => self.map_set_in_reif(constraint),
            "set_in" => self.map_set_in(constraint),
            // Global cardinality
            "global_cardinality" => self.map_global_cardinality(constraint),
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
