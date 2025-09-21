use crate::{constraints::props::{Propagate, Prune}, variables::{VarId, Val}, variables::views::{Context, View}};

/// Global minimum constraint: `result = min(vars...)`.
/// This constraint enforces that the result variable equals the minimum value among all input variables.
#[derive(Clone, Debug)]
#[doc(hidden)]
pub struct Min {
    vars: Vec<VarId>,
    result: VarId,
}

impl Min {
    pub fn new(vars: Vec<VarId>, result: VarId) -> Self {
        Self { vars, result }
    }
}

impl Prune for Min {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        if self.vars.is_empty() {
            // Empty set - undefined behavior, but we'll just return
            return Some(());
        }

        // For result = min(vars), we need to enforce:
        // 1. result <= min(all vars) 
        // 2. result >= minimum possible value among all vars
        // 3. Each var >= result (since result is the minimum)
        // 4. At least one var must be able to equal result

        // Step 1: Calculate bounds for result based on all variables
        let mut min_of_mins = None;
        let mut min_of_maxs = None;

        for &var in &self.vars {
            let var_min = var.min(ctx);
            let var_max = var.max(ctx);

            // Update minimum of minimums (lower bound for result)
            min_of_mins = Some(match min_of_mins {
                None => var_min,
                Some(current) => if var_min < current { var_min } else { current },
            });

            // Update minimum of maximums (upper bound for result)
            min_of_maxs = Some(match min_of_maxs {
                None => var_max,
                Some(current) => if var_max < current { var_max } else { current },
            });
        }

        let result_min = min_of_mins.unwrap();
        let result_max = min_of_maxs.unwrap();

        // Step 2: Propagate bounds to result variable
        let _min = self.result.try_set_min(result_min, ctx)?;
        let _max = self.result.try_set_max(result_max, ctx)?;

        // Step 3: Get updated result bounds
        let result_min_updated = self.result.min(ctx);
        let result_max_updated = self.result.max(ctx);

        // Step 4: Propagate back to input variables
        // Each variable must be >= result (since result is the minimum)
        for &var in &self.vars {
            let _min = var.try_set_min(result_min_updated, ctx)?;
        }

        // Step 5: Ensure at least one variable can achieve the minimum
        // If result is fixed to a specific value, at least one variable must be able to equal it
        if result_min_updated == result_max_updated {
            let target_value = result_min_updated;
            let mut can_achieve_min = false;

            for &var in &self.vars {
                let var_min = var.min(ctx);
                let var_max = var.max(ctx);
                
                if var_min <= target_value && target_value <= var_max {
                    can_achieve_min = true;
                    break;
                }
            }

            if !can_achieve_min {
                // No variable can achieve the required minimum - constraint is unsatisfiable
                return None;
            }
        }

        // Step 6: Additional propagation - if only one variable can achieve the current minimum,
        // we might be able to tighten bounds further
        let current_min = result_min_updated;
        let mut vars_that_can_be_min = Vec::new();

        for &var in &self.vars {
            let var_min = var.min(ctx);
            let var_max = var.max(ctx);
            
            if var_min <= current_min && current_min <= var_max {
                vars_that_can_be_min.push(var);
            }
        }

        // If all variables except those that can be minimum have a minimum > current_min,
        // we can potentially tighten the result's upper bound
        let mut next_minimum = None;
        for &var in &self.vars {
            let var_min = var.min(ctx);
            
            if var_min > current_min {
                next_minimum = Some(match next_minimum {
                    None => var_min,
                    Some(current) => if var_min < current { var_min } else { current },
                });
            }
        }

        // If we have variables that can't be the minimum, use their minimums to bound result
        if let Some(next_min) = next_minimum {
            if vars_that_can_be_min.len() == 1 {
                // Only one variable can achieve the minimum
                let only_min_var = vars_that_can_be_min[0];
                let var_max = only_min_var.max(ctx);
                
                // The result can't be larger than this variable's maximum
                // (since it's the only one that can be minimum)
                let new_result_max = if var_max < next_min { var_max } else { 
                    // Take the minimum of var_max and (next_min - 1) if applicable
                    match (var_max, next_min) {
                        (Val::ValI(max_i), Val::ValI(next_i)) => {
                            Val::ValI(if max_i < next_i - 1 { max_i } else { next_i - 1 })
                        },
                        (Val::ValF(max_f), Val::ValF(next_f)) => {
                            // For floats, we can use a very small epsilon
                            let epsilon = f64::EPSILON;
                            Val::ValF(if max_f < next_f - epsilon { max_f } else { next_f - epsilon })
                        },
                        _ => var_max, // Mixed types - keep current max
                    }
                };
                
                let _max = self.result.try_set_max(new_result_max, ctx)?;
            }
        }

        Some(())
    }
}

impl Propagate for Min {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        core::iter::once(self.result)
            .chain(self.vars.iter().copied())
    }
}
