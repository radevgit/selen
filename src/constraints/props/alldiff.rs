use crate::{
    constraints::props::{Propagate, Prune},
    constraints::gac::Variable,
    constraints::gac_hybrid::HybridGAC,
    variables::{Val, VarId},
    variables::views::{Context, View},
};
use std::collections::HashSet;

/// Ultra-efficient AllDifferent constraint implementation
/// 
/// This implementation combines several optimization techniques:
/// 1. Early termination on impossible configurations
/// 2. Efficient small-case handling for 2-3 variables
/// 3. Scalable propagation for larger problems
/// 4. Proper integration with the constraint propagation framework
#[derive(Clone, Debug)]
#[doc(hidden)]
pub struct AllDiff {
    vars: Vec<VarId>,
}

impl AllDiff {
    pub fn new(vars: Vec<VarId>) -> Self {
        Self { vars }
    }
    
    /// Check if a domain is a singleton (single value)
    fn is_singleton(&self, min_val: Val, max_val: Val) -> bool {
        match (min_val, max_val) {
            (Val::ValI(min_i), Val::ValI(max_i)) => min_i == max_i,
            (Val::ValF(min_f), Val::ValF(max_f)) => min_f == max_f,
            (Val::ValI(min_i), Val::ValF(max_f)) => (min_i as f64) == max_f,
            (Val::ValF(min_f), Val::ValI(max_i)) => min_f == (max_i as f64),
        }
    }
    
    /// Check if two values are equal using proper interval context.
    /// For the target variable's context, we use its interval if available.
    /// For the forbidden value, we fall back to a conservative comparison.
    fn values_equal(&self, val1: Val, val2: Val, target_var: VarId, ctx: &Context) -> bool {
        let target_interval = ctx.vars().get_float_interval(target_var);
        val1.equals_with_intervals(&val2, target_interval, None)
    }
    
    /// Exclude a specific value from a variable's domain
    fn exclude_value_from_domain(&self, var: VarId, forbidden_value: Val, ctx: &mut Context) -> Option<()> {
        let current_min = var.min(ctx);
        let current_max = var.max(ctx);
        
        // If the forbidden value is outside the current domain, nothing to do
        if forbidden_value < current_min || forbidden_value > current_max {
            return Some(());
        }
        
        // If the forbidden value is the only value in the domain, domain becomes empty
        if self.is_singleton(current_min, current_max) && self.values_equal(current_min, forbidden_value, var, ctx) {
            return None; // Domain becomes empty - constraint violation
        }
        
        // If forbidden value is at the minimum bound, move minimum up
        if self.values_equal(current_min, forbidden_value, var, ctx) {
            let new_min = self.get_next_value(forbidden_value);
            var.try_set_min(new_min, ctx)?;
            return Some(());
        }
        
        // If forbidden value is at the maximum bound, move maximum down
        if self.values_equal(current_max, forbidden_value, var, ctx) {
            let new_max = self.get_prev_value(forbidden_value);
            var.try_set_max(new_max, ctx)?;
            return Some(());
        }
        
        // For values in the middle of the domain, we cannot exclude them with interval domains.
        // This is a fundamental limitation - the constraint will be enforced when variables
        // become assigned during search.
        
        Some(())
    }
    
    /// Get the next representable value
    fn get_next_value(&self, value: Val) -> Val {
        match value {
            Val::ValI(i) => Val::ValI(i + 1),
            Val::ValF(f) => Val::ValF(f + f64::EPSILON), // Simple increment for floats
        }
    }
    
    /// Get the previous representable value
    fn get_prev_value(&self, value: Val) -> Val {
        match value {
            Val::ValI(i) => Val::ValI(i - 1),
            Val::ValF(f) => Val::ValF(f - f64::EPSILON), // Simple decrement for floats
        }
    }
    
    /// GAC-based propagation using HybridGAC for optimal performance
    /// Automatically selects BitSetGAC for small domains and SparseSetGAC for large domains
    fn propagate_gac(&self, ctx: &mut Context) -> Option<()> {
        
        // Check if all variables have integer domains (GAC requirement)
        for &var in &self.vars {
            let min_val = var.min(ctx);
            let max_val = var.max(ctx);
            
            match (min_val, max_val) {
                (Val::ValI(_), Val::ValI(_)) => {
                    // Integer domain - can use GAC
                }
                _ => {
                    // Float domain - fall back to basic propagation
                    return self.propagate_basic(ctx);
                }
            }
        }
        
        // Create HybridGAC instance - automatically selects best implementation
        let mut gac = HybridGAC::new();
        
        // Add variables directly with their current domain bounds
        for (var_idx, &var) in self.vars.iter().enumerate() {
            let min_val = var.min(ctx);
            let max_val = var.max(ctx);
            
            if let (Val::ValI(min_i), Val::ValI(max_i)) = (min_val, max_val) {
                if min_i > max_i {
                    return None; // Empty domain
                }
                
                // Add variable to GAC with its current bounds
                // HybridGAC automatically chooses BitSet for small domains (≤128) 
                // and SparseSet for larger domains
                if let Err(_) = gac.add_variable(Variable(var_idx), min_i, max_i) {
                    return None; // Invalid domain
                }
            }
        }
        
        // Collect all variables for propagation
        let all_vars: Vec<Variable> = (0..self.vars.len()).map(Variable).collect();
        
        // Apply GAC propagation using the hybrid approach
        let (_changed, consistent) = gac.propagate_alldiff(&all_vars);
        if !consistent {
            return None; // GAC detected inconsistency
        }
        
        // Apply GAC results back to variable domains using direct access to bounds
        for (var_idx, &var) in self.vars.iter().enumerate() {
            let gac_var = Variable(var_idx);
            
            if gac.is_assigned(gac_var) {
                // Variable became assigned - set bounds to the single value
                if let Some(assigned_val) = gac.assigned_value(gac_var) {
                    var.try_set_min(Val::ValI(assigned_val), ctx)?;
                    var.try_set_max(Val::ValI(assigned_val), ctx)?;
                }
            } else {
                // For unassigned variables, use efficient bounds access
                if let Some((new_min, new_max)) = gac.get_bounds(gac_var) {
                    var.try_set_min(Val::ValI(new_min), ctx)?;
                    var.try_set_max(Val::ValI(new_max), ctx)?;
                } else {
                    return None; // Empty domain
                }
            }
        }
        
        Some(())
    }
    
    /// Basic propagation as fallback for GAC
    fn propagate_basic(&self, ctx: &mut Context) -> Option<()> {
        let mut assigned_values = Vec::new();
        
        // First pass: collect assigned values and check for conflicts
        for &var in &self.vars {
            let min_val = var.min(ctx);
            let max_val = var.max(ctx);
            
            if self.is_singleton(min_val, max_val) {
                for existing_val in &assigned_values {
                    if self.values_equal(min_val, *existing_val, var, ctx) {
                        return None; // Conflict detected
                    }
                }
                assigned_values.push(min_val);
            }
        }
        
        // Second pass: remove assigned values from unassigned variables
        for &var in &self.vars {
            let min_val = var.min(ctx);
            let max_val = var.max(ctx);
            
            if !self.is_singleton(min_val, max_val) { // Not assigned
                for &assigned_val in &assigned_values {
                    self.exclude_value_from_domain(var, assigned_val, ctx)?;
                }
            }
        }
        
        Some(())
    }
    
    /// Check if we can apply a basic feasibility test
    fn quick_feasibility_check(&self, ctx: &Context) -> bool {
        // For integer domains, we can count unique values available
        // For float domains, this is harder due to infinite precision
        let mut all_int_values = HashSet::new();
        let mut has_float_vars = false;
        let mut any_empty = false;
        
        for &var in &self.vars {
            let min_val = var.min(ctx);
            let max_val = var.max(ctx);
            
            match (min_val, max_val) {
                (Val::ValI(min_i), Val::ValI(max_i)) => {
                    if min_i > max_i {
                        any_empty = true;
                        break;
                    }
                    
                    // For integer domains, count discrete values
                    for v in min_i..=max_i {
                        all_int_values.insert(v);
                    }
                }
                _ => {
                    // Float variables have continuous domains - assume feasible
                    has_float_vars = true;
                }
            }
        }
        
        if any_empty {
            return false;
        }
        
        // If we have only integer variables, check if enough values available
        if !has_float_vars {
            let unique_count = all_int_values.len();
            let var_count = self.vars.len();
            return unique_count >= var_count;
        }
        
        // With float variables, assume feasible (constraint will be checked during search)
        true
    }
}

impl Prune for AllDiff {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        let n = self.vars.len();
        
        // Empty constraint is trivially satisfied
        if n == 0 {
            return Some(());
        }
        
        // Single variable is always consistent
        if n == 1 {
            return Some(());
        }
        
        // Quick feasibility check
        if !self.quick_feasibility_check(ctx) {
            return None;
        }
        
        // Use GAC as the preferred algorithm for all AllDifferent constraints
        // GAC provides the strongest propagation and handles all problem sizes efficiently
        self.propagate_gac(ctx)
    }
}

impl Propagate for AllDiff {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        self.vars.iter().copied()
    }
}
