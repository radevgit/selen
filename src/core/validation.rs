//! Model validation and constraint analysis.
//!
//! This module provides comprehensive validation of CSP models before solving
//! to catch modeling errors early and provide helpful error messages.
//!
//! # Validation Types
//!
//! The validator checks for several categories of problems:
//! - **Variable domain issues**: Empty domains, invalid bounds, type mismatches
//! - **Constraint conflicts**: Contradictory constraints that make models unsolvable
//! - **Reference validation**: All constraint variables exist and are properly referenced
//! - **Constraint compatibility**: Duplicate variables in constraints that require uniqueness
//!
//! # Automatic Validation
//!
//! Validation runs automatically before solving, so users typically don't need
//! to call validation methods directly. However, explicit validation can be useful
//! for debugging model construction.
//!
//! # Example
//!
//! ```rust
//! use selen::prelude::*;
//!
//! let mut m = Model::default();
//! let x = m.int(1, 10);
//! let y = m.int(5, 15);
//!
//! // This will be caught by validation before solving
//! post!(m, x == int(20)); // x can't equal 20 (outside domain)
//! post!(m, x == int(5));  // Conflicting with above
//!
//! // Validation error will be reported when solve() is called
//! match m.solve() {
//!     Ok(solution) => println!("Solution: {:?}", solution),
//!     Err(e) => println!("Validation error: {}", e),
//! }
//! ```

use crate::core::error::SolverError;
use crate::variables::{Vars, Var, VarId};
use crate::constraints::props::Propagators;
use crate::optimization::constraint_metadata::ConstraintType;
use std::collections::{HashMap, HashSet};

#[doc(hidden)]
/// Comprehensive model validation system that checks for:
/// - Conflicting constraints that make the model unsolvable
/// - Invalid variable domains (empty, inconsistent bounds)
/// - Constraint compatibility issues (duplicate variables in AllDifferent, etc.)
/// - Variable reference validation (all constraint variables exist)
/// 
/// This validation runs automatically before solving to catch modeling errors early.
pub struct ModelValidator<'a> {
    vars: &'a Vars,
    props: &'a Propagators,
}

/// Validation result with specific error types for better error reporting
#[derive(Debug, Clone)]
pub enum ValidationError {
    /// Empty or invalid variable domain
    InvalidDomain {
        variable_id: VarId,
        issue: DomainIssue,
    },
    /// Constraint references non-existent variable
    InvalidVariableReference {
        constraint_id: usize,
        variable_id: VarId,
        constraint_type: String,
    },
    /// Conflicting constraints detected
    ConflictingConstraints {
        conflict_type: ConflictType,
        variables: Vec<VarId>,
        constraint_details: String,
    },
    /// Constraint has invalid parameters
    InvalidConstraintParameters {
        constraint_id: usize,
        constraint_type: String,
        issue: String,
    },
}

#[derive(Debug, Clone)]
pub enum DomainIssue {
    EmptyDomain,
    InvalidBounds { min: i32, max: i32 },
    FloatPrecisionIssue { interval: String },
}

#[derive(Debug, Clone)]
pub enum ConflictType {
    /// Multiple equality constraints on same variable with different values
    DirectValueConflict,
    /// AllDifferent constraint with insufficient domain size
    AllDifferentDomainTooSmall,
    /// Constraint combination that creates empty intersection
    EmptyIntersection,
}

impl<'a> ModelValidator<'a> {
    /// Create a new validator for the given model components
    pub fn new(vars: &'a Vars, props: &'a Propagators) -> Self {
        Self { vars, props }
    }
    
    /// Perform comprehensive model validation
    /// 
    /// This method runs all validation checks and returns the first error found,
    /// or Ok(()) if the model is valid and ready for solving.
    pub fn validate(&self) -> Result<(), SolverError> {
        // 1. Validate variable domains
        self.validate_variable_domains()?;
        
        // 2. Validate constraint variable references
        self.validate_constraint_references()?;
        
        // 3. Check for constraint conflicts
        self.validate_constraint_conflicts()?;
        
        // 4. Validate constraint parameters
        self.validate_constraint_parameters()?;
        
        Ok(())
    }
    
    /// Check that all variable domains are valid and non-empty
    fn validate_variable_domains(&self) -> Result<(), SolverError> {
        for (var_id, var) in self.vars.iter_with_indices() {
            match var {
                Var::VarI(sparse_set) => {
                    // Check for empty domain which might indicate invalid bounds
                    if sparse_set.is_empty() {
                        // For empty domains, check if this might be due to invalid input bounds
                        let min_val = sparse_set.min_universe_value();
                        let max_val = sparse_set.max_universe_value();
                        
                        // If max_universe_value < min_universe_value, this suggests invalid input bounds
                        if max_val < min_val {
                            return Err(SolverError::InvalidDomain {
                                message: format!("Variable created with invalid bounds: min ({}) > max ({})", min_val, max_val),
                                variable_name: Some(format!("var_{:?}", var_id)),
                                domain_info: Some("integer variable bounds are reversed".to_string()),
                            });
                        } else {
                            return Err(SolverError::InvalidDomain {
                                message: "Variable domain is empty".to_string(),
                                variable_name: Some(format!("var_{:?}", var_id)),
                                domain_info: Some("integer domain with no valid values".to_string()),
                            });
                        }
                    }
                    
                    let min_val = sparse_set.min_universe_value();
                    let max_val = sparse_set.max_universe_value();
                    
                    if min_val > max_val {
                        return Err(SolverError::InvalidDomain {
                            message: "Variable domain bounds are invalid".to_string(),
                            variable_name: Some(format!("var_{:?}", var_id)),
                            domain_info: Some(format!("min ({}) > max ({})", min_val, max_val)),
                        });
                    }
                    
                    // Check for extremely large domains that might cause performance issues
                    let domain_size = sparse_set.universe_size();
                    if domain_size > crate::variables::domain::MAX_SPARSE_SET_DOMAIN_SIZE as usize {
                        return Err(SolverError::InvalidDomain {
                            message: "Variable domain is too large and may cause performance issues".to_string(),
                            variable_name: Some(format!("var_{:?}", var_id)),
                            domain_info: Some(format!("domain size: {} (max: {})", domain_size, crate::variables::domain::MAX_SPARSE_SET_DOMAIN_SIZE)),
                        });
                    }
                },
                Var::VarF(interval) => {
                    if interval.min > interval.max {
                        return Err(SolverError::InvalidDomain {
                            message: format!("Float variable created with invalid bounds: min ({}) > max ({})", interval.min, interval.max),
                            variable_name: Some(format!("var_{:?}", var_id)),
                            domain_info: Some("float variable bounds are reversed".to_string()),
                        });
                    }
                    
                    if interval.min.is_infinite() || interval.max.is_infinite() {
                        return Err(SolverError::InvalidDomain {
                            message: "Float variable has infinite bounds".to_string(),
                            variable_name: Some(format!("var_{:?}", var_id)),
                            domain_info: Some(format!("bounds: [{}, {}]", interval.min, interval.max)),
                        });
                    }
                    
                    if interval.min.is_nan() || interval.max.is_nan() {
                        return Err(SolverError::InvalidDomain {
                            message: "Float variable has NaN bounds".to_string(),
                            variable_name: Some(format!("var_{:?}", var_id)),
                            domain_info: Some("NaN values are not allowed in variable bounds".to_string()),
                        });
                    }
                },
            }
        }
        Ok(())
    }
    
    /// Validate that all constraint variable references are valid
    fn validate_constraint_references(&self) -> Result<(), SolverError> {
        let constraint_registry = self.props.get_constraint_registry();
        
        for constraint_id in constraint_registry.get_all_constraint_ids() {
            if let Some(metadata) = constraint_registry.get_constraint(constraint_id) {
                // Check that all variables referenced by this constraint exist
                for &var_id in &metadata.variables {
                    if var_id.to_index() >= self.vars.count() {
                        return Err(SolverError::InvalidVariable {
                            message: "Constraint references non-existent variable".to_string(),
                            variable_id: Some(format!("var_{:?}", var_id)),
                            expected: Some(format!("0 to {}", self.vars.count() - 1)),
                        });
                    }
                }
            }
        }
        Ok(())
    }
    
    /// Check for conflicting constraints that make the model unsolvable
    fn validate_constraint_conflicts(&self) -> Result<(), SolverError> {
        let constraint_registry = self.props.get_constraint_registry();
        
        // Group constraints by the variables they affect
        let mut variable_constraints: HashMap<VarId, Vec<(usize, &ConstraintType)>> = HashMap::with_capacity(64);
        
        for constraint_id in constraint_registry.get_all_constraint_ids() {
            if let Some(metadata) = constraint_registry.get_constraint(constraint_id) {
                for &var_id in &metadata.variables {
                    variable_constraints
                        .entry(var_id)
                        .or_insert_with(Vec::new)
                        .push((constraint_id.0, &metadata.constraint_type));
                }
            }
        }
        
        // Check for obvious conflicts
        for (_var_id, constraints) in variable_constraints.iter() {
            // Look for multiple equality constraints on the same variable
            let mut equality_constraints = Vec::new();
            for &(constraint_id, constraint_type) in constraints {
                if matches!(constraint_type, ConstraintType::Equals) {
                    equality_constraints.push(constraint_id);
                }
            }
            
            // For now, we'll be more permissive with multiple equality constraints.
            // The original logic was too strict and flagged valid cases like x==y and x==5.
            // We should only flag conflicts when we can prove they're incompatible.
            // TODO: Implement more sophisticated conflict detection that analyzes constraint values
            if equality_constraints.len() > 1 {
                // Skip validation for now - let the solver handle constraint compatibility
                // This avoids false positives while allowing valid constraint combinations
            }
        }
        
        // Check AllDifferent constraints for sufficient domain size
        self.validate_alldiff_constraints()?;
        
        Ok(())
    }
    
    /// Validate AllDifferent constraints have sufficient domain sizes
    fn validate_alldiff_constraints(&self) -> Result<(), SolverError> {
        let constraint_registry = self.props.get_constraint_registry();
        let alldiff_constraints = constraint_registry.get_constraints_by_type(&ConstraintType::AllDifferent);
        
        for constraint_id in alldiff_constraints {
            if let Some(metadata) = constraint_registry.get_constraint(constraint_id) {
                let variables = &metadata.variables;
                let num_variables = variables.len();
                
                if num_variables <= 1 {
                    continue; // Trivially satisfiable
                }
                
                // For AllDifferent constraints, check for obvious conflicts
                // 1. If two variables are already fixed to the same value, that's a conflict
                // 2. If we have more variables than available distinct values, that's a conflict
                let mut fixed_values = std::collections::HashSet::new();
                let mut all_possible_values = std::collections::HashSet::new();
                
                for &var_id in variables {
                    let var = &self.vars[var_id];
                    match var {
                        Var::VarI(sparse_set) => {
                            // Add all possible values to the union
                            for val in sparse_set.iter() {
                                all_possible_values.insert(val);
                            }
                            
                            // If variable is fixed (size 1), check for duplicates
                            if sparse_set.size() == 1 {
                                let fixed_val = sparse_set.iter().next().unwrap();
                                if fixed_values.contains(&fixed_val) {
                                    return Err(SolverError::ConflictingConstraints {
                                        constraint_names: Some(vec![format!("alldiff_constraint_{}", constraint_id.0)]),
                                        variables: Some(variables.iter().map(|id| format!("var_{:?}", id)).collect()),
                                        context: Some(format!(
                                            "AllDifferent constraint has two variables fixed to the same value: {}",
                                            fixed_val
                                        )),
                                    });
                                }
                                fixed_values.insert(fixed_val);
                            }
                        }
                        Var::VarF(_) => {
                            // Float variables have essentially infinite domains for AllDifferent purposes
                            continue;
                        }
                    };
                }
                
                // Check if we have enough distinct values available
                if all_possible_values.len() < num_variables {
                    return Err(SolverError::ConflictingConstraints {
                        constraint_names: Some(vec![format!("alldiff_constraint_{}", constraint_id.0)]),
                        variables: Some(variables.iter().map(|id| format!("var_{:?}", id)).collect()),
                        context: Some(format!(
                            "AllDifferent constraint requires {} distinct values, but only {} distinct values are available in the domains",
                            num_variables, all_possible_values.len()
                        )),
                    });
                }
            }
        }
        Ok(())
    }
    
    /// Validate constraint parameters for common issues
    fn validate_constraint_parameters(&self) -> Result<(), SolverError> {
        let constraint_registry = self.props.get_constraint_registry();
        
        for constraint_id in constraint_registry.get_all_constraint_ids() {
            if let Some(metadata) = constraint_registry.get_constraint(constraint_id) {
                match &metadata.constraint_type {
                    ConstraintType::AllDifferent => {
                        // Check for duplicate variables in AllDifferent
                        let variables = &metadata.variables;
                        let mut seen_vars = HashSet::new();
                        for &var_id in variables {
                            if !seen_vars.insert(var_id) {
                                return Err(SolverError::InvalidConstraint {
                                    message: "AllDifferent constraint contains duplicate variables".to_string(),
                                    constraint_name: Some(format!("alldiff_constraint_{}", constraint_id.0)),
                                    variables: Some(vec![format!("var_{:?} (duplicate)", var_id)]),
                                });
                            }
                        }
                    },
                    ConstraintType::Addition | ConstraintType::Multiplication => {
                        // These constraints need exactly 3 operands: x, y, result
                        // But the variable count can be 2 if one operand is a constant
                        let operand_count = if let crate::optimization::constraint_metadata::ConstraintData::NAry { operands } = &metadata.data {
                            operands.len()
                        } else {
                            metadata.variables.len()
                        };
                        
                        if operand_count != 3 {
                            return Err(SolverError::InvalidConstraint {
                                message: format!(
                                    "{:?} constraint requires exactly 3 operands (x, y, result), got {}",
                                    metadata.constraint_type, operand_count
                                ),
                                constraint_name: Some(format!("constraint_{}", constraint_id.0)),
                                variables: Some(metadata.variables.iter().map(|id| format!("var_{:?}", id)).collect()),
                            });
                        }
                        
                        // Variable count should be 2 or 3 (depending on whether constants are involved)
                        if metadata.variables.len() < 2 || metadata.variables.len() > 3 {
                            return Err(SolverError::InvalidConstraint {
                                message: format!(
                                    "{:?} constraint requires 2-3 variables, got {}",
                                    metadata.constraint_type, metadata.variables.len()
                                ),
                                constraint_name: Some(format!("constraint_{}", constraint_id.0)),
                                variables: Some(metadata.variables.iter().map(|id| format!("var_{:?}", id)).collect()),
                            });
                        }
                    },
                    ConstraintType::Division | ConstraintType::Modulo => {
                        // Division and modulo need special validation for zero divisors
                        if metadata.variables.len() != 3 {
                            return Err(SolverError::InvalidConstraint {
                                message: format!(
                                    "{:?} constraint requires exactly 3 variables (dividend, divisor, result), got {}",
                                    metadata.constraint_type, metadata.variables.len()
                                ),
                                constraint_name: Some(format!("constraint_{}", constraint_id.0)),
                                variables: Some(metadata.variables.iter().map(|id| format!("var_{:?}", id)).collect()),
                            });
                        }
                        
                        // Check if divisor variable domain includes zero
                        if metadata.variables.len() >= 2 {
                            let divisor_var_id = metadata.variables[1];
                            let divisor_var = &self.vars[divisor_var_id];
                            if let Var::VarI(sparse_set) = divisor_var {
                                if sparse_set.contains(0) {
                                    return Err(SolverError::InvalidConstraint {
                                        message: "Division/Modulo constraint has divisor that can be zero".to_string(),
                                        constraint_name: Some(format!("constraint_{}", constraint_id.0)),
                                        variables: Some(vec![format!("var_{:?} (divisor)", divisor_var_id)]),
                                    });
                                }
                            }
                        }
                    },
                    ConstraintType::Minimum | ConstraintType::Maximum => {
                        // Min/Max constraints need at least 2 input variables plus 1 result variable
                        if metadata.variables.len() < 2 {
                            return Err(SolverError::InvalidInput {
                                message: format!(
                                    "{:?} constraint requires at least 1 input variable, got {}",
                                    metadata.constraint_type, 
                                    metadata.variables.len().saturating_sub(1) // Subtract result variable
                                ),
                                function_name: Some(match metadata.constraint_type {
                                    ConstraintType::Minimum => "min".to_string(),
                                    ConstraintType::Maximum => "max".to_string(),
                                    _ => unreachable!(),
                                }),
                                expected: Some("non-empty slice of variable IDs".to_string()),
                            });
                        }
                        
                        // Check for empty input list (all variables except the last one which is the result)
                        let input_var_count = metadata.variables.len().saturating_sub(1);
                        if input_var_count == 0 {
                            return Err(SolverError::InvalidInput {
                                message: format!(
                                    "Cannot compute {} of empty variable list",
                                    match metadata.constraint_type {
                                        ConstraintType::Minimum => "minimum",
                                        ConstraintType::Maximum => "maximum", 
                                        _ => unreachable!(),
                                    }
                                ),
                                function_name: Some(match metadata.constraint_type {
                                    ConstraintType::Minimum => "min".to_string(),
                                    ConstraintType::Maximum => "max".to_string(),
                                    _ => unreachable!(),
                                }),
                                expected: Some("non-empty slice of variable IDs".to_string()),
                            });
                        }
                    },
                    _ => {
                        // Additional constraint-specific validations can be added here
                    }
                }
            }
        }
        Ok(())
    }
}

impl From<ValidationError> for SolverError {
    fn from(validation_error: ValidationError) -> Self {
        match validation_error {
            ValidationError::InvalidDomain { variable_id, issue } => {
                let (message, domain_info) = match issue {
                    DomainIssue::EmptyDomain => (
                        "Variable domain is empty".to_string(),
                        Some("no valid values in domain".to_string())
                    ),
                    DomainIssue::InvalidBounds { min, max } => (
                        "Variable domain bounds are invalid".to_string(),
                        Some(format!("min ({}) > max ({})", min, max))
                    ),
                    DomainIssue::FloatPrecisionIssue { interval } => (
                        "Float variable precision issue".to_string(),
                        Some(interval)
                    ),
                };
                
                SolverError::InvalidDomain {
                    message,
                    variable_name: Some(format!("var_{:?}", variable_id)),
                    domain_info,
                }
            },
            ValidationError::InvalidVariableReference { constraint_id, variable_id, constraint_type } => {
                SolverError::InvalidVariable {
                    message: format!("{} constraint references invalid variable", constraint_type),
                    variable_id: Some(format!("var_{:?}", variable_id)),
                    expected: Some(format!("constraint_{}", constraint_id)),
                }
            },
            ValidationError::ConflictingConstraints { conflict_type, variables, constraint_details } => {
                let context = match conflict_type {
                    ConflictType::DirectValueConflict => "Direct value conflict detected".to_string(),
                    ConflictType::AllDifferentDomainTooSmall => "AllDifferent domain too small".to_string(),
                    ConflictType::EmptyIntersection => "Constraints create empty solution space".to_string(),
                };
                
                SolverError::ConflictingConstraints {
                    constraint_names: None,
                    variables: Some(variables.iter().map(|id| format!("var_{:?}", id)).collect()),
                    context: Some(format!("{}: {}", context, constraint_details)),
                }
            },
            ValidationError::InvalidConstraintParameters { constraint_id, constraint_type, issue } => {
                SolverError::InvalidConstraint {
                    message: issue,
                    constraint_name: Some(format!("{}_constraint_{}", constraint_type, constraint_id)),
                    variables: None,
                }
            },
        }
    }
}