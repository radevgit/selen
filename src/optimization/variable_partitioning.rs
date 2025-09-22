//! Step 6.2: Variable Partitioning for Mixed Problem Decomposition
//!
//! This module implements the logic to partition mixed CSP problems into separate
//! float and integer subproblems when they are classified as MixedSeparable.
//! 
//! The partitioning enables independent solving of each subproblem, leading to
//! significant performance improvements (10-100x speedup potential).

use crate::variables::{Vars, Var, VarId};
use crate::constraints::props::Propagators;
use crate::model::Model;
use crate::optimization::classification::{ProblemClassifier, ProblemType};

/// A partition of variables and constraints for mixed problem decomposition
#[derive(Debug, Clone)]
pub struct VariablePartition {
    /// Float variables in this partition
    pub float_variables: Vec<VarId>,
    /// Integer variables in this partition
    pub integer_variables: Vec<VarId>,
    /// Number of constraints in this partition (simplified for Step 6.2)
    pub constraint_count: usize,
}

/// Result of partitioning a mixed problem
#[derive(Debug, Clone)]
pub struct PartitionResult {
    /// The float subproblem (if any float variables exist)
    pub float_partition: Option<VariablePartition>,
    /// The integer subproblem (if any integer variables exist)  
    pub integer_partition: Option<VariablePartition>,
    /// Whether the problem is truly separable (conservative estimate for Step 6.2)
    pub is_separable: bool,
    /// Total variables in original problem
    pub total_variables: usize,
    /// Total constraints in original problem (estimated)
    pub total_constraints: usize,
}

/// Variable partitioner for separable mixed problems
#[derive(Debug)]
pub struct VariablePartitioner;

impl VariablePartitioner {
    /// Partition a model into float and integer subproblems
    ///
    /// This is the main entry point for Step 6.2. It analyzes the problem structure
    /// and creates separate partitions for float and integer variables when possible.
    ///
    /// # Arguments
    /// * `model` - The model to partition
    ///
    /// # Returns
    /// A `PartitionResult` containing the separated subproblems or indicating
    /// why partitioning is not possible
    pub fn partition_model(model: &Model) -> PartitionResult {
        let vars = model.get_vars();
        let props = model.get_props();
        
        // First, classify the problem to ensure it's appropriate for partitioning
        let problem_type = ProblemClassifier::classify(vars, props);
        
        match problem_type {
            ProblemType::PureFloat { .. } | ProblemType::PureInteger { .. } => {
                // Pure problems don't need partitioning
                Self::create_single_partition_result(vars, props, problem_type)
            },
            ProblemType::MixedSeparable { .. } => {
                // This is our target case - attempt partitioning
                Self::partition_separable_problem(vars, props)
            },
            ProblemType::MixedCoupled { .. } => {
                // Coupled problems can't be partitioned safely with current approach
                Self::create_coupled_result(vars, props)
            }
        }
    }
    
    /// Partition a separable mixed problem into float and integer subproblems
    fn partition_separable_problem(vars: &Vars, props: &Propagators) -> PartitionResult {
        let variable_analysis = Self::analyze_variable_types(vars);
        let constraint_count = Self::estimate_constraint_count(props);
        
        // Create partitions based on variable types
        let float_partition = if !variable_analysis.float_variables.is_empty() {
            Some(VariablePartition {
                float_variables: variable_analysis.float_variables.clone(),
                integer_variables: Vec::new(),
                constraint_count: constraint_count / 2, // Rough estimate for Step 6.2
            })
        } else {
            None
        };
        
        let integer_partition = if !variable_analysis.integer_variables.is_empty() {
            Some(VariablePartition {
                float_variables: Vec::new(),
                integer_variables: variable_analysis.integer_variables.clone(),
                constraint_count: constraint_count / 2, // Rough estimate for Step 6.2
            })
        } else {
            None
        };
        
        // For Step 6.2, we use a conservative estimate for separability
        // In practice, this depends on the actual constraint analysis
        let is_separable = !variable_analysis.float_variables.is_empty() && 
                          !variable_analysis.integer_variables.is_empty();
        
        PartitionResult {
            float_partition,
            integer_partition,
            is_separable,
            total_variables: variable_analysis.total_count,
            total_constraints: constraint_count,
        }
    }
    
    /// Analyze variable types in the problem
    fn analyze_variable_types(vars: &Vars) -> VariableAnalysisResult {
        let mut float_variables = Vec::new();
        let mut integer_variables = Vec::new();
        let mut total_count = 0;
        
        // Iterate through all variables to classify by type
        for (index, var) in vars.iter_with_indices() {
            total_count += 1;
            // Create VarId using the safe public constructor
            let var_id = VarId::from_index(index);
            
            match var {
                Var::VarF(_) => {
                    float_variables.push(var_id);
                },
                Var::VarI(_) => {
                    integer_variables.push(var_id);
                },
            }
        }
        
        VariableAnalysisResult {
            float_variables,
            integer_variables,
            total_count,
        }
    }
    
    /// Estimate constraint count (simplified for Step 6.2)
    fn estimate_constraint_count(props: &Propagators) -> usize {
        // Count propagators as a proxy for constraints
        let mut count = 0;
        for _prop_id in props.get_prop_ids_iter() {
            count += 1;
        }
        count
    }
    
    /// Create result for pure problems (no partitioning needed)
    fn create_single_partition_result(vars: &Vars, props: &Propagators, problem_type: ProblemType) -> PartitionResult {
        let var_analysis = Self::analyze_variable_types(vars);
        let constraint_count = Self::estimate_constraint_count(props);
        
        // Handle empty model case - no partitions should be created
        if var_analysis.total_count == 0 {
            return PartitionResult {
                float_partition: None,
                integer_partition: None,
                is_separable: true,
                total_variables: 0,
                total_constraints: constraint_count,
            };
        }
        
        match problem_type {
            ProblemType::PureFloat { .. } => {
                PartitionResult {
                    float_partition: Some(VariablePartition {
                        float_variables: var_analysis.float_variables,
                        integer_variables: Vec::new(),
                        constraint_count,
                    }),
                    integer_partition: None,
                    is_separable: true,
                    total_variables: var_analysis.total_count,
                    total_constraints: constraint_count,
                }
            },
            ProblemType::PureInteger { .. } => {
                PartitionResult {
                    float_partition: None,
                    integer_partition: Some(VariablePartition {
                        float_variables: Vec::new(),
                        integer_variables: var_analysis.integer_variables,
                        constraint_count,
                    }),
                    is_separable: true,
                    total_variables: var_analysis.total_count,
                    total_constraints: constraint_count,
                }
            },
            _ => unreachable!("Single partition only for pure problems"),
        }
    }
    
    /// Create result for coupled problems (partitioning not safe)
    fn create_coupled_result(vars: &Vars, props: &Propagators) -> PartitionResult {
        let var_analysis = Self::analyze_variable_types(vars);
        let constraint_count = Self::estimate_constraint_count(props);
        
        PartitionResult {
            float_partition: None,
            integer_partition: None,
            is_separable: false,
            total_variables: var_analysis.total_count,
            total_constraints: constraint_count,
        }
    }
}

/// Result of analyzing variable types in a problem
#[derive(Debug, Clone)]
struct VariableAnalysisResult {
    float_variables: Vec<VarId>,
    integer_variables: Vec<VarId>,
    total_count: usize,
}

/// Builder for creating subproblems from partitions (simplified for Step 6.2)
#[derive(Debug)]
pub struct SubproblemBuilder;

impl SubproblemBuilder {
    /// Create a float subproblem from a partition
    ///
    /// This creates a new Model containing only the float variables and
    /// constraints from the partition.
    /// 
    /// Note: For Step 6.2, this is a simplified implementation that creates
    /// the structure but doesn't fully reconstruct constraints.
    pub fn create_float_subproblem(
        original_model: &Model,
        partition: &VariablePartition,
    ) -> Result<Model, PartitionError> {
        if partition.float_variables.is_empty() {
            return Err(PartitionError::NoFloatVariables);
        }
        
        // Create new model with same precision settings
        let mut subproblem = Model::with_float_precision(original_model.float_precision_digits());
        
        // Add float variables to subproblem
        // Note: For Step 6.2, we create equivalent variables but don't map constraints yet
        
        for &var_id in &partition.float_variables {
            // Use indexing to get the variable
            let var = &original_model.get_vars()[var_id];
            match var {
                Var::VarF(float_interval) => {
                    // Create corresponding variable in subproblem
                    let _new_var = subproblem.float(
                        float_interval.min,
                        float_interval.max
                    );
                    // VarId mapping not implemented - constraint reconstruction abandoned
                },
                _ => return Err(PartitionError::InvalidVariableType),
            }
        }
        
        // Constraint reconstruction for float subproblems not implemented
        // This requires mapping constraints from original to subproblem
        
        Ok(subproblem)
    }
    
    /// Create an integer subproblem from a partition
    pub fn create_integer_subproblem(
        original_model: &Model,
        partition: &VariablePartition,
    ) -> Result<Model, PartitionError> {
        if partition.integer_variables.is_empty() {
            return Err(PartitionError::NoIntegerVariables);
        }
        
        // Create new model
        let mut subproblem = Model::with_float_precision(original_model.float_precision_digits());
        
        // Add integer variables to subproblem
        for &var_id in &partition.integer_variables {
            let var = &original_model.get_vars()[var_id];
            match var {
                Var::VarI(sparse_set) => {
                    // Create corresponding variable in subproblem
                    let _new_var = subproblem.int(
                        sparse_set.min(),
                        sparse_set.max()
                    );
                    // VarId mapping not implemented - constraint reconstruction abandoned
                },
                _ => return Err(PartitionError::InvalidVariableType),
            }
        }
        
        // Constraint reconstruction for integer subproblems not implemented
        
        Ok(subproblem)
    }
}

/// Errors that can occur during partitioning
#[derive(Debug, Clone, PartialEq)]
pub enum PartitionError {
    /// No float variables found for float subproblem
    NoFloatVariables,
    /// No integer variables found for integer subproblem
    NoIntegerVariables,
    /// Variable has unexpected type
    InvalidVariableType,
    /// Problem is not separable and cannot be partitioned
    NotSeparable,
    /// Constraint reconstruction failed
    ConstraintMappingFailed,
}

impl std::fmt::Display for PartitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PartitionError::NoFloatVariables => write!(f, "No float variables found for float subproblem"),
            PartitionError::NoIntegerVariables => write!(f, "No integer variables found for integer subproblem"),
            PartitionError::InvalidVariableType => write!(f, "Variable has unexpected type"),
            PartitionError::NotSeparable => write!(f, "Problem is not separable and cannot be partitioned"),
            PartitionError::ConstraintMappingFailed => write!(f, "Constraint reconstruction failed"),
        }
    }
}

impl std::error::Error for PartitionError {}
