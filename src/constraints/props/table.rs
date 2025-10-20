use crate::{
    constraints::props::{Propagate, Prune},
    variables::{Val, VarId},
    variables::views::{Context, View},
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
#[doc(hidden)]
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

    /// Check if a tuple is supported by current domains (all values are in domains)
    fn is_tuple_supported(&self, tuple: &[Val], ctx: &Context) -> bool {
        for (i, &var) in self.vars.iter().enumerate() {
            let tuple_val = tuple[i];
            let min_val = var.min(ctx);
            let max_val = var.max(ctx);
            
            if tuple_val < min_val || tuple_val > max_val {
                return false;
            }
        }
        true
    }

    /// Check if there's at least one supported tuple in the table
    fn has_supported_tuple(&self, ctx: &Context) -> bool {
        self.tuples.iter().any(|tuple| self.is_tuple_supported(tuple, ctx))
    }

    /// Get all possible values for a variable that appear in supported tuples
    fn get_supported_values(&self, var_index: usize, ctx: &Context) -> Vec<Val> {
        let mut supported_values = Vec::new();

        for tuple in &self.tuples {
            // Only consider tuples where all values are in current domains
            if !self.is_tuple_supported(tuple, ctx) {
                continue;
            }

            let tuple_val = tuple[var_index];
            
            // Add value if not already in list (using exact comparison for now)
            if !supported_values.iter().any(|&v| v == tuple_val) {
                supported_values.push(tuple_val);
            }
        }
        
        supported_values
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
        // GAC (Generalized Arc Consistency) implementation
        // Quick feasibility check: ensure at least one tuple is supported by current domains
        if !self.has_supported_tuple(ctx) {
            return None; // No supported tuples - constraint is unsatisfiable
        }

        // Iteratively narrow domains until fixpoint
        // This is the key difference from AC3: we keep iterating until no changes
        loop {
            let mut changed = false;

            // For each variable, narrow its domain to only values that appear in supported tuples
            for var_index in 0..self.vars.len() {
                let var = self.vars[var_index];
                let old_min = var.min(ctx);
                let old_max = var.max(ctx);

                // Narrow domain to supported values
                self.narrow_domain_to_supported(var_index, ctx)?;

                let new_min = var.min(ctx);
                let new_max = var.max(ctx);

                if old_min != new_min || old_max != new_max {
                    changed = true;
                }
            }

            // If nothing changed, we've reached fixpoint
            if !changed {
                break;
            }

            // Verify we still have at least one supported tuple after changes
            if !self.has_supported_tuple(ctx) {
                return None;
            }
        }

        Some(())
    }
}

impl Propagate for Table {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        self.vars.clone().into_iter()
    }
}