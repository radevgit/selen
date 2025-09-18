use crate::{
    props::{Propagate, Prune},
    vars::{Val, VarId},
    views::{Context, View},
};

/// Table constraint implementation
/// 
/// Constrains variables to have values that appear as tuples in the allowed table.
/// This is a powerful global constraint that can express complex relationships
/// between variables by explicitly listing all valid combinations.
/// 
/// The table constraint is particularly useful for:
/// - Configuration problems where only certain combinations are valid
/// - Lookup tables and compatibility constraints
/// - Non-linear relationships that can't be expressed with arithmetic constraints
#[derive(Clone, Debug)]
pub struct Table {
    vars: Vec<VarId>,
    tuples: Vec<Vec<Val>>,
}

impl Table {
    pub fn new(vars: Vec<VarId>, tuples: Vec<Vec<Val>>) -> Self {
        // Validate that all tuples have the same arity as variables
        debug_assert!(
            tuples.iter().all(|tuple| tuple.len() == vars.len()),
            "All tuples must have the same arity as the number of variables"
        );
        
        Self { vars, tuples }
    }

    /// Check if a partial assignment is compatible with at least one tuple
    fn has_compatible_tuple(&self, assignment: &[Option<Val>], ctx: &Context) -> bool {
        'tuple_loop: for tuple in &self.tuples {
            // Check if this tuple is compatible with current domains
            for (i, &var) in self.vars.iter().enumerate() {
                let tuple_val = tuple[i];
                
                // Check if tuple value is within current domain
                let min_val = var.min(ctx);
                let max_val = var.max(ctx);
                
                if tuple_val < min_val || tuple_val > max_val {
                    continue 'tuple_loop; // This tuple is incompatible
                }
                
                // If we have a specific assignment, check compatibility
                if let Some(assigned_val) = assignment[i] {
                    if !self.values_equal(tuple_val, assigned_val, var, ctx) {
                        continue 'tuple_loop; // This tuple is incompatible
                    }
                }
            }
            return true; // Found at least one compatible tuple
        }
        false // No compatible tuples found
    }

    /// Get all possible values for a variable at given position that appear in valid tuples
    fn get_supported_values(&self, var_index: usize, ctx: &Context) -> Vec<Val> {
        let mut supported_values = Vec::new();
        let var = self.vars[var_index];
        let min_val = var.min(ctx);
        let max_val = var.max(ctx);

        for tuple in &self.tuples {
            let tuple_val = tuple[var_index];
            
            // Check if tuple value is within current domain
            if tuple_val >= min_val && tuple_val <= max_val {
                // Check if this tuple is compatible with other variables' domains
                let mut compatible = true;
                for (i, &other_var) in self.vars.iter().enumerate() {
                    if i == var_index {
                        continue;
                    }
                    
                    let other_min = other_var.min(ctx);
                    let other_max = other_var.max(ctx);
                    let other_tuple_val = tuple[i];
                    
                    if other_tuple_val < other_min || other_tuple_val > other_max {
                        compatible = false;
                        break;
                    }
                }
                
                if compatible && !supported_values.iter().any(|&v| self.values_equal(v, tuple_val, var, ctx)) {
                    supported_values.push(tuple_val);
                }
            }
        }
        
        supported_values
    }

    /// Check if two values are equal using proper precision context
    fn values_equal(&self, val1: Val, val2: Val, target_var: VarId, ctx: &Context) -> bool {
        let target_interval = ctx.vars().get_float_interval(target_var);
        val1.equals_with_intervals(&val2, target_interval, None)
    }

    /// Narrow domain to only supported values
    fn narrow_domain_to_supported(&self, var_index: usize, ctx: &mut Context) -> Option<()> {
        let var = self.vars[var_index];
        let supported_values = self.get_supported_values(var_index, ctx);
        
        if supported_values.is_empty() {
            return None; // No supported values - constraint is unsatisfiable
        }

        // Find the minimum and maximum supported values
        let mut min_supported = supported_values[0];
        let mut max_supported = supported_values[0];
        
        for &val in &supported_values {
            if val < min_supported {
                min_supported = val;
            }
            if val > max_supported {
                max_supported = val;
            }
        }

        // Tighten the domain bounds
        let current_min = var.min(ctx);
        let current_max = var.max(ctx);
        
        if min_supported > current_min {
            var.try_set_min(min_supported, ctx)?;
        }
        
        if max_supported < current_max {
            var.try_set_max(max_supported, ctx)?;
        }

        Some(())
    }
}

impl Prune for Table {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        // Quick feasibility check: ensure at least one tuple is compatible with current domains
        let assignment = vec![None; self.vars.len()]; // No specific assignments yet
        if !self.has_compatible_tuple(&assignment, ctx) {
            return None; // No compatible tuples found
        }

        // For each variable, narrow its domain to only values that appear in compatible tuples
        for var_index in 0..self.vars.len() {
            self.narrow_domain_to_supported(var_index, ctx)?;
        }

        // Additional consistency check: verify we still have compatible tuples after domain narrowing
        if !self.has_compatible_tuple(&assignment, ctx) {
            return None;
        }

        Some(())
    }
}

impl Propagate for Table {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        self.vars.clone().into_iter()
    }
}