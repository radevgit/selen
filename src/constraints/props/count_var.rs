use crate::{
    constraints::props::{Propagate, Prune},
    variables::VarId,
    variables::views::{Context, View},
};

/// Count with variable target constraint implementation
/// 
/// Ensures that exactly `count_var` variables from `vars` equal `target_var`.
/// This is a generalization of the count constraint that supports variable targets.
#[derive(Clone, Debug)]
#[doc(hidden)]
pub struct CountVar {
    vars: Vec<VarId>,
    target_var: VarId,
    count_var: VarId,
}

impl CountVar {
    pub fn new(vars: Vec<VarId>, target_var: VarId, count_var: VarId) -> Self {
        Self {
            vars,
            target_var,
            count_var,
        }
    }

    /// Count number of variables that definitely equal the target variable's value
    fn count_definitely_equal(&self, ctx: &Context) -> i64 {
        let target_min = self.target_var.min(ctx);
        let target_max = self.target_var.max(ctx);
        
        // If target is not fixed, we can't determine definite equality yet
        if target_min != target_max {
            return 0;
        }
        
        self.vars.iter()
            .filter(|&&var| {
                let min = var.min(ctx);
                let max = var.max(ctx);
                min == max && min == target_min
            })
            .count() as i64
    }

    /// Count number of variables that could possibly equal the target variable's value
    fn count_possibly_equal(&self, ctx: &Context) -> i64 {
        let target_min = self.target_var.min(ctx);
        let target_max = self.target_var.max(ctx);
        
        self.vars.iter()
            .filter(|&&var| {
                let var_min = var.min(ctx);
                let var_max = var.max(ctx);
                // Variable possibly equals target if their domains overlap
                var_min <= target_max && var_max >= target_min
            })
            .count() as i64
    }

    /// Propagate bounds on the count variable based on current variable states
    fn propagate_count_bounds(&self, ctx: &mut Context) -> Option<()> {
        let definitely_equal = self.count_definitely_equal(ctx);
        let possibly_equal = self.count_possibly_equal(ctx);
        
        // The count variable must be at least the number of variables definitely equal
        // and at most the number of variables possibly equal
        let definitely_equal_val = crate::variables::Val::ValI(definitely_equal as i32);
        let possibly_equal_val = crate::variables::Val::ValI(possibly_equal as i32);
        
        // Set lower bound: at least definitely_equal
        self.count_var.try_set_min(definitely_equal_val, ctx)?;
        
        // Set upper bound: at most possibly_equal  
        self.count_var.try_set_max(possibly_equal_val, ctx)?;
        
        // If count is fixed and target is fixed, we might be able to propagate to the vars
        let count_min = self.count_var.min(ctx);
        let count_max = self.count_var.max(ctx);
        let target_min = self.target_var.min(ctx);
        let target_max = self.target_var.max(ctx);
        
        if count_min == count_max && target_min == target_max {
            let target_count = match count_min {
                crate::variables::Val::ValI(i) => i as i64,
                crate::variables::Val::ValF(f) => f as i64,
            };
            
            // If we already have enough variables equal to target, forbid others from having target value
            if definitely_equal == target_count {
                for &var in &self.vars {
                    let min = var.min(ctx);
                    let max = var.max(ctx);
                    if min != max && min <= target_min && target_min <= max {
                        // Remove target_min from this variable's domain by constraining around it
                        // This is a simplified version - full domain removal is complex
                        match (target_min, min, max) {
                            (crate::variables::Val::ValI(tgt), crate::variables::Val::ValI(min_val), crate::variables::Val::ValI(max_val)) => {
                                if tgt == min_val && tgt < max_val {
                                    var.try_set_min(crate::variables::Val::ValI(tgt + 1), ctx)?;
                                } else if tgt == max_val && tgt > min_val {
                                    var.try_set_max(crate::variables::Val::ValI(tgt - 1), ctx)?;
                                }
                            }
                            _ => {} // Skip for float values
                        }
                    }
                }
            }
            // If we need all remaining possible variables to be equal to target
            else if possibly_equal == target_count {
                for &var in &self.vars {
                    let min = var.min(ctx);
                    let max = var.max(ctx);
                    if min != max && min <= target_min && target_min <= max {
                        // Force this variable to equal target_min
                        var.try_set_min(target_min, ctx)?;
                        var.try_set_max(target_min, ctx)?;
                    }
                }
            }
        }
        
        Some(())
    }
}

impl Prune for CountVar {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        // Propagate bounds on the count variable based on current variable states
        self.propagate_count_bounds(ctx)
    }
}

impl Propagate for CountVar {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        self.vars.iter()
            .chain(std::iter::once(&self.target_var))
            .chain(std::iter::once(&self.count_var))
            .copied()
    }
}

#[cfg(test)]
mod test_count_var_direct {
    use super::*;
    use crate::variables::Vars;
    use crate::variables::views::Context;
    use crate::variables::Val;
    
    #[test]
    fn test_count_var_constraint_direct() {
        let mut vars = Vars::new();
        let v1 = vars.new_var_with_bounds(Val::int(1), Val::int(3));
        let v2 = vars.new_var_with_bounds(Val::int(1), Val::int(3));
        let v3 = vars.new_var_with_bounds(Val::int(1), Val::int(3));
        let target_var = vars.new_var_with_bounds(Val::int(1), Val::int(1));
        let count_var = vars.new_var_with_bounds(Val::int(0), Val::int(3));
        
        let count_var_prop = CountVar::new(vec![v1, v2, v3], target_var, count_var);
        let mut events = Vec::new();
        let mut ctx = Context::new(&mut vars, &mut events);
        
        // This should work now - our implementation should run
        let result = count_var_prop.prune(&mut ctx);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_count_var_trait_object_dispatch() {
        println!("=== Testing CountVar trait object dispatch ===");
        
        let mut vars = Vars::new();
        let v1 = vars.new_var_with_bounds(Val::int(1), Val::int(3));
        let v2 = vars.new_var_with_bounds(Val::int(1), Val::int(3));
        let target_var = vars.new_var_with_bounds(Val::int(2), Val::int(2));
        let count_var = vars.new_var_with_bounds(Val::int(0), Val::int(2));
        
        let count_var_prop = CountVar::new(vec![v1, v2], target_var, count_var);
        
        // Store as trait object exactly like the propagator system does
        let trait_object: Box<dyn Prune> = Box::new(count_var_prop);
        let shared_trait_object = std::rc::Rc::new(trait_object);
        
        let mut events = Vec::new();
        let mut ctx = Context::new(&mut vars, &mut events);
        
        println!("Calling prune through trait object...");
        let result = shared_trait_object.as_ref().prune(&mut ctx);
        println!("Trait object prune result: {:?}", result.is_some());
        
        assert!(result.is_some(), "CountVar constraint should work through trait object");
    }
}
