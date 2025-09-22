//! Constraint Metadata Collection
//! 
//! This module provides infrastructure for collecting and storing metadata about
//! constraints as they are created. This enables optimization systems to introspect
//! constraint patterns and extract constraint values for precision-aware optimization.

use crate::variables::{VarId, Val};
use crate::variables::views::View;
use std::collections::HashMap;

/// Unique identifier for a constraint
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConstraintId(pub usize);

/// Metadata about a constraint
#[derive(Debug, Clone, PartialEq)]
pub struct ConstraintMetadata {
    /// Type of constraint
    pub constraint_type: ConstraintType,
    /// Variables involved in this constraint
    pub variables: Vec<VarId>,
    /// Additional constraint-specific data
    pub data: ConstraintData,
}

/// Types of constraints we can track
#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintType {
    /// x <= y constraint
    LessThanOrEquals,
    /// x < y constraint (implemented as x.next() <= y)
    LessThan,
    /// x >= y constraint
    GreaterThanOrEquals,
    /// x > y constraint
    GreaterThan,
    /// x == y constraint
    Equals,
    /// x != y constraint
    NotEquals,
    /// AllDifferent constraint
    AllDifferent,
    /// AllEqual constraint
    AllEqual,
    /// Element constraint (array\[index\] = value)
    Element,
    /// Sum constraint
    Sum,
    /// Addition constraint (x + y = z)
    Addition,
    /// Multiplication constraint (x * y = z)
    Multiplication,
    /// Modulo constraint (x % y = z)
    Modulo,
    /// Division constraint (x / y = z)
    Division,
    /// Absolute value constraint (|x| = z)
    AbsoluteValue,
    /// Minimum constraint (min(x, y, ...) = z)
    Minimum,
    /// Maximum constraint (max(x, y, ...) = z)
    Maximum,
    /// Boolean AND constraint (result = a AND b AND ...)
    BooleanAnd,
    /// Boolean OR constraint (result = a OR b OR ...)
    BooleanOr,
    /// Boolean NOT constraint (result = NOT operand)
    BooleanNot,
    /// Count constraint (count(vars, value) = count_var)
    Count,
    /// Table constraint (table(vars, tuples))
    Table,
    /// Between constraint (lower <= middle <= upper)
    Between,
    /// At least N constraint (at_least(vars, value, count))
    AtLeast,
    /// At most N constraint (at_most(vars, value, count))
    AtMost,
    /// Exactly N constraint (exactly(vars, value, count))
    Exactly,
    /// If-then-else constraint (if condition then constraint1 else constraint2)
    IfThenElse,
    /// Complex constraint that couldn't be categorized
    Complex {
        /// Number of variables involved
        variable_count: usize,
        /// Whether this constraint is linear
        is_linear: bool,
        /// Whether this constraint involves only binary operations
        is_binary: bool,
    },
}

/// Constraint-specific data
#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintData {
    /// Binary constraint with two operands
    Binary {
        left: ViewInfo,
        right: ViewInfo,
    },
    /// Unary constraint with one operand
    Unary {
        operand: ViewInfo,
    },
    /// N-ary constraint with multiple operands
    NAry {
        operands: Vec<ViewInfo>,
    },
    /// No additional data
    None,
}

/// Information about a view in a constraint
#[derive(Debug, Clone, PartialEq)]
pub enum ViewInfo {
    /// Direct variable reference
    Variable { var_id: VarId },
    /// Constant value
    Constant { value: ConstraintValue },
    /// Transformed view (e.g., x.next(), x.prev())
    Transformed { 
        base_var: VarId,
        transformation: TransformationType,
    },
    /// Complex view that couldn't be analyzed
    Complex,
}

/// Types of view transformations
#[derive(Debug, Clone, PartialEq)]
pub enum TransformationType {
    /// Next value transformation (x.next())
    Next,
    /// Previous value transformation (x.prev())
    Previous,
    /// Negation transformation (-x)
    Negation,
    /// Absolute value transformation (|x|)
    Absolute,
    /// Scale transformation (k * x)
    Scale(f64),
    /// Offset transformation (x + k)
    Offset(f64),
}

/// Values that can appear in constraints
#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintValue {
    Integer(i32),
    Float(f64),
}

impl From<Val> for ConstraintValue {
    fn from(val: Val) -> Self {
        match val {
            Val::ValI(i) => ConstraintValue::Integer(i),
            Val::ValF(f) => ConstraintValue::Float(f),
        }
    }
}

/// Registry for constraint metadata
#[derive(Debug, Clone, Default)]
pub struct ConstraintRegistry {
    /// Metadata for each constraint
    constraints: HashMap<ConstraintId, ConstraintMetadata>,
    /// Next available constraint ID
    next_id: usize,
    /// Index from variable to constraints that affect it
    var_to_constraints: HashMap<VarId, Vec<ConstraintId>>,
}

impl ConstraintRegistry {
    /// Create a new constraint registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a new constraint and return its ID
    pub fn register_constraint(
        &mut self,
        constraint_type: ConstraintType,
        variables: Vec<VarId>,
        data: ConstraintData,
    ) -> ConstraintId {
        let id = ConstraintId(self.next_id);
        self.next_id += 1;

        let metadata = ConstraintMetadata {
            constraint_type,
            variables: variables.clone(),
            data,
        };

        self.constraints.insert(id, metadata);

        // Update variable to constraint mapping
        for var_id in variables {
            self.var_to_constraints
                .entry(var_id)
                .or_default()
                .push(id);
        }

        id
    }

    /// Get metadata for a constraint
    pub fn get_constraint(&self, id: ConstraintId) -> Option<&ConstraintMetadata> {
        self.constraints.get(&id)
    }

    /// Get all constraints affecting a variable
    pub fn get_constraints_for_variable(&self, var_id: VarId) -> Vec<ConstraintId> {
        self.var_to_constraints
            .get(&var_id)
            .cloned()
            .unwrap_or_default()
    }

    /// Get all constraints of a specific type
    pub fn get_constraints_by_type(&self, constraint_type: &ConstraintType) -> Vec<ConstraintId> {
        self.constraints
            .iter()
            .filter(|(_, metadata)| &metadata.constraint_type == constraint_type)
            .map(|(id, _)| *id)
            .collect()
    }

    /// Get all constraint IDs registered in the system
    pub fn get_all_constraint_ids(&self) -> Vec<ConstraintId> {
        self.constraints.keys().cloned().collect()
    }

    /// Analyze constraints for a variable to extract simple patterns
    pub fn analyze_variable_constraints(&self, var_id: VarId) -> VariableConstraintAnalysis {
        let constraint_ids = self.get_constraints_for_variable(var_id);
        let mut analysis = VariableConstraintAnalysis::default();

        for constraint_id in constraint_ids {
            if let Some(metadata) = self.get_constraint(constraint_id) {
                self.analyze_constraint_for_variable(var_id, metadata, &mut analysis);
            }
        }

        analysis
    }

    /// Analyze a single constraint's effect on a variable
    fn analyze_constraint_for_variable(
        &self,
        var_id: VarId,
        metadata: &ConstraintMetadata,
        analysis: &mut VariableConstraintAnalysis,
    ) {
        if let ConstraintData::Binary { left, right } = &metadata.data {
            match &metadata.constraint_type {
                ConstraintType::LessThanOrEquals => {
                    if let (ViewInfo::Variable { var_id: left_var }, ViewInfo::Constant { value }) = (left, right) {
                        if *left_var == var_id {
                            match value {
                                ConstraintValue::Float(f) => analysis.upper_bounds.push(*f),
                                ConstraintValue::Integer(i) => analysis.upper_bounds.push(*i as f64),
                            }
                        }
                    }
                    if let (ViewInfo::Constant { value }, ViewInfo::Variable { var_id: right_var }) = (left, right) {
                        if *right_var == var_id {
                            match value {
                                ConstraintValue::Float(f) => analysis.lower_bounds.push(*f),
                                ConstraintValue::Integer(i) => analysis.lower_bounds.push(*i as f64),
                            }
                        }
                    }
                }
                ConstraintType::LessThan => {
                    // x < y can be implemented as x.next() <= y or direct comparison
                    // Handle x.next() <= constant pattern (most common)
                    if let (ViewInfo::Transformed { base_var, transformation }, ViewInfo::Constant { value }) = (left, right) {
                        if *base_var == var_id && *transformation == TransformationType::Next {
                            match value {
                                ConstraintValue::Float(f) => {
                                    // x.next() <= f means x < f (strict upper bound)
                                    analysis.strict_upper_bounds.push(*f);
                                }
                                ConstraintValue::Integer(i) => {
                                    analysis.strict_upper_bounds.push(*i as f64);
                                }
                            }
                        }
                    }
                    // Handle direct x < constant pattern
                    if let (ViewInfo::Variable { var_id: left_var }, ViewInfo::Constant { value }) = (left, right) {
                        if *left_var == var_id {
                            match value {
                                ConstraintValue::Float(f) => {
                                    analysis.strict_upper_bounds.push(*f);
                                }
                                ConstraintValue::Integer(i) => {
                                    analysis.strict_upper_bounds.push(*i as f64);
                                }
                            }
                        }
                    }
                    // Handle constant < x pattern (uncommon but possible)
                    if let (ViewInfo::Constant { value }, ViewInfo::Variable { var_id: right_var }) = (left, right) {
                        if *right_var == var_id {
                            match value {
                                ConstraintValue::Float(f) => {
                                    analysis.strict_lower_bounds.push(*f);
                                }
                                ConstraintValue::Integer(i) => {
                                    analysis.strict_lower_bounds.push(*i as f64);
                                }
                            }
                        }
                    }
                }
                ConstraintType::GreaterThanOrEquals => {
                    if let (ViewInfo::Variable { var_id: left_var }, ViewInfo::Constant { value }) = (left, right) {
                        if *left_var == var_id {
                            match value {
                                ConstraintValue::Float(f) => analysis.lower_bounds.push(*f),
                                ConstraintValue::Integer(i) => analysis.lower_bounds.push(*i as f64),
                            }
                        }
                    }
                    if let (ViewInfo::Constant { value }, ViewInfo::Variable { var_id: right_var }) = (left, right) {
                        if *right_var == var_id {
                            match value {
                                ConstraintValue::Float(f) => analysis.upper_bounds.push(*f),
                                ConstraintValue::Integer(i) => analysis.upper_bounds.push(*i as f64),
                            }
                        }
                    }
                }
                ConstraintType::GreaterThan => {
                    // Handle x > y patterns
                    if let (ViewInfo::Variable { var_id: left_var }, ViewInfo::Constant { value }) = (left, right) {
                        if *left_var == var_id {
                            // x > constant
                            let bound_value = match value {
                                ConstraintValue::Float(f) => *f,
                                ConstraintValue::Integer(i) => *i as f64,
                            };
                            analysis.strict_lower_bounds.push(bound_value);
                        }
                    } else if let (ViewInfo::Constant { value }, ViewInfo::Variable { var_id: right_var }) = (left, right) {
                        if *right_var == var_id {
                            // constant > x  =>  x < constant
                            let bound_value = match value {
                                ConstraintValue::Float(f) => *f,
                                ConstraintValue::Integer(i) => *i as f64,
                            };
                            analysis.strict_upper_bounds.push(bound_value);
                        }
                    } else {
                        // Complex patterns involving multiple variables or transformations
                        analysis.has_complex_constraints = true;
                    }
                }
                ConstraintType::Equals => {
                    if let (ViewInfo::Variable { var_id: left_var }, ViewInfo::Constant { value }) = (left, right) {
                        if *left_var == var_id {
                            match value {
                                ConstraintValue::Float(f) => analysis.equality_values.push(*f),
                                ConstraintValue::Integer(i) => analysis.equality_values.push(*i as f64),
                            }
                        }
                    }
                }
                ConstraintType::NotEquals => {
                    // Not equals constraints don't directly provide bounds,
                    // but they do make the problem more complex
                    analysis.has_complex_constraints = true;
                }
                ConstraintType::Complex { .. } | 
                ConstraintType::Addition | 
                ConstraintType::Multiplication | 
                ConstraintType::Modulo |
                ConstraintType::Division |
                ConstraintType::AbsoluteValue |
                ConstraintType::Minimum |
                ConstraintType::Maximum |
                ConstraintType::BooleanAnd |
                ConstraintType::BooleanOr |
                ConstraintType::BooleanNot |
                ConstraintType::AllDifferent | 
                ConstraintType::AllEqual |
                ConstraintType::Element |
                ConstraintType::Count |
                ConstraintType::Table |
                ConstraintType::Between |
                ConstraintType::AtLeast |
                ConstraintType::AtMost |
                ConstraintType::Exactly |
                ConstraintType::IfThenElse |
                ConstraintType::Sum => {
                    analysis.has_complex_constraints = true;
                }
            }
        } else {
            analysis.has_complex_constraints = true;
        }
    }

    /// Get total number of registered constraints
    pub fn constraint_count(&self) -> usize {
        self.constraints.len()
    }
}

/// Analysis result for constraints affecting a single variable
#[derive(Debug, Clone, Default)]
pub struct VariableConstraintAnalysis {
    /// Upper bounds from <= constraints
    pub upper_bounds: Vec<f64>,
    /// Lower bounds from >= constraints  
    pub lower_bounds: Vec<f64>,
    /// Strict upper bounds from < constraints
    pub strict_upper_bounds: Vec<f64>,
    /// Strict lower bounds from > constraints
    pub strict_lower_bounds: Vec<f64>,
    /// Equality values from = constraints
    pub equality_values: Vec<f64>,
    /// Whether variable has complex constraints that couldn't be analyzed
    pub has_complex_constraints: bool,
}

impl VariableConstraintAnalysis {
    /// Get the effective upper bound considering all constraints
    pub fn get_effective_upper_bound(&self, _step_size: f64) -> Option<f64> {
        let mut min_bound: Option<f64> = None;

        // Consider <= constraints
        for &bound in &self.upper_bounds {
            min_bound = Some(min_bound.map_or(bound, |current| current.min(bound)));
        }

        // Consider < constraints (use ULP-based precision instead of step_size)
        for &bound in &self.strict_upper_bounds {
            let strict_bound = crate::optimization::ulp_utils::UlpUtils::strict_upper_bound(bound);
            min_bound = Some(min_bound.map_or(strict_bound, |current| current.min(strict_bound)));
        }

        // Consider equality constraints
        for &value in &self.equality_values {
            min_bound = Some(min_bound.map_or(value, |current| current.min(value)));
        }

        min_bound
    }

    /// Get the effective lower bound considering all constraints
    pub fn get_effective_lower_bound(&self, _step_size: f64) -> Option<f64> {
        let mut max_bound: Option<f64> = None;

        // Consider >= constraints
        for &bound in &self.lower_bounds {
            max_bound = Some(max_bound.map_or(bound, |current| current.max(bound)));
        }

        // Consider > constraints (use ULP-based precision instead of step_size)
        for &bound in &self.strict_lower_bounds {
            let strict_bound = crate::optimization::ulp_utils::UlpUtils::strict_lower_bound(bound);
            max_bound = Some(max_bound.map_or(strict_bound, |current| current.max(strict_bound)));
        }

        // Consider equality constraints
        for &value in &self.equality_values {
            max_bound = Some(max_bound.map_or(value, |current| current.max(value)));
        }

        max_bound
    }

    /// Check if variable has simple constraint pattern that can be optimized
    pub fn is_simple_pattern(&self) -> bool {
        !self.has_complex_constraints && 
        self.equality_values.is_empty() && // No equality constraints
        (self.upper_bounds.len() + self.strict_upper_bounds.len()) <= 1 && // At most one upper bound
        (self.lower_bounds.len() + self.strict_lower_bounds.len()) <= 1    // At most one lower bound
    }
}

/// Utility function to analyze a view and extract constraint information
pub fn analyze_view<T: View>(view: &T) -> ViewInfo {
    if let Some(var_id) = view.get_underlying_var() {
        // Check if this is a transformed view
        // For now, we'll use a simple heuristic - more sophisticated analysis would be needed
        // to detect transformations like .next(), .prev(), etc.
        ViewInfo::Variable { var_id }
    } else {
        // Check if this is a TypedConstant (by attempting downcasting via Any trait)
        // Since we don't have access to Any trait here, we'll use a different approach:
        // Try to evaluate the view with a dummy context to see if it's constant
        // For now, mark as Complex - this could be improved with type checking
        ViewInfo::Complex
    }
}

/// Extract constant value from a view if possible
pub fn extract_constant_value<T: View>(_view: &T) -> Option<ConstraintValue> {
    // This is a placeholder - a full implementation would need access to
    // the view's evaluation context to determine if it's constant
    None
}
