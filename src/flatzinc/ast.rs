//! Abstract Syntax Tree (AST) for FlatZinc
//!
//! Represents the parsed structure of a FlatZinc model.

use crate::flatzinc::tokenizer::Location;

/// A complete FlatZinc model
#[derive(Debug, Clone)]
pub struct FlatZincModel {
    pub predicates: Vec<PredicateDecl>,
    pub var_decls: Vec<VarDecl>,
    pub constraints: Vec<Constraint>,
    pub solve_goal: SolveGoal,
}

/// Predicate declaration
#[derive(Debug, Clone)]
pub struct PredicateDecl {
    pub name: String,
    pub params: Vec<PredParam>,
    pub location: Location,
}

/// Predicate parameter
#[derive(Debug, Clone)]
pub struct PredParam {
    pub param_type: Type,
    pub name: String,
}

/// Variable declaration
#[derive(Debug, Clone)]
pub struct VarDecl {
    pub var_type: Type,
    pub name: String,
    pub annotations: Vec<Annotation>,
    pub init_value: Option<Expr>,
    pub location: Location,
}

/// Type in FlatZinc
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// Basic types
    Bool,
    Int,
    Float,
    
    /// Integer range: int_min..int_max
    IntRange(i64, i64),
    
    /// Integer set: {1, 2, 3}
    IntSet(Vec<i64>),
    
    /// Float range: float_min..float_max
    FloatRange(f64, f64),
    
    /// Set of int
    SetOfInt,
    
    /// Set with specific domain
    SetRange(i64, i64),
    
    /// Array type: array[index_set] of element_type
    Array {
        index_sets: Vec<IndexSet>,
        element_type: Box<Type>,
    },
    
    /// Variable type (var before the actual type)
    Var(Box<Type>),
}

/// Index set for arrays
#[derive(Debug, Clone, PartialEq)]
pub enum IndexSet {
    /// 1..n
    Range(i64, i64),
    
    /// Explicit set
    Set(Vec<i64>),
}

/// Constraint statement
#[derive(Debug, Clone)]
pub struct Constraint {
    pub predicate: String,
    pub args: Vec<Expr>,
    pub annotations: Vec<Annotation>,
    pub location: Location,
}

/// Solve goal
#[derive(Debug, Clone)]
pub enum SolveGoal {
    Satisfy {
        annotations: Vec<Annotation>,
    },
    Minimize {
        objective: Expr,
        annotations: Vec<Annotation>,
    },
    Maximize {
        objective: Expr,
        annotations: Vec<Annotation>,
    },
}

/// Expression
#[derive(Debug, Clone)]
pub enum Expr {
    /// Boolean literal
    BoolLit(bool),
    
    /// Integer literal
    IntLit(i64),
    
    /// Float literal
    FloatLit(f64),
    
    /// String literal
    StringLit(String),
    
    /// Identifier (variable reference)
    Ident(String),
    
    /// Array literal: [1, 2, 3]
    ArrayLit(Vec<Expr>),
    
    /// Set literal: {1, 2, 3}
    SetLit(Vec<Expr>),
    
    /// Integer range: 1..10
    Range(Box<Expr>, Box<Expr>),
    
    /// Array access: arr[idx]
    ArrayAccess {
        array: Box<Expr>,
        index: Box<Expr>,
    },
}

/// Annotation (e.g., :: output_var)
#[derive(Debug, Clone)]
pub struct Annotation {
    pub name: String,
    pub args: Vec<Expr>,
}

impl FlatZincModel {
    pub fn new() -> Self {
        FlatZincModel {
            predicates: Vec::new(),
            var_decls: Vec::new(),
            constraints: Vec::new(),
            solve_goal: SolveGoal::Satisfy {
                annotations: Vec::new(),
            },
        }
    }
}

impl Default for FlatZincModel {
    fn default() -> Self {
        Self::new()
    }
}
