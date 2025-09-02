use crate::{
    props::{Propagate, Prune},
    vars::{Val, VarId},
    views::{Context, View},
    utils::{float_equal, float_next, float_prev},
};

/// All-different constraint: all variables in the set must have distinct values
/// 
/// This propagator ensures that all variables are assigned different values.
/// It uses more efficient algorithms than pairwise not-equals constraints:
/// 1. Domain consistency checking
/// 2. Hall's theorem for identifying forced assignments
/// 3. ULP-aware float comparisons for precision
#[derive(Clone, Debug)]
pub struct AllDifferent {
    vars: Vec<VarId>,
}

impl AllDifferent {
    pub fn new(vars: Vec<VarId>) -> Self {
        Self { vars }
    }
}

impl Prune for AllDifferent {
    fn prune(&mut self, ctx: &mut Context) -> Option<()> {
        // If fewer than 2 variables, constraint is trivially satisfied
        if self.vars.len() < 2 {
            return Some(());
        }

        // Apply domain consistency for all-different
        self.propagate_domain_consistency(ctx)?;
        
        // Apply Hall's theorem for identifying necessary assignments
        self.propagate_halls_theorem(ctx)?;

        Some(())
    }
}

impl AllDifferent {
    /// Apply domain consistency: if a variable has a singleton domain,
    /// remove that value from all other variables' domains
    fn propagate_domain_consistency(&self, ctx: &mut Context) -> Option<()> {
        let mut assigned_values = Vec::new();
        
        // Collect all assigned (singleton) values
        for &var in &self.vars {
            let min_val = var.min(ctx);
            let max_val = var.max(ctx);
            
            // Check if variable is assigned (singleton domain)
            if is_singleton(min_val, max_val) {
                assigned_values.push((var, min_val));
            }
        }
        
        // Check for conflicts: two variables assigned to the same value
        for i in 0..assigned_values.len() {
            for j in (i + 1)..assigned_values.len() {
                if values_equal(assigned_values[i].1, assigned_values[j].1) {
                    return None; // Constraint violated
                }
            }
        }
        
        // Remove assigned values from all other variables' domains
        for &(assigned_var, assigned_val) in &assigned_values {
            for &var in &self.vars {
                if var != assigned_var {
                    exclude_value_from_domain(var, assigned_val, ctx)?;
                }
            }
        }
        
        Some(())
    }
    
    /// Apply Hall's theorem: if k variables have domains that collectively
    /// contain only k values, then those variables must be assigned to those values
    fn propagate_halls_theorem(&self, ctx: &mut Context) -> Option<()> {
        // For efficiency, we'll implement a simplified version for small sets
        // Full Hall's theorem checking is complex and expensive
        
        // Check for the common case: if we have n variables and only n possible values
        // across all their domains, then we have a Hall set
        let all_possible_values = self.collect_all_possible_values(ctx);
        
        if all_possible_values.len() < self.vars.len() {
            // Not enough values for all variables - constraint cannot be satisfied
            return None;
        }
        
        // For small sets (common in practice), check explicit Hall sets
        if self.vars.len() <= 4 {
            self.check_small_hall_sets(ctx)?;
        }
        
        Some(())
    }
    
    /// Collect all possible values across all variable domains
    fn collect_all_possible_values(&self, ctx: &mut Context) -> Vec<Val> {
        let mut all_values = Vec::new();
        
        for &var in &self.vars {
            let min_val = var.min(ctx);
            let max_val = var.max(ctx);
            
            // For integer domains, enumerate all values
            if let (Val::ValI(min_i), Val::ValI(max_i)) = (min_val, max_val) {
                for val in min_i..=max_i {
                    let val_obj = Val::int(val);
                    if !all_values.iter().any(|&v| values_equal(v, val_obj)) {
                        all_values.push(val_obj);
                    }
                }
            } else {
                // For float domains, this is more complex - for now, just add bounds
                if !all_values.iter().any(|&v| values_equal(v, min_val)) {
                    all_values.push(min_val);
                }
                if !all_values.iter().any(|&v| values_equal(v, max_val)) {
                    all_values.push(max_val);
                }
            }
        }
        
        all_values
    }
    
    /// Check for Hall sets in small variable sets (brute force for small n)
    fn check_small_hall_sets(&self, ctx: &mut Context) -> Option<()> {
        let n = self.vars.len();
        
        // Check all possible subsets of size 2 to n-1
        for subset_size in 2..n {
            // Generate all combinations of subset_size variables
            let combinations = self.generate_combinations(subset_size);
            
            for combination in combinations {
                let mut union_values = Vec::new();
                
                // Collect union of domains for this combination
                for &var_idx in &combination {
                    let var = self.vars[var_idx];
                    let min_val = var.min(ctx);
                    let max_val = var.max(ctx);
                    
                    // Add values from this variable's domain to union
                    if let (Val::ValI(min_i), Val::ValI(max_i)) = (min_val, max_val) {
                        for val in min_i..=max_i {
                            let val_obj = Val::int(val);
                            if !union_values.iter().any(|&v| values_equal(v, val_obj)) {
                                union_values.push(val_obj);
                            }
                        }
                    }
                }
                
                // Hall's theorem: if |union_values| == |combination|, we have a Hall set
                if union_values.len() == combination.len() {
                    // Remove these values from all variables NOT in the combination
                    for (var_idx, &var) in self.vars.iter().enumerate() {
                        if !combination.contains(&var_idx) {
                            for &val in &union_values {
                                exclude_value_from_domain(var, val, ctx)?;
                            }
                        }
                    }
                }
            }
        }
        
        Some(())
    }
    
    /// Generate all combinations of k indices from 0..n
    fn generate_combinations(&self, k: usize) -> Vec<Vec<usize>> {
        let n = self.vars.len();
        let mut result = Vec::new();
        let mut current = Vec::new();
        
        self.generate_combinations_recursive(0, n, k, &mut current, &mut result);
        result
    }
    
    fn generate_combinations_recursive(
        &self,
        start: usize,
        n: usize,
        k: usize,
        current: &mut Vec<usize>,
        result: &mut Vec<Vec<usize>>,
    ) {
        if current.len() == k {
            result.push(current.clone());
            return;
        }
        
        for i in start..n {
            current.push(i);
            self.generate_combinations_recursive(i + 1, n, k, current, result);
            current.pop();
        }
    }
}

impl Propagate for AllDifferent {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        self.vars.iter().cloned()
    }
}

/// Check if a domain represents a single value (singleton)
fn is_singleton(min: Val, max: Val) -> bool {
    match (min, max) {
        (Val::ValI(min_i), Val::ValI(max_i)) => min_i == max_i,
        (Val::ValF(min_f), Val::ValF(max_f)) => float_equal(min_f, max_f),
        (Val::ValI(_), Val::ValF(_)) | (Val::ValF(_), Val::ValI(_)) => false, // Mixed types are not singleton
    }
}

/// Check if two values are equal using ULP-aware comparison
fn values_equal(a: Val, b: Val) -> bool {
    match (a, b) {
        (Val::ValI(a_i), Val::ValI(b_i)) => a_i == b_i,
        (Val::ValF(a_f), Val::ValF(b_f)) => float_equal(a_f, b_f),
        _ => false, // Different types are not equal
    }
}

/// Exclude a specific value from a variable's domain
fn exclude_value_from_domain(var: VarId, value: Val, ctx: &mut Context) -> Option<()> {
    let min_val = var.min(ctx);
    let max_val = var.max(ctx);
    
    match value {
        Val::ValI(target_i) => {
            if let (Val::ValI(min_i), Val::ValI(max_i)) = (min_val, max_val) {
                if target_i < min_i || target_i > max_i {
                    return Some(()); // Value not in domain anyway
                }
                
                if min_i == max_i && min_i == target_i {
                    return None; // Would make domain empty
                }
                
                if min_i == target_i {
                    // Remove from lower bound
                    var.try_set_min(Val::int(target_i + 1), ctx)?;
                } else if max_i == target_i {
                    // Remove from upper bound  
                    var.try_set_max(Val::int(target_i - 1), ctx)?;
                }
                // For values in the middle of integer domains, the basic CSP solver
                // doesn't support holes, so we can't remove them directly
            }
        }
        Val::ValF(target_f) => {
            if let (Val::ValF(min_f), Val::ValF(max_f)) = (min_val, max_val) {
                if target_f < min_f || target_f > max_f {
                    return Some(()); // Value not in domain anyway
                }
                
                if float_equal(min_f, max_f) && float_equal(min_f, target_f) {
                    return None; // Would make domain empty
                }
                
                if float_equal(min_f, target_f) {
                    // Remove from lower bound using ULP
                    var.try_set_min(Val::float(float_next(target_f)), ctx)?;
                } else if float_equal(max_f, target_f) {
                    // Remove from upper bound using ULP
                    var.try_set_max(Val::float(float_prev(target_f)), ctx)?;
                }
            }
        }
    }
    
    Some(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{model::Model, vars::Val};

    #[test]
    fn test_all_different_basic() {
        let mut model = Model::default();
        
        let x = model.new_var(Val::int(1), Val::int(3));
        let y = model.new_var(Val::int(1), Val::int(3));
        let z = model.new_var(Val::int(1), Val::int(3));
        
        model.all_different(vec![x, y, z]);
        
        // Should find solutions where all variables have different values
        let solutions: Vec<_> = model.enumerate().collect();
        assert!(!solutions.is_empty());
        
        // Verify all solutions have distinct values
        for solution in solutions {
            let x_val = solution[x];
            let y_val = solution[y];
            let z_val = solution[z];
            
            assert_ne!(x_val, y_val);
            assert_ne!(x_val, z_val);
            assert_ne!(y_val, z_val);
        }
    }
    
    #[test]
    fn test_all_different_impossible() {
        let mut model = Model::default();
        
        // Three variables with only two possible values - impossible
        let x = model.new_var(Val::int(1), Val::int(2));
        let y = model.new_var(Val::int(1), Val::int(2));
        let z = model.new_var(Val::int(1), Val::int(2));
        
        model.all_different(vec![x, y, z]);
        
        let solutions: Vec<_> = model.enumerate().collect();
        assert!(solutions.is_empty());
    }
}
