//! Integration layer between CSP constraint solver and LP solver
//!
//! This module provides functionality to:
//! 1. Extract linear constraints from CSP propagators
//! 2. Convert them to LP problem format
//! 3. Apply LP solutions back to CSP variable domains
//!
//! # Example Flow
//! ```ignore
//! // In CSP solving:
//! let system = extract_linear_system(&propagators, &vars)?;
//! if system.is_suitable_for_lp() {
//!     let lp_problem = system.to_lp_problem(&vars);
//!     let solution = solve(&lp_problem)?;
//!     apply_lp_solution(&system, &solution, &mut context)?;
//! }
//! ```

use crate::variables::{VarId, Vars, Val};
use crate::variables::views::Context;
use crate::lpsolver::{LpProblem, LpSolution, LpStatus};

/// Type of relation in a linear constraint
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintRelation {
    /// Equality: sum(coeffs * vars) = rhs
    Equality,
    /// Less-or-equal: sum(coeffs * vars) ≤ rhs
    LessOrEqual,
    /// Greater-or-equal: sum(coeffs * vars) ≥ rhs
    GreaterOrEqual,
}

/// A single linear constraint extracted from CSP
#[derive(Debug, Clone)]
pub struct LinearConstraint {
    /// Coefficients for each variable
    pub coefficients: Vec<f64>,
    
    /// Variable IDs (same length as coefficients)
    pub variables: Vec<VarId>,
    
    /// Type of constraint relation
    pub relation: ConstraintRelation,
    
    /// Right-hand side constant
    pub rhs: f64,
}

impl LinearConstraint {
    /// Create a new linear constraint
    pub fn new(
        coefficients: Vec<f64>,
        variables: Vec<VarId>,
        relation: ConstraintRelation,
        rhs: f64,
    ) -> Self {
        assert_eq!(coefficients.len(), variables.len(), 
                   "Coefficients and variables must have same length");
        Self {
            coefficients,
            variables,
            relation,
            rhs,
        }
    }
    
    /// Create an equality constraint
    pub fn equality(coefficients: Vec<f64>, variables: Vec<VarId>, rhs: f64) -> Self {
        Self::new(coefficients, variables, ConstraintRelation::Equality, rhs)
    }
    
    /// Create a less-or-equal constraint
    pub fn less_or_equal(coefficients: Vec<f64>, variables: Vec<VarId>, rhs: f64) -> Self {
        Self::new(coefficients, variables, ConstraintRelation::LessOrEqual, rhs)
    }
    
    /// Create a greater-or-equal constraint
    pub fn greater_or_equal(coefficients: Vec<f64>, variables: Vec<VarId>, rhs: f64) -> Self {
        Self::new(coefficients, variables, ConstraintRelation::GreaterOrEqual, rhs)
    }
    
    /// Convert to standard form (≤) constraints
    /// Returns (coefficients, rhs) pairs for one or two constraints
    pub fn to_standard_form(&self) -> Vec<(Vec<f64>, f64)> {
        match self.relation {
            ConstraintRelation::LessOrEqual => {
                // Already in standard form: a^T x ≤ b
                vec![(self.coefficients.clone(), self.rhs)]
            }
            ConstraintRelation::GreaterOrEqual => {
                // a^T x ≥ b  →  -a^T x ≤ -b
                let neg_coeffs: Vec<f64> = self.coefficients.iter().map(|&c| -c).collect();
                vec![(neg_coeffs, -self.rhs)]
            }
            ConstraintRelation::Equality => {
                // a^T x = b  →  a^T x ≤ b AND -a^T x ≤ -b
                let neg_coeffs: Vec<f64> = self.coefficients.iter().map(|&c| -c).collect();
                vec![
                    (self.coefficients.clone(), self.rhs),
                    (neg_coeffs, -self.rhs),
                ]
            }
        }
    }
}

/// A system of linear constraints extracted from CSP
#[derive(Debug, Clone)]
pub struct LinearConstraintSystem {
    /// All float variables involved in the linear system
    pub variables: Vec<VarId>,
    
    /// All linear constraints
    pub constraints: Vec<LinearConstraint>,
    
    /// Optional objective function for optimization
    pub objective: Option<LinearObjective>,
}

/// Linear objective function
#[derive(Debug, Clone)]
pub struct LinearObjective {
    /// Coefficients for objective function
    pub coefficients: Vec<f64>,
    
    /// Whether to minimize (true) or maximize (false)
    pub minimize: bool,
}

impl LinearConstraintSystem {
    /// Create a new empty linear constraint system
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
            constraints: Vec::new(),
            objective: None,
        }
    }
    
    /// Add a constraint to the system
    pub fn add_constraint(&mut self, constraint: LinearConstraint) {
        // Update variable list
        for &var in &constraint.variables {
            if !self.variables.contains(&var) {
                self.variables.push(var);
            }
        }
        self.constraints.push(constraint);
    }
    
    /// Set the objective function for optimization
    pub fn set_objective(&mut self, coefficients: Vec<f64>, minimize: bool) {
        assert_eq!(coefficients.len(), self.variables.len(),
                   "Objective coefficients must match number of variables");
        self.objective = Some(LinearObjective { coefficients, minimize });
    }
    
    /// Check if this system is suitable for LP solving
    /// 
    /// Returns true if:
    /// - Has at least 1 linear constraint
    /// - Has at least 2 variables (single variable is trivial)
    /// 
    /// LP is always beneficial for linear systems with multiple variables,
    /// including:
    /// - Pure float problems
    /// - Mixed integer-float problems (LP relaxation provides bounds)
    /// - Any domain size (LP handles both small and large domains efficiently)
    pub fn is_suitable_for_lp(&self, _vars: &Vars) -> bool {
        // Need at least one constraint and at least 2 variables
        // Single variable problems are trivial and don't need LP
        !self.constraints.is_empty() && self.variables.len() >= 2
    }
    
    /// Convert this system to an LpProblem for the LP solver
    /// 
    /// # Arguments
    /// * `vars` - Variable store to extract current bounds
    /// 
    /// # Returns
    /// An LpProblem in standard form (maximize c^T x s.t. Ax ≤ b, l ≤ x ≤ u)
    /// 
    /// Note: Constants (variables with lower_bound == upper_bound) are substituted
    /// into constraints and not included as LP variables.
    pub fn to_lp_problem(&self, vars: &Vars) -> LpProblem {
        // Use the default LP tolerance for consistency with solver
        let tolerance = crate::lpsolver::LpConfig::default().feasibility_tol;
        
        // Step 1: Identify constants and build mapping from system vars to LP vars
        let mut var_to_lp_index: std::collections::HashMap<VarId, usize> = std::collections::HashMap::new();
        let mut lp_index_to_var: Vec<VarId> = Vec::new();
        let mut constants: std::collections::HashMap<VarId, f64> = std::collections::HashMap::new();
        
        for &var in &self.variables {
            let (lower, upper) = extract_bounds(var, vars);
            if (upper - lower).abs() < tolerance {
                // This is a constant
                constants.insert(var, lower);
            } else {
                // This is a decision variable
                let lp_idx = lp_index_to_var.len();
                var_to_lp_index.insert(var, lp_idx);
                lp_index_to_var.push(var);
            }
        }
        
        let n_vars = lp_index_to_var.len();
        
        // Build objective vector (default to zero if no objective)
        // Only include non-constant variables
        let c = if let Some(ref obj) = self.objective {
            // Map objective coefficients to LP variable order (excluding constants)
            let mut obj_vec = vec![0.0; n_vars];
            for (sys_idx, &var) in self.variables.iter().enumerate() {
                if let Some(&lp_idx) = var_to_lp_index.get(&var) {
                    if sys_idx < obj.coefficients.len() {
                        obj_vec[lp_idx] = if obj.minimize {
                            -obj.coefficients[sys_idx]
                        } else {
                            obj.coefficients[sys_idx]
                        };
                    }
                }
            }
            obj_vec
        } else {
            vec![0.0; n_vars]
        };
        
        // Build constraint matrix A and RHS b
        // Substitute constants and only include non-constant variables
        // Pre-allocate: estimate 2 rows per constraint (for equality constraints)
        let estimated_rows = self.constraints.len() * 2;
        let mut a = Vec::with_capacity(estimated_rows);
        let mut b = Vec::with_capacity(estimated_rows);
        
        if self.constraints.len() > 20 {
            eprintln!("LP BUILD: Processing {} constraints with {} variables (output suppressed for performance)...", 
                self.constraints.len(), n_vars);
        }
        
        for constraint in &self.constraints {
            // Only print detailed info for small problems (avoid performance hit)
            if self.constraints.len() <= 20 {
                eprintln!("LP BUILD: Converting constraint with {} vars, relation {:?}, rhs {}", 
                    constraint.variables.len(), constraint.relation, constraint.rhs);
            }
            
            // Convert each constraint to standard form (one or two ≤ constraints)
            for (std_coeffs, std_rhs) in constraint.to_standard_form() {
                // Build row with only non-constant variables
                // Substitute constants into RHS
                let mut row = vec![0.0; n_vars];
                let mut rhs_adjusted = std_rhs;
                
                for (j, &var) in constraint.variables.iter().enumerate() {
                    let coeff = std_coeffs[j];
                    
                    if let Some(&const_val) = constants.get(&var) {
                        // This is a constant - move it to RHS
                        rhs_adjusted -= coeff * const_val;
                        if self.constraints.len() <= 20 {
                            eprintln!("LP BUILD:   var {:?} is constant = {}, adjusting RHS by -{} * {} = {}", 
                                var, const_val, coeff, const_val, -coeff * const_val);
                        }
                    } else if let Some(&lp_idx) = var_to_lp_index.get(&var) {
                        // This is a decision variable
                        row[lp_idx] = coeff;
                    }
                }
                
                // Only print rows for small problems (printing 225-element vectors is SLOW!)
                if self.constraints.len() <= 20 {
                    eprintln!("LP BUILD: Constraint row = {:?}, rhs = {}", row, rhs_adjusted);
                }
                a.push(row);
                b.push(rhs_adjusted);
            }
        }
        
        // Extract variable bounds (only for non-constant variables)
        let lower_bounds: Vec<f64> = lp_index_to_var.iter()
            .map(|&v| extract_bounds(v, vars).0)
            .collect();
            
        let upper_bounds: Vec<f64> = lp_index_to_var.iter()
            .map(|&v| extract_bounds(v, vars).1)
            .collect();
        
        let n_constraints = a.len();
        
        eprintln!("LP BUILD: Final problem: {} variables (excluding {} constants), {} constraints", 
            n_vars, constants.len(), n_constraints);
        
        LpProblem::new(n_vars, n_constraints, c, a, b, lower_bounds, upper_bounds)
    }
    
    /// Number of variables in the system
    pub fn n_variables(&self) -> usize {
        self.variables.len()
    }
    
    /// Number of constraints in the system
    pub fn n_constraints(&self) -> usize {
        self.constraints.len()
    }
}

impl Default for LinearConstraintSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Extract variable bounds from the CSP variable store
/// 
/// Returns (lower_bound, upper_bound) as f64 values
fn extract_bounds(var: VarId, vars: &Vars) -> (f64, f64) {
    use crate::variables::views::ViewRaw;
    
    let lower = match var.min_raw(vars) {
        Val::ValF(f) => f,
        Val::ValI(i) => i as f64,
    };
    let upper = match var.max_raw(vars) {
        Val::ValF(f) => f,
        Val::ValI(i) => i as f64,
    };
    (lower, upper)
}

/// Apply LP solution back to CSP variable domains
/// 
/// This updates the variable bounds in the CSP to match the LP solution,
/// effectively pruning the search space. The strategy is to tighten the
/// variable bounds based on the LP solution values.
/// 
/// # Arguments
/// * `system` - The linear constraint system with variable mapping
/// * `solution` - The LP solution containing optimal values
/// * `ctx` - Mutable context for updating variable domains
/// 
/// # Returns
/// `None` if any bound update causes inconsistency (prune this branch),
/// `Some(())` if all updates succeeded
/// 
/// # Strategy
/// For each variable in the LP solution:
/// 1. Extract the LP solution value
/// 2. Tighten the CSP domain bounds towards this value
/// 3. Use a tolerance for floating point comparisons
pub fn apply_lp_solution(
    system: &LinearConstraintSystem,
    solution: &LpSolution,
    ctx: &mut Context,
) -> Option<()> {
    use crate::variables::views::View;
    
    // Only apply if we got an optimal solution
    if solution.status != LpStatus::Optimal {
        return Some(()); // Don't fail, just don't update
    }
    
    // Use the default LP tolerance for consistency with solver
    let tolerance = crate::lpsolver::LpConfig::default().feasibility_tol;
    
    // Rebuild the mapping from LP indices to variables (excluding constants)
    let mut lp_index_to_var: Vec<VarId> = Vec::new();
    for &var in &system.variables {
        let (lower, upper) = extract_bounds(var, ctx.vars());
        if (upper - lower).abs() >= tolerance {
            // Non-constant variable
            lp_index_to_var.push(var);
        }
    }
    
    // Sanity check: LP solution should have same number of variables
    if solution.x.len() != lp_index_to_var.len() {
        eprintln!("LP APPLY: WARNING: LP solution has {} variables but expected {}", 
            solution.x.len(), lp_index_to_var.len());
        return Some(()); // Don't apply if mismatch
    }
    
    // Update each non-constant variable with its LP solution value
    for (lp_idx, &var_id) in lp_index_to_var.iter().enumerate() {
        let lp_value = solution.x[lp_idx];
        
        // Get current bounds
        let (current_lower, current_upper) = extract_bounds(var_id, ctx.vars());
        
        eprintln!("LP APPLY: var {:?} LP_value={} bounds=[{}, {}]", var_id, lp_value, current_lower, current_upper);
        
        // Skip constants (variables where lower == upper)
        // These are fixed values and shouldn't be modified
        if (current_upper - current_lower).abs() < tolerance {
            eprintln!("LP APPLY: var {:?} is constant, skipping", var_id);
            continue;
        }
        
        // Sanity check: LP value should be within current bounds
        if lp_value < current_lower - tolerance || lp_value > current_upper + tolerance {
            // LP solution violates current bounds - this indicates
            // numerical issues or inconsistency. Skip this variable.
            continue;
        }
        
        // Strategy: Tighten bounds towards LP solution
        // We use a conservative approach: only tighten if LP value
        // is significantly different from current bounds
        
        // Tighten lower bound if LP value is higher
        if lp_value > current_lower + tolerance {
            let new_min = Val::ValF(lp_value);
            if var_id.try_set_min(new_min, ctx).is_none() {
                // Inconsistency detected - propagation failed
                return None;
            }
        }
        
        // Tighten upper bound if LP value is lower
        if lp_value < current_upper - tolerance {
            let new_max = Val::ValF(lp_value);
            if var_id.try_set_max(new_max, ctx).is_none() {
                // Inconsistency detected - propagation failed
                return None;
            }
        }
    }
    
    Some(())
}

// NOTE: For Phase 2, we'll need to add extraction methods to the Propagators struct
// in src/constraints/props/mod.rs since the linear module is private.
// 
// The extraction will happen in two steps:
// 1. Add a method to Propagators: pub fn extract_linear_system(&self) -> LinearConstraintSystem
// 2. That method will have access to the private linear module and can downcast propagators
//
// For now, we focus on the LinearConstraintSystem infrastructure which is complete.

#[cfg(test)]
mod tests {
    use super::*;
    
    // Helper to create VarIds for testing
    fn var(index: usize) -> VarId {
        VarId::from_index(index)
    }
    
    #[test]
    fn test_linear_constraint_creation() {
        let vars = vec![var(0), var(1)];
        let coeffs = vec![2.0, 3.0];
        
        let constraint = LinearConstraint::equality(coeffs.clone(), vars.clone(), 10.0);
        assert_eq!(constraint.coefficients, coeffs);
        assert_eq!(constraint.variables.len(), vars.len());
        assert_eq!(constraint.relation, ConstraintRelation::Equality);
        assert_eq!(constraint.rhs, 10.0);
    }
    
    #[test]
    fn test_constraint_to_standard_form() {
        let vars = vec![var(0), var(1)];
        
        // Test ≤ constraint (already standard)
        let le = LinearConstraint::less_or_equal(vec![1.0, 2.0], vars.clone(), 5.0);
        let std = le.to_standard_form();
        assert_eq!(std.len(), 1);
        assert_eq!(std[0].0, vec![1.0, 2.0]);
        assert_eq!(std[0].1, 5.0);
        
        // Test ≥ constraint (negate)
        let ge = LinearConstraint::greater_or_equal(vec![1.0, 2.0], vars.clone(), 5.0);
        let std = ge.to_standard_form();
        assert_eq!(std.len(), 1);
        assert_eq!(std[0].0, vec![-1.0, -2.0]);
        assert_eq!(std[0].1, -5.0);
        
        // Test = constraint (two constraints)
        let eq = LinearConstraint::equality(vec![1.0, 2.0], vars.clone(), 5.0);
        let std = eq.to_standard_form();
        assert_eq!(std.len(), 2);
        assert_eq!(std[0].0, vec![1.0, 2.0]);
        assert_eq!(std[0].1, 5.0);
        assert_eq!(std[1].0, vec![-1.0, -2.0]);
        assert_eq!(std[1].1, -5.0);
    }
    
    #[test]
    fn test_linear_system_creation() {
        let mut system = LinearConstraintSystem::new();
        assert_eq!(system.n_variables(), 0);
        assert_eq!(system.n_constraints(), 0);
        
        let vars = vec![var(0), var(1)];
        let constraint = LinearConstraint::less_or_equal(vec![1.0, 2.0], vars, 5.0);
        system.add_constraint(constraint);
        
        assert_eq!(system.n_variables(), 2);
        assert_eq!(system.n_constraints(), 1);
    }
    
    #[test]
    fn test_system_deduplicates_variables() {
        let mut system = LinearConstraintSystem::new();
        
        // Add two constraints with overlapping variables
        let c1 = LinearConstraint::less_or_equal(
            vec![1.0, 2.0], 
            vec![var(0), var(1)], 
            5.0
        );
        let c2 = LinearConstraint::less_or_equal(
            vec![3.0, 1.0], 
            vec![var(1), var(2)], 
            10.0
        );
        
        system.add_constraint(c1);
        system.add_constraint(c2);
        
        // Should have 3 unique variables: 0, 1, 2
        assert_eq!(system.n_variables(), 3);
        assert_eq!(system.n_constraints(), 2);
    }
}
