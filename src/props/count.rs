use crate::{
    props::{Propagate, Prune},
    vars::{Val, VarId},
    views::{Context, View},
};

/// Count constraint implementation
/// 
/// Ensures that exactly `count_var` variables from `vars` equal `target_value`.
/// This is a global constraint commonly used in scheduling, resource allocation,
/// and counting problems.
#[derive(Clone, Debug)]
pub struct Count {
    vars: Vec<VarId>,
    target_value: Val,
    count_var: VarId,
}

impl Count {
    pub fn new(vars: Vec<VarId>, target_value: Val, count_var: VarId) -> Self {
        Self { vars, target_value, count_var }
    }

    /// Count how many variables are definitely equal to target_value (fixed to it)
    fn count_definitely_equal(&self, ctx: &Context) -> i64 {
        self.vars.iter()
            .filter(|&&var| {
                let min_val = var.min(ctx);
                let max_val = var.max(ctx);
                min_val == max_val && min_val == self.target_value
            })
            .count() as i64
    }

    /// Count how many variables possibly equal target_value (contains it in domain)
    fn count_possibly_equal(&self, ctx: &Context) -> i64 {
        self.vars.iter()
            .filter(|&&var| {
                let min_val = var.min(ctx);
                let max_val = var.max(ctx);
                min_val <= self.target_value && max_val >= self.target_value
            })
            .count() as i64
    }
}

impl Prune for Count {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        // Get current count bounds
        let count_min = self.count_var.min(ctx);
        let count_max = self.count_var.max(ctx);
        
        // Convert to i64 for easier arithmetic
        let target_count_min = match count_min {
            Val::ValI(i) => i as i64,
            Val::ValF(f) => f.ceil() as i64,
        };
        
        let target_count_max = match count_max {
            Val::ValI(i) => i as i64,
            Val::ValF(f) => f.floor() as i64,
        };
        
        // Count current state
        let definitely_equal = self.count_definitely_equal(ctx);
        let possibly_equal = self.count_possibly_equal(ctx);

        // Check feasibility
        if definitely_equal > target_count_max {
            return None; // Too many variables already equal target
        }
        
        if possibly_equal < target_count_min {
            return None; // Not enough variables can equal target
        }

        // If we already have enough definite equals and need no more
        if definitely_equal == target_count_max {
            // Force all undecided variables to NOT equal target_value
            for &var in &self.vars {
                let min_val = var.min(ctx);
                let max_val = var.max(ctx);
                
                // If variable is undecided about target_value, exclude it
                if min_val <= self.target_value && max_val >= self.target_value && min_val != max_val {
                    // Try to exclude target_value from domain
                    if min_val == self.target_value {
                        // target_value is the minimum, so increase minimum
                        let next_val = match self.target_value {
                            Val::ValI(i) => Val::ValI(i + 1),
                            Val::ValF(f) => Val::ValF(f + f.abs() * f64::EPSILON + f64::EPSILON),
                        };
                        var.try_set_min(next_val, ctx)?;
                    } else if max_val == self.target_value {
                        // target_value is the maximum, so decrease maximum
                        let prev_val = match self.target_value {
                            Val::ValI(i) => Val::ValI(i - 1),
                            Val::ValF(f) => Val::ValF(f - f.abs() * f64::EPSILON - f64::EPSILON),
                        };
                        var.try_set_max(prev_val, ctx)?;
                    }
                    // Note: If target_value is in the middle of domain, we can't easily exclude it
                    // without more sophisticated domain representation
                }
            }
        }

        // If we need more variables to equal target and have exactly the right number possible
        if definitely_equal < target_count_min && possibly_equal == target_count_min {
            // Force all undecided variables that can equal target to equal it
            for &var in &self.vars {
                let min_val = var.min(ctx);
                let max_val = var.max(ctx);
                
                if min_val <= self.target_value && max_val >= self.target_value && min_val != max_val {
                    // Force this variable to equal target_value
                    var.try_set_min(self.target_value, ctx)?;
                    var.try_set_max(self.target_value, ctx)?;
                }
            }
        }

        // Update count variable bounds based on current state
        self.propagate_count_bounds(ctx)?;
        
        Some(())
    }
}

impl Count {
    /// Propagate bounds on the count variable based on current variable states
    fn propagate_count_bounds(&self, ctx: &mut Context) -> Option<()> {
        let definitely_equal = self.count_definitely_equal(ctx);
        let possibly_equal = self.count_possibly_equal(ctx);

        // Minimum count: variables that are definitely equal
        let min_count = definitely_equal;
        
        // Maximum count: variables that could possibly be equal
        let max_count = possibly_equal;

        // Update count variable bounds
        let current_min = self.count_var.min(ctx);
        if let Val::ValI(current_min_val) = current_min {
            if (current_min_val as i64) < min_count {
                let new_min = match min_count {
                    i if i <= i32::MAX as i64 => Val::ValI(i as i32),
                    _ => Val::ValF(min_count as f64),
                };
                self.count_var.try_set_min(new_min, ctx)?;
            }
        } else {
            let new_min = Val::ValI(min_count as i32);
            self.count_var.try_set_min(new_min, ctx)?;
        }

        let current_max = self.count_var.max(ctx);
        if let Val::ValI(current_max_val) = current_max {
            if (current_max_val as i64) > max_count {
                let new_max = match max_count {
                    i if i <= i32::MAX as i64 => Val::ValI(i as i32),
                    _ => Val::ValF(max_count as f64),
                };
                self.count_var.try_set_max(new_max, ctx)?;
            }
        } else {
            let new_max = Val::ValI(max_count as i32);
            self.count_var.try_set_max(new_max, ctx)?;
        }

        Some(())
    }
}

impl Propagate for Count {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        let mut triggers = self.vars.clone();
        triggers.push(self.count_var);
        triggers.into_iter()
    }
}