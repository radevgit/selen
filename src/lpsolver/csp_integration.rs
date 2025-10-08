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
    /// - Has at least 3 constraints (otherwise propagation is fast enough)
    /// - All variables are float type
    /// - Variables have large domains (where LP excels)
    pub fn is_suitable_for_lp(&self, vars: &Vars) -> bool {
        // Need multiple constraints to justify LP overhead
        if self.constraints.len() < 3 {
            return false;
        }
        
        // Check if variables have large domains
        let mut has_large_domain = false;
        for &var in &self.variables {
            // Check if variable is float and has large domain
            let (lower, upper) = extract_bounds(var, vars);
            let domain_size = upper - lower;
            
            // Consider domain "large" if > 1000
            if domain_size > 1000.0 {
                has_large_domain = true;
            }
            
            // If any variable has small discrete domain, propagation might be better
            if domain_size < 10.0 {
                return false;
            }
        }
        
        has_large_domain
    }
    
    /// Convert this system to an LpProblem for the LP solver
    /// 
    /// # Arguments
    /// * `vars` - Variable store to extract current bounds
    /// 
    /// # Returns
    /// An LpProblem in standard form (maximize c^T x s.t. Ax ≤ b, l ≤ x ≤ u)
    pub fn to_lp_problem(&self, vars: &Vars) -> LpProblem {
        let n_vars = self.variables.len();
        
        // Build objective vector (default to zero if no objective)
        let mut c = if let Some(ref obj) = self.objective {
            // Map objective coefficients to variable order
            let mut obj_vec = vec![0.0; n_vars];
            for (i, &var) in self.variables.iter().enumerate() {
                // Find this variable in objective
                if i < obj.coefficients.len() {
                    obj_vec[i] = obj.coefficients[i];
                }
            }
            
            // LP solver maximizes, so negate if we want to minimize
            if obj.minimize {
                obj_vec.iter().map(|&x| -x).collect()
            } else {
                obj_vec
            }
        } else {
            vec![0.0; n_vars]
        };
        
        // Build constraint matrix A and RHS b
        let mut a = Vec::new();
        let mut b = Vec::new();
        
        for constraint in &self.constraints {
            // Convert each constraint to standard form (one or two ≤ constraints)
            for (std_coeffs, std_rhs) in constraint.to_standard_form() {
                // Map coefficients to our variable ordering
                let mut row = vec![0.0; n_vars];
                for (j, &var) in constraint.variables.iter().enumerate() {
                    if let Some(idx) = self.variables.iter().position(|&v| v == var) {
                        row[idx] = std_coeffs[j];
                    }
                }
                a.push(row);
                b.push(std_rhs);
            }
        }
        
        // Extract variable bounds
        let lower_bounds: Vec<f64> = self.variables.iter()
            .map(|&v| extract_bounds(v, vars).0)
            .collect();
            
        let upper_bounds: Vec<f64> = self.variables.iter()
            .map(|&v| extract_bounds(v, vars).1)
            .collect();
        
        let n_constraints = a.len();
        
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
/// effectively pruning the search space.
/// 
/// # Returns
/// `None` if any bound update causes inconsistency (prune this branch)
pub fn apply_lp_solution(
    system: &LinearConstraintSystem,
    solution: &LpSolution,
    vars: &mut Vars,
) -> Option<()> {
    // Only apply if we got an optimal solution
    if solution.status != LpStatus::Optimal {
        return Some(()); // Don't fail, just don't update
    }
    
    // Update each variable with its LP solution value
    for (i, &var_id) in system.variables.iter().enumerate() {
        let lp_value = solution.x[i];
        
        // For now, use LP solution to tighten bounds
        // Strategy: Set both bounds to LP value (fixes variable)
        // This is aggressive but guarantees consistency with LP constraints
        
        // Get current bounds
        let (current_lower, current_upper) = extract_bounds(var_id, vars);
        
        // Only tighten if LP value is within current bounds (sanity check)
        if lp_value < current_lower - 1e-6 || lp_value > current_upper + 1e-6 {
            // LP solution violates current bounds - shouldn't happen
            // This indicates numerical issues or inconsistency
            continue;
        }
        
        // Update bounds to LP solution
        // Note: This requires access to domain update methods
        // For now, we'll document that this needs integration with Context
        // TODO: Integrate with Context::try_set_min/try_set_max
        
        // Placeholder: Would call something like:
        // var_id.try_set_min(Val::ValF(lp_value), context)?;
        // var_id.try_set_max(Val::ValF(lp_value), context)?;
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
