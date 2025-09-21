use crate::{
    constraints::props::{Propagate, Prune},
    variables::{Val, VarId},
    variables::views::{Context, View},
};

/// AllEqual constraint implementation
/// 
/// Ensures all variables in the set have the same value by computing
/// the intersection of all variable domains and propagating it to each variable.
#[derive(Clone, Debug)]
#[doc(hidden)]
pub struct AllEqual {
    vars: Vec<VarId>,
}

impl AllEqual {
    pub fn new(vars: Vec<VarId>) -> Self {
        Self { vars }
    }

    /// Compute the intersection of all variable domains
    fn compute_domain_intersection(&self, ctx: &Context) -> Option<(Val, Val)> {
        if self.vars.is_empty() {
            return None;
        }

        // Start with the first variable's domain
        let first_var = self.vars[0];
        let mut intersection_min = first_var.min(ctx);
        let mut intersection_max = first_var.max(ctx);

        // Intersect with all other variable domains
        for &var in &self.vars[1..] {
            let var_min = var.min(ctx);
            let var_max = var.max(ctx);

            // Update intersection bounds
            if var_min > intersection_min {
                intersection_min = var_min;
            }
            if var_max < intersection_max {
                intersection_max = var_max;
            }

            // Check if intersection becomes empty
            if intersection_min > intersection_max {
                return None;
            }
        }

        Some((intersection_min, intersection_max))
    }
}

impl Prune for AllEqual {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        // Compute the intersection of all variable domains
        let (new_min, new_max) = self.compute_domain_intersection(ctx)?;

        // Apply the intersection to all variables
        for &var in &self.vars {
            // Update minimum bound if necessary
            if var.min(ctx) < new_min {
                let _min = var.try_set_min(new_min, ctx)?;
            }

            // Update maximum bound if necessary  
            if var.max(ctx) > new_max {
                let _max = var.try_set_max(new_max, ctx)?;
            }
        }

        Some(())
    }
}

impl Propagate for AllEqual {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        self.vars.iter().copied()
    }
}