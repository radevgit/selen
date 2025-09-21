use crate::{constraints::props::{Propagate, Prune}, variables::{VarId, Val}, variables::views::{Context, View}};

/// Global maximum constraint: `result = max(vars...)`.
/// This constraint enforces that the result variable equals the maximum value among all input variables.
#[derive(Clone, Debug)]
#[doc(hidden)]
pub struct Max {
    vars: Vec<VarId>,
    result: VarId,
}

impl Max {
    pub fn new(vars: Vec<VarId>, result: VarId) -> Self {
        Self { vars, result }
    }
}

impl Prune for Max {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        if self.vars.is_empty() {
            // Empty set - undefined behavior, but we'll just return
            return Some(());
        }

        // For result = max(vars), we need to enforce:
        // 1. result >= max(all vars minimums) 
        // 2. result <= maximum possible value among all vars  
        // 3. Each var <= result (since result is the maximum)
        // 4. At least one var must be able to equal result

        // Step 1: Calculate bounds for result based on all variables
        let mut max_of_mins = None;
        let mut max_of_maxs = None;

        for &var in &self.vars {
            let var_min = var.min(ctx);
            let var_max = var.max(ctx);

            // Update maximum of minimums (lower bound for result)
            max_of_mins = Some(match max_of_mins {
                None => var_min,
                Some(current) => if var_min > current { var_min } else { current },
            });

            // Update maximum of maximums (upper bound for result)
            max_of_maxs = Some(match max_of_maxs {
                None => var_max,
                Some(current) => if var_max > current { var_max } else { current },
            });
        }

        let result_min = max_of_mins.unwrap();
        let result_max = max_of_maxs.unwrap();

        // Step 2: Propagate bounds to result variable
        let _min = self.result.try_set_min(result_min, ctx)?;
        let _max = self.result.try_set_max(result_max, ctx)?;

        // Step 3: Get updated result bounds
        let result_min_updated = self.result.min(ctx);
        let result_max_updated = self.result.max(ctx);

        // Step 4: Propagate back to input variables
        // Each variable must be <= result (since result is the maximum)
        for &var in &self.vars {
            let _max = var.try_set_max(result_max_updated, ctx)?;
        }

        // Step 5: Ensure at least one variable can achieve the maximum
        // If result is fixed to a specific value, at least one variable must be able to equal it
        if result_min_updated == result_max_updated {
            let target_value = result_max_updated;
            let mut can_achieve_max = false;

            for &var in &self.vars {
                let var_min = var.min(ctx);
                let var_max = var.max(ctx);
                
                if var_min <= target_value && target_value <= var_max {
                    can_achieve_max = true;
                    break;
                }
            }

            if !can_achieve_max {
                // No variable can achieve the required maximum - constraint is unsatisfiable
                return None;
            }
        }

        // Step 6: Additional propagation - if only one variable can achieve the current maximum,
        // we might be able to tighten bounds further
        let current_max = result_max_updated;
        let mut vars_that_can_be_max = Vec::new();

        for &var in &self.vars {
            let var_min = var.min(ctx);
            let var_max = var.max(ctx);
            
            if var_min <= current_max && current_max <= var_max {
                vars_that_can_be_max.push(var);
            }
        }

        // If all variables except those that can be maximum have a maximum < current_max,
        // we can potentially tighten the result's lower bound
        let mut prev_maximum = None;
        for &var in &self.vars {
            let var_max = var.max(ctx);
            
            if var_max < current_max {
                prev_maximum = Some(match prev_maximum {
                    None => var_max,
                    Some(current) => if var_max > current { var_max } else { current },
                });
            }
        }

        // If we have variables that can't be the maximum, use their maximums to bound result
        if let Some(prev_max) = prev_maximum {
            if vars_that_can_be_max.len() == 1 {
                // Only one variable can achieve the maximum
                let only_max_var = vars_that_can_be_max[0];
                let var_min = only_max_var.min(ctx);
                
                // The result can't be smaller than this variable's minimum
                // (since it's the only one that can be maximum)
                let new_result_min = if var_min > prev_max { var_min } else { 
                    // Take the maximum of var_min and (prev_max + 1) if applicable
                    match (var_min, prev_max) {
                        (Val::ValI(min_i), Val::ValI(prev_i)) => {
                            Val::ValI(if min_i > prev_i + 1 { min_i } else { prev_i + 1 })
                        },
                        (Val::ValF(min_f), Val::ValF(prev_f)) => {
                            // For floats, we can use a very small epsilon
                            let epsilon = f64::EPSILON;
                            Val::ValF(if min_f > prev_f + epsilon { min_f } else { prev_f + epsilon })
                        },
                        _ => var_min, // Mixed types - keep current min
                    }
                };
                
                let _min = self.result.try_set_min(new_result_min, ctx)?;
            }
        }

        Some(())
    }
}

impl Propagate for Max {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        core::iter::once(self.result)
            .chain(self.vars.iter().copied())
    }
}
