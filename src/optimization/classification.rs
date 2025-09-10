//! Problem classification for selecting optimal solving algorithms
//!
//! This module analyzes the structure of constraint satisfaction problems to determine
//! the most efficient solving approach. The classification is based on:
//! 
//! 1. **Variable types**: Pure integer, pure float, or mixed
//! 2. **Constraint patterns**: Separable vs coupled constraints across types
//! 3. **Problem complexity**: Linear bounds vs nonlinear relationships
//!
//! The classifier enables automatic algorithm selection without user intervention.

use crate::vars::{Vars, Var};
use crate::props::Propagators;

/// Classification of constraint satisfaction problems for algorithm selection
#[derive(Debug, Clone, PartialEq)]
pub enum ProblemType {
    /// All variables are floating-point - can use direct bounds optimization
    /// This enables O(1) analytical solutions for simple optimization problems
    PureFloat {
        float_var_count: usize,
        has_linear_bounds_only: bool,
    },
    
    /// All variables are integer - existing binary search works well
    /// No optimization needed, current solver is appropriate for discrete domains
    PureInteger {
        integer_var_count: usize,
    },
    
    /// Mixed integer and float variables with separable constraints
    /// Can solve float and integer parts independently, then combine
    MixedSeparable {
        integer_var_count: usize,
        float_var_count: usize,
    },
    
    /// Mixed variables with coupled constraints requiring MINLP techniques
    /// Need branch-and-bound on integers with float subproblem optimization
    MixedCoupled {
        integer_var_count: usize,
        float_var_count: usize,
        coupling_strength: CouplingStrength,
    },
}

/// Degree of coupling between integer and float variables
#[derive(Debug, Clone, PartialEq)]
pub enum CouplingStrength {
    /// Linear coupling (e.g., 2*x_int + 3*y_float <= 10)
    /// Can be handled efficiently with interval arithmetic
    Linear,
    
    /// Nonlinear coupling (e.g., x_int * y_float <= 10)  
    /// Requires more sophisticated MINLP algorithms
    Nonlinear,
    
    /// Complex coupling with multiple interaction patterns
    /// May need advanced branch-and-bound techniques
    Complex,
}

/// Problem classifier that analyzes Model structure
/// Core classifier for analyzing CSP problem characteristics
#[derive(Debug)]
pub struct ProblemClassifier;

impl ProblemClassifier {
    /// Classify a problem based on its variables and constraints
    /// 
    /// This is the main entry point for problem classification. It analyzes
    /// the variable types and constraint patterns to determine the optimal
    /// solving strategy.
    ///
    /// # Arguments
    /// * `vars` - The variables in the problem
    /// * `props` - The constraints/propagators in the problem
    ///
    /// # Returns
    /// A `ProblemType` indicating the most efficient solving approach
    pub fn classify(vars: &Vars, props: &Propagators) -> ProblemType {
        let var_analysis = Self::analyze_variables(vars);
        let constraint_analysis = Self::analyze_constraints(props, &var_analysis);
        
        Self::determine_problem_type(var_analysis, constraint_analysis)
    }
    
    /// Analyze variable types in the problem
    fn analyze_variables(vars: &Vars) -> VariableAnalysis {
        let mut integer_count = 0;
        let mut float_count = 0;
        
        // Count variables by type
        for var in vars.iter() {
            match var {
                Var::VarI(_) => integer_count += 1,
                Var::VarF(_) => float_count += 1,
            }
        }
        
        VariableAnalysis {
            integer_count,
            float_count,
            total_count: integer_count + float_count,
        }
    }
    
    /// Analyze constraint patterns to detect coupling
    fn analyze_constraints(props: &Propagators, var_analysis: &VariableAnalysis) -> ConstraintAnalysis {
        // For now, we'll implement a simple heuristic
        // TODO: In future steps, we'll analyze actual constraint types and dependencies
        
        let has_constraints = props.constraint_count() > 0;
        let appears_linear = true; // Conservative assumption for now
        
        // Determine if constraints couple different variable types
        let has_coupling = if var_analysis.integer_count > 0 && var_analysis.float_count > 0 {
            // Mixed variables - assume coupling exists if there are constraints
            // In reality, we'd analyze the constraint dependency graph
            has_constraints
        } else {
            false // Pure problems can't have cross-type coupling
        };
        
        ConstraintAnalysis {
            has_constraints,
            appears_linear,
            has_coupling,
            coupling_strength: if has_coupling {
                if appears_linear {
                    CouplingStrength::Linear
                } else {
                    CouplingStrength::Nonlinear
                }
            } else {
                CouplingStrength::Linear // Default for no coupling
            },
        }
    }
    
    /// Determine the final problem type based on analysis
    fn determine_problem_type(
        var_analysis: VariableAnalysis,
        constraint_analysis: ConstraintAnalysis,
    ) -> ProblemType {
        match (var_analysis.integer_count, var_analysis.float_count) {
            (0, 0) => {
                // No variables - this shouldn't happen in practice
                ProblemType::PureInteger { integer_var_count: 0 }
            },
            (0, float_count) => {
                // Pure float problem
                ProblemType::PureFloat {
                    float_var_count: float_count,
                    has_linear_bounds_only: constraint_analysis.appears_linear,
                }
            },
            (integer_count, 0) => {
                // Pure integer problem
                ProblemType::PureInteger { integer_var_count: integer_count }
            },
            (integer_count, float_count) => {
                // Mixed problem - check for coupling
                if constraint_analysis.has_coupling {
                    ProblemType::MixedCoupled {
                        integer_var_count: integer_count,
                        float_var_count: float_count,
                        coupling_strength: constraint_analysis.coupling_strength,
                    }
                } else {
                    ProblemType::MixedSeparable {
                        integer_var_count: integer_count,
                        float_var_count: float_count,
                    }
                }
            },
        }
    }
}

/// Analysis results for variables in the problem
#[derive(Debug, Clone)]
struct VariableAnalysis {
    integer_count: usize,
    float_count: usize,
    total_count: usize,
}

/// Analysis results for constraints in the problem
#[derive(Debug, Clone)]
struct ConstraintAnalysis {
    has_constraints: bool,
    appears_linear: bool,
    has_coupling: bool,
    coupling_strength: CouplingStrength,
}

impl ProblemType {
    /// Check if this problem type can benefit from efficient float optimization
    pub fn can_use_efficient_float_optimization(&self) -> bool {
        match self {
            ProblemType::PureFloat { has_linear_bounds_only: true, .. } => true,
            ProblemType::MixedSeparable { .. } => true, // Float part can be optimized
            _ => false,
        }
    }
    
    /// Check if this problem requires the existing integer search
    pub fn requires_integer_search(&self) -> bool {
        match self {
            ProblemType::PureInteger { .. } => true,
            ProblemType::MixedCoupled { .. } => true, // Branch on integers
            _ => false,
        }
    }
    
    /// Get a human-readable description of the solving strategy
    pub fn strategy_description(&self) -> &'static str {
        match self {
            ProblemType::PureFloat { has_linear_bounds_only: true, .. } => {
                "Direct bounds optimization (O(1) analytical solution)"
            },
            ProblemType::PureFloat { has_linear_bounds_only: false, .. } => {
                "Interval arithmetic with bounds consistency"
            },
            ProblemType::PureInteger { .. } => {
                "Binary search with constraint propagation (current solver)"
            },
            ProblemType::MixedSeparable { .. } => {
                "Independent optimization of float and integer parts"
            },
            ProblemType::MixedCoupled { coupling_strength: CouplingStrength::Linear, .. } => {
                "MINLP with linear coupling (branch-and-bound + interval arithmetic)"
            },
            ProblemType::MixedCoupled { coupling_strength: CouplingStrength::Nonlinear, .. } => {
                "MINLP with nonlinear coupling (advanced branch-and-bound)"
            },
            ProblemType::MixedCoupled { coupling_strength: CouplingStrength::Complex, .. } => {
                "Complex MINLP (sophisticated branch-and-bound techniques)"
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Model;
    
    fn create_test_vars() -> Vars {
        Vars::new()
    }
    
    fn create_test_props() -> Propagators {
        Propagators::default()
    }
    
    #[test]
    fn test_pure_float_classification() {
        // Create a model to properly add variables
        let mut model = Model::default();
        let _var_id = model.new_var_float(1.0, 10.0);
        
        let problem_type = ProblemClassifier::classify(model.get_vars(), model.get_props());
        
        match problem_type {
            ProblemType::PureFloat { float_var_count: 1, has_linear_bounds_only: true } => {
                // Correct classification
            },
            _ => panic!("Expected PureFloat classification, got {:?}", problem_type),
        }
        
        assert!(problem_type.can_use_efficient_float_optimization());
        assert!(!problem_type.requires_integer_search());
    }
    
    #[test]
    fn test_pure_integer_classification() {
        let mut model = Model::default();
        let _var_id = model.new_var_int(1, 10);
        
        let problem_type = ProblemClassifier::classify(model.get_vars(), model.get_props());
        
        match problem_type {
            ProblemType::PureInteger { integer_var_count: 1 } => {
                // Correct classification
            },
            _ => panic!("Expected PureInteger classification, got {:?}", problem_type),
        }
        
        assert!(!problem_type.can_use_efficient_float_optimization());
        assert!(problem_type.requires_integer_search());
    }
    
    #[test]
    fn test_mixed_separable_classification() {
        let mut model = Model::default();
        let _int_var = model.new_var_int(1, 5);
        let _float_var = model.new_var_float(1.0, 10.0);
        
        let problem_type = ProblemClassifier::classify(model.get_vars(), model.get_props());
        
        // With no constraints, should be classified as separable
        match problem_type {
            ProblemType::MixedSeparable { integer_var_count: 1, float_var_count: 1 } => {
                // Correct classification
            },
            _ => panic!("Expected MixedSeparable classification, got {:?}", problem_type),
        }
        
        assert!(problem_type.can_use_efficient_float_optimization());
        assert!(!problem_type.requires_integer_search());
    }
    
    #[test]
    fn test_strategy_descriptions() {
        let pure_float = ProblemType::PureFloat {
            float_var_count: 1,
            has_linear_bounds_only: true,
        };
        
        let description = pure_float.strategy_description();
        assert!(description.contains("O(1)"));
        assert!(description.contains("analytical"));
        
        let pure_integer = ProblemType::PureInteger { integer_var_count: 1 };
        let description = pure_integer.strategy_description();
        assert!(description.contains("Binary search"));
        assert!(description.contains("current solver"));
    }
    
    #[test]
    fn test_variable_analysis() {
        let vars = create_test_vars();
        
        // Test empty variables
        let analysis = ProblemClassifier::analyze_variables(&vars);
        assert_eq!(analysis.integer_count, 0);
        assert_eq!(analysis.float_count, 0);
        assert_eq!(analysis.total_count, 0);
        
        // Test with actual variables from a model
        let mut model = Model::default();
        let _int_var1 = model.new_var_int(1, 5);
        let _int_var2 = model.new_var_int(1, 3);
        let _float_var = model.new_var_float(1.0, 10.0);
        
        let analysis = ProblemClassifier::analyze_variables(model.get_vars());
        assert_eq!(analysis.integer_count, 2);
        assert_eq!(analysis.float_count, 1);
        assert_eq!(analysis.total_count, 3);
    }
}
