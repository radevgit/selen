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
        Self {
            vars,
            target_value,
            count_var,
        }
    }

    /// Count number of variables that definitely equal the target value
    fn count_definitely_equal(&self, ctx: &Context) -> i64 {
        self.vars.iter()
            .filter(|&&var| {
                let min = var.min(ctx);
                let max = var.max(ctx);
                min == max && min == self.target_value
            })
            .count() as i64
    }

    /// Count number of variables that could possibly equal the target value
    fn count_possibly_equal(&self, ctx: &Context) -> i64 {
        self.vars.iter()
            .filter(|&&var| {
                let min = var.min(ctx);
                let max = var.max(ctx);
                min <= self.target_value && self.target_value <= max
            })
            .count() as i64
    }

    /// Propagate bounds on the count variable based on current variable states
    fn propagate_count_bounds(&self, ctx: &mut Context) -> Option<()> {
        let definitely_equal = self.count_definitely_equal(ctx);
        let possibly_equal = self.count_possibly_equal(ctx);
        
        // The count variable must be at least the number of variables definitely equal
        // and at most the number of variables possibly equal
        
        // Set lower bound: at least definitely_equal
        self.count_var.try_set_min(Val::ValI(definitely_equal as i32), ctx)?;
        
        // Set upper bound: at most possibly_equal  
        self.count_var.try_set_max(Val::ValI(possibly_equal as i32), ctx)?;
        
        // If count is fixed, we might be able to propagate to the vars
        let count_min = self.count_var.min(ctx);
        let count_max = self.count_var.max(ctx);
        if count_min == count_max {
            let target_count = match count_min {
                Val::ValI(i) => i as i64,
                Val::ValF(f) => f as i64,
            };
            
            // If we already have enough variables equal to target, forbid others
            if definitely_equal == target_count {
                for &var in &self.vars {
                    let min = var.min(ctx);
                    let max = var.max(ctx);
                    if min != max && min <= self.target_value && self.target_value <= max {
                        // Remove target_value from this variable's domain
                        if self.target_value == min {
                            // Exclude by increasing min
                            match self.target_value {
                                Val::ValI(i) => var.try_set_min(Val::ValI(i + 1), ctx)?,
                                Val::ValF(f) => var.try_set_min(Val::ValF(f + 1.0), ctx)?,
                            };
                        } else if self.target_value == max {
                            // Exclude by decreasing max
                            match self.target_value {
                                Val::ValI(i) => var.try_set_max(Val::ValI(i - 1), ctx)?,
                                Val::ValF(f) => var.try_set_max(Val::ValF(f - 1.0), ctx)?,
                            };
                        }
                        // For values in the middle, this is more complex - skip for now
                    }
                }
            }
            // If we need all remaining possible variables to be equal to target
            else if possibly_equal == target_count {
                for &var in &self.vars {
                    let min = var.min(ctx);
                    let max = var.max(ctx);
                    if min != max && min <= self.target_value && self.target_value <= max {
                        // Force this variable to equal target_value
                        var.try_set_min(self.target_value, ctx)?;
                        var.try_set_max(self.target_value, ctx)?;
                    }
                }
            }
        }
        
        Some(())
    }
}

impl Prune for Count {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        // Propagate bounds on the count variable based on current variable states
        self.propagate_count_bounds(ctx)
    }
}

impl Propagate for Count {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        self.vars.iter().chain(std::iter::once(&self.count_var)).copied()
    }
}

#[cfg(test)]
mod test_count_direct {
    use super::*;
    use crate::vars::Vars;
    use crate::views::Context;
    
    #[test]
    fn test_count_constraint_direct() {
        let mut vars = Vars::new();
        let v1 = vars.new_var_with_bounds(Val::int(1), Val::int(3));
        let v2 = vars.new_var_with_bounds(Val::int(1), Val::int(3));
        let v3 = vars.new_var_with_bounds(Val::int(1), Val::int(3));
        let count_var = vars.new_var_with_bounds(Val::int(1), Val::int(1));
        
        let count = Count::new(vec![v1, v2, v3], Val::int(1), count_var);
        let mut events = Vec::new();
        let mut ctx = Context::new(&mut vars, &mut events);
        
        // This should work now - our implementation should run
        let result = count.prune(&mut ctx);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_count_trait_object_dispatch() {
        println!("=== Testing Count trait object dispatch ===");
        
        let mut vars = Vars::new();
        let v1 = vars.new_var_with_bounds(Val::int(1), Val::int(3));
        let v2 = vars.new_var_with_bounds(Val::int(1), Val::int(3));
        let count_var = vars.new_var_with_bounds(Val::int(1), Val::int(1));
        
        let count = Count::new(vec![v1, v2], Val::int(1), count_var);
        
        // Store as trait object exactly like the propagator system does
        let trait_object: Box<dyn Prune> = Box::new(count);
        let shared_trait_object = std::rc::Rc::new(trait_object);
        
        let mut events = Vec::new();
        let mut ctx = Context::new(&mut vars, &mut events);
        
        println!("Calling prune through trait object...");
        let result = shared_trait_object.as_ref().prune(&mut ctx);
        println!("Trait object prune result: {:?}", result.is_some());
        
        assert!(result.is_some(), "Count constraint should work through trait object");
    }
}