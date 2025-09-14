use crate::{
    props::{Propagate, Prune},
    vars::{Val, VarId},
    views::{Context, View},
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
pub struct AllDifferent {
    vars: Vec<VarId>,
}

impl AllDifferent {
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
    
    /// Specialized propagation for small problems (≤ 10 variables)
    fn propagate_small(&self, ctx: &mut Context) -> Option<()> {
        let n = self.vars.len();
        
        // For very small problems, use direct domain checking
        if n <= 3 {
            return self.propagate_naive(ctx);
        }
        
        // For small problems (4-10 variables), use simplified matching
        let mut assigned_values = Vec::new();
        let mut unassigned_vars = Vec::new();
        
        // Collect assigned variables
        for &var in &self.vars {
            let min_val = var.min(ctx);
            let max_val = var.max(ctx);
            
            if self.is_singleton(min_val, max_val) {
                // Variable is assigned
                for existing_val in &assigned_values {
                    if self.values_equal(min_val, *existing_val, var, ctx) {
                        return None; // Conflict
                    }
                }
                assigned_values.push(min_val);
            } else {
                unassigned_vars.push(var);
            }
        }
        
        // Remove assigned values from unassigned variables
        for &var in &unassigned_vars {
            for &assigned_val in &assigned_values {
                self.exclude_value_from_domain(var, assigned_val, ctx)?;
            }
        }
        
        Some(())
    }
    
    /// Naive propagation for very small problems (≤ 3 variables)
    fn propagate_naive(&self, ctx: &mut Context) -> Option<()> {
        let n = self.vars.len();
        
        // For n=2: simple mutual exclusion
        if n == 2 {
            let var1 = self.vars[0];
            let var2 = self.vars[1];
            
            let min1 = var1.min(ctx);
            let max1 = var1.max(ctx);
            let min2 = var2.min(ctx);
            let max2 = var2.max(ctx);
            
            // If both variables are assigned to the same value, fail
            if self.is_singleton(min1, max1) && self.is_singleton(min2, max2) && self.values_equal(min1, min2, var1, ctx) {
                return None;
            }
            
            // If one is assigned, remove that value from the other
            if self.is_singleton(min1, max1) {
                self.exclude_value_from_domain(var2, min1, ctx)?;
            }
            if self.is_singleton(min2, max2) {
                self.exclude_value_from_domain(var1, min2, ctx)?;
            }
            
            return Some(());
        }
        
        // For n=3: more complex but still manageable
        if n == 3 {
            // Apply simple propagation rules
            for i in 0..3 {
                let var_i = self.vars[i];
                let min_i = var_i.min(ctx);
                let max_i = var_i.max(ctx);
                
                if self.is_singleton(min_i, max_i) {
                    // Variable i is assigned, remove this value from others
                    for j in 0..3 {
                        if i != j {
                            self.exclude_value_from_domain(self.vars[j], min_i, ctx)?;
                        }
                    }
                }
            }
            
            return Some(());
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
            return all_int_values.len() >= self.vars.len();
        }
        
        // With float variables, assume feasible (constraint will be checked during search)
        true
    }
}

impl Prune for AllDifferent {
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
        
        // Use specialized algorithms based on problem size
        if n <= 10 {
            return self.propagate_small(ctx);
        }
        
        // For large problems, use basic constraint propagation
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
}

impl Propagate for AllDifferent {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        self.vars.iter().copied()
    }
}
