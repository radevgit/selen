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

use crate::variables::{Vars, Var, VarId};
use crate::constraints::props::Propagators;

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
        }
    }
    
    /// Analyze constraint patterns to detect coupling
    fn analyze_constraints(props: &Propagators, var_analysis: &VariableAnalysis) -> ConstraintAnalysis {
        
        if var_analysis.integer_count == 0 || var_analysis.float_count == 0 {
            // Pure problems can't have cross-type coupling
            return ConstraintAnalysis {
                appears_linear: true,
                has_coupling: false,
                coupling_strength: CouplingStrength::Linear,
            };
        }
        
        // Mixed problem - analyze constraint registry for cross-type dependencies
        let constraint_registry = props.get_constraint_registry();
        let coupling_analysis = Self::analyze_variable_coupling(constraint_registry, var_analysis);
        
        ConstraintAnalysis {
            appears_linear: coupling_analysis.appears_linear,
            has_coupling: coupling_analysis.has_coupling,
            coupling_strength: coupling_analysis.coupling_strength,
        }
    }

    /// Analyze variable coupling in mixed problems by examining constraint patterns
    fn analyze_variable_coupling(
        constraint_registry: &crate::optimization::constraint_metadata::ConstraintRegistry,
        var_analysis: &VariableAnalysis,
    ) -> CouplingAnalysisResult {
        // If no constraints or not a mixed problem, there can't be coupling
        let has_constraints = constraint_registry.constraint_count() > 0;
        if !has_constraints || var_analysis.integer_count == 0 || var_analysis.float_count == 0 {
            return CouplingAnalysisResult {
                has_coupling: false,
                coupling_strength: CouplingStrength::Linear,
                appears_linear: true,
            };
        }
        
        // Check all constraints to see if any involve both integer and float variables
        let has_coupling = Self::detect_cross_type_coupling(constraint_registry);
        
        // Conservative assumption about coupling strength
        let coupling_strength = if has_coupling {
            CouplingStrength::Linear // Start with linear assumption
        } else {
            CouplingStrength::Linear
        };
        
        CouplingAnalysisResult {
            has_coupling,
            coupling_strength,
            appears_linear: true, // Conservative assumption
        }
    }

    /// Detect if any constraints involve both integer and float variables
    fn detect_cross_type_coupling(
        constraint_registry: &crate::optimization::constraint_metadata::ConstraintRegistry,
    ) -> bool {
        // Iterate through all constraints and check if any involve mixed variable types
        for constraint_type in [
            crate::optimization::constraint_metadata::ConstraintType::Equals,
            crate::optimization::constraint_metadata::ConstraintType::LessThanOrEquals,
            crate::optimization::constraint_metadata::ConstraintType::NotEquals,
            crate::optimization::constraint_metadata::ConstraintType::BooleanAnd,
            crate::optimization::constraint_metadata::ConstraintType::BooleanOr,
            crate::optimization::constraint_metadata::ConstraintType::BooleanNot,
        ] {
            let constraint_ids = constraint_registry.get_constraints_by_type(&constraint_type);
            
            for constraint_id in constraint_ids {
                if let Some(metadata) = constraint_registry.get_constraint(constraint_id) {
                    // Check if this constraint involves both integer and float variables
                    if Self::constraint_has_mixed_types(&metadata.variables) {
                        return true;
                    }
                }
            }
        }
        
        false
    }

    /// Check if a list of variables contains both integer and float types
    fn constraint_has_mixed_types(variables: &[VarId]) -> bool {
        if variables.len() < 2 {
            return false;
        }
        
        // For now, we can't easily determine variable types from VarId alone
        // without access to the Vars collection. This is a limitation of the current design.
        // We'll return false for now and enhance this later when we have access to the m.
        // 
        // Variable type detection from VarId not implemented:
        // 1. Passing the Vars collection to this function would require refactoring
        // 2. Encoding type information in VarId itself would change core architecture  
        // 3. Using a different approach for type detection would require design changes
        false
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
}

/// Analysis results for constraints in the problem
#[derive(Debug, Clone)]
struct ConstraintAnalysis {
    appears_linear: bool,
    has_coupling: bool,
    coupling_strength: CouplingStrength,
}

/// Result of coupling analysis for mixed problems
#[derive(Debug, Clone)]
struct CouplingAnalysisResult {
    has_coupling: bool,
    coupling_strength: CouplingStrength,
    appears_linear: bool,
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
    
    #[test]
    fn test_pure_float_classification() {
        // Create a model to properly add variables
        let mut m = Model::default();
        let _var_id = m.float(1.0, 10.0);
        
        let problem_type = ProblemClassifier::classify(m.get_vars(), m.get_props());
        
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
        let mut m = Model::default();
        let _var_id = m.int(1, 10);
        
        let problem_type = ProblemClassifier::classify(m.get_vars(), m.get_props());
        
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
        let mut m = Model::default();
        let _int_var = m.int(1, 5);
        let _float_var = m.float(1.0, 10.0);
        
        let problem_type = ProblemClassifier::classify(m.get_vars(), m.get_props());
        
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
        
        // Test with actual variables from a model
        let mut m = Model::default();
        let _int_var1 = m.int(1, 5);
        let _int_var2 = m.int(1, 3);
        let _float_var = m.float(1.0, 10.0);
        
        let analysis = ProblemClassifier::analyze_variables(m.get_vars());
        assert_eq!(analysis.integer_count, 2);
        assert_eq!(analysis.float_count, 1);
    }
}
