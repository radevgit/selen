// Design proposal for efficient float constraint solving
// Based on well-known techniques: Bounds Consistency + Direct Optimization
// EXTENSION: Mixed Integer-Float Problems (MINLP)

use selen::prelude::*;

/// Efficient solver strategy classifier for floating-point and mixed problems
#[derive(Debug, Clone)]
pub enum MixedProblemType {
    /// Pure float optimization: maximize/minimize single float variable with bounds constraints
    /// Solution: Direct analytical solution - no search needed!
    /// Example: maximize x subject to x < 5.5, x ∈ [1.0, 10.0] → x* = 5.5 - ε
    PureFloatOptimization {
        objective_var: VarId,
        is_maximize: bool,
        bound_constraints: Vec<BoundConstraint>,
    },
    
    /// Pure integer problem - use existing discrete CSP algorithms  
    /// Solution: Current binary search + constraint propagation works well
    /// Example: maximize x subject to x < 10, x ∈ {1,2,3,...,15} → x* = 9
    PureIntegerProblem {
        integer_constraints: Vec<IntegerConstraint>,
    },
    
    /// Mixed problem with separable variables (no cross-constraints)
    /// Solution: Solve float and integer parts independently, then combine
    /// Example: maximize (x_int + y_float) where x ∈ {1,2,3}, y ∈ [1.0,3.0], no x*y constraints
    SeparableMixed {
        float_part: Box<MixedProblemType>,
        integer_part: Box<MixedProblemType>,
        coupling_constraints: Vec<CouplingConstraint>,
    },
    
    /// Complex mixed problem with coupled integer-float constraints
    /// Solution: Hybrid approach - fix integers, optimize floats, then branch on integers
    /// Example: x_int * y_float ≤ 10, maximize (x_int + y_float)
    /// This is the hard case that requires MINLP (Mixed Integer Nonlinear Programming) techniques
    CoupledMixed {
        integer_vars: Vec<VarId>,
        float_vars: Vec<VarId>, 
        mixed_constraints: Vec<Box<dyn MixedConstraint>>,
        objective: ObjectiveFunction,
    },
}

/// Key insight: Different constraint types require different handling in mixed problems
#[derive(Debug, Clone)]
pub struct CouplingConstraint {
    constraint_type: CouplingType,
    integer_vars: Vec<VarId>,
    float_vars: Vec<VarId>,
}

#[derive(Debug, Clone)]
pub enum CouplingType {
    /// Linear coupling: a*x_int + b*y_float ≤ c
    /// Can be handled efficiently with bounds propagation
    Linear { coeffs_int: Vec<f64>, coeffs_float: Vec<f64>, rhs: f64 },
    
    /// Bilinear coupling: x_int * y_float ≤ c  
    /// Requires MINLP techniques (harder case)
    Bilinear { pairs: Vec<(VarId, VarId)>, bounds: Vec<f64> },
    
    /// Logical coupling: if x_int = 1 then y_float ≥ 5.0
    /// Can be handled with conditional constraint activation
    Conditional { condition: IntegerCondition, consequence: FloatConstraint },
}

#[derive(Debug, Clone)]
pub struct IntegerConstraint {
    vars: Vec<VarId>,
    constraint_type: String, // Simplified for now
}

#[derive(Debug, Clone)]
pub struct IntegerCondition {
    var: VarId,
    value: i32,
}

#[derive(Debug, Clone)] 
pub struct FloatConstraint {
    var: VarId,
    bound_type: BoundType,
    value: f64,
}

#[derive(Debug, Clone)]
pub struct ObjectiveFunction {
    objective_type: ObjectiveType,
    terms: Vec<ObjectiveTerm>,
}

#[derive(Debug, Clone)]
pub enum ObjectiveType {
    Minimize,
    Maximize,
}

#[derive(Debug, Clone)]
pub struct ObjectiveTerm {
    var: VarId,
    coefficient: f64,
}

pub trait MixedConstraint: std::fmt::Debug {
    fn propagate_mixed(&self, int_domains: &mut [Vec<i32>], float_domains: &mut [FloatInterval]) -> bool;
    fn is_satisfied_mixed(&self, int_values: &[i32], float_values: &[f64]) -> bool;
}

/// Extended solver for mixed integer-float problems
/// Based on well-known MINLP (Mixed Integer Nonlinear Programming) techniques
pub struct MixedIntegerFloatSolver {
    integer_domains: Vec<Vec<i32>>,     // Discrete domains for integers
    float_domains: Vec<FloatInterval>,  // Continuous intervals for floats  
    problem_type: MixedProblemType,
}

impl MixedIntegerFloatSolver {
    /// Solve using problem-type-specific algorithms
    pub fn solve(&mut self) -> Option<(Vec<i32>, Vec<f64>)> {
        match &self.problem_type {
            MixedProblemType::PureFloatOptimization { .. } => {
                // Use the efficient float algorithm from before
                let float_solution = self.solve_pure_float_optimization()?;
                Some((vec![], float_solution))
            },
            
            MixedProblemType::PureIntegerProblem { .. } => {
                // Use existing discrete CSP solver (current approach works well)
                let int_solution = self.solve_pure_integer_problem()?;
                Some((int_solution, vec![]))
            },
            
            MixedProblemType::SeparableMixed { float_part, integer_part, coupling_constraints } => {
                // ALGORITHM: Independent optimization + coupling check
                self.solve_separable_mixed(float_part, integer_part, coupling_constraints)
            },
            
            MixedProblemType::CoupledMixed { integer_vars, float_vars, mixed_constraints, objective } => {
                // ALGORITHM: Branch-and-bound on integers + float optimization
                // This is the "well-known solution" for MINLP problems
                self.solve_coupled_mixed(integer_vars, float_vars, mixed_constraints, objective)
            },
        }
    }
    
    /// ALGORITHM 1: Pure float optimization (unchanged from before)
    fn solve_pure_float_optimization(&mut self) -> Option<Vec<f64>> {
        // Same as before - direct analytical solution
        println!("Using efficient float optimization - O(1) solution");
        Some(vec![5.4999]) // Placeholder
    }
    
    /// ALGORITHM 2: Pure integer optimization (use existing approach)
    fn solve_pure_integer_problem(&mut self) -> Option<Vec<i32>> {
        // Current binary search + constraint propagation works fine for pure integer
        println!("Using existing integer CSP solver - works well");
        Some(vec![9]) // Placeholder
    }
    
    /// ALGORITHM 3: Separable mixed problems 
    /// Well-known technique: "Decomposition methods"
    fn solve_separable_mixed(
        &mut self,
        _float_part: &MixedProblemType,
        _integer_part: &MixedProblemType, 
        _coupling_constraints: &[CouplingConstraint]
    ) -> Option<(Vec<i32>, Vec<f64>)> {
        // 1. Solve float part optimally (using efficient float algorithm)
        let float_solution = vec![5.4999]; // Optimal float solution
        
        // 2. Solve integer part optimally (using existing integer CSP)
        let int_solution = vec![9]; // Optimal integer solution
        
        // 3. Check coupling constraints
        if self.check_coupling_constraints(&int_solution, &float_solution) {
            Some((int_solution, float_solution))
        } else {
            // If coupling violated, fall back to coupled algorithm
            println!("Coupling constraints violated - falling back to coupled solver");
            None
        }
    }
    
    /// ALGORITHM 4: Coupled mixed problems  
    /// Well-known technique: "Branch-and-bound for MINLP"
    /// References: Grossmann & Kravanja (1997), Floudas (1995)
    fn solve_coupled_mixed(
        &mut self,
        integer_vars: &[VarId],
        float_vars: &[VarId],
        _mixed_constraints: &[Box<dyn MixedConstraint>],
        objective: &ObjectiveFunction
    ) -> Option<(Vec<i32>, Vec<f64>)> {
        println!("Using Branch-and-Bound MINLP algorithm");
        
        // MINLP Algorithm:
        // 1. Branch on integer variables (like current binary search)
        // 2. For each integer assignment, solve float subproblem optimally  
        // 3. Use bounds to prune integer branches early
        
        let mut best_solution = None;
        let mut best_objective_value = match objective.objective_type {
            ObjectiveType::Maximize => f64::NEG_INFINITY,
            ObjectiveType::Minimize => f64::INFINITY,
        };
        
        // Enumerate integer assignments (could be optimized with branch-and-bound)
        for int_assignment in self.enumerate_integer_assignments(integer_vars) {
            // Fix integers, solve float subproblem optimally
            if let Some(float_assignment) = self.solve_float_subproblem(&int_assignment, float_vars) {
                let objective_value = self.evaluate_objective(&int_assignment, &float_assignment, objective);
                
                // Update best solution if better
                let is_better = match objective.objective_type {
                    ObjectiveType::Maximize => objective_value > best_objective_value,
                    ObjectiveType::Minimize => objective_value < best_objective_value,
                };
                
                if is_better {
                    best_objective_value = objective_value;
                    best_solution = Some((int_assignment, float_assignment));
                }
            }
        }
        
        best_solution
    }
    
    /// For a fixed integer assignment, solve the float optimization subproblem
    /// This is where we leverage the efficient float algorithms!
    fn solve_float_subproblem(&self, int_assignment: &[i32], _float_vars: &[VarId]) -> Option<Vec<f64>> {
        // With integers fixed, the float problem becomes a pure continuous optimization
        // Use the efficient float optimization algorithms from before!
        
        println!("  Fixed integers: {:?}, solving float subproblem optimally", int_assignment);
        
        // Example: If we fixed x_int = 3, then constraint "x_int * y_float ≤ 10" 
        // becomes "3 * y_float ≤ 10" → "y_float ≤ 3.333..." 
        // This is now a pure float bounds constraint - solve in O(1)!
        
        Some(vec![3.3333]) // Placeholder
    }
    
    /// Enumerate integer assignments (could use branch-and-bound for efficiency)
    fn enumerate_integer_assignments(&self, _integer_vars: &[VarId]) -> Vec<Vec<i32>> {
        // For now, enumerate all combinations
        // In practice, use branch-and-bound with bounds from float subproblems
        vec![
            vec![1], vec![2], vec![3] // Example integer assignments
        ]
    }
    
    fn evaluate_objective(&self, int_vals: &[i32], float_vals: &[f64], objective: &ObjectiveFunction) -> f64 {
        let mut value = 0.0;
        for term in &objective.terms {
            // Simplified - would need to map VarId to actual values
            value += term.coefficient * if term.var.0 < int_vals.len() {
                int_vals[term.var.0] as f64
            } else {
                float_vals[term.var.0 - int_vals.len()]
            };
        }
        value
    }
    
    fn check_coupling_constraints(&self, _int_vals: &[i32], _float_vals: &[f64]) -> bool {
        // Check if coupling constraints are satisfied
        true // Placeholder
    }
}

/// Problem classifier for mixed problems
pub fn classify_mixed_problem(
    int_domains: &[Vec<i32>],
    float_domains: &[FloatInterval], 
    constraints: &[Box<dyn MixedConstraint>]
) -> MixedProblemType {
    
    // Check for pure problems first (most efficient)
    if int_domains.is_empty() && !float_domains.is_empty() {
        // Pure float problem - use efficient algorithm
        return MixedProblemType::PureFloatOptimization {
            objective_var: VarId(0),
            is_maximize: true,
            bound_constraints: vec![],
        };
    }
    
    if float_domains.is_empty() && !int_domains.is_empty() {
        // Pure integer problem - existing solver works well
        return MixedProblemType::PureIntegerProblem {
            integer_constraints: vec![],
        };
    }
    
    // Analyze constraint coupling
    let has_coupling = constraints.iter().any(|c| constraint_couples_types(c));
    
    if !has_coupling {
        // Variables are separable - can solve independently
        MixedProblemType::SeparableMixed {
            float_part: Box::new(MixedProblemType::PureFloatOptimization {
                objective_var: VarId(0),
                is_maximize: true,
                bound_constraints: vec![],
            }),
            integer_part: Box::new(MixedProblemType::PureIntegerProblem {
                integer_constraints: vec![],
            }),
            coupling_constraints: vec![],
        }
    } else {
        // Complex coupled problem - use MINLP
        MixedProblemType::CoupledMixed {
            integer_vars: (0..int_domains.len()).map(VarId).collect(),
            float_vars: (int_domains.len()..int_domains.len() + float_domains.len()).map(VarId).collect(),
            mixed_constraints: constraints.to_vec(),
            objective: ObjectiveFunction {
                objective_type: ObjectiveType::Maximize,
                terms: vec![],
            },
        }
    }
}

fn constraint_couples_types(_constraint: &Box<dyn MixedConstraint>) -> bool {
    // Analyze if constraint involves both integer and float variables
    // For now, assume some coupling exists
    true
}

#[cfg(test)]
mod mixed_tests {
    use super::*;
    
    #[test]
    fn test_pure_float_efficiency() {
        // Pure float problem should still be O(1)
        let mut solver = MixedIntegerFloatSolver {
            integer_domains: vec![],
            float_domains: vec![FloatInterval::new(1.0, 10.0)],
            problem_type: MixedProblemType::PureFloatOptimization {
                objective_var: VarId(0),
                is_maximize: true,
                bound_constraints: vec![],
            },
        };
        
        let solution = solver.solve().expect("Should solve");
        assert_eq!(solution.0.len(), 0); // No integers
        assert_eq!(solution.1.len(), 1); // One float
    }
    
    #[test]
    fn test_separable_mixed() {
        // When constraints don't couple types, should solve efficiently
        let mut solver = MixedIntegerFloatSolver {
            integer_domains: vec![vec![1, 2, 3]],
            float_domains: vec![FloatInterval::new(1.0, 10.0)],
            problem_type: MixedProblemType::SeparableMixed {
                float_part: Box::new(MixedProblemType::PureFloatOptimization {
                    objective_var: VarId(1),
                    is_maximize: true,
                    bound_constraints: vec![],
                }),
                integer_part: Box::new(MixedProblemType::PureIntegerProblem {
                    integer_constraints: vec![],
                }),
                coupling_constraints: vec![],
            },
        };
        
        let solution = solver.solve().expect("Should solve");
        assert_eq!(solution.0.len(), 1); // One integer
        assert_eq!(solution.1.len(), 1); // One float
    }
    
    #[test]
    fn test_problem_classification() {
        // Pure float case
        let float_only = classify_mixed_problem(&[], &[FloatInterval::new(0.0, 10.0)], &[]);
        match float_only {
            MixedProblemType::PureFloatOptimization { .. } => {}, // Correct
            _ => panic!("Should classify as pure float"),
        }
        
        // Pure integer case  
        let int_only = classify_mixed_problem(&[vec![1, 2, 3]], &[], &[]);
        match int_only {
            MixedProblemType::PureIntegerProblem { .. } => {}, // Correct
            _ => panic!("Should classify as pure integer"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoundConstraint {
    var: VarId,
    constraint_type: BoundType,
    value: f64,
}

#[derive(Debug, Clone)]
pub enum BoundType {
    LessThan,           // x < value
    LessThanOrEqual,    // x ≤ value  
    GreaterThan,        // x > value
    GreaterThanOrEqual, // x ≥ value
}

#[derive(Debug, Clone)]  
pub struct LinearConstraint {
    coefficients: Vec<(VarId, f64)>, // [(var, coeff), ...]
    operator: ComparisonOp,
    rhs: f64,
}

#[derive(Debug, Clone)]
pub enum ComparisonOp {
    LessThanOrEqual,    // Σ(coeff * var) ≤ rhs
    GreaterThanOrEqual, // Σ(coeff * var) ≥ rhs  
    Equal,              // Σ(coeff * var) = rhs
}

pub trait Constraint: std::fmt::Debug {
    fn propagate(&self, vars: &mut [FloatInterval]) -> bool;
    fn is_satisfied(&self, vars: &[FloatInterval]) -> bool;
}

/// Efficient Float CSP Solver using problem classification
pub struct EfficientFloatSolver {
    variables: Vec<FloatInterval>,
    problem_type: FloatProblemType,
}

impl EfficientFloatSolver {
    /// Solve using the most efficient algorithm for the detected problem type
    pub fn solve(&mut self) -> Option<Vec<f64>> {
        match &self.problem_type {
            FloatProblemType::LinearOptimization { objective_var, is_maximize, bound_constraints } => {
                // ALGORITHM 1: Direct analytical solution - O(1) time!
                self.solve_linear_optimization(*objective_var, *is_maximize, bound_constraints)
            },
            
            FloatProblemType::BoundsConsistency { linear_constraints } => {
                // ALGORITHM 2: Bounds consistency propagation - O(n*m) time where n=vars, m=constraints
                self.solve_bounds_consistency(linear_constraints)
            },
            
            FloatProblemType::HybridSearch { nonlinear_constraints, search_variables } => {
                // ALGORITHM 3: Hybrid approach - only search when necessary
                self.solve_hybrid(nonlinear_constraints, search_variables)
            },
        }
    }
    
    /// ALGORITHM 1: Direct optimization - perfect for simple cases like "maximize x < 5.5"
    fn solve_linear_optimization(
        &mut self, 
        objective_var: VarId, 
        is_maximize: bool,
        constraints: &[BoundConstraint]
    ) -> Option<Vec<f64>> {
        let mut objective_interval = self.variables[objective_var.0].clone();
        
        // Apply all bound constraints to tighten the objective variable's interval  
        for constraint in constraints {
            if constraint.var == objective_var {
                match (constraint.constraint_type.clone(), is_maximize) {
                    (BoundType::LessThan, true) => {
                        // maximize x < value → x* = value - ε
                        let max_value = objective_interval.floor_to_step(constraint.value - objective_interval.step);
                        objective_interval.max = objective_interval.max.min(max_value);
                    },
                    (BoundType::LessThanOrEqual, true) => {
                        // maximize x ≤ value → x* = value (if value is step-aligned)
                        let max_value = objective_interval.floor_to_step(constraint.value);
                        objective_interval.max = objective_interval.max.min(max_value);
                    },
                    (BoundType::GreaterThan, false) => {
                        // minimize x > value → x* = value + ε
                        let min_value = objective_interval.ceil_to_step(constraint.value + objective_interval.step);
                        objective_interval.min = objective_interval.min.max(min_value);
                    },
                    (BoundType::GreaterThanOrEqual, false) => {
                        // minimize x ≥ value → x* = value (if value is step-aligned)
                        let min_value = objective_interval.ceil_to_step(constraint.value);
                        objective_interval.min = objective_interval.min.max(min_value);
                    },
                    // Other combinations don't directly affect the optimum
                    _ => {}
                }
            }
        }
        
        // Return the optimal value - either max or min depending on objective
        if objective_interval.is_empty() {
            return None; // Infeasible
        }
        
        let optimal_value = if is_maximize {
            objective_interval.max
        } else {
            objective_interval.min
        };
        
        // Construct full solution (for this simple case, only objective matters)
        let mut solution = vec![0.0; self.variables.len()];
        solution[objective_var.0] = optimal_value;
        Some(solution)
    }
    
    /// ALGORITHM 2: Bounds consistency - iterative interval tightening
    fn solve_bounds_consistency(&mut self, constraints: &[LinearConstraint]) -> Option<Vec<f64>> {
        let mut changed = true;
        let mut iterations = 0;
        const MAX_ITERATIONS: usize = 100;
        
        // Iterate until fixed point (no more interval changes)
        while changed && iterations < MAX_ITERATIONS {
            changed = false;
            iterations += 1;
            
            for constraint in constraints {
                if self.propagate_linear_constraint(constraint) {
                    changed = true;
                }
            }
            
            // Check for empty domains  
            for interval in &self.variables {
                if interval.is_empty() {
                    return None; // Infeasible
                }
            }
        }
        
        // Extract solution (if all variables are fixed) or return any feasible point
        let solution: Vec<f64> = self.variables.iter()
            .map(|interval| {
                if interval.is_fixed() {
                    interval.min
                } else {
                    interval.mid() // Return midpoint if not fully determined
                }
            })
            .collect();
            
        Some(solution)
    }
    
    /// ALGORITHM 3: Hybrid approach - only search when bounds consistency stalls
    fn solve_hybrid(&mut self, _constraints: &[Box<dyn Constraint>], _search_vars: &[VarId]) -> Option<Vec<f64>> {
        // 1. First apply bounds consistency as much as possible
        // 2. Only if progress stalls, do limited branching on most constrained variables
        // 3. Avoid exponential search explosion
        
        // This would be the fallback for complex nonlinear constraints
        // For now, return a placeholder
        println!("Hybrid solver not implemented - falling back to current approach");
        None
    }
    
    /// Propagate a single linear constraint using interval arithmetic
    fn propagate_linear_constraint(&mut self, constraint: &LinearConstraint) -> bool {
        // Example: 2*x + 3*y ≤ 10
        // If x ∈ [1,3] and y ∈ [2,4], then:
        // 2*x ∈ [2,6], 3*y ∈ [6,12], sum ∈ [8,18]
        // Since sum ≤ 10, we need sum ∈ [8,10], which constrains variables
        
        let mut changed = false;
        
        // This would implement interval arithmetic propagation
        // For brevity, showing the concept rather than full implementation
        
        for &(var_id, coeff) in &constraint.coefficients {
            // Calculate bounds for this variable given others are fixed
            // Update interval if tighter bounds found
            // Set changed = true if any interval was modified
            let _current_interval = &mut self.variables[var_id.0];
            // ... interval arithmetic implementation ...
        }
        
        changed
    }
}

/// Problem classifier - analyzes constraints to pick optimal algorithm
pub fn classify_float_problem(
    variables: &[FloatInterval], 
    constraints: &[Box<dyn Constraint>]
) -> FloatProblemType {
    // Analyze constraint types and patterns
    
    // Check for simple single-variable optimization
    if constraints.len() <= 3 && variables.len() == 1 {
        // This looks like: maximize x subject to bounds
        return FloatProblemType::LinearOptimization {
            objective_var: VarId(0),
            is_maximize: true, // Would be determined from solve call
            bound_constraints: vec![], // Would be extracted from constraints
        };
    }
    
    // Check if all constraints are linear
    if constraints.iter().all(|c| is_linear_constraint(c)) {
        return FloatProblemType::BoundsConsistency {
            linear_constraints: vec![], // Would be converted from constraint objects
        };
    }
    
    // Default to hybrid for complex cases
    FloatProblemType::HybridSearch {
        nonlinear_constraints: constraints.to_vec(),
        search_variables: (0..variables.len()).map(VarId).collect(),
    }
}

fn is_linear_constraint(_constraint: &Box<dyn Constraint>) -> bool {
    // Would analyze constraint to determine if it's linear
    // For now, assume most are linear
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_linear_optimization_efficiency() {
        // Test case: maximize x subject to x < 5.5, x ∈ [1, 10]
        // This should be solved in O(1) time, not O(log n) binary search!
        
        let objective_var = VarId(0);
        let mut solver = EfficientFloatSolver {
            variables: vec![FloatInterval::with_step(1.0, 10.0, 1e-4)],
            problem_type: FloatProblemType::LinearOptimization {
                objective_var,
                is_maximize: true,
                bound_constraints: vec![
                    BoundConstraint {
                        var: objective_var,
                        constraint_type: BoundType::LessThan,
                        value: 5.5,
                    }
                ],
            },
        };
        
        let solution = solver.solve().expect("Should have solution");
        
        // Should find x* = 5.5 - 1e-4 = 5.4999
        assert!((solution[0] - 5.4999).abs() < 1e-5);
        
        // Most importantly: this should require ZERO propagation steps!
        // It's a direct analytical solution.
    }
    
    #[test] 
    fn test_problem_classification() {
        // Test that simple optimization problems are correctly classified
        let variables = vec![FloatInterval::new(1.0, 10.0)];
        let constraints: Vec<Box<dyn Constraint>> = vec![];
        
        let problem_type = classify_float_problem(&variables, &constraints);
        
        match problem_type {
            FloatProblemType::LinearOptimization { .. } => {
                // Correct classification!
            },
            _ => panic!("Should classify simple case as LinearOptimization"),
        }
    }
}
